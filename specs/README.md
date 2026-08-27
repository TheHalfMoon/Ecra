# Ecra Spec Kit Index

This directory contains Ecra's canonical Spec-Driven Development packages.

## Start here

For current execution state, read:

1. `../EXECUTION.md`
2. `000-ecra-platform/roadmap.md`
3. the active slice `STATUS.md`
4. the active slice `tasks.md`

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

## Active slice

### ECR-001 — Trusted Domain Kernel

Directory: `001-trusted-domain-kernel/`  
Lifecycle: `IMPLEMENTING` on its bounded feature branch/PR.  
Operational status: `001-trusted-domain-kernel/STATUS.md`.

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
contracts/domain-v1.md
  ↓
implementation-clarifications.md (when present)
  ↓
plan.md
  ↓
tasks.md
  ↓
quickstart.md
  ↓
analyze.md
  ↓
checklists/
```

The status file is execution navigation, not a replacement for normative requirements. `spec.md`, contracts, and approved clarifications govern semantics.

## Future slices

The full program currently reserves ECR-001 through ECR-031 in the platform roadmap. A roadmap row does **not** mean a corresponding `specs/<slice>/` directory must already exist.

Create a slice directory only when planning work for that slice begins and its dependency/preparation rules allow it. Do not pre-create empty feature packages merely to mirror the roadmap.

The roadmap currently covers these capability families:

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

Every bounded feature package progresses through:

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

A successful individual task, commit, CI run, or PR review does not by itself close a slice.

## Required package contents

A normal implementation slice should contain, as applicable:

- `STATUS.md` — current execution ledger and next eligible work.
- `spec.md` — requirements and success criteria.
- `research.md` — decisions backed by primary sources/donors/standards.
- `data-model.md` — domain/wire model where needed.
- `contracts/` — normative interface/wire contracts.
- `implementation-clarifications.md` — only when implementation discovers a real underspecification; must converge back into canonical package before closure.
- `plan.md` — implementation architecture and boundaries.
- `tasks.md` — exact ordered executable tasks.
- `quickstart.md` — verification workflow.
- `analyze.md` — consistency/traceability analysis.
- `checklists/` — requirements/quality gates.

## Status vocabulary

- `PLANNED` — roadmap decomposition only.
- `SPEC_READY` — requirements complete enough for planning.
- `PLAN_READY` — implementation plan/contracts/research complete.
- `TASKS_READY` — executable tasks and analysis ready.
- `IMPLEMENTING` — bounded branch/PR active.
- `VERIFIED_ON_BRANCH` — a phase/task group has exact-head evidence on the active implementation branch.
- `BLOCKED` — dependency or evidence gate prevents continuation.
- `CLOSED_CANONICAL` — implementation, tests, docs, convergence, merge, and required post-merge evidence complete.
- `DEFERRED` — intentionally held outside current critical path.

`VERIFIED_ON_BRANCH` is deliberately narrower than `CLOSED_CANONICAL`.
