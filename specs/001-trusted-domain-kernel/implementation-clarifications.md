# ECR-001 Implementation Clarifications

**Status:** FOLDED_INTO_PRIMARY_CONTRACT  
**Folded:** 2026-08-27 during Phase 12 / T077  
**Primary normative sources:** `data-model.md` and `contracts/domain-v1.md`

This file preserves the bounded implementation resolutions C1–C12 discovered during ECR-001 implementation. They were originally `NORMATIVE_FOR_ECR_001_V1` while the planning-era primary documents lagged implementation. Phase 12 T077 folded every resolution below into the primary data model and v1 contract.

If this historical record ever differs from the converged primary documents, `data-model.md` and `contracts/domain-v1.md` govern. Do not create a second wire/domain contract from this file.

## C1 — ObservationPayloadRef

`Observation.payload` is a reference, never an arbitrary embedded page/tool/model blob.

```text
ObservationPayloadRef
- artifact(ArtifactId)
- evidence(EvidenceId)
- resource(ResourceId)
- external_ref(non-empty string)
```

`external_ref` is opaque, non-authoritative provider/storage metadata. It grants no access and is not parsed for authority.

## C2 — FactValue

```text
FactValue
- text(string)
- boolean(bool)
- integer(I-JSON-safe i64)
- decimal(non-empty canonical decimal string)
- resource(ResourceId)
- artifact(ArtifactId)
```

Canonical decimal strings use optional leading `-`, one or more decimal digits, optional `.` plus one or more digits, no exponent, no leading `+`, no redundant leading zero except `0` / `0.x`, and no negative-zero spelling.

## C3 — LineageRef

```text
LineageRef
- observation(ObservationId)
- fact(FactId)
- artifact(ArtifactId)
```

Lineage uses stable IDs, not provider locators/display labels. Cycle/graph policy is downstream.

## C4 — EvidenceRef validation

`EvidenceRef.id` is stable evidence identity. Optional typed links remain references only. `external_ref`, when present, is non-empty and non-authoritative. Evidence kind/digest/source labels prove nothing by themselves.

## C5 — Freshness basis pairing

`basis_time` and `basis_kind` are both absent or both present. `basis_evidence` may additionally identify support. This is structural consistency only; source-reported time is not automatically trusted.

## C6 — Artifact byte size

`ArtifactRef.byte_size_decimal`, when present, is a canonical non-negative base-10 integer string (`0` or a non-zero digit followed by digits). Empty, signed, negative, fractional and redundant-leading-zero forms are invalid.

## C7 — Free-form metadata

Media/logical/storage metadata, predicates, tag names, correlation IDs and external references obey their primary-type non-empty rules and remain non-authoritative. `CapabilityRequest.reason` is also non-authoritative; ECR-001 v1 does not infer authorization, approval or identity from its text.

## C8 — InformationRef task ordering

`Fact.derived_from` depends on the base `InformationRef`, so the reference type exists before construction of `InformationUse`. This was a task/dependency-order correction only and does not create a second wire version.

## C9 — Action parameter binding

```text
ActionParametersRef
- none
- bound_artifact
    - artifact: ArtifactId
    - binding_digest: SecurityDigest
- bound_external
    - external_ref: non-empty string
    - binding_digest: SecurityDigest

ActionParameterRef
- action: ActionId
- path: non-empty opaque string
```

Every non-empty parameter reference carries a v1 SHA-256 SecurityDigest. References do not grant access. ActionDigest binds the serialized parameter reference including the binding digest. Later executors must verify materialized payloads; that I/O is outside ECR-001.

## C10 — Effect / idempotency / retry compatibility

Effect:
- mutation `none` requires reversibility `not_applicable`;
- `local`/`external` reject `not_applicable`;
- mutation `unknown` requires reversibility `unknown`.

Idempotency:
- naturally-idempotent/non-idempotent/unknown do not carry `key_ref`;
- idempotent-with-key requires non-empty `key_ref`.

Retry:
- `safe` only with naturally-idempotent and mutation != unknown;
- `requires_same_idempotency_key` only with idempotent-with-key and mutation != unknown;
- `requires_external_reconciliation` only with external/unknown mutation;
- `never_blind_retry` for any otherwise structurally valid combination;
- non-idempotent/unknown never pair with `safe` or `requires_same_idempotency_key`.

Reversibility does not upgrade retry safety.

## C11 — Receipt / verification bounded values

```text
ClaimRef
- namespace: non-empty string
- reference: non-empty string

ErrorSummary
- code: non-empty string
- message?: non-empty string
```

ActionReceipt enforces `completed_at >= started_at` when both exist and remains executor-known evidence only.

Verification evidence cardinality:
- verified/rejected/inconclusive require at least one EvidenceRef;
- not_evaluated may carry none;
- ECR-004 owns evidence sufficiency/independence policy.

## C12 — Fixture storage and versioned wire envelopes

Public persisted/interchange values remain `Versioned<T>` with explicit schema version. Repository semantic fixtures may store inner `T` bodies for readability only when the fixture runner supplies/verifies the v1 envelope and separately tests compatibility/strict-field cases.

This is not a wire exception. Adapters, persistence, external interchange and canonical security inputs must not omit required version envelopes.