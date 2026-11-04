# SSE Event Stream Spec

Sources: [M6] flashbots/mev-share `specs/events/v0.1.md` — verified (SSE framing,
schema, `:ping` heartbeat). [M7] flashbots/mev-share-node `mevshare/hints.go`,
`mevshare/types.go` — verified (hint implication chain, JSON field casing).
Endpoint URL `https://relay.mev-share.net/api/v1/events` is taken from the project
context, not verified against [M6]/[M7] (`unknown`).

## Transport & framing

Plain HTTP GET; response is `text/event-stream`. Each event is one SSE `data:` line
carrying the JSON object, terminated by a blank line:

```
data: {"hash":"0x...","logs":[],"txs":[{"to":"0x..."}]}

```

If no event was sent in the last 15 seconds, the server emits `:ping` ([M6]):

```
:ping
```

Reconnect by re-issuing the GET; the v0.1 stream is live-only (no replay, no
`id:`/`Last-Event-ID` semantics defined in [M6]).

## Event JSON schema

```typescript
{
    hash: string,            // hex tx or bundle hash
    logs?: CleanLog[],       // {address, topics[], data}
    txs: Array<{
        to?: string,              // recipient address
        functionSelector?: string, // 4-byte selector
        callData?: string,         // full calldata
    }>,
}
```

`logs` and `txs` are always present in the node implementation ([M7] `Hint`), even when
empty. A transaction event has one `txs` entry; a bundle event may have several.
Optional `gasUsed`/`mevGasPrice` may also appear depending on node config ([M7]).

Hint implication chain enforced by the node ([M7] `extractTxHints`): sharing
`function_selector` or `calldata` implies `contract_address`; sharing `calldata` implies
`function_selector`. If `hash` is not among the hints, no hint is emitted at all.

## Hash-only event

Only the tx/bundle hash; `to`, selector, and calldata are withheld:

```
data: {"hash":"0x3232323232323232323232323232323232323232323232323232323232323232","logs":[],"txs":[]}

```

## Logs-only event

Hash plus the `Swap` log; no `txs` entries (no contract/selector/calldata hints):

```
data: {"hash":"0x3232323232323232323232323232323232323232323232323232323232323232","logs":[{"address":"0x8ad599c3a0ff1de082011efddc58f1908eb6e6d8","topics":["0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822"],"data":"0x"}],"txs":[]}

```

## Full-calldata event

`to`, 4-byte selector, and full calldata present together ([M7] implication chain):

```
data: {"hash":"0x3232323232323232323232323232323232323232323232323232323232323232","logs":[],"txs":[{"to":"0x7a250d5630b4cf539739df2c5dacb4c659f2488d","functionSelector":"0x38ed1739","callData":"0x38ed173900000000000000000000000000000000000000000000000000000000000000000000"}]}

```

Note: this crate's internal `SanitizedEvent` is a different, flatter shape
(`mev-share/src/hint/sanitizer.rs`) and is not wire-compatible with this SSE schema;
an adapter is required before publishing to searchers.
