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
```

Rules:
- nil/invalid IDs rejected;
- IDs are identity only, never cryptographic proof;
- display labels, usernames, email addresses, paths and protocol subject strings are not substitutes.

## 3. Trust root

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
- `revoked` cannot create new material and is rejected for assertion validation unless a future explicitly versioned historical-validation policy says otherwise;
- removal/destruction is operational key availability, not equivalent to revocation.

## 5. Signature algorithm

```text
SignatureAlgorithm {
  name
  version
}
```

V1 allowlist is plan/dependency locked. Candidate portable test/signing suite is `ed25519`, but platform-native non-exportable signing may require another explicitly represented algorithm. Unknown algorithms fail closed.

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
  delegation_id: opaque typed id/ref?
}
```

If implementation needs a separate delegation identifier, it must be typed and exact. Absence means no on-behalf-of delegation claim; it never means ANY principal.

ECR-031 validates identity/delegation binding only. ECR-003 later interprets whether such delegation is permitted for an action.

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
  trust_snapshot: TrustSnapshotRef
}
```

Validation does not read ambient time/environment/network state.

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

`IdentityAssertionDigest` is domain-specific and distinct from `ContentDigest`, `ActionDigest` and `LedgerDigest`.

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
create_or_open_root
create_key
sign_or_mac
verify_if_backend_owned?  // only if backend contract requires it
protect_root_secret / unprotect_root_secret OR equivalent protected operation
delete_or_revoke_backend_material where safe
capabilities
health/locked status
```

The implementation plan may narrow operations. Generic `export_private_key()` is forbidden from the trusted v1 interface.

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

HKDF `info` is canonical/domain-separated and prevents derived keys being reused across cryptographic purposes. Salt/IKM handling is frozen in the contract and implementation clarification if the native backend cannot expose suitable key material directly.

## 11. Protected authenticity anchor

```text
ProtectedAnchorV1 {
  version
  anchor_id
  trust_root_id
  key_id
  purpose
  payload_digest
  algorithm
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

Canonical protected-anchor signing/MAC input:

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

The pure validator consumes replay state but does not own the durable replay database. If v1 issues single-use assertions, the plan must assign durable nonce state to an explicit ECR-031 store and use ECR-002-compatible durability patterns without making the run ledger identity authority.

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

ECR-031 may persist only metadata and protected envelopes needed for its own trust state. Any SQLite/file store must:
- version schemas;
- validate all untrusted bytes on read;
- use atomic/crash-safe replacement or transactional semantics;
- keep raw protected backend keys out of ordinary DB rows;
- have migration/corruption fixtures;
- avoid mixing identity authority into ECR-002 run projections.

The exact store choice remains a plan/dependency task, not a license to reuse SQLite without threat analysis.

## 16. Platform-specific metadata

### macOS

Opaque Keychain persistent reference/tag/access-control metadata only. No assumption that every key is Secure Enclave-backed. `synchronizing_store=false` is required for the v1 local-only path.

### Windows

Opaque DPAPI-protected blob/metadata or CNG handle if research requires asymmetric native keys. Default DPAPI backend capability reports user+machine binding accurately.

### Linux

Opaque Secret Service item/object reference and **non-secret** lookup attributes only. Secret bytes must be the item secret, never attributes. The service may be locked/unavailable and such states fail closed.

## 17. Serialization and limits

All untrusted serialized structures must have hard byte/count/depth limits before expensive cryptographic or allocation work. Exact limits are frozen in the contract/implementation clarification before implementation and covered by hostile-input tests.

No unbounded certificate chain, delegation list, attribute map or metadata bag is part of v1.
