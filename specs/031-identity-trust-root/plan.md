# ECR-031 Implementation Plan — Identity, Trust Root & Sensitive Storage

**Status:** PLAN_CANDIDATE / PASS_1_REMEDIATED  
**Dependencies:** ECR-001/ECR-002 `CLOSED_CANONICAL`  
**Target language:** Rust  
**Trusted crate candidate:** `crates/ecra-identity`

## 1. Summary

Implement one bounded Rust crate that turns ECR-001 identity references into cryptographically validated local identity context, bootstraps one Ecra-local installation principal, owns protected local trust-root/key lifecycle, exposes a non-ambient assertion issuance boundary, and provides versioned authenticated protection for sensitive local bytes.

Native platform key stores are explicit I/O backends. Canonical assertion/trust-state/envelope validation remains deterministic and testable. ECR-031 deliberately stops before authorization, secret-use mediation, independent outcome verification, provider execution, protocol federation, sync or local-model gateway work.

## 2. Proposed repository shape

```text
crates/ecra-identity/
  Cargo.toml
  README.md
  src/
    lib.rs
    error.rs
    ids.rs
    algorithm.rs
    bootstrap.rs
    assertion.rs
    validation.rs
    issuance.rs
    key.rs
    envelope.rs
    anchor.rs
    backend.rs
    store.rs
    platform/
      mod.rs
      macos.rs
      windows.rs
      linux.rs
  tests/
    bootstrap.rs
    assertion_contract.rs
    validation.rs
    issuance.rs
    key_lifecycle.rs
    envelope.rs
    anchor.rs
    backend_boundaries.rs
    macos_backend.rs
    portability.rs
    redaction.rs
    migration.rs

contracts/ecra-identity-v1/
  valid/
  invalid/
  expected/
  migrations/

scripts/
  check-identity-deps.sh
  check-identity-unsafe.sh

.github/workflows/ecr-031.yml
```

Do not create `ecra-keystore`, `ecra-crypto`, or other speculative crates unless implementation evidence forces a plan amendment.

## 3. Dependency direction

```text
platform native APIs / crypto dependencies
               ↑
          ecra-identity
               ↑
           ecra-core
```

Initial dependency direction:
- `ecra-identity -> ecra-core` required;
- no `ecra-identity -> ecra-run` dependency is planned;
- ECR-031 owns a tiny protected trust-state store rather than reusing ECR-002 run tables as identity authority;
- later application services compose `ecra-run` + `ecra-identity` externally.

Forbidden:
- `ecra-core -> ecra-identity`;
- model/browser/network/protocol/policy SDK dependencies in `ecra-identity`;
- native platform types in canonical public wire/domain structures.

## 4. Workstream A — primitives and strict contracts

Implement:
- `TrustRootId`, `KeyId`, `ProtectedObjectId`, enrollment/replay/delegation IDs using repository typed-ID conventions;
- exact versioned enums for key purpose/status, signature and AEAD algorithms;
- strict assertion/protected-trust-state/protected-envelope/protected-anchor serializers/deserializers;
- hard parser limits and typed errors;
- canonical JCS/domain-separated bytes.

Before cryptography, contract fixtures must prove:
- unknown/duplicate fields rejected;
- IDs/timestamps/versions bounded;
- no Actor/Principal conversion shortcut;
- no authority-bearing fields in validated identity output;
- OS username/email/display data cannot become canonical PrincipalId.

## 5. Workstream B — local bootstrap and enrollment

V1 bootstraps exactly an **Ecra-local installation principal**. It does not prove a person's legal/external identity.

Bootstrap inputs:
- explicit production `SecureRandom`;
- explicit issuance/bootstrap time context;
- selected production `TrustBackend`.

Bootstrap sequence:
1. generate opaque `PrincipalId`, `TrustRootId`, enrollment ID and initial purpose-key IDs from approved randomness;
2. create/store initial protected root/master and Ed25519 software signing secrets through the native backend;
3. construct initial `ProtectedTrustStateV1` with enrollment, key metadata and state generation;
4. protect/authenticate that trust state;
5. durably publish using the ECR-031 atomic protected-state store;
6. reopen/authenticate and validate invariants;
7. only then return `EnrolledPrincipalHandle`.

Crash semantics:
- backend material without published/verified protected state is `incomplete_bootstrap`;
- restart does not silently mint a second principal/root;
- cleanup of orphan backend material is explicit/bounded and tested.

No OS username/email/Actor label is imported as PrincipalId.

## 6. Workstream C — protected authoritative trust state

ECR-031 v1 **does require** a small owned persistent trust-state store; this is no longer deferred to implementation discovery.

Authoritative security state is the authenticated `ProtectedTrustStateV1` envelope. Ordinary metadata/indexes are non-authoritative projections.

Store requirements:
- one bounded current protected trust-state object per local installation/root;
- crash-safe atomic replacement (`temp -> durable flush -> rename/replace -> directory durability where supported`) with platform behavior tested on the accepted macOS path;
- strict version/size limits;
- authenticate/decrypt before semantic use;
- migration fixture for v1 schema envelope/version behavior;
- never persist root/private/derived keys in ordinary file metadata.

`VerifiedTrustSnapshot` is created only by opening/authenticating protected state plus validating lifecycle invariants. Pure assertion validation and assertion issuance accept this type, not raw unsigned metadata.

Rollback claim:
- v1 detects tampering under the backend-key boundary;
- `state_generation` prevents internal ambiguity but is not a hardware monotonic counter;
- restoring an older valid protected trust state together with equivalent authorized OS trust-store state is outside the universal v1 rollback-resistance claim.

## 7. Workstream D — assertion signing/validation and issuance

### Validation

Pure validation pipeline:

```text
bytes
  -> structural validation
  -> VerifiedTrustSnapshot
  -> signature verification
  -> principal
  -> actor
  -> audience
  -> time
  -> delegation
  -> replay
  -> ValidatedIdentityContext
```

No ambient clock/environment/network/native-backend call occurs inside pure validation.

### Issuance

No generic `issue(principal_id, ...)` API exists.

Issuance sequence:
1. reopen/authenticate local enrollment and protected trust state -> `EnrolledPrincipalHandle` + `VerifiedTrustSnapshot`;
2. create a process-local, non-serializable `IssuerSession` fixed to that principal/root/current assertion-signing key;
3. caller supplies allowed actor/audience/time/replay request values but **not** a replacement subject principal;
4. issue/sign assertion for the session principal only.

V1 on-behalf-of issuance cannot mint another arbitrary principal. Broader delegation authorization is ECR-003.

No ECR-031 IPC/network issuance service exists.

## 8. Workstream E — frozen v1 signing custody

Canonical v1 assertion and protected-anchor signing algorithm: **Ed25519**.

Custody model:
- generate 32-byte software Ed25519 seed/key material from production CSPRNG;
- store it only as native-backend-protected secret material;
- materialize only for bounded signing operations into a redacted/zeroizing secret wrapper;
- promptly release/zeroize after use;
- persist only public verification material in `KeyRecord`/protected trust state.

Claims:
- macOS v1 acceptance proves Data Protection Keychain protection of the software signing secret at rest;
- it does **not** claim Secure Enclave signing, hardware-backed private operations, or non-exportability for this portable path;
- capability flags for those properties remain false on the portable Ed25519 path;
- a future native non-exportable signing suite requires a versioned contract/algorithm extension and evidence.

This resolves the former portability-vs-Secure-Enclave ambiguity without weakening custody.

## 9. Workstream F — key lifecycle

Implement purpose-scoped transitions:

```text
create -> activate -> rotate -> retire -> revoke
```

Rules:
- one active key per trust-root/purpose in verified protected state;
- active-only new signing/protection;
- retired compatibility narrowly defined;
- revoked assertion-signing key rejects current assertions;
- destruction/unavailability distinct from revocation;
- every lifecycle change produces and atomically publishes a newly authenticated protected trust state;
- ordinary metadata cannot reactivate/unrevoke a key.

## 10. Workstream G — protected envelope

Dependency-lock target:
- HKDF-SHA-256 for domain-separated derived keys;
- ChaCha20-Poly1305 RFC 8439 for AEAD;
- 96-bit unique random nonce;
- full tag;
- JCS-derived AAD.

V1 portable custody assumes the backend can release the protected master secret into bounded process memory for the operation. The master secret uses the same redacted/zeroizing handling discipline as signing material; no hardware non-exportability claim is made.

API shape:

```text
protect(object metadata, SensitiveBytes) -> ProtectedEnvelopeV1
open(ProtectedEnvelopeV1, expected metadata) -> SensitiveBytes
```

Authentication failure returns no plaintext.

## 11. Workstream H — protected authenticity anchor

Implement key-backed `ProtectedAnchorV1` over exact domain-separated payload digest, using a purpose-specific Ed25519 software key protected under the same v1 custody model.

Initial consumer integration is fixture/API-level only. Do not mutate ECR-002 ledger digest semantics and do not call anchor verification ECR-004 outcome verification.

## 12. Workstream I — TrustBackend

### Common interface

Narrow Rust-owned trait supporting:
- capability report;
- create/store/open/delete protected secret material;
- locked/unavailable health state;
- no generic raw private-key export API beyond the bounded secret-open operation required by the portable v1 software crypto path.

The API returns `SensitiveBytes` only to bounded ECR-031 crypto operations; callers outside the trusted crate do not receive raw root/signing material.

### macOS first live backend

V1 acceptance requires live Data Protection Keychain tests on the trusted repository macOS runner:
- local-only/non-synchronizing items;
- protected root/master/signing secret store/open/delete;
- unavailable/not-found normalization;
- capability report truth.

No Secure Enclave signing requirement exists in v1. Hardware-backed/non-exportable flags are false for portable software signing.

### Windows

Implementation only after exact dependency/API review. If included:
- default user+machine DPAPI semantics accurately represented;
- no machine-wide default;
- no cross-machine recovery claim;
- no hardware-signing claim.

### Linux

Implementation only after exact DBus/Secret Service dependency review. If included:
- opaque IDs/fixed namespace only in lookup attributes;
- secret material only in the item secret;
- unavailable/locked service fail closed;
- upstream 0.2 draft status retained.

No platform is marked verified without native/live or explicitly approved equivalent evidence.

## 13. Randomness and time boundaries

Define explicit boundaries:

```text
SecureRandom
BootstrapContext.created_at
IssuanceContext.issued_at/not_before/expires_at
IdentityValidationContext.evaluated_at
```

Pure validation does not call clock/random. Production bootstrap/issuance/encryption use OS CSPRNG through accepted dependency. Deterministic test providers are test-only.

## 14. Error and secret-handling strategy

- typed category/code errors;
- explicit bootstrap/enrollment/issuer-session/trust-snapshot errors;
- public crypto failures may collapse to authentication failure;
- no secret/plaintext/private-key bytes in Debug/Display/loggable source chain;
- backend native errors normalized and redacted;
- tests capture formatted errors and assert synthetic sentinel secrets absent.

## 15. Dependency research gate

Before `Cargo.toml` production dependency changes, record exact candidate versions/licenses/advisories/features for:
- Ed25519 implementation;
- ChaCha20-Poly1305;
- HKDF/SHA-256;
- zeroization/secrecy wrapper;
- CSPRNG;
- macOS Security/Keychain binding dependency;
- optional Windows dependency if implemented;
- optional Linux D-Bus/Secret Service dependency if implemented.

Prefer small, widely reviewed libraries with minimal features, but current repository evidence decides. No source copying. Update `research/donor-license-ledger.md` before adoption.

## 16. CI plan

Create trusted push-only ECR-031 workflow on implementation branch and `main` using the repository-scoped self-hosted posture.

Required baseline:

```bash
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
cargo test --doc --workspace --locked
cargo test --workspace --locked --offline
bash scripts/check-core-unsafe.sh
bash scripts/check-core-deps.sh
bash scripts/check-run-unsafe.sh
bash scripts/check-run-deps.sh
bash scripts/check-identity-unsafe.sh
bash scripts/check-identity-deps.sh
```

Explicit ECR-031 targets:

```text
bootstrap
assertion_contract
validation
issuance
key_lifecycle
envelope
anchor
redaction
backend_boundaries
migration
macos_backend
portability
```

ECR-001/ECR-002 workflows remain regression oracles on `main`.

## 17. Constitution gates

### G1 Domain coherence — PASS
Reuses ECR-001 Actor/Principal/IdentityAssertion references; no second principal namespace.

### G2 Authority — PASS
Validated context has no authority; assertion issuance requires enrolled identity handle/session and cannot mint arbitrary caller-selected principals. ECR-003 remains authorization owner.

### G3 Provenance — PASS
Issuer/root/key/digest/enrollment/trust-snapshot provenance explicit.

### G4 Side effects — PASS
Bootstrap, key create/rotate/revoke and protected-state writes are consequential local effects with crash/atomicity tests.

### G5 Verification — PASS
Crypto validation authenticates identity/protected bytes, not ECR-004 external outcome truth.

### G6 Durability — PASS
ProtectedTrustStateV1 is authoritative, crash-safe persisted, reopen-authenticated; incomplete bootstrap and rollback boundaries are explicit.

### G7 Privacy/secrets — PASS
No plaintext fallback; bounded secret materialization; redacted errors; authenticated storage.

### G8 Local-first — PASS
No cloud account/provider required.

### G9 Interoperability — PASS
Native stores/protocol standards are adapters; no external token becomes local authority.

### G10 Donor/license — PASS_PENDING_DEPENDENCY_LOCK
Exact dependency/license/advisory review remains T001 before adoption.

### G11 Upstream/browser maintenance — PASS-N/A
No browser patch/bridge.

### G12 Benchmarks — PASS
Reproducible security/contract acceptance; no superiority claims.

### G13 Information flow / egress — PASS
No remote egress; later disclosure remains ECR-003.

### G14 Identity / principal binding — PASS
Bootstrap defines an Ecra-local principal without external proofing claim; issuance is session-bound and fail closed.

### G15 Bounded execution — PASS
Hard parser/state limits; no recursive model/tool loop; bounded native/crypto operations.

No constitutional gate fails after Pass-1 remediation. G10 is an implementation dependency-lock gate, not a planning defect.

## 18. Complexity tracking

### One new trusted crate
Necessary because ECR-001 must remain zero-I/O while ECR-031 needs keystore/crypto/persistence operations.

### One tiny protected trust-state store
Necessary because lifecycle/revocation/enrollment must not depend on unsigned filesystem metadata. Reusing ECR-002 run tables would create the wrong authority coupling. Simpler unsigned metadata would fail A2/R-053.

### Native backend abstraction
Necessary because root/software-key custody differs by platform. Plaintext file keys violate the spec.

### Portable software Ed25519 v1
Chosen to keep one stable v1 assertion/anchor wire while making custody claims honest. Secure Enclave algorithm variation is deferred to a versioned extension rather than hidden behind a misleading abstraction.

## 19. Implementation authorization gate

Implementation is forbidden until:
- contract/data model/threat model include Pass-1 remediation;
- exact dependency candidates are researched enough to make tasks executable;
- requirements checklist is rerun against C1–C4/C5;
- tasks explicitly cover bootstrap, protected trust snapshot, issuance misuse and software-signing custody;
- analyze Pass 2 reports zero critical planning drift and G1–G15 pass/explicit N/A;
- platform roadmap/status/EXECUTION truth is synchronized to `TASKS_READY` on an exact green planning head.
