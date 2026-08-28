# Ecra Spec Kit Index

This directory contains Ecra's canonical Spec-Driven Development packages.

## Start here

For current execution state, read:

1. `../EXECUTION.md`
2. `000-ecra-platform/roadmap.md`
3. `000-ecra-platform/STATUS.md`
4. the selected/active slice `STATUS.md` and `tasks.md` once that package exists
5. exact live GitHub branch/head, PR, CI, review and changed-file truth

Do not infer implementation eligibility from directory names alone. The roadmap dependency graph and live repository evidence decide what may be planned or implemented.

## Platform package

`000-ecra-platform/` is the spec-of-specs and contains cross-platform planning:

- `roadmap.md` — immutable `ECR-###` IDs and dependencies.
- `architecture.md` — platform layers, dependency direction, providers, trust zones.
- `threat-model.md` — assets, adversaries, trust boundaries, attack classes.
- `gap-audit.md` — planning coverage and explicit gaps/deferrals.
- `risk-register.md` — persistent program risks and owning slices.
- `benchmark-matrix.md` — acceptance/evaluation program.
- `decision-log.md` — accepted architecture decisions and revisit triggers.
- `pre-implementation-review-2026-08-27.md` — pre-code review and remediation record.

## Canonically closed slices

### ECR-001 — Trusted Domain Kernel

Directory: `001-trusted-domain-kernel/`  
Lifecycle: `CLOSED_CANONICAL`.  
PR #1 merged; canonical closure-ledger head `85e4bf657b6c33e3f88d83e92e7a35279d177349` passed CI `33099434232`.

The package remains the canonical trusted-domain record and dependency for later slices.

### ECR-002 — Durable Run, Ledger & Budgets

Directory: `002-durable-run-ledger/`  
Lifecycle: canonical closure convergence after PR #2 merge.  
Final feature head: `87fd9fc560bf5ca21a07a4d25473f305b4c05f05`; feature CI `33153413462` passed.  
Merge commit: `40efc8a64a9562f0f3eb2555b350cfa03d3e0675`; post-merge ECR-002 CI `33154108410` and ECR-001 regression CI `33154108397` passed.  
Operational/closure ledger: `002-durable-run-ledger/STATUS.md`.

ECR-002 owns local synthetic/non-sensitive run durability, budgets, recovery semantics and deterministic `.ecra` interchange. It does not authorize real sensitive persistence, authentication/trust roots, general authorization/declassification policy, independent verification/reconciliation, or provider/browser/model/tool execution.

## Next dependency-eligible planning

After the final ECR-002 closure-convergence head passes the permanent ECR-002 workflow, two slices are dependency-eligible for bounded planning:

```text
ECR-031 — Identity, Trust Root & Sensitive Storage Foundations
ECR-004 — Verification & Reconciliation
```

ECR-031 is selected as the next critical-path planning slice because ECR-003 additionally depends on it and sensitive-state progression remains blocked on its trust-root/protected-storage semantics. ECR-004 remains independently planning-eligible and must not be folded into ECR-031.

No ECR-031 package should be treated as implementation-authorized until it independently completes specify → research/plan/data-model/contracts → checklist → tasks → analyze with all required constitutional gates.

## Future slices

The full program reserves ECR-001 through ECR-031 in the platform roadmap. A roadmap row does not mean a corresponding directory must already exist.

Create a slice directory only when planning work for that slice begins and dependency/preparation rules allow it. Do not pre-create empty packages merely to mirror the roadmap.

Capability families:

```text
Trusted domain / durability / identity / policy / verification
Evaluation and threat harness
Browser prototype / browser foundation / browser product
Search evidence / workspace / memory / semantic capability routing
Skill IR / compile / replay / repair
MCP / ACP / A2A / Agent Skills gateway
Plugin and sandbox runtime
Terminal / developer workspace / data analytics
Local model gateway
Sync / registry / supply chain / privacy / accessibility
Source compliance / public benchmark program / portability / ecosystem gateway
```

## Package lifecycle

Every bounded implementation slice progresses through:

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

- `STATUS.md` — current execution ledger and next eligible work.
- `spec.md` — requirements and success criteria.
- `research.md` — decisions backed by primary sources/donors/standards.
- `data-model.md` — domain/wire/state model.
- `contracts/` — normative interfaces/wire contracts.
- slice `threat-model.md` when a security/state boundary is introduced.
- `implementation-clarifications.md` only when implementation discovers a real underspecification; it must converge back before closure.
- `plan.md` — implementation architecture and constitutional gates.
- `tasks.md` — exact ordered executable tasks with target paths.
- `quickstart.md` — verification workflow.
- `analyze.md` — consistency/traceability analysis.
- `checklists/` — requirements/quality gates.

## Status vocabulary

- `PLANNED` — roadmap decomposition only.
- `SPEC_READY` — requirements complete enough for planning.
- `PLAN_READY` — research/data model/contracts/plan complete.
- `TASKS_READY` — executable tasks and analysis ready.
- `IMPLEMENTING` — bounded branch/PR active.
- `VERIFIED_ON_BRANCH` — a phase/task group has exact-head evidence on the active implementation branch.
- `BLOCKED` — dependency or evidence gate prevents continuation.
- `CLOSED_CANONICAL` — implementation, tests, docs, convergence, merge and required post-merge evidence complete.
- `DEFERRED` — intentionally held outside current critical path.

`VERIFIED_ON_BRANCH` is deliberately narrower than `CLOSED_CANONICAL`.
