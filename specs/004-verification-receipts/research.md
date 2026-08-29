# Research: ECR-004 Verification & Reconciliation

**Status:** IMPLEMENTATION_DEPENDENCIES_REVIEWED  
**Canonical implementation base:** `4fb61f8b41267983fc460c666fddd7781d91653c`  
**Dependencies:** ECR-001/ECR-002 `CLOSED_CANONICAL`  
**Implementation branch:** `004-verification-receipts-impl`

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
- `RunEvent::VerificationRecorded` or a reconciliation-resolution event in ECR-002 v1;
- clearing unresolved attempts by rewriting ECR-002 state;
- synthesizing `ReceiptRecorded` after external reconciliation;
- weakening `RunResumed`, `ExecutionCompleted`, or blind-retry guards after reconciliation.

## R3 — New crate boundary: ecra-verify

**Decision:** implementation adds `crates/ecra-verify` depending only on `ecra-core`, `ecra-run`, and the exact already-locked dependency set accepted by T001 below. Pure verification/aggregation/reconciliation logic stays separate from local journal I/O. No browser/network/model/provider/process/policy crate is allowed.

**Why:** ECR-004 needs canonical domain types from ECR-001 and durable attempt/retry truth from ECR-002 while remaining independently auditable. A dedicated crate prevents verification logic from becoming an executor/provider adapter.

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

**Why:** this records the independently evaluated external-effect claim without inventing provider response data or changing durable execution history.

## R10 — Retry disposition is advisory and cannot resolve the closed ECR-002 run

**Decision:** ECR-004 may compute a `RetryDispositionV1` from exact ECR-001 retry/idempotency semantics plus reconciliation outcome:

```text
effect_confirmed   -> duplicate_retry_blocked
still_unknown      -> reconciliation_required
no_effect_confirmed + safe -> semantically_retryable
no_effect_confirmed + requires_same_idempotency_key -> semantically_retryable_same_key
no_effect_confirmed + never_blind_retry -> requires_explicit_nonblind_path
```

This result is advisory runtime safety metadata only. `semantically_retryable` means only that a future owning runtime/policy path may consider proposing a **new attempt** under the applicable action semantics. It is not `CapabilityGrant`, approval, authorization lease, executor command, or permission to resume the existing ECR-002 run.

**Closed ECR-002 boundary:** a prepared-without-receipt attempt remains in ECR-002 `unresolved_attempts` after every ECR-004 reconciliation outcome. ECR-004 does not clear `PreparedAttemptState::unresolved`, append `ReceiptRecorded`, append a new run event, make `RunResumed`/`ExecutionCompleted` legal, or schedule a retry. Only an explicitly versioned future ECR-002 repair/resolution protocol could change that durable execution-state rule.

## R11 — Sidecar journal and integrity scope

**Decision:** persist only strict ECR-004 records/evidence references in a local append-only journal. Each entry carries sequence, previous digest, and canonical entry digest so substitution/corruption is detectable under normal local integrity assumptions.

**Claim boundary:** without ECR-031 protected anchoring, the journal MUST NOT be described as resistant to an attacker who can rewrite the entire store and recompute the chain. ECR-004 does not depend on ECR-031 and must remain useful with synthetic/non-sensitive fixtures now.

## R12 — Sensitive-data boundary

**Decision:** ECR-004 v1 acceptance persists only synthetic/non-sensitive verification metadata, IDs, digests, and bounded notes. Raw private/sensitive external evidence payloads remain out of the journal.

**Why:** ECR-031/ECR-025 gates are not dependencies of ECR-004. This slice must not smuggle sensitive persistence into an independently eligible path.

## R13 — Resource bounds

**Decision:** v1 uses the exact bounded ceilings frozen in the normative contract for evidence refs, receipts per target, checkpoint requirements, reconciliation support IDs, notes/rule identifiers, journal entry bytes, and query materialization. All arithmetic is checked; oversized input fails before expensive materialization where practical.

## R14 — No donor/source-code adoption

ECR-004 planning and implementation are independently written against canonical Ecra contracts. No external donor source code is authorized or adopted by T001. Public dependency APIs only are accepted.

## T001 — Exact dependency, license, advisory and MSRV admission

**Review date:** 2026-08-29  
**Authorization base:** `4fb61f8b41267983fc460c666fddd7781d91653c`  
**Dependency-gate evidence on that exact base:** ECR-001 run `33237289643` SUCCESS; ECR-002 run `33237289693` SUCCESS.  
**Workspace MSRV/toolchain:** Rust `1.98`; both exact-base gates compiled/tested the current locked dependency graph with the pinned 1.98 toolchain.

### Accepted direct runtime set for ecra-verify

| Dependency | Exact locked version / source | Features | License / boundary | Decision |
|---|---|---|---|---|
| `ecra-core` | workspace path | n/a | Ecra source | ACCEPT — canonical verification/evidence/action types |
| `ecra-run` | workspace path | n/a | Ecra source | ACCEPT — read-only run/attempt/retry truth |
| `serde` | 1.0.229 | `derive` | MIT OR Apache-2.0 | ACCEPT — strict typed wire values |
| `serde_json` | 1.0.151 | default | MIT OR Apache-2.0 | ACCEPT — strict JSON parsing/tests |
| `serde_jcs` | 0.2.0 | default | MIT OR Apache-2.0 | ACCEPT — repository-aligned RFC 8785 canonicalization |
| `sha2` | 0.11.0 | default | MIT OR Apache-2.0 | ACCEPT — domain-separated SHA-256 journal binding |
| `thiserror` | 2.0.20 | default | MIT OR Apache-2.0 | ACCEPT — typed machine error derivation |
| `uuid` | 1.26.0 | `serde` only | Apache-2.0 OR MIT; upstream MSRV 1.85 | ACCEPT — opaque ECR-004 IDs, no RNG/generation feature |
| `rusqlite` | =0.40.2 | `default-features = false`, `bundled` | MIT; bundled SQLite public domain | ACCEPT — bounded local sidecar journal only |

`rusqlite =0.40.2` retains the already-reviewed ECR-002 native boundary through `libsqlite3-sys 0.38.2` and bundled SQLite `3.53.2`. ECR-004 adds no new native implementation family.

### Accepted dev-only set

| Dependency | Exact locked version | License | Decision |
|---|---:|---|---|
| `proptest` | 1.11.0 | MIT OR Apache-2.0 | ACCEPT — bounded property/adversarial tests |
| `tempfile` | 3.27.0 | MIT OR Apache-2.0 | ACCEPT — isolated SQLite/reopen tests |

### Rejected / unnecessary dependencies

- `zip` — REJECT for ECR-004; portable `.ecra` archive ownership remains ECR-002 and verification journal persistence does not require ZIP.
- `url` — REJECT for ECR-004 trusted crate; external locators remain opaque canonical evidence metadata and ECR-004 performs no remote fetch.
- any browser, network, HTTP, model, provider, process-execution, protocol, policy/authorization, identity-backend, telemetry, async runtime or remote database dependency — REJECT by FR-041/SC-011.
- any second canonicalization, cryptographic hash, UUID, or SQLite abstraction library — REJECT; existing locked repository primitives are sufficient.
- source-copy/vendor adoption from donor projects — REJECT; dependency API use only.

### Advisory review

- `libsqlite3-sys`: RUSTSEC-2022-0090 is patched in `>=0.25.1`; current locked `0.38.2` is outside the affected range.
- `sha2`: RUSTSEC-2021-0100 affected `0.9.7` and is patched in `>=0.9.8`; current locked `0.11.0` is outside the affected release.
- 2026-08-20 Rust supply-chain campaign: RUSTSEC-2026-0260 identifies malicious `arrayref 0.3.10` through `proc-macro1`; related malicious packages include `proc-macro1`/`proc-macro-en` and affected releases in the same campaign. Repository search on the exact authorization state found no `arrayref` or `proc-macro1` path. ECR-004 adds no dependency requiring those packages.
- No advisory review authorizes ignoring a future lockfile delta. Any version/feature/transitive change must rerun dependency/advisory review before acceptance.

### MSRV and feature conclusion

- Repository workspace `rust-version = "1.98"` remains the ECR-004 MSRV floor.
- Exact canonical ECR-001/ECR-002 gates on `4fb61f8b...` succeeded under pinned Rust 1.98 with the reused dependency graph.
- `uuid 1.26.0` declares Rust 1.85, below the workspace floor.
- `rusqlite 0.40.2` is already compiled/tested in the exact-base ECR-002 gate; ECR-004 reuses the same minimal `bundled` feature profile.
- No default feature widening is authorized beyond the table above.

**T001 conclusion:** `DEPENDENCY_ADMISSION_ACCEPTED`. The implementation may proceed to T002 using only this bounded set, subject to locked CI proving the added local crate does not change the transitive dependency boundary unexpectedly.
