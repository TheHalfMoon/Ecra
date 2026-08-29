# ECR-004 Status — Verification & Reconciliation

**Slice:** ECR-004  
**Lifecycle:** TASKS_READY_CANDIDATE / PLANNING_NON_CANONICAL  
**Dependencies:** ECR-001 `CLOSED_CANONICAL`, ECR-002 `CLOSED_CANONICAL`  
**Planning base:** `f6d8eb6ff6a60aa0ad8a6f52686a62f12cd374b0`  
**Planning branch:** `004-verification-receipts`  
**Planning PR:** #5  
**Constitution:** v1.1.0

ECR-004 is independently planning-eligible from ECR-001/ECR-002. This branch contains planning only and does not authorize implementation until the package is merged to canonical `main` and the exact merged planning state passes the required ECR-001/ECR-002 regression gates.

## Planning package

```text
spec.md
research.md
data-model.md
contracts/verification-reconciliation-v1.md
threat-model.md
plan.md
tasks.md
quickstart.md
implementation-clarifications.md
analyze.md
checklists/requirements.md
STATUS.md
```

## Frozen v1 boundaries

- reuse ECR-001 `VerificationReceipt` as the only canonical independent verification record;
- `ActionReceipt` remains executor-observed execution evidence and never self-verifies;
- no second `verified` flag on Fact/Artifact/run metadata;
- exact target/evidence/verifier/method/outcome binding;
- deterministic aggregate states: `Absent`, `Verified`, `Rejected`, `Inconclusive`, `Conflicted`;
- critical verification checkpoints are requirements, not authority;
- exact ECR-002 UNKNOWN attempt reconciliation produces `effect_confirmed`, `no_effect_confirmed`, or `still_unknown` without fabricating `ActionReceipt`;
- retry disposition is fail-closed advisory metadata for a future new-attempt proposal only, never execution authorization or same-run scheduling;
- every reconciliation outcome leaves ECR-002 `RunState`, prepared-attempt receipt/unresolved state, `unresolved_attempts`, and `RunPhase` unchanged;
- ECR-002 `RunEvent` v1 wire contract is unchanged and no run-resolution event is introduced;
- ECR-004 uses a separate append-only verification journal with rebuildable indexes;
- no sidecar projection represents or mutates ECR-002 run resolution;
- journal hash chaining is corruption/substitution detection only, not hostile complete-store tamper resistance;
- acceptance persists synthetic/non-sensitive evidence metadata/references/digests only;
- no browser/network/model/provider/process/policy/identity-backend execution dependency enters v1.

## Analyze history

### Pass 1 — A-001

Blocking issue: canonical ECR-001 `EvidenceRef` keeps decision-grade metadata private and exposes only `id()`/`kind()`.

Resolution: IC-001 authorizes only read-only accessors for existing artifact/observation/receipt/external-ref/content-digest/as-of fields, with no wire/canonical/validation change and full ECR-001 regressions. `tasks.md` T011A owns the prerequisite.

### Pass 2 review — A-002

Blocking issue discovered before merge: ECR-002 v1 removes an unresolved attempt only when a real `ReceiptRecorded` is accepted. `ReconciliationRequested` does not resolve it. Original retry-advisory wording could therefore be misread as if `no_effect_confirmed` made the same run directly retryable.

Resolution: IC-002 + FR-046 + SC-013 freeze a read-only compatibility boundary. ECR-004 records effect truth and advisory new-attempt semantics only. It does not clear `unresolved_attempts`, mutate `PreparedAttemptState`, append an ECR-002 event/receipt, resume/complete the existing run, or schedule a retry. Phase 5 and final gates require explicit ECR-002 compatibility proof.

### Analyze Pass 3

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

## Planned implementation architecture

After planning becomes canonical and exact dependency regressions pass, create a fresh implementation branch from that exact canonical head and add one `crates/ecra-verify` crate:

```text
error.rs
ids.rs
request.rs
evidence.rs
aggregate.rs
checkpoint.rs
reconcile.rs
journal.rs
store.rs
```

The crate consumes canonical ECR-001/ECR-002 types and keeps pure verification logic separate from local sidecar journal I/O. Its ECR-002 dependency is read-only with respect to run resolution.

## Current execution state

```text
CURRENT_TASK                    PLANNING_PR_REVIEW_AFTER_PASS_3
CURRENT_STATE                   TASKS_READY_CANDIDATE_NON_CANONICAL
PLANNING_BASE                   f6d8eb6ff6a60aa0ad8a6f52686a62f12cd374b0
PLANNING_BRANCH                 004-verification-receipts
PLANNING_PR                     5
ANALYZE_PASS                    3
ANALYZE_RESULT                  ZERO_BLOCKING_PLANNING_DRIFT_FOUND
IMPLEMENTATION_AUTHORIZED       NO
NEXT_IF_PLANNING_MERGED_GREEN   CREATE_IMPLEMENTATION_BRANCH_FROM_EXACT_CANONICAL_HEAD
```

## Canonical next steps

1. converge platform lifecycle/index/PR text with Analyze Pass 3 and IC-002;
2. process all actionable PR #5 planning-review findings on the exact converged head;
3. merge the exact planning head by an allowed non-rebase method only when review/mergeability are clean;
4. freeze the resulting canonical `main` SHA;
5. require ECR-001 and ECR-002 permanent workflows to succeed on that exact canonical head;
6. create `004-verification-receipts-impl` from that exact eligible head;
7. execute `tasks.md` from T001 in dependency order.

## Parallel ECR-031 boundary

ECR-031 is a separate active implementation PR and currently has a native macOS provisioning prerequisite. ECR-004 does not depend on ECR-031, so planning/implementation may proceed independently once its own canonical gates pass. ECR-004 must not absorb ECR-031 identity/protected-storage scope or use its blocker as justification to persist real sensitive evidence.

ECR-005 remains blocked by its complete dependency set and does not become eligible merely because ECR-004 planning is ready.