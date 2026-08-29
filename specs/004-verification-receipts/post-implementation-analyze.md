# ECR-004 Post-Implementation Analyze

**Slice:** ECR-004 — Verification & Reconciliation  
**Review:** T048 analyze-equivalent post-implementation consistency pass  
**Implementation branch:** `004-verification-receipts-impl`  
**Canonical base:** `4fb61f8b41267983fc460c666fddd7781d91653c`  
**Constitution:** v1.1.0

## Scope reviewed

The pass compares canonical `spec.md`, `research.md`, `data-model.md`, contracts, `threat-model.md`, `plan.md`, `quickstart.md`, `tasks.md`, implementation, tests, workflow, donor/license evidence and active lifecycle documents. It also re-checks FR-001–FR-046, SC-001–SC-013 and G1–G15 against exact implementation behavior.

## Result

No implementation defect or unresolved constitutional blocker was found. Verification/reconciliation semantics remain fail-closed and preserve the frozen ECR-001/ECR-002 boundaries.

Three documentation/convergence drifts were found. They do not authorize semantic weakening and MUST be corrected before T050 final exact-head gate.

### C-001 — DecisionGradeAssessmentV1 data-model shape is stale

**Severity:** MUST converge documentation; implementation behavior is safe and tested.

`data-model.md` currently describes a boolean `decision_grade` plus one reason. The implementation uses:

```text
DecisionGradeAssessmentV1
  status: DecisionGradeStatusV1 { decision_grade | non_decision_grade }
  reasons: [DecisionGradeReasonV1]
```

The reason set is deterministic and can retain multiple simultaneous fail-closed causes (`missing_evidence_binding`, `missing_immutable_binding`, `missing_evaluation_time`, `missing_freshness`, `evidence_from_future`, `evidence_stale`, `self_attesting_execution_receipt`, `model_judgment_requires_independent_evidence`). This is stricter/more inspectable than the stale single-reason prose and does not create verification truth.

**Required T049 action:** converge `data-model.md` to the implemented closed status/reason set and preserve the statement that the assessment is derived/non-authoritative.

### C-002 — Journal domain-separator prose differs from the canonical implementation/golden

**Severity:** MUST converge documentation; implementation/golden behavior is internally consistent and exact-head tested.

`plan.md` shows an illustrative digest string containing `ecra/verification-journal-entry/v1\0`, while the implemented `journal.rs` and fixed golden contract use the canonical domain separator:

```text
ecra/verification-journal/v1\0
```

The requirement is domain-separated canonical SHA-256 and the committed golden fixes the implementation value. No historical record exists outside the implementation branch requiring migration.

**Required T049 action:** update plan/data-model/quickstart wording where necessary so only the implemented/golden domain separator is normative.

### C-003 — Checkpoint satisfying-state prose omits implemented Inconclusive rejection

**Severity:** MUST converge documentation; implementation is fail-closed.

The implementation rejects `Absent`, `Inconclusive`, and `Conflicted` as satisfying states. `data-model.md` explicitly names `Absent` and `Conflicted` but does not name `Inconclusive` in the prohibition sentence even though the spec/acceptance semantics already require inconclusive checkpoints not to satisfy verified completion.

**Required T049 action:** state explicitly that `Absent`, `Inconclusive`, and `Conflicted` are prohibited satisfying states; `Verified` is normal and `Rejected` is allowed only for an explicit negative requirement.

## Verified non-drifts

- No second `VerificationReceipt` or target namespace exists.
- `ActionReceipt` remains executor-observed and cannot self-verify.
- No verified flag was added to Fact/Artifact/run metadata.
- Reconciliation binds exact RunId/ActionAttemptRef/ActionRef and retains supporting verification IDs.
- IC-003 empty support is limited to evidence-absent `still_unknown`; conclusive outcomes require support.
- ECR-004 takes `RunState` read-only and exposes no event/receipt/run-resolution bridge.
- `semantically_retryable*` remains advisory for a future new-attempt proposal only.
- ECR-002 same-run RunResumed/ExecutionCompleted/blind-retry guards remain authoritative.
- Journal rows are append-only under store APIs/triggers; projections are rebuildable and non-authoritative.
- Integrity wording does not claim hostile complete-store tamper resistance.
- Persistence acceptance is synthetic/non-sensitive references/digests/metadata only.
- No browser/network/model/provider/process/policy/authorization/identity/telemetry direct runtime dependency entered `ecra-verify`.
- `url`/`zip` are inherited only through canonical upstream workspace crates and are not ECR-004-owned capabilities.
- Resource bounds and offline behavior are explicitly tested.
- No donor implementation source entered the slice.

## Lifecycle/documentation convergence required by T049

In addition to C-001–C-003, lifecycle prose must be brought to live truth before T050:

1. `EXECUTION.md` still describes ECR-004 as pre-implementation planning and must describe the implementation/review frontier.
2. ECR-004 `STATUS.md` must record T045 exact-head Phase 7 evidence and the Phase 8 frontier.
3. platform `STATUS.md`, roadmap/index navigation and `specs/README.md` must not describe ECR-004 as merely planned once implementation is ready for final review.
4. `tasks.md` completion markers may be converged for tasks whose implementation/evidence is already complete; T050–T053 remain lifecycle gates until their own evidence exists.
5. PR #6 body is stale and must be updated before review-ready transition.

## Analyze conclusion

**Result:** `CONVERGENCE_REQUIRED_NO_IMPLEMENTATION_BLOCKER`.

The only open work from this analyze pass is explicit T049 documentation/lifecycle convergence. No new implementation task is required and no requirement may be weakened to close the drifts. After T049, the complete final feature head must pass T050 exact-head CI before PR #6 can leave Draft.