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

## Phase 2 — VERIFIED_ON_BRANCH

T009–T018 are complete on the exact verified Phase 2 head:

```text
exact head  2ab8d6d80f43bf7dd07ee43659555a573c47021b
CI run      33107289499 — SUCCESS
job         run-ledger / 98640449273 — SUCCESS
```

The exact-head gate passed locked build, rustfmt, strict Clippy, workspace tests, ECR-001 regression contract targets, ECR-002 event contract targets, rustdoc, offline replay, and both unsafe/dependency boundary suites.

Phase 2 establishes the strict v1 error taxonomy, integer wrappers, run phases/suspension reasons, all 17 run-event kinds, strict envelopes, domain-separated RFC 8785 + SHA-256 `LedgerDigest`, fixtures, golden canonical bytes/digest, and machine-readable error coverage.

## Current implementation position

```text
Phase 1 T001–T008  VERIFIED_ON_BRANCH
Phase 2 T009–T018  VERIFIED_ON_BRANCH
Phase 3 T019–T026  ACTIVE
Phase 4 T027–T034  blocked by Phase 3 exact-head verification
Phase 5 T035–T044  blocked by Phase 3 exact-head verification
Phase 6 T045–T051  blocked
Phase 7 T052–T059  blocked
Phase 8 T060–T066  blocked
Phase 9 T067–T073  blocked
```

Immediate work:

```text
T019 derived RunState + PreparedAttemptState + ordered projections
T020 pure RunReducer
T021 exact transition matrix + terminal rejection
T022 attempt uniqueness and exact receipt binding
T023 recovery-boundary unresolved semantics
T024 v1 resume blockers
T025 exhaustive transition tests
T026 deterministic 1,000x replay property evidence
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
