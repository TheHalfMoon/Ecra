# Contract: Ecra Trusted Domain v1

**Feature:** ECR-001  
**Contract version:** 1.0  
**Status:** CONVERGED_IMPLEMENTATION_CONTRACT  
**Constitution:** v1.1.0

This contract defines externally observable v1 semantics for the Ecra trusted-domain kernel. It is synchronized with the implemented `ecra-core` types, validation, machine errors and committed fixtures. Implementation details may vary only if these invariants, canonical bytes and fixture behavior remain unchanged.

## 1. Versioning / Strict Parsing

Normative persisted/interchange values use:

```json
{
  "schema_version": { "major": 1, "minor": 0 },
  "value": {}
}
```

Rules:
- major != 1 -> machine code `unsupported_major_version`;
- newer unsupported minor -> `unsupported_minor_version`;
- malformed or missing strict envelope fields -> `serialization_failed`;
- undocumented fields on strict/security-sensitive v1 objects -> `serialization_failed`;
- parsing never silently widens authority, identity or information-flow semantics.

There is no `missing_schema_version` machine code in ECR-001 v1. `Versioned<T>::from_json_slice` strictly deserializes the envelope and then validates supported schema versions.

Repository fixture bodies may omit the envelope only under the fixture-runner convention in §28; this is not a wire exception.

## 2. Canonical JSON

Normative canonical bytes conform to RFC 8785 JCS.

Tests cover ordering, Unicode/escapes, numeric edge cases, invalid/non-I-JSON values where applicable, and fixed-point behavior.

JCS is canonicalization only. It is not a signature, authentication or authorization claim.

## 3. Typed Identifier Contract

All security/audit IDs are UUID strings in JSON and distinct strong Rust newtypes:

```text
ActorId
PrincipalId
IdentityAssertionId
RunId
ResourceId
WorkspaceId
BrowserSpaceId
ContainerId
TabId
SessionId
TaskId
CapabilityRequestId
CapabilityGrantId
ObservationId
FactId
EvidenceId
ArtifactId
ActionId
ActionAttemptId
ReceiptId
VerificationId
```

Invalid:
- empty/non-UUID strings;
- implicit conversion among ID categories;
- deriving authorization/trust from ID format or display text.

Same UUID bytes in different ID categories remain different types.

## 4. Actor / Principal Contract

`ActorKind`:

```text
human
agent
system
```

Actor contains ActorId, ActorKind and optional label.

`PrincipalRef` contains PrincipalId. `IdentityAssertionRef` contains IdentityAssertionId + PrincipalId.

Invariants:
- ActorId is attribution, not proof of authentication;
- ActorId cannot substitute for PrincipalId by implicit conversion;
- IdentityAssertionRef existence does not imply validity/current trust;
- actor labels and other text never grant authority;
- assertion validity/trust roots belong to ECR-031.

## 5. Origin Contract

Origin kinds:

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

Web origin is structured or explicitly opaque. Full URL/path is not origin identity.

Invariant: origin expresses provenance/security context, never permission or instruction authority.

## 6. Resource Contract

Every `ResourceRef` contains:
- ResourceId;
- ResourceKind;
- optional non-authoritative locator;
- optional WebOrigin where applicable.

Initial kinds:

```text
web_resource
local_resource
workspace_resource
tool_resource
artifact
abstract
```

Invariant: locator text is not canonical security identity and must not be used by core constructors to infer access or equivalence.

## 7. Scope Contract

Security scope uses:

```text
ScopeConstraint<T>
not_applicable
exact(T)
one_of(non-empty list<T>)
any_explicit
```

Scope contains explicit constraints for workspace, browser space, container, tab, session, task, web origins and ResourceIds, plus optional PurposeRef.

Mandatory rules:
1. missing/empty must not mean unrestricted;
2. `one_of([])` is invalid;
3. `any_explicit` is the only wildcard/unrestricted value;
4. `not_applicable` is not wildcard;
5. PurposeRef namespace/name are non-empty structured metadata and grant nothing;
6. subset/intersection/narrowing authorization belongs to ECR-003.

## 8. Capability Contract

`OperationRef` contains non-empty `namespace` + `name`. It is an operation identifier, not provider/policy syntax.

`CapabilityRequest` required fields:
- CapabilityRequestId;
- PrincipalRef;
- OperationRef;
- target ResourceRef;
- explicit Scope;
- requested_by ActorId.

Optional request fields:
- IdentityAssertionRef;
- temporal validity;
- `reason` free-form metadata.

`reason` is non-authoritative. Its text cannot manufacture permission, approval or identity.

`CapabilityGrant` required fields:
- CapabilityGrantId;
- PrincipalRef;
- OperationRef;
- target ResourceRef;
- explicit Scope;
- issued_by ActorId.

Optional grant fields:
- temporal validity;
- parent CapabilityGrantId;
- delegation depth.

Rules:
- request never implicitly constructs/deserializes as grant;
- request identity assertion principal must match request principal;
- delegation depth must be greater than zero when a delegation reference exists;
- `not_before <= expires_at`;
- parent existence/narrowing/revocation/authentication is downstream work.

## 9. Information Classification Contract

`InformationClass`:

```text
public
private
sensitive
secret
unknown
```

`InformationClassification` contains class + zero or more `InformationPolicyTag { namespace, name }` values. Tag namespace/name must be non-empty.

Invariants:
- classification grants no authority;
- unknown is not public;
- tags are data, not executable policy;
- deriving/transcribing information does not implicitly declassify it;
- ECR-003 owns inheritance/join/declassification policy.

## 10. Information Reference Contract

```text
InformationRef
observation(ObservationId)
fact(FactId)
artifact(ArtifactId)
action_parameter(ActionParameterRef)
```

The base reference type is available for Fact lineage before `InformationUse`; this is construction/task ordering only, not a different wire version.

## 11. Evidence Contract

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

`EvidenceRef` contains EvidenceId + kind and may include typed artifact/observation/receipt links, non-empty opaque `external_ref`, ContentDigest and `as_of`.

Rules:
- EvidenceId is stable evidence identity;
- references are not arbitrary evidence blobs;
- external reference text is non-authoritative;
- evidence kind or digest presence alone proves nothing;
- later verifier policy decides evidentiary sufficiency.

## 12. Observation Contract

`ObservationPayloadRef` is a tagged reference:

```text
artifact(ArtifactId)
evidence(EvidenceId)
resource(ResourceId)
external_ref(non-empty string)
```

Arbitrary page/tool/model payload blobs are not embedded in this trusted-domain reference. External reference text grants no access.

Observation fields:
- ObservationId;
- ActorId;
- Origin;
- optional observed_at;
- subject ResourceRef;
- ObservationPayloadRef;
- InformationClassification;
- evidence list.

Invariant: Observation records what was observed, not universal truth, permission or verification.

## 13. Fact / Provenance Contract

`FactValue`:

```text
text(string)
boolean(bool)
integer(I-JSON-safe i64)
decimal(canonical decimal string)
resource(ResourceId)
artifact(ArtifactId)
```

Canonical decimal rules:
- optional leading `-`;
- digits with optional fractional component;
- no exponent or leading `+`;
- no redundant leading zero;
- negative zero spellings are invalid.

Provenance:

```text
user_provided
observed_web
observed_local
retrieved
tool_provided
model_inferred
system_derived
```

Dispute state:

```text
undisputed
contradicted
disputed
inconclusive
unknown
```

Fact fields include stable FactId, ResourceRef subject, non-empty predicate, FactValue, provenance, classification, freshness, dispute, evidence list and derived InformationRef list.

There is no independent `verified` Fact field/state. Verification status comes only from VerificationReceipt records. A model-inferred Fact remains model-inferred after verification.

## 14. Freshness Contract

Freshness states:

```text
current
stale
unknown
```

Basis kinds:

```text
observed_at
retrieved_at
published_at
effective_at
source_reported
other
```

FreshnessAssessment may include `assessed_at`, `basis_kind`, `basis_time`, `basis_evidence`.

Mandatory structural rule: `basis_kind` and `basis_time` are both absent or both present. `basis_evidence` may be independently present.

Freshness never rewrites provenance/verification and source-reported time is not automatically trusted.

## 15. Artifact / Lineage Contract

`LineageRef`:

```text
observation(ObservationId)
fact(FactId)
artifact(ArtifactId)
```

Lineage uses stable IDs, not locators/display labels.

Artifact kinds:

```text
file
document
image
structured_data
model_output
browser_snapshot
network_capture
other
```

ArtifactRef supports:
- ArtifactId/kind;
- optional non-empty media type/logical name;
- InformationClassification;
- optional ContentDigest;
- optional canonical `byte_size_decimal`;
- optional non-empty storage locator;
- lineage list.

`byte_size_decimal` is `0` or a non-zero digit followed by digits. Signed, negative, fractional, empty and redundant-leading-zero forms are invalid.

Storage locator is non-authoritative.

## 16. Information Use / Disclosure Contract

Use kinds:

```text
local_compute
model_context
persist
log_or_diagnostic
external_disclosure
remote_provider
other
```

`InformationUse` requires a non-empty source list and may identify destination ResourceRef, destination WebOrigin and declared output classification.

Invariants:
- InformationUse is declaration, not authorization;
- read authority over source A plus write/tool authority to B does not imply allowed A -> B flow;
- external/remote use is a policy boundary owned by ECR-003.

## 17. Generic and Security Digest Contract

Generic metadata:

```text
ContentDigest
- algorithm: string
- hex: string
```

ContentDigest does not itself imply authenticity.

Security binding:

```text
SecurityDigest
- algorithm: sha256
- hex: validated SHA-256 representation

ActionDigest(SecurityDigest)
```

SecurityDigest v1 permits SHA-256 only.

## 18. Action Parameter Binding Contract

```text
ActionParametersRef
none
bound_artifact {
  artifact: ArtifactId,
  binding_digest: SecurityDigest
}
bound_external {
  external_ref: non-empty string,
  binding_digest: SecurityDigest
}
```

Every non-empty parameter reference carries a security digest. ArtifactId/external_ref are references only and grant no access.

A later executor that materializes parameter data must verify `binding_digest` before use; that I/O behavior is outside ECR-001.

```text
ActionParameterRef
- action: ActionId
- path: non-empty opaque string
```

Path is descriptive lineage/addressing metadata, not policy syntax.

## 19. Action Intent Contract

Every ActionIntent contains:
- ActionId;
- ActorId;
- optional PrincipalRef / IdentityAssertionRef;
- OperationRef;
- target ResourceRef;
- explicit Scope;
- ActionParametersRef;
- zero or more InformationUse declarations;
- EffectProfile;
- IdempotencySpec;
- RetryClass;
- optional created_at;
- optional non-empty correlation_id.

Rules:
- operation is requested operation, not authority grant;
- principal and identity assertion, when both present, must bind the same PrincipalId;
- correlation text is metadata and grants nothing;
- the Rust `ActionSemantics` helper is construction-only; canonical JSON remains flat with `effect`, `idempotency`, `retry`.

## 20. Effect / Idempotency / Retry Contract

Mutation domain:

```text
none
local
external
unknown
```

Reversibility:

```text
not_applicable
reversible
conditional
irreversible
unknown
```

Effect validation:
1. mutation `none` requires reversibility `not_applicable`;
2. mutation `local`/`external` rejects `not_applicable`;
3. mutation `unknown` requires reversibility `unknown`.

Idempotency:

```text
naturally_idempotent
idempotent_with_key
non_idempotent
unknown
```

Idempotency validation:
1. naturally-idempotent/non-idempotent/unknown must not carry key_ref;
2. idempotent-with-key requires non-empty key_ref.

Retry:

```text
safe
requires_same_idempotency_key
requires_external_reconciliation
never_blind_retry
```

Retry matrix:
1. `safe` only with naturally-idempotent and mutation != unknown;
2. `requires_same_idempotency_key` only with idempotent-with-key and mutation != unknown;
3. `requires_external_reconciliation` only with external or unknown mutation;
4. `never_blind_retry` allowed for every otherwise valid effect/idempotency pair;
5. non-idempotent/unknown never pair with `safe` or `requires_same_idempotency_key`.

Reversibility does not upgrade retry safety. Unknown never normalizes permissively.

## 21. Action Digest / ActionRef Contract

Every security-binding action reference is:

```text
ActionRef
- id: ActionId
- digest: ActionDigest
```

ActionDigest v1:

```text
SHA-256(
  UTF8("ecra/action-intent/v1\0")
  || RFC8785_JCS(Versioned<ActionIntent>)
)
```

The canonical ActionIntent contains all security-relevant fields, including ActionParametersRef and its binding digest, and contains no derived digest field.

Invariant: same ActionId + different canonical security-relevant body is a mismatch and must fail exact binding validation.

## 22. Action Attempt Contract

```text
ActionAttemptRef
- id: ActionAttemptId
- action: ActionRef
```

ActionAttemptId is distinct from ActionId. One action may later have multiple attempts. ECR-002 owns attempt creation/lifecycle/retry orchestration.

## 23. Action Receipt Contract

`ErrorSummary`:

```text
code: non-empty string
message?: non-empty string
```

ErrorSummary is executor diagnostic metadata, not DomainError or verification.

ActionReceipt required:
- ReceiptId;
- exact ActionAttemptRef;
- executor ActorId;
- ActionOutcome.

Outcomes:

```text
executor_observed_success
executor_observed_failure
unknown
```

Optional:
- start/completion timestamps;
- evidence refs;
- non-empty external reference;
- ErrorSummary.

Checks:
- completion >= start when both exist;
- attempt must bind the exact ActionIntent when validated against an intent;
- UNKNOWN remains valid;
- receipt is executor-known evidence only and never independent verification.

## 24. Verification Receipt Contract

`ClaimRef`:

```text
namespace: non-empty string
reference: non-empty string
```

Verification targets:

```text
action(ActionRef)
action_attempt(ActionAttemptRef)
receipt(ReceiptId)
fact(FactId)
artifact(ArtifactId)
claim(ClaimRef)
```

Verification outcomes:

```text
verified
rejected
inconclusive
not_evaluated
```

Methods:

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

VerificationReceipt contains VerificationId, verifier ActorId, optional verifier PrincipalRef, target, method, evidence list, outcome, optional evaluated_at and optional non-empty notes.

Evidence cardinality:
- verified/rejected/inconclusive require at least one EvidenceRef;
- not_evaluated may carry an empty evidence list.

Invariants:
- ActionReceipt cannot cast/deserialize as VerificationReceipt;
- executor-observed success is not VERIFIED;
- VerificationReceipt is the canonical verification record;
- notes are non-authoritative;
- ECR-004 owns evidence sufficiency and independence policy.

## 25. Time Contract

EpochMillis is caller-provided and within I-JSON exact integer range. No core validation reads the OS clock.

Time-sensitive checks use explicit `EvaluationContext { now }`.

## 26. Exact Machine Error Contract

The exact v1 `ErrorCategory` values are:

```text
Compatibility
Identifier
Identity
Origin
Resource
Scope
Capability
Temporal
Information
Canonicalization
Digest
Action
Attempt
Receipt
Verification
Serialization
```

The exact v1 `ErrorCode::as_str()` values are:

```text
unsupported_major_version
unsupported_minor_version
invalid_identifier
invalid_epoch_millis
invalid_temporal_range
invalid_origin
invalid_resource
invalid_scope
invalid_capability
invalid_identity
invalid_information
canonicalization_failed
invalid_content_digest
invalid_security_digest
invalid_action
invalid_attempt
invalid_receipt
invalid_verification
serialization_failed
```

Code-to-category mapping is defined by the machine API and covered exhaustively by tests. Display messages are not machine contracts.

Conceptual validation labels such as identity-reference, information-flow-shape, evidence, action-semantics or action-reference errors do not create additional machine categories in v1; they map to the broader categories above.

## 27. Minimum Valid / Invalid Contract Coverage

The committed corpus and tests must collectively cover at least:
- Human/Agent/System Actors;
- PrincipalRef/IdentityAssertionRef separation;
- user/web/local/tool/model/memory origins;
- ResourceRef identity vs locator;
- all ScopeConstraint variants;
- distinct capability request/grant and delegated grant;
- classification classes;
- ObservationPayloadRef and classified observation;
- FactValue forms, model-inferred Fact, conflict/dispute and freshness pairing;
- ArtifactRef lineage/size/digest metadata;
- InformationUse source-to-sink declarations;
- ActionParametersRef exact binding;
- read-only/local/external/unknown action semantics and retry matrix;
- fixed ActionDigest and field mutation sensitivity;
- distinct attempts;
- success/failure/UNKNOWN receipts;
- verified/rejected/inconclusive/not-evaluated verification;
- invalid strict fields/version/envelope forms;
- malformed IDs/origin/resource/scope/classification/freshness/digests;
- request/grant and receipt/verification type confusion;
- invalid parameter binding, action semantics, receipt timing/binding and verification evidence cardinality.

## 28. Repository Fixture Storage Convention

Public wire/persistence values remain `Versioned<T>`.

Files under `contracts/ecra-domain-v1/{valid,invalid}/` may store semantic inner bodies to avoid duplicating the same v1 envelope in every fixture. The fixture runner must:
1. pair each fixture with its declared target type;
2. construct/round-trip the corresponding `Versioned<T>` value;
3. validate the v1 envelope separately;
4. separately exercise full-envelope unsupported-version and unknown-strict-field cases.

This convention does not permit adapters, persistence, ActionDigest inputs or external interchange to omit required version envelopes.

## 29. Dependency Boundary

`ecra-core` production dependencies must not include:
- async runtime;
- network/HTTP;
- database/storage driver;
- browser automation;
- model SDK;
- Cedar/policy engine;
- MCP/ACP/A2A SDK;
- process/filesystem execution abstraction;
- telemetry exporter.

Pure dependencies for serialization/JSON/UUID/URL/JCS/errors/SHA-256 are permitted only after exact version/license/security review and donor-ledger update.

## 30. Enforcement Ownership

This contract defines structure, not policy/runtime truth:

- ECR-002: attempts, run state, budgets/cancellation, persistence/integrity chain;
- ECR-031: identity assertion validation, trust roots, key lifecycle, protected sensitive storage;
- ECR-003: authorization decisions/leases, grant narrowing, disclosure/declassification, approvals/secrets;
- ECR-004: verifier orchestration, evidence sufficiency, reconciliation.

ECR-001 remains zero-I/O and must not absorb these downstream responsibilities.