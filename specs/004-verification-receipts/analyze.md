# Analyze: ECR-004 Verification & Reconciliation

**Branch:** `004-verification-receipts`  
**Planning base:** `f6d8eb6ff6a60aa0ad8a6f52686a62f12cd374b0`  
**Pass:** 2  
**Result:** `ZERO_BLOCKING_PLANNING_DRIFT_FOUND`  
**Implementation authorization:** NO — planning must first become canonical and pass required dependency regressions.

## 1. Inputs reviewed

- `.specify/memory/constitution.md` v1.1.0
- `specs/000-ecra-platform/roadmap.md`
- `specs/000-ecra-platform/gap-audit.md`
- ECR-001 canonical verification/evidence/action contracts
- ECR-002 canonical run/attempt/recovery/event contracts
- `spec.md`
- `research.md`
- `data-model.md`
- `contracts/verification-reconciliation-v1.md`
- `threat-model.md`
- `plan.md`
- `quickstart.md`
- `tasks.md`
- `checklists/requirements.md`
- `implementation-clarifications.md`

## 2. Pass-1 blocking finding and remediation

### A-001 — ECR-004 requires canonical EvidenceRef metadata that was not publicly readable

**Severity:** MUST / BLOCKING before remediation.

`EvidenceRef` already owns the artifact/observation/receipt/external-reference/content-digest/as-of fields required for ECR-004 decision-grade checks, but the canonical ECR-001 API exposes only `id()` and `kind()` accessors. The original plan therefore risked either duplicating ECR-001 evidence or inspecting its serialized JSON representation.

**Resolution:** `implementation-clarifications.md` IC-001 now authorizes only minimal read-only accessors for those existing fields, with no field/wire/canonical/validation change. `tasks.md` T011A owns the change and requires unchanged serialization semantics plus full ECR-001 regression evidence before T012.

**Status:** REMEDIATED.

No other blocking implementation prerequisite was found. `RunState::run_id()`, `prepared_attempts()`/unresolved state, `PreparedAttemptState::attempt()`, and `ActionAttemptRef::action()` already expose the exact typed binding required by reconciliation.

## 3. Requirement-to-task traceability

### FR coverage

| Requirement range | Primary owning tasks | Status |
|---|---|---|
| FR-001–FR-004 canonical verification/type separation | T002, T009, T010 | OWNED |
| FR-005–FR-010 strict request/determinism/errors | T006–T011 | OWNED |
| FR-011–FR-017 evidence quality/provenance/safe metadata | T011A–T013, T016, T038, T042 | OWNED |
| FR-018–FR-021 deterministic aggregate/conflict | T014–T016 | OWNED |
| FR-022–FR-025 checkpoints/non-authority | T018–T021 | OWNED |
| FR-026–FR-035 reconciliation/UNKNOWN/retry safety | T023–T029 | OWNED |
| FR-036–FR-040 append-only identity/journal/persistence/reopen | T006, T023, T031–T039 | OWNED |
| FR-041–FR-045 boundaries/bounds/errors/offline | T001–T005, T013, T020, T027, T038, T040–T045 | OWNED |

```text
FR_TOTAL=45
FR_OWNED=45
FR_UNOWNED=0
```

### SC coverage

| Success criterion | Primary owning tasks | Status |
|---|---|---|
| SC-001 strict target/evidence + receipt separation | T008–T011A | OWNED |
| SC-002 deterministic 1,000x/portability | T014, T016, T041 | OWNED |
| SC-003 conflict never hidden | T014–T016, T021–T022 | OWNED |
| SC-004 UNKNOWN/reconciliation matrix | T024–T029 | OWNED |
| SC-005 duplicate retry/no-effect non-authority | T026, T028–T029 | OWNED |
| SC-006 no fabricated receipt/no ECR-002 mutation | T027, T029, T034 | OWNED |
| SC-007 reopen/journal equivalence | T031–T039 | OWNED |
| SC-008 mutable decision-grade evidence | T011A–T013, T017 | OWNED |
| SC-009 bounded hostile input | T008, T021, T040, T045 | OWNED |
| SC-010 exact-head complete gates | T004–T005, phase gates, T043, T045, T050 | OWNED |
| SC-011 dependency/architecture boundary | T003–T005, T010, T020, T027, T038, T043–T045, T050 | OWNED |
| SC-012 traceability/convergence/closure | T046–T053 | OWNED |

```text
SC_TOTAL=12
SC_OWNED=12
SC_UNOWNED=0
```

## 4. Cross-artifact semantic consistency

### Verification truth

**PASS.** All artifacts preserve one canonical ECR-001 `VerificationReceipt`. `VerificationRequestV1` is construction input only; aggregate/checkpoint/reconciliation objects are derived or separately scoped records, never competing verification receipts.

### Executor vs verifier

**PASS.** `ActionReceipt` remains provider/executor-observed evidence. Spec, research, contract, threat model and tasks all prohibit self-verification and fabricated receipts.

### UNKNOWN and reconciliation

**PASS.** ECR-002 unresolved attempt truth remains authoritative for execution state. ECR-004 may append effect reconciliation evidence, but `still_unknown` remains blocking and no reconciliation record mutates ECR-002 state.

### Retry semantics

**PASS.** ECR-004 derives semantic retry safety only after reconciliation and continues to use ECR-001 `RetryClass`/`IdempotencyClass`. It grants no capability, approval or execution authorization.

### Evidence/provenance

**PASS after A-001 remediation.** ECR-004 reads existing canonical evidence metadata through typed read-only accessors and never rewrites provenance/freshness/dispute state. Mutable decision-grade evidence rules require immutable binding/freshness where applicable.

### Conflict semantics

**PASS.** Aggregate state is closed and deterministic; simultaneous `Verified` and `Rejected` is always `Conflicted`; no last-write-wins path exists.

### Checkpoints

**PASS.** Critical-point verification is modeled as bounded exact-target requirements and a derived evaluation. It neither creates ECR-002 run completion events nor carries authority.

### Persistence ownership

**PASS.** Sidecar journal is ECR-004-owned; ECR-002 strict v1 `RunEvent` is unchanged. Journal rows are canonical ECR-004 persisted truth; SQL indexes are rebuildable projections.

### Integrity claim

**PASS.** The digest chain is consistently described as normal corruption/substitution detection only. Full-store hostile rewrite resistance is explicitly not claimed without protected anchoring from another authorized slice.

### Sensitive data

**PASS.** v1 acceptance stores synthetic/non-sensitive references/digests/bounded metadata only. Real private/sensitive evidence persistence remains outside authorization and does not covertly depend on unfinished ECR-031.

### External execution

**PASS.** No browser/network/model/provider/process evidence acquisition exists in ECR-004 v1. Later adapters own live acquisition and pass explicit evidence data inward.

## 5. Dependency consistency

- ECR-004 depends only on ECR-001 and ECR-002, both canonically closed.
- ECR-031 is not a dependency and its native-acceptance blocker is not bypassed.
- ECR-003 is not imported; authorization remains explicitly outside scope.
- ECR-005 remains blocked by its full dependency set even if ECR-004 closes.
- IC-001 modifies only ECR-001 read-only API accessors under mandatory ECR-001 regression coverage; it does not reopen ECR-001 semantics.

**Result:** PASS.

## 6. Constitution G1–G15 recheck

| Gate | Result | Evidence |
|---|---|---|
| G1 Domain coherence | PASS | canonical ECR-001/ECR-002 types reused; no second receipt/run truth |
| G2 Authority | PASS | verification/reconciliation/retry disposition explicitly non-authoritative |
| G3 Provenance | PASS | evidence metadata retained; no provenance rewrite; IC-001 typed access only |
| G4 Side effects | PASS | only local append-only journal mutation; external effects are observed, never executed |
| G5 Verification | PASS | executor receipt separated from independent receipt; conflict preserved |
| G6 Durability | PASS | restart/replay/migration/concurrency/corruption tasks owned |
| G7 Privacy/secrets | PASS | synthetic/non-sensitive metadata only; sentinel tasks owned |
| G8 Local-first | PASS | complete offline fixture path; no cloud dependency |
| G9 Interoperability | PASS-N/A | no protocol adapter in v1 |
| G10 Donor/license | PASS | no donor code; T001/T044 require exact dependency/license reconciliation |
| G11 Upstream/browser maintenance | PASS-N/A | no browser privileged surface |
| G12 Benchmarks/claims | PASS | deterministic/property/resource claims only; verifier accuracy deferred |
| G13 Information flow/egress | PASS | no remote acquisition/egress; explicit evidence input only |
| G14 Identity/principal | PASS | optional verifier principal is evidence, not authentication minting |
| G15 Bounded execution | PASS | count/byte/query bounds and hostile-input tasks explicit |

```text
FAILED_CONSTITUTION_GATES=0
PASS_NA_GATES=2
```

## 7. Platform gap ownership check

ECR-004 package owns the platform gaps assigned to it:

- idempotency/retry reconciliation matrix;
- UNKNOWN outcome preservation;
- blind retry prevention;
- duplicate external side-effect reconciliation;
- executor receipt vs verifier result;
- no duplicate verified truth flag on `Fact`;
- critical-point verification requirements;
- mutable decision-grade evidence handling;
- malicious evidence/verification capture boundary.

Verifier statistical quality, independent-source corroboration and provider-specific live acquisition remain with ECR-005/ECR-009/ECR-028 or later adapters as already planned.

**Result:** PASS.

## 8. Task-order review

**PASS.** Dependency order is executable:

```text
workspace/deps/CI
  ↓
strict request/types
  ↓
IC-001 canonical evidence accessors
  ↓
decision-grade evidence + aggregate
  ↓
checkpoints
  ↓
reconciliation/retry safety
  ↓
journal/persistence
  ↓
hostile input/docs
  ↓
traceability/convergence/review/merge/post-merge
```

No task requires a later task to define an earlier contract. Persistence follows semantic rules. Final review/merge tasks cannot run before exact-head gates.

## 9. Residual non-blocking implementation questions

These are implementation details already owned by tasks and do not change v1 semantics:

1. exact accepted dependency feature sets after T001 current-advisory/MSRV review;
2. exact Rust return borrowing signatures for IC-001 read-only accessors;
3. exact SQLite trigger/index implementation while preserving the frozen logical schema and append-only contract;
4. whether canonical JCS/hash helpers are reused through public helpers or a minimal dependency already accepted by the repository;
5. exact error enum naming while preserving required machine-readable categories/codes.

None authorizes scope expansion.

## 10. Analyze result

```text
PASS_1_BLOCKERS_FOUND=1
PASS_1_BLOCKERS_REMEDIATED=1
UNOWNED_FR=0
UNOWNED_SC=0
MUST_LEVEL_PLANNING_GAPS=0
FAILED_CONSTITUTION_GATES=0
CROSS_ARTIFACT_BLOCKING_CONTRADICTIONS=0
RESULT=ZERO_BLOCKING_PLANNING_DRIFT_FOUND
```

The ECR-004 package is a `TASKS_READY` planning candidate. It is not implementation-authorized yet. Next canonical steps are planning status/index/platform convergence, planning PR review/merge, exact canonical ECR-001/ECR-002 regression evidence on the merged planning head, then creation of the implementation branch from that exact eligible canonical head.