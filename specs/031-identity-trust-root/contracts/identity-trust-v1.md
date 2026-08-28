# ECR-031 Contract — Identity & Trust v1

**Status:** NORMATIVE_PLANNING_CANDIDATE / PASS_1_REMEDIATED  
**Wire family:** strict UTF-8 JSON + RFC 8785 JCS for canonical signed/MACed/AAD material  
**Version:** `1.0`

## 1. Contract rules

- All JSON objects reject unknown fields.
- Duplicate JSON object keys are invalid.
- All IDs use canonical ECR typed UUID representation.
- Integers use repository I-JSON-safe checked representation where applicable.
- Unsupported major versions fail closed; newer unsupported minor semantics are rejected unless an explicit compatibility rule exists.
- No display label, path, username, email or protocol string is an authority/principal identity.
- Canonical cryptographic inputs include an Ecra domain-separation prefix distinct per primitive.
- Raw private/root/derived keys and plaintext secrets never occur in public wire structures.
- Identity validation and assertion issuance are distinct APIs. Validation can be pure; issuance is a protected stateful operation.
- An assertion is identity evidence only. No wire field grants capability, approval, declassification or authorization.

## 2. Local installation bootstrap and enrollment

ECR-031 v1 creates an **Ecra-local principal** for the protected local installation/user context. It does not claim legal identity proofing, ownership of an email/username, NIST IAL/AAL/FAL certification, or external federation assurance.

### 2.1 Enrollment record

```json
{
  "version": { "major": 1, "minor": 0 },
  "enrollment_id": "00000000-0000-0000-0000-000000000030",
  "principal_id": "00000000-0000-0000-0000-000000000004",
  "trust_root_id": "00000000-0000-0000-0000-000000000002",
  "created_at": "2026-08-28T00:00:00Z",
  "kind": "ecra_local_installation_principal"
}
```

Rules:
- `principal_id`, `trust_root_id`, and `enrollment_id` are generated from the production CSPRNG/typed-ID boundary, not imported from OS username/email/display data.
- OS account/user context may determine which native keystore is reachable, but it does not become the canonical PrincipalId.
- bootstrap creates the trust root, initial purpose keys and protected trust state as one logical transaction.
- bootstrap is not complete until the protected trust state is durably published and can be reopened/authenticated.
- if backend key material exists but the protected trust state is absent/incomplete after a crash, the state is `incomplete_bootstrap`; v1 does not silently mint a second principal/root or treat orphaned material as enrolled identity.
- recovery may clean up orphaned backend material where safe, but cleanup is not identity enrollment.

### 2.2 EnrolledPrincipalHandle

`EnrolledPrincipalHandle` is an opaque in-process capability to request issuance for exactly one already-enrolled local principal. It contains no raw key material and is not serializable as a reusable token.

It can be created only by reopening/authenticating the protected local enrollment/trust state. Caller-provided `PrincipalId` is not sufficient to obtain it.

## 3. Assertion issuance boundary

ECR-031 MUST NOT expose a generic public API equivalent to:

```text
issue(principal_id, actor_id, ...)
issue_on_behalf_of(arbitrary_principal_id, actor_id, ...)
```

### 3.1 IssuerSession

Issuance requires an opaque `IssuerSession` created from:
- an authenticated `EnrolledPrincipalHandle`; and
- a `VerifiedTrustSnapshot` whose assertion-signing key is currently active.

`IssuerSession` is process-local, non-serializable, bounded to one enrolled principal/trust root and one issuance context. It is not a CapabilityGrant and does not authorize downstream actions.

The caller may request the actor/audience binding for the new assertion, but cannot substitute another subject principal by supplying an ID. V1 on-behalf-of issuance is limited to the enrolled local principal already bound to the IssuerSession. Broader delegation/authorization remains ECR-003.

No IPC/network assertion-minting service is part of ECR-031 v1.

### 3.2 V1 signing custody

The canonical v1 assertion and protected-anchor signing algorithm is `ed25519`.

The Ed25519 private seed/key material:
- is generated from the approved production CSPRNG;
- is stored only as a secret protected by the selected native `TrustBackend`;
- may be materialized in bounded process memory only for the signing operation when the backend requires software signing;
- is held in a redacted/zeroizing secret wrapper and released promptly after use;
- is never persisted in ordinary metadata, run artifacts, logs, environment variables or plaintext files.

This v1 path **does not claim** Secure Enclave, hardware-backed, or non-exportable signing. A future native non-exportable signing suite requires a versioned contract/algorithm extension and backend-specific evidence.

## 4. Identity assertion wire

```json
{
  "version": { "major": 1, "minor": 0 },
  "assertion_id": "00000000-0000-0000-0000-000000000001",
  "issuer": {
    "trust_root_id": "00000000-0000-0000-0000-000000000002",
    "key_id": "00000000-0000-0000-0000-000000000003"
  },
  "subject_principal_id": "00000000-0000-0000-0000-000000000004",
  "actor_binding": {
    "actor_id": "00000000-0000-0000-0000-000000000005"
  },
  "on_behalf_of": null,
  "audience": {
    "service": "ecra_policy_local",
    "instance_id": null
  },
  "issued_at": "2026-08-28T00:00:00Z",
  "not_before": "2026-08-28T00:00:00Z",
  "expires_at": "2026-08-28T00:05:00Z",
  "nonce": null,
  "attributes": {},
  "signature": {
    "algorithm": "ed25519",
    "key_id": "00000000-0000-0000-0000-000000000003",
    "bytes_b64url": "..."
  }
}
```

### 4.1 Canonical signature payload

The signed payload is the complete assertion object **without** the `signature` member.

```text
identity_assertion_signing_input =
  UTF8("ecra.identity-assertion.v1\n") || RFC8785_JCS(assertion_without_signature)
```

### 4.2 Assertion digest

```text
IdentityAssertionDigest =
  SHA-256(
    UTF8("ecra.identity-assertion-digest.v1\n") ||
    RFC8785_JCS(assertion_without_signature)
  )
```

This is identity assertion content identity only. It is not a `ContentDigest`, `ActionDigest`, `LedgerDigest`, signature or authorization token.

### 4.3 Validation order

Fail-closed order:

1. input byte/depth/count limits;
2. UTF-8/JSON parse, duplicate/unknown-field rejection;
3. version/enum/ID/timestamp structural validation;
4. require a `VerifiedTrustSnapshot`; ordinary unsigned key metadata is never accepted as lifecycle authority;
5. issuer/key existence and algorithm allowlist from that verified snapshot;
6. key lifecycle status applicable to assertion validation;
7. canonical signature verification;
8. explicit expected principal if supplied;
9. exact actor binding;
10. exact audience binding;
11. not-before/expiry against caller-supplied `evaluated_at`;
12. explicit on-behalf-of structural/binding rules;
13. nonce/replay rule from explicit replay context;
14. construct `ValidatedIdentityContext`.

No identity context is returned from a partially validated assertion.

## 5. On-behalf-of contract

V1 does not implement arbitrary recursive delegation chains.

When present:

```json
"on_behalf_of": {
  "principal_id": "00000000-0000-0000-0000-000000000004",
  "delegation_id": "00000000-0000-0000-0000-000000000006"
}
```

Rules:
- exact IDs only;
- missing field means no delegation claim;
- this field is authenticated by the assertion signature;
- v1 issuance may bind only the principal already owned by the `IssuerSession`; a caller cannot mint for an arbitrary principal ID;
- validation proves binding/evidence only;
- whether a delegation is allowed for an action remains ECR-003 policy.

## 6. Authoritative trust state and lifecycle

Security-critical lifecycle state is authoritative only after protected-state authentication.

### 6.1 ProtectedTrustStateV1 plaintext model

The plaintext below is never stored as ordinary cleartext. It is serialized canonically then stored inside a `ProtectedEnvelopeV1` with purpose `trust_state`.

```json
{
  "version": { "major": 1, "minor": 0 },
  "trust_root_id": "00000000-0000-0000-0000-000000000002",
  "enrollment": {
    "enrollment_id": "00000000-0000-0000-0000-000000000030",
    "principal_id": "00000000-0000-0000-0000-000000000004",
    "kind": "ecra_local_installation_principal"
  },
  "state_generation": 1,
  "keys": [],
  "revoked_key_ids": [],
  "updated_at": "2026-08-28T00:00:00Z"
}
```

Rules:
- one authenticated protected trust-state object is the v1 lifecycle authority for enrollment, active generation, retirement and revocation metadata;
- ordinary DB/file indexes may cache public/rebuildable metadata but are not trusted to activate/unrevoke a key;
- every lifecycle change rewrites/publishes a newly authenticated protected trust state using crash-safe atomic replacement semantics;
- reopening must authenticate/decrypt the protected state before constructing any validation/issuance snapshot;
- stale/unsigned/rebuildable metadata that disagrees with protected state is rejected or rebuilt, never preferred.

### 6.2 VerifiedTrustSnapshot

`VerifiedTrustSnapshot` is an immutable in-memory validation input created only after:
1. the selected native backend successfully opens the required root/protection material;
2. the `ProtectedEnvelopeV1` carrying `ProtectedTrustStateV1` authenticates successfully;
3. internal lifecycle invariants validate with no duplicate active generation, malformed record or unknown incompatible version.

It contains only the validated lifecycle/public verification data required for the operation. It is not constructible from ordinary unsigned metadata by the production API.

### 6.3 Rollback boundary

V1 provides tamper/authentication detection against filesystem/database modification when the attacker lacks protected backend key authority. It does **not** claim universal monotonic rollback resistance if an attacker can restore an older, still-valid authenticated protected trust-state blob together with the corresponding authorized OS trust-store state.

Therefore:
- `state_generation` detects internal ordering inconsistencies but is not marketed as a hardware monotonic counter;
- restoring the entire authorized OS trust store / equivalent backend state is outside the filesystem-only adversary guarantee;
- a future monotonic/external anchor may strengthen rollback guarantees without changing current claims.

## 7. Key lifecycle contract

### 7.1 Key record wire

```json
{
  "version": { "major": 1, "minor": 0 },
  "key_id": "00000000-0000-0000-0000-000000000003",
  "trust_root_id": "00000000-0000-0000-0000-000000000002",
  "purpose": "identity_assertion_signing",
  "algorithm": "ed25519",
  "generation": 1,
  "status": "active",
  "public_material_b64url": "...",
  "created_at": "2026-08-28T00:00:00Z",
  "activated_at": "2026-08-28T00:00:00Z",
  "retired_at": null,
  "revoked_at": null
}
```

`public_material_b64url` is permitted only for public verification material. Private/symmetric material is prohibited.

### 7.2 Lifecycle matrix

| Current | Operation | Result |
|---|---|---|
| active | sign/protect new | ALLOW if purpose matches and snapshot is verified/current |
| active | verify/decrypt existing | ALLOW if purpose matches |
| active | rotate | create next generation, protect required new secret material, atomically publish protected state with new active generation; prior active becomes retired according to purpose |
| retired_verify_or_decrypt_only | sign/protect new | DENY |
| retired_verify_or_decrypt_only | verify/decrypt existing | ALLOW only for contract-supported historical compatibility |
| retired_verify_or_decrypt_only | reactivate | DENY in v1 |
| revoked | sign/protect new | DENY |
| revoked | verify assertion for current identity context | DENY in v1 |
| revoked | decrypt protected data | DENY for current trust-state/enrollment objects; consumer historical-data behavior requires an explicitly versioned purpose rule and MUST NOT be inferred |

Destruction/unavailability is not represented as revocation. If required material is unavailable, return typed unavailable state.

## 8. Protected envelope wire

```json
{
  "version": { "major": 1, "minor": 0 },
  "object_id": "00000000-0000-0000-0000-000000000010",
  "purpose": "identity_state",
  "information_class": "sensitive",
  "key_ref": {
    "trust_root_id": "00000000-0000-0000-0000-000000000002",
    "key_id": "00000000-0000-0000-0000-000000000011",
    "generation": 1
  },
  "algorithm": "chacha20_poly1305_rfc8439",
  "nonce_b64url": "...",
  "ciphertext_b64url": "..."
}
```

The canonical v1 wire uses a single `ciphertext_b64url` containing AEAD ciphertext+tag.

### 8.1 AAD

```text
ProtectedEnvelopeAadV1 = RFC8785_JCS({
  "version": {"major":1,"minor":0},
  "object_id": ...,
  "purpose": ...,
  "information_class": ...,
  "key_ref": ...,
  "algorithm": ...
})
```

```text
AEAD_AAD = UTF8("ecra.protected-envelope-aad.v1\n") || ProtectedEnvelopeAadV1
```

### 8.2 AEAD v1

For `chacha20_poly1305_rfc8439`:
- key: 256 bits;
- nonce: 96 bits;
- nonce MUST be unique for each encryption under the same derived key;
- full authentication tag; no truncation;
- failed authentication returns no plaintext.

### 8.3 Key derivation

When an Ecra master key is legitimately materialized from the reviewed native backend as IKM:

```text
salt = SHA-256(
  UTF8("ecra.protected-envelope-hkdf-salt.v1\n") ||
  trust_root_id || key_id || generation
)

info = UTF8("ecra.protected-envelope-key.v1\n") ||
       RFC8785_JCS({purpose, object_domain, algorithm})

DEK = HKDF-SHA-256(IKM, salt, info, 32)
```

The master secret is protected by the native backend and materialized only for bounded operations in a redacted/zeroizing wrapper. No hardware-protected/non-exportable claim is made for this portable v1 path.

If a future backend uses non-exportable cryptographic operations, that is a versioned suite extension; ECR-031 MUST NOT extract a hardware-protected key merely to satisfy this formula.

## 9. Protected anchor wire

```json
{
  "version": { "major": 1, "minor": 0 },
  "anchor_id": "00000000-0000-0000-0000-000000000020",
  "trust_root_id": "00000000-0000-0000-0000-000000000002",
  "key_id": "00000000-0000-0000-0000-000000000021",
  "purpose": "run_ledger_head",
  "payload_digest": "sha256:<hex>",
  "algorithm": "ed25519",
  "signature_or_mac_b64url": "..."
}
```

Signing input:

```text
UTF8("ecra.protected-anchor.v1\n") || RFC8785_JCS({
  version,
  trust_root_id,
  key_id,
  purpose,
  payload_digest,
  algorithm
})
```

`ProtectedAnchorV1` is not a `VerificationReceipt` and does not assert external action correctness. Its Ed25519 key uses the same software-key/native-backend-protection custody rule as assertion signing unless a distinct purpose key is configured.

## 10. Backend capability contract

Canonical backend capability output contains booleans/closed enums only; it contains no raw native handles or secrets.

```json
{
  "backend_kind": "macos_keychain",
  "user_scoped": true,
  "machine_bound": true,
  "hardware_backed_private_operations": false,
  "non_exportable_private_key": false,
  "user_presence_gate": false,
  "biometric_gate": false,
  "locked_state_observable": true,
  "synchronizing_store": false
}
```

For the portable v1 Ed25519 software-signing path, `hardware_backed_private_operations` and `non_exportable_private_key` MUST be false even though the seed is stored in Keychain. Live tests determine actual backend availability/locked/local-only properties. Future native signing suites report their own evidence separately.

## 11. Backend selection contract

Production selection is explicit and platform-bounded:

```text
macOS   -> macos_keychain backend or typed unavailable/unsupported
Windows -> windows_dpapi / reviewed native backend or typed unavailable/unsupported
Linux   -> freedesktop_secret_service backend or typed unavailable/locked/unsupported
```

`memory`, `plaintext`, `environment`, and `file_key` are not production backend choices.

A test backend is compiled/constructed only through test-specific API/feature that cannot be reached by ordinary production configuration.

## 12. Linux Secret Service constraint

Lookup attributes MUST be limited to non-secret metadata such as opaque Ecra object IDs and a fixed application namespace. Prohibited attributes include secret/token/password values, private key material, decrypted assertions, sensitive user/workspace text and sensitive filenames/URLs.

## 13. Error wire

Public errors expose typed category/code and safe bounded context only.

```json
{
  "category": "identity_validation",
  "code": "assertion_signature_invalid",
  "safe_context": null
}
```

Required additional codes include:

```text
incomplete_bootstrap
enrollment_not_found
issuer_session_unavailable
issuer_principal_mismatch
trust_snapshot_authentication_failed
trust_snapshot_stale_or_inconsistent
```

Authentication/decryption failures at untrusted boundaries may collapse cryptographic detail to `authentication_failed` to reduce oracle surface. Internal instrumentation still MUST NOT contain secret/plaintext/key bytes.

## 14. Parser hard limits — v1 planning bounds

```text
MAX_ASSERTION_BYTES            64 KiB
MAX_PROTECTED_ENVELOPE_BYTES   64 MiB + 16 KiB metadata overhead
MAX_PROTECTED_TRUST_STATE      1 MiB
MAX_KEYS_PER_TRUST_ROOT        256
MAX_REVOKED_KEY_IDS            256
MAX_ATTRIBUTES                 32
MAX_ATTRIBUTE_NAME_BYTES       128
MAX_ATTRIBUTE_VALUE_BYTES      1024
MAX_AUDIENCE_SERVICE_BYTES     128
MAX_BACKEND_LOCATOR_BYTES      4096
MAX_PUBLIC_KEY_BYTES           4096
MAX_SIGNATURE_BYTES            4096
MAX_JSON_DEPTH                 16
```

ECR-031 v1 does not accept unbounded delegation/certificate chains.

## 15. Required fixture families

### Valid
- completed local bootstrap/enrollment;
- verified protected trust snapshot;
- minimal local assertion issued through an `IssuerSession`;
- exact actor/principal/audience assertion;
- bounded on-behalf-of assertion for the enrolled principal;
- active and retired key records;
- protected envelope;
- protected anchor;
- backend capability examples.

### Invalid
- unknown fields/duplicate keys;
- unsupported versions/algorithms;
- nil/invalid IDs;
- malformed timestamps and inverted validity window;
- OS username/email/label substituted as principal identity;
- incomplete bootstrap/orphan backend material;
- caller-selected arbitrary principal issuance;
- issuance without `EnrolledPrincipalHandle`/`IssuerSession`;
- wrong actor/principal/audience/issuer/key;
- invalid signature;
- expired/not-yet-valid;
- revoked key;
- replayed single-use nonce;
- unsigned/stale/replaced trust metadata offered as lifecycle authority;
- protected trust-state authentication mutation;
- envelope nonce/key/AAD/ciphertext/tag mutation;
- oversized fields/depth/count;
- secret-like Linux lookup attribute fixture;
- production test-backend selection attempt;
- false Secure Enclave/non-exportable capability claim on portable Ed25519 v1 path.

## 16. Compatibility and rollback rule

V1 readers:
- accept exactly supported `1.x` minors only when the implementation explicitly declares backward-compatible semantics;
- reject major != 1;
- never ignore unknown security-relevant fields to emulate forward compatibility.

Any algorithm/format migration requires fixtures proving old protected data/assertions are either safely readable/validatable under documented rules or explicitly rejected with recovery guidance.

Authenticated protected state prevents undetected modification under the stated key boundary; v1 does not promise monotonic rollback resistance against restoration of an older valid protected state plus equivalent authorized keystore state.
