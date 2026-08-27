# Contract: Ecra Trusted Domain v1

**Feature:** ECR-001  
**Contract version:** 1.0  
**Status:** NORMATIVE_FOR_IMPLEMENTATION

This contract defines externally observable semantics for the v1 trusted-domain kernel. Rust implementation details may vary only if these invariants and fixture behavior remain unchanged.

## 1. Versioning

Top-level normative values MUST be wrapped in:

```json
{
  "schema_version": { "major": 1, "minor": 0 },
  "value": {}
}
```

Rules:

- major != 1 → `unsupported_major_version`;
- minor > supported minor → `unsupported_minor_version`;
- missing version → `missing_schema_version`;
- unknown fields in normative security-sensitive v1 objects are rejected unless a later contract explicitly designates an extension field.

## 2. Canonical JSON

Canonical bytes MUST conform to RFC 8785 JCS.

Contract tests MUST include:

- map/object key ordering;
- Unicode/non-BMP ordering;
- escaped control characters;
- negative zero handling;
- rejection of non-I-JSON numeric values;
- canonicalization fixed-point check.

Any digest field defined by later contracts MUST name the algorithm and the canonical input domain; no unversioned implicit hashing is permitted.

## 3. Typed IDs

All IDs are UUID strings in JSON and strong newtypes in Rust.

Invalid:
- empty strings;
- non-UUID strings;
- automatic conversion between ID categories.

A caller MAY use the same UUID bytes for different fixture ID categories, but the Rust types remain non-interchangeable and authorization MUST NOT rely on ID formatting.

## 4. Actor Contract

Valid kinds:

```text
human
agent
system
```

Required:
- `id`;
- `kind`.

Optional:
- non-authoritative `label`.

No actor label, model name, email, username, or display string grants authority.

## 5. Origin Contract

Valid origin kinds:

```text
user_input
web
local
retrieval
tool
model
memory
system_policy
```

For web origins, scheme/host/port are structured and validated. Full URL/path is not the origin.

**Invariant:** Origin expresses provenance/security context; it does not grant permission or instruction authority.

## 6. Capability Contract

`CapabilityRequest` and `CapabilityGrant` MUST have different type names and deserialize into different Rust types.

Required request fields:
- `id`;
- `principal`;
- `operation`;
- `target`;
- `scope`;
- `requested_by`.

Required grant fields:
- `id`;
- `principal`;
- `operation`;
- `target`;
- `scope`;
- `issued_by`.

Optional:
- temporal bounds;
- purpose/task/workspace/browser scope;
- parent grant/delegation depth.

Structural invariants:
- `not_before <= expires_at` when both exist;
- negative/invalid delegation depth encoding rejected;
- malformed web origin rejected;
- requesting a capability never constructs a grant implicitly.

Authorization/subset semantics are explicitly out of scope until ECR-003.

## 7. Observation and Fact Contract

Observation represents an event/source observation.

Fact represents a claim/value that may be derived from one or more observations/evidence items.

They MUST NOT be aliases of the same struct/type.

Provenance values:

```text
user_provided
observed_web
observed_local
retrieved
tool_provided
model_inferred
system_derived
```

Trust values:

```text
unverified
verified
contradicted
disputed
inconclusive
```

Freshness values:

```text
current
stale
unknown
```

**Invariant:** Changing verification/trust state never erases original provenance.

**Invariant:** A `model_inferred` fact may become `verified`, but remains `model_inferred` in provenance.

## 8. Evidence Contract

Evidence kinds:

```text
observation
artifact
structured_tool_result
network_receipt
external_state
computation
model_judgment
other
```

Evidence is a reference. A later verifier assigns evidentiary weight. Evidence kind alone is not proof.

## 9. Artifact Contract

Artifact references MUST permit:
- stable ArtifactId;
- kind;
- optional media type;
- optional content digest;
- optional byte size encoded safely for canonical JSON;
- optional storage locator;
- zero or more lineage references.

Artifact bytes are out of scope for `ecra-core`.

## 10. Action Contract

Every `ActionIntent` MUST include:
- ActionId;
- actor;
- operation/capability reference;
- target;
- scope;
- parameters/reference;
- side-effect class;
- idempotency specification;
- retry class.

Side-effect classes:

```text
read_only
local_mutation
reversible_external_mutation
irreversible_external_mutation
unknown
```

Idempotency classes:

```text
naturally_idempotent
idempotent_with_key
non_idempotent
unknown
```

Retry classes:

```text
safe
requires_same_idempotency_key
requires_external_reconciliation
never_blind_retry
```

Mandatory semantic checks:

1. `idempotent_with_key` requires a non-empty idempotency-key reference.
2. `non_idempotent` MUST NOT pair with `safe` retry.
3. `unknown` idempotency MUST NOT pair with `safe` retry.
4. `irreversible_external_mutation` with non-idempotent/unknown idempotency MUST use a conservative retry class (`requires_external_reconciliation` or `never_blind_retry`).
5. `unknown` side-effect class is never normalized to read-only.

## 11. Action Receipt Contract

Action outcomes:

```text
confirmed_success
confirmed_failure
unknown
```

Required:
- receipt ID;
- exact action ID;
- executor actor;
- outcome.

Optional:
- start/completion times;
- evidence references;
- external reference;
- structured error summary.

Checks:
- completion time >= start time when both exist;
- unknown remains a valid final executor-known state;
- receipt is not independent verification.

## 12. Verification Receipt Contract

Verification outcomes:

```text
verified
rejected
inconclusive
not_evaluated
```

Verification method values:

```text
structured_external_state
api_or_tool_result
network_receipt
artifact_validation
dom_or_accessibility_state
deterministic_computation
independent_model_judgment
other
```

Required:
- verification ID;
- verifier actor;
- verification target;
- method;
- outcome;
- evidence list (which may be empty only for `not_evaluated` or explicitly documented inconclusive cases).

**Invariant:** ActionReceipt cannot be cast/deserialized as VerificationReceipt.

## 13. Time Contract

Epoch milliseconds are caller-provided value objects and MUST remain within the I-JSON exact integer range.

No domain validation function is permitted to call the OS clock.

Time-sensitive evaluation uses:

```text
EvaluationContext
- now: EpochMillis
```

## 14. Error Contract

Callers MUST be able to distinguish at least:

```text
compatibility
identifier
origin
scope
capability
temporal
action_semantics
receipt
evidence
canonicalization
```

Display messages may change without a major schema change. Machine-readable variants/codes are the contract.

## 15. Valid/Invalid Fixture Requirements

Before ECR-001 closes, contract fixtures MUST include at least:

### Valid

- human actor;
- agent actor;
- system actor;
- user/web/local/tool/model/memory origins;
- scoped capability request;
- scoped capability grant with expiry;
- delegated grant representation;
- DOM/web observation;
- model-inferred fact with observation evidence;
- verified model-inferred fact preserving provenance;
- contradicted fact with multiple evidence refs;
- read-only action;
- keyed-idempotent external mutation;
- non-idempotent external mutation with `never_blind_retry`;
- unknown action receipt;
- confirmed-success action receipt;
- independent verification receipt;
- artifact with digest and lineage.

### Invalid

- unsupported major/minor version;
- malformed UUID;
- malformed origin;
- temporal range reversed;
- request parsed as grant;
- keyed idempotency without key;
- non-idempotent action marked safe retry;
- unknown idempotency marked safe retry;
- receipt completion preceding start;
- unsupported enum value;
- unknown security-sensitive field under strict v1 parsing;
- non-I-JSON numeric/canonicalization case;
- verification object missing required target/verifier.

## 16. Dependency Boundary Contract

`ecra-core` runtime dependencies MUST NOT include:

- async runtime;
- HTTP/network client/server;
- database/storage driver;
- browser/CDP/WebDriver library;
- model SDK;
- Cedar/policy engine;
- MCP/ACP/A2A SDK;
- filesystem/process execution abstraction;
- telemetry exporter.

A new dependency outside the research-approved candidate set requires plan amendment or implementation-time justification proving it preserves zero-I/O semantics.
