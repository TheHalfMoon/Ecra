# ECR-002 Status — Durable Run, Ledger & Budgets

**Slice:** ECR-002  
**Lifecycle:** IMPLEMENTING / PHASE_9_PRE_MERGE_CONVERGENCE  
**Dependency:** ECR-001 `CLOSED_CANONICAL`  
**Branch:** `002-durable-run-ledger`  
**Base main:** `5caf5dc4e7f26d07fabac3333713a44f0af22ea1`  
**PR:** #2 — OPEN / DRAFT  
**Constitution:** v1.1.0

This is the active ECR-002 execution ledger. Normative semantics live in `spec.md`, `data-model.md`, and `contracts/run-ledger-v1.md`. `implementation-clarifications.md` is now historical only because its C1 bounds were folded into the primary normative documents during T070 convergence.

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

## Verified implementation phases

### Phase 1 — T001–T008 VERIFIED_ON_BRANCH

```text
exact head  4577123486fcaf856a3640aeacb3b7dcee733cc3
CI run      33105751992 — SUCCESS
```

Workspace/crate/CI/dependency boundaries established with generated locked dependency graph and permanent push-only ECR-002 workflow.

### Phase 2 — T009–T018 VERIFIED_ON_BRANCH

```text
exact head  2ab8d6d80f43bf7dd07ee43659555a573c47021b
CI run      33107289499 — SUCCESS
job         98640449273 — SUCCESS
```

Strict v1 errors, integer wrappers, event envelope, run events and domain-separated RFC8785+SHA-256 `LedgerDigest` established with valid/invalid fixtures and golden bytes/hash.

### Phase 3 — T019–T026 VERIFIED_ON_BRANCH

```text
exact head  ac45fcc835674341ae6b9ad18484e6dacda36809
CI run      33143735332 — SUCCESS
```

Pure deterministic reducer, exact phase matrix, attempt/receipt binding, recovery-boundary UNKNOWN semantics, resume blockers and 1,000x deterministic replay established.

### Phase 4 — T027–T034 VERIFIED_ON_BRANCH

```text
exact head  69f65ab5b07e6c8a0dbabec6681123c67ae01f5a
CI run      33145231800 — SUCCESS
job         98764652133 — SUCCESS
```

All 14 budget dimensions, checked I-JSON-safe accounting, preflight refusal, soft/hard threshold evidence, hard suspension and loop-stop evidence established.

### Phase 5 — T035–T044 VERIFIED_ON_BRANCH

```text
exact head  90dfb87a2b17ba749663d999c4659ad4244bd131
CI run      33145935409 — SUCCESS
job         98766883647 — SUCCESS
```

SQLite WAL+FULL configuration/read-back, STRICT schema v1, append-only events, Immediate expected-head append, authoritative replay, rebuildable projections, synthetic blobs, mutation rejection and process-crash durability established.

### Phase 6 — T045–T051 VERIFIED_ON_BRANCH

```text
exact head  04d51e913c88e38d2730950e711ab498a3b6e296
CI run      33146742762 — SUCCESS
job         98769387841 — SUCCESS
```

Durable prepare-before-effect, exact receipt persistence, recovery scanning/boundaries, retry refusal, crash matrix A–D, distinct attempts and competing-writer evidence established.

### Phase 7 — T052–T059 VERIFIED_ON_BRANCH

```text
implementation commit  4cf186372d27e90ad78b4e5e22c28b390e01da89
exact verified head    ff4031302e30a46d3d15d2928548f7e8c19e5d9c
focused helper run     33151102307 — SUCCESS
permanent CI run       33151219953 — SUCCESS
job                    98783466698 — SUCCESS
```

Strict deterministic Stored-only `.ecra` archive profile, parser limits, malicious corpus, manifest/content/ledger validation, reducer validation, deterministic golden bytes and no SQLite/WAL export established. Temporary helper files were removed before the stable implementation state.

### Phase 8 — T060–T066 VERIFIED_ON_BRANCH

```text
phase8 ledger head  e86e1822e621c0563f2764fe784902e3204b0085
CI run              33152251783 — SUCCESS
job                 98786745867 — SUCCESS
Rust/Cargo           1.98.0
Cargo.lock SHA-256   b720472bf40a554ab61afb74eae95dd625bc6b2604e47a632991faea630e42c6
rusqlite             0.40.2, bundled
libsqlite3-sys       0.38.2
bundled SQLite       3.53.2
zip                  8.6.0, default-features=false
```

The permanent gate passed locked build, rustfmt, strict Clippy, workspace tests, ECR-001 regressions, all explicit ECR-002 targets including archive/boundaries/portability, rustdoc, offline replay, both core/run unsafe+dependency checks and exact dependency evidence.

Phase 8 also established executable source-purity/no-provider/no-network scans, synthetic-fixture secret-marker auditing, documentation non-claim enforcement, LF/CRLF/JSON portability and the run architecture/misuse map.

## Phase 9 — ACTIVE

### T067 — COMPLETE_ON_BRANCH

`traceability-closure.md` maps FR-001–FR-057 and SC-001–SC-016 to implementation/test/contract evidence.

### T068 — COMPLETE_ON_BRANCH

The same artifact re-checks constitution G1–G15 and platform risks R-006/R-019/R-033/R-039/R-042/R-052/R-053 with zero implicitly accepted Critical risk.

### T069 — COMPLETE_ON_BRANCH

`post-implementation-analyze.md` found:

```text
UNOWNED_FR=0
UNOWNED_SC=0
FAILED_CONSTITUTION_GATES=0
IMPLICITLY_ACCEPTED_CRITICAL_RISKS=0
MUST_LEVEL_IMPLEMENTATION_DEFECTS_FOUND=0
CONVERGENCE_DRIFT_FOUND=4
```

The four findings were documentation/convergence drift only; no production semantic redesign was required.

### T070 — ACTIVE

Completed convergence work so far:

```text
C1 numeric UTF-8 bounds folded into data-model.md                      COMPLETE
C1 numeric UTF-8 bounds folded into contracts/run-ledger-v1.md        COMPLETE
implementation-clarifications.md made historical/non-normative        COMPLETE
plan.md dependency/lifecycle wording converged                        COMPLETE
research.md implementation resolution for rusqlite 0.40.2 added       COMPLETE
quickstart.md converged to implemented verification surface            COMPLETE
EXECUTION.md converged from Phase 5 to Phase 9 truth                  COMPLETE
spec.md semantic re-check                                              NO SEMANTIC DRIFT FOUND
threat-model.md security acceptance re-check                           NO SECURITY DRIFT FOUND
STATUS.md/tasks.md/post-analysis synchronization                       ACTIVE
final complete exact-head CI after convergence                         REQUIRED
```

Latest green convergence baseline before the remaining ledger synchronization was:

```text
head    0dd8ef6efc8a61197d6520c94a54f661b07961a4
CI run  33152748423 — SUCCESS
```

Any T070 documentation mutation after that SHA moves the verification head. T070 is not complete until the final converged head passes the full permanent ECR-002 gate.

## Current implementation position

```text
Phase 1 T001–T008  VERIFIED_ON_BRANCH
Phase 2 T009–T018  VERIFIED_ON_BRANCH
Phase 3 T019–T026  VERIFIED_ON_BRANCH
Phase 4 T027–T034  VERIFIED_ON_BRANCH
Phase 5 T035–T044  VERIFIED_ON_BRANCH
Phase 6 T045–T051  VERIFIED_ON_BRANCH
Phase 7 T052–T059  VERIFIED_ON_BRANCH
Phase 8 T060–T066  VERIFIED_ON_BRANCH
Phase 9 T067        COMPLETE_ON_BRANCH
Phase 9 T068        COMPLETE_ON_BRANCH
Phase 9 T069        COMPLETE_ON_BRANCH
Phase 9 T070        ACTIVE
Phase 9 T071–T073   BLOCKED_BY_DEPENDENCY_ORDER
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

## Remaining closure order

```text
finish T070 convergence synchronization
→ full exact-head ECR-002 CI on final converged feature head
→ T071 clean PR/review/check gate
→ T072 exact expected-head non-rebase merge
→ post-merge canonical-main ECR-002 CI
→ T073 CLOSED_CANONICAL ledger/platform/roadmap/EXECUTION convergence
→ identify next genuinely dependency-eligible slice from canonical main
```

## Evidence discipline

- no task/phase PASS without the required exact-head evidence;
- any code/test/workflow/contract/status/convergence mutation moves the verification head;
- no Ready/merge until full exact-head ECR-002 gate + clean reviews/checks;
- no `CLOSED_CANONICAL` until exact-head merge + canonical-main gate + closure ledger convergence.
