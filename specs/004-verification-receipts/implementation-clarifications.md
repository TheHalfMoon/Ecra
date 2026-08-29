# ECR-004 Implementation Clarifications

## IC-001 — Read-only canonical EvidenceRef accessors

### Discovery

ECR-004 decision-grade verification must inspect the canonical ECR-001 evidence binding already present in `EvidenceRef`: optional artifact, observation, receipt, external reference, content digest, and as-of time.

Live ECR-001 currently keeps those fields private and exposes public accessors only for `id()` and `kind()`. ECR-004 must not recover the hidden fields by serializing/parsing JSON, duplicating the wire type, or introducing a second evidence model.

### Canonical resolution

ECR-004 implementation MAY add minimal read-only accessors to `ecra_core::EvidenceRef` for the already-existing fields required by verification:

```text
artifact() -> Option<ArtifactId>
observation() -> Option<ObservationId>
receipt() -> Option<ReceiptId>
external_ref() -> Option<&str>
content_digest() -> Option<&ContentDigest>
as_of() -> Option<EpochMillis>
```

Exact borrowing/copy signatures may follow existing Rust conventions, but the semantic surface is read-only.

### Prohibited changes

IC-001 does NOT authorize:

- adding/removing/renaming `EvidenceRef` fields;
- changing ECR-001 JSON/wire shape or canonical bytes;
- changing validation semantics;
- changing provenance/freshness/dispute ownership;
- adding a verification flag to `EvidenceRef`, `Fact`, `ActionReceipt`, or other ECR-001 types;
- parsing serialized ECR-001 JSON inside ECR-004 to bypass the typed API;
- adding authority/policy semantics.

### Evidence requirement

The accessor commit must run the full ECR-001 regression suite and include tests proving serialization/canonical semantics are unchanged. It is a prerequisite to ECR-004 decision-grade evidence implementation.

### Task ordering

Execute IC-001 after the ECR-004 workspace/CI foundation is green and before T012. `tasks.md` owns it as T011A.

### Constitution impact

G1/G3/G5 improve: ECR-004 consumes the canonical ECR-001 evidence type directly instead of creating a competing representation. G2 is unchanged because accessors carry no authority. No other gate changes.