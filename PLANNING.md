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
- `specs/000-ecra-platform/pre-implementation-review-2026-08-27.md` — blocking pre-code architecture review and remediation ownership.
- `research/donor-license-ledger.md` — donor/reference/dependency/source-reuse boundaries and license review policy.

## Current Implementation-Eligible Slice

### ECR-001 — Trusted Domain Kernel

Status: **TASKS_READY** — post-remediation analyze passed; implementation has not yet landed on `main`.

Package:

- `specs/001-trusted-domain-kernel/spec.md`
- `specs/001-trusted-domain-kernel/research.md`
- `specs/001-trusted-domain-kernel/data-model.md`
- `specs/001-trusted-domain-kernel/contracts/domain-v1.md`
- `specs/001-trusted-domain-kernel/plan.md`
- `specs/001-trusted-domain-kernel/tasks.md`
- `specs/001-trusted-domain-kernel/quickstart.md`
- `specs/001-trusted-domain-kernel/checklists/requirements.md`
- `specs/001-trusted-domain-kernel/analyze.md`

The slice defines the provider-neutral zero-I/O Rust domain contract for:

```text
Actor / PrincipalRef / IdentityAssertionRef
Origin
ResourceId / ResourceRef
Scope / explicit ScopeConstraint
CapabilityRequest / CapabilityGrant
InformationClassification / InformationUse
Observation / Fact / Provenance / Freshness
Evidence / ArtifactRef
ActionIntent / ActionDigest / ActionRef
MutationDomain / Reversibility / Idempotency / Retry
ActionAttemptId
ActionReceipt
VerificationReceipt
Versioning / canonicalization / structured errors
```

No authentication validator, browser, database, policy engine, model, MCP/ACP/A2A adapter, secret store, or runtime execution implementation belongs to ECR-001.

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

## Review-Sensitive Rules

The pre-implementation review strengthened several invariants that implementation agents must not regress:

- Actor attribution is not authenticated Principal identity.
- missing/empty scope never means unrestricted; `ANY` is explicit.
- read authority does not imply disclosure authority.
- Resource locator strings are not canonical security identity.
- exact action security semantics bind through ActionDigest/ActionRef.
- ActionIntent and ActionAttempt are different identities.
- executor-observed success is not VERIFIED.
- Fact has no independent mutable VERIFIED truth flag.
- generic ContentDigest is not automatically a security/authenticity digest.

## Strategic Documents

These explain product ambition but do not replace implementation specs:

- `README.md`
- `VISION.md`
- `CONSTITUTION.md`
- `ROADMAP.md`

When strategic wording and an active Spec Kit package differ on implementation semantics, `.specify/memory/constitution.md`, the canonical platform roadmap, and the active slice govern until the discrepancy is deliberately reconciled.
