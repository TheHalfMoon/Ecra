# AGENTS.md — Ecra Repository Execution Rules

This repository uses Spec-Driven Development based on GitHub Spec Kit concepts and a platform-scale “spec of specs” roadmap.

## Canonical Sources of Truth

Before any material implementation or architecture change, read in this order:

1. `.specify/memory/constitution.md` — binding governance and cross-spec gates.
2. `specs/000-ecra-platform/roadmap.md` — immutable platform slice IDs, dependencies, statuses.
3. `specs/000-ecra-platform/gap-audit.md` — planning coverage and explicit deferrals/open research.
4. `specs/000-ecra-platform/risk-register.md` — platform-critical risks.
5. the active slice under `specs/<NNN-feature>/`:
   - `spec.md`
   - `research.md`
   - `data-model.md` when applicable
   - `contracts/`
   - `plan.md`
   - `quickstart.md`
   - `tasks.md`
   - `analyze.md` when present
   - checklists
6. implementation truth on the exact current branch/head.

Root `README.md`, `VISION.md`, `CONSTITUTION.md`, and `ROADMAP.md` are product-readable strategic documents. The Spec Kit constitution, canonical platform roadmap, and active feature package govern implementation details when wording differs.

## Spec Kit Lifecycle

For each bounded slice:

```text
constitution
  ↓
specify
  ↓
clarify (only when truly blocking)
  ↓
research / plan / data model / contracts
  ↓
requirements checklist
  ↓
tasks
  ↓
analyze consistency
  ↓
implement
  ↓
verify exact head
  ↓
converge remaining gaps
  ↓
CLOSED_CANONICAL
```

For this platform, the roadmap is a spec-of-specs. Do not create one giant implementation branch spanning multiple roadmap slices.

## Eligibility

A slice is implementation-eligible only when:

- its roadmap dependencies are `CLOSED_CANONICAL`, or
- its own spec explicitly authorizes bounded fixture-only/research work that cannot counterfeit a missing dependency;
- it has `SPEC_READY`, then `PLAN_READY`, then `TASKS_READY` artifacts as required;
- the latest required analyze pass has no critical planning defect;
- mandatory constitution gates pass.

Do not skip to a visually exciting later phase (Firefox fork, Search, Terminal, plugins, local models) while an earlier trust/dependency slice is incomplete.

## Current First Slice

At the time of constitution v1.1.0 planning convergence, the first implementation slice is:

```text
ECR-001 — Trusted Domain Kernel
specs/001-trusted-domain-kernel/
```

Re-read `specs/000-ecra-platform/roadmap.md` before acting because status may change.

## Task Execution

- Follow task IDs and dependency order from the active `tasks.md`.
- Do not mark a task complete because code merely compiles; satisfy the task's linked FR/SC/contract/test intent.
- `[P]` permits parallel work only when files/dependencies truly do not conflict.
- Exact file paths in tasks are authoritative unless implementation proves the planned structure impossible; amend plan/tasks before creating a different architecture.
- Do not silently broaden scope to adjacent roadmap slices.
- Do not add speculative abstractions/crates/providers “for later”.
- If implementation discovers a MUST-level planning defect, stop the affected task, record the defect and amend/converge the active Spec Kit package rather than silently weakening the requirement.

## Technical Content Language

Repository content, code, comments, commit messages, PR bodies, reports, task text, and reviewer responses should be written in English unless a specific artifact intentionally targets another language.

## Safety / Trust Rules

Never implement a shortcut that violates the constitution. In particular:

- no ambient agent authority;
- Actor attribution is not authenticated Principal identity;
- no treating external/web/model/tool/memory content as permission, identity, or approval;
- no absent/empty security scope interpreted as unrestricted; `ANY` must be explicit;
- no treating read authority as disclosure authority; remote model/search/tool/protocol use is an information-flow boundary;
- no using human-readable resource locators as canonical security identity where a stable ResourceId/provider identity exists;
- no CapabilityRequest-as-CapabilityGrant shortcut;
- no approval/receipt binding that ignores exact ActionDigest/ActionRef semantics;
- ActionIntent is distinct from ActionAttempt;
- no ActionReceipt-as-Verification shortcut;
- executor-observed success is not `VERIFIED`;
- no independent mutable `Fact.verified` truth flag outside VerificationReceipt-derived assessment;
- no coercing UNKNOWN external outcome into success/failure;
- no blind retry of non-idempotent consequential actions;
- no generic ContentDigest treated as authenticity/security proof without its owning security contract;
- no raw secret propagation into generic model/log/memory paths when mediated use is possible;
- no hidden telemetry;
- no external protocol as the internal trusted domain model;
- no unbounded recursive/model/tool/process loops once runtime slices exist; resource budgets are mandatory where applicable;
- no copied donor source without provenance/license record;
- no browser/security/privacy/performance superlatives without reproducible benchmarks.

## Repository / Git Discipline

- Live repository truth overrides stale handoffs or remembered SHAs.
- Inspect the current branch/head and relevant specs before mutation.
- Use bounded feature branches/PRs once implementation begins.
- No force-push, rebase, or destructive history rewriting unless repository governance is explicitly amended to permit it.
- Do not claim PASS, MERGED, or CLOSED_CANONICAL without exact-head/post-merge evidence as applicable.
- A docs-only planning state is not implementation completion.

## Testing and Closure

Before a slice closes:

1. run the active slice `quickstart.md`/verification guide;
2. run every required unit/contract/integration/security/migration/benchmark gate;
3. record exact head SHA and results;
4. re-check constitution gates;
5. check spec → research/data-model/contracts → plan → tasks → implementation traceability;
6. append convergence tasks for remaining gaps instead of hiding them;
7. update donor/license records;
8. update the master roadmap status truthfully.

`CLOSED_CANONICAL` means the implemented repository state satisfies the active Spec Kit package. It is not a narrative status label.

## Planning Changes

When a new requirement or idea appears:

1. determine whether it belongs to an existing slice;
2. if not, update the spec-of-specs with a new immutable `ECR-###` entry;
3. name dependencies and owning acceptance evidence;
4. update gap audit/risk register if it creates a new persistent data class, privileged capability, identity/trust-root concern, information-flow/remote-egress path, external protocol, public claim, browser patch, or security boundary;
5. create a bounded Spec Kit package before implementation.

## Donor / Research Use

`research/donor-license-ledger.md` is the starting point, not final legal approval. Before source reuse or adding a dependency, verify the exact upstream version, license, notices, security posture, and transitive implications relevant to that change.

Conceptual inspiration must not be mislabeled as copied code, and copied code must not be mislabeled as inspiration.
