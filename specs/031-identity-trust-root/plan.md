# ECR-031 Implementation Plan — Identity, Trust Root & Sensitive Storage

**Status:** PLAN_CANDIDATE  
**Dependencies:** ECR-001/ECR-002 `CLOSED_CANONICAL`  
**Target language:** Rust  
**Trusted crate candidate:** `crates/ecra-identity`

## 1. Summary

Implement one bounded Rust crate that turns ECR-001 identity references into cryptographically validated local identity context, owns local trust-root/key lifecycle and exposes a versioned authenticated-protection primitive for sensitive local bytes. Native platform key stores are explicit I/O backends; identity validation and canonical cryptographic input construction remain deterministic and testable.

The slice deliberately stops before authorization, secret-use mediation, independent verification, provider execution, protocol federation, sync or local-model gateway work.

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
    assertion.rs
    validation.rs
    key.rs
    envelope.rs
    anchor.rs
    backend.rs
    store.rs              # only if implementation proves ECR-031-owned metadata durability necessary
    platform/
      mod.rs
      macos.rs
      windows.rs
      linux.rs
  tests/
    assertion_contract.rs
    validation.rs
    key_lifecycle.rs
    envelope.rs
    anchor.rs
    backend_boundaries.rs
    macos_backend.rs
    portability.rs
    redaction.rs
    migration.rs          # if a persistent metadata store exists

contracts/ecra-identity-v1/
  valid/
  invalid/
  expected/
  migrations/             # only if persisted schema exists

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
          ↑          ↑
     ecra-core    ecra-run? (only if an exact current durability need exists)
```

Preferred initial dependency direction:
- `ecra-identity -> ecra-core` required;
- avoid `ecra-identity -> ecra-run` unless ECR-031 itself truly needs run-ledger types;
- later application services can compose `ecra-run` + `ecra-identity` without either becoming the other's authority source.

Forbidden:
- `ecra-core -> ecra-identity`;
- model/browser/network/protocol/policy SDK dependencies in `ecra-identity`;
- native platform types in canonical public wire/domain structures.

## 4. Workstream A — primitives and strict contracts

Implement:
- `TrustRootId`, `KeyId`, `ProtectedObjectId`, optional replay/delegation IDs using repository typed-ID conventions;
- exact versioned enums for key purpose/status, signature and AEAD algorithms;
- strict assertion/protected-envelope/protected-anchor serializers/deserializers;
- hard parser limits and typed errors;
- canonical JCS/domain-separated bytes.

Before cryptography, contract fixtures must prove:
- unknown/duplicate fields rejected;
- IDs/timestamps/versions bounded;
- no Actor/Principal conversion shortcut;
- no authority-bearing fields in validated identity output.

## 5. Workstream B — assertion signing/validation

Implement pure validation pipeline from contract order:

```text
bytes -> structural validation -> trust/key lookup snapshot -> signature -> principal -> actor -> audience -> time -> delegation -> replay -> ValidatedIdentityContext
```

Inputs:
- immutable assertion bytes/object;
- immutable trust/key snapshot or narrow key resolver;
- explicit `IdentityValidationContext`.

No ambient clock/environment/network.

Signing/issuance is a separate service path using explicit clock/random/backend inputs. The validator must be independently usable against fixtures/public verification material.

## 6. Workstream C — key lifecycle

Implement purpose-scoped key metadata and transitions:

```text
create -> activate -> rotate -> retire -> revoke
```

Rules:
- one active key per trust-root/purpose;
- active-only new signing/protection;
- retired compatibility narrowly defined;
- revoked assertion-signing key rejects current assertions;
- destruction/unavailability distinct from revocation;
- no raw private key in serializable metadata.

If persistent metadata is needed, use a small versioned local store with crash-safe atomicity/migration fixtures. Do not duplicate ECR-002's run ledger or make run history the identity database.

## 7. Workstream D — protected envelope

Lock exact crypto dependencies only after dependency research. Preferred candidate:
- HKDF-SHA-256 for domain-separated derived keys when raw master IKM is legitimately available;
- ChaCha20-Poly1305 RFC 8439 for AEAD;
- 96-bit unique random nonce;
- full tag;
- JCS-derived AAD.

If native hardware-backed operations cannot safely expose IKM, use a backend-protected wrapping model instead of extracting a protected key. Amend the contract before implementation if required.

API shape should make authenticated decryption impossible to confuse with unauthenticated bytes:

```text
protect(object metadata, SensitiveBytes) -> ProtectedEnvelopeV1
open(ProtectedEnvelopeV1, expected metadata) -> SensitiveBytes
```

`SensitiveBytes` must use redacted Debug behavior and zeroization where a reviewed dependency/implementation makes it meaningful without overclaiming complete memory erasure.

## 8. Workstream E — protected authenticity anchor

Implement key-backed `ProtectedAnchorV1` over exact domain-separated payload digest. Initial consumer integration is fixture/API-level only unless the active ECR-031 tasks explicitly add an ECR-002 optional anchor adapter.

Do not mutate existing ECR-002 ledger digest semantics.

## 9. Workstream F — TrustBackend

### Common interface

Narrow Rust-owned trait with:
- backend capability report;
- create/open protected root material/key;
- requested signing/MAC/protection operation;
- locked/unavailable health state;
- explicit deletion/revocation hooks only where meaningful.

No generic raw private-key export.

### macOS first live backend

The existing trusted CI runner is macOS, so v1 acceptance can require live macOS native backend tests.

Plan:
- use Data Protection Keychain for local-only protected items;
- no synchronizable/iCloud item for v1;
- prefer system access control appropriate to operation;
- Secure Enclave path only if algorithm/product constraints make it usable without weakening the contract;
- capability report reflects actual configuration.

### Windows

Implementation only after exact dependency/API review. If included in v1:
- default user+machine DPAPI protection semantics accurately represented;
- no `CRYPTPROTECT_LOCAL_MACHINE` default;
- no cross-machine recovery claim;
- if asymmetric native signing required, explicitly research CNG/NCrypt instead of simulating it with DPAPI.

### Linux

Implementation only after exact DBus/Secret Service dependency review. If included in v1:
- opaque IDs/fixed namespace only in lookup attributes;
- secret material only in secret value;
- unavailable/locked service fail closed;
- documentation says upstream 0.2 is currently draft.

### Cross-platform acceptance rule

No platform is marked verified without native/live or explicitly approved equivalent evidence. Compile-only support is labeled compile-only.

## 10. Persistent state decision

Default plan: minimize ECR-031-owned persistence.

Persist only what cannot be reconstructed from native backend plus protected metadata:
- trust-root ID/backend binding;
- key public metadata/lifecycle/generation;
- replay nonce state only if v1 issues single-use assertions;
- protected envelopes owned by ECR-031 itself.

If SQLite is chosen, reuse reviewed `rusqlite` version only if dependency evidence supports it, but create ECR-031-specific schema/migration contracts rather than coupling to ECR-002 tables.

## 11. Randomness and time boundaries

Define traits/inputs such as:

```text
SecureRandom
Clock or explicit IssuanceContext
ValidationContext.evaluated_at
```

Pure validation never calls them. Issuance/encryption requires CSPRNG in production; deterministic test providers are explicit test-only dependencies.

## 12. Error and secret-handling strategy

- typed category/code errors;
- public crypto failures may collapse to authentication failure;
- no secret/plaintext/private-key bytes in Debug/Display/source chain exposed to logs;
- backend native errors normalized and redacted;
- tests capture formatted errors and assert synthetic sentinel secrets absent.

## 13. Dependency research gate

Before `Cargo.toml` production dependency changes, record exact candidate versions/licenses/advisories/features for:
- Ed25519/signature implementation candidate;
- ChaCha20-Poly1305;
- HKDF/SHA-256;
- zeroization/secrecy wrapper if used;
- macOS Security/Keychain binding dependency;
- Windows dependency if implemented;
- Linux D-Bus/Secret Service dependency if implemented.

Prefer RustCrypto/widely maintained crates with minimal features, but repository evidence—not brand familiarity—decides.

No source copying. Update `research/donor-license-ledger.md` before implementation commit that adopts a dependency/native binding.

## 14. CI plan

Create trusted push-only ECR-031 workflow on implementation branch and `main`, using the same repository-scoped self-hosted security posture.

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

Plus explicit ECR-031 targets for assertion contract, validation, lifecycle, envelope, anchor, redaction, boundaries and live macOS backend.

ECR-001/ECR-002 workflows remain regression oracles on `main`.

## 15. Constitution gates

### G1 Domain coherence — PASS
Reuses ECR-001 Actor/Principal/IdentityAssertion references; new types only for trust/key/protected storage.

### G2 Authority — PASS
Validated identity context contains no grant/approval/authorization. Identity is input to later ECR-003 fail-closed policy.

### G3 Provenance — PASS
Assertion issuer/trust root/signing key/digest are explicit; protected objects bind key/purpose metadata.

### G4 Side effects — PASS/NARROW
Key create/rotate/revoke/protected-write are consequential local effects and must use exact durable operation tests; no external provider side effects.

### G5 Verification — PASS
Cryptographic validation authenticates assertion/envelope/anchor bytes but does not create ECR-004 outcome verification truth.

### G6 Durability — PASS
Key lifecycle/protected metadata has explicit crash/migration behavior if persisted; native backend unavailable states are honest.

### G7 Privacy/secrets — PASS
No plaintext fallback, minimal raw-key exposure, redacted errors, protected-at-rest contract.

### G8 Local-first — PASS
No cloud account/provider required.

### G9 Interoperability — PASS
Native stores/protocol standards are adapters; no external identity token becomes Ecra authority.

### G10 Donor/license — PASS_PENDING_DEPENDENCY_LOCK
Primary references recorded; exact dependency/license ledger required before implementation dependency adoption.

### G11 Upstream/browser maintenance — PASS-N/A
No browser patch/bridge.

### G12 Benchmarks — PASS
Acceptance is reproducible security/contract evidence; no superiority claims.

### G13 Information flow / egress — PASS
No remote egress in ECR-031; protected/plaintext data remains local. Later disclosure still requires ECR-003.

### G14 Identity / principal binding — PASS
This slice is the explicit owner; actor/principal/audience/on-behalf-of/trust-root semantics are typed and fail closed.

### G15 Bounded execution — PASS
Strict parser limits, bounded attributes/no arbitrary chain; native calls receive timeout/cancellation treatment where APIs permit; no recursive model/tool loops.

No gate currently fails. G10 must become evidence-backed before implementation dependency lock.

## 16. Complexity tracking

### One new trusted crate
Justified because ECR-001 must remain zero-I/O and ECR-031 requires native keystore/crypto operations. Simpler alternative—putting identity validation and OS I/O into `ecra-core`—violates the closed zero-I/O contract.

### Native backend abstraction
Justified because sensitive persistence needs protected device/user-local key custody and platform semantics differ materially. Simpler alternative—one plaintext file key—violates FR-022/FR-023 and R-053.

### Explicit algorithm agility
Kept narrow to a closed allowlist because Secure Enclave/native backend algorithm capabilities may differ. Simpler hard-coded Ed25519 everywhere risks false hardware-backed claims; generic arbitrary algorithms would be unauditable.

## 17. Implementation authorization gate

Implementation is forbidden until:
- contract/data model/threat model are complete;
- exact dependency candidates are researched enough to make tasks executable;
- requirements checklist passes;
- tasks map every FR/SC and name target paths;
- analyze-equivalent pass reports zero critical planning drift;
- platform roadmap/status/EXECUTION truth is synchronized to `TASKS_READY` on an exact green planning head.
