# ECR-031 Status — Identity, Trust Root & Sensitive Storage Foundations

**Slice:** ECR-031  
**Lifecycle:** PLANNING_REWORK  
**Dependencies:** ECR-001 `CLOSED_CANONICAL`, ECR-002 `CLOSED_CANONICAL`  
**Canonical planning base:** `aadc19c972e619222d426674d7542dd9c00dbe44`  
**ECR-002 closure-head CI:** `33155302100` — SUCCESS  
**ECR-001 regression CI:** `33155302026` — SUCCESS  
**Planning analyze pass 1:** `44e85aa9ccd28e185a5761889aa12b50459f286e` — `PLANNING_REWORK_REQUIRED`  
**Constitution:** v1.1.0

ECR-031 is the selected next critical-path planning slice, but implementation is currently forbidden. Analyze Pass 1 found four MUST-level gaps affecting G2/G6/G14. The package must converge C1–C4 into the primary spec/data-model/contract/threat-model/plan/quickstart/tasks/checklist and then pass Analyze Pass 2 with zero blocking drift before `TASKS_READY`.

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

## Analyze Pass 1 blockers

### C1 — Local principal bootstrap / enrollment

V1 must define an Ecra-local installation principal bootstrap. It MUST NOT claim legal/real-world identity proofing and MUST NOT derive canonical `PrincipalId` from an OS username, email, Actor label, account display name or protocol subject string. First enrollment must bind a fresh opaque principal to the protected local trust root and fail closed on partial initialization.

### C2 — Protected authoritative trust snapshot

Security-critical current-generation/revocation state must be authenticated/protected under the trust root/backend. Ordinary unsigned DB/file metadata may be audit/projection material but MUST NOT reactivate a retired/revoked key. Validation consumes only a verified trust snapshot. V1 does not claim universal monotonic rollback resistance if an attacker can restore the entire authorized native trust store or equivalent root state.

### C3 — Non-ambient assertion issuance

ECR-031 MUST NOT expose an arbitrary `issue(principal_id, ...)` identity mint. Issuance requires an opaque enrolled-principal/issuer session derived from successful protected bootstrap or an explicitly validated parent identity path. The subject principal comes from that session, never from an arbitrary caller-selected ID. General delegation authorization remains ECR-003.

### C4 — Frozen v1 signing custody

V1 portable assertion/protected-anchor signing is Ed25519. The Ed25519 private key is generated from CSPRNG and persisted only as a protected secret under the native trust backend; it may be materialized only for bounded signing use and is then zeroized according to the selected secret wrapper. V1 MUST NOT claim this signing key is Secure Enclave-backed/non-exportable. Native hardware signing is a future versioned algorithm-suite extension.

## Current planning direction — not implementation authorization

- Reuse ECR-001 `PrincipalRef`, `PrincipalId`, `IdentityAssertionRef`, `IdentityAssertionId`, `Actor` and `ActorId`; do not create competing principal/actor identities.
- Add one bounded trusted crate candidate, `ecra-identity`, rather than speculative crate decomposition.
- Pure assertion/envelope validation receives explicit evaluation context; no hidden clock/randomness in canonical validation.
- Native trust-store access is behind a Rust-owned fail-closed backend interface.
- Production must never silently fall back to plaintext or an in-memory key when a platform trust store is unavailable/locked.
- v1 protected payloads use versioned authenticated encryption and bind security-relevant metadata as AAD.
- Key lifecycle distinguishes active, verification/decryption-only retirement and revocation; transitions are explicit and security-critical state is protected/authenticated.
- Stronger ledger authenticity can later consume a protected trust-root signature/MAC primitive without rewriting ECR-002 run truth.
- Fully compromised same-user account/kernel/debugger or equivalent keystore authority remains outside the guaranteed containment claim unless a specific backend provides narrower stronger evidence.

## Next planning work

1. fold C1–C4 into primary normative planning documents;
2. add bootstrap/trust-snapshot/issuance-misuse/signing-custody fixtures and task ownership;
3. re-run the requirements checklist;
4. run Analyze Pass 2;
5. require G1–G15 PASS / explicit N/A and zero MUST-level drift;
6. only then update lifecycle to `TASKS_READY`, synchronize platform lifecycle docs, obtain exact-head main CI, and create an implementation branch/PR.
