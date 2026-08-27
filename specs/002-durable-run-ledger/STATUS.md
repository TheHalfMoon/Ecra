# ECR-002 Status — Durable Run, Ledger & Budgets

**Slice:** ECR-002  
**Lifecycle:** IMPLEMENTING  
**Dependency:** ECR-001 `CLOSED_CANONICAL`  
**Branch:** `002-durable-run-ledger`  
**Base main:** `5caf5dc4e7f26d07fabac3333713a44f0af22ea1` — planning/status CI `33103802150` SUCCESS  
**PR:** Draft PR to be opened from this branch after lifecycle activation  
**Constitution:** v1.1.0

This is the active ECR-002 execution ledger. Normative semantics live in `spec.md`, `data-model.md`, `contracts/run-ledger-v1.md`, and approved convergence updates.

## Canonical planning gate

```text
planning package commit  c83a208ad84b2d1da892a80a6911989eaff25ade
synchronized main head   5caf5dc4e7f26d07fabac3333713a44f0af22ea1
main CI                   33103802150 — SUCCESS
analyze                    ZERO_BLOCKING_PLANNING_DRIFT_FOUND
FR-001–FR-057             OWNED
SC-001–SC-016             OWNED
G1–G15                    PASS / explicit N/A
```

## Current implementation position

```text
Phase 1 T001–T008  ACTIVE
Phase 2 T009–T018  blocked by Phase 1 baseline
Phase 3 T019–T026  blocked
Phase 4 T027–T034  blocked
Phase 5 T035–T044  blocked
Phase 6 T045–T051  blocked
Phase 7 T052–T059  blocked
Phase 8 T060–T066  blocked
Phase 9 T067–T073  blocked
```

Immediate work:

```text
T001 add crates/ecra-run workspace skeleton
T002 add exact reviewed dependency candidates + lockfile
T003 forbid unsafe + architecture/misuse docs
T004/T005 add dependency/unsafe boundary scripts
T006 add trusted push-only ECR-002 CI
T007 update donor/license dependency ledger
T008 prove first workspace head green before semantic implementation
```

## Fixed implementation boundaries

```text
authoritative run truth     append-only ordered events
projection                  rebuildable/non-authoritative
attempt before effect       durable commit required
missing receipt             UNKNOWN/reconciliation-required
local store                 SQLite / rusqlite, WAL + FULL
write transaction           Immediate + expected-head compare
budget arithmetic           typed checked I-JSON-safe integers
portable artifact           deterministic strict Stored-only .ecra ZIP
real sensitive persistence  NOT AUTHORIZED
provider/network execution  NOT IN ECR-002
hostile tamper resistance   NOT CLAIMED
```

## Downstream ownership preserved

- authentication/trust roots/protected storage -> ECR-031;
- authorization/declassification/approval/budget-revision policy -> ECR-003;
- independent verification/reconciliation decisions -> ECR-004;
- provider/browser/model/tool/process execution -> later owning slices;
- telemetry/privacy/redaction product controls -> ECR-025.

## Evidence discipline

- no task/phase PASS without required exact-head evidence;
- any code/test/workflow/contract/status mutation moves the verification head;
- no Ready/merge until full exact-head ECR-002 gate + clean reviews;
- no `CLOSED_CANONICAL` until exact-head merge + canonical-main gate + closure ledger.
