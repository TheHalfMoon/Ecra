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

```text
HEAD   18ad19ae4b4f4d5f48270485af666e7204b95a0e
RUN    33249643366
JOB    99093000858
RESULT SUCCESS
```

Phase 6 covers the strict append-only journal, domain-separated JCS/SHA-256 chain, transactional SQLite v1 store/migration, immutable canonical rows, rebuildable projections, expected-head concurrency, corruption/migration/reopen/replay evidence, synthetic-secret sentinel checks, offline operation and the explicit integrity-only claim boundary.

### Phase 7 — T040 hostile input/resource ceilings

```text
HEAD   815b95ed0f95513e583aa077f04e863998d0d425
RUN    33250068524
JOB    99094119979
RESULT SUCCESS
```

T040 proves exact maxima and max+1 typed failures for request evidence, receipts per target, checkpoint requirements, reconciliation support IDs/notes, journal bytes and 4,096-entry query materialization, plus bounded arbitrary JSON parsing without panic.

### Phase 7 — T041 portability

```text
HEAD   2a86dd909abfcb9d8658eab589787eb376a73004
RUN    33250250973
JOB    99094604997
RESULT SUCCESS
```

T041 proves accepted JSON whitespace/CRLF/field-order variants preserve canonical journal digest/bytes, aggregate behavior and reconciliation support ordering.

### Phase 7 — T042 v1 documentation

T042 is implemented in commit `1fa0ab70e2664803a200733b93888a3f29c604bf`. Its README content is included in the later exact-head T043 success below and documents decision-grade evidence, checkpoint semantics, reconciliation/retry non-authority, unchanged ECR-002 unresolved-state truth, future-new-attempt-only advisory semantics, synthetic/offline persistence and explicit assurance non-claims.

### Phase 7 — T043 complete quickstart exact-head gate

```text
HEAD   67207e1bc91434555bfe31997f4af9f641324a76
RUN    33250358128
JOB    99094901800
RESULT SUCCESS
```

Every quickstart and closure prerequisite step succeeded on that exact head: locked metadata/build, rustfmt, strict Clippy, workspace tests, all explicit ECR-001 regressions, all explicit ECR-002 regressions, every explicit ECR-004 quickstart target, dedicated ECR-002 unresolved-state compatibility acceptance, rustdoc, offline replay, all unsafe/dependency boundary scripts and dependency evidence.

Toolchain/dependency evidence from the same job:

```text
rustc                 1.98.0 (88d9e12ae 2026-08-18)
cargo                 1.98.0 (797e8a9bc 2026-08-05)
Cargo.lock SHA-256    b8112ece8111599af10b92bc2a2e54dd006985ec32a300e47c5f8c356383a2f6
```

The direct normal `ecra-verify` dependency surface is exactly `ecra-core`, `ecra-run`, `rusqlite 0.40.2`, `serde 1.0.229`, `serde_jcs 0.2.0`, `serde_json 1.0.151`, `sha2 0.11.0`, `thiserror 2.0.20`, and `uuid 1.26.0`. `url`/`zip` remain inherited only through canonical upstream workspace crates and are not ECR-004 direct capabilities.

### Phase 7 — T044 donor/license/dependency reconciliation

`specs/004-verification-receipts/research.md` and `research/donor-license-ledger.md` now reconcile the final implementation against the exact T043 dependency evidence. No donor implementation source was copied/adapted/vendored, no new provider/network/process/policy/identity/telemetry dependency entered `ecra-verify`, and the direct dependency set matches T001.

T044 is complete as repository documentation. The current documentation head must still pass T045 exact-head CI before Phase 7 is `VERIFIED_ON_BRANCH`.

## Current execution state

```text
CURRENT_TASK                    T045
CURRENT_STATE                   AWAITING_EXACT_HEAD_PHASE_7_CLOSURE_GATE
IMPLEMENTATION_BASE             4fb61f8b41267983fc460c666fddd7781d91653c
IMPLEMENTATION_BRANCH           004-verification-receipts-impl
IMPLEMENTATION_PR               6_DRAFT
T001_T039                       COMPLETE_WITH_EXACT_HEAD_EVIDENCE
T040                            COMPLETE_WITH_EXACT_HEAD_EVIDENCE
T041                            COMPLETE_WITH_EXACT_HEAD_EVIDENCE
T042                            COMPLETE_INCLUDED_IN_T043_EXACT_HEAD
T043                            COMPLETE_WITH_EXACT_HEAD_EVIDENCE
T044                            COMPLETE_PENDING_T045_DOCUMENTATION_HEAD_GATE
T045                            NEXT_REQUIRED
PHASE_7_PRE_CLOSURE_HEAD        67207e1bc91434555bfe31997f4af9f641324a76
PHASE_7_PRE_CLOSURE_RUN         33250358128
PHASE_7_PRE_CLOSURE_JOB         99094901800
PHASE_7_PRE_CLOSURE_RESULT      SUCCESS
```

## Canonical next order

```text
T045 exact-head Phase 7 closure gate
  ↓
T046 FR/SC traceability closure
  ↓
T047 constitution G1–G15 and risk/gap recheck
  ↓
T048 post-implementation analyze-equivalent review
  ↓
T049 package/platform/index/EXECUTION convergence
  ↓
T050 final exact-head implementation gate
  ↓
T051 move PR #6 out of Draft and resolve every actionable review finding
  ↓
T052 merge exact expected head by allowed non-rebase method + canonical-main workflows
  ↓
T053 post-merge canonical closure and dependency re-evaluation
```

## Parallel ECR-031 boundary

ECR-031 remains independently blocked at native macOS Data Protection Keychain acceptance. No valid Apple Development signing identity, suitable provisioning profile, or configured usable Apple developer account/team is available for the same macOS user that owns the self-hosted `macbook` runner.

That blocker must not be bypassed with legacy file-based Keychain, plaintext/file/env/memory fallback, ad-hoc signing substitution, weakened native tests, or assurance overclaims. ECR-004 does not depend on ECR-031 and continues independently.

ECR-005 remains blocked by its complete dependency set.
