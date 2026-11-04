# Searcher Strategy — Optimal Sizing, Bid Equilibrium, Latency Cost

Sources: [S19] Uniswap v2-periphery `UniswapV2Library.getAmountOut` (997/1000 fee) — verified.
[S12] this crate: `src/searcher/pool.rs:27-31`, `src/searcher/optimizer.rs`,
`src/matchmaker/score.rs:5-12`, `src/bundle/validator.rs:20-22`, `src/sim/market.rs:22`,
`tests/test_searcher_optimizer.rs`, `tests/test_fee_escalation.rs` — verified.
[G1] Krishna, *Auction Theory*, 2nd ed., ch. 2 (symmetric first-price BNE) — to-verify.
[G2] Zhou, Qin, Torres, Le, Gervais, "High-Frequency Trading on Decentralized On-Chain
Exchanges", IEEE S&P 2021 (closed-form two-hop CPMM optimum) — to-verify.
[M4] mev-share-node `mevshare/api.go` (`mev_simBundle` 500 ms timeout) — verified via
`docs/searcher-rpc-spec.md:51`. [E1] Ethereum consensus specs `SECONDS_PER_SLOT = 12` — to-verify.
Every model assumption below is labelled; mev-share's live builder ranking rule is `unknown`.

## 1. Profit optimization — optimal trade size `x*` on constant-product AMMs

Setup: borrow `x` of token T, sell into pool A (reserves `R1` of T in, `R2` of U out),
then sell the proceeds into pool B (`R3` of U in, `R4` of T out). Fee factor `γ = 0.997` [S19].

Hop 1: `y(x) = γ·R2·x / (R1 + γ·x)`. Hop 2: `out(x) = γ·R4·y / (R3 + γ·y)`.
Substituting and clearing denominators:

```
out(x) = γ²·R2·R4·x / (R1·R3 + x·(γ·R3 + γ²·R2))
       = E_b · x / (E_a + x)                       (virtual single pool, no extra fee)
E_a = R1·R3 / (γ·R3 + γ²·R2)        E_b = γ²·R2·R4 / (γ·R3 + γ²·R2)
```

Net profit `P(x) = E_b·x/(E_a + x) − x`. `P'(x) = E_a·E_b/(E_a + x)² − 1 = 0` gives

```
x*     = sqrt(E_a · E_b) − E_a           (profitable iff E_b > E_a, i.e. P'(0) > 0)
P(x*)  = (sqrt(E_b) − sqrt(E_a))²
```

`P''(x) = −2·E_a·E_b/(E_a + x)³ < 0`, so `P` is strictly concave and `x*` is the unique
global maximum; slippage is fully captured by the `E_a + x` denominator ([G2] derives the
same form). Verification against this crate's integer ternary search [S12]:

| Case (R1, R2, R3, R4)                 | closed-form `x*`   | `optimizer.rs` result | `P(x*)`      |
|---------------------------------------|--------------------|-----------------------|--------------|
| (100 ETH, 210k USDC, 200k USDC, 100 ETH) | 1.059484 ETH    | 1.059475 ETH          | 0.022907 ETH |
| (1000, 1000, 1000, 2000)              | 205.911134         | 205.911134            | 84.417578    |

Residual gap (≈9e-6 ETH) is the floor division in `pool.rs:31` plus the 20-step search
budget, not model error. Practical caveats [S12]: `u128` overflows the `997·x·R_out`
product near 18-decimal reserves (optimizer returns `(0, 0)`, fails closed), and the
closed form is only valid at the 30 bps fee the crate models (`UNISWAP_V2_FEE_BPS`).
Gas is a constant offset: use `P(x*) − gas_cost` as the bidder's private value `v` below.

## 2. Bidding equilibrium — builder bribe vs. user kickback

Mechanism as implemented in this crate [S12]: sealed first-price auction per target hash;
rank by `bid_amount / gas_used` (`score.rs:5-7`), tie-break on larger user refund
(`score.rs:12`); `bid_amount = builder_payment + user_kickback` (`market.rs:22`); a bundle
with `kickback_bps < min` is rejected before ranking (`validator.rs:20-22`). Let `v_i` be
searcher i's private value (`P(x*) − gas`), `k` the kickback floor (0.9 in every test here;
the production default is `unknown`), and the bid `(u_i, b_i)` with `u_i ≥ k·v_i`.

**Split result (weak dominance).** Holding the total `t_i = u_i + b_i` fixed, ranking depends
only on `t_i`, and the tie-break strictly prefers larger `u_i`. Hence for any `(u, b)` with
`b > 0`, the deviation `(u + b, 0)` wins weakly more often and pays the same, so under this
crate's ranking the kickback-maximal split weakly dominates and `b_i* = 0`. Proof: the two
bids have identical primary score; the deviation is ranked no lower in every tie. ∎

**Builder-ranked variant (assumed, matches `tests/test_fee_escalation.rs`).** If the
decisive ranker is the builder and it orders by `b_i` alone, the kickback is a pure
constraint, set binding at `u_i = k·v_i`, and the searchers compete only over the residual
`r_i = (1 − k)·v_i`:

```
Symmetric BNE, n risk-neutral bidders, r_i iid U[0, r̄]  [G1, ch. 2]:
  utility of bidding as type z when true type is r:  (r − β(z)) · (z/r̄)^(n−1)
  FOC at z = r:  β'(r)·r^(n−1) + (n−1)·r^(n−2)·β(r) = (n−1)·r^(n−1)
  ⇒ d/dr [ r^(n−1)·β(r) ] = (n−1)·r^(n−1)
  ⇒ β(r) = r − (1/r^(n−1))·∫₀^r t^(n−1) dt = r·(n−1)/n
  b_i* = (1 − k)·v_i·(n−1)/n ,   u_i* = k·v_i ,   searcher keeps (1 − k)·v_i / n in expectation
```

Complete information (all `v_i` common knowledge): Bertrand logic — any `b` below the
second-highest residual is undercut, so `b_(1)* = r_(2) + ε` and, with `n` equal values,
`b* → (1 − k)·v`, searcher profit `→ 0`. `tests/test_fee_escalation.rs` reproduces this:
5 equal bidders raising `builder_share_bps` by 5 % per round converge to exactly
`builder_payment = 0.10·P`, `searcher_profit = 0`, with `user_kickback ≥ 0.90·P` in every
round and the next escalation rejected by the `≤ 10000 bps` bound [S12].

**User protection corollary.** In both variants `u_i ≥ k·v_i` is never bid below: the floor is
enforced by validation, not by equilibrium, so the bidding war only redistributes the
`(1 − k)` residual between searcher and builder. Whether real builders also weight the user
refund (which would move the equilibrium toward §2's first result) is `unknown`.

## 3. Latency vs. profit — cost of simulation delay on inclusion probability

Model (assumed): one inclusion opportunity per slot of length `T = 12 s` [E1]; the hint
arrives at a time uniform in `[0, T]`; the bundle must reach the builder before the slot's
cut-off; the searcher's end-to-end delay is `L = L_sim + L_net`. Then

```
Pr[include | L] = max(0, 1 − L/T)
E[profit](L)    = (1 − L/T)·π − c_sim           π = expected auction payoff from §2
∂E/∂L           = −π/T                           ⇒ marginal cost ≈ π / 12 000 per ms
```

Worked numbers with `π = 0.0229 ETH` (table in §1, assuming the bid is won at zero bribe):

| `L_sim`             | inclusion loss | expected cost (ETH) |
|---------------------|----------------|---------------------|
| 50 ms               | 0.42 %         | 0.000095            |
| 200 ms              | 1.67 %         | 0.000382            |
| 500 ms ([M4] cap)   | 4.17 %         | 0.000954            |

Decision rule: run a simulation step only if its accuracy gain `Δπ` exceeds `π·ΔL/T`;
a full `mev_simBundle` round trip at the [M4] 500 ms timeout is worth it only when it
changes the bid by more than ≈4 % of `π`. The relay-side auction itself is negligible on
this axis: merging 500 bundles measured 17.9 µs release-mode in `docs/executive-summary.md`
(≈1.5e-4 % of a slot). The builder's real cut-off distribution, and whether late bundles are
queued for the next slot rather than dropped, are `unknown`; the uniform-arrival model is
the stated assumption, not a measurement.
