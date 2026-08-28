# ECR-031 Implementation Clarifications

## IC-001 — Phase 4 dependency-order correction

**Date:** 2026-08-28
**Discovered at:** exact green Phase 3 closure head `7eaede3f9f10461c307c8900c021273a4dbffa03`
**Classification:** MUST-level implementation dependency defect
**Status:** REMEDIATED_IN_PLAN

### Finding

The original task graph placed T035–T042 before T043–T053 and T059–T068. Live implementation review showed that this order is not executable without weakening frozen requirements:

- local bootstrap requires the approved production randomness boundary owned by T044;
- protected secret handling required by bootstrap/rotation/backend operations requires T043;
- the authenticated trust-state store required by T041 depends on `ProtectedEnvelopeV1`, HKDF-SHA-256, ChaCha20-Poly1305 and authenticated open owned by T045–T050;
- rotation/bootstrap protected-secret operations require the Rust-owned `TrustBackend` and production/test selection boundary owned by T059–T060;
- crash/bootstrap tests in T040 depend on the protected store and bootstrap transaction, so they cannot precede those implementations;
- the original T035 combined pure schema work with generated/durable bootstrap completion, creating a cycle with T041.

Implementing placeholders, plaintext fallbacks, unsigned trust state, caller-supplied “generated” IDs, or a fake production backend would violate FR-021–FR-025 and C1/C2. Those shortcuts are prohibited.

### Remediation

Requirements, security claims and task ownership are not weakened. Task IDs already referenced by the planning package remain stable.

Execution dependencies are corrected as follows:

```text
Phase 3 exact closure gate
        ↓
T043 SensitiveBytes
        ↓
T044 SecureRandom
        ↓
T045 → T046 → T047 → T048 → T049 → T050
        ↓
T059 → T060
        ↓
T035 pure trust/enrollment/key schemas and invariants
        ↓
T036 unambiguous active-key selection
        ↓
T041 authenticated protected trust-state store
        ↓
T041A complete local bootstrap/enrollment transaction
        ↓
T038 retirement semantics
        ↓
T037 rotation using the protected store
        ↓
T039 revocation semantics
        ↓
T040 exhaustive bootstrap/lifecycle/crash tests
        ↓
T042 Phase 4 exact-head gate
```

T041A is a convergence subtask added because the original T035 mixed schema definition with a durable transaction that necessarily depends on T041. T041A owns the complete generated-ID bootstrap transaction: generate fresh opaque identifiers via `SecureRandom`, create/protect required backend secrets, publish authenticated protected state atomically, reopen/authenticate, then and only then return `EnrolledPrincipalHandle`. Partial initialization remains `incomplete_bootstrap` and never silently mints a second identity.

After T042, unfinished original phase families resume without renumbering: T051–T053, T054–T058, T061–T068, then T069 onward.

### Security effect

This correction strengthens implementation ordering. It does not broaden ECR-031 scope and does not change:

- Ecra-local identity non-claims;
- protected trust state as the sole lifecycle authority;
- no plaintext/in-memory/environment/file-key production fallback;
- no arbitrary caller-selected principal issuance;
- Ed25519 portable signing custody claims;
- rollback non-claim;
- ECR-003/ECR-004/ECR-016 and other slice boundaries.

### Gate

No semantic post-Phase-3 code may begin until this clarification plus synchronized `tasks.md`, `plan.md`, `STATUS.md`, `EXECUTION.md` and `analyze.md` is committed and the permanent ECR-031 workflow succeeds on that exact convergence head.
