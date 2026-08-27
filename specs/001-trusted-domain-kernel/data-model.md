# Data Model: Trusted Domain Kernel

**Feature:** ECR-001  
**Status:** CONVERGED_IMPLEMENTATION_CONTRACT  
**Constitution:** v1.1.0

This is the normative conceptual data model for Ecra's first trusted-core contract after implementation convergence. It describes the v1 wire/domain semantics implemented by `crates/ecra-core` and exercised by the committed fixture/test corpus.

ECR-001 defines zero-I/O value objects, references, canonicalization, security binding and structural validation only. It does not authenticate identities, authorize actions/disclosures, persist runs, execute attempts, verify external outcomes, or perform network/browser/model/process/filesystem I/O.

## 1. Version Envelope

```text
Versioned<T>
- schema_version: SchemaVersion
- value: T

SchemaVersion
- major: u16
- minor: u16
```

Rules:
- v1 supports major `1` and current minor `0`;
- unsupported major -> `unsupported_major_version`;
- newer unsupported minor -> `unsupported_minor_version`;
- malformed/missing envelope fields or unknown strict fields fail structural deserialization as `serialization_failed`;
- security-sensitive values reject undocumented fields;
- parsing never silently upgrades authority or information-flow semantics.

`Versioned<T>::from_json_slice` performs strict Serde decoding first and then validates schema compatibility. There is no separate `missing_schema_version` machine code in ECR-001 v1.

## 2. Strong Identifiers

Normative JSON representation: UUID string. Each is a distinct Rust newtype:

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

Rules:
- no implicit conversion among ID categories;
- same UUID bytes across categories do not make the types equivalent;
- ID form/name/prefix does not encode permission or trust;
- generation is caller-owned and requires no core randomness or clock.

## 3. Actor vs Principal / Identity Assertion Reference

```text
Actor
- id: ActorId
- kind: ActorKind
- label?: string

ActorKind
- human
- agent
- system

PrincipalRef
- id: PrincipalId

IdentityAssertionRef
- id: IdentityAssertionId
- principal: PrincipalId
```

Rules:
- Actor is attribution/runtime participation, not proof of authentication;
- Actor label is non-authoritative;
- a PrincipalRef does not claim an assertion is valid/current;
- an IdentityAssertionRef is an opaque reference, not validation evidence;
- ECR-031 owns assertion validation, trust roots, on-behalf-of relationships and key lifecycle;
- downstream durable state must reject conflicting ActorKind definitions for the same ActorId.

## 4. Origin

```text
Origin
- user_input
- web(WebOrigin)
- local
- retrieval
- tool
- model
- memory
- system_policy

WebOrigin
- structured web tuple or explicit opaque origin
```

Rules:
- full URL/path is not origin identity;
- origin is provenance/security context, not instruction class;
- origin never grants authority;
- opaque origins remain opaque and are not normalized into a fake web tuple.

## 5. Resource Identity

```text
ResourceRef
- id: ResourceId
- kind: ResourceKind
- locator?: string
- origin?: WebOrigin

ResourceKind
- web_resource
- local_resource
- workspace_resource
- tool_resource
- artifact
- abstract
```

Rules:
- `id` is the Ecra stable reference used for joins;
- `locator` is descriptive/provider addressing metadata and non-authoritative;
- provider-specific canonical identity/alias resolution is later work;
- policy must not infer equivalence or authority from locator text alone.

## 6. Explicit Scope Algebra

Every security-relevant dimension uses an explicit constraint:

```text
ScopeConstraint<T>
- not_applicable
- exact(T)
- one_of(non-empty canonicalized list<T>)
- any_explicit
```

Rules:
- `one_of` must be non-empty;
- missing/empty never means ANY;
- `any_explicit` is the only unrestricted representation for a dimension;
- `not_applicable` means the dimension does not apply, not unrestricted authority.

```text
Scope
- workspace: ScopeConstraint<WorkspaceId>
- browser_space: ScopeConstraint<BrowserSpaceId>
- container: ScopeConstraint<ContainerId>
- tab: ScopeConstraint<TabId>
- session: ScopeConstraint<SessionId>
- task: ScopeConstraint<TaskId>
- origins: ScopeConstraint<WebOrigin>
- resources: ScopeConstraint<ResourceId>
- purpose?: PurposeRef

PurposeRef
- namespace: non-empty string
- name: non-empty string
```

`PurposeRef` is structured metadata used by later policy. It grants nothing by itself. ECR-003 owns subset/intersection/narrowing semantics.

## 7. Time Values

```text
EpochMillis(i64)
```

Values must remain in the I-JSON exact integer range.

```text
EvaluationContext
- now: EpochMillis

TemporalValidity
- not_before?: EpochMillis
- expires_at?: EpochMillis
```

Rules:
- when both temporal endpoints exist, `not_before <= expires_at`;
- the core never reads the system clock;
- time-sensitive checks receive caller-supplied `EvaluationContext`.

## 8. Capability Request and Grant

```text
OperationRef
- namespace: non-empty string
- name: non-empty string
```

Operation identifiers are provider-neutral names such as `browser/read`; they are not Cedar/MCP/provider policy expressions.

```text
CapabilityRequest
- id: CapabilityRequestId
- principal: PrincipalRef
- operation: OperationRef
- target: ResourceRef
- scope: Scope
- temporal?: TemporalValidity
- requested_by: ActorId
- identity_assertion?: IdentityAssertionRef
- reason?: string

CapabilityGrant
- id: CapabilityGrantId
- principal: PrincipalRef
- operation: OperationRef
- target: ResourceRef
- scope: Scope
- temporal?: TemporalValidity
- issued_by: ActorId
- parent_grant?: CapabilityGrantId
- delegation_depth?: u16
```

Rules:
- request and grant are non-interchangeable types/IDs;
- no implicit request -> grant conversion;
- request identity assertion principal must match the requested principal when present;
- a delegation reference requires a parent grant and depth greater than zero;
- structural validation does not imply authorization;
- `reason` is free-form non-authoritative request metadata; v1 does not infer permission, approval or identity from its text;
- parent existence/subset/revocation is ECR-003/ECR-031.

## 9. Information Classification

```text
InformationClass
- public
- private
- sensitive
- secret
- unknown

InformationClassification
- class: InformationClass
- policy_tags: list<InformationPolicyTag>

InformationPolicyTag
- namespace: non-empty string
- name: non-empty string
```

Rules:
- classification grants no authority;
- unknown is conservative, not public;
- tags are data, not executable policy expressions;
- later policy owns joins, inheritance and declassification.

## 10. Information References

The base reference exists before `InformationUse` because Fact lineage depends on it:

```text
InformationRef
- observation(ObservationId)
- fact(FactId)
- artifact(ArtifactId)
- action_parameter(ActionParameterRef)
```

`action_parameter` becomes meaningful with the Phase 7 action binding shape; this task-order detail does not change the wire contract.

## 11. Evidence Reference

```text
EvidenceRef
- id: EvidenceId
- kind: EvidenceKind
- artifact?: ArtifactId
- observation?: ObservationId
- receipt?: ReceiptId
- external_ref?: non-empty string
- content_digest?: ContentDigest
- as_of?: EpochMillis

EvidenceKind
- observation
- artifact
- structured_tool_result
- network_receipt
- external_state
- computation
- model_judgment
- other
```

Rules:
- evidence contains stable references, not arbitrary evidence blobs;
- `EvidenceRef.id` is stable evidence identity;
- `external_ref`, when present, is opaque/non-authoritative and non-empty;
- evidence kind, a label, or presence of a digest proves nothing by itself;
- immutable capture digest/as-of metadata supports later verifier policy.

## 12. Observation

```text
ObservationPayloadRef
- artifact(ArtifactId)
- evidence(EvidenceId)
- resource(ResourceId)
- external_ref(non-empty string)
```

`external_ref` is opaque provider/storage metadata. It grants no access and is not parsed as authority.

```text
Observation
- id: ObservationId
- actor: ActorId
- origin: Origin
- observed_at?: EpochMillis
- subject: ResourceRef
- payload: ObservationPayloadRef
- classification: InformationClassification
- evidence: list<EvidenceRef>
```

An Observation records what was seen/retrieved; it is not universal truth, permission or independent verification.

## 13. Fact

```text
FactValue
- text(string)
- boolean(bool)
- integer(I-JSON-safe i64)
- decimal(canonical decimal string)
- resource(ResourceId)
- artifact(ArtifactId)
```

Canonical decimal strings:
- optional leading `-`;
- one or more decimal digits;
- optional `.` followed by one or more digits;
- no exponent or leading `+`;
- no redundant leading zero except `0` / `0.x`;
- `-0` and negative-zero spellings are rejected.

```text
Fact
- id: FactId
- subject: ResourceRef
- predicate: non-empty string
- value: FactValue
- provenance: Provenance
- classification: InformationClassification
- freshness: FreshnessAssessment
- dispute: DisputeState
- evidence: list<EvidenceRef>
- derived_from: list<InformationRef>
```

`Provenance`:
- `user_provided`
- `observed_web`
- `observed_local`
- `retrieved`
- `tool_provided`
- `model_inferred`
- `system_derived`

`DisputeState`:
- `undisputed`
- `contradicted`
- `disputed`
- `inconclusive`
- `unknown`

There is no `Fact.verified` truth flag. Verification truth is represented only by `VerificationReceipt` records targeting the Fact/claim. A model-inferred Fact remains model-inferred after verification.

`FactAssessment` is an API construction helper only; canonical Fact JSON remains flat with `provenance`, `freshness` and `dispute` fields.

## 14. Freshness Assessment

```text
FreshnessState
- current
- stale
- unknown

FreshnessBasisKind
- observed_at
- retrieved_at
- published_at
- effective_at
- source_reported
- other

FreshnessAssessment
- state: FreshnessState
- assessed_at?: EpochMillis
- basis_kind?: FreshnessBasisKind
- basis_time?: EpochMillis
- basis_evidence?: EvidenceId
```

Rules:
- `basis_kind` and `basis_time` are either both absent or both present;
- `basis_evidence` may independently identify support;
- freshness does not change provenance or verification;
- source-reported timestamps are not automatically trusted.

## 15. Artifact

```text
LineageRef
- observation(ObservationId)
- fact(FactId)
- artifact(ArtifactId)
```

Lineage uses stable IDs, never locators or display labels. Cycle/graph policy is downstream.

```text
ArtifactRef
- id: ArtifactId
- kind: ArtifactKind
- media_type?: non-empty string
- logical_name?: non-empty string
- classification: InformationClassification
- content_digest?: ContentDigest
- byte_size_decimal?: canonical non-negative decimal integer string
- storage_locator?: non-empty string
- lineage: list<LineageRef>

ArtifactKind
- file
- document
- image
- structured_data
- model_output
- browser_snapshot
- network_capture
- other
```

`byte_size_decimal` is `0` or a non-zero digit followed by digits. Empty, signed, negative, fractional and redundant-leading-zero forms are invalid. Storage locator and descriptive names remain non-authoritative.

## 16. Information Use / Disclosure Intent

```text
InformationUseKind
- local_compute
- model_context
- persist
- log_or_diagnostic
- external_disclosure
- remote_provider
- other

InformationUse
- sources: non-empty list<InformationRef>
- kind: InformationUseKind
- destination?: ResourceRef
- destination_origin?: WebOrigin
- declared_output_classification?: InformationClassification
```

Rules:
- use declaration is not authorization;
- remote/external use can name destination/origin;
- read authority over source A plus write authority at B does not imply allowed A -> B flow;
- ECR-003 owns source-to-sink policy and declassification.

## 17. Digest Types

Generic metadata:

```text
ContentDigest
- algorithm: string
- hex: string
```

A `ContentDigest` is metadata and is not automatically an authenticity/security digest.

Security binding:

```text
SecurityDigest
- algorithm: SecurityDigestAlgorithm
- hex: string

SecurityDigestAlgorithm
- sha256

ActionDigest(SecurityDigest)
```

`ActionDigest` v1 is SHA-256 over:

```text
UTF8("ecra/action-intent/v1\0") || RFC8785_JCS(Versioned<ActionIntent>)
```

The canonical ActionIntent contains every security-relevant field and excludes any derived digest field.

## 18. Action Parameter Binding

```text
ActionParametersRef
- none
- bound_artifact
    - artifact: ArtifactId
    - binding_digest: SecurityDigest
- bound_external
    - external_ref: non-empty string
    - binding_digest: SecurityDigest
```

Rules:
- every non-empty parameter set carries a SecurityDigest;
- in v1 SecurityDigest permits SHA-256 only;
- ArtifactId/external_ref are references only and do not grant access;
- a later executor that materializes parameters must verify the binding digest before use; that I/O behavior is outside ECR-001;
- ActionDigest binds the serialized ActionParametersRef including its digest;
- trusted v1 objects do not silently embed unconstrained parameter JSON.

```text
ActionParameterRef
- action: ActionId
- path: non-empty opaque string
```

`path` is descriptive lineage/addressing metadata, not authority or provider policy syntax.

## 19. Action Intent

```text
ActionIntent
- id: ActionId
- actor: ActorId
- principal?: PrincipalRef
- identity_assertion?: IdentityAssertionRef
- operation: OperationRef
- target: ResourceRef
- scope: Scope
- parameters: ActionParametersRef
- information_use: list<InformationUse>
- effect: EffectProfile
- idempotency: IdempotencySpec
- retry: RetryClass
- created_at?: EpochMillis
- correlation_id?: non-empty string
```

Rules:
- `operation` is a requested operation, not a CapabilityGrant;
- principal and identity assertion, when both present, must name the same PrincipalId;
- correlation identity is metadata and cannot authorize anything;
- `ActionSemantics` is only an API construction helper grouping effect/idempotency/retry validation; canonical JSON remains flat with the three fields above.

## 20. Effect / Idempotency / Retry

```text
MutationDomain
- none
- local
- external
- unknown

Reversibility
- not_applicable
- reversible
- conditional
- irreversible
- unknown

EffectProfile
- mutation: MutationDomain
- reversibility: Reversibility
```

Effect rules:
- `mutation=none` requires `reversibility=not_applicable`;
- `mutation=local|external` requires reversibility other than `not_applicable`;
- `mutation=unknown` requires `reversibility=unknown`.

```text
IdempotencyClass
- naturally_idempotent
- idempotent_with_key
- non_idempotent
- unknown

IdempotencySpec
- class: IdempotencyClass
- key_ref?: string
```

Idempotency rules:
- `naturally_idempotent`, `non_idempotent` and `unknown` must not carry `key_ref`;
- `idempotent_with_key` requires a non-empty `key_ref`.

```text
RetryClass
- safe
- requires_same_idempotency_key
- requires_external_reconciliation
- never_blind_retry
```

Retry matrix:
- `safe` only with `naturally_idempotent` and mutation other than `unknown`;
- `requires_same_idempotency_key` only with `idempotent_with_key` and mutation other than `unknown`;
- `requires_external_reconciliation` only when mutation is `external` or `unknown`;
- `never_blind_retry` is allowed for every otherwise structurally valid combination;
- non-idempotent/unknown idempotency never pairs with `safe` or `requires_same_idempotency_key`.

Reversibility never upgrades retry safety. UNKNOWN never normalizes to permissive semantics.

## 21. Immutable Action Reference and Attempts

```text
ActionRef
- id: ActionId
- digest: ActionDigest
```

Rules:
- ActionRef binds ID and exact canonical ActionIntent content;
- same ActionId with different security-relevant fields is a mismatch;
- later approvals/authorization decisions/receipts bind ActionRef, not ActionId alone.

```text
ActionAttemptRef
- id: ActionAttemptId
- action: ActionRef
```

One ActionIntent may later have multiple attempts. ECR-001 validates reference shape/binding only; ECR-002 owns creation, lifecycle, retry and reconciliation orchestration.

## 22. Action Receipt

```text
ErrorSummary
- code: non-empty string
- message?: non-empty string
```

ErrorSummary is bounded executor diagnostic metadata, not a DomainError, capability or verification result.

```text
ActionReceipt
- id: ReceiptId
- attempt: ActionAttemptRef
- executor_actor: ActorId
- started_at?: EpochMillis
- completed_at?: EpochMillis
- outcome: ActionOutcome
- evidence: list<EvidenceRef>
- external_reference?: non-empty string
- error?: ErrorSummary

ActionOutcome
- executor_observed_success
- executor_observed_failure
- unknown
```

Rules:
- receipt is executor-known evidence, not verification;
- exact ActionRef + ActionAttemptId are bound;
- when both times exist, `completed_at >= started_at`;
- UNKNOWN remains UNKNOWN;
- external references and diagnostic text are non-authoritative.

## 23. Verification Receipt

```text
ClaimRef
- namespace: non-empty string
- reference: non-empty string
```

ClaimRef is opaque structured target metadata; it is not policy syntax or evidence of truth.

```text
VerificationTarget
- action(ActionRef)
- action_attempt(ActionAttemptRef)
- receipt(ReceiptId)
- fact(FactId)
- artifact(ArtifactId)
- claim(ClaimRef)

VerificationMethod
- structured_external_state
- api_or_tool_result
- network_receipt
- artifact_validation
- dom_or_accessibility_state
- deterministic_computation
- independent_model_judgment
- other

VerificationOutcome
- verified
- rejected
- inconclusive
- not_evaluated
```

```text
VerificationReceipt
- id: VerificationId
- verifier: ActorId
- verifier_principal?: PrincipalRef
- target: VerificationTarget
- method: VerificationMethod
- evidence: list<EvidenceRef>
- outcome: VerificationOutcome
- evaluated_at?: EpochMillis
- notes?: non-empty string
```

Rules:
- `verified`, `rejected` and `inconclusive` require at least one EvidenceRef;
- `not_evaluated` may carry an empty evidence list;
- VerificationReceipt is the authoritative verification record;
- it does not mutate Fact provenance/classification/freshness;
- notes are non-authoritative metadata;
- ECR-004 later owns evidence sufficiency and independence policy.

## 24. Machine Error Contract

The exact v1 `ErrorCategory` set is:

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

The exact v1 `ErrorCode` set is:

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

Code-to-category mapping is defined by `ErrorCode::category()` and tested exhaustively. Display text is not an API contract. Detailed conceptual failures such as identity-reference, information-flow-shape or action-reference errors intentionally map into the broader machine categories above rather than introducing unimplemented category names.

## 25. Fixture Storage vs Wire Contract

The public persisted/interchange contract remains `Versioned<T>`.

Repository semantic fixtures under `contracts/ecra-domain-v1/{valid,invalid}/` may store the inner `T` body. The fixture runner pairs each body with its declared type/schema, constructs and round-trips `Versioned<T>`, and separately exercises full-envelope compatibility/strict-field cases.

This is a fixture-storage convention only. Adapters, persistence, external interchange and ActionDigest canonical security inputs must not omit the version envelope where v1 requires it.

## 26. State / Enforcement Ownership

```text
ECR-001: canonical zero-I/O value objects, references, structural invariants and security binding.
ECR-002: RunState, ActionAttempt lifecycle, budgets/cancellation, append-only persistence/integrity chain.
ECR-031: authentication assertions, trust roots, key lifecycle/revocation, protected sensitive-storage envelope.
ECR-003: capability narrowing, authorization decisions/leases, source-to-sink disclosure/declassification, approvals and secrets.
ECR-004: independent verification orchestration, evidence sufficiency and reconciliation/UNKNOWN resolution.
```

This boundary is normative: ECR-001 must not become an orchestrator, authentication system, policy engine, runtime executor, persistence layer or verifier service.