# ECR-004 Planning Requirements Checklist

**Purpose:** verify the planning package is complete enough for TASKS_READY review before implementation authorization.

## Specification and ownership

- [x] Purpose, dependencies and explicit non-goals are defined.
- [x] ECR-001 VerificationReceipt remains the single canonical independent verification record.
- [x] ActionReceipt remains executor-observed evidence and cannot self-verify.
- [x] UNKNOWN preservation, exact reconciliation binding and blind-retry prevention are explicit.
- [x] Conflicting verification remains visible; no last-write-wins behavior is allowed.
- [x] Checkpoints are exact-target requirements without capability, approval or authorization semantics.
- [x] Reconciliation does not fabricate ActionReceipt or mutate ECR-002 run-event truth.
- [x] Decision-grade mutable evidence requires immutable binding and freshness metadata where required.
- [x] Provider/browser/model/network acquisition and execution are outside v1.
- [x] Raw sensitive/private evidence persistence is outside v1 acceptance.
- [x] FR-001 through FR-045 are present and owned by tasks.
- [x] SC-001 through SC-012 are present and owned by tasks.
- [x] Persistence, migration, concurrency, corruption, reopen, bounds and typed errors have executable tasks.
- [x] ECR-001 and ECR-002 regression gates have executable tasks.
- [x] Traceability, analyze, convergence, review, exact-head merge and post-merge closure tasks exist.

## Data model and contract

- [x] ECR-004-specific IDs are typed, opaque and non-authoritative.
- [x] VerificationRequestV1 is construction input, not a competing truth record.
- [x] Aggregate states are closed and deterministic.
- [x] Reconciliation outcomes are effect_confirmed, no_effect_confirmed and still_unknown.
- [x] Retry disposition is closed and explicitly non-authoritative.
- [x] Sidecar journal body, sequence, digest and persistence ownership are specified.
- [x] Canonical journal truth is distinct from rebuildable indexes.
- [x] v1 count/byte/query bounds and machine-readable error classes are specified.

## Threat model

- [x] Executor self-verification is covered.
- [x] Wrong-target/cross-attempt evidence substitution is covered.
- [x] Conflict hiding is covered.
- [x] Absence-of-evidence/no-effect confusion is covered.
- [x] Blind retry and duplicate-effect threats are covered.
- [x] Mutable evidence and evidence-as-authority injection are covered.
- [x] Provenance rewrite is covered.
- [x] Journal corruption, projection poisoning and duplicate IDs are covered.
- [x] Resource exhaustion is covered.
- [x] Compromised verifier/capture is a documented boundary.
- [x] Raw sensitive-data leakage and verification-as-authorization bypass are covered.
- [x] Full-store hostile rewrite is outside the unprotected journal integrity claim.

## Constitution and authorization guard

- [x] G1 through G8 pass.
- [x] G9 is explicit PASS-N/A.
- [x] G10 passes subject to implementation-time dependency/license re-verification.
- [x] G11 is explicit PASS-N/A.
- [x] G12 through G15 pass.
- [x] This planning branch is not implementation authorization.
- [x] Implementation starts only after the package is canonical on main and the exact canonical planning head passes required ECR-001/ECR-002 regressions.
- [x] ECR-031 remains a separate blocked implementation slice and is not bypassed by this package.

## Result

PLANNING_CHECKLIST_PASS_CANDIDATE

This result authorizes analyze/review of the planning package only. It is not canonical TASKS_READY truth until analyze is clean and the planning package is merged under repository governance.
