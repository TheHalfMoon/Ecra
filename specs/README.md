# Ecra Spec Kit Index

This directory contains Ecra's canonical Spec-Driven Development packages.

## Start here

For current execution state, read:

1. `../EXECUTION.md`
2. `000-ecra-platform/roadmap.md`
3. `000-ecra-platform/STATUS.md`
4. the active slice `STATUS.md`
5. the active slice `tasks.md`

Do not infer implementation eligibility from directory names alone. The roadmap dependency graph and live repository evidence decide what may be implemented.

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

## Closed slice

### ECR-001 — Trusted Domain Kernel

Directory: `001-trusted-domain-kernel/`  
Lifecycle: `CLOSED_CANONICAL`.  
PR #1 merged; canonical closure-ledger head `85e4bf657b6c33e3f88d83e92e7a35279d177349` passed CI `33099434232`.

The package remains the canonical trusted-domain record and dependency for later slices.

## Active slice

### ECR-002 — Durable Run, Ledger & Budgets

Directory: `002-durable-run-ledger/`  
Lifecycle: `TASKS_READY` on canonical planning state; implementation branch/PR is the next action.  
Operational status: `002-durable-run-ledger/STATUS.md`.

Read the package in this order:

```text
STATUS.md
  ↓
spec.md
  ↓
research.md
  ↓
data-model.md
  ↓
contracts/run-ledger-v1.md
  ↓
threat-model.md
  ↓
plan.md
  ↓
tasks.md
  ↓
quickstart.md
  ↓
analyze.md
  ↓
checklists/requirements.md
```

Planning result:

```text
FR-001–FR-057 OWNED
SC-001–SC-016 OWNED
G1–G15 PASS / explicit N/A
ZERO_BLOCKING_PLANNING_DRIFT_FOUND
```

Implementation is restricted to local synthetic/non-sensitive durability. Real sensitive persistence remains gated by ECR-031/ECR-003/ECR-025, and independent reconciliation/verification remains ECR-004.

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
