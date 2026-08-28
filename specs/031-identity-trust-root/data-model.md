# ECR-031 Data Model — Identity, Trust Root & Sensitive Storage

**Status:** PLANNING / NORMATIVE_CANDIDATE  
**Canonicalization:** strict JSON with RFC 8785 JCS where signed/hashed bytes are required  
**Version baseline:** major `1`, minor `0`

## 1. Ownership rules

ECR-031 reuses these ECR-001 types without semantic replacement:

```text
Actor
ActorId
PrincipalId
PrincipalRef
IdentityAssertionId
IdentityAssertionRef
ContentDigest (only as generic content identity where explicitly called for)
```

ECR-031 introduces only identity/trust/protected-storage concepts not already owned.

## 2. Identifier types

All IDs are typed opaque 128-bit UUID-compatible identifiers encoded in the repository's existing canonical UUID string form unless implementation proves a stronger existing repository rule.

```rust
TrustRootId
KeyId
ProtectedObjectId
AssertionNonceId
EnrollmentId
DelegationId
```

Rules:
- nil/invalid IDs rejected;
- IDs are identity only, never cryptographic proof;
- display labels, usernames, email addresses, paths and protocol subject strings are not substitutes.

## 3. Local principal enrollment and trust root

### LocalPrincipalEnrollmentV1

```text
LocalPrincipalEnrollmentV1 {
  version
  enrollment_id: EnrollmentId
  principal_id: PrincipalId
  trust_root_id: TrustRootId
  created_at: Timestamp
  trust_state_generation: u64-safe
  backend_binding_digest: BackendBindingDigest
}
```

Semantics:
- this represents an **Ecra-local installation principal**, not a legally/externally proofed human identity;
- `principal_id` is freshly generated and never derived from OS username/email/display metadata;
- the enrollment is usable only after its protected authoritative trust state has been created/opened successfully;
- partial bootstrap produces no `EnrolledPrincipalHandle` or usable issuer session;
- `backend_binding_digest` binds non-secret canonical backend identity/configuration metadata and is not itself authority.

### EnrolledPrincipalHandle

Opaque, non-serializable runtime handle produced only after successful protected enrollment/trust-state verification.

```text
EnrolledPrincipalHandle {
  principal: PrincipalRef
  enrollment_id: EnrollmentId
  trust_root_id: TrustRootId
  verified_trust_snapshot: VerifiedTrustSnapshot
}
```

It must not expose raw root/private keys. Callers cannot construct it from IDs alone.

### IssuerSession

Opaque, non-serializable bounded issuance capability internal to ECR-031:

```text
IssuerSession {
  enrolled_principal: EnrolledPrincipalHandle
  assertion_signing_key_id: KeyId
  session_created_at: Timestamp
}
```

The assertion subject comes from `enrolled_principal.principal`; there is no caller-supplied arbitrary subject principal parameter. `IssuerSession` is identity issuance context, not an ECR-003 authorization decision.

### TrustRootRecord

```text
TrustRootRecord {
  version
  trust_root_id
  backend_kind
  backend_locator_ref
  created_at
  status
  current_generation_by_purpose
}
```

`backend_locator_ref` is opaque backend metadata sufficient to locate protected material. It MUST NOT contain raw key material or other sensitive plaintext. Platform-specific locator content is non-authoritative outside the selected backend.

### TrustRootStatus

```text
active
locked
unavailable
revoked
```

`locked` and `unavailable` are operational states, not reasons to bypass protection.

## 4. Key record

```text
KeyRecord {
  version
  key_id
  trust_root_id
  purpose
  algorithm
  generation
  status
  public_material?      // only when algorithm/use requires safe public verification material
  created_at
  activated_at
  retired_at?
  revoked_at?
}
```

Raw private/root/derived symmetric key bytes are never serialized in `KeyRecord`.

### KeyPurpose

```text
identity_assertion_signing
protected_envelope_root
protected_anchor_signing
protected_anchor_mac
```

The contract may narrow this set before implementation. A key purpose cannot be silently reused for another cryptographic domain.

### KeyStatus

```text
active
retired_verify_or_decrypt_only
revoked
```

V1 invariant:
- one active key per `(trust_root_id, purpose)`;
- `retired_verify_or_decrypt_only` cannot create new signatures/envelopes;
- `revoked` cannot create new material and is rejected for current assertion validation;
- removal/destruction is operational key availability, not equivalent to revocation.

### ProtectedTrustStateV1

Security-critical lifecycle state is protected/authenticated under the trust root/backend rather than trusted from ordinary metadata.

```text
ProtectedTrustStateV1 {
  version
  enrollment_id: EnrollmentId
  principal_id: PrincipalId
  trust_root_id: TrustRootId
  generation: u64-safe
  active_key_by_purpose: ordered map<KeyPurpose, KeyId>
  key_records: bounded ordered collection<KeyRecord>
  revoked_key_ids: bounded ordered set<KeyId>
  previous_state_digest?: TrustStateDigest
}
```

This state is itself stored through the protected backend/envelope contract. Ordinary DB/file rows may mirror it for audit/UI but are non-authoritative unless their bytes are authenticated as this exact protected state.

### VerifiedTrustSnapshot

```text
VerifiedTrustSnapshot {
  enrollment_id: EnrollmentId
  principal: PrincipalRef
  trust_root_id: TrustRootId
  generation: u64-safe
  active_key_by_purpose
  key_records
  revoked_key_ids
  trust_state_digest: TrustStateDigest
  verified_at: Timestamp
}
```

Only successful authentication/opening/validation of `ProtectedTrustStateV1` may construct this type. Pure assertion validation consumes `VerifiedTrustSnapshot`, never an unsigned `TrustRootRecord`/ordinary projection.

Rollback boundary:
- stale ordinary metadata cannot override this snapshot;
- v1 does not claim universal monotonic rollback resistance if an attacker can restore/control the entire authorized native trust-store state/root itself.

## 5. Signature algorithm and v1 signing custody

```text
SignatureAlgorithm {
  name
  version
}
```

V1 canonical assertion/protected-anchor signing algorithm is:

```text
ed25519
```

Unknown algorithms fail closed.

The Ed25519 private signing key is generated from approved CSPRNG/key generation and persisted only as protected sensitive bytes under the native `TrustBackend`. It may be materialized only for bounded signing use and uses the selected redacted/zeroizing in-process wrapper. This v1 custody model does **not** claim Secure Enclave/non-exportable/hardware-backed signing. Native non-exportable signing is a future versioned algorithm-suite extension.

Algorithm choice is part of the signed assertion/protected-anchor contract and cannot be caller-defined arbitrary text.

## 6. Identity assertion

### IdentityAssertionV1

```text
IdentityAssertionV1 {
  version: { major: 1, minor: 0 }
  assertion_id: IdentityAssertionId
  issuer: AssertionIssuer
  subject_principal_id: PrincipalId
  actor_binding: ActorBinding
  on_behalf_of: OnBehalfOfBinding?
  audience: AssertionAudience
  issued_at: Timestamp
  not_before: Timestamp?
  expires_at: Timestamp
  nonce: AssertionNonceId?
  attributes: AssertionAttributes
  signature: AssertionSignature
}
```

Strict unknown-field rejection applies.

Issuance rule: `subject_principal_id` is sourced from `IssuerSession.enrolled_principal.principal`; arbitrary caller-selected principal IDs cannot be minted.

### AssertionIssuer

```text
AssertionIssuer {
  trust_root_id: TrustRootId
  key_id: KeyId
}
```

### ActorBinding

```text
ActorBinding {
  actor_id: ActorId
}
```

This means the assertion is usable only for the exact attributable actor context. It does not grant that actor authority.

### OnBehalfOfBinding

V1 is single-user/desktop first and deliberately bounded:

```text
OnBehalfOfBinding {
  principal_id: PrincipalId
  delegation_id: DelegationId
}
```

Absence means no on-behalf-of delegation claim; it never means ANY principal. ECR-031 authenticates the binding as identity evidence. ECR-003 later interprets whether such delegation is permitted for an action.

### AssertionAudience

```text
AssertionAudience {
  service: closed enum/string identifier
  instance_id?: opaque id
}
```

V1 must use a bounded canonical identifier. Examples may include `ecra_policy_local` or another contract-defined consumer. Arbitrary URL audience matching is not introduced unless required by a later protocol adapter.

### AssertionAttributes

Only identity attributes necessary for the current assertion class are allowed. Free-form metadata cannot be parsed as authority. Sensitive profile data is not required for v1.

### AssertionSignature

```text
AssertionSignature {
  algorithm: SignatureAlgorithm
  key_id: KeyId
  bytes_b64url: bytes
}
```

Signed bytes are JCS of the assertion payload excluding the `signature` field, with explicit Ecra domain separation.

## 7. Validation context

```text
IdentityValidationContext {
  evaluated_at: Timestamp
  expected_actor_id: ActorId
  expected_audience: AssertionAudience
  expected_principal_id?: PrincipalId
  replay_state: ReplayValidationInput
  trust_snapshot: VerifiedTrustSnapshot
}
```

Validation does not read ambient time/environment/network state and does not accept raw/unsigned trust metadata in place of the verified snapshot.

### ValidatedIdentityContext

```text
ValidatedIdentityContext {
  assertion_ref: IdentityAssertionRef
  principal: PrincipalRef
  actor_id: ActorId
  issuer_trust_root_id: TrustRootId
  signing_key_id: KeyId
  audience: AssertionAudience
  on_behalf_of?: ValidatedOnBehalfOf
  evaluated_at: Timestamp
  assertion_digest: IdentityAssertionDigest
  trust_state_digest: TrustStateDigest
}
```

Explicitly absent:

```text
CapabilityGrant
ScopeConstraint granting authority
Approval
AuthorizationDecision
DeclassificationDecision
Secret bytes
```

`IdentityAssertionDigest` and `TrustStateDigest` are domain-specific and distinct from `ContentDigest`, `ActionDigest` and `LedgerDigest`.

## 8. Trust backend interface model

```text
TrustBackendCapabilities {
  backend_kind
  user_scoped: bool
  machine_bound: bool
  hardware_backed_private_operations: bool
  non_exportable_private_key: bool
  user_presence_gate: bool
  biometric_gate: bool
  locked_state_observable: bool
  synchronizing_store: bool
}
```

A capability value is evidence about the selected backend contract, not a security grant.

`TrustBackend` minimum operation families:

```text
bootstrap_or_open_enrollment
protect_secret / open_protected_secret
store_protected_trust_state / open_protected_trust_state
capabilities
health/locked status
delete_backend_material where safe
```

V1 canonical signing itself uses the protected Ed25519 software key after bounded authenticated opening; there is no generic `export_private_key()` API and no arbitrary principal-mint API.

## 9. Protected envelope

### ProtectedEnvelopeV1

```text
ProtectedEnvelopeV1 {
  version: { major: 1, minor: 0 }
  object_id: ProtectedObjectId
  purpose: ProtectedPurpose
  information_class: ProtectedInformationClass
  key_ref: EnvelopeKeyRef
  algorithm: AeadAlgorithm
  nonce_b64url: bytes
  ciphertext_b64url: bytes
  tag_b64url: bytes? // representation may be combined if dependency API emits ciphertext+tag
  aad: ProtectedAad
}
```

All fields required to interpret the ciphertext are authenticated. Exact serialized representation is frozen in the contract before implementation.

### ProtectedPurpose

Closed enum, e.g.:

```text
identity_state
trust_state
assertion_signing_key
protected_anchor_signing_key
consumer_sensitive_blob
ledger_anchor_material
```

ECR-031 must not use purpose strings as authority. Domain separation prevents one protected class being replayed as another.

### ProtectedInformationClass

A narrow storage-protection class linked to ECR-001 information-classification semantics. It does not declassify or authorize disclosure.

### EnvelopeKeyRef

```text
EnvelopeKeyRef {
  trust_root_id: TrustRootId
  key_id: KeyId
  generation: u64-safe
}
```

### AeadAlgorithm

V1 candidate:

```text
chacha20_poly1305_rfc8439
```

Unknown algorithms fail closed. Nonce length/key length/tag length are algorithm-fixed and validated before decryption.

### ProtectedAad

Canonical AAD binds at minimum:

```text
version
object_id
purpose
information_class
trust_root_id
key_id
generation
algorithm
```

No mutable display metadata belongs in AAD unless its mutation must invalidate the envelope.

## 10. Derived key context

If HKDF is used:

```text
DerivedKeyContext {
  protocol: "ecra.protected-envelope.v1"
  trust_root_id
  key_id
  generation
  purpose
  object_domain
}
```

HKDF `info` is canonical/domain-separated and prevents derived keys being reused across cryptographic purposes. If a native backend cannot legitimately expose suitable IKM, the implementation must use an equivalent backend-protected wrapping/operation design after contract convergence; it must never extract hardware-protected material merely to satisfy this formula.

## 11. Protected authenticity anchor

```text
ProtectedAnchorV1 {
  version
  anchor_id
  trust_root_id
  key_id
  purpose
  payload_digest
  algorithm: ed25519
  signature_or_mac
}
```

### ProtectedAnchorPurpose

Closed enum examples:

```text
run_ledger_head
artifact_manifest
trust_state_snapshot
```

It is distinct from outcome verification.

### Payload binding

Canonical protected-anchor signing input:

```text
"ecra.protected-anchor.v1" || JCS({purpose, trust_root_id, key_id, payload_digest, algorithm})
```

Exact domain-separation encoding must be byte-frozen in `contracts/identity-trust-v1.md`.

## 12. Replay state

Assertion replay handling is class-dependent.

```text
ReplayValidationInput {
  mode: reusable_within_validity | single_use_nonce
  nonce_seen?: bool
}
```

The pure validator consumes replay state but does not own the durable replay database. If v1 issues single-use assertions, the plan must assign durable nonce state to an explicit ECR-031 protected store and use ECR-002-compatible durability patterns without making the run ledger identity authority.

## 13. Timestamp model

Use the repository's existing strict timestamp convention if present. Requirements:
- UTC normalized canonical representation;
- total ordering;
- no local timezone ambiguity;
- `not_before <= expires_at`;
- bounded validity duration configured by assertion class/issuer, not arbitrary attacker input;
- validation takes `evaluated_at` explicitly.

## 14. Error model

```text
IdentityError {
  category
  code
  safe_context
  source? // internal only, redacted at public boundary
}
```

Candidate categories:

```text
invalid_input
compatibility
identity_validation
trust_backend
key_state
cryptographic_authentication
protected_storage
corruption
platform_unavailable
bootstrap
issuance
```

Candidate codes include:

```text
assertion_signature_invalid
assertion_expired
assertion_not_yet_valid
assertion_audience_mismatch
assertion_actor_mismatch
assertion_principal_mismatch
assertion_delegation_invalid
assertion_replay_rejected
trust_root_unavailable
trust_root_locked
trust_snapshot_authentication_failed
trust_snapshot_stale_or_mismatched
bootstrap_incomplete
issuer_session_unavailable
subject_principal_override_rejected
key_not_found
key_not_active
key_revoked
unsupported_algorithm
unsupported_version
protected_envelope_invalid
authentication_failed
backend_unsupported
backend_invariant_violation
```

Errors must not include plaintext, secret bytes, private keys, decrypted values or sensitive backend payloads.

## 15. Persistence model

ECR-031 may persist only metadata and protected envelopes needed for its own trust state.

Authoritative rule:
- `ProtectedTrustStateV1` authenticated/opened under the selected native trust backend/root is authoritative for security-critical key generation/status/revocation;
- ordinary SQLite/files may contain versioned audit/projection metadata but cannot independently authorize key state;
- stale ordinary metadata cannot reactivate a key;
- any SQLite/file store must validate bytes, use crash-safe atomicity/transactions, keep raw protected keys out of rows and have migration/corruption fixtures;
- identity authority never comes from ECR-002 run projections.

The exact projection/store choice remains an implementation decision bounded by this authority rule.

## 16. Platform-specific metadata

### macOS

Opaque Data Protection Keychain persistent reference/tag/access-control metadata only. `synchronizing_store=false` is required for the v1 local-only path. V1 stores/protects wrapped Ed25519 signing/master secrets in Keychain and does not claim Secure Enclave Ed25519 signing.

### Windows

Opaque DPAPI-protected blob/metadata if implemented. Default DPAPI backend capability reports user+machine binding accurately. Native non-exportable signing, if ever added, requires separate CNG/NCrypt research/contract.

### Linux

Opaque Secret Service item/object reference and **non-secret** lookup attributes only. Secret bytes must be the item secret, never attributes. The service may be locked/unavailable and such states fail closed.

## 17. Serialization and limits

All untrusted serialized structures must have hard byte/count/depth limits before expensive cryptographic or allocation work. Exact limits are frozen in the contract before implementation and covered by hostile-input tests.

No unbounded certificate chain, delegation list, attribute map or metadata bag is part of v1.
