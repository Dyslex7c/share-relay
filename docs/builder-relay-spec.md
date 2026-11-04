Sources:
- [S8] [MEV-Share v0.1](https://github.com/flashbots/mev-share/blob/09d9ea87d4f907aa69ffd914802209ae8ea680a4/specs/bundles/v0.1.md), Request/Response Scheme, body, Bundle Composition, validity — verified.
- [S9] [mev-share-node builders.go](https://github.com/flashbots/mev-share-node/blob/67003f8e8700bef34444dda71dc18c4585e5c2f6/mevshare/builders.go), JSONRPCBuilderBackend.SendBundle, BuildersBackend.SendBundle, cleanBundle — verified.
- [S10] [Flashbots coinbase.transfer()](https://docs.flashbots.net/flashbots-auction/advanced/coinbase-payment), payment semantics and contract recipients — verified.
- [S11] [JSON-RPC specification](https://www.jsonrpc.org/specification), sections 4–5.1 — verified.
- [S12] This crate: `src/matchmaker/packer.rs:4–30`, `src/builder/dispatch.rs:14–24`, `src/privacy/sandwich_proof.rs:7–23`, `src/matchmaker/revert_guard.rs:8–12` — verified.
# Builder dispatch and packed bundle protocol
## Method and transport
`builder_sendBundle` support/schema: **unknown**; it is not defined by [S8] or invoked by [S9]. Do not assume builders accept it.
Use HTTP JSON-RPC `mev_sendBundle` for configured v0.1 builders; the legacy refund-recipient adapter uses `eth_sendBundle` with a different schema [S9].
## Packing schema (restricted non-reverting v0.1 profile [S8])
`Hex` denotes a prefixed hex string; transaction bytes must be signed and individually encoded, never an undelimited concatenation [S8, body].
```typescript
Bundle = { version: "v0.1", inclusion: { block: Hex, maxBlock?: Hex },
  body: ({ tx: Hex, canRevert: false } | { bundle: Bundle })[],
  validity?: { refund?: { bodyIdx: number, percent: number }[],
    refundConfig?: { address: Hex, percent: number }[] } }
Request = { jsonrpc: "2.0", id: string | number, method: "mev_sendBundle", params: [Bundle] }
```
Proposed relay packing policy: one outer Bundle per MegaBundle; each ordered winning bundle becomes a nested `{bundle: Bundle}` child [S8, S12].
Resolve user hash references privately into signed user transactions before dispatch; each child puts the user first and signed backruns afterward, preserving atomic composition [S8; S12 ordering guard].
Set outer/child inclusion to the selected block (hex quantity); upstream dispatch pins `block` and `maxBlock` to that block and strips privacy/metadata [S8, S9].
## Execution and settlement constraints
Relay policy: include each child wholly in order, reject reverts via `canRevert:false`, and re-simulate the packed sequence against builder state; isolated success is insufficient [S8; proposed policy].
Refund constraints use `validity.refund` body indices and integer percentages, not basis points; nested user `refundConfig` allocations sum to 100 [S8, validity].
Coinbase transfers execute inside signed transactions/contracts; account for fees and contract-recipient gas/reentrancy. The block coinbase recipient is not necessarily the builder operator [S10].
Internal MegaBundle totals aggregate `(builder_payment, user_refund)` with saturation; they neither execute transfers nor create extra RPC fields [S12].
## Acknowledgement and errors
Success: matching request `id`, `result:{bundleHash:Hex}`; no `error`. Failure: matching `id` (or null if unknown), `error:{code:integer,message:string,data?:any}`; no `result` [S8, S11]. Illustrative acknowledgement:
```json
{"jsonrpc":"2.0","id":1,"result":{"bundleHash":"0x1111111111111111111111111111111111111111111111111111111111111111"}}
```
Handle transport failures and RPC errors separately [S9]; -32601 means unsupported method, -32602 invalid params, -32603 internal error; builder-specific codes/retry guarantees: `unknown` [S11]. Proposed policy: retain per-builder status, retry only transient failures while inclusion remains valid, never treat acknowledgement as settlement.
Implementation gap: this crate only stores URLs, concatenates raw bytes, and aggregates totals; JSON encoding, HTTP dispatch, packed simulation, acknowledgement validation and retry policy remain unimplemented [S12].
