Sources:
- [S12] This crate: `src/lib.rs:1-8`; `src/hint/sse.rs:10-24`; `src/orchestrator.rs:5-31`; `src/privacy/sandwich_proof.rs:7-23`; `src/matchmaker/revert_guard.rs:7-12`; `src/hint/sanitizer.rs:14-23`; `src/hint/types.rs:13-21`; `src/privacy/encrypted.rs:4-11`; `Cargo.toml:9-14`; `tests/test_sanitizer.rs:17-38`; `tests/test_sandwich_proof.rs:11-25` — verified.
- [S17] [Tokio broadcast::channel](https://docs.rs/tokio/latest/tokio/sync/broadcast/fn.channel.html), capacity/preallocation/power-of-two sizing and independently lagging receivers — verified.
# Security audit — MEV-Share relay implementation
## Method and scope
Audited the current public Rust library and tests only; no HTTP/SSE server binary exists, so production transport findings are conditional [S12]. `Forbid` blocks are hostile-input tests of a proposed control, not evidence that the control exists.
## DoS findings
`Finding SA-DOS-1 (high; conditional transport DoS).` `SseBroadcaster::subscribe` is unbounded and needs no authorization; SSE endpoints must not expose it directly [S12].
`Finding SA-DOS-2 (medium; capacity model unclear).` The orchestrator capacity is 1024, but `bytes = 1.5` is unused and no byte-size adapter exists; each retained event's size is therefore `unknown` [S12].
Forbid: expose authenticated admission with global/per-IP/per-searcher quotas, connection/header limits and expirations; cap event/message bytes, reject oversize events, and skip/disconnect lagging receivers [S17].
`Finding SA-DOS-3 (high; input-suppression availability risk).` Publishing fails with no subscribers, so attack pressure need not exhaust capacity to make ingestion unavailable [S12].
Forbid: define bounded live traffic for publication policy; retain a bounded overflow policy or drop without returning subscriber errors to user ingestion [S12].
`Finding SA-DOS-4 (unsupported goal).` There is no pending bundle queue/storage at all [S12].
Forbid: implement one bounded queue with authenticated submitter quotas, payload/size cost, expiry/replacement, pressure thresholds and deterministic overflow rejection [S12].
Budget targets: maximum subscriptions, retained bytes and pending entries — `unknown` until production bytes/rate limits are measured [S17].
## Sandwich prevention
`verify_strict_backrun_ordering` checks only that item 0 equals a supplied `TxHashReference` and subsequent items are `SignedTransaction`; it is not cryptographic verification and does not prove the hash identifies the actual user transaction [S12].
`Finding SA-BYPASS-1 (critical).` Ordering, validation, simulation, merging and revert filtering are separate functions, while the orchestrator only sanitizes/publishes; no path enforces them before dispatch [S12].
Forbid: authenticate/authorize submission, then require validation, strict ordering and packed simulation/revert checks in one mandatory dispatch pipeline [S12].
Reviewer note SA-BYPASS-2: `canRevert: false` is not in this enum and is assigned elsewhere in the builder path, so relay-local failure filtering is not equivalent to builder inclusion safety [S12].
## Calldata confidentiality
With calldata masking off, the sanitizer sets only optional fields and never clones `tx.data`; the selector is also omitted unless explicitly enabled [S12].
With calldata enabled, full calldata is intentionally disclosed. With only the selector enabled, exactly four bytes are exposed; do not relabel that as zero-knowledge. “Completely unrecoverable” is false as a blanket claim: metadata/logs/hash correlation is outside this in-process masking guarantee [S12].
SA-PII-1 (high; public fields). Public `PrivateTransaction.data`, `tx.hints` and `SanitizedEvent` mean arbitrary callers can leak or log raw fields, and there is no cargo-audited redaction gate [S12].
SA-PII-2 (medium; hint semantics). Defaults are maximum-privacy, but the four booleans are independent; case-by-case opt-ins are easy to misread as privacy-preserving while still revealing selectors or destinations [S12].
SA-PII-3 (medium; logs can leak). Sanitizer inserts exactly the caller-provided topics when logs sharing is enabled, so masking is topic-equality rather than content-scrubbing [S12].
SA-PII-4 (low; current-scope caveat). No secret provider ever reads raw private transactions outside unit tests, and no end-to-end example currently wires the SSE/HTTP transport [S12].
## Checked enforcement points and current algorithm
Ordering is checked at bundle-item equality only, then sorted/merged and separately filtered for user-positive balances; return type is an error string rather than a typed security result [S12].
Hard guarantee today: an opted-in field can be disclosed; nothing more is proven by tests. Unsupported or deferred: signatures, target transaction authentication, builder-side non-revert inclusion and calldata non-recoverability [S12].
Any test claiming index-0 enforcement must explicitly call `verify_strict_backrun_ordering`; existing positive tests assert literal order but do not verify a signature [S12].
## Immediate remediation checklist
1. Rate-limit and authenticate SSE subscription; cap retained bytes and pending bundles; make strict ordering/validation mandatory before dispatch and treat bundled `canRevert` semantics explicitly [S12].
2. Redact public raw transaction fields or confine them behind a narrow sanitizer boundary; add adversarial tests for malformed bundles, log leakage, oversize calldata and griefing queues before production claims [S12].
