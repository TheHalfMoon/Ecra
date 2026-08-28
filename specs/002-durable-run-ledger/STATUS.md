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

## Phase 3 — VERIFIED_ON_BRANCH

T019–T026 are complete on the exact verified Phase 3 head:

```text
exact head  ac45fcc835674341ae6b9ad18484e6dacda36809
CI run      33143735332 — SUCCESS
```

The exact-head gate passed locked build, rustfmt, strict Clippy, workspace tests, ECR-001 regressions, ECR-002 reducer/attempt/portability targets, rustdoc, offline replay, and both boundary suites.

Phase 3 establishes the pure deterministic reducer, exact v1 phase matrix, attempt/receipt binding, recovery-boundary UNKNOWN semantics, resume blockers, and 1,000x deterministic replay evidence.

## Phase 4 — VERIFIED_ON_BRANCH

T027–T034 are complete on the exact verified Phase 4 head:

```text
exact head  69f65ab5b07e6c8a0dbabec6681123c67ae01f5a
CI run      33145231800 — SUCCESS
job         run-ledger / 98764652133 — SUCCESS
```

The exact-head gate passed locked build, rustfmt, strict Clippy, workspace tests, ECR-001 regressions, explicit ECR-002 event/reducer/attempt/budget/portability targets, rustdoc, offline replay, and both boundary suites.

Phase 4 establishes all 14 fixed budget dimensions, strict I-JSON-safe checked accounting, remaining-budget/preflight refusal, first-crossing soft evidence, exact hard-exhaustion evidence/suspension, deterministic hard stops, and preservation of unresolved-attempt truth.

## Phase 5 — VERIFIED_ON_BRANCH

T035–T044 are complete on the exact verified Phase 5 head:

```text
exact head  90dfb87a2b17ba749663d999c4659ad4244bd131
CI run      33145935409 — SUCCESS
job         run-ledger / 98766883647 — SUCCESS
```

The exact-head gate passed locked build, rustfmt, strict Clippy, workspace tests, ECR-001 regression targets, explicit ECR-002 migration/SQLite store/crash-recovery targets, rustdoc, offline replay, and both boundary suites.

Phase 5 establishes WAL + FULL SQLite configuration with read-back, deterministic STRICT schema v1 and append-only authoritative events, transactional schema handling, Immediate expected-head append, reducer-before-commit validation, strict authoritative replay, rebuildable projections, synthetic content-addressed blobs, mutation/corruption rejection, and process-crash durability evidence.

## Current implementation position

```text
Phase 1 T001–T008  VERIFIED_ON_BRANCH
Phase 2 T009–T018  VERIFIED_ON_BRANCH
Phase 3 T019–T026  VERIFIED_ON_BRANCH
Phase 4 T027–T034  VERIFIED_ON_BRANCH
Phase 5 T035–T044  VERIFIED_ON_BRANCH
Phase 6 T045–T051  ACTIVE
Phase 7 T052–T059  blocked by Phase 6 exact-head verification
Phase 8 T060–T066  blocked
Phase 9 T067–T073  blocked
```

Phase 6 semantic mutation starts only after a full exact-head ECR-002 gate passes on this Phase 5 ledger state.

Phase 6 candidate implementation was materialized and passed its focused pre-commit gate at implementation commit `2105f1f4853b7c7b4f0a48484ba635d0eddbdd67`; T045–T051 remain ACTIVE until the permanent full exact-head gate passes on the candidate-plus-ledger head.

Immediate work:

```text
T045 durable prepare_attempt store API
T046 exact-bound record_receipt store API
T047 recovery scan + explicit recovery-boundary append
T048 ECR-001-preserving retry guard + unresolved blind-retry block
T049 crash matrix A–D
T050 multiple-attempt/one-action distinctness + receipt cross-bind rejection
T051 two-connection expected-head concurrency
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
