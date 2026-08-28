# ECR-031 Threat Model — Identity, Trust Root & Sensitive Storage

**Status:** PLANNING / REQUIRED_SECURITY_GATE  
**Constitution:** v1.1.0  
**Primary platform risks:** R-018, R-036, R-052, R-053, R-054 plus cross-cutting R-003/R-004/R-005 where identity material becomes input to later consumers.

## 1. Assets

- correctness of Principal/Actor/on-behalf-of identity binding;
- trust-root/private/symmetric key material;
- key lifecycle/revocation state;
- protected sensitive plaintext;
- assertion signatures and canonical payloads;
- protected-envelope authenticity/confidentiality;
- protected-anchor authenticity;
- backend capability/assurance truth;
- migration/version metadata needed to safely interpret protected state.

## 2. Trust boundaries

```text
Untrusted caller / model / browser / protocol / file bytes
                     │
                     ▼
           Strict ECR-031 parser/validator
                     │
                     ▼
             ecra-identity trusted crate
              │                 │
              ▼                 ▼
      Native TrustBackend    protected local metadata/store
              │
      ┌───────┼─────────┐
      ▼       ▼         ▼
   macOS    Windows    Linux
 Keychain/   DPAPI/    Secret Service
 SE where    native     implementation
 applicable backend
```

ECR-031 does not trust:
- Actor labels or self-asserted principal strings;
- model/web/protocol statements about identity;
- serialized identity/envelope bytes before strict validation;
- database/filesystem integrity without cryptographic validation;
- a backend merely because it is local;
- a platform capability unless the selected backend reports/tests it.

## 3. Adversaries

### A1 — Malicious/untrusted model or content
Attempts to inject a principal ID, assertion, display label or token that looks authoritative.

Mitigation: typed assertion validation; content is data; no authority output from ECR-031.

### A2 — Unrelated local process
Attempts to read/replace protected metadata, call exposed local endpoints, replay assertions or steal key material.

Mitigation: narrow process API, OS keystore protections, authenticated envelopes, no generic key export, replay/audience/actor binding.

### A3 — Same-user rogue process
May have significant local account privileges and potentially access the same OS credential facilities depending on platform ACL/application identity semantics.

Mitigation target: use the strongest reasonable app/item access controls and narrow endpoints. General containment against an attacker with equivalent keystore authorization is **not guaranteed**. Backend-specific stronger claims require evidence.

### A4 — Filesystem/database attacker
Can copy, modify, truncate or replace ECR-031 metadata/envelope files but lacks protected key authority.

Mitigation: strict versioning, AEAD, protected signatures/MACs, atomic/transactional persistence, fail-closed parsing.

### A5 — Stale/revoked credential holder
Possesses an old valid assertion or key-era artifact and tries to use it after rotation/revocation.

Mitigation: exact key status/trust snapshot, audience/time/actor/replay context, explicit lifecycle semantics.

### A6 — Malicious platform/backend response
Corrupt/unexpected native errors, handles or capability claims reach the Rust boundary.

Mitigation: minimal adapter, strict normalization, safe errors, backend invariant tests, no native type leakage into domain types.

### A7 — Fully compromised OS/kernel/debugger/firmware
May inspect process memory, invoke keychain as user, tamper runtime, or subvert crypto/backend.

General ECR-031 guarantee: **out of containment scope**. Secure Enclave/non-exportable/user-presence controls may provide narrower backend-specific resistance but do not justify a universal claim.

## 4. Threats and required controls

### T1 — Actor → Principal confusion
Attack: attacker supplies `ActorId`/label and downstream assumes authenticated principal.

Controls:
- ECR-001 type separation preserved;
- only `ValidatedIdentityContext` can bridge actor/principal for later policy;
- compile-fail/type-confusion tests;
- no conversion shortcut `ActorId -> PrincipalId`.

### T2 — Assertion parameter substitution
Attack: modify subject, actor, audience, delegation, issuer, key or time after signing.

Controls:
- all security-relevant fields in canonical signed payload;
- strict unknown-field/duplicate-key rejection;
- JCS canonicalization;
- mutation corpus.

### T3 — Assertion replay / confused relying party
Attack: valid assertion for one actor/service used for another.

Controls:
- expected actor and audience required in validation context;
- bounded validity;
- nonce/replay rule where single-use semantics required;
- no protocol token passthrough.

### T4 — Delegation escalation
Attack: absence or free-form text interpreted as broad on-behalf-of rights.

Controls:
- explicit typed binding;
- no arbitrary recursive chain in v1;
- missing means no delegation claim;
- ECR-003 still decides authority.

### T5 — Revoked/stale key accepted
Controls:
- exact issuer key id + lifecycle snapshot;
- revoked assertion signing key fails current validation;
- one active generation per purpose;
- rotation/revocation transition tests.

### T6 — Private/root key leakage
Vectors: logs, Debug, errors, environment variables, generic SQLite rows, test fixtures, memory-backend fallback.

Controls:
- no raw private/root key in public types;
- redacted error model;
- secret-string scan/log capture tests;
- production backend enum excludes memory/plaintext/env/file key;
- OS-protected backend or fail closed.

### T7 — Plaintext fallback under backend failure
Attack: keystore locked/unavailable and application stores plaintext to remain functional.

Control: prohibited. Typed `locked/unavailable/unsupported` failure; negative tests.

### T8 — Ciphertext/metadata substitution
Attack: move ciphertext under another object/purpose/classification/key or flip bytes.

Controls:
- exact AAD binding;
- domain-separated key derivation/purpose;
- AEAD full tag;
- wrong AAD/key/nonce/ciphertext/tag tests.

### T9 — Nonce reuse
Impact: catastrophic confidentiality/authentication loss for ChaCha20-Poly1305 under same key.

Controls:
- CSPRNG nonce generation through explicit boundary;
- 96-bit exact nonce;
- generation/purpose domain separation;
- deterministic injected nonce only in tests;
- never caller-supplied arbitrary production nonce unless contract proves uniqueness ownership.

### T10 — Cryptographic oracle via detailed errors
Control: public/untrusted boundary may collapse tag/signature failure into `authentication_failed`; details remain safe/internal without secret bytes.

### T11 — Linux Secret Service metadata leak
Attack: sensitive values placed in lookup attributes that may be unencrypted.

Controls:
- attributes limited to opaque non-secret Ecra identifiers/fixed namespace;
- negative secret-like attribute fixtures;
- doc warning and boundary test.

### T12 — Windows assurance overclaim
Attack/product defect: default DPAPI represented as portable/cross-machine/hardware-backed.

Controls:
- backend capability truth;
- same-user/same-machine semantics documented/tested;
- no sync/recovery claim.

### T13 — macOS assurance overclaim
Attack/product defect: ordinary Keychain key claimed Secure Enclave/non-exportable/user-presence protected.

Controls:
- report actual item/key configuration;
- live backend tests;
- `hardware_backed_private_operations` only true when operation uses a verified Secure Enclave-backed key path.

### T14 — Plain hash-chain authenticity overclaim
Attack: attacker rewrites ECR-002 store and recomputes all hashes.

Controls:
- `ProtectedAnchor` distinct from `LedgerDigest`;
- stronger claim only when anchor key remains protected;
- no retroactive change of ECR-002 digest semantics.

### T15 — Protected anchor mistaken for outcome verification
Control: distinct types/names/API and negative architecture tests. Only ECR-004 owns independent verification outcome.

### T16 — Malformed/oversized input DoS
Controls:
- hard byte/count/depth limits before expensive crypto;
- bounded attributes/no arbitrary chain;
- fuzz/property/malicious corpus.

### T17 — Test backend reaches production
Controls:
- test-only construction/feature; production backend selection enum excludes it;
- build/boundary tests inspect source/configuration;
- no environment variable that silently switches protection off.

### T18 — Migration/downgrade attack
Controls:
- version-authenticated metadata;
- reject unsupported newer/invalid formats;
- migrations transactionally/atomically preserve protected semantics;
- never decrypt then rewrite plaintext as downgrade fallback.

## 5. Native backend threat notes

### macOS
- Use Data Protection Keychain for new local-only items unless research proves a reason otherwise.
- Secure Enclave is only claimed where the exact private-key operation is supported/configured.
- User-presence/biometric access control can affect unattended agent behavior; required operations must not silently weaken access control to satisfy automation.
- No iCloud synchronization in v1.

### Windows
- Default DPAPI is scoped to the same user/machine under normal use; machine-wide flag is not a safe default for user secret isolation.
- Password/admin recovery semantics can affect data availability; ECR-031 does not promise recovery from destructive credential resets.
- If asymmetric non-exportable keys are required, CNG/NCrypt research must be explicit rather than pretending DPAPI is a signing HSM.

### Linux
- Secret Service implementation/security varies with desktop/service.
- Current 0.2 spec is a draft.
- Attributes are non-secret; secrets go only through the secret value/session path.
- Locked/unavailable service must fail closed.

## 6. Security test matrix

Must include:
- exact assertion mutation corpus;
- actor/principal/audience/delegation/replay/time negatives;
- key lifecycle transition table;
- revoked key/current assertion rejection;
- envelope mutation/wrong-key/AAD/tag/nonce negatives;
- parser limits/fuzz corpus;
- redaction/no-secret-log tests;
- backend-unavailable no-fallback tests;
- Linux attribute secrecy boundary test;
- native macOS live tests on trusted runner;
- ECR-001/ECR-002 regression gates;
- source/dependency/unsafe boundary gates.

## 7. Claims allowed after v1 evidence

Potentially allowed, narrowly:
- identity assertions are cryptographically bound to explicit principal/actor/audience context under an Ecra trust root;
- ECR-031 protected envelopes provide authenticated encryption under the documented backend/key assumptions;
- selected native backend protects root/key material according to its tested capability contract;
- protected anchors provide key-backed authenticity under the stated trust-root boundary.

Not allowed as generic claims:
- impossible to hack;
- secure against a compromised OS/user account;
- all keys hardware-backed;
- cross-device recoverable;
- NIST-certified identity assurance;
- LedgerDigest is tamper-proof;
- protected anchor means external action verified.

## 8. Incident/failure posture

If root/key compromise is suspected:
- mark relevant key/trust state compromised/revoked through explicit lifecycle API;
- stop issuance/new protection with affected key;
- do not silently re-sign/re-encrypt historical artifacts as if compromise never occurred;
- surface affected protected objects/assertions for explicit remediation;
- preserve audit evidence without logging secret material;
- later ECR-003/ECR-025 products own user-facing policy/privacy response.
