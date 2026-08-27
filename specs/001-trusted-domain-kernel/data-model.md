# Data Model: Trusted Domain Kernel

**Feature:** ECR-001  
**Status:** PLAN_READY_INPUT

This is the normative conceptual data model for the first trusted-core contract. Exact Rust field names may differ only when the implementation plan records an equivalent mapping and contract fixtures remain semantically identical.

## 1. Version Envelope

Every top-level persisted/wire fixture uses:

```text
Versioned<T>
- schema_version: SchemaVersion
- value: T
```

`SchemaVersion`
- `major: u16`
- `minor: u16`

Rules:
- ECR-001 supports major `1` only.
- unsupported major or unsupported newer minor fails with typed compatibility error;
- parsing never silently upgrades authority/security semantics.

## 2. Identifiers

Strong opaque IDs:

```text
ActorId
RunId
CapabilityId
ObservationId
FactId
EvidenceId
ArtifactId
ActionId
ReceiptId
VerificationId
```

Normative JSON: UUID string.

Rules:
- ID newtypes are not mutually convertible without explicit code;
- display metadata is never identity;
- ID form does not encode permission, actor kind, or trust.

## 3. Actor

```text
Actor
- id: ActorId
- kind: ActorKind
- label?: string
```

`ActorKind`
- `human`
- `agent`
- `system`

Rules:
- `label` is non-authoritative metadata;
- actor kind survives serialization;
- future identity/authentication material belongs to later slices.

## 4. Origin

```text
Origin
- kind: OriginKind
- detail: OriginDetail
```

`OriginKind`
- `user_input`
- `web`
- `local`
- `retrieval`
- `tool`
- `model`
- `memory`
- `system_policy`

`WebOrigin`
- `scheme`
- `host`
- `port?`
- `opaque: bool`

Additional detail records MAY contain a provider/tool/model/memory identifier but MUST NOT imply authority.

Rules:
- full resource URL/path is separate from origin;
- origin is provenance/security context, not instruction class;
- web/tool/model/memory origin never self-authorizes.

## 5. Resource and Scope

```text
ResourceRef
- kind: ResourceKind
- locator: string
- origin?: WebOrigin
```

Initial `ResourceKind`:
- `web_resource`
- `local_resource`
- `workspace_resource`
- `tool_resource`
- `artifact`
- `abstract`

```text
Scope
- workspace_id?: string
- browser_space_id?: string
- container_id?: string
- tab_id?: string
- session_id?: string
- allowed_origins: list<WebOrigin>
- resource_constraints: list<ResourceConstraint>
- task_id?: string
- purpose?: string
```

ECR-001 validates structure only. Subset/narrowing semantics belong to ECR-003.

## 6. Time Values

```text
EpochMillis(i64)
```

Rules:
- must fit I-JSON exact integer range;
- core never reads system time;
- evaluation uses caller-supplied `EvaluationContext { now }`.

```text
TemporalValidity
- not_before?: EpochMillis
- expires_at?: EpochMillis
```

Validation:
- if both exist, `not_before <= expires_at`.

## 7. Capability Request and Grant

```text
CapabilityRequest
- id: CapabilityId
- principal: ActorId
- operation: OperationRef
- target: ResourceRef
- scope: Scope
- temporal?: TemporalValidity
- requested_by: ActorId
- reason?: string
```

```text
CapabilityGrant
- id: CapabilityId
- principal: ActorId
- operation: OperationRef
- target: ResourceRef
- scope: Scope
- temporal?: TemporalValidity
- issued_by: ActorId
- parent_grant?: CapabilityId
- delegation_depth?: u16
```

`OperationRef`
- stable namespace/name string pair, e.g. `browser/read`, without policy-engine syntax.

Rules:
- request and grant are distinct types;
- grant construction requires explicit API;
- temporal validity and structural scope are validated;
- parent grant existence/narrowing is verified later by ECR-003.

## 8. Observation

```text
Observation
- id: ObservationId
- actor: ActorId
- origin: Origin
- observed_at?: EpochMillis
- subject: ResourceRef
- payload: ObservationPayloadRef
- evidence: list<EvidenceRef>
```

Observation is what Ecra or an actor observed/retrieved, not a claim of universal truth.

`ObservationPayloadRef` points to structured inline small data or an artifact reference. Large content is never required inside the core domain object.

## 9. Fact

```text
Fact
- id: FactId
- subject: ResourceRef
- predicate: string
- value: FactValue
- provenance: Provenance
- trust_state: TrustState
- freshness: Freshness
- evidence: list<EvidenceRef>
- derived_from: list<FactId>
```

`Provenance`
- `user_provided`
- `observed_web`
- `observed_local`
- `retrieved`
- `tool_provided`
- `model_inferred`
- `system_derived`

`TrustState`
- `unverified`
- `verified`
- `contradicted`
- `disputed`
- `inconclusive`

`Freshness`
- `current`
- `stale`
- `unknown`

Rules:
- verification never changes original provenance;
- staleness does not imply falsehood;
- contradiction can exist without choosing a winner.

## 10. Evidence

```text
EvidenceRef
- id: EvidenceId
- kind: EvidenceKind
- artifact?: ArtifactId
- observation?: ObservationId
- receipt?: ReceiptId
- external_ref?: string
```

`EvidenceKind`
- `observation`
- `artifact`
- `structured_tool_result`
- `network_receipt`
- `external_state`
- `computation`
- `model_judgment`
- `other`

Rules:
- references, not arbitrary evidence blobs;
- evidence kind does not itself guarantee trust;
- later verifier contracts decide evidentiary weight.

## 11. Artifact

```text
ArtifactRef
- id: ArtifactId
- kind: ArtifactKind
- media_type?: string
- logical_name?: string
- content_digest?: ContentDigest
- byte_size_decimal?: string
- storage_locator?: string
- lineage: list<LineageRef>
```

`ArtifactKind`
- `file`
- `document`
- `image`
- `structured_data`
- `model_output`
- `browser_snapshot`
- `network_capture`
- `other`

`ContentDigest`
- `algorithm`
- `hex`

ECR-001 does not dictate storage. `storage_locator` is opaque to trust logic.

## 12. Action Intent

```text
ActionIntent
- id: ActionId
- actor: ActorId
- capability: OperationRef
- target: ResourceRef
- scope: Scope
- parameters: ActionParametersRef
- side_effect: SideEffectClass
- idempotency: IdempotencySpec
- retry: RetryClass
- created_at?: EpochMillis
- correlation_id?: string
```

`SideEffectClass`
- `read_only`
- `local_mutation`
- `reversible_external_mutation`
- `irreversible_external_mutation`
- `unknown`

`IdempotencyClass`
- `naturally_idempotent`
- `idempotent_with_key`
- `non_idempotent`
- `unknown`

`IdempotencySpec`
- `class`
- `key_ref?`

`RetryClass`
- `safe`
- `requires_same_idempotency_key`
- `requires_external_reconciliation`
- `never_blind_retry`

Selected invariants:
- `idempotent_with_key` requires `key_ref`;
- `non_idempotent` or `unknown` MUST NOT pair with an implicitly permissive retry policy;
- `unknown` side effect is treated conservatively by downstream policy.

## 13. Action Receipt

```text
ActionReceipt
- id: ReceiptId
- action: ActionId
- executor_actor: ActorId
- started_at?: EpochMillis
- completed_at?: EpochMillis
- outcome: ActionOutcome
- evidence: list<EvidenceRef>
- external_reference?: string
- error?: ErrorSummary
```

`ActionOutcome`
- `confirmed_success`
- `confirmed_failure`
- `unknown`

Rules:
- receipt is executor evidence, not independent verification;
- `unknown` is valid and must not be coerced to failure/success;
- completed timestamp cannot precede started timestamp;
- receipt cannot refer to a different action by implicit parameter equivalence; it uses stable ActionId.

## 14. Verification Receipt

```text
VerificationReceipt
- id: VerificationId
- verifier: ActorId
- target: VerificationTarget
- method: VerificationMethod
- evidence: list<EvidenceRef>
- outcome: VerificationOutcome
- evaluated_at?: EpochMillis
- notes?: string
```

`VerificationTarget`
- action: ActionId
- receipt: ReceiptId
- fact: FactId
- artifact: ArtifactId
- abstract_claim: string/reference

`VerificationMethod`
- `structured_external_state`
- `api_or_tool_result`
- `network_receipt`
- `artifact_validation`
- `dom_or_accessibility_state`
- `deterministic_computation`
- `independent_model_judgment`
- `other`

`VerificationOutcome`
- `verified`
- `rejected`
- `inconclusive`
- `not_evaluated`

Rules:
- target and verifier are explicit;
- verification never mutates the underlying receipt/fact provenance;
- `not_evaluated` is distinct from `inconclusive`.

## 15. Errors

Machine-distinguishable categories:

```text
CompatibilityError
IdentifierError
OriginError
ScopeError
CapabilityError
TemporalError
ActionSemanticError
ReceiptError
EvidenceError
CanonicalizationError
```

Public callers MUST NOT need to parse display strings to determine error category.

## 16. Canonicalization and Digests

Normative canonicalization uses RFC 8785 JCS over the versioned JSON form where digest identity is required.

Rules:
- no NaN/Infinity;
- no duplicate JSON keys;
- values outside exact JSON number range use documented string forms;
- canonical bytes are test-fixture-visible;
- ECR-001 does not define ledger chaining or signatures.

## 17. State Transition Ownership

ECR-001 defines value objects and validation only. It does not define a persistent run state machine. ECR-002 will compose these entities into lifecycle transitions.

The following ownership boundary is normative:

```text
ECR-001: What an Actor/Action/Receipt/Fact/Capability IS.
ECR-002: How a Run changes over time and persists events.
ECR-003: Whether a Capability is authorized.
ECR-004: Whether a result/claim is verified and how retry/reconciliation proceeds.
```

This boundary prevents the core type model from becoming an accidental orchestrator.
