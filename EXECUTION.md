# Ecra Execution Guide

> **Operational start-here document.** Recover live work from this file, platform roadmap/status, the selected planning slice package, and exact GitHub truth; do not depend on private chat state.

## Source-of-truth order

1. `.specify/memory/constitution.md`
2. `EXECUTION.md`
3. `specs/000-ecra-platform/roadmap.md`
4. `specs/000-ecra-platform/STATUS.md`
5. relevant platform architecture/threat/gap/risk/benchmark/decision artifacts
6. `specs/README.md`
7. selected slice package, once created
8. exact live branch/head, PR, CI, reviews and changed files

Stale prose must be updated to live evidence, never the reverse.

## Current execution truth

```text
ECR-001 — Trusted Domain Kernel: CLOSED_CANONICAL
ECR-001 closure-ledger head: 85e4bf657b6c33e3f88d83e92e7a35279d177349
ECR-001 closure-ledger CI: 33099434232 — SUCCESS

ECR-002 — Durable Run, Ledger & Budgets: CANONICAL_CLOSURE_CONVERGENCE
Final feature head: 87fd9fc560bf5ca21a07a4d25473f305b4c05f05
Final feature CI: 33153413462 / job 98790541842 — SUCCESS
PR: #2 — MERGED
Merge commit: 40efc8a64a9562f0f3eb2555b350cfa03d3e0675
Post-merge ECR-002 CI: 33154108410 / job 98792690359 — SUCCESS
Post-merge ECR-001 CI: 33154108397 / job 98792690901 — SUCCESS
Review: CodeRabbit SUCCESS; no review threads or inline findings

T001–T070: COMPLETE
T071: COMPLETE — exact-head CI/review/readiness satisfied
T072: COMPLETE — exact verified head merged non-rebase + main CI passed
T073: ACTIVE — canonical closure/index convergence + final closure-head CI
```

`CLOSED_CANONICAL` is not considered finally sealed until the last T073 documentation-convergence head itself passes the complete permanent ECR-002 workflow on `main`.

## ECR-002 canonical package

```text
specs/002-durable-run-ledger/STATUS.md
specs/002-durable-run-ledger/spec.md
specs/002-durable-run-ledger/research.md
specs/002-durable-run-ledger/data-model.md
specs/002-durable-run-ledger/contracts/run-ledger-v1.md
specs/002-durable-run-ledger/implementation-clarifications.md  # historical/non-normative
specs/002-durable-run-ledger/threat-model.md
specs/002-durable-run-ledger/plan.md
specs/002-durable-run-ledger/tasks.md
specs/002-durable-run-ledger/quickstart.md
specs/002-durable-run-ledger/analyze.md
specs/002-durable-run-ledger/traceability-closure.md
specs/002-durable-run-ledger/post-implementation-analyze.md
specs/002-durable-run-ledger/checklists/requirements.md
```

Final ECR-002 result:

```text
FR-001–FR-057 PASS
SC-001–SC-016 PASS with feature-head and post-merge evidence
G1–G15 PASS / explicit PASS-N/A
UNOWNED_FR=0
UNOWNED_SC=0
FAILED_CONSTITUTION_GATES=0
IMPLICITLY_ACCEPTED_CRITICAL_RISKS=0
MUST_LEVEL_IMPLEMENTATION_DEFECTS_FOUND=0
CONVERGENCE_DRIFT_FOUND=4
CONVERGENCE_DRIFT_REMEDIATED=4
```

## Closed trusted-substrate invariants

```text
Actor != authenticated Principal
CapabilityRequest != CapabilityGrant
classification != permission
InformationUse != authorization
ActionDigest != signature/approval
ActionIntent != ActionAttemptRef != ActionReceipt != VerificationReceipt
executor_observed_success != verified
UNKNOWN remains UNKNOWN
projection != authoritative event history
LedgerDigest != authentication/signature/MAC/VerificationReceipt
budget != authority
`.ecra` != protected secret container
```

ECR-002 additionally freezes:

```text
authoritative run truth     append-only ordered RunEventEnvelope history
attempt before effect       durable AttemptPrepared commit required
missing receipt             UNKNOWN / reconciliation-required
local store                 SQLite via rusqlite 0.40.2
SQLite engine               bundled SQLite 3.53.2 via libsqlite3-sys 0.38.2
SQLite durability           WAL + synchronous=FULL, asserted at open
write transaction           Immediate + expected-head compare
budget accounting           typed checked I-JSON-safe integers
portable artifact           deterministic strict Stored-only ZIP via zip 8.6.0
real sensitive persistence  NOT AUTHORIZED by ECR-002
provider/network execution  NOT IN ECR-002
hostile rewrite claim       not provided by plain hash chain
```

## Next genuinely dependency-eligible planning

Once the T073 closure-head CI is green, ECR-002 is a closed dependency and two slices become dependency-eligible for bounded planning:

```text
ECR-031 — Identity, Trust Root & Sensitive Storage Foundations
  depends on ECR-001 + ECR-002

ECR-004 — Verification & Reconciliation
  depends on ECR-001 + ECR-002
```

### Selected next slice: ECR-031

ECR-031 is selected as the next critical-path planning package because:

1. ECR-003 additionally depends on ECR-031;
2. real sensitive persistence remains blocked until protected storage/trust-root semantics exist;
3. authenticated Principal/IdentityAssertion/on-behalf-of semantics must precede privileged policy/execution;
4. ECR-004 remains independently planning-eligible in parallel and must not be counterfeited inside ECR-031.

There is currently no canonical `specs/031-identity-trust-root/` package. Create it only after ECR-002's final closure-convergence head is green. Planning must proceed through specify → research/plan/data-model/contracts → checklist → tasks → analyze before any ECR-031 implementation branch/PR exists.

## ECR-031 scope boundary to preserve

Roadmap-owned outcome:

```text
identity/principal assertions and on-behalf-of binding
device/user-local trust root
key lifecycle and revocation
protected sensitive-storage/authenticity envelope semantics
```

Do not smuggle into ECR-031:
- general authorization/declassification/approval policy (ECR-003);
- independent verification/reconciliation decisions (ECR-004);
- browser/provider/model/tool execution;
- local-model gateway work (ECR-021);
- broad sync/telemetry/portability product behavior.

## CI architecture

The repository-scoped self-hosted macOS runner `macbook` remains the trusted execution oracle. Persistent personal runners must not execute untrusted fork PR code.

Closed ECR-001 and ECR-002 workflows remain push gates on `main` so new trusted-substrate changes cannot silently regress either contract.

## Execution rule

Finish T073 exact-head closure verification first. Then re-read canonical main and start ECR-031 planning from that exact state. Do not implement ECR-003, ECR-004, browser, search, local models, or other later surfaces out of dependency order. No force-push, rebase or destructive history rewriting. Never mark PASS, MERGED or `CLOSED_CANONICAL` without exact-head/post-merge evidence.
