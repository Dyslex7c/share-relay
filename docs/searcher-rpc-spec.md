# Searcher JSON-RPC Spec (MEV-Share v0.1)

Sources: [M3] flashbots/mev-share `specs/bundles/v0.1.md` — verified (bundles v0.1).
[M4] flashbots/mev-share-node `mevshare/types.go`, `mevshare/api.go` — verified (method
names, `mev_simBundle` request/response, error strings). [M5] same repo
`jsonrpcserver/jsonrpc_server.go` — verified (error codes). v0.1 defines only
`mev_sendBundle`; `mev_simBundle` is a node implementation endpoint, not part of [M3].

## Methods

```typescript
mev_sendBundle(params: [MevSendBundleParams]) -> { bundleHash: string }
mev_simBundle(params: [MevSendBundleParams, SimAuxArgs?]) -> SimMevBundleResponse
```

All quantities are `0x`-prefixed hex; `params` is always a positional array.

## mev_sendBundle request

`version` (required, `"v0.1"`); `inclusion` (`block` required, `maxBlock` optional);
`body` (array of `{hash}` | `{tx, canRevert}` | `{bundle}`); `validity` optional
(`refund: [{bodyIdx, percent}]`, `refundConfig: [{address, percent}]`); `privacy`
optional (`hints: ["calldata"|"contract_address"|"logs"|"function_selector"|"hash"|"tx_hash"]`,
`builders: [string]`). Omitting hints shares nothing ([M3]).

```json
{"jsonrpc":"2.0","id":1,"method":"mev_sendBundle","params":[{
  "version":"v0.1",
  "inclusion":{"block":"0x463e","maxBlock":"0x4648"},
  "body":[
    {"hash":"0x3232323232323232323232323232323232323232323232323232323232323232"},
    {"tx":"0x020424242424212312312312301230123012301230123012301230123012032424242123123124242421231231242424212312312424242123123124242421231231","canRevert":false}
  ],
  "validity":{"refund":[{"bodyIdx":0,"percent":90}],
    "refundConfig":[{"address":"0x8EC1237b1E80A6adf191F40D4b7D095E21cdb18f","percent":100}]},
  "privacy":{"hints":["hash","contract_address","logs"],
    "builders":["0x0000000000000000000000000000000000000000"]}
}]}
```

```json
{"jsonrpc":"2.0","id":1,"result":{"bundleHash":"0x4567456745674567456745674567456745674567456745674567456745674567"}}
```

The `bundleHash` covers the list of bundle bodies, or the single bundle body ([M3]).

## mev_simBundle

Same `MevSendBundleParams` as above, optionally followed by `SimAuxArgs`
(`parentBlock`, `blockNumber`, `coinbase`, `timestamp`, `gasLimit`, `baseFee`, `timeout`).
Simulation runs with a 500 ms timeout in [M4].

```json
{"jsonrpc":"2.0","id":2,"method":"mev_simBundle","params":[{
  "version":"v0.1","inclusion":{"block":"0x463e"},
  "body":[{"hash":"0x3232323232323232323232323232323232323232323232323232323232323232"},
  {"tx":"0x020424242424212312312312301230123012301230123012301230123012032424242123123124242421231231242424212312312424242123123124242421231231","canRevert":false}]
},{"blockNumber":"0x463e","timestamp":"0x64","gasLimit":"0x1c9c380"}]}
```

Response ([M4] `SimMevBundleResponse`): `success` (bool), `error`, `stateBlock`,
`mevGasPrice`, `profit` (profit delta), `refundableValue`, `gasUsed`, `logs`, `execError`,
`revert`.

```json
{"jsonrpc":"2.0","id":2,"result":{"success":true,"stateBlock":"0x463d",
  "mevGasPrice":"0x3b9aca00","profit":"0xde0b6b3a7640000","refundableValue":"0xc7d713b49da0000",
  "gasUsed":"0x33450","logs":[]}}
```

Simulation failure still returns a result with `success:false` and `error`/`execError`;
transport/RPC failure returns a JSON-RPC error instead.

## Error codes ([M5])

| Code | Message | When |
|-|-|-|
| -32700 | parse error / `invalid jsonrpc version` / `invalid id type` | body decode or version/id check |
| -32600 | `x-flashbots-origin header is too long` | origin header > 255 chars |
| -32601 | `method not found` | unknown method |
| -32603 | marshal error text | result could not be serialized |
| -32000 | application error text | any handler error, including bad params |

Application strings from [M4] include `invalid inclusion`, `invalid bundle body size`,
`invalid bundle body`, `backrun not found`, `backrun invalid bundle`,
`backrun invalid inclusion`, `mev-share service error`. Invalid `params` also surface as
-32000, not -32602, in this server ([M5]). Code -32602 is defined but never emitted.

```json
{"jsonrpc":"2.0","id":1,"error":{"code":-32000,"message":"backrun not found"}}
```

`privacy.wantRefund` and `special_logs`/`default_logs` hints appear in [M4] but are not in
the v0.1 spec; treat them as implementation extensions.
