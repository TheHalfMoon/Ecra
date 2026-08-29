# ecra-verify

`ecra-verify` is the ECR-004 trusted verification/reconciliation crate.

Phase 1 establishes only the dependency, unsafe-code, CI and architecture boundary. Semantic verification is not claimed until the owning tasks and exact-head gates complete.

## Boundaries

- reuse ECR-001 `VerificationReceipt`, `VerificationTarget` and `EvidenceRef`;
- read ECR-002 `RunState`/attempt truth without mutating run state or events;
- never fabricate `ActionReceipt`;
- never authorize, schedule or execute provider work;
- never turn `no_effect_confirmed` into same-run resume/retry permission;
- isolate local sidecar journal I/O from pure verification logic;
- persist synthetic/non-sensitive metadata/references/digests only in v1 acceptance;
- keep Ecra-authored Rust under `#![forbid(unsafe_code)]`.

The journal integrity design is a corruption/substitution-detection mechanism under its stated local assumptions. It is not a hostile full-store tamper-resistance claim.
