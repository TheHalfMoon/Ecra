# ECR-001 Free-Form Field Authority Audit

**Task:** T066  
**Requirement:** FR-054  
**Scope:** `crates/ecra-core` public v1 domain contract

## Rule

Free-form text is data or descriptive metadata only. No free-form value may be parsed to manufacture authentication, authorization, approval, scope widening, resource identity, verification, or executable policy. Authority-bearing semantics must come from the explicit typed structures that own them.

## Audited fields

| Type / field | Classification | Authoritative counterpart / rule |
|---|---|---|
| `Actor.label` | display metadata | `ActorId` + `ActorKind`; label cannot authenticate a `PrincipalId`. |
| `PurposeRef.{namespace,name}` | declared purpose metadata | Scope authority remains the explicit `ScopeConstraint<T>` dimensions. Purpose text grants nothing. |
| `ResourceRef.locator` | provider/display locator | `ResourceId` is stable resource identity; locator strings may alias or change. |
| `ArtifactRef.media_type` | descriptive format metadata | Does not identify an artifact or grant access. |
| `ArtifactRef.logical_name` | descriptive display metadata | `ArtifactId` remains identity. |
| `ArtifactRef.storage_locator` | opaque storage location metadata | `ArtifactId` remains identity; locator grants no storage authority. |
| `EvidenceRef.external_ref` | opaque provider/location metadata | `EvidenceId` + typed links remain evidence identity; external text proves nothing. |
| `ObservationPayloadRef::ExternalRef` | opaque payload location | It points outside the domain object; its content is not authority or verification. |
| `InformationPolicyTag.{namespace,name}` | opaque policy label data | A tag can be evaluated by later policy but cannot execute policy or self-authorize. |
| `ActionParametersRef::BoundExternal.external_ref` | opaque parameter location | `SecurityDigest` binds the referenced parameter content; the reference string is not trust. |
| `ActionParameterRef.path` | descriptive parameter address | It is lineage metadata and never permission/provider syntax. |
| `ActionIntent.correlation_id` when present | correlation metadata | It participates in the v1 ActionDigest as intent data but is not approval or authorization. |
| `ErrorSummary.{code,message}` | executor diagnostic metadata | It is neither `DomainError` authority nor independent verification. |
| `ActionReceipt.external_reference` | opaque executor/provider reference | Exact `ActionRef` + `ActionAttemptId` bind the receipt; provider text cannot verify it. |
| `VerificationReceipt.notes` | human/verifier notes | `VerificationTarget`, method, evidence and `VerificationOutcome` own verification semantics. |
| `ClaimRef.{namespace,reference}` | descriptive claim target identity | Claim text is not policy syntax and does not prove truth. |

## Intentionally not classified as free-form authority metadata

Some strings are domain data rather than descriptive metadata and therefore must not be conflated with this audit:

- `FactValue::Text` is information payload.
- `Fact.predicate` is claim data.
- canonical decimal/byte-size strings are deterministic numeric encodings.
- `OperationRef` is a structured operation identifier consumed by later authorization; it is not a display label and must not be treated as interchangeable with arbitrary provider text.
- digest hex is validated cryptographic representation, not free-form text.

These values still do not self-authorize, but their semantics are owned by their dedicated domain types rather than by the free-form metadata rule.

## Evidence

Rustdoc carries the repository-wide boundary in `crates/ecra-core/src/lib.rs` and type-level warnings remain on `Actor`, `ResourceRef`, `ArtifactRef`, `PurposeRef`, `InformationPolicyTag`, `ActionParametersRef`, `ActionParameterRef`, `ErrorSummary`, and `ClaimRef`.

`crates/ecra-core/tests/non_authoritative_metadata.rs` proves that authority-looking labels, purposes, locators, provider references, receipt external references and verification notes cannot replace stable typed identity, explicit scope, receipt binding or verification outcome.

No ECR-001 parser interprets these free-form values as permission syntax. ECR-003 owns authorization and information-flow policy; ECR-031 owns trust-root identity validity.
