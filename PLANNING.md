# Ecra Planning Index

Ecra uses Spec-Driven Development with a platform-scale Spec Kit “spec of specs” decomposition.

## Start Here

For implementation work, use this order:

1. `.specify/memory/constitution.md` — binding governance and Definition of Done.
2. `EXECUTION.md` — current execution truth, next eligible work and exact verification rules.
3. `specs/README.md` — Spec Kit navigation.
4. `specs/000-ecra-platform/roadmap.md` — canonical ECR dependency graph.
5. the active slice package and its `STATUS.md` / `tasks.md` / `analyze.md`.
6. exact live Git/GitHub branch, PR, CI, review and head truth.

`EXECUTION.md` exists so a new human or coding agent can continue from repository truth without a private chat handoff.

## Canonical Governance

- `.specify/memory/constitution.md` — binding Spec Kit constitution and Definition of Done.
- `AGENTS.md` — repository execution rules for coding agents.
- `EXECUTION.md` — operational execution entry point.
- `specs/README.md` — feature-package navigation and lifecycle rules.

## Closed Trusted Substrate

### ECR-001 — Trusted Domain Kernel

Lifecycle: **CLOSED_CANONICAL**.  
Closure-ledger head `85e4bf657b6c33e3f88d83e92e7a35279d177349`; CI `33099434232` SUCCESS.

ECR-001 owns provider-neutral zero-I/O trusted domain types and invariants, including Actor vs Principal/IdentityAssertion references.

### ECR-002 — Durable Run, Ledger & Budgets

Lifecycle: **CLOSED_CANONICAL**.  
Final feature head `87fd9fc560bf5ca21a07a4d25473f305b4c05f05`; merge `40efc8a64a9562f0f3eb2555b350cfa03d3e0675`.  
Final closure-convergence head `aadc19c972e619222d426674d7542dd9c00dbe44`; ECR-002 CI `33155302100` and ECR-001 regression `33155302026` SUCCESS.

ECR-002 owns durable run truth, attempts/recovery, budgets, SQLite durability and deterministic synthetic/non-sensitive `.ecra` interchange. It does not own authentication/trust roots, authorization, independent outcome verification or provider execution.

## Active Slice — ECR-031

### Identity, Trust Root & Sensitive Storage Foundations

Lifecycle: **TASKS_READY_PENDING_EXACT_GREEN_HEAD**.  
Dependencies: ECR-001/ECR-002 `CLOSED_CANONICAL`.  
Analyze Pass 2: `a3c7d563c139c65886f169f9181c07a997038f1f` — `ZERO_BLOCKING_PLANNING_DRIFT_FOUND`.

Planning package:

```text
specs/031-identity-trust-root/STATUS.md
specs/031-identity-trust-root/spec.md
specs/031-identity-trust-root/research.md
specs/031-identity-trust-root/data-model.md
specs/031-identity-trust-root/contracts/identity-trust-v1.md
specs/031-identity-trust-root/threat-model.md
specs/031-identity-trust-root/plan.md
specs/031-identity-trust-root/tasks.md
specs/031-identity-trust-root/quickstart.md
specs/031-identity-trust-root/analyze.md
specs/031-identity-trust-root/checklists/requirements.md
```

Planning result:

```text
FR-001–FR-058 OWNED
SC-001–SC-016 OWNED
G1–G15 PASS / explicit PASS-N/A
UNOWNED_FR=0
UNOWNED_SC=0
MUST_LEVEL_PLANNING_GAPS=0
FAILED_CONSTITUTION_GATES=0
PASS_1_BLOCKERS_REMEDIATED=4/4
```

Pass-1 blockers now frozen in canonical planning:

```text
C1 bootstrap       opaque Ecra-local principal; no OS-label/external-proofing shortcut
C2 lifecycle       authenticated ProtectedTrustStateV1 -> VerifiedTrustSnapshot authority
C3 issuance        EnrolledPrincipalHandle + VerifiedTrustSnapshot -> fixed IssuerSession; no arbitrary mint
C4 signing custody portable Ed25519 software key protected by native backend; no Secure Enclave/hardware claim
```

Implementation remains blocked until the final synchronized planning head passes both permanent ECR-001 and ECR-002 workflows. `031-identity-trust-root` must be created from that exact green SHA, followed by a Draft implementation PR and T001 dependency/license/advisory/MSRV re-verification.

## Parallel eligibility

ECR-004 — Verification & Reconciliation — is independently planning-eligible from ECR-001/ECR-002. It remains separate from ECR-031. ECR-003 remains implementation-blocked until ECR-031 is `CLOSED_CANONICAL`.

## Platform Execution Shape

```text
Trusted substrate
ECR-001 [CLOSED] -> ECR-002 [CLOSED] -> {ECR-031 [TASKS_READY GATE], ECR-004 [ELIGIBLE]} -> ECR-003 -> ECR-005

Browser wedge
ECR-006 -> ECR-007 -> ECR-008

Trusted knowledge/context
ECR-009 -> ECR-010 -> ECR-011

Learn once / replay cheaply
ECR-012 -> ECR-013 -> ECR-014 -> ECR-015

Ecosystem and work surfaces
ECR-016 / ECR-017 -> ECR-018 / ECR-019 / ECR-020 / ECR-021
```

ECR-022–ECR-030 own later sync, registry, supply chain, privacy/diagnostics, accessibility/i18n, source compliance, benchmark, portability and ecosystem concerns according to the roadmap dependency graph.

## Execution Rule

Do not implement a later `ECR-###` slice because it is exciting or independently codeable. Follow dependency ordering in the canonical roadmap.

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

- Actor attribution is not authenticated Principal identity.
- IdentityAssertionRef existence is not validated identity.
- missing/empty scope never means unrestricted; `ANY` is explicit.
- web/model/tool/memory content is not instruction authority.
- read authority does not imply disclosure authority.
- CapabilityRequest does not become CapabilityGrant.
- identity evidence does not become authorization.
- ActionIntent and ActionAttempt are different identities.
- executor-observed success is not VERIFIED.
- UNKNOWN external outcome is not silently coerced.
- generic/plain digests are not automatically protected authenticity proof.
- remote providers are information-disclosure boundaries.
- raw secrets should remain behind mediated/protected handles when possible.

## Keeping the Repository Easy to Continue

Whenever execution materially advances:
1. update the active slice `STATUS.md`;
2. update `EXECUTION.md` if active phase/slice or exact verified baseline changes;
3. update platform roadmap/status when lifecycle changes;
4. keep task ordering and clarifications in the active Spec Kit package;
5. do not leave the only accurate state in a chat handoff, commit message or PR comment.

## Strategic Documents

Root `README.md`, `VISION.md`, `CONSTITUTION.md`, and `ROADMAP.md` explain product ambition but do not replace the canonical Spec Kit constitution, operational guide, platform roadmap, or active slice package.
