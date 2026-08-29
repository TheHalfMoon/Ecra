# Ecra Spec Kit Index

This directory contains Ecra's canonical Spec-Driven Development packages.

## Start here

For current execution state, read:

1. `../EXECUTION.md`
2. `000-ecra-platform/roadmap.md`
3. `000-ecra-platform/STATUS.md`
4. the active slice `STATUS.md`, `tasks.md`, and analyze/convergence artifacts
5. exact live GitHub branch/head, PR, CI, review and changed-file truth

Do not infer implementation eligibility from directory names alone. The roadmap dependency graph, active package, and exact live evidence decide what may be planned or implemented.

## Platform package

`000-ecra-platform/` is the spec-of-specs and owns immutable ECR IDs/dependencies plus cross-platform architecture, threat, risk, gap, benchmark and decision records.

## Canonically closed slices

### ECR-001 — Trusted Domain Kernel

Directory: `001-trusted-domain-kernel/`  
Lifecycle: `CLOSED_CANONICAL`.  
PR #1 merged; closure-ledger head `85e4bf657b6c33e3f88d83e92e7a35279d177349` passed CI `33099434232`.

### ECR-002 — Durable Run, Ledger & Budgets

Directory: `002-durable-run-ledger/`  
Lifecycle: `CLOSED_CANONICAL`.  
Final closure-convergence main head `aadc19c972e619222d426674d7542dd9c00dbe44` passed ECR-002 CI `33155302100` and ECR-001 regression CI `33155302026`.

ECR-002 owns synthetic/non-sensitive local run durability, budgets, recovery and deterministic `.ecra` interchange. It does not authorize real sensitive persistence, authentication/trust roots, authorization, independent verification, or provider execution.

### ECR-004 — Verification & Reconciliation

Directory: `004-verification-receipts/`  
Lifecycle: `CLOSED_CANONICAL`.  
Merged implementation PR: #7.  
Merged feature head: `990addb79e6fe5a1ad2b16dae159c624959e2128`.  
Canonical implementation merge: `2a95fbb4f20b1646505cb179f4822a758a546895`.  
Closure-convergence head: `c159c96061a73ead9710985d07608e2b417fe275`.

Required workflows passed on the exact implementation merge state:

```text
ECR-001  RUN 33255780673  JOB 99109106995  SUCCESS
ECR-002  RUN 33255780671  JOB 99109107144  SUCCESS
ECR-004  RUN 33255780663  JOB 99109107058  SUCCESS
```

They passed again on exact closure-convergence head `c159c96061a73ead9710985d07608e2b417fe275` before the T053 lifecycle marker:

```text
ECR-001  RUN 33256430974  JOB 99110882402  SUCCESS
ECR-002  RUN 33256430942  JOB 99110916386  SUCCESS
ECR-004  RUN 33256430965  JOB 99110882233  SUCCESS
```

The T053 closure marker must itself pass ECR-001 + ECR-002 + ECR-004 on the exact canonical `main` head before an external `CLOSED_CANONICAL` claim is made.

Frozen v1 boundaries remain:

- ECR-001 `VerificationReceipt` is the only canonical independent verification record;
- `ActionReceipt` remains executor-observed evidence and cannot self-verify;
- Fact/artifact/run metadata gain no competing verified truth flag;
- deterministic aggregates expose conflict rather than last-write-wins;
- UNKNOWN reconciliation never fabricates an `ActionReceipt` or mutates ECR-002 run-event truth;
- reconciliation never clears ECR-002 unresolved state, changes `RunPhase`, or makes the same run resumable/completable;
- `semantically_retryable*` is advisory for a future new-attempt proposal only;
- ECR-004 journal persistence is separate from ECR-002 run storage and v1 stores only synthetic/non-sensitive evidence metadata/references/digests;
- journal chaining is local integrity/corruption/substitution detection, not hostile complete-store tamper resistance;
- no browser/network/model/provider/process/policy/authorization/identity/telemetry runtime dependency enters `ecra-verify`.

Package:

```text
STATUS.md
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
post-implementation-analyze.md
traceability-closure.md
checklists/requirements.md
```

## Active trusted-substrate slice

### ECR-031 — Identity, Trust Root & Sensitive Storage Foundations

Directory: `031-identity-trust-root/`  
Implementation branch: `031-identity-trust-root`.  
Draft implementation PR: #4.  
Lifecycle: `IMPLEMENTING / BLOCKED_EXTERNAL_NATIVE_ACCEPTANCE`.  
Current blocking task: T064.

The live branch/package is authoritative for exact progress. The native macOS Data Protection Keychain acceptance gate remains blocked because the trusted runner user lacks a valid Apple Development signing identity, suitable provisioning profile, configured Xcode developer account registry and usable development team. No legacy/plaintext/ad-hoc fallback or weakened native acceptance is authorized.

## Dependency boundary

- ECR-003 remains implementation-blocked until ECR-031 is `CLOSED_CANONICAL`.
- ECR-005 remains blocked until every listed dependency is `CLOSED_CANONICAL`; closing ECR-004 does not satisfy the still-open ECR-003/ECR-031 requirements.
- No later slice becomes implementation-eligible merely because ECR-031 is externally blocked.

## Future slices

The full program reserves ECR-001 through ECR-031 in the platform roadmap. A roadmap row does not imply implementation authorization or package completion.

## Package lifecycle

```text
SPEC_READY
  ↓
PLAN_READY
  ↓
TASKS_READY
  ↓
IMPLEMENTING
  ↓
exact-head verification
  ↓
convergence / analyze
  ↓
merge + required post-merge verification
  ↓
CLOSED_CANONICAL
```

A successful individual task, commit, CI run, review or merge does not by itself close a slice.

## Required package contents

A normal stateful implementation slice should contain, as applicable:

- `STATUS.md` — execution ledger and next eligible work;
- `spec.md` — requirements and success criteria;
- `research.md` — primary-source/donor/dependency decisions;
- `data-model.md` — domain/wire/state model;
- `contracts/` — normative interfaces/wire contracts;
- threat model;
- `implementation-clarifications.md` when implementation discovers a real underspecification;
- `plan.md` — architecture and constitutional gates;
- `tasks.md` — ordered executable tasks;
- `quickstart.md` — verification workflow;
- analyze/traceability/convergence artifacts;
- checklists.

## Status vocabulary

- `PLANNED` — roadmap decomposition only.
- `PLANNING_REWORK` — blocking planning defect; implementation forbidden.
- `SPEC_READY` — requirements complete enough for planning.
- `PLAN_READY` — research/data model/contracts/plan complete.
- `TASKS_READY` — executable tasks and latest planning analyze clean.
- `IMPLEMENTING` — bounded branch/PR active, including final convergence/review work.
- `VERIFIED_ON_BRANCH` — phase/task group has exact-head evidence on the active branch.
- `BLOCKED` — dependency/evidence gate prevents continuation.
- `CLOSED_CANONICAL` — implementation, tests, docs, convergence, merge and required post-merge evidence complete.
- `DEFERRED` — intentionally outside current critical path.

`VERIFIED_ON_BRANCH` is deliberately narrower than `CLOSED_CANONICAL`.
