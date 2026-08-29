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
- The journal digest chain is an integrity/corruption/substitution detector under local-store assumptions. It is **not** a hostile whole-store tamper-resistance guarantee and has no protected external trust anchor in v1.
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

```text
HEAD   fb3fdf1ce113a55d3d7276f54681a7f55dc542b3
RUN    33247815573
JOB    99088239340
RESULT SUCCESS
```

Phase 5 covers strict reconciliation identity/binding, canonical support receipt resolution, fail-closed effect/no-effect/unknown derivation, ECR-002 byte/semantic non-mutation, unresolved-attempt preservation, same-run resume/completion/blind-retry guard compatibility, and future-new-attempt-only retry advisory semantics.

### Phase 6 — T031–T039

T031–T038 implement and test:

- strict versioned `VerificationJournalEntryV1` with positive bounded sequence, exact previous-digest rule, repository-aligned JCS material, domain-separated SHA-256 and fixed digest goldens;
- transactional SQLite v1 initialization with schema marker, append-only authoritative `verification_journal`, and rebuildable receipt/checkpoint/reconciliation indexes separate from ECR-002 run storage;
- SQL update/delete rejection for canonical journal rows while projections remain rebuildable and non-authoritative;
- expected-head compare-and-append under `BEGIN IMMEDIATE`, where a stale competing writer fails closed;
- authoritative duplicate-ID detection independent of projection contents;
- corruption detection for malformed entry JSON, row/entry metadata mismatch, sequence gaps/reordering, previous-digest mismatch and duplicate canonical identities;
- newer-schema fail-closed behavior and failed-initialization rollback evidence;
- projection deletion/poisoning followed by canonical rebuild;
- reopen/replay preservation of byte-equivalent aggregate/checkpoint views and identical reconciliation records;
- synthetic/non-sensitive sentinel tests proving raw secret sentinel text is absent from persisted journal material and derived Debug output.

The initial Phase 6 store candidate passed build but failed only rustfmt. The formatting repair was forward-only and did not change store semantics.

T039 exact-head evidence:

```text
HEAD   18ad19ae4b4f4d5f48270485af666e7204b95a0e
RUN    33249643366
JOB    99093000858
RESULT SUCCESS
```

Every required Phase 6 gate succeeded on the exact head: locked metadata/build, rustfmt, strict Clippy, workspace tests, ECR-001 regressions, ECR-002 regressions, ECR-004 targets, rustdoc, offline replay, unsafe/dependency boundaries and dependency/toolchain evidence.

**Integrity claim boundary:** the v1 journal hash chain detects corruption, substitution and broken linkage when replayed under its local assumptions. Because v1 has no independently protected root/head anchor, ECR-004 does not claim resistance to an adversary that can rewrite the entire store consistently, verifier infallibility, provider authenticity, or exactly-once external effects.

Phase 6 is `VERIFIED_ON_BRANCH`; ECR-004 remains Draft and is not `CLOSED_CANONICAL`.

## Current execution state

```text
CURRENT_TASK                    T040
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
T031_T039                       COMPLETE_WITH_EXACT_HEAD_EVIDENCE
PHASE_6_HEAD                    18ad19ae4b4f4d5f48270485af666e7204b95a0e
PHASE_6_RUN                     33249643366
PHASE_6_JOB                     99093000858
PHASE_6_RESULT                  SUCCESS
T040                            ELIGIBLE
T041_PLUS                       ORDERED_BY_TASK_GRAPH
```

## Canonical next order

```text
T040 hostile/bounded-input and exact-max resource tests
  ↓
T041 strict JSON/canonicalization portability tests
  ↓
T042 exact v1 usage and assurance-boundary documentation
  ↓
T043 complete quickstart exact-head evidence
  ↓
T044 donor/license/dependency implementation reconciliation
  ↓
T045 exact-head Phase 7 closure gate
  ↓
T046–T053 traceability, convergence, review, merge and canonical closure
```

## Parallel ECR-031 boundary

ECR-031 remains independently blocked at native macOS Data Protection Keychain acceptance. No valid Apple Development signing identity, suitable provisioning profile, or configured usable Apple developer account/team is available for the same macOS user that owns the self-hosted `macbook` runner.

That blocker must not be bypassed with legacy file-based Keychain, plaintext/file/env/memory fallback, ad-hoc signing substitution, weakened native tests, or assurance overclaims. ECR-004 does not depend on ECR-031 and continues independently.

ECR-005 remains blocked by its complete dependency set.
