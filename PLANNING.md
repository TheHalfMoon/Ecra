# Ecra Planning Index

Ecra uses Spec-Driven Development with a platform-scale Spec Kit “spec of specs” decomposition.

## Start Here

For implementation work, use this order:

1. `.specify/memory/constitution.md` — binding governance and Definition of Done.
2. `EXECUTION.md` — current active slice, exact progress, next eligible work, and execution gates.
3. `specs/README.md` — Spec Kit navigation.
4. `specs/000-ecra-platform/roadmap.md` — canonical ECR dependency graph.
5. the active slice package and its `STATUS.md` / `tasks.md`.
6. exact live Git/GitHub branch, PR, CI, review, and head truth.

`EXECUTION.md` exists so a new human or coding agent can continue from repository truth without a private chat handoff.

## Canonical Governance

- `.specify/memory/constitution.md` — binding Spec Kit constitution and Definition of Done.
- `AGENTS.md` — repository execution rules for coding agents.
- `EXECUTION.md` — operational execution entry point.
- `specs/README.md` — feature-package navigation and lifecycle rules.

## Platform Planning

- `specs/000-ecra-platform/roadmap.md` — canonical implementation dependency graph with immutable `ECR-###` slice IDs.
- `specs/000-ecra-platform/architecture.md` — platform layers, trust zones, dependency direction and provider boundaries.
- `specs/000-ecra-platform/threat-model.md` — assets, adversaries, trust boundaries and attack classes.
- `specs/000-ecra-platform/gap-audit.md` — planning coverage across product/security/browser/search/memory/skills/operations/legal/evaluation.
- `specs/000-ecra-platform/risk-register.md` — strategic/technical/security risks and owning slices.
- `specs/000-ecra-platform/benchmark-matrix.md` — cross-phase metrics, external benchmark families and internal mandatory suites.
- `specs/000-ecra-platform/decision-log.md` — accepted architecture decisions and revisit triggers.
- `specs/000-ecra-platform/pre-implementation-review-2026-08-27.md` — blocking pre-code architecture review and remediation ownership.
- `research/donor-license-ledger.md` — donor/reference/dependency/source-reuse boundaries and license review policy.

## Current Implementation Slice

### ECR-001 — Trusted Domain Kernel

Lifecycle: **IMPLEMENTING** on `001-trusted-domain-kernel` / draft PR #1.  
Operational ledger: `specs/001-trusted-domain-kernel/STATUS.md`.

Latest fully verified implementation head before Phase 5:

```text
992dd31c44104aa619b0ea59429063f69e559014
```

At that exact head, build, fmt, strict Clippy, tests, rustdoc, offline replay, and dependency-boundary gates passed.

Progress:

```text
T001–T006   Phase 1   VERIFIED_ON_BRANCH
T007–T014   Phase 2   VERIFIED_ON_BRANCH
T015–T023   Phase 3   VERIFIED_ON_BRANCH
T024–T028   Phase 4   VERIFIED_ON_BRANCH
T029–T038   Phase 5   NEXT_ACTIVE_PHASE
T039+                BLOCKED_BY_ORDERING
```

`VERIFIED_ON_BRANCH` is not `CLOSED_CANONICAL`. No dependent ECR slice becomes implementation-eligible until the full ECR-001 package is closed and canonicalized.

Package:

- `specs/001-trusted-domain-kernel/STATUS.md`
- `specs/001-trusted-domain-kernel/spec.md`
- `specs/001-trusted-domain-kernel/research.md`
- `specs/001-trusted-domain-kernel/data-model.md`
- `specs/001-trusted-domain-kernel/contracts/domain-v1.md`
- `specs/001-trusted-domain-kernel/implementation-clarifications.md` when implementation finds a real underspecification
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

## Platform Execution Shape

The canonical dependency details live in `specs/000-ecra-platform/roadmap.md`. For orientation:

```text
Trusted substrate
ECR-001 → ECR-002 → {ECR-031, ECR-004} → ECR-003 → ECR-005

Browser wedge
ECR-006 → ECR-007 → ECR-008

Trusted knowledge/context
ECR-009 → ECR-010 → ECR-011

Learn once / replay cheaply
ECR-012 → ECR-013 → ECR-014 → ECR-015

Ecosystem and work surfaces
ECR-016 / ECR-017 → ECR-018 / ECR-019 / ECR-020 / ECR-021
```

ECR-022–ECR-030 own later sync, registry, supply chain, privacy/diagnostics, accessibility/i18n, source compliance, benchmark, portability, and ecosystem concerns according to the roadmap dependency graph.

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
exact-head verification
  ↓
convergence + analyze
  ↓
merge + required post-merge verification
  ↓
CLOSED_CANONICAL
```

Only `CLOSED_CANONICAL` dependencies unlock dependent implementation unless the dependent spec explicitly authorizes bounded fixture-only research.

## Review-Sensitive Rules

Implementation agents must not regress these invariants:

- Actor attribution is not authenticated Principal identity.
- missing/empty scope never means unrestricted; `ANY` is explicit.
- web/model/tool/memory content is not instruction authority.
- read authority does not imply disclosure authority.
- Resource locator strings are not canonical security identity.
- CapabilityRequest does not become CapabilityGrant.
- classification/provenance/freshness do not grant authority.
- exact action security semantics bind through ActionDigest/ActionRef.
- ActionIntent and ActionAttempt are different identities.
- executor-observed success is not VERIFIED.
- Fact has no independent mutable VERIFIED truth flag.
- UNKNOWN external outcome is not silently coerced.
- generic ContentDigest is not automatically a security/authenticity digest.
- remote providers are information-disclosure boundaries.

## Keeping the Repository Easy to Continue

Whenever execution materially advances:

1. update the active slice `STATUS.md`;
2. update `EXECUTION.md` if the active phase/slice or exact verified baseline changes;
3. update the platform roadmap when lifecycle state changes (`TASKS_READY` → `IMPLEMENTING` → `CLOSED_CANONICAL`);
4. keep task ordering and clarifications in the active Spec Kit package;
5. do not leave the only accurate state in a chat handoff, commit message, or PR comment.

## Strategic Documents

These explain product ambition but do not replace implementation specs:

- `README.md`
- `VISION.md`
- `CONSTITUTION.md`
- `ROADMAP.md`

When strategic wording and an active Spec Kit package differ on implementation semantics, `.specify/memory/constitution.md`, `EXECUTION.md` for current operational state, the canonical platform roadmap, and the active slice govern until the discrepancy is deliberately reconciled.
