# Ecra Planning Index

Ecra uses Spec-Driven Development with a platform-scale Spec Kit “spec of specs” decomposition.

## Start Here

For implementation work, use this order:

1. `.specify/memory/constitution.md` — binding governance and Definition of Done.
2. `EXECUTION.md` — current execution truth, next eligible work and exact verification rules.
3. `specs/README.md` — Spec Kit navigation.
4. `specs/000-ecra-platform/roadmap.md` — canonical ECR dependency graph.
5. the active slice package and its `STATUS.md` / `tasks.md` / analyze artifacts.
6. exact live Git/GitHub branch, PR, CI, review and head truth.

`EXECUTION.md` exists so a new human or coding agent can continue from repository truth without a private chat handoff.

## Canonical Governance

- `.specify/memory/constitution.md` — binding Spec Kit constitution and Definition of Done.
- `AGENTS.md` — repository execution rules for coding agents.
- `EXECUTION.md` — operational execution entry point.
- `specs/README.md` — feature-package navigation and lifecycle rules.
- `specs/000-ecra-platform/roadmap.md` — immutable ECR dependency graph.

## Closed trusted substrate

### ECR-001 — Trusted Domain Kernel

Lifecycle: **CLOSED_CANONICAL**.  
Closure-ledger head `85e4bf657b6c33e3f88d83e92e7a35279d177349`; CI `33099434232` SUCCESS.

### ECR-002 — Durable Run, Ledger & Budgets

Lifecycle: **CLOSED_CANONICAL**.  
Final closure-convergence head `aadc19c972e619222d426674d7542dd9c00dbe44`; ECR-002 CI `33155302100` and ECR-001 regression `33155302026` SUCCESS.

### ECR-004 — Verification & Reconciliation

Lifecycle: **CLOSED_CANONICAL**.  
Merged implementation PR #7.  
Merged feature head `990addb79e6fe5a1ad2b16dae159c624959e2128`.  
Canonical implementation merge `2a95fbb4f20b1646505cb179f4822a758a546895`.  
Closure-convergence head `c159c96061a73ead9710985d07608e2b417fe275`.

Closure-convergence exact-head gates:

```text
ECR-001  RUN 33256430974  JOB 99110882402  SUCCESS
ECR-002  RUN 33256430942  JOB 99110916386  SUCCESS
ECR-004  RUN 33256430965  JOB 99110882233  SUCCESS
```

The canonical `main` head containing the T053 marker must pass the same three workflows before an external final closure claim is made.

ECR-004 remains bounded to independent verification/reconciliation. It does not grant authority, declassify data, validate identity, store real sensitive evidence, execute providers, clear ECR-002 unresolved attempts, or repair/resume the same unresolved run.

## Active dependency frontier — ECR-031

### Identity, Trust Root & Sensitive Storage Foundations

Lifecycle: **IMPLEMENTING / BLOCKED_EXTERNAL_NATIVE_ACCEPTANCE**.  
Implementation branch: `031-identity-trust-root`.  
Draft implementation PR: #4.  
Current blocking task: T064.

The live ECR-031 branch/package controls exact task progress. Native macOS Data Protection Keychain acceptance remains blocked because the trusted runner user lacks the Apple Development signing identity, suitable provisioning profile, configured Xcode developer account registry and usable development team required for the app-like acceptance host.

The exact unblock condition is external: configure a valid Apple developer account/team in Xcode for the same runner user and allow Xcode to create/install an Apple Development certificate/signing identity plus suitable provisioning profile, then rerun T064 on the exact feature head.

Do not substitute legacy file-based Keychain, ad-hoc signing, plaintext/file/environment/memory fallback, `synchronizing=true`, weakened live acceptance, or unsupported Secure Enclave/hardware/non-exportability claims.

## Blocked dependents

### ECR-003 — Authority, Information Flow, Policy & Secrets

Lifecycle: **PLANNED_BLOCKED**.  
Depends on ECR-001, ECR-002 and ECR-031. Implementation remains forbidden until ECR-031 is `CLOSED_CANONICAL` and the normal Spec Kit lifecycle authorizes the slice.

### ECR-005 — Evaluation & Threat Harness

Lifecycle: **PLANNED_BLOCKED_BY_DEPENDENCIES**.  
Depends on ECR-001, ECR-002, ECR-003, ECR-004 and ECR-031. ECR-004 closure alone does not unblock it.

No later ECR slice becomes implementation-eligible merely because the ECR-031 lane is externally blocked.

## Platform execution shape

```text
Trusted substrate
ECR-001 [CLOSED] -> ECR-002 [CLOSED]
        -> {ECR-031 [BLOCKED_EXTERNAL], ECR-004 [CLOSED]}
        -> ECR-003 [BLOCKED] -> ECR-005 [BLOCKED]

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

Do not implement a later `ECR-###` slice because it is independently codeable or because the current critical path has an external blocker. Follow dependency ordering in the canonical roadmap.

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

Only `CLOSED_CANONICAL` dependencies unlock dependent implementation unless the dependent spec explicitly authorizes bounded fixture-only research that cannot counterfeit the missing dependency.

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
