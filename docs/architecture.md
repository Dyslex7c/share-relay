# MEV-Share Matchmaker — 4-Layer Architecture

Sources: [M1] flashbots/mev-share spec repo (`specs/`) — status: to-verify.
[M2] flashbots/mev-share-node — status: to-verify. Method names below are
as recalled from [M1]; exact wire fields are `unknown` until verified.

## Layers
1. **Ingestion** — user JSON-RPC (`eth_sendPrivateTransaction`-style, name per [M1]: to-verify).
   Accepts a signed tx plus hint preferences (which fields may be disclosed) and a
   builder/validity policy. Nothing leaves this layer unsanitized.
2. **Event Stream** — SSE multiplexer. Hint sanitizer redacts tx fields down to the
   user-permitted set (e.g. `hash`, `logs`, `calldata` subsets — set per [M1]) and
   fans one event out to N searcher subscribers.
3. **Bundle Collector** — searcher JSON-RPC (`mev_sendBundle`-style, per [M1]: to-verify).
   Accepts *blind* backrun bundles that reference the user tx by hash only.
4. **Simulation & Forwarding** — merges user tx + candidate backruns, resolves
   conflicts (same-slot competing bundles), runs the blind auction, validates
   searcher kickback (refund) against the user's required share, and dispatches
   the winning bundle to builders. Kickback percentage default: `unknown`.

## Lifecycle of a privatized trade
```
User ──signed tx + hints──▶ [1 Ingestion]
                                 │ sanitized hint
                                 ▼
Searchers ◀──SSE event──── [2 Event Stream]
    │ blind backrun bundle (by tx hash)
    ▼
[3 Bundle Collector] ──▶ [4 Sim & Forward] ──▶ Builder ──▶ Block
                              │ auction + kickback check
                              └──▶ refund to User (on inclusion)
```
