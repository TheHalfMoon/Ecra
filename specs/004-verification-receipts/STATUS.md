# ECR-004 Status — Verification & Reconciliation

**Slice:** ECR-004  
**Lifecycle:** IMPLEMENTING  
**Dependencies:** ECR-001 `CLOSED_CANONICAL`, ECR-002 `CLOSED_CANONICAL`  
**Canonical implementation base:** `4fb61f8b41267983fc460c666fddd7781d91653c`  
**Implementation branch:** `004-verification-receipts-impl`  
**Implementation PR:** #6 (Draft)  
**Constitution:** v1.1.0

ECR-004 planning became canonical through merged PR #5. Exact canonical planning head `4fb61f8b41267983fc460c666fddd7781d91653c` passed both required dependency regressions:

```text
ECR-001 CI  33237289643  SUCCESS  exact head 4fb61f8b41267983fc460c666fddd7781d91653c
ECR-002 CI  33237289693  SUCCESS  exact head 4fb61f8b41267983fc460c666fddd7781d91653c
```

## Frozen v1 boundaries

- ECR-001 `VerificationReceipt` remains the only canonical independent verification record.
- `ActionReceipt` is executor-observed execution evidence and never self-verifies.
- Fact/Artifact/run metadata gains no parallel `verified` flag.
- ECR-004 reconciliation never fabricates `ActionReceipt`, appends ECR-002 events, clears `unresolved_attempts`, resumes/completes the same run, or schedules execution.
- Retry disposition is advisory for a future new-attempt proposal only.
- ECR-004 persistence is a separate append-only synthetic/non-sensitive verification journal.
- No browser/network/model/provider/process/policy/identity-backend execution dependency enters v1.

## Planning clarifications

- **IC-001:** ECR-004 may add only read-only accessors for the already-existing canonical `EvidenceRef` artifact/observation/receipt/external-ref/content-digest/as-of fields. No field, wire, canonicalization or validation change is authorized.
- **IC-002:** reconciliation evidence resolves effect truth only and cannot resolve ECR-002 v1 run state.

## Phase 1 — workspace, dependency and CI boundary

T001–T004 are implemented. T005 exact-head evidence:

```text
HEAD   e223ba5fbf8c375c580e7a93f524be3fd4c311fa
RUN    33237728338
JOB    99061549466
RESULT SUCCESS
```

Every required Phase 1 step succeeded: locked metadata/build, rustfmt, strict Clippy, workspace tests, ECR-001/ECR-002 regressions, ECR-004 targets, rustdoc, offline replay, all unsafe/dependency boundaries and dependency/toolchain evidence.

Phase 1 is `VERIFIED_ON_BRANCH`; ECR-004 is not yet `CLOSED_CANONICAL`.

## Phase 2 — strict request contract

T006–T010 are implemented:

- typed non-nil `CheckpointId` and `ReconciliationId`;
- strict ECR-004 machine-readable error category/code surface with bounded static diagnostics;
- strict exact-v1.0 `VerificationRequestV1` with bounded evidence/rule/notes, duplicate-ID rejection and unknown-field rejection;
- valid/invalid fixtures covering every canonical verification target, all method classes and all outcomes;
- validated request construction produces only the canonical ECR-001 `VerificationReceipt`;
- architecture/type tests preserve `ActionReceipt != VerificationReceipt` and reject parallel verified/authority surfaces.

T011 exact-head evidence:

```text
HEAD   40c18b4bcf1e6c124587cdfbc0e423822eb5b138
RUN    33245650032
JOB    99082565826
RESULT SUCCESS
```

The exact Phase 2 head passed locked metadata/build, rustfmt, strict Clippy, workspace tests, ECR-001 regressions, ECR-002 regressions, ECR-004 request/boundary tests, rustdoc, offline replay and every dependency/unsafe boundary.

Phase 2 request semantics are therefore `VERIFIED_ON_BRANCH` through T011. T011A remains a separate prerequisite before Phase 3.

## Current execution state

```text
CURRENT_TASK                    T011A
CURRENT_STATE                   IMPLEMENTING
IMPLEMENTATION_BASE             4fb61f8b41267983fc460c666fddd7781d91653c
IMPLEMENTATION_BRANCH           004-verification-receipts-impl
IMPLEMENTATION_PR               6_DRAFT
T001_T005                        COMPLETE_WITH_EXACT_HEAD_EVIDENCE
T006_T011                        COMPLETE_WITH_EXACT_HEAD_EVIDENCE
PHASE_2_HEAD                     40c18b4bcf1e6c124587cdfbc0e423822eb5b138
PHASE_2_RUN                      33245650032
PHASE_2_JOB                      99082565826
PHASE_2_RESULT                   SUCCESS
T011A                            ELIGIBLE
T012_PLUS                        NOT_REACHED
```

## Canonical next order

```text
T011A EvidenceRef read-only accessors + unchanged ECR-001 serialization/canonical semantics
  ↓ exact regression gate
T012 decision-grade evidence
T013 freshness
T014 deterministic aggregate
T015 fixtures
T016 permutation/determinism properties
T017 exact-head Phase 3 gate
```

No Phase 3 evidence logic starts before T011A completes and its exact regression evidence exists.

## Parallel ECR-031 boundary

ECR-031 remains a separate Draft implementation PR with the native macOS provisioning prerequisite at T064/T068. ECR-004 does not depend on ECR-031 and must not absorb identity/protected-storage scope or persist real sensitive evidence.

ECR-005 remains blocked by its complete dependency set.
