Sources:
- [S8] [MEV-Share v0.1](https://github.com/flashbots/mev-share/blob/09d9ea87d4f907aa69ffd914802209ae8ea680a4/specs/bundles/v0.1.md), validity/refund/refundConfig — verified.
- [S13] [Ethereum JSON-RPC](https://ethereum.org/en/developers/docs/apis/json-rpc/), eth_chainId, eth_getTransactionReceipt, eth_getTransactionByHash, eth_getBalance, eth_getBlockByNumber — verified.
- [S14] [Geth built-in tracers](https://geth.ethereum.org/docs/developers/evm-tracing/built-in-tracers), callTracer and prestateTracer — verified.
- [S15] [Solidity contracts](https://docs.soliditylang.org/en/latest/contracts.html#receive-ether-function), Receive Ether Function/Fallback Function — verified.
- [S16] This crate: `src/matchmaker/kickback.rs:4–17` — verified (in-memory accounting only).
# User MEV refund integration guide
## Scope and routing
This is a proposed wallet integration procedure, not a claim of shipped MetaMask or Rabby support; their automatic MEV-refund attribution: `unknown`.
Before signing, confirm chain ID, refund recipient(s) and agreed allocation; submit privately and retain the signed transaction hash and returned bundle hash. Never substitute a wallet service address silently (integration policy).
Use `validity.refund` for required refund percentages and `validity.refundConfig` for recipient addresses and allocation percentages summing to 100; do not assume every refund goes to the signer [S8].
## Verification and wallet crediting (integration policy using [S13, S14])
1. Confirm `eth_chainId` and the saved recipient; obtain the settlement transaction hash plus its claimed association with the original transaction from the relay/builder. Discovery endpoint/schema: `unknown`.
2. Fetch receipts for original and settlement transactions; require successful `status`, and match their block hashes to canonical blocks. A relay acknowledgement or simulation is not proof of payment [S13].
3. For a direct top-level ETH payment, check `eth_getTransactionByHash` recipient and `value` against the successful receipt; reconcile against the expected payer and saved routing [S13].
4. For internal ETH payments, use a trusted backend's `debug_traceTransaction` with `callTracer`; check actual value-bearing CALL recipients, excluding reverted frames/ancestors and DELEGATECALL context values. Provider trace availability: `unknown` [S14].
5. Receipts do not enumerate all internal ETH transfers; `eth_getBalance` across blocks is only a cross-check because swaps, gas and unrelated transfers also change balances. Reconcile transaction-level state differences where needed [S13, S14].
6. Attribute verified payment to the saved user transaction only with supporting bundle/settlement evidence; an unrelated incoming payment is not automatically a MEV refund (integration policy).
7. Deduplicate by chain, settlement transaction and transfer location; reconcile traces/logs without counting both. Show provisional/finalized credit, refresh on-chain balances, and roll back attribution on reorgs. Use the `finalized` block tag; confirmation-count policy: `unknown` [S13].
## Direct transfers versus smart contract receivers
A direct ETH payment credits the recipient's on-chain balance; the wallet refreshes/displays it rather than issuing an additional credit [S13].
A configured contract receiver must accept the settlement call: plain empty-calldata ETH calls need a payable `receive` or fallback; test gas/revert behavior before routing refunds there [S8, S15].
For a proposed escrow/receiver, audit beneficiary mapping, withdrawal authorization and reentrancy protection; verify trusted-emitter logs against its actual ABI and state. No standard MEV refund event or claim ABI is specified here.
Contract receipt is not necessarily spendable wallet ETH: show escrow entitlement separately until withdrawal succeeds; never count both deposit and withdrawal as new refunds (integration policy).
The crate's `credit_refund` accumulates locally and `claim_refund` removes/returns a balance; neither sends ETH nor verifies settlement. Durable payouts, wallet hooks and an on-chain receiver are unimplemented [S16].
## Slippage recovery math (illustrative accounting model, not a guarantee)
Let P be attributable refundable backrun surplus after the agreed builder-model deductions; r the agreed refund fraction; a the user's recipient fraction; C any additional user-paid claim/settlement cost, not already deducted from P.
Compute in integer wei: `R_pool = floor(P * r)` and `R_user = floor(R_pool * a)`; reconcile actual payout and rounding, not gross arbitrage profit alone [S8, validity; illustrative model].
Let S be execution shortfall versus a recorded quote, valued in the same asset at a recorded price/time; do not equate configured slippage tolerance with realized loss.
`Remaining_loss = max(0, S + C - R_user)`; for positive S, `Recovered_fraction = min(S, max(0, R_user - C)) / S` (derived accounting identities).
Illustrative ETH example: `S=0.010`, `P=0.008`, agreed `r=90%`, `a=100%`, `C=0.0002`; these are assumptions, not measured returns or default guarantees.
Then `R_user=0.0072 ETH`; net recovery `0.0070 ETH`; remaining loss `0.0030 ETH`; recovered fraction `70%` (arithmetic from the stated inputs).
A backrun refund may offset losses, but cannot promise full recovery: available surplus, inclusion and actual settlement remain conditional; verify P under the builder's model rather than trusting a displayed estimate [S8].
