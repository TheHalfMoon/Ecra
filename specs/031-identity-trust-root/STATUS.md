# ECR-031 Status — Identity, Trust Root & Sensitive Storage Foundations

**Slice:** ECR-031  
**Lifecycle:** PLANNING / SPECIFICATION  
**Dependencies:** ECR-001 `CLOSED_CANONICAL`, ECR-002 `CLOSED_CANONICAL`  
**Canonical planning base:** `aadc19c972e619222d426674d7542dd9c00dbe44`  
**ECR-002 closure-head CI:** `33155302100` — SUCCESS  
**ECR-001 regression CI:** `33155302026` — SUCCESS  
**Constitution:** v1.1.0

ECR-031 is the selected next critical-path planning slice. This package is planning-only until `spec.md`, research/data model/contracts/threat model/plan, requirements checklist, executable tasks and analyze-equivalent consistency review are complete with no blocking constitutional defect.

## Roadmap-owned outcome

```text
IdentityAssertion validation
Actor ↔ Principal / on-behalf-of binding
Device/user-local trust root
Key issue/rotation/revocation
Protected authenticity/MAC/signature envelope primitives
Sensitive local-storage protection contracts
```

## Hard boundaries

ECR-031 MUST NOT implement or counterfeit:

- general authorization, capability narrowing, declassification, approval or execution leases — ECR-003;
- independent verification/reconciliation decisions — ECR-004;
- browser/model/tool/process/provider execution;
- protocol token passthrough or MCP/ACP/A2A gateway semantics — ECR-016;
- local-model gateway behavior — ECR-021;
- multi-device encrypted sync/recovery — ECR-022;
- telemetry/privacy product controls — ECR-025;
- general import/export portability — ECR-029.

A validated identity assertion answers **who / on whose behalf / under which trust root and bounded assertion context**. It never means **what is authorized**.

## Current planning direction — not implementation authorization

- Reuse ECR-001 `PrincipalRef`, `PrincipalId`, `IdentityAssertionRef`, `IdentityAssertionId`, `Actor` and `ActorId`; do not create competing principal/actor identities.
- Add one bounded trusted crate candidate, `ecra-identity`, rather than speculative crate decomposition.
- Pure assertion/envelope validation receives explicit evaluation context; no hidden clock/randomness in canonical validation.
- Native trust-store access is behind a Rust-owned fail-closed backend interface.
- Production must never silently fall back to plaintext or an in-memory key when a platform trust store is unavailable/locked.
- v1 protected payloads use versioned authenticated encryption and bind security-relevant metadata as AAD.
- Key lifecycle distinguishes active, verification/decryption-only retirement and revocation; transitions are explicit and durable where required.
- Stronger ledger authenticity can later consume a protected trust-root signature/MAC primitive without rewriting ECR-002 run truth.
- Fully compromised same-user account/kernel/debugger or equivalent keystore authority remains outside the guaranteed containment claim unless a specific hardware-backed backend provides a narrower stronger guarantee.

## Next planning work

1. freeze functional requirements and success criteria in `spec.md`;
2. record primary-source and platform-keystore research;
3. freeze data/wire contracts and threat model;
4. pass G1–G15 in `plan.md`;
5. generate traceable executable `tasks.md` and requirements checklist;
6. run analyze-equivalent consistency review;
7. only then update lifecycle to `TASKS_READY` and create an implementation branch/PR.
