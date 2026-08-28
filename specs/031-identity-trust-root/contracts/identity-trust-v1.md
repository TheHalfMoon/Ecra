# ECR-031 Contract — Identity & Trust v1

**Status:** NORMATIVE_PLANNING_CANDIDATE  
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

## 2. Identity assertion wire

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

The example algorithm is a planning fixture, not a claim every native backend can provide hardware-backed Ed25519.

### 2.1 Canonical signature payload

The signed payload is the complete assertion object **without** the `signature` member.

```text
identity_assertion_signing_input =
  UTF8("ecra.identity-assertion.v1\n") || RFC8785_JCS(assertion_without_signature)
```

No alternate whitespace/property-order representation changes the signed bytes after JCS canonicalization.

### 2.2 Assertion digest

```text
IdentityAssertionDigest =
  SHA-256(
    UTF8("ecra.identity-assertion-digest.v1\n") ||
    RFC8785_JCS(assertion_without_signature)
  )
```

This is identity assertion content identity only. It is not a `ContentDigest`, `ActionDigest`, `LedgerDigest`, signature or authorization token.

### 2.3 Validation order

Fail-closed order:

1. input byte/depth/count limits;
2. UTF-8/JSON parse, duplicate/unknown-field rejection;
3. version/enum/ID/timestamp structural validation;
4. issuer/key existence and algorithm allowlist;
5. key lifecycle status applicable to assertion validation;
6. canonical signature verification;
7. explicit expected principal if supplied;
8. exact actor binding;
9. exact audience binding;
10. not-before/expiry against caller-supplied `evaluated_at`;
11. explicit on-behalf-of structural/binding rules;
12. nonce/replay rule from explicit replay context;
13. construct `ValidatedIdentityContext`.

No identity context is returned from a partially validated assertion.

## 3. On-behalf-of contract

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
- validation proves binding/evidence only;
- whether a delegation is allowed for an action remains ECR-003 policy.

## 4. Key lifecycle contract

### 4.1 Key record wire

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

### 4.2 Lifecycle matrix

| Current | Operation | Result |
|---|---|---|
| active | sign/protect new | ALLOW if purpose matches |
| active | verify/decrypt existing | ALLOW if purpose matches |
| active | rotate | create next generation then atomically designate it active; prior active becomes retired mode according to purpose |
| retired_verify_or_decrypt_only | sign/protect new | DENY |
| retired_verify_or_decrypt_only | verify/decrypt existing | ALLOW only for contract-supported historical compatibility |
| retired_verify_or_decrypt_only | reactivate | DENY in v1 |
| revoked | sign/protect new | DENY |
| revoked | verify assertion for current identity context | DENY in v1 |
| revoked | decrypt protected data | policy is purpose-specific and MUST be frozen before implementation; no implicit allow |

Destruction/unavailability is not represented as revocation. If required material is unavailable, return typed unavailable state.

## 5. Protected envelope wire

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

The selected Rust implementation may expose ciphertext and tag as one byte sequence. The canonical v1 wire uses a single `ciphertext_b64url` containing the full AEAD ciphertext+tag unless dependency research requires a normative correction before implementation.

### 5.1 AAD

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

AEAD input AAD:

```text
UTF8("ecra.protected-envelope-aad.v1\n") || ProtectedEnvelopeAadV1
```

### 5.2 AEAD v1 candidate

For `chacha20_poly1305_rfc8439`:
- key: 256 bits;
- nonce: 96 bits;
- nonce MUST be unique for each encryption under the same derived key;
- full authentication tag; no truncation;
- failed authentication returns no plaintext.

### 5.3 Key derivation candidate

When an Ecra master key is available as IKM to a reviewed crypto boundary:

```text
salt = SHA-256(
  UTF8("ecra.protected-envelope-hkdf-salt.v1\n") ||
  trust_root_id || key_id || generation
)

info = UTF8("ecra.protected-envelope-key.v1\n") ||
       RFC8785_JCS({purpose, object_domain, algorithm})

DEK = HKDF-SHA-256(IKM, salt, info, 32)
```

If a native backend never exposes suitable IKM, implementation MUST use an equivalent backend-protected wrapping/operation design and amend this candidate before code. It MUST NOT extract hardware-protected keys merely to satisfy this formula.

## 6. Protected anchor wire

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

Signing/MAC input:

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

`ProtectedAnchorV1` is not a `VerificationReceipt` and does not assert external action correctness.

## 7. Backend capability contract

Canonical backend capability output contains booleans/closed enums only; it contains no raw native handles or secrets.

```json
{
  "backend_kind": "macos_keychain",
  "user_scoped": true,
  "machine_bound": true,
  "hardware_backed_private_operations": false,
  "non_exportable_private_key": false,
  "user_presence_gate": true,
  "biometric_gate": true,
  "locked_state_observable": true,
  "synchronizing_store": false
}
```

Example values are illustrative. Live backend tests determine actual reported capabilities; they MUST NOT be copied blindly into implementation.

## 8. Backend selection contract

Production selection is explicit and platform-bounded:

```text
macOS   -> macos_keychain backend or typed unavailable/unsupported
Windows -> windows_dpapi / reviewed native backend or typed unavailable/unsupported
Linux   -> freedesktop_secret_service backend or typed unavailable/locked/unsupported
```

`memory`, `plaintext`, `environment`, and `file_key` are not production backend choices.

A test backend is compiled/constructed only through test-specific API/feature that cannot be reached by ordinary production configuration.

## 9. Linux Secret Service constraint

Lookup attributes MUST be limited to non-secret metadata such as opaque Ecra object IDs and a fixed application namespace. Prohibited attributes include:
- secret/token/password value;
- private key material;
- decrypted assertion content;
- sensitive user/workspace text;
- private filenames/URLs when their disclosure is sensitive.

This is normative because upstream Secret Service explicitly does not treat attributes as secret material.

## 10. Error wire

Public errors expose typed category/code and safe bounded context only.

```json
{
  "category": "identity_validation",
  "code": "assertion_signature_invalid",
  "safe_context": null
}
```

Authentication/decryption failures at untrusted boundaries may collapse cryptographic detail to `authentication_failed` to reduce oracle surface. Internal instrumentation still MUST NOT contain secret/plaintext/key bytes.

## 11. Parser hard limits — v1 planning bounds

The following bounds are normative candidates and MUST be tested before implementation closure:

```text
MAX_ASSERTION_BYTES            64 KiB
MAX_PROTECTED_ENVELOPE_BYTES   64 MiB + 16 KiB metadata overhead
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

## 12. Required fixture families

### Valid
- minimal local assertion;
- exact actor/principal/audience assertion;
- bounded on-behalf-of assertion;
- active and retired key records;
- protected envelope;
- protected anchor;
- backend capability examples.

### Invalid
- unknown fields/duplicate keys;
- unsupported versions/algorithms;
- nil/invalid IDs;
- malformed timestamps and inverted validity window;
- wrong actor/principal/audience/issuer/key;
- invalid signature;
- expired/not-yet-valid;
- revoked key;
- replayed single-use nonce;
- envelope nonce/key/AAD/ciphertext/tag mutation;
- oversized fields/depth/count;
- secret-like Linux lookup attribute fixture;
- production test-backend selection attempt.

## 13. Compatibility rule

V1 readers:
- accept exactly supported `1.x` minors only when the implementation explicitly declares backward-compatible semantics;
- reject major != 1;
- never ignore unknown security-relevant fields to emulate forward compatibility.

Any algorithm/format migration requires fixtures proving old protected data/assertions are either safely readable/validatable under documented rules or explicitly rejected with recovery guidance.
