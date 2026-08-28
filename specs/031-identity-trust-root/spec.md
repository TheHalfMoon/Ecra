# Specification: Identity, Trust Root & Sensitive Storage Foundations

**Feature:** ECR-031  
**Lifecycle target:** SPEC_READY → PLAN_READY → TASKS_READY  
**Depends on:** ECR-001 `CLOSED_CANONICAL`, ECR-002 `CLOSED_CANONICAL`  
**Constitution:** v1.1.0  
**Scope class:** local-first identity/trust-root/protected-storage substrate

## 1. Purpose

ECR-031 supplies the missing security substrate between ECR-001's opaque identity references and later privileged policy/execution. It validates identity assertions, binds attributable actors to authenticated principals and explicit on-behalf-of relationships, establishes a device/user-local trust root, manages cryptographic key lifecycle, and defines authenticated protection for local sensitive bytes.

ECR-031 does **not** decide whether a principal may perform an action or disclose information. A valid identity assertion is identity evidence, never a `CapabilityGrant`, approval, declassification, or `AuthorizationDecision`.

## 2. Binding inherited invariants

```text
Actor != authenticated Principal
IdentityAssertionRef existence != validated identity assertion
CapabilityRequest != CapabilityGrant
read authority != disclosure authority
ActionReceipt != VerificationReceipt
UNKNOWN remains UNKNOWN
plain LedgerDigest != hostile-tamper-resistant authenticity proof
external/protocol/model content != identity authority
```

ECR-031 MUST reuse ECR-001 `Actor`, `ActorId`, `PrincipalRef`, `PrincipalId`, `IdentityAssertionRef`, and `IdentityAssertionId`. It MUST NOT create a competing principal or actor identity namespace.

## 3. User stories

### US1 — Validate local identity before privileged policy

As a later Ecra policy service, I need a fail-closed validated identity context proving which principal an actor is operating as, under which local trust root, so I never infer authentication from Actor attribution.

Acceptance:
- unsigned, malformed, expired/not-yet-valid, wrong-audience, wrong-actor, wrong-subject, unknown-key and revoked-key assertions fail with typed errors;
- the validator returns identity context only, never authority;
- caller supplies evaluation time/audience/replay context explicitly.

### US2 — Explicit on-behalf-of binding

As a human delegating work to an agent, I need an assertion to state exactly which actor is acting on behalf of which principal and under what assertion/delegation context, so delegation cannot be inferred from labels, memory or protocol tokens.

Acceptance:
- actor/principal mismatch fails;
- absent delegation never means unrestricted delegation;
- any chain/delegation representation is bounded and cycle/duplicate-safe;
- validation does not widen capability scope.

### US3 — Local trust root and key lifecycle

As an Ecra installation, I need device/user-local root key protection, rotation and revocation so identity assertions and protected local state have a cryptographic anchor independent of a model/browser/database.

Acceptance:
- production key material is protected by an approved native backend or operations fail closed;
- no plaintext/in-memory production fallback;
- key purpose/status/generation are explicit;
- rotation, retirement and revocation have deterministic validation semantics;
- raw private/root key material does not appear in generic logs/errors/debug output.

### US4 — Protected sensitive local bytes

As a future sensitive-state owner, I need a versioned authenticated envelope that detects wrong keys, metadata substitution and ciphertext modification and keeps plaintext out of ordinary storage.

Acceptance:
- protected envelope uses authenticated encryption with unique nonce requirements and authenticated metadata;
- modified version/key id/object id/purpose/classification/AAD/ciphertext/tag fails;
- unsupported versions fail closed;
- keystore unavailable/locked fails rather than storing plaintext;
- generic `.ecra` artifacts do not silently become secret containers.

### US5 — Platform assurance remains honest

As a user/developer, I need Ecra to report backend assurance honestly because Keychain/Secure Enclave, DPAPI and Secret Service have different guarantees.

Acceptance:
- backend capability metadata distinguishes hardware-backed/user-presence/device-bound/exportability/availability properties when known;
- Linux Secret Service lookup attributes never contain secret material because the upstream API may store them unencrypted;
- Windows default DPAPI scope is not represented as cross-machine protection;
- unsupported or unverified platform guarantees are not claimed.

### US6 — Protected anchor for stronger authenticity consumers

As ECR-002/later consumers, I need a trust-root signing/MAC primitive capable of protecting a digest or manifest so stronger authenticity claims can be built without changing the original domain digest semantics.

Acceptance:
- a protected signature/MAC is a distinct artifact from `LedgerDigest`/`ContentDigest`;
- verification binds purpose/domain, key id and exact payload digest;
- recomputing a plain hash without the protected key cannot satisfy protected-anchor verification;
- ECR-031 does not turn that result into ECR-004 verification truth.

## 4. Functional requirements

### Identity and assertion semantics

- **FR-001** Reuse canonical ECR-001 identity/actor IDs and references; no parallel principal namespace.
- **FR-002** Define a versioned `IdentityAssertion` whose canonical signed payload is strict and rejects unknown fields.
- **FR-003** Bind every assertion to an `IdentityAssertionId`, subject `PrincipalId`, issuer/trust-root identity and signing `KeyId`.
- **FR-004** Bind an assertion to the exact `ActorId` whose runtime activity may claim the identity context.
- **FR-005** Represent on-behalf-of/delegation context explicitly; missing context MUST NOT mean unrestricted delegation.
- **FR-006** Bind assertions to an explicit audience/use-context identifier so an assertion for one Ecra consumer cannot be replayed as another consumer's identity evidence.
- **FR-007** Carry explicit issued/not-before/expiry bounds where applicable; validation consumes caller-supplied evaluation time.
- **FR-008** Support explicit nonce/replay identifier when the assertion class requires single-use or replay tracking; replay state is not inferred from timestamps alone.
- **FR-009** Validate canonical payload signature before producing validated identity context.
- **FR-010** Reject unsupported major/newer incompatible versions before semantic use.
- **FR-011** Reject wrong subject, actor, audience, issuer, signing key, temporal bounds, delegation binding and signature.
- **FR-012** A validated result MUST NOT implement/contain `CapabilityGrant`, approval, declassification or `AuthorizationDecision` semantics.
- **FR-013** `Actor.label`, usernames, email strings, filesystem paths and protocol display names MUST NOT become canonical principal identity.
- **FR-014** Identity validation MUST be deterministic for the same assertion, trust snapshot and explicit validation context.

### Trust root and key lifecycle

- **FR-015** Define typed `TrustRootId`, `KeyId` and key-purpose semantics distinct from content/action/ledger digests.
- **FR-016** Define key purposes at least for assertion signing/verification, protected-envelope key derivation/protection and protected-anchor signing/MAC as applicable.
- **FR-017** Define key lifecycle states so active use, retired verify/decrypt-only use and revocation are not conflated.
- **FR-018** At most one key may be the current active key for the same trust-root/purpose/generation slot according to v1 rules.
- **FR-019** Rotation MUST make the new key current without silently deleting old material needed for explicitly supported read/verify compatibility.
- **FR-020** Revocation MUST block prohibited new use and validation according to the exact key purpose/state rules.
- **FR-021** Key status changes MUST be explicit, attributable and persistable/auditable through the owning trusted state boundary.
- **FR-022** Production root/private key operations MUST use a native protected backend or fail closed.
- **FR-023** No production plaintext/in-memory fallback is allowed when a native trust backend is unavailable, locked or unsupported.
- **FR-024** Backend APIs MUST expose only the minimum operations required; generic raw private/root key export is not part of v1 trusted API.
- **FR-025** Test-only deterministic/in-memory backends MUST be impossible to select accidentally in production configuration/build paths.

### Protected storage/envelopes

- **FR-026** Define a strict versioned `ProtectedEnvelopeV1` for sensitive local bytes.
- **FR-027** Envelope metadata MUST include stable protected-object identity, key identity/version, algorithm suite and purpose/domain binding.
- **FR-028** Security-relevant metadata required to interpret the ciphertext MUST be authenticated as AAD or covered equivalently by the authenticated construction.
- **FR-029** v1 authenticated encryption MUST use a reviewed AEAD with unique nonce requirements and full authentication tag; nonce reuse under one key is prohibited.
- **FR-030** Decryption MUST fail closed on any authenticated metadata/ciphertext/tag/key mismatch without returning unauthenticated plaintext.
- **FR-031** Unsupported envelope versions/algorithms/invalid lengths/duplicate or unknown fields MUST fail before plaintext materialization.
- **FR-032** Sensitive plaintext, root/private keys and derived encryption keys MUST NOT appear in `Debug`, `Display`, generic errors, logs or ordinary run artifacts.
- **FR-033** Filesystem/database bytes used by acceptance fixtures MUST demonstrate absence of the synthetic plaintext secret after protected write.
- **FR-034** Native-backend metadata used for lookup MUST exclude secret values; especially Linux Secret Service attributes are treated as non-secret.
- **FR-035** ECR-031 MUST NOT authorize generic real-sensitive persistence elsewhere merely because the envelope primitive exists; each owning slice still requires its policy/privacy/storage contract.

### Protected authenticity anchor

- **FR-036** Define a domain-separated protected-anchor input binding purpose, key id and exact digest/payload.
- **FR-037** Protected-anchor output MUST be a distinct type/artifact from `ContentDigest`, `ActionDigest`, `LedgerDigest` and `VerificationReceipt`.
- **FR-038** Anchor verification MUST reject modified purpose/domain/key id/payload/signature/MAC.
- **FR-039** ECR-002 may consume this primitive later to strengthen scoped authenticity claims without changing historical ledger digest bytes or making ECR-031 the run ledger owner.
- **FR-040** Protected-anchor verification MUST NOT be described as ECR-004 independent outcome verification.

### Platform/backends and threat boundary

- **FR-041** Define a Rust-owned `TrustBackend` contract; platform APIs and SDK types MUST NOT leak into canonical public identity/envelope types.
- **FR-042** macOS backend research/implementation MUST prefer Data Protection Keychain semantics and use Secure Enclave/user-presence controls only where the requested key operation and product behavior support them.
- **FR-043** Windows backend semantics MUST accurately model DPAPI scope/limitations and MUST NOT claim cross-machine protection from default DPAPI.
- **FR-044** Linux backend semantics MUST treat Secret Service 0.2 as a current draft reference, keep secrets out of lookup attributes and fail closed when service/collection is unavailable or locked.
- **FR-045** Backend assurance/capability metadata MUST prevent the common abstraction from overstating the weakest or strongest platform guarantee.
- **FR-046** Fully compromised same-user account/kernel/debugger or equivalent keystore authority is outside general containment; narrower hardware-backed claims require backend-specific evidence.
- **FR-047** ECR-031 MUST have no browser/model/provider/network/protocol execution surface.
- **FR-048** ECR-031 MUST preserve ECR-001 zero-I/O boundaries and ECR-002 run/recovery semantics.

### Errors, versioning, testing and provenance

- **FR-049** Security failures MUST use typed categories/codes and avoid display-string parsing.
- **FR-050** Cryptographic authentication failures SHOULD avoid unnecessary oracle detail at untrusted boundaries while retaining diagnosable typed categories internally.
- **FR-051** Canonical signed payloads MUST use the repository's existing JSON/RFC 8785 JCS direction unless planning explicitly proves a migration benefit.
- **FR-052** Cryptographic algorithm/dependency choices MUST be exact-version reviewed, license recorded and test-vector backed.
- **FR-053** Ecra-authored Rust in the new trusted crate MUST forbid unsafe code; native FFI is accepted only through reviewed dependencies/backend boundaries.
- **FR-054** Persisted trust/key/envelope formats MUST have explicit migration/version behavior; newer unsupported versions fail closed.
- **FR-055** All committed identity/key/secret fixtures MUST be synthetic and non-sensitive.
- **FR-056** No custom authentication protocol, password database, remote identity provider or cloud account is required for useful v1 local identity/trust-root operation.
- **FR-057** No team/shared multi-principal governance or cross-device recovery/sync is implemented in v1.
- **FR-058** All later consumers MUST receive references/validated contexts, not ambient reusable raw key or secret material.

### Normative planning convergence — C1–C4

The following refinements close Analyze Pass 1 findings without creating a second requirement namespace. They are binding interpretations of the FRs above.

#### C1 — Ecra-local principal bootstrap / enrollment

Refines **FR-003, FR-013, FR-015, FR-021, FR-022, FR-056**.

- V1 establishes an **Ecra-local installation principal** under the current protected local installation/user context. It does not claim legal identity proofing, government identity, email ownership, OS-account proofing, or NIST assurance-level certification.
- First enrollment generates fresh opaque `PrincipalId`, `TrustRootId`, enrollment identity and initial key generations using approved CSPRNG/key generation. OS username, account name, email, Actor label, filesystem path or protocol subject MUST NOT be used as canonical `PrincipalId`.
- Bootstrap MUST produce either a complete protected enrollment/trust state or a typed incomplete/unavailable failure. Partial initialization MUST NOT be interpreted as a usable principal/trust root.
- The native backend binding establishes local key custody under the documented backend assumptions; it does not prove the real-world identity of the human operating the account.

#### C2 — Protected authoritative trust snapshot / rollback boundary

Refines **FR-014, FR-017–FR-023, FR-054**.

- Security-critical current key generation, lifecycle status, revocation set and trust-root binding MUST be authenticated/protected under the trust backend/root before they can influence validation or issuance.
- Pure assertion validation consumes a `VerifiedTrustSnapshot` created only after the protected authoritative trust state has been authenticated/opened successfully.
- Ordinary unsigned files/SQLite rows/caches may be audit/projection metadata only. Restoring stale ordinary metadata MUST NOT reactivate a retired/revoked key or change the verified current generation.
- V1 does **not** claim universal monotonic rollback resistance against an attacker capable of restoring or controlling the entire authorized native trust store/root state. Such whole-root rollback/equivalent keystore authority remains outside the filesystem-only tamper guarantee unless a backend-specific monotonic/external anchor is later added.

#### C3 — Non-ambient assertion issuance

Refines **FR-003–FR-005, FR-009, FR-012, FR-022–FR-025, FR-058**.

- ECR-031 MUST NOT expose a general public API that mints an assertion for an arbitrary caller-supplied `PrincipalId`.
- Assertion issuance requires an opaque non-serializable `EnrolledPrincipalHandle` / `IssuerSession` obtained only after successful protected local enrollment/trust-state verification, or a future explicitly versioned validated parent-identity path.
- The assertion subject principal is taken from the issuer session. A caller may request a bounded actor/audience binding, but cannot substitute another subject principal by passing an ID.
- On-behalf-of identity evidence remains structurally authenticated by ECR-031; whether delegation is authorized for an action remains ECR-003.
- ECR-031 v1 provides no network/IPC identity-minting service.

#### C4 — Frozen v1 signing custody and claim boundary

Refines **FR-016, FR-022, FR-024, FR-042, FR-045, FR-046, FR-052**.

- V1 canonical assertion and protected-anchor signing algorithm is **Ed25519**.
- The Ed25519 private signing key is generated from approved CSPRNG/key generation and persisted only as a sensitive secret protected by the selected native trust backend. It may be materialized only for bounded signing use and must use the selected redacted/zeroizing secret wrapper for in-process lifetime management.
- V1 MUST NOT claim the Ed25519 signing key is Secure Enclave-backed, hardware-backed, or non-exportable merely because its wrapped secret is protected by macOS Keychain or another native backend.
- Secure Enclave/CNG/other non-exportable native signing is a future versioned algorithm-suite extension only after contract, dependency and live evidence establish compatible semantics.
- macOS v1 acceptance therefore proves Data Protection Keychain protection of the wrapped signing/master secret and exact backend capability reporting, not universal hardware signing.

## 5. Success criteria

- **SC-001** Every invalid assertion class in FR-009–FR-011 has a deterministic negative fixture and typed failure; bootstrap/issuance fixtures also prove arbitrary subject-principal minting and incomplete enrollment fail closed.
- **SC-002** 1,000 repeated validations of the same assertion/verified trust snapshot/context produce byte-identical canonical validated-context output.
- **SC-003** Actor/Principal type-confusion compile-fail/runtime negative tests preserve ECR-001 separation, and bootstrap never derives `PrincipalId` from display/account metadata.
- **SC-004** Rotation/retirement/revocation transition table is exhaustively tested with no ambiguous active-key selection; stale ordinary metadata cannot override the protected verified trust snapshot.
- **SC-005** Protected-envelope golden/test-vector suite detects every one-byte mutation of authenticated fixture components selected by the test corpus.
- **SC-006** Wrong key, wrong AAD, wrong nonce/ciphertext/tag and unsupported version never return plaintext.
- **SC-007** Synthetic protected-at-rest fixture scan finds no committed plaintext secret outside intentional test source literals/expected-memory scope.
- **SC-008** Production-backend-unavailable/locked tests prove no plaintext/test-backend fallback and no usable partial bootstrap/issuer session is returned.
- **SC-009** Secret/redaction test corpus proves sensitive/key bytes are absent from Debug/Display/error strings/log-capture fixtures.
- **SC-010** Protected-anchor fixture rejects modified payload/domain/key/signature and remains type-distinct from LedgerDigest/VerificationReceipt.
- **SC-011** macOS native backend has live CI coverage on the repository's trusted macOS runner for its supported v1 operations, including protection/opening of the wrapped Ed25519 signing/master secret; no Secure Enclave signing claim is required for v1.
- **SC-012** Windows/Linux backends are not claimed verified without corresponding target/native evidence; any implemented support has explicit compile/fixture/live coverage status.
- **SC-013** `ecra-core` and `ecra-run` complete regression gates remain green.
- **SC-014** Dependency/unsafe/source-I/O boundary checks enforce that no model/browser/network/protocol/policy engine leaks into ECR-031.
- **SC-015** Donor/reference/license ledger records exact crypto/native-backend dependencies and distinguishes conceptual reference from source reuse.
- **SC-016** Post-implementation traceability maps FR-001–FR-058 and SC-001–SC-016, including C1–C4 refinements, to implementation/tests/contracts with zero unowned requirement and G1–G15 passing or explicitly N/A.

## 6. Non-goals

- authorization/declassification/approval/secret-use mediation;
- independent action outcome verification;
- remote identity federation product/provider integration;
- proof of legal/real-world human identity from local bootstrap;
- password manager/autofill/browser credential UX;
- WebAuthn/passkey product flows;
- cloud account requirement;
- team/organization/multi-device governance;
- sync/recovery/export product design;
- universal monotonic rollback resistance against restoration/control of the entire native trust store;
- hardware-backed/non-exportable Ed25519 signing in v1;
- hardware-token/TPM universal abstraction unless later evidence requires it;
- changing ECR-001 IDs or ECR-002 ledger semantics;
- local-model/browser/search/terminal/plugin execution.

## 7. Release/security claims

ECR-031 may claim only the guarantees demonstrated by its selected backend and cryptographic contract. “Encrypted”, “tamper-resistant”, “hardware-backed”, “device-bound”, “user-presence protected” and similar terms MUST be scoped to exact evidence. General resistance to a fully compromised user account/kernel/debugger or an attacker with equivalent native trust-store authority is not claimed. Local bootstrap establishes an Ecra-local principal/trust context; it does not claim externally proofed human identity. V1 Ed25519 signing uses native-backend-protected wrapped key material and does not imply Secure Enclave/non-exportable signing.
