# MEV-Share — Kickback Economics

Sources: [M1] flashbots/mev-share spec repo (`specs/`) — status: to-verify.
The 90% default refund below is an assumed parameter; the actual mev-share
default is `unknown` pending [M1]. All figures are illustrative.

## Searcher value formula
```
Extracted_MEV = Profit - Gas_Cost - User_Kickback - Builder_Payment
```
`Profit` is gross arbitrage value. A bundle is rational iff `Extracted_MEV > 0`.

## Auction mechanism: first-price sealed bid
- Bids are blind (bundles bind by tx hash). Each bid is its committed
  user kickback + builder payment; rivals cannot see it.
- The relay simulates each bundle, drops reverts, forwards the highest valid
  payment. Winner pays their own bid (first-price).
- Searchers shade bids; overpaying wins but can erase profit. Tie-break: `unknown`.

## User refund guarantee
- `User_Kickback >= refund_pct * (Profit - Gas_Cost)` with `refund_pct = 90%`
  default, paid to the original tx signer on inclusion.
- A bundle below the user's required share is rejected before dispatch,
  regardless of total bid.

## Numerical example (1.0 ETH arbitrage)
```
Profit           = 1.000 ETH
Gas_Cost         = 0.050 ETH
Net surplus      = 0.950 ETH
User_Kickback    = 0.855 ETH  (90% of net surplus, to tx signer)
Builder_Payment  = 0.060 ETH
Extracted_MEV    = 1.000 - 0.050 - 0.855 - 0.060 = 0.035 ETH (searcher keeps)
```
