# Data Model: ECR-004 Verification & Reconciliation

## 1. Canonical inherited types

ECR-004 does not redefine these ECR-001/ECR-002 concepts:

```text
VerificationId
VerificationReceipt
VerificationTarget
VerificationMethod
VerificationOutcome
EvidenceRef
ActionRef
ActionAttemptRef
ActionReceipt
ActorId
PrincipalRef
RunId
RunState
RetryClass
IdempotencyClass
```

`VerificationReceipt` remains the only canonical independent verification record. `ActionReceipt` remains executor-observed execution evidence.

## 2. ECR-004-specific typed IDs

### CheckpointId

Opaque non-nil UUID identity for a persisted `VerificationCheckpointV1`.

### ReconciliationId

Opaque non-nil UUID identity for a persisted `ReconciliationRecordV1`.

IDs are identity only. They are never capability, authority, freshness, ordering, or truth.

## 3. VerificationRequestV1

Construction input used to create a canonical ECR-001 `VerificationReceipt`.

```text
VerificationRequestV1
  version: 1.0
  receipt_id: VerificationId
  verifier: ActorId
  verifier_principal: PrincipalRef?
  target: VerificationTarget
  method: VerificationMethod
  evidence: [EvidenceRef]
  proposed_outcome: VerificationOutcome
  evaluated_at: EpochMillis?
  rule_id: bounded string
  notes: bounded string?
```

Invariants:
- one exact target;
- evidence count within v1 bound;
- `Verified`/`Rejected`/`Inconclusive` requires evidence;
- `rule_id` non-empty and bounded;
- notes non-empty when present and bounded;
- no capability/authorization/approval/declassification fields;
- output receipt target/method/evidence/outcome must exactly match validated request inputs.

## 4. DecisionGradeAssessmentV1

Derived validation result over the evidence supplied to a request.

```text
DecisionGradeAssessmentV1
  decision_grade: bool
  reason: DecisionGradeReasonV1
```

Closed reasons:

```text
sufficient_immutable_binding
not_required_for_nonconclusive_outcome
missing_immutable_binding
missing_freshness_basis
self_attesting_execution_receipt_only
unsupported_evidence_shape
```

This object is not verification truth and is not persisted as a competing outcome. It explains whether a conclusive proposed outcome may be emitted as a receipt.

## 5. VerificationAggregateViewV1

Derived deterministic view for one exact `VerificationTarget`.

```text
VerificationAggregateViewV1
  target: VerificationTarget
  state: AggregateVerificationStateV1
  receipt_ids: [VerificationId]
  verified_ids: [VerificationId]
  rejected_ids: [VerificationId]
  inconclusive_ids: [VerificationId]
  not_evaluated_ids: [VerificationId]
```

Closed aggregate state:

```text
absent
verified
rejected
inconclusive
conflicted
```

Ordering is deterministic by canonical typed ID bytes/text. Aggregation never deletes or rewrites receipts.

## 6. VerificationRequirementV1

One requirement inside a checkpoint.

```text
VerificationRequirementV1
  target: VerificationTarget
  accepted_states: non-empty bounded set of AggregateVerificationStateV1
```

`conflicted` and `absent` are not permitted as satisfying states in v1. A requirement normally accepts exactly `verified`; specialized negative checks may accept `rejected` only when the requirement semantics explicitly describe rejection as the desired evidence state.

## 7. VerificationCheckpointV1

```text
VerificationCheckpointV1
  version: 1.0
  id: CheckpointId
  label: bounded string
  requirements: [VerificationRequirementV1]
```

Invariants:
- label non-empty/bounded;
- requirements non-empty/bounded;
- duplicate exact targets rejected;
- no authority/policy/approval fields.

### CheckpointEvaluationV1

Derived only:

```text
CheckpointEvaluationV1
  checkpoint_id: CheckpointId
  satisfied: bool
  satisfied_targets: [VerificationTarget]
  unsatisfied_targets: [VerificationTarget]
  conflicted_targets: [VerificationTarget]
```

## 8. ReconciliationOutcomeV1

Closed enum:

```text
effect_confirmed
no_effect_confirmed
still_unknown
```

No `success` synonym exists because this record describes independently verified effect truth, not provider execution self-report.

## 9. ReconciliationRecordV1

```text
ReconciliationRecordV1
  version: 1.0
  id: ReconciliationId
  run_id: RunId
  attempt: ActionAttemptRef
  action: ActionRef
  outcome: ReconciliationOutcomeV1
  verification_receipts: non-empty [VerificationId]
  reconciled_at: EpochMillis?
  notes: bounded string?
```

Invariants:
- `attempt.action_ref == action` exactly;
- attempt must exist in the supplied ECR-002 `RunState` for `run_id`;
- supporting receipt IDs must resolve to receipts whose target is the exact attempt/action/receipt evidence relevant to the reconciliation rule;
- `effect_confirmed` requires a non-conflicted conclusive verification basis;
- `no_effect_confirmed` requires explicit evidence of no effect; mere absence of provider receipt/evidence is insufficient;
- `still_unknown` is permitted for insufficient/conflicted evidence and remains blocking;
- append-only: a later record does not mutate an older record.

## 10. RetryDispositionV1

Derived safety classification, not authorization:

```text
duplicate_retry_blocked
reconciliation_required
semantically_retryable
semantically_retryable_same_key
requires_explicit_nonblind_path
```

Inputs:
- exact ECR-001 `ActionIntent` semantics;
- exact prior `ActionAttemptRef`;
- ECR-002 durable attempt state;
- latest deterministic reconciliation view.

Rules:
- `effect_confirmed` -> `duplicate_retry_blocked`;
- `still_unknown` or no reconciliation -> `reconciliation_required`;
- `no_effect_confirmed` + `RetryClass::Safe` -> `semantically_retryable`;
- `no_effect_confirmed` + `RequiresSameIdempotencyKey` -> `semantically_retryable_same_key` only for exact same key binding;
- `RequiresExternalReconciliation` or `NeverBlindRetry` never becomes authorization; a caller still needs the owning execution/authorization path.

## 11. VerificationJournalEntryV1

Strict append-only envelope:

```text
VerificationJournalEntryV1
  version: 1.0
  sequence: positive bounded integer
  previous_digest: VerificationJournalDigest?
  body: VerificationJournalBodyV1
  entry_digest: VerificationJournalDigest
```

Body variants:

```text
verification_receipt { receipt: VerificationReceipt }
checkpoint_defined    { checkpoint: VerificationCheckpointV1 }
reconciliation_recorded { record: ReconciliationRecordV1 }
```

The digest is domain-separated SHA-256 over canonical versioned material excluding `entry_digest` itself and including sequence/previous digest/body.

Claim boundary: this is an integrity chain under ordinary local assumptions, not hostile-tamper resistance against a complete-store rewriter.

## 12. Persistence model

ECR-004 owns a separate SQLite journal store; it does not add ECR-002 `RunEvent` variants.

Minimum logical schema:

```text
verification_meta
  schema_version INTEGER

verification_journal
  sequence INTEGER PRIMARY KEY
  entry_json TEXT NOT NULL
  entry_digest TEXT NOT NULL UNIQUE

verification_receipt_index
  verification_id TEXT PRIMARY KEY
  sequence INTEGER NOT NULL
  target_key TEXT NOT NULL

checkpoint_index
  checkpoint_id TEXT PRIMARY KEY
  sequence INTEGER NOT NULL

reconciliation_index
  reconciliation_id TEXT PRIMARY KEY
  run_id TEXT NOT NULL
  attempt_id TEXT NOT NULL
  sequence INTEGER NOT NULL
```

Indexes are projections. Canonical journal entries are authoritative for ECR-004 persisted truth and projections must be rebuildable.

## 13. Versioning and migration

- store schema begins at v1;
- wire records use strict v1 major/minor compatibility rules aligned with repository conventions;
- unsupported newer store/wire versions fail closed before mutation;
- v0/empty-store initialization is transactional;
- failed migration leaves original user version and journal bytes unchanged.

## 14. Bounds to freeze in implementation planning

Recommended initial v1 ceilings, subject to exact implementation fixture validation:

```text
evidence refs per verification request      32
receipts aggregated per target             256
checkpoint requirements                    128
verification IDs per reconciliation record  64
rule_id UTF-8 bytes                         128
label UTF-8 bytes                           256
notes UTF-8 bytes                          4096
journal entry JSON bytes                  65536
entries materialized by one query          4096
```

All bounds are fail-closed and checked before expensive processing where practical.