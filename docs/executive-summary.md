Sources:
- [S12] This crate: census of `mev-share/` — 26 Rust files / 493 lines under `src`, 17 integration test files (28 tests), 8 docs; `#![forbid(unsafe_code)]` at `src/lib.rs:1`; no `unwrap`/`expect`/`panic!` in `src` — verified by grep/find.
- [S18] Measurement harness: standalone bench crate kept outside the repo, 200k-iteration sanitize loop and 2k-iteration merge loop timed with `Instant`, inputs generated in-crate (500 candidates over 50 targets, 1.0 ETH at 9000 bps); run on 8-core macOS 15.7.3 in both debug and `--release` — figures below are from that single run, not a percentile study.
# Executive summary — MEV-Share Searcher Bundle Relay
## What was built
A pure-Rust library modelling the four MEV-Share layers: ingestion (`hint::tx`), sanitization and SSE fan-out (`hint::sanitizer`, `hint::sse`), blind bundle intake (`bundle::item|types|validator|searcher`), and matchmaking (`matchmaker::score|merger|revert_guard|kickback|sim|packer`). Privacy guards: strict index-0 target ordering (`privacy::sandwich_proof`) and a replay cache (`privacy::replay`). `MatchmakerEngine` currently wires ingestion → sanitize → broadcast only.
## Benchmarks (measured) [S18]
| Operation | debug | release |
|-|-|-|
| Hint sanitization, hash-only default | 71 ns | 12 ns |
| Hint sanitization, all five hints enabled | 183 ns | 122 ns |
| Merge 500 bundles across 50 targets | 174.8 µs | 17.9 µs |
| Split of 1.0 ETH at 9000 bps | user 0.9 / builder 0.1 ETH, exact | identical |
Sanitization is sub-microsecond, so the sub-millisecond claim holds with roughly four orders of margin; conflict resolution sits 115x under the 20 ms test budget in debug and 1445x in release.
Kickback is threshold-enforced, not constant-enforced: `validate_bundle_structure` rejects `kickback_bps < min_kickback_bps`, and that minimum is a caller argument. 90% (9000 bps) is the tested policy value, not a built-in relay default. The split itself is overflow-free integer wei math and is exact for 1 ETH.
Caveat: single-host wall-clock figures. The 20 ms assertion in `tests/test_auction_bench.rs` retains large headroom but is environment-dependent and could flake on a loaded CI runner; there is no criterion harness and no percentile data.
## Not implemented (production blockers)
No HTTP, JSON-RPC or SSE server exists, so nothing is network-reachable, and `SseBroadcaster::subscribe` is unauthenticated and unbounded. There is no cryptography: `privacy::encrypted::EncryptedBundle` is a plain data struct with no cipher, key exchange or AEAD behind it. There is no EVM: `simulate_bundle_dry_run` does arithmetic on a caller-supplied `target_value` and always returns `success: true` with `gas_used: 0`, so revert filtering cannot catch a real revert. There is no builder dispatch: `BuilderDispatcher` only stores URLs and `format_mev_share_bundle_payload` concatenates raw bytes, which is not the `mev_sendBundle` JSON wire format. Validation, ordering, simulation, merge and revert-guard are independent functions with no enforced pipeline, and the kickback ledger is in-memory with no durable payout. Findings and severities are in `docs/security-audit.md`.
## Deployment recommendations
1. Do not route real private order flow to this code. It is a design and measurement prototype carrying one critical and two high audit findings; treat it as a specification harness.
2. Build the transport layer before anything else: authenticated SSE with per-identity quotas, bounded subscriber counts, byte-size caps and a bounded pending-bundle queue with deterministic overflow rejection.
3. Replace the arithmetic simulator with real execution. Auction ranking, revert guarding and kickback accounting are only meaningful once `success`, `gas_used` and profit come from an EVM rather than from inputs the caller chooses.
4. Collapse the guards into one mandatory dispatch path (validate → strict ordering → simulate → revert-guard → merge → pack → dispatch) so no caller can reach a builder while skipping a check.
5. Make the kickback floor explicit policy configuration with a documented default and an audited minimum, instead of a per-call argument; target the upstream v0.1 `validity.refund` / `refundConfig` percentage mechanism for wire compatibility.
6. Before claiming interoperability, conform to the documented `mev_sendBundle` v0.1 schema and the real SSE event shape (`hash`, `logs`, `txs[]`); the crate's flat `SanitizedEvent` is deliberately not wire-compatible and needs an adapter.
7. Re-benchmark under criterion with percentiles on representative hardware before publishing any latency or anonymity figure, and re-run the origin-leakage and sandwich tests against the real transport rather than the in-process broadcaster.
