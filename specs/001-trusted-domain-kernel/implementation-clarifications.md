# ECR-001 Implementation Clarifications

**Status:** NORMATIVE_FOR_ECR_001_V1  
**Reason:** implementation review found named value objects whose wire shapes were not fully expanded in `data-model.md`, plus task-order and cross-field ambiguities. These clarifications narrow the contract; they do not widen ECR-001 scope or add runtime behavior. They MUST be folded into the primary contract/data-model/tasks during final convergence before `CLOSED_CANONICAL`.

## C1 — ObservationPayloadRef

`Observation.payload` is a reference, never an arbitrary embedded page/tool/model blob.

```text
ObservationPayloadRef
- artifact(ArtifactId)
- evidence(EvidenceId)
- resource(ResourceId)
- external_ref(non-empty string)
```

`external_ref` is opaque, non-authoritative provider/storage metadata. It grants no access and MUST NOT be parsed for authority.

## C2 — FactValue

ECR-001 v1 deliberately supports a small deterministic value domain:

```text
FactValue
- text(string)
- boolean(bool)
- integer(I-JSON-safe i64)
- decimal(non-empty canonical decimal string)
- resource(ResourceId)
- artifact(ArtifactId)
```

Canonical decimal strings use optional leading `-`, one or more decimal digits, optional `.` followed by one or more digits, no exponent, no leading `+`, and no redundant leading zero except `0` / `0.x`. `-0` and negative-zero spellings are rejected. Rich structured values belong to a later version rather than silently embedding unconstrained JSON in the trusted v1 contract.

## C3 — LineageRef

Artifact lineage is a stable-ID relation, not a locator or display label:

```text
LineageRef
- observation(ObservationId)
- fact(FactId)
- artifact(ArtifactId)
```

Provider text is never lineage authority. Cycles and graph policy are downstream concerns.

## C4 — EvidenceRef validation

`EvidenceRef.id` is the stable evidence identity. Optional typed links remain references only. When `external_ref` is present it MUST be non-empty. Evidence kind, presence of a digest, or source label proves nothing by itself.

## C5 — Freshness basis pairing

`basis_time` and `basis_kind` are either both absent or both present. `basis_evidence` may additionally identify supporting evidence. This is structural consistency only; source-reported times are not trusted automatically.

## C6 — Artifact byte size

`ArtifactRef.byte_size_decimal`, when present, is a canonical non-negative base-10 integer string (`0` or a non-zero digit followed by digits). This avoids I-JSON precision limits for large artifacts while keeping deterministic representation. Empty, signed, negative, fractional and redundant-leading-zero forms are invalid.

## C7 — Free-form metadata

Optional `media_type`, `logical_name`, `storage_locator`, predicates, tag namespace/name, correlation IDs and external references MUST be non-empty when present/required. They remain non-authoritative metadata and MUST NOT be parsed into capabilities.

## C8 — InformationRef task ordering

`Fact.derived_from` normatively depends on `InformationRef`, so the base reference type is introduced with Phase 5 evidence/fact work rather than waiting for Phase 6. Phase 6/T039 remains responsible for `InformationUseKind`, `InformationUse`, source-to-sink declarations and their tests. This is a dependency-order correction only.

## C9 — Action parameter binding

`ActionDigest` MUST bind the exact parameter payload semantics. A locator or ArtifactId by itself is insufficient because referenced content can change while the identifier/locator remains the same.

ECR-001 v1 therefore uses:

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
- every non-empty parameter set carries a `SecurityDigest`; in v1 this is SHA-256 because SecurityDigest v1 permits only SHA-256;
- ArtifactId/external_ref are references only and do not grant access;
- the executor/provider that later materializes parameters MUST verify the binding digest before using the payload; that I/O behavior is outside ECR-001;
- `ActionDigest` binds the serialized ActionParametersRef, including its security digest;
- parameter payloads are not silently embedded as unconstrained JSON in trusted v1 objects.

For source-to-sink lineage involving an action parameter, v1 additionally defines:

```text
ActionParameterRef
- action: ActionId
- path: non-empty opaque string
```

`InformationRef` gains `action_parameter(ActionParameterRef)` during Phase 7. `path` is descriptive addressing metadata, not authority or provider policy syntax.

## C10 — Effect / idempotency / retry compatibility

The selected invariants in `data-model.md` are made executable as the following fail-closed v1 matrix.

### EffectProfile

- `mutation=none` requires `reversibility=not_applicable`.
- `mutation=local` or `external` requires reversibility other than `not_applicable`.
- `mutation=unknown` requires `reversibility=unknown`; unknown effect state is never represented as known reversible/non-mutating behavior.

### IdempotencySpec

- `naturally_idempotent` MUST NOT carry `key_ref`.
- `idempotent_with_key` MUST carry a non-empty `key_ref`.
- `non_idempotent` and `unknown` MUST NOT carry `key_ref`; a key string does not upgrade their semantics.

### RetryClass

- `safe` is permitted only with `naturally_idempotent` and a mutation domain other than `unknown`.
- `requires_same_idempotency_key` is permitted only with `idempotent_with_key` and a mutation domain other than `unknown`.
- `requires_external_reconciliation` is permitted only when mutation domain is `external` or `unknown`; it remains conservative and is not an authorization to retry.
- `never_blind_retry` is permitted for any structurally valid effect/idempotency combination.
- `non_idempotent` or `unknown` idempotency can never pair with `safe` or `requires_same_idempotency_key`.

Reversibility never upgrades retry safety: an irreversible action may still be naturally idempotent (for example a delete-like operation), while a reversible action may still be non-idempotent. These axes remain independent.
