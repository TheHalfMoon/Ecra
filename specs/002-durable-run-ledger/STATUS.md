# ECR-002 Status — Durable Run, Ledger & Budgets

**Slice:** ECR-002  
**Lifecycle:** IMPLEMENTING  
**Dependency:** ECR-001 `CLOSED_CANONICAL`  
**Branch:** `002-durable-run-ledger`  
**Base main:** `5caf5dc4e7f26d07fabac3333713a44f0af22ea1`  
**PR:** #2 — OPEN / DRAFT  
**Constitution:** v1.1.0

This is the active ECR-002 execution ledger. Normative semantics live in `spec.md`, `data-model.md`, `contracts/run-ledger-v1.md`, and approved/folded convergence updates.

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

## Phase 1 — VERIFIED_ON_BRANCH

T001–T008 are complete on the verified workspace baseline:

```text
exact head  4577123486fcaf856a3640aeacb3b7dcee733cc3
CI run      33105751992 — SUCCESS
runner      macbook — self-hosted macOS
Rust        1.98.0
```

The gate passed locked workspace build, rustfmt, strict Clippy, workspace tests, ECR-001 regression targets, ECR-002 package tests, rustdoc, offline replay, and both crate boundary suites.

The generated `Cargo.lock` is committed with SHA-256 `b720472bf40a554ab61afb74eae95dd625bc6b2604e47a632991faea630e42c6`. Temporary bootstrap/lock-helper workflows were removed; `.github/workflows/ecr-002.yml` is the trusted push-only exact-head gate.

## Current implementation position

```text
Phase 1 T001–T008  VERIFIED_ON_BRANCH
Phase 2 T009–T018  ACTIVE
Phase 3 T019–T026  blocked by Phase 2 exact-head verification
Phase 4 T027–T034  blocked
Phase 5 T035–T044  blocked
Phase 6 T045–T051  blocked
Phase 7 T052–T059  blocked
Phase 8 T060–T066  blocked
Phase 9 T067–T073  blocked
```

Immediate work:

```text
T009 typed RunError taxonomy
T010 EventSequence + BudgetAmount wrappers
T011 LedgerDigest + canonical domain-separated digest
T012 RunPhase + SuspensionReason + RunErrorSummary
T013 strict v1 RunEvent bodies
T014 strict RunEventEnvelope validation
T015–T018 golden/valid/invalid/error-matrix contract evidence
```

## Active implementation clarification

`implementation-clarifications.md` C1 fixes numeric parser bounds that planning described only as “bounded”:

```text
SuspensionReason::other.code  <= 256 UTF-8 bytes and non-empty
intervention_recorded.note    <= 4096 UTF-8 bytes when present
```

C1 must be folded into the primary data model/contract before canonical closure.

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
