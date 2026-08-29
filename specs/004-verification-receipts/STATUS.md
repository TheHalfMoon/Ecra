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
- **IC-003:** `verification_receipts` is normally non-empty. The sole empty-support exception is a `still_unknown` record produced because no supporting verification receipt exists. `effect_confirmed` and `no_effect_confirmed` always require at least one resolved supporting receipt; absence of evidence can never prove no effect.

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

T006–T010 are implemented. T011 exact-head evidence:

```text
HEAD   40c18b4bcf1e6c124587cdfbc0e423822eb5b138
RUN    33245650032
JOB    99082565826
RESULT SUCCESS
```

T011A then added only the IC-001 read-only canonical `EvidenceRef` accessors. ECR-001 field/wire/schema/validation semantics were not changed, the temporary branch-only write-capable bootstrap workflow was removed before acceptance, and regression tests preserve JSON/JCS behavior.

T011A exact-head evidence:

```text
HEAD   75cac2aed9099d7ba82295c442b37764b284302c
RUN    33245970650
JOB    99083386559
RESULT SUCCESS
```

Phase 2 including T011A is `VERIFIED_ON_BRANCH`.

## Phase 3 — decision-grade evidence and deterministic aggregation

T012–T016 are implemented:

- decision-grade assessment requires evidence binding and fail-closes conclusive outcomes without immutable binding;
- explicit freshness rules use only supplied `as_of` evidence metadata and supplied evaluation time, with no ambient clock or remote fetch;
- an execution receipt cannot alone prove its own conclusive verification claim;
- independent model judgment does not outrank missing independent non-model evidence;
- conclusive canonical `VerificationReceipt` construction is routed through decision-grade assessment rather than a bypass;
- aggregate state is deterministic across `Absent`, `Verified`, `Rejected`, `Inconclusive`, and `Conflicted`;
- aggregate views retain the complete sorted receipt-ID set plus sorted per-outcome receipt-ID sets;
- `Verified + Rejected` is always `Conflicted`; `NotEvaluated` alone is `Absent`; no last-write-wins rule exists;
- aggregation fixtures, all six three-receipt permutations, 1,000 identical aggregate evaluations, and ECR-001 Fact non-mutation evidence are covered by tests.

T017 exact-head evidence:

```text
HEAD   f5181ca4f903f2d039463b03b3e328b1fa9c30dd
RUN    33246658250
JOB    99085187943
RESULT SUCCESS
```

Phase 3 is `VERIFIED_ON_BRANCH`; ECR-004 remains Draft and is not `CLOSED_CANONICAL`.

## Phase 4 — critical verification checkpoints

T018–T021 are implemented:

- strict bounded checkpoint and requirement contracts;
- duplicate exact target rejection;
- deterministic canonical requirement ordering and deterministic checkpoint evaluation;
- explicit satisfied, unsatisfied and conflicted target sets;
- `Absent`, `Inconclusive` and `Conflicted` cannot satisfy a v1 requirement;
- specialized negative requirements may explicitly accept `Rejected`;
- checkpoints expose no authority, policy, approval, declassification, secret or executor surface;
- valid/invalid fixtures cover all-satisfied, absent, rejected, inconclusive, conflicted, duplicate-target, over-limit and false-completion cases.

Intermediate Phase 4 heads failed only rustfmt/Clippy quality gates and were repaired forward-only without suppressions or semantic weakening.

T022 exact-head evidence:

```text
HEAD   412de3f481d84154c5c2a85f11c6a6da0c89e35a
RUN    33247226826
JOB    99086690683
RESULT SUCCESS
```

Every required Phase 4 step succeeded on the exact head: locked metadata/build, rustfmt, strict Clippy, workspace tests, ECR-001 regressions, ECR-002 regressions, ECR-004 targets, rustdoc, offline replay, unsafe/dependency boundaries and dependency/toolchain evidence.

Phase 4 is `VERIFIED_ON_BRANCH`; ECR-004 remains Draft and is not `CLOSED_CANONICAL`.

## Current execution state

```text
CURRENT_TASK                    T023
CURRENT_STATE                   IMPLEMENTING
IMPLEMENTATION_BASE             4fb61f8b41267983fc460c666fddd7781d91653c
IMPLEMENTATION_BRANCH           004-verification-receipts-impl
IMPLEMENTATION_PR               6_DRAFT
T001_T005                       COMPLETE_WITH_EXACT_HEAD_EVIDENCE
T006_T011                       COMPLETE_WITH_EXACT_HEAD_EVIDENCE
T011A                           COMPLETE_WITH_EXACT_HEAD_EVIDENCE
T012_T017                       COMPLETE_WITH_EXACT_HEAD_EVIDENCE
T018_T022                       COMPLETE_WITH_EXACT_HEAD_EVIDENCE
PHASE_4_HEAD                    412de3f481d84154c5c2a85f11c6a6da0c89e35a
PHASE_4_RUN                     33247226826
PHASE_4_JOB                     99086690683
PHASE_4_RESULT                  SUCCESS
T023                            ELIGIBLE
T024_PLUS                       ORDERED_BY_TASK_GRAPH
```

## Canonical next order

```text
T023 strict reconciliation record/outcome contract
  ↓
T024 exact RunId/attempt/action binding against read-only RunState
  ↓
T025 supporting verification receipt resolution
  ↓
T026 fail-closed effect/no-effect/still-unknown rules
  ↓
T027 ECR-002 non-mutation and architecture proof
  ↓
T028 retry disposition advisory
  ↓
T029 exhaustive reconciliation/retry matrix
  ↓
T030 exact-head Phase 5 gate
```

## Parallel ECR-031 boundary

ECR-031 remains independently blocked at native macOS Data Protection Keychain acceptance. No valid Apple Development signing identity, suitable provisioning profile, or configured usable Apple developer account/team is available for the same macOS user that owns the self-hosted `macbook` runner.

That blocker must not be bypassed with legacy file-based Keychain, plaintext/file/env/memory fallback, ad-hoc signing substitution, weakened native tests, or assurance overclaims. ECR-004 does not depend on ECR-031 and continues independently.

ECR-005 remains blocked by its complete dependency set.
