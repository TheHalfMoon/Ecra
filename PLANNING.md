# Ecra Planning Index

Ecra uses Spec-Driven Development with a platform-scale Spec Kit “spec of specs” decomposition.

## Canonical Governance

- `.specify/memory/constitution.md` — binding Spec Kit constitution and Definition of Done.
- `AGENTS.md` — repository execution rules for coding agents.

## Platform Planning

- `specs/000-ecra-platform/roadmap.md` — canonical implementation dependency graph with immutable `ECR-###` slice IDs.
- `specs/000-ecra-platform/architecture.md` — platform layers, trust zones, dependency direction and provider boundaries.
- `specs/000-ecra-platform/threat-model.md` — initial assets/adversaries/trust boundaries/attack classes.
- `specs/000-ecra-platform/gap-audit.md` — planning coverage across product/security/browser/search/memory/skills/operations/legal/evaluation.
- `specs/000-ecra-platform/risk-register.md` — strategic/technical/security risks and owning slices.
- `specs/000-ecra-platform/benchmark-matrix.md` — cross-phase metrics, external benchmark families and internal mandatory suites.
- `specs/000-ecra-platform/decision-log.md` — accepted architecture decisions and revisit triggers.
- `research/donor-license-ledger.md` — donor/reference/dependency/source-reuse boundaries and license review policy.

## Current Implementation-Eligible Slice

### ECR-001 — Trusted Domain Kernel

Status: **TASKS_READY** (planning complete; implementation has not started).

Package:

- `specs/001-trusted-domain-kernel/spec.md`
- `specs/001-trusted-domain-kernel/research.md`
- `specs/001-trusted-domain-kernel/data-model.md`
- `specs/001-trusted-domain-kernel/contracts/domain-v1.md`
- `specs/001-trusted-domain-kernel/plan.md`
- `specs/001-trusted-domain-kernel/tasks.md`
- `specs/001-trusted-domain-kernel/quickstart.md`
- `specs/001-trusted-domain-kernel/checklists/requirements.md`

The slice defines the provider-neutral zero-I/O Rust domain contract for:

```text
Actor
Origin
Resource / Scope
CapabilityRequest / CapabilityGrant
Observation / Fact / Provenance
Evidence / ArtifactRef
ActionIntent
Side-effect / idempotency / retry semantics
ActionReceipt
VerificationReceipt
Versioning / canonicalization / structured errors
```

No browser, database, policy engine, model, MCP, or runtime execution implementation belongs to ECR-001.

## Execution Rule

Do not implement a later `ECR-###` slice because it is exciting or independently codeable. Follow dependency ordering in the canonical roadmap.

Normal progression for each slice:

```text
SPEC_READY
  ↓
PLAN_READY
  ↓
TASKS_READY
  ↓
IMPLEMENTING
  ↓
verification + analyze/convergence
  ↓
CLOSED_CANONICAL
```

Only `CLOSED_CANONICAL` dependencies unlock dependent implementation unless the dependent spec explicitly authorizes bounded fixture-only research.

## Strategic Documents

These explain product ambition but do not replace implementation specs:

- `README.md`
- `VISION.md`
- `CONSTITUTION.md`
- `ROADMAP.md`

When strategic wording and an active Spec Kit package differ on implementation semantics, `.specify/memory/constitution.md` and the canonical active slice govern until the discrepancy is deliberately reconciled.
