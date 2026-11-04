# Share-Relay

A pure-Rust, memory-safe implementation of an MEV-Share searcher bundle relay, matchmaker, and privacy engine.

`share-relay` implements the core layers of the Flashbots MEV-Share protocol: private transaction ingestion, hint sanitization with selective disclosure, blind bundle intake, auction matchmaking, and block builder dispatch.

```
       +-------------------------------------------------------------+
       |                  Private Transaction Ingestion              |
       +-------------------------------------------------------------+
                                      |
                                      v
       +-------------------------------------------------------------+
       |               Hint Sanitizer (Selective Disclosure)         |
       |               - Hash-only default                           |
       |               - Permitted fields (logs, calldata, to)       |
       +-------------------------------------------------------------+
                                      |
                                      v
       +-------------------------------------------------------------+
       |                 SSE Multiplexer / Fanout Stream             |
       +-------------------------------------------------------------+
                                      |
                     +----------------+----------------+
                     |                                 |
                     v                                 v
             [Searcher Bot A]                  [Searcher Bot B]
                     |                                 |
                     +----------------+----------------+
                                      |
                                      v
       +-------------------------------------------------------------+
       |                    Blind Bundle Intake                      |
       |                    - Structural validation                  |
       |                    - Minimum kickback threshold             |
       +-------------------------------------------------------------+
                                      |
                                      v
       +-------------------------------------------------------------+
       |                    Privacy & Safety Guards                  |
       |                    - Strict index-0 anti-sandwich           |
       |                    - Nonce/target replay prevention         |
       +-------------------------------------------------------------+
                                      |
                                      v
       +-------------------------------------------------------------+
       |                   Matchmaker Auction Engine                 |
       |                   - Multi-target conflict resolution        |
       |                   - Gas / kickback bundle scoring           |
       |                   - Revert guard simulation                 |
       +-------------------------------------------------------------+
                                      |
                                      v
       +-------------------------------------------------------------+
       |               Builder Dispatch & Settlement                 |
       |               - Execution payload formatting                |
       |               - Kickback refund accounting                  |
       +-------------------------------------------------------------+
```

---

## Key Highlights

* **Pure Rust & Zero Unsafe Code:** Built with `#![forbid(unsafe_code)]` at the crate root. No `unwrap`, `expect`, or `panic!` across all core library modules.
* **Sub-Microsecond Hint Sanitization:** Redacts sensitive transaction fields down to user-specified hint preferences (hash, address, logs, function selector, or calldata) in **12 ns** (release).
* **High-Throughput Matchmaking:** Resolves multi-target searcher bundle conflicts, scores bids, and merges non-conflicting bundles across 500 candidates in **17.9 µs** (release).
* **Anti-Sandwich by Construction:** Guarantees target transactions are pinned strictly at index 0 followed exclusively by backrun transactions, rejecting frontrun attempts and sandwich attacks before execution.
* **Deterministic Kickback Accounting:** Tracks accumulated refunds and builder payment distributions in an atomic ledger using overflow-free integer wei arithmetic.

---

## Architecture Overview

The crate is organized into modular subsystems:

| Module | Path | Description |
|---|---|---|
| **`hint`** | `src/hint/` | Private transaction parsing, selective disclosure sanitization, and SSE event streaming. |
| **`bundle`** | `src/bundle/` | Searcher registration, structural bundle validation, and backrun payload assembly. |
| **`privacy`** | `src/privacy/` | Replay cache for target transactions and anti-sandwich structure verification. |
| **`matchmaker`** | `src/matchmaker/` | Bundle scoring, conflict resolution merger, revert filter, and user refund ledger. |
| **`builder`** | `src/builder/` | Block builder endpoint registry and execution payload formatting. |
| **`searcher`** | `src/searcher/` | Reference searcher implementations: optimal trade sizing, listener, bidder, and arbitrage pathfinder. |
| **`sim`** | `src/sim/` | Market simulation harness for constant-product pools and auction rounds. |

---

## Measured Benchmarks

Measurements conducted on Apple Silicon (8-core):

| Operation | Debug | Release |
|---|---|---|
| Hint sanitization (hash-only default) | 71 ns | **12 ns** |
| Hint sanitization (all 5 hints enabled) | 183 ns | **122 ns** |
| Merge 500 bundles across 50 targets | 174.8 µs | **17.9 µs** |
| Kickback split (1.0 ETH @ 9000 bps) | Exact (0.9 ETH user / 0.1 ETH builder) | Exact |

---

## Verification & Test Suite

The project includes 26 distinct test suites covering unit logic, integration tests, concurrency stress tests, and end-to-end auction flows:

```bash
cargo test
```

### Test Coverage Highlights
* **`test_sandwich_proof`**: Verifies that any bundle attempting to insert a transaction before the target transaction or sandwich the user is rejected.
* **`test_auction_bench`**: Tests rapid resolution of 500 concurrent bids across 50 targets within a 20 ms latency budget.
* **`test_searcher_concurrency`**: Stress-tests 100 concurrent searchers racing to one winner in sub-5ms latency.
* **`test_sanitizer`**: Validates zero-leakage masking of raw calldata and addresses under default hint profiles.
* **`test_revert_guard`**: Ensures non-profitable or reverting bundles are pruned prior to builder dispatch.
* **`test_kickback`**: Validates atomic accumulation and one-time claim semantics of user refund balances.

---

## Documentation

Comprehensive design specifications and threat models are available in the `docs/` directory:

* [`docs/architecture.md`](docs/architecture.md): 4-layer architecture specification.
* [`docs/threat-model.md`](docs/threat-model.md): Attack vectors, information leakage, and relay mitigations.
* [`docs/security-audit.md`](docs/security-audit.md): Security analysis, audit findings, and production deployment recommendations.
* [`docs/builder-relay-spec.md`](docs/builder-relay-spec.md): Builder payload schemas and coinbase payment semantics.
* [`docs/searcher-strategy.md`](docs/searcher-strategy.md): Closed-form arbitrage sizing and bidding equilibrium.
* [`docs/kickback-economics.md`](docs/kickback-economics.md): User refund mechanisms and incentive alignment.
* [`docs/sse-stream-spec.md`](docs/sse-stream-spec.md): Event stream schemas and filtering rules.
* [`docs/searcher-rpc-spec.md`](docs/searcher-rpc-spec.md): Searcher JSON-RPC bundle submission API.

---

## License

Licensed under either of Apache License, Version 2.0 or MIT License at your option.
