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

## Verified branch checkpoints

### Phase 1 — T001–T005

```text
HEAD   e223ba5fbf8c375c580e7a93f524be3fd4c311fa
RUN    33237728338
JOB    99061549466
RESULT SUCCESS
```

### Phase 2 — T006–T011

```text
HEAD   40c18b4bcf1e6c124587cdfbc0e423822eb5b138
RUN    33245650032
JOB    99082565826
RESULT SUCCESS
```

### T011A — IC-001 read-only ECR-001 accessors

```text
HEAD   75cac2aed9099d7ba82295c442b37764b284302c
RUN    33245970650
JOB    99083386559
RESULT SUCCESS
```

### Phase 3 — T012–T017

```text
HEAD   f5181ca4f903f2d039463b03b3e328b1fa9c30dd
RUN    33246658250
JOB    99085187943
RESULT SUCCESS
```

Phase 3 covers decision-grade evidence, explicit freshness, immutable evidence binding, self-attestation rejection, model-judgment fail-closed behavior, deterministic aggregate states, per-outcome receipt-ID retention, conflict fixtures, permutation invariance, 1,000 byte-equivalent evaluations, and ECR-001 non-mutation proof.

### Phase 4 — T018–T022

```text
HEAD   412de3f481d84154c5c2a85f11c6a6da0c89e35a
RUN    33247226826
JOB    99086690683
RESULT SUCCESS
```

Phase 4 covers strict bounded checkpoints, duplicate-target rejection, canonical ordering, deterministic satisfied/unsatisfied/conflicted views, prohibited satisfying states, specialized negative requirements, authority-surface exclusion, and the full checkpoint fixture matrix.

### Phase 5 — T023–T030

T023–T029 implement and test:

- strict bounded `ReconciliationRecordV1` and closed reconciliation outcomes;
- exact `RunId` + durable unresolved `ActionAttemptRef` + underlying `ActionRef` binding against read-only ECR-002 `RunState`;
- canonical support receipt resolution with missing/duplicate/cross-target rejection and deterministic support-ID retention;
- `effect_confirmed`, `no_effect_confirmed`, and `still_unknown` derived from exact supporting verification evidence rather than caller-selected outcome;
- IC-003 evidence-absent `still_unknown` without fabricated support IDs;
- immutable evidence requirement for conclusive reconciliation;
- byte/semantic non-mutation proof for ECR-002 state across all reconciliation outcomes;
- unresolved prepared attempt membership remains unchanged and no provider `ActionReceipt` or ECR-002 `RunEvent` is synthesized;
- same-run `RunResumed`, `ExecutionCompleted`, and blind-retry guards remain blocked for the unresolved prior attempt;
- `RetryDispositionV1` remains advisory only for a future new-attempt proposal;
- duplicate-effect block, reconciliation-required, safe semantic retry, exact same-key semantic retry, key mutation rejection, external-reconciliation and never-blind paths are covered by the retry matrix.

Intermediate Phase 5 heads failed only rustfmt quality gates and were repaired forward-only without suppressions or semantic weakening.

T030 exact-head evidence:

```text
HEAD   fb3fdf1ce113a55d3d7276f54681a7f55dc542b3
RUN    33247815573
JOB    99088239340
RESULT SUCCESS
```

Every required Phase 5 gate succeeded on the exact head: locked metadata/build, rustfmt, strict Clippy, workspace tests, ECR-001 regressions, ECR-002 regressions, ECR-004 targets, rustdoc, offline replay, unsafe/dependency boundaries and dependency/toolchain evidence.

Phase 5 is `VERIFIED_ON_BRANCH`; ECR-004 remains Draft and is not `CLOSED_CANONICAL`.

## Current execution state

```text
CURRENT_TASK                    T031
CURRENT_STATE                   IMPLEMENTING
IMPLEMENTATION_BASE             4fb61f8b41267983fc460c666fddd7781d91653c
IMPLEMENTATION_BRANCH           004-verification-receipts-impl
IMPLEMENTATION_PR               6_DRAFT
T001_T005                       COMPLETE_WITH_EXACT_HEAD_EVIDENCE
T006_T011                       COMPLETE_WITH_EXACT_HEAD_EVIDENCE
T011A                           COMPLETE_WITH_EXACT_HEAD_EVIDENCE
T012_T017                       COMPLETE_WITH_EXACT_HEAD_EVIDENCE
T018_T022                       COMPLETE_WITH_EXACT_HEAD_EVIDENCE
T023_T030                       COMPLETE_WITH_EXACT_HEAD_EVIDENCE
PHASE_5_HEAD                    fb3fdf1ce113a55d3d7276f54681a7f55dc542b3
PHASE_5_RUN                     33247815573
PHASE_5_JOB                     99088239340
PHASE_5_RESULT                  SUCCESS
T031                            ELIGIBLE
T032_PLUS                       ORDERED_BY_TASK_GRAPH
```

## Canonical next order

```text
T031 strict verification journal entry and digest chain
  ↓
T032 canonical digest goldens and mutation tests
  ↓
T033 transactional SQLite v1 journal/index store
  ↓
T034 append-only truth and rebuildable projection enforcement
  ↓
T035 expected-head concurrency
  ↓
T036 corruption/migration/projection recovery matrix
  ↓
T037 restart/reopen deterministic replay
  ↓
T038 synthetic/non-sensitive sentinel boundaries
  ↓
T039 exact-head Phase 6 gate
```

## Parallel ECR-031 boundary

ECR-031 remains independently blocked at native macOS Data Protection Keychain acceptance. No valid Apple Development signing identity, suitable provisioning profile, or configured usable Apple developer account/team is available for the same macOS user that owns the self-hosted `macbook` runner.

That blocker must not be bypassed with legacy file-based Keychain, plaintext/file/env/memory fallback, ad-hoc signing substitution, weakened native tests, or assurance overclaims. ECR-004 does not depend on ECR-031 and continues independently.

ECR-005 remains blocked by its complete dependency set.
