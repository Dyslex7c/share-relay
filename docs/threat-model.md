# MEV-Share — Threat Model

Sources: [M1] flashbots/mev-share spec repo (`specs/`) — status: to-verify.
[M2] flashbots/mev-share-node — status: to-verify. Hint field sets and
default masks below are per user preference ([M1]); exact defaults: `unknown`.

## 1. Frontrunning & Sandwich Attacks
- **Attack:** A searcher sees full calldata + amount, front-runs the buy and
  back-runs the sell, sandwiching the user at worse execution prices.
- **Mitigation (relay-enforced):** Calldata/amount are disclosed only if the
  user opts in; default hints exclude them. A searcher that cannot see the
  trade direction/size cannot position a frontrun leg. Backrun-only ordering
  (user tx pinned first in every forwarded bundle) is enforced at forwarding.

## 2. Statistical Deanonymization
- **Attack:** Emitting contract addresses, 4-byte function selectors, or log
  topics narrows the user's intent to a handful of known strategies; combined
  with amount buckets, repeated events become linkable to one wallet/flow.
- **Mitigation (relay-enforced):** Hint sanitizer redacts or coarsens fields:
  selectors/addresses emitted only on explicit opt-in, amounts bucketed (not
  exact), and logs filtered to user-approved contracts/topics. Sanitizer is
  default-deny: any field not whitelisted by the user's hints is dropped.

## 3. Searcher Collusion & Bundle Probing
- **Attack:** Malicious searchers submit crafted bundles whose inclusion /
  revert behavior (or kickback deltas) probes hidden parameters, and collude
  across identities to aggregate the reconstructed trade.
- **Mitigation (relay-enforced):** Bundles are blind — they bind to the user
  tx by hash only, never to plaintext fields. Probing bundles that revert are
  not forwarded and leak no execution trace; auction outcomes reveal only
  win/lose, not the user tx's private state. Rate-limiting per searcher
  identity and simulation-only evaluation (no on-chain probing) bound the
  information gained per attempt. Collusion resistance beyond this: `unknown`.
