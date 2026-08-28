# ECR-031 Research — Identity, Trust Root & Sensitive Storage Foundations

**Status:** PLANNING_RESEARCH  
**Date:** 2026-08-28  
**Scope:** primary standards/platform documentation and repository architecture decisions only; no donor source copied.

## 1. Research questions

1. How should Ecra validate a principal/actor identity assertion without turning identity into authorization?
2. What local trust-root model is portable enough for Ecra while preserving platform-specific assurance truth?
3. How should keys rotate/retire/revoke without making old protected state unreadable or old assertions silently valid forever?
4. Which authenticated-encryption/signature/KDF primitives fit Ecra's existing JSON + RFC 8785 canonical contract direction?
5. How can ECR-031 provide stronger protected authenticity for future consumers without rewriting ECR-002 digest semantics or counterfeiting ECR-004 verification?
6. What guarantees can macOS Keychain/Secure Enclave, Windows DPAPI and Linux Secret Service actually support?

## 2. Primary references

### NIST digital identity

- NIST SP 800-63-4, *Digital Identity Guidelines*, final July 2025: https://csrc.nist.gov/pubs/sp/800/63/4/final
- NIST SP 800-63B-4, *Authentication and Authenticator Management*, final July 2025: https://csrc.nist.gov/pubs/sp/800/63/b/4/final
- NIST SP 800-63C-4, *Federation and Assertions*, final July 2025: https://csrc.nist.gov/pubs/sp/800/63/c/4/final

Use: conceptual/security reference for assertion validation, authentication lifecycle, audience/relying-party context and separation of authentication from downstream authorization. Ecra is not claiming NIST assurance-level certification in ECR-031.

### Key management

- NIST SP 800-57 Part 1 Rev. 5, *Recommendation for Key Management: Part 1 — General*: https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final

Use: key purpose, lifecycle, protection, compromise/revocation and trust-anchor planning reference.

### Cryptographic primitives

- RFC 8439, ChaCha20-Poly1305 AEAD: https://www.rfc-editor.org/info/rfc8439/
- RFC 5869, HKDF: https://www.rfc-editor.org/info/rfc5869/
- RFC 8032, EdDSA/Ed25519: https://www.rfc-editor.org/info/rfc8032/

Use: stable algorithm semantics/test-vector references. These RFCs are conceptual/protocol references; implementation will use exact reviewed Rust dependencies rather than copied reference source.

### Apple

- Apple Platform Security, *Keychain data protection*: https://support.apple.com/guide/security/keychain-data-protection-secb0694df1a/web
- Apple Developer, *Protecting keys with the Secure Enclave*: https://developer.apple.com/documentation/security/protecting-keys-with-the-secure-enclave
- Apple Developer, `SecAccessControlCreateFlags`: https://developer.apple.com/documentation/security/secaccesscontrolcreateflags
- Apple Developer, `kSecUseDataProtectionKeychain`: https://developer.apple.com/documentation/security/ksecusedataprotectionkeychain

Observed planning facts:
- Keychain is intended for small sensitive values/keys and mediates access through system security services.
- Keychain access control can require user presence/biometric/passcode conditions.
- Secure Enclave-backed private-key operations can avoid exposing plaintext private key material to the application.
- Hardware/user-presence behavior is operation/platform dependent; Ecra must expose capability truth rather than label all macOS keys hardware-backed.
- Data Protection Keychain should be preferred for new macOS items unless a specific legacy requirement exists; ECR-031 has no iCloud synchronization requirement.

### Windows

- Microsoft, `CryptUnprotectData`: https://learn.microsoft.com/windows/win32/api/dpapi/nf-dpapi-cryptunprotectdata
- Microsoft, DPAPI example/limitations: https://learn.microsoft.com/windows/win32/seccrypto/example-c-program-using-cryptprotectdata

Observed planning facts:
- DPAPI decrypt performs integrity checking.
- Default protection is ordinarily same user credentials + same computer.
- Default DPAPI is not a cross-machine or multi-user secret-sync contract.
- Administrative password reset/recovery conditions can affect recoverability; ECR-031 must not promise universal recovery.

### Linux desktop Secret Service

- Freedesktop Secret Service API, Version 0.2 DRAFT, published 2026-04-08: https://specifications.freedesktop.org/secret-service/latest/

Observed planning facts:
- The API stores secrets through a service in the user's login session and may require unlocking.
- Lookup attributes are explicitly not secret and may be stored unencrypted.
- Sessions bind secret transfer to the client connection.
- The current upstream page is explicitly a **DRAFT**, so Ecra must not market it as a final stable security standard.

## 3. Decision R1 — Identity assertion proves identity context, not authority

**Decision:** `IdentityAssertion` validation yields `ValidatedIdentityContext`. It never yields or embeds `CapabilityGrant`, declassification, approval or authorization lease.

Rationale:
- repository constitution separates Actor attribution, authenticated Principal and authorization;
- NIST assertion/federation guidance treats assertions as authentication/attribute evidence for a relying party, not as Ecra's capability model;
- ECR-003 owns authorization/policy.

Rejected:
- `ActorId` as authenticated identity;
- protocol token string as Principal identity;
- signed assertion containing generic Ecra capabilities.

## 4. Decision R2 — Reuse ECR-001 IDs and references

**Decision:** ECR-031 must reuse `PrincipalId`, `PrincipalRef`, `IdentityAssertionId`, `IdentityAssertionRef`, `ActorId`, and `Actor` from `ecra-core`.

New IDs are limited to concepts not yet represented, e.g. `TrustRootId`, `KeyId`, `ProtectedObjectId`, and replay/assertion nonce identifiers if required.

Rationale: G1 and D-035 prohibit a second identity model.

## 5. Decision R3 — One bounded `ecra-identity` crate candidate

**Decision:** plan one crate for the current slice, with pure canonical types/validation plus explicit I/O traits and `cfg`-bounded native backends. Do not split into speculative `identity-core`, `keystore`, `crypto`, `platform` crates without implementation evidence.

Constraints:
- `ecra-core` remains zero-I/O/provider independent;
- Ecra-authored code forbids unsafe;
- native FFI is behind reviewed dependencies/platform adapter modules;
- no model/browser/network/protocol/policy dependency.

Revisit trigger: exact dependency/native build constraints make one crate materially harder to audit or cross-compile safely.

## 6. Decision R4 — Explicit validation context; no hidden clock

**Decision:** pure validation accepts a `ValidationContext` including evaluation time, expected audience/use, actor binding and replay context/reference. Canonical validation does not call an OS clock or random source.

Rationale: deterministic tests/replay, auditability and separation from runtime environment.

Runtime services may obtain clock/random values through explicit injected boundaries before constructing the validation/issuance request.

## 7. Decision R5 — Local root-key backend is fail-closed

**Decision:** production root/private key operations use a `TrustBackend` implementation backed by an approved native protected store or return a typed unavailable/locked/unsupported error.

There is **no** production fallback to:
- plaintext key files;
- environment variables;
- generic SQLite plaintext;
- an automatically generated process-memory root key;
- a test backend.

Test-only memory backends must be compile/configuration-separated from production selection.

## 8. Decision R6 — Platform abstraction carries assurance capabilities

**Decision:** `TrustBackendCapabilities` describes what the backend can actually support, for example:

```text
hardware_backed_private_operations
user_presence_gate
biometric_gate
user_scoped
machine_bound
non_exportable_private_key
locked_state
synchronizing_store
```

The common API must not claim the strongest platform guarantee for all backends.

Initial truth:
- macOS may support Secure Enclave/user-presence for compatible private-key operations;
- Windows default DPAPI is user+machine scoped but is not inherently a non-exportable asymmetric-key hardware root;
- Linux Secret Service is service/implementation dependent and lookup attributes are non-secret.

## 9. Decision R7 — Canonical assertion payload remains JSON + RFC 8785 JCS

**Decision:** continue the repository's existing strict JSON/JCS canonicalization direction for signed identity payloads.

Rationale:
- cross-language inspectability;
- ECR-001 already has canonical JSON/JCS machinery and compatibility discipline;
- adopting COSE/CBOR internally now would create a second canonicalization/versioning surface without demonstrated need.

COSE may remain a later protocol-adapter reference; it is not ECR-031's internal authority model.

## 10. Decision R8 — Signature suite candidate: Ed25519

**Decision candidate to freeze in plan/contracts:** Ed25519 for software/local assertion and protected-anchor signing where the selected native backend can securely own/use compatible key material.

Rationale:
- stable RFC 8032 reference/test vectors;
- compact deterministic signatures;
- strong Rust ecosystem support.

Important platform constraint:
- Apple Secure Enclave exposes supported elliptic-curve operations but does not imply Ed25519 support. Therefore v1 MUST NOT fake one uniform hardware-backed Ed25519 key across every backend.
- If platform-native non-exportable signing requires a different algorithm, the contract must represent algorithm suites explicitly and keep assertion verification algorithm-agile within a tight allowlist.

**Planning conclusion:** do not hard-code `Ed25519Only` into the trust-backend abstraction until native-backend feasibility is confirmed. Use an explicit versioned `SignatureAlgorithm` allowlist and prefer Ed25519 for software-wrapped/local portable test vectors.

## 11. Decision R9 — Protected envelope candidate: ChaCha20-Poly1305 + HKDF-SHA-256

**Decision candidate:**
- root/native backend protects a high-entropy Ecra master wrapping/derivation key or performs equivalent protected operation;
- HKDF-SHA-256 derives domain-separated per-purpose/per-generation envelope keys where key derivation is used;
- ChaCha20-Poly1305 uses a 256-bit key, unique 96-bit nonce and full tag as specified by RFC 8439;
- all interpretation-critical metadata is authenticated as AAD;
- random nonces come from an explicit CSPRNG boundary.

Nonce reuse with one key is a protocol violation. Tests must include deterministic vectors by injecting fixed test randomness; production ciphertext is intentionally nondeterministic.

Revisit before implementation dependency lock if a platform-native AEAD/wrapping primitive yields a simpler stronger boundary without leaking raw key material.

## 12. Decision R10 — Key lifecycle is purpose-scoped and generation-aware

Model at least:

```text
Active
RetiredVerifyOrDecryptOnly
Revoked
DestroyedOrUnavailable (only if semantics can be represented safely)
```

Rules:
- only one current active generation per trust-root/purpose in v1;
- new signing/encryption uses active key only;
- retired key use is explicit and limited to compatibility reads/verification where allowed;
- revoked key cannot issue/sign/protect new material and validation behavior is fail-closed according to contract;
- deletion/destruction and revocation are distinct concepts;
- a missing old key produces typed unavailable state, not fabricated corruption/success.

## 13. Decision R11 — Protected envelope is not blanket authorization for sensitive persistence

ECR-031 provides a cryptographic storage primitive and backend contract. It does not by itself authorize every slice to persist real private/secret data.

Later owners still require:
- ECR-003 source-to-sink/secret-use policy;
- ECR-025 privacy/retention/redaction;
- their own migration/deletion/export contracts.

## 14. Decision R12 — Protected anchor is separate from ordinary digests and verification

ECR-031 may define `ProtectedAnchor`/signature input over a domain-separated digest/payload. This can later allow a consumer such as ECR-002 to distinguish:

```text
plain LedgerDigest integrity chain
vs
protected key-backed authenticity anchor
```

It must not:
- change existing ECR-002 digest bytes;
- call a protected anchor a `VerificationReceipt`;
- claim action outcome correctness;
- imply defense after the protected root itself is compromised.

## 15. Decision R13 — Linux lookup attributes are public metadata

Because the current Secret Service spec explicitly says attributes are not secret and may be stored unencrypted, Ecra's Linux backend must put only non-sensitive opaque identifiers/classification-safe labels in attributes. Secret bytes, plaintext principal assertions, key material, tokens, sensitive object names or private workspace content are forbidden in lookup attributes.

## 16. Decision R14 — Recovery/sync is out of v1

Cross-device recovery, passphrase export, cloud escrow, team recovery and synchronized keychains are outside ECR-031 v1. They create separate threat/identity/availability questions owned by later slices.

Failure to recover an unavailable/destroyed local key must be reported honestly. ECR-031 does not invent a weaker recovery channel.

## 17. Threat/assurance boundary

General guarantee target:
- protect against ordinary filesystem/database disclosure, accidental plaintext persistence, unrelated/unprivileged local clients where OS protections apply, metadata/ciphertext tampering and stale/revoked assertion use;
- preserve cryptographic distinction between plain digests and protected authenticity.

Not generally guaranteed:
- fully compromised same-user account with equivalent keystore authorization;
- compromised kernel/hypervisor/debugger;
- malicious firmware/hardware;
- availability after destructive credential/key loss;
- remote identity proofing/federation assurance certification.

Hardware/user-presence guarantees are backend-specific and require evidence.

## 18. Dependency/source-reuse policy

No upstream implementation source is copied by this research artifact. Before implementation:
- select exact Rust crypto dependencies and versions;
- review licenses/advisories/features/transitive native code;
- record native backend dependencies and why they are required;
- preserve `#![forbid(unsafe_code)]` in Ecra-authored trusted Rust;
- add RFC/library test vectors and negative mutation tests;
- update `research/donor-license-ledger.md` before dependency adoption.

## 19. Open research resolved for planning

The roadmap's identity/key-lifecycle and protected-storage gaps are sufficiently bounded to proceed to a concrete contract/plan. Remaining implementation-specific questions—exact macOS signing algorithm support, Windows native binding crate/API, Linux DBus library choice—are dependency-selection tasks, not reasons to weaken the spec.

If dependency research cannot meet fail-closed or auditability requirements, implementation must stop and append a planning clarification rather than introduce a plaintext fallback.
