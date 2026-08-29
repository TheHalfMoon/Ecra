# Research: ECR-004 Verification & Reconciliation

**Status:** DECISIONS_FROZEN_FOR_PLANNING  
**Base:** `f6d8eb6ff6a60aa0ad8a6f52686a62f12cd374b0`  
**Dependencies:** ECR-001/ECR-002 `CLOSED_CANONICAL`

## R1 — Reuse the canonical VerificationReceipt

**Decision:** ECR-004 reuses `ecra_core::VerificationReceipt`, `VerificationTarget`, `VerificationMethod`, `VerificationOutcome`, and `EvidenceRef` without defining a second verification result type.

**Why:** ECR-001 deliberately made `VerificationReceipt` the authoritative independent verification record and deliberately kept `Fact` free of a `verified` flag. A second receipt/result object would violate Constitution G1/G5.

**Rejected:**
- adding `verified: bool` to `Fact`, `ActionReceipt`, artifacts, or run state;
- creating `Ecr004VerificationReceipt` with parallel semantics;
- treating executor success as implicit `Verified`.

## R2 — Preserve ECR-002 execution truth unchanged

**Decision:** ECR-004 v1 does not add variants to the ECR-002 `RunEvent` v1 wire contract and does not mutate or fabricate `ActionReceipt` records. It reads exact `RunState`/attempt truth and stores verification/reconciliation records in an ECR-004-owned sidecar journal.

**Why:** ECR-002 is `CLOSED_CANONICAL`; its v1 run event/reducer/archive semantics are already dependency truth. Verification is orthogonal to execution truth. A sidecar avoids making old run-event readers reject new variants and avoids making verification overwrite provider-observed execution state.

**Rejected:**
- `RunEvent::VerificationRecorded` in ECR-002 v1;
- clearing unresolved attempts by rewriting ECR-002 state;
- synthesizing `ReceiptRecorded` after external reconciliation.

## R3 — New crate boundary: ecra-verify

**Decision:** implementation should add `crates/ecra-verify` depending on `ecra-core` and `ecra-run`. Pure verification/aggregation/reconciliation logic stays separate from local journal I/O. No browser/network/model/provider/process/policy crate is allowed.

**Why:** ECR-004 needs canonical domain types from ECR-001 and durable attempt/retry truth from ECR-002 while remaining independently auditable. A dedicated crate prevents verification logic from becoming an executor/provider adapter.

**Dependency posture:**
- `ecra-core`: required canonical targets/receipts/evidence;
- `ecra-run`: required `RunState`, retry/idempotency/recovery truth;
- `serde`, `serde_json`, `sha2`: already repository-approved primitives if required for strict records/digests;
- `rusqlite`: already accepted by ECR-002 and may be reused for a local append-only verification journal after exact dependency re-verification;
- no new external runtime/provider dependency is planned.

## R4 — Verification request is construction input, not new truth

**Decision:** define a strict `VerificationRequestV1` that pre-allocates the canonical `VerificationId`, binds one `VerificationTarget`, verifier `ActorId`, optional `PrincipalRef`, method, explicit evidence list, optional evaluated time, and bounded notes/rule metadata. The successful output is the canonical ECR-001 `VerificationReceipt`.

**Why:** requests need strict target/evidence binding, but only the receipt is verification truth.

**Rejected:** storing request status as a competing verification outcome.

## R5 — Decision-grade evidence rule

**Decision:** conclusive verification over mutable external state requires immutable evidence binding (`content_digest`, artifact/snapshot, receipt plus independently checked state, or deterministic computation input). If the rule requires freshness, `as_of`/explicit evaluation time must be present. Evidence without sufficient immutable binding can yield `Inconclusive` but not decision-grade `Verified`/`Rejected`.

**Why:** a URL/string locator can change after verification. Decision-grade claims need inspectable evidence identity.

**Boundary:** ECR-004 evaluates evidence metadata supplied to it; it does not fetch remote content. ECR-009/ECR-006/later adapters own acquisition and source-quality semantics.

## R6 — Method is not a trust score

**Decision:** preserve the ECR-001 method ordering as descriptive method classes. `StructuredExternalState` is not automatically true, and `IndependentModelJudgment` is not automatically false; every conclusive result still requires rule-specific evidence validation.

**Why:** method labels alone cannot prove independence, freshness, correctness, or target binding.

## R7 — Deterministic receipt aggregation

**Decision:** aggregate all immutable receipts for one exact target into a derived `VerificationAggregateViewV1`:

```text
Absent       no conclusive/inconclusive receipts
Verified     >=1 Verified, no Rejected
Rejected     >=1 Rejected, no Verified
Inconclusive no conclusive receipt, >=1 Inconclusive
Conflicted   >=1 Verified and >=1 Rejected
```

`NotEvaluated` never satisfies a checkpoint. All receipt IDs remain visible and deterministically ordered.

**Why:** no last-write-wins and no hidden disagreement.

## R8 — Critical-point verification checkpoints

**Decision:** define `VerificationCheckpointV1` as a bounded set of exact `VerificationTarget` requirements and accepted aggregate states. Evaluation returns a derived checkpoint view with every unsatisfied/conflicted target.

**Boundary:** checkpoints are requirements, not authority. They contain no grants, approvals, policy decisions, declassification, secrets, or executor handles.

## R9 — Reconciliation outcome is effect truth, not execution receipt

**Decision:** define append-only `ReconciliationRecordV1` bound to exact `RunId`, `ActionAttemptRef`, underlying `ActionRef`, and supporting canonical verification receipt IDs. v1 outcomes are:

- `effect_confirmed` — independent evidence confirms the attempted effect occurred;
- `no_effect_confirmed` — independent evidence confirms the attempted effect did not occur;
- `still_unknown` — evidence is insufficient/conflicted.

**Why:** this resolves the externally relevant effect claim without inventing provider response data.

## R10 — Retry semantics remain split from authorization

**Decision:** ECR-004 may compute a `RetryDispositionV1` from exact ECR-001 retry/idempotency semantics plus reconciliation outcome:

```text
effect_confirmed   -> duplicate_retry_blocked
still_unknown      -> reconciliation_required
no_effect_confirmed + safe -> semantically_retryable
no_effect_confirmed + requires_same_idempotency_key -> semantically_retryable_same_key
no_effect_confirmed + never_blind_retry -> still_requires_explicit_nonblind_path
```

This result is advisory runtime safety state only. It is not `CapabilityGrant`, approval, authorization lease, or executor command.

## R11 — Sidecar journal and integrity scope

**Decision:** persist only strict ECR-004 records/evidence references in a local append-only journal. Each entry carries sequence, previous digest, and canonical entry digest so substitution/corruption is detectable under normal local integrity assumptions.

**Claim boundary:** without ECR-031 protected anchoring, the journal MUST NOT be described as resistant to an attacker who can rewrite the entire store and recompute the chain. ECR-004 does not depend on ECR-031 and must remain useful with synthetic/non-sensitive fixtures now.

## R12 — Sensitive-data boundary

**Decision:** ECR-004 v1 acceptance persists only synthetic/non-sensitive verification metadata, IDs, digests, and bounded notes. Raw private/sensitive external evidence payloads remain out of the journal.

**Why:** ECR-031/ECR-025 gates are not dependencies of ECR-004. This slice must not smuggle sensitive persistence into an independently eligible path.

## R13 — Resource bounds

**Decision:** v1 planning will freeze small explicit limits for:
- evidence refs per request/receipt;
- receipts aggregated per target;
- checkpoint requirements;
- reconciliation supporting receipt IDs;
- notes/rule identifiers;
- journal entry size/count loaded per query.

All arithmetic is checked; oversized input fails before expensive materialization where practical.

## R14 — No donor/source-code adoption

ECR-004 planning is derived from canonical Ecra contracts and constitution. No external donor source code is adopted by this planning package. Any new dependency discovered during implementation requires the ordinary dependency/license/advisory gate before use.

## Open implementation questions resolved by tasks

1. Exact SQLite schema/indexes and migration version for the ECR-004 journal.
2. Exact v1 numeric limits after fixture sizing.
3. Whether `sha2`/`serde_jcs` are reached through existing canonical helpers or direct accepted dependencies.
4. Exact error-code enumeration while preserving machine-readable failure classes.

None of these questions changes the frozen semantic boundary above.