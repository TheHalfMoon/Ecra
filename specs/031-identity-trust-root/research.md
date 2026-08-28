# ECR-031 Research — Identity, Trust Root & Sensitive Storage Foundations

**Status:** PLANNING_RESEARCH / PASS_1_REMEDIATED / T001_REVIEWED  
**Date:** 2026-08-28  
**Scope:** primary standards/platform documentation and implementation-time dependency review; no donor source copied.

## 1. Research questions

1. How should Ecra validate a principal/actor identity assertion without turning identity into authorization?
2. How does the first local principal/trust root bootstrap without claiming external identity proofing?
3. What local trust-root model is portable enough while preserving platform-specific assurance truth?
4. Which state is authoritative for key rotation/revocation under filesystem rollback/tamper?
5. Who may issue identity assertions before ECR-003 exists?
6. Which authenticated-encryption/signature/KDF primitives fit Ecra's JSON + RFC 8785 canonical direction?
7. How can ECR-031 provide stronger protected authenticity without rewriting ECR-002 digest semantics or counterfeiting ECR-004 verification?
8. What guarantees can macOS Keychain, Windows DPAPI and Linux Secret Service actually support?

## 2. Primary references

### NIST digital identity

- NIST SP 800-63-4, *Digital Identity Guidelines*, final July 2025: https://csrc.nist.gov/pubs/sp/800/63/4/final
- NIST SP 800-63B-4, *Authentication and Authenticator Management*, final July 2025: https://csrc.nist.gov/pubs/sp/800/63/b/4/final
- NIST SP 800-63C-4, *Federation and Assertions*, final July 2025: https://csrc.nist.gov/pubs/sp/800/63/c/4/final

Use: conceptual/security reference for assertion validation, authentication lifecycle, relying-party context and separation of identity evidence from downstream authorization. ECR-031 does **not** claim NIST IAL/AAL/FAL certification or external identity proofing.

### Key management

- NIST SP 800-57 Part 1 Rev. 5, *Recommendation for Key Management: Part 1 — General*: https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final

Use: key purpose, lifecycle, protection, compromise/revocation and trust-anchor planning.

### Cryptographic primitives

- RFC 8439, ChaCha20-Poly1305 AEAD: https://www.rfc-editor.org/info/rfc8439/
- RFC 5869, HKDF: https://www.rfc-editor.org/info/rfc5869/
- RFC 8032, EdDSA/Ed25519: https://www.rfc-editor.org/info/rfc8032/

Use: algorithm semantics/test vectors. Implementation uses exact reviewed Rust dependencies rather than copied reference source.

### Apple

- Apple Platform Security, *Keychain data protection*: https://support.apple.com/guide/security/keychain-data-protection-secb0694df1a/web
- Apple Developer, *Protecting keys with the Secure Enclave*: https://developer.apple.com/documentation/security/protecting-keys-with-the-secure-enclave
- Apple Developer, `SecAccessControlCreateFlags`: https://developer.apple.com/documentation/security/secaccesscontrolcreateflags
- Apple Developer, `kSecUseDataProtectionKeychain`: https://developer.apple.com/documentation/security/ksecusedataprotectionkeychain

Planning facts:
- Keychain is appropriate for small sensitive values/keys and access is mediated by system security services.
- Secure Enclave/private-key operations are algorithm/operation specific; Keychain storage does not imply Secure Enclave signing.
- Data Protection Keychain is preferred for the v1 local-only path; no iCloud synchronization is required.

### Windows

- Microsoft, `CryptUnprotectData`: https://learn.microsoft.com/windows/win32/api/dpapi/nf-dpapi-cryptunprotectdata
- Microsoft DPAPI example/limitations: https://learn.microsoft.com/windows/win32/seccrypto/example-c-program-using-cryptprotectdata

Planning facts:
- DPAPI decrypt performs integrity checking.
- default protection is ordinarily same user credentials + same computer;
- it is not a cross-machine sync/recovery or asymmetric hardware-signing contract.

### Linux desktop Secret Service

- Freedesktop Secret Service API, Version 0.2 DRAFT, published 2026-04-08: https://specifications.freedesktop.org/secret-service/latest/

Planning facts:
- service may require unlocking;
- lookup attributes are explicitly not secret and may be stored unencrypted;
- current upstream page is explicitly **DRAFT**.

## 3. Decision R1 — Identity assertion proves identity context, not authority

`IdentityAssertion` validation yields `ValidatedIdentityContext`. It never yields/embeds `CapabilityGrant`, declassification, approval or authorization lease. ECR-003 remains policy owner.

## 4. Decision R2 — Reuse ECR-001 IDs and references

Reuse `PrincipalId`, `PrincipalRef`, `IdentityAssertionId`, `IdentityAssertionRef`, `ActorId`, and `Actor`. New IDs are limited to trust/key/protected/enrollment/replay concepts.

## 5. Decision R3 — One bounded `ecra-identity` crate

One crate contains pure canonical validation plus explicit I/O traits/native backends/protected trust-state store. Do not split speculative identity/keystore/crypto crates.

`ecra-core` remains zero-I/O; no model/browser/network/protocol/policy dependency enters `ecra-identity`.

## 6. Decision R4 — Explicit validation context; no hidden clock

Pure validation consumes explicit evaluation time, expected actor/audience/principal and replay context. Runtime bootstrap/issuance/encryption obtain clock/randomness through explicit boundaries.

## 7. Decision R5 — Native root/secret backend is fail-closed

Production secret/key custody uses an approved native `TrustBackend` or returns typed unavailable/locked/unsupported failure. No plaintext file, environment, generic DB plaintext, process-memory fallback or production test backend.

## 8. Decision R6 — Platform abstraction carries assurance capabilities

`TrustBackendCapabilities` reports actual tested properties. Common code never upgrades the weakest backend or Keychain storage into a hardware-signing claim.

## 9. Decision R7 — Canonical assertion/trust payloads remain JSON + RFC 8785 JCS

Continue existing strict JSON/JCS direction for signed/authenticated canonical material. COSE/CBOR is not introduced as a competing internal model in ECR-031.

## 10. Decision R8 — Frozen v1 signing suite: portable Ed25519 software key under native custody

**Decision:** canonical ECR-031 v1 assertion and protected-anchor signing algorithm is Ed25519.

Custody:
- generate software Ed25519 seed/key material from production CSPRNG;
- store only as secret material protected by the selected native backend;
- materialize only for bounded signing inside a redacted/zeroizing wrapper;
- public key may appear in validated metadata;
- no ordinary plaintext persistence/logging/export.

Claim boundary:
- this is **not** Secure Enclave signing;
- this is **not** a non-exportable/hardware-backed private-operation guarantee;
- macOS v1 proves Keychain protection of software key material at rest;
- future native non-exportable signing requires a versioned algorithm-suite extension and evidence.

Why: one portable wire/test baseline is safer than silently varying algorithms by platform, while avoiding false hardware claims.

## 11. Decision R9 — Protected envelope: ChaCha20-Poly1305 + HKDF-SHA-256

V1 target:
- native backend protects a high-entropy master secret;
- bounded operation may materialize that secret into redacted/zeroizing memory;
- HKDF-SHA-256 derives domain-separated per-purpose/generation keys;
- ChaCha20-Poly1305 uses 256-bit key, unique 96-bit nonce and full tag;
- interpretation-critical metadata is AAD.

No hardware-non-exportability claim is made for this portable path.

## 12. Decision R10 — Key lifecycle is purpose-scoped and generation-aware

States: Active, RetiredVerifyOrDecryptOnly, Revoked; destruction/unavailability is distinct operational availability.

Only one active generation per trust-root/purpose. New use requires active key; retired/revoked behavior is exact and fail closed.

## 13. Decision R11 — Protected envelope does not authorize downstream sensitive persistence

ECR-031 supplies a primitive. ECR-003/ECR-025 and each state-owning slice still own disclosure/retention/deletion/export authorization.

## 14. Decision R12 — Protected anchor distinct from ordinary digests and outcome verification

`ProtectedAnchor` can key-authenticate a domain-separated digest without changing ECR-002 `LedgerDigest` bytes and without becoming `VerificationReceipt`.

## 15. Decision R13 — Linux lookup attributes are public metadata

Secret Service lookup attributes contain only non-sensitive opaque Ecra identifiers/fixed namespace. Secret bytes remain in the secret value.

## 16. Decision R14 — Recovery/sync out of v1

No cross-device recovery, passphrase export, cloud escrow, team recovery or synchronized keychain contract. Missing/destructively lost key is reported honestly.

## 17. Decision R15 — Bootstrap creates an Ecra-local principal, not externally proofed identity

First bootstrap generates opaque local Principal/TrustRoot/Enrollment IDs. OS username/email/display label is not imported as PrincipalId.

The reachable OS user/session determines native-keystore context only. ECR-031 does not claim external/legal identity proofing or NIST assurance certification.

Bootstrap is complete only after native secret creation + authenticated protected trust state durable publish + authenticated reopen. Orphan native material after crash yields `incomplete_bootstrap`; no silent second principal/root mint.

## 18. Decision R16 — Protected trust state is lifecycle authority

One authenticated `ProtectedTrustStateV1` envelope owns enrollment, key-generation/current-key/retirement/revocation security state. Ordinary DB/files may be rebuildable public metadata only.

Only successful backend open + trust-state AEAD authentication + invariant validation creates `VerifiedTrustSnapshot` for validation/issuance.

V1 does not claim universal monotonic rollback resistance: restoration of an older valid protected state together with equivalent authorized OS keystore state may roll state back. `state_generation` is not marketed as a hardware monotonic counter.

## 19. Decision R17 — Assertion issuance is non-ambient

No generic `issue(principal_id, ...)` exists.

An `EnrolledPrincipalHandle` is obtained only by authenticating/reopening local enrollment/trust state. It plus current `VerifiedTrustSnapshot` creates a process-local non-serializable `IssuerSession` fixed to one principal/root/key.

Caller may request actor/audience binding but cannot substitute subject PrincipalId. V1 cannot mint arbitrary on-behalf-of principals. No IPC/network issuer service is included. Broader delegation authorization remains ECR-003.

## 20. Threat/assurance boundary

Target protection:
- filesystem/database disclosure/tamper where backend key authority is not compromised;
- accidental plaintext persistence;
- unrelated local clients subject to OS protections;
- stale/revoked assertion/key use;
- arbitrary-principal assertion minting;
- metadata substitution and malformed input.

Not generally guaranteed:
- fully compromised same-user account with equivalent keystore authority;
- kernel/hypervisor/debugger compromise;
- malicious firmware/hardware;
- monotonic rollback resistance against restoration of valid protected+keystore state;
- recovery after destructive key loss;
- external identity proofing/federation certification.

## 21. Dependency/source-reuse policy

No upstream source copied by this artifact. Before implementation:
- select exact Rust crypto/native dependencies and versions;
- review license/advisory/features/MSRV/transitive native code;
- retain `#![forbid(unsafe_code)]` in Ecra-authored trusted Rust;
- add RFC/library vectors and negative mutation tests;
- update `research/donor-license-ledger.md` before dependency adoption.

## 22. Planning conclusion

Pass-1 blockers are now resolved in research direction:
- C1 local bootstrap/non-claim -> R15;
- C2 protected authoritative trust state/rollback boundary -> R16;
- C3 non-ambient issuance -> R17;
- C4 portable Ed25519 software signing under native custody -> R8.

Exact dependency selection remains T001 and may stop implementation if current versions/security evidence fail the plan. It is not permission to weaken these decisions.

## 23. T001 implementation-time dependency review — 2026-08-28

T001 was re-run immediately before dependency adoption against current upstream crate manifests/documentation, the repository Rust 1.98.0 toolchain contract, and current RustSec package/advisory records. This section freezes the direct dependency *intent* for T003; the generated `Cargo.lock` remains the exact transitive version authority and must receive its own exact-head CI/dependency evidence before T009/T010 can pass.

### 23.1 Toolchain compatibility

Repository toolchain: Rust/Cargo 1.98.0, edition 2024.

All accepted direct candidates below declare MSRV 1.85, so none requires lowering or changing the repository toolchain:

| Candidate | Exact direct version | Upstream MSRV | License | T001 disposition |
|---|---:|---:|---|---|
| `ed25519-dalek` | `=3.0.0` | 1.85 | BSD-3-Clause | ACCEPT |
| `chacha20poly1305` | `=0.11.0` | 1.85 | Apache-2.0 OR MIT | ACCEPT |
| `hkdf` | `=0.13.0` | 1.85 | MIT OR Apache-2.0 | ACCEPT |
| `sha2` | `=0.11.0` | 1.85 | MIT OR Apache-2.0 | ACCEPT; same version already used by ECR-001/ECR-002 |
| `zeroize` | `=1.9.0` | 1.85 | MIT OR Apache-2.0 | ACCEPT |
| `getrandom` | `=0.4.3` | 1.85 | MIT OR Apache-2.0 | ACCEPT |
| `security-framework` | `=3.7.0` | 1.85 | MIT OR Apache-2.0 | ACCEPT on `cfg(target_os = "macos")` only |

Primary manifests/reference pages reviewed:

- https://docs.rs/crate/ed25519-dalek/3.0.0/source/Cargo.toml
- https://docs.rs/crate/chacha20poly1305/0.11.0/source/Cargo.toml
- https://docs.rs/crate/hkdf/0.13.0/source/Cargo.toml
- https://docs.rs/crate/sha2/0.11.0/source/Cargo.toml
- https://docs.rs/crate/zeroize/1.9.0
- https://docs.rs/crate/getrandom/0.4.3/source/Cargo.toml.orig
- https://docs.rs/crate/security-framework/3.7.0/source/Cargo.toml

### 23.2 Minimal feature lock

T003 should adopt only these direct feature shapes unless implementation proves a documented necessity and this review is amended first:

```toml
ed25519-dalek = { version = "=3.0.0", default-features = false, features = ["fast", "zeroize"] }
chacha20poly1305 = { version = "=0.11.0", default-features = false, features = ["alloc", "zeroize"] }
hkdf = { version = "=0.13.0", default-features = false }
sha2 = { version = "=0.11.0", default-features = false }
zeroize = { version = "=1.9.0", default-features = false, features = ["alloc"] }
getrandom = { version = "=0.4.3", default-features = false }

[target.'cfg(target_os = "macos")'.dependencies]
security-framework = { version = "=3.7.0", default-features = false, features = ["OSX_10_15"] }
```

Reasons:

- `ed25519-dalek`: keep `zeroize`; retain `fast` precomputed tables; do not enable `rand_core`, `batch`, `serde`, `pem`, `pkcs8`, `hazmat` or legacy compatibility. Ecra owns seed generation through `SecureRandom` and stores its own strict wire form.
- `chacha20poly1305`: disable its default `getrandom` feature so AEAD does not own hidden randomness; Ecra supplies nonces through its explicit randomness boundary. Keep only allocation and zeroization support needed by the planned envelope implementation.
- `hkdf`: 0.13.0 has no default features. Do not enable the optional `kdf` trait dependency unless an implementation-time need is separately justified.
- `sha2`: use SHA-256 only through the existing RustCrypto implementation; no competing hash package or copied implementation.
- `zeroize`: enable allocation support for the planned `SensitiveBytes` container; do not enable derive unless a later implementation task demonstrates a bounded need and re-reviews `zeroize_derive`.
- `getrandom`: use the direct fallible system RNG API; do not enable `wasm_js` or `sys_rng`, and do not add `rand` merely to generate ECR-031 secrets/IDs/nonces.
- `security-framework`: disable its default Secure Transport/ALPN/session-ticket feature set. Enable only `OSX_10_15`, which exposes the Data Protection Keychain location used by the frozen macOS v1 acceptance path. This dependency is macOS-target-only.

### 23.3 Native / unsafe boundary

Ecra-authored trusted Rust remains `#![forbid(unsafe_code)]`.

Accepted dependencies are not being claimed universally unsafe-free. In particular:

- `security-framework` and `security-framework-sys` wrap Apple's C Security.framework/CoreFoundation APIs and therefore form an explicit reviewed native/FFI boundary outside Ecra-authored Rust;
- `getrandom` uses platform OS entropy APIs and target-specific system dependencies; this is accepted solely for the explicit CSPRNG boundary and does **not** create or verify a Windows/Linux `TrustBackend`;
- cryptographic crates may contain dependency-owned low-level/architecture-specific implementation code; T004 records the boundary and forbids Ecra-authored unsafe rather than making a false whole-dependency-graph `unsafe` claim.

No Windows DPAPI crate and no Linux Secret Service crate is accepted in Phase 1. Their appearance in planning research does not authorize dependency adoption. Windows/Linux native trust backends remain unimplemented/unverified.

### 23.4 Advisory review

RustSec package/advisory records were checked immediately before adoption intent was recorded.

- `ed25519-dalek`: RUSTSEC-2022-0093 is patched in `>=2`; candidate 3.0.0 is outside the affected range. The optional `hazmat` feature remains disabled.
- `curve25519-dalek`: the historical timing advisory RUSTSEC-2024-0344 is patched in `>=4.1.3`; `ed25519-dalek 3.0.0` requires curve25519-dalek 5.0.0.
- `sha2`: RUSTSEC-2021-0100 affected 0.9.7 and is patched in `>=0.9.8`; candidate 0.11.0 is outside the affected range.
- `chacha20`: RUSTSEC-2019-0029 is patched in `>=0.2.3`, and the advisory explicitly states `chacha20poly1305` is unaffected by that counter-overflow issue; candidate `chacha20poly1305 0.11.0` uses modern `chacha20 0.10`.
- `zeroize_derive`: RUSTSEC-2021-0115 concerns old derive behavior; ECR-031 does not enable the derive feature in the T001 feature lock.
- `security-framework`: the historical RUSTSEC-2017-0003 issue is patched in `>=0.1.12`; candidate 3.7.0 is outside the affected range. ECR-031 also disables Secure Transport-oriented default features and uses the crate only for Keychain integration.

No direct candidate was found to be in an affected range during this implementation-time review. This is not a permanent security guarantee: after `Cargo.lock` is generated, T008/T009 dependency evidence must review the exact resolved graph, and future lockfile/feature changes require a fresh advisory disposition.

RustSec references:

- https://rustsec.org/packages/ed25519-dalek.html
- https://rustsec.org/advisories/RUSTSEC-2022-0093.html
- https://rustsec.org/advisories/RUSTSEC-2024-0344.html
- https://rustsec.org/advisories/RUSTSEC-2021-0100.html
- https://rustsec.org/advisories/RUSTSEC-2019-0029.html
- https://rustsec.org/packages/zeroize_derive.html
- https://rustsec.org/packages/security-framework.html

### 23.5 Rejected / deferred candidates

Rejected for this v1 dependency boundary:

- `rand`/`rand_chacha` as an ambient production RNG layer — unnecessary; `getrandom` is the explicit system entropy boundary;
- `ring`, OpenSSL, libsodium, platform TLS stacks or general crypto suites — broader capability/native surface than the frozen algorithms require;
- `ed25519-dalek` serialization/PKCS#8/PEM/batch/hazmat/legacy features — not required by the ECR-031 wire or custody contract;
- `chacha20poly1305` default randomness feature — conflicts with explicit nonce/randomness ownership;
- `security-framework` default Secure Transport/ALPN/session-ticket features — unrelated to Keychain custody;
- Secure Enclave signing APIs as the v1 Ed25519 path — contradict the frozen portable software-Ed25519 custody decision;
- Windows DPAPI and Linux Secret Service crates — deferred until their corresponding native backend is actually implemented and can produce platform-specific evidence.

### 23.6 T001 conclusion

T001 dependency research is complete enough to authorize T002 and the bounded T003 manifest delta above. It does **not** authorize marking T003/T009/T010 complete until the exact generated lockfile, dependency tree, native boundary, CI and license/advisory evidence are committed and pass on the exact branch head.