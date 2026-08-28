# ECR-002 Status — Durable Run, Ledger & Budgets

**Slice:** ECR-002  
**Lifecycle:** CLOSED_CANONICAL  
**Dependency:** ECR-001 `CLOSED_CANONICAL`  
**Branch:** `002-durable-run-ledger` — merged  
**PR:** #2 — MERGED  
**Final feature head:** `87fd9fc560bf5ca21a07a4d25473f305b4c05f05`  
**Final feature-head CI:** `33153413462` / job `98790541842` — SUCCESS  
**Merge commit:** `40efc8a64a9562f0f3eb2555b350cfa03d3e0675`  
**Post-merge main ECR-002 CI:** `33154108410` / job `98792690359` — SUCCESS  
**Post-merge main ECR-001 regression CI:** `33154108397` / job `98792690901` — SUCCESS  
**Review state:** CodeRabbit SUCCESS; zero formal reviews, zero inline review comments, zero review threads; Qodo billing notice informational only  
**Constitution:** v1.1.0

This is the canonical ECR-002 closure ledger required by `AGENTS.md`. Normative semantics live in `spec.md`, converged `data-model.md`, `contracts/run-ledger-v1.md`, implementation truth on canonical `main`, and the evidence recorded below. `implementation-clarifications.md` is historical/non-normative because C1 was folded into the primary contract during T070.

## Canonical closure

```text
Phase 1  T001–T008   VERIFIED_ON_BRANCH_AND_MERGED
Phase 2  T009–T018   VERIFIED_ON_BRANCH_AND_MERGED
Phase 3  T019–T026   VERIFIED_ON_BRANCH_AND_MERGED
Phase 4  T027–T034   VERIFIED_ON_BRANCH_AND_MERGED
Phase 5  T035–T044   VERIFIED_ON_BRANCH_AND_MERGED
Phase 6  T045–T051   VERIFIED_ON_BRANCH_AND_MERGED
Phase 7  T052–T059   VERIFIED_ON_BRANCH_AND_MERGED
Phase 8  T060–T066   VERIFIED_ON_BRANCH_AND_MERGED
Phase 9  T067–T073   COMPLETE
Lifecycle             CLOSED_CANONICAL
```

T071 is satisfied because the exact final feature head passed the complete ECR-002 gate, PR #2 was Ready/mergeable, CodeRabbit completed successfully, and no actionable review thread/comment remained. T072 is satisfied because PR #2 merged only the exact verified feature head using a non-rebase merge commit and canonical main passed the complete ECR-002 gate. T073 is this canonical closure convergence.

## Final pre-merge evidence

```text
Head:          87fd9fc560bf5ca21a07a4d25473f305b4c05f05
ECR-002 run:   33153413462
Job:           98790541842
Runner:        macbook — self-hosted macOS
Result:        SUCCESS
CodeRabbit:    SUCCESS — Review completed
Review threads: 0
```

The exact final feature head passed locked build, rustfmt, strict Clippy, full workspace tests, ECR-001 regression targets, all explicit ECR-002 contract/security/migration/crash/archive/portability targets, rustdoc, offline replay, core/run unsafe and dependency boundaries, and exact dependency evidence.

## Merge and post-merge evidence

```text
PR:                    #2 — MERGED
Expected feature head:  87fd9fc560bf5ca21a07a4d25473f305b4c05f05
Merge method:           merge commit / non-rebase
Merge commit:           40efc8a64a9562f0f3eb2555b350cfa03d3e0675
Merge parent 1:         5caf5dc4e7f26d07fabac3333713a44f0af22ea1
Merge parent 2:         87fd9fc560bf5ca21a07a4d25473f305b4c05f05
Main ECR-002 run:       33154108410
Main ECR-002 job:       98792690359
Main ECR-002 result:    SUCCESS
Main ECR-001 run:       33154108397
Main ECR-001 job:       98792690901
Main ECR-001 result:    SUCCESS
```

Canonical `main` at the merge commit passed the complete ECR-002 verification surface after the merge, and the closed ECR-001 regression workflow also remained green.

## Phase evidence index

```text
Phase 1  head 4577123486fcaf856a3640aeacb3b7dcee733cc3  CI 33105751992
Phase 2  head 2ab8d6d80f43bf7dd07ee43659555a573c47021b  CI 33107289499  job 98640449273
Phase 3  head ac45fcc835674341ae6b9ad18484e6dacda36809  CI 33143735332
Phase 4  head 69f65ab5b07e6c8a0dbabec6681123c67ae01f5a  CI 33145231800  job 98764652133
Phase 5  head 90dfb87a2b17ba749663d999c4659ad4244bd131  CI 33145935409  job 98766883647
Phase 6  head 04d51e913c88e38d2730950e711ab498a3b6e296  CI 33146742762  job 98769387841
Phase 7  head ff4031302e30a46d3d15d2928548f7e8c19e5d9c  CI 33151219953  job 98783466698
Phase 8  head e86e1822e621c0563f2764fe784902e3204b0085  CI 33152251783  job 98786745867
T070     head 84d8cb5a8c0a28ab7adba42d2cd049e014c8f368  CI 33153174953  job 98789740534
T071     head 87fd9fc560bf5ca21a07a4d25473f305b4c05f05  CI 33153413462  job 98790541842
```

## Final dependency/toolchain evidence

```text
Rust/Cargo           1.98.0
Cargo.lock SHA-256   b720472bf40a554ab61afb74eae95dd625bc6b2604e47a632991faea630e42c6
rusqlite             0.40.2, bundled
libsqlite3-sys       0.38.2
bundled SQLite       3.53.2
zip                  8.6.0, default-features=false
```

## Final implemented boundaries

```text
authoritative run truth     append-only ordered RunEventEnvelope history
projection                  rebuildable/non-authoritative RunState cache
attempt before effect       durable AttemptPrepared commit required
missing receipt             UNKNOWN/reconciliation-required
local store                 SQLite via rusqlite, WAL + synchronous=FULL
write transaction           Immediate + expected-head compare
budget arithmetic           typed checked I-JSON-safe integers
portable artifact           deterministic strict Stored-only .ecra ZIP
Ecra-owned unsafe           forbidden in ecra-run
real sensitive persistence  NOT AUTHORIZED by ECR-002
provider/network execution  NOT IN ECR-002
hostile tamper resistance   NOT CLAIMED for plain LedgerDigest chain
```

## Traceability / convergence result

```text
FR-001–FR-057                         PASS
SC-001–SC-016                         PASS with feature/post-merge evidence complete
G1–G15                                PASS / explicit PASS-N/A
UNOWNED_FR                             0
UNOWNED_SC                             0
FAILED_CONSTITUTION_GATES              0
IMPLICITLY_ACCEPTED_CRITICAL_RISKS     0
MUST_LEVEL_IMPLEMENTATION_DEFECTS      0
CONVERGENCE_DRIFT_FOUND                4
CONVERGENCE_DRIFT_REMEDIATED           4
```

C1 deterministic UTF-8 bounds are normative in `data-model.md` and `contracts/run-ledger-v1.md`:

```text
SuspensionReason::other.code  1..=256 bytes
intervention_recorded.note    0..=4096 bytes when present
```

## Downstream ownership preserved

- identity/principal assertions, trust roots, key lifecycle and protected sensitive storage -> ECR-031;
- authorization/declassification/approval/budget-revision policy -> ECR-003;
- independent verification and UNKNOWN reconciliation decisions -> ECR-004;
- provider/browser/model/tool/process execution -> later owning slices;
- telemetry/privacy/redaction product controls -> ECR-025.

## Next dependency-eligible work

Closing ECR-002 makes two roadmap slices dependency-eligible for bounded planning:

```text
ECR-031 Identity, Trust Root & Sensitive Storage Foundations
  depends on ECR-001 + ECR-002

ECR-004 Verification & Reconciliation
  depends on ECR-001 + ECR-002
```

The selected next critical-path planning slice is **ECR-031** because ECR-003 additionally depends on it and real sensitive persistence remains blocked on its trust/storage contract. ECR-004 is independently planning-eligible and remains a parallel candidate; neither is implementation-authorized until its own Spec Kit package completes specify → plan → tasks → analyze and constitutional gates.

## Closure-head rule

This closure document update itself moves canonical `main`. ECR-002 may be treated as fully `CLOSED_CANONICAL` only after the final closure-convergence `main` head passes the complete permanent ECR-002 workflow. The resulting exact closure head/run must remain recoverable from repository/GitHub truth before ECR-031 implementation is authorized.
