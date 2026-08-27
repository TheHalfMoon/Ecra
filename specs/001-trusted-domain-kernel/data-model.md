# Data Model: Trusted Domain Kernel

**Feature:** ECR-001  
**Status:** REVISED_PLAN_READY_INPUT  
**Constitution:** v1.1.0

This is the normative conceptual data model for Ecra's first trusted-core contract. Exact Rust field/module names may differ only when the implementation plan records an equivalent mapping and contract fixtures preserve the semantics below.

ECR-001 defines value objects and validation only. It deliberately does not authenticate identities, authorize actions/disclosures, persist runs, execute attempts, or verify outcomes.

## 1. Version Envelope

```text
Versioned<T>
- schema_version: SchemaVersion
- value: T
```

```text
SchemaVersion
- major: u16
- minor: u16
```

Rules:
- v1 supports major `1` only;
- unsupported major/newer unsupported minor fails typed compatibility handling;
- security-sensitive objects reject undocumented fields;
- parsing never silently upgrades authority or information-flow semantics.

## 2. Strong Identifiers

Normative JSON representation: UUID string. Each is a different Rust newtype.

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
- ID form/name/prefix does not encode permission/trust;
- generation is caller-owned and requires no core randomness/clock.

## 3. Actor vs Principal / Identity Assertion Reference

```text
Actor
- id: ActorId
- kind: ActorKind
- label?: string
```

`ActorKind`:
- `human`
- `agent`
- `system`

```text
PrincipalRef
- id: PrincipalId
```

```text
IdentityAssertionRef
- id: IdentityAssertionId
- principal: PrincipalId
```

Rules:
- Actor is attribution/runtime participation, not proof of authentication;
- Actor label is non-authoritative;
- a PrincipalRef does not claim its assertion is valid/current;
- ECR-031 validates identity assertions/trust roots/on-behalf-of relationships;
- downstream stores/runs must reject conflicting ActorKind definitions for the same ActorId.

## 4. Origin

```text
Origin
- kind: OriginKind
- detail: OriginDetail
```

`OriginKind`:
- `user_input`
- `web`
- `local`
- `retrieval`
- `tool`
- `model`
- `memory`
- `system_policy`

```text
WebOrigin
- scheme: string
- host: string
- port?: u16
- opaque: bool
```

Rules:
- full URL/path is not origin;
- origin is provenance/security context, not instruction class;
- origin never grants authority;
- opaque origins must not be normalized into a fake web tuple.

## 5. Resource Identity

```text
ResourceRef
- id: ResourceId
- kind: ResourceKind
- locator?: string
- origin?: WebOrigin
```

Initial `ResourceKind`:
- `web_resource`
- `local_resource`
- `workspace_resource`
- `tool_resource`
- `artifact`
- `abstract`

Rules:
- `id` is the Ecra stable reference used for joins;
- `locator` is descriptive/provider addressing metadata and non-authoritative;
- provider-specific canonical identity/alias resolution is later work;
- policy must not infer equivalence/authority from locator string text alone.

## 6. Explicit Scope Algebra

Every security-relevant dimension uses an explicit constraint:

```text
ScopeConstraint<T>
- not_applicable
- exact(T)
- one_of(list<T>)
- any_explicit
```

Rules:
- `one_of` must be non-empty and deduplicated/canonicalized as specified by implementation contract;
- missing/empty never means ANY;
- `any_explicit` is the only unrestricted representation for that dimension;
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
```

```text
PurposeRef
- namespace: string
- name: string
```

`PurposeRef` is structured metadata used by later policy; it grants nothing by itself.

ECR-003 owns subset/intersection/narrowing semantics.

## 7. Time Values

```text
EpochMillis(i64)
```

Must remain in I-JSON exact integer range.

```text
EvaluationContext
- now: EpochMillis
```

```text
TemporalValidity
- not_before?: EpochMillis
- expires_at?: EpochMillis
```

Rule: when both exist, `not_before <= expires_at`. The core never reads the system clock.

## 8. Capability Request and Grant

```text
OperationRef
- namespace: string
- name: string
```

Example: `browser/read`; no Cedar/protocol/provider expression syntax.

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
```

```text
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
- no implicit request→grant conversion;
- structural validation does not imply authorization;
- parent existence/subset/revocation is ECR-003/ECR-031.

## 9. Information Classification

```text
InformationClass
- public
- private
- sensitive
- secret
- unknown
```

```text
InformationClassification
- class: InformationClass
- policy_tags: list<InformationPolicyTag>
```

```text
InformationPolicyTag
- namespace: string
- name: string
```

Rules:
- classification grants no authority;
- unknown is conservative, not public;
- tags are data, not executable policy expressions;
- later policy owns joins/inheritance/declassification.

## 10. Observation

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

An Observation records what was seen/retrieved, not universal truth or permission.

## 11. Fact

```text
Fact
- id: FactId
- subject: ResourceRef
- predicate: string
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

**There is no `Fact.verified` truth flag.** Verification truth is represented by VerificationReceipt records that target the Fact/claim.

## 12. Freshness Assessment

```text
FreshnessState
- current
- stale
- unknown
```

```text
FreshnessBasisKind
- observed_at
- retrieved_at
- published_at
- effective_at
- source_reported
- other
```

```text
FreshnessAssessment
- state: FreshnessState
- assessed_at?: EpochMillis
- basis_kind?: FreshnessBasisKind
- basis_time?: EpochMillis
- basis_evidence?: EvidenceId
```

Rules:
- freshness does not change provenance or verification;
- source-reported timestamps are not automatically trusted;
- current/stale must be explainable when basis is known.

## 13. Information References and Use / Disclosure Intent

```text
InformationRef
- observation(ObservationId)
- fact(FactId)
- artifact(ArtifactId)
- action_parameter(ActionParameterRef)
```

```text
InformationUseKind
- local_compute
- model_context
- persist
- log_or_diagnostic
- external_disclosure
- remote_provider
- other
```

```text
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
- read authority over a source plus write authority at destination does not imply allowed flow;
- ECR-003 owns source-to-sink policy and declassification.

## 14. Evidence

```text
EvidenceRef
- id: EvidenceId
- kind: EvidenceKind
- artifact?: ArtifactId
- observation?: ObservationId
- receipt?: ReceiptId
- external_ref?: string
- content_digest?: ContentDigest
- as_of?: EpochMillis
```

`EvidenceKind`:
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
- evidence kind alone proves nothing;
- immutable capture/digest/as-of is supported for later decision-grade verifier policy.

## 15. Artifact

```text
ArtifactRef
- id: ArtifactId
- kind: ArtifactKind
- media_type?: string
- logical_name?: string
- classification: InformationClassification
- content_digest?: ContentDigest
- byte_size_decimal?: string
- storage_locator?: string
- lineage: list<LineageRef>
```

`ArtifactKind`:
- `file`
- `document`
- `image`
- `structured_data`
- `model_output`
- `browser_snapshot`
- `network_capture`
- `other`

`storage_locator` is non-authoritative opaque metadata to storage layers.

## 16. Digest Types

Generic metadata:

```text
ContentDigest
- algorithm: string
- hex: string
```

A ContentDigest is not automatically an authenticity/security digest.

Security action binding:

```text
SecurityDigest
- algorithm: SecurityDigestAlgorithm
- hex: string

SecurityDigestAlgorithm
- sha256
```

```text
ActionDigest(SecurityDigest)
```

`ActionDigest` v1 is SHA-256 over:

```text
UTF8("ecra/action-intent/v1\0") || JCS(Versioned<ActionIntent>)
```

where the canonical ActionIntent excludes any derived digest field itself. Exact fixtures define the byte domain.

## 17. Action Intent

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
- correlation_id?: string
```

`operation` is **not** a CapabilityGrant. Authorization occurs later.

## 18. Effect / Idempotency / Retry

```text
MutationDomain
- none
- local
- external
- unknown
```

```text
Reversibility
- not_applicable
- reversible
- conditional
- irreversible
- unknown
```

```text
EffectProfile
- mutation: MutationDomain
- reversibility: Reversibility
```

Selected invariants:
- `mutation=none` requires `reversibility=not_applicable`;
- mutating actions cannot use `not_applicable` reversibility;
- unknown never normalizes to non-mutating/reversible.

```text
IdempotencyClass
- naturally_idempotent
- idempotent_with_key
- non_idempotent
- unknown
```

```text
IdempotencySpec
- class: IdempotencyClass
- key_ref?: string
```

```text
RetryClass
- safe
- requires_same_idempotency_key
- requires_external_reconciliation
- never_blind_retry
```

Selected invariants:
- keyed idempotency requires key_ref;
- non-idempotent/unknown cannot pair with unconditional `safe` retry;
- conservative combinations are required for unknown/destructive cases.

## 19. Immutable Action Reference

```text
ActionRef
- id: ActionId
- digest: ActionDigest
```

Rules:
- ActionRef binds ID and exact canonical ActionIntent content;
- same ActionId with different security-relevant fields produces a digest mismatch;
- later approvals/authorization decisions/receipts bind ActionRef, not ActionId alone.

## 20. Execution Attempt Identity

```text
ActionAttemptRef
- id: ActionAttemptId
- action: ActionRef
```

ECR-001 validates/reference-shapes only. ECR-002 owns attempt creation/state/lifecycle/retries.

## 21. Action Receipt

```text
ActionReceipt
- id: ReceiptId
- attempt: ActionAttemptRef
- executor_actor: ActorId
- started_at?: EpochMillis
- completed_at?: EpochMillis
- outcome: ActionOutcome
- evidence: list<EvidenceRef>
- external_reference?: string
- error?: ErrorSummary
```

`ActionOutcome`:
- `executor_observed_success`
- `executor_observed_failure`
- `unknown`

Rules:
- receipt is executor-known evidence, not verification;
- exact action digest + attempt are bound;
- completion >= start when both exist;
- UNKNOWN remains UNKNOWN.

## 22. Verification Receipt

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
- notes?: string
```

`VerificationTarget` can reference:
- ActionRef;
- ActionAttemptRef;
- ReceiptId;
- FactId;
- ArtifactId;
- typed/opaque ClaimRef.

`VerificationMethod`:
- `structured_external_state`
- `api_or_tool_result`
- `network_receipt`
- `artifact_validation`
- `dom_or_accessibility_state`
- `deterministic_computation`
- `independent_model_judgment`
- `other`

`VerificationOutcome`:
- `verified`
- `rejected`
- `inconclusive`
- `not_evaluated`

Rules:
- VerificationReceipt is the authoritative verification record;
- it does not mutate Fact provenance/classification/freshness;
- `not_evaluated` differs from `inconclusive`.

## 23. Error Categories

Machine-distinguishable categories/codes include:

```text
CompatibilityError
IdentifierError
IdentityReferenceError
OriginError
ResourceError
ScopeError
CapabilityError
TemporalError
InformationFlowShapeError
EvidenceError
DigestError
ActionSemanticError
ActionReferenceError
AttemptError
ReceiptError
VerificationError
CanonicalizationError
```

Display text is not an API contract.

## 24. State / Enforcement Ownership

```text
ECR-001: canonical zero-I/O value objects, references and structural invariants.
ECR-002: RunState, ActionAttempt lifecycle, budgets/cancellation, append-only persistence/integrity chain.
ECR-031: authentication assertions, trust roots, key lifecycle/revocation, protected sensitive-storage envelope.
ECR-003: capability narrowing, source-to-sink disclosure/declassification, approvals, secrets, immutable AuthorizationDecision/lease.
ECR-004: independent verification orchestration, evidence sufficiency, reconciliation/UNKNOWN resolution.
```

This boundary is normative: ECR-001 must not become an orchestrator or policy engine.
