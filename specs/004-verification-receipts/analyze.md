# Analyze: ECR-004 Verification & Reconciliation

**Branch:** `004-verification-receipts`  
**Planning base:** `f6d8eb6ff6a60aa0ad8a6f52686a62f12cd374b0`  
**Pass:** 3  
**Result:** `ZERO_BLOCKING_PLANNING_DRIFT_FOUND`  
**Implementation authorization:** NO — planning must first become canonical and pass required dependency regressions.

## 1. Inputs reviewed

- `.specify/memory/constitution.md` v1.1.0
- `specs/000-ecra-platform/roadmap.md`
- `specs/000-ecra-platform/gap-audit.md`
- ECR-001 canonical verification/evidence/action contracts
- ECR-002 canonical run/attempt/recovery/event/reducer contracts
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

## 2. Blocking findings and remediation

### A-001 — ECR-004 requires canonical EvidenceRef metadata that was not publicly readable

**Severity:** MUST / BLOCKING before remediation.

`EvidenceRef` already owns the artifact/observation/receipt/external-reference/content-digest/as-of fields required for ECR-004 decision-grade checks, but the canonical ECR-001 API exposes only `id()` and `kind()` accessors. The original plan therefore risked either duplicating ECR-001 evidence or inspecting its serialized JSON representation.

**Resolution:** `implementation-clarifications.md` IC-001 authorizes only minimal read-only accessors for those existing fields, with no field/wire/canonical/validation change. `tasks.md` T011A owns the change and requires unchanged serialization semantics plus full ECR-001 regression evidence before T012.

**Status:** REMEDIATED.

### A-002 — Sidecar no-effect evidence cannot resolve the closed ECR-002 run state

**Severity:** MUST / BLOCKING before remediation.

Post-planning review of canonical ECR-002 showed an important execution-state boundary: a prepared-without-receipt attempt remains in `RunState::unresolved_attempts`; `RunResumed`, `ExecutionCompleted`, and blind-retry paths remain blocked while that state persists; and the reducer removes the unresolved marker only when a real `ReceiptRecorded` for the exact attempt is accepted. `ReconciliationRequested` is only a durable hook and does not resolve the attempt.

The original ECR-004 package correctly prohibited fabricated `ActionReceipt` and ECR-002 event mutation, but its `semantically_retryable*` wording could still be read as if `no_effect_confirmed` made the same ECR-002 run directly retryable. That would contradict the `CLOSED_CANONICAL` ECR-002 v1 state machine.

**Resolution:** IC-002 and FR-046/SC-013 now freeze the boundary across spec/research/model/contract/threat/plan/tasks/quickstart/checklist:

- ECR-004 reconciliation records independent effect truth only;
- every reconciliation outcome leaves ECR-002 run/attempt/unresolved state unchanged;
- no ECR-002 event or `ActionReceipt` is synthesized;
- `semantically_retryable*` means only advisory eligibility for a future **new-attempt proposal** under an owning runtime/policy path;
- the existing unresolved ECR-002 run does not become resumable/completable/retryable through ECR-004 v1;
- any operational run repair/resolution requires explicit future ECR-002 versioned ownership.

Phase 5 tasks T024/T027–T030 and the final gate now require explicit ECR-002 unresolved-state compatibility tests.

**Status:** REMEDIATED.

No additional blocking implementation prerequisite was found after both remediations.

## 3. Requirement-to-task traceability

### FR coverage

| Requirement range | Primary owning tasks | Status |
|---|---|---|
| FR-001–FR-004 canonical verification/type separation | T002, T009, T010 | OWNED |
| FR-005–FR-010 strict request/determinism/errors | T006–T011 | OWNED |
| FR-011–FR-017 evidence quality/provenance/safe metadata | T011A–T013, T016, T038, T042 | OWNED |
| FR-018–FR-021 deterministic aggregate/conflict | T014–T016 | OWNED |
| FR-022–FR-025 checkpoints/non-authority | T018–T021 | OWNED |
| FR-026–FR-035 reconciliation/UNKNOWN/retry advisory | T023–T030 | OWNED |
| FR-036–FR-040 append-only identity/journal/persistence/reopen | T006, T023, T031–T039 | OWNED |
| FR-041–FR-045 boundaries/bounds/errors/offline | T001–T005, T013, T020, T027, T038, T040–T045 | OWNED |
| FR-046 preserve ECR-002 unresolved execution state | T003–T004, T024, T027–T030, T034, T042–T050 | OWNED |

```text
FR_TOTAL=46
FR_OWNED=46
FR_UNOWNED=0
```

### SC coverage

| Success criterion | Primary owning tasks | Status |
|---|---|---|
| SC-001 strict target/evidence + receipt separation | T008–T011A | OWNED |
| SC-002 deterministic 1,000x/portability | T014, T016, T041 | OWNED |
| SC-003 conflict never hidden | T014–T016, T021–T022 | OWNED |
| SC-004 UNKNOWN/reconciliation matrix | T024–T030 | OWNED |
| SC-005 duplicate retry/no-effect non-authority | T026, T028–T030 | OWNED |
| SC-006 no fabricated receipt/no ECR-002 mutation | T027–T030, T034 | OWNED |
| SC-007 reopen/journal equivalence | T031–T039 | OWNED |
| SC-008 mutable decision-grade evidence | T011A–T013, T017 | OWNED |
| SC-009 bounded hostile input | T008, T021, T040, T045 | OWNED |
| SC-010 exact-head complete gates | T004–T005, phase gates, T043, T045, T050 | OWNED |
| SC-011 dependency/architecture boundary | T003–T005, T010, T020, T027, T038, T043–T045, T050 | OWNED |
| SC-012 traceability/convergence/closure | T046–T053 | OWNED |
| SC-013 unresolved ECR-002 state remains guarded | T003–T004, T024, T027–T030, T034, T042–T051 | OWNED |

```text
SC_TOTAL=13
SC_OWNED=13
SC_UNOWNED=0
```

## 4. Cross-artifact semantic consistency

### Verification truth

**PASS.** All artifacts preserve one canonical ECR-001 `VerificationReceipt`. `VerificationRequestV1` is construction input only; aggregate/checkpoint/reconciliation objects are derived or separately scoped records, never competing verification receipts.

### Executor vs verifier

**PASS.** `ActionReceipt` remains provider/executor-observed evidence. Spec, research, contract, threat model and tasks all prohibit self-verification and fabricated receipts.

### UNKNOWN and reconciliation

**PASS after A-002 remediation.** ECR-002 unresolved attempt truth remains authoritative for execution state. ECR-004 may append effect reconciliation evidence, but no outcome clears the unresolved marker, changes `PreparedAttemptState`, changes `RunPhase`, or creates an ECR-002 event/receipt.

### Retry semantics

**PASS after A-002 remediation.** ECR-004 derives semantic retry advisory only after reconciliation and continues to use ECR-001 `RetryClass`/`IdempotencyClass`. `semantically_retryable*` is explicitly limited to a future new-attempt proposal and does not override ECR-002 same-run guards or grant capability/approval/execution authorization.

### Evidence/provenance

**PASS after A-001 remediation.** ECR-004 reads existing canonical evidence metadata through typed read-only accessors and never rewrites provenance/freshness/dispute state. Mutable decision-grade evidence rules require immutable binding/freshness where applicable.

### Conflict semantics

**PASS.** Aggregate state is closed and deterministic; simultaneous `Verified` and `Rejected` is always `Conflicted`; no last-write-wins path exists.

### Checkpoints

**PASS.** Critical-point verification is modeled as bounded exact-target requirements and a derived evaluation. It neither creates ECR-002 run completion events nor carries authority.

### Persistence ownership

**PASS.** Sidecar journal is ECR-004-owned; ECR-002 strict v1 `RunEvent` is unchanged. Journal rows are canonical ECR-004 persisted truth; SQL indexes are rebuildable projections. No sidecar projection represents ECR-002 run resolution.

### Integrity claim

**PASS.** The digest chain is consistently described as normal corruption/substitution detection only. Full-store hostile rewrite resistance is explicitly not claimed without protected anchoring from another authorized slice.

### Sensitive data

**PASS.** v1 acceptance stores synthetic/non-sensitive references/digests/bounded metadata only. Real private/sensitive evidence persistence remains outside authorization and does not covertly depend on unfinished ECR-031.

### External execution

**PASS.** No browser/network/model/provider/process evidence acquisition exists in ECR-004 v1. No reconciliation API schedules or executes a retry. Later owning adapters/runtimes acquire evidence or propose new attempts.

## 5. Dependency consistency

- ECR-004 depends only on ECR-001 and ECR-002, both canonically closed.
- ECR-031 is not a dependency and its native-acceptance blocker is not bypassed.
- ECR-003 is not imported; authorization remains explicitly outside scope.
- ECR-005 remains blocked by its full dependency set even if ECR-004 closes.
- IC-001 modifies only ECR-001 read-only API accessors under mandatory ECR-001 regression coverage; it does not reopen ECR-001 semantics.
- IC-002 explicitly avoids changing the ECR-002 v1 event/reducer/state contract; ECR-004 consumes `RunState` read-only and requires unchanged ECR-002 regressions.

**Result:** PASS.

## 6. Constitution G1–G15 recheck

| Gate | Result | Evidence |
|---|---|---|
| G1 Domain coherence | PASS | canonical ECR-001/ECR-002 types reused; no second receipt/run-resolution truth |
| G2 Authority | PASS | verification/reconciliation/retry disposition explicitly non-authoritative |
| G3 Provenance | PASS | evidence metadata retained; no provenance rewrite; IC-001 typed access only |
| G4 Side effects | PASS | only local append-only journal mutation; external effects observed, never executed; same-run state untouched |
| G5 Verification | PASS | executor receipt separated from independent receipt; conflict preserved |
| G6 Durability | PASS | ECR-002 unresolved state remains durable dependency truth; sidecar restart/replay/migration/concurrency/corruption tasks owned |
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

A-002 narrows the operational claim correctly: ECR-004 owns independent reconciliation evidence and duplicate-retry advisory, not ECR-002 run-state repair. A future versioned repair/resolution protocol remains an explicit later convergence need rather than being smuggled into this slice.

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
reconciliation/retry advisory + IC-002 ECR-002 compatibility proof
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

None authorizes scope expansion or ECR-002 state mutation.

## 10. Analyze result

```text
PASS_1_BLOCKERS_FOUND=1
PASS_1_BLOCKERS_REMEDIATED=1
PASS_2_NEW_BLOCKERS_FOUND=1
PASS_2_NEW_BLOCKERS_REMEDIATED=1
FR_TOTAL=46
FR_OWNED=46
FR_UNOWNED=0
SC_TOTAL=13
SC_OWNED=13
SC_UNOWNED=0
MUST_LEVEL_PLANNING_GAPS=0
FAILED_CONSTITUTION_GATES=0
CROSS_ARTIFACT_BLOCKING_CONTRADICTIONS=0
RESULT=ZERO_BLOCKING_PLANNING_DRIFT_FOUND
```

The ECR-004 package is a `TASKS_READY` planning candidate after Pass 3. It is not implementation-authorized yet. Next canonical steps are planning status/index/platform convergence, PR #5 review on the exact converged head, allowed non-rebase merge, exact canonical ECR-001/ECR-002 regression evidence on the merged planning head, then creation of the implementation branch from that exact eligible canonical head.