# ECR-002 Status — Durable Run, Ledger & Budgets

**Slice:** ECR-002  
**Lifecycle:** TASKS_READY  
**Dependency:** ECR-001 `CLOSED_CANONICAL`  
**Branch:** not yet created at planning gate  
**PR:** none at planning gate  
**Constitution:** v1.1.0

This file is the active execution ledger once ECR-002 planning is canonical. Normative semantics live in `spec.md`, `data-model.md`, `contracts/run-ledger-v1.md`, and approved convergence updates.

## Planning gate

```text
spec.md                  COMPLETE
research.md              COMPLETE
run-ledger-v1 contract   COMPLETE
data-model.md             COMPLETE
threat-model.md           COMPLETE
plan.md                   COMPLETE
quickstart.md             COMPLETE
tasks.md                  COMPLETE (T001–T073)
requirements checklist   PASS
analyze.md                ZERO_BLOCKING_PLANNING_DRIFT_FOUND
constitution G1–G15      PASS / N/A where explicitly scoped
```

## Authorized scope

ECR-002 may implement:
- local serialized/replayable run state;
- append-only ordered run events;
- exact ActionAttemptRef preparation/receipt durability;
- UNKNOWN/unresolved recovery hooks;
- typed bounded resource accounting;
- SQLite local store with WAL + FULL semantics;
- schema migration fixtures;
- deterministic synthetic/non-sensitive `.ecra` archives;
- content-addressed synthetic blobs;
- cancellation/intervention/recovery events.

## Explicitly not authorized

```text
principal authentication / trust roots / protected keys
real-sensitive-state storage enablement
authorization / declassification / approval policy
independent verification / reconciliation decisions
browser/model/tool/process provider execution
remote/network/cloud durability
multi-device sync
distributed workflow service
hash-chain hostile-tamper-resistance claims
```

Owners remain ECR-031/ECR-003/ECR-004 and later slices.

## Implementation order

```text
Phase 1 T001–T008  workspace/crate/CI/dependencies
Phase 2 T009–T018  errors/primitives/events/digest
Phase 3 T019–T026  pure reducer/state machine
Phase 4 T027–T034  budgets
Phase 5 T035–T044  SQLite/migrations/store/projections
Phase 6 T045–T051  attempt guard/recovery/concurrency
Phase 7 T052–T059  deterministic .ecra
Phase 8 T060–T066  portability/security/docs/gates
Phase 9 T067–T073  traceability/convergence/review/merge/closure
```

## Next exact actions after this package reaches canonical `main`

```text
1. verify canonical main SHA and no competing PR/branch state
2. create branch `002-durable-run-ledger` from that exact main SHA
3. update branch-local roadmap/platform STATUS/EXECUTION/this STATUS to IMPLEMENTING
4. open Draft PR
5. execute T001
6. run the ECR-001 regression gate before broad semantic work
7. continue task dependency order without skipping gates
```

## Evidence discipline

- never claim a task/phase PASS without exact-head evidence where required;
- any final code/test/workflow/contract/ledger mutation moves the verification head;
- no merge until clean exact-head CI/review state;
- no `CLOSED_CANONICAL` until merge + canonical-main ECR-002 gate + closure-ledger convergence.
