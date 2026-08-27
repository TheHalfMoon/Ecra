# ECR-001 Implementation Clarifications

**Status:** NORMATIVE_FOR_ECR_001_V1  
**Reason:** implementation review found named value objects whose wire shapes were not fully expanded in `data-model.md`. These clarifications narrow the contract; they do not widen ECR-001 scope or add runtime behavior. They MUST be folded into the primary contract/data-model during final convergence before `CLOSED_CANONICAL`.

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

Canonical decimal strings use optional leading `-`, one or more decimal digits, optional `.` followed by one or more digits, no exponent, no leading `+`, and no redundant leading zero except `0` / `0.x`. `-0` and negative zero spellings are rejected. Rich structured values belong to a later version rather than silently embedding unconstrained JSON in the trusted v1 contract.

## C3 — LineageRef

Artifact lineage is a stable-ID relation, not a locator or display label:

```text
LineageRef
- observation(ObservationId)
- fact(FactId)
- artifact(ArtifactId)
```

ECR-001 MUST reject empty/self-contradictory provider text as lineage authority. Cycles and graph policy are downstream concerns.

## C4 — EvidenceRef validation

`EvidenceRef.id` is the stable evidence identity. Optional typed links remain references only. When `external_ref` is present it MUST be non-empty. Evidence kind, presence of a digest, or source label proves nothing by itself.

## C5 — Freshness basis pairing

`basis_time` and `basis_kind` are either both absent or both present. `basis_evidence` may additionally identify supporting evidence. This is structural consistency only; source-reported times are not trusted automatically.

## C6 — Artifact byte size

`ArtifactRef.byte_size_decimal`, when present, is a canonical non-negative base-10 integer string (`0` or a non-zero digit followed by digits). This avoids I-JSON precision limits for large artifacts while keeping deterministic representation. Empty, signed, negative, fractional and redundant-leading-zero forms are invalid.

## C7 — Free-form metadata

Optional `media_type`, `logical_name`, `storage_locator`, predicates, tag namespace/name and external references MUST be non-empty when present/required. They remain non-authoritative metadata and MUST NOT be parsed into capabilities.
