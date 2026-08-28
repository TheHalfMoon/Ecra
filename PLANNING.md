# Ecra Planning Index

Ecra uses Spec-Driven Development with a platform-scale Spec Kit “spec of specs” decomposition.

## Start Here

For implementation work, use this order:

1. `.specify/memory/constitution.md` — binding governance and Definition of Done.
2. `EXECUTION.md` — current execution/closure truth, next eligible work and exact verification rules.
3. `specs/README.md` — Spec Kit navigation.
4. `specs/000-ecra-platform/roadmap.md` — canonical ECR dependency graph.
5. the selected/active slice package and its `STATUS.md` / `tasks.md`, once created.
6. exact live Git/GitHub branch, PR, CI, review and head truth.

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

## Closed Trusted Substrate

### ECR-001 — Trusted Domain Kernel

Lifecycle: **CLOSED_CANONICAL**.  
Operational ledger: `specs/001-trusted-domain-kernel/STATUS.md`.

ECR-001 owns the provider-neutral zero-I/O trusted domain contract for Actor/PrincipalRef/IdentityAssertionRef, origin/resource/scope, capability request/grant, information classification/use, observations/facts/provenance/freshness/evidence, action/action-attempt identities, receipts, verification receipts, versioning/canonicalization and structured errors.

### ECR-002 — Durable Run, Ledger & Budgets

Lifecycle: **canonical closure convergence** after exact-head merge/post-merge verification.  
Operational/closure ledger: `specs/002-durable-run-ledger/STATUS.md`.

Evidence:

```text
Final feature head:       87fd9fc560bf5ca21a07a4d25473f305b4c05f05
Feature ECR-002 CI:       33153413462 — SUCCESS
Merge commit:             40efc8a64a9562f0f3eb2555b350cfa03d3e0675
Post-merge ECR-002 CI:    33154108410 — SUCCESS
Post-merge ECR-001 CI:    33154108397 — SUCCESS
Review:                   CodeRabbit SUCCESS; no review threads/inline findings
```

ECR-002 owns append-only durable run truth, deterministic replay/projections, exact action-attempt preparation and UNKNOWN recovery semantics, typed budgets, local SQLite durability, and deterministic strict `.ecra` interchange for synthetic/non-sensitive state. It does not authorize real sensitive persistence, authentication/trust roots, general policy/declassification, independent verification/reconciliation, or provider/browser/model/tool execution.

The final T073 closure-convergence `main` head must itself pass the permanent ECR-002 workflow before downstream implementation authorization relies on ECR-002 as a closed dependency.

## Next Selected Planning Slice

### ECR-031 — Identity, Trust Root & Sensitive Storage Foundations

Roadmap dependencies: ECR-001 + ECR-002.  
Selected state: **NEXT_CRITICAL_PATH_PLANNING**, after final ECR-002 closure-head verification.

ECR-031 owns:

```text
identity/principal assertions and on-behalf-of binding
device/user-local trust root
key lifecycle and revocation
protected sensitive-storage/authenticity envelope semantics
```

It must not absorb:

```text
general authorization/declassification/approval policy  -> ECR-003
independent verification/reconciliation decisions       -> ECR-004
browser/provider/model/tool execution                    -> later owning slices
local model gateway                                      -> ECR-021
broad sync/telemetry/portability product behavior        -> ECR-022/ECR-025/ECR-029
```

ECR-004 is independently dependency-eligible for bounded planning after ECR-002 closure, but ECR-031 is selected first because ECR-003 additionally depends on it and real sensitive-state progression remains blocked on its trust-root/protected-storage semantics.

No ECR-031 implementation branch or PR may exist until its package independently completes specify → research/plan/data-model/contracts → checklist → tasks → analyze and all mandatory constitution gates.

## Platform Execution Shape

The canonical dependency details live in `specs/000-ecra-platform/roadmap.md`. For orientation:

```text
Trusted substrate
ECR-001 [CLOSED] → ECR-002 [CLOSED] → {ECR-031 [NEXT], ECR-004 [ELIGIBLE]} → ECR-003 → ECR-005

Browser wedge
ECR-006 → ECR-007 → ECR-008

Trusted knowledge/context
ECR-009 → ECR-010 → ECR-011

Learn once / replay cheaply
ECR-012 → ECR-013 → ECR-014 → ECR-015

Ecosystem and work surfaces
ECR-016 / ECR-017 → ECR-018 / ECR-019 / ECR-020 / ECR-021
```

ECR-022–ECR-030 own later sync, registry, supply chain, privacy/diagnostics, accessibility/i18n, source compliance, benchmark, portability and ecosystem concerns according to the roadmap dependency graph.

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

1. update the selected/active slice `STATUS.md`;
2. update `EXECUTION.md` if the active phase/slice or exact verified baseline changes;
3. update the platform roadmap when lifecycle state changes (`TASKS_READY` → `IMPLEMENTING` → `CLOSED_CANONICAL`);
4. keep task ordering and clarifications in the active Spec Kit package;
5. do not leave the only accurate state in a chat handoff, commit message or PR comment.

## Strategic Documents

These explain product ambition but do not replace implementation specs:

- `README.md`
- `VISION.md`
- `CONSTITUTION.md`
- `ROADMAP.md`

When strategic wording and an active Spec Kit package differ on implementation semantics, `.specify/memory/constitution.md`, `EXECUTION.md` for current operational state, the canonical platform roadmap and the active slice govern until the discrepancy is deliberately reconciled.
