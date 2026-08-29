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

## IC-002 — T064 requires a provisioning-authorized app-like macOS test host

**Date:** 2026-08-29
**Discovered at:** T064 external-readiness investigation after ECR-004 canonical closure
**Classification:** MUST-level native-acceptance harness defect
**Status:** REMEDIATED_IN_IMPLEMENTATION / EXTERNAL_ASSETS_STILL_REQUIRED

### Finding

The first T064 workflow executed the ignored Rust library tests directly from Cargo's ordinary test binary while the tests themselves correctly stated that Data Protection Keychain acceptance requires a provisioned app-like macOS host.

That execution shape was insufficient for the claimed acceptance boundary. Apple TN3137 documents that macOS Data Protection Keychain access groups are derived from the host process code-signing entitlements, those restricted entitlements must be authorized by a provisioning profile, and a command-line-style executable needs an app-like bundle structure in which to embed that profile. Apple TN3125 further documents that provisioning-profile entitlements are an authorization allowlist and that wildcard authorization in the profile must be resolved to concrete, non-wildcard entitlements in the signed program. Apple's guidance for signing nonbundled executables with restricted entitlements likewise requires an app-like wrapper.

A bare Cargo test executable therefore cannot become valid T064 evidence merely because an Apple Development certificate and provisioning profile exist on the runner. The test executable must run as the signed main executable of a provisioning-authorized app-like bundle.

### Remediation

T064 keeps its stable task ID and frozen security intent. Repository-owned live acceptance now uses `scripts/run-ecr031-macos-live-acceptance.sh` to:

1. require an interactive macOS console session owned by the runner user;
2. require `security`, `codesign`, `xcodebuild`, and `python3`;
3. select an unexpired macOS provisioning profile that authorizes the fixed synthetic bundle identifier `dev.ecra.identity.t064` and its concrete keychain access group;
4. require an `Apple Development` signing identity for the same profile team;
5. build the exact `ecra-identity` Rust library test executable without running it bare;
6. place that executable at `EcraT064Host.app/Contents/MacOS/EcraT064Host` with a matching `Info.plist`;
7. embed the selected provisioning profile at `Contents/embedded.provisionprofile`;
8. claim only the concrete application-identifier, team-identifier, and keychain-access-groups entitlements authorized by that profile;
9. sign the app-like bundle with the matching Apple Development identity and verify the signature plus claimed entitlements;
10. execute both ignored Data Protection Keychain acceptance tests only through that signed app-like host;
11. delete all temporary host material after the run.

The dedicated readiness workflow now invokes the same repository script in `--readiness-only` mode. Missing or mismatched signing/profile assets are a real failing gate rather than a green diagnostic with failed `continue-on-error` probes. Xcode account/team registry checks remain diagnostics because a valid certificate/profile may be installed by other authorized means; the security acceptance boundary is the actual signing identity, provisioning-profile authorization, signed entitlements, and live Data Protection Keychain behavior.

### Security effect

This remediation strengthens the T064 evidence boundary. It does not:

- substitute the legacy file-based Keychain;
- enable `synchronizing=true`;
- introduce plaintext/file/environment/memory fallback;
- accept ad-hoc signing;
- claim Secure Enclave, hardware-backed, non-exportable, user-presence, cross-machine, or recovery guarantees;
- treat possession of a certificate/profile as proof that the live Keychain tests passed;
- change ECR-003/ECR-004/ECR-005 or later-slice ownership.

### Remaining external prerequisite

Repository-side host packaging is now defined, but the trusted macOS runner must still possess an unexpired macOS provisioning profile authorizing `dev.ecra.identity.t064` plus a matching Apple Development signing identity. Xcode account/team configuration is one acceptable way to obtain those assets but is not itself the acceptance claim.

T064 remains open until both live ignored tests pass through the signed app-like host on one exact feature head. T068 remains blocked until that evidence exists.