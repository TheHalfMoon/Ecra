# ECR-004 Verification & Reconciliation v1 Contract

This file is normative for ECR-004 v1. It extends no ECR-002 run-event wire type and does not redefine ECR-001 `VerificationReceipt`.

## 1. Canonical receipt ownership

A successful ECR-004 verifier emits exactly an ECR-001 `VerificationReceipt`.

```text
VerificationReceipt {
  id,
  verifier,
  verifier_principal?,
  target,
  method,
  evidence[],
  outcome,
  evaluated_at?,
  notes?
}
```

Requirements:
- `target` is an existing ECR-001 `VerificationTarget`;
- `verified`, `rejected`, and `inconclusive` require non-empty evidence;
- a receipt is independent verification truth only for its exact target;
- no receipt implies capability, approval, authorization, declassification, or provider execution authority.

## 2. VerificationRequestV1 JSON shape

```json
{
  "version": { "major": 1, "minor": 0 },
  "receipt_id": "<verification-id>",
  "verifier": "<actor-id>",
  "verifier_principal": null,
  "target": { "kind": "action_attempt", "value": {} },
  "method": "structured_external_state",
  "evidence": [],
  "proposed_outcome": "not_evaluated",
  "evaluated_at": null,
  "rule_id": "exact_rule_identifier",
  "notes": null
}
```

Unknown fields reject. `rule_id` is a bounded opaque rule identifier, never executable code or policy authority.

## 3. Decision-grade rules

A conclusive (`verified` or `rejected`) proposed outcome is accepted only if the rule declares its evidence requirements satisfied.

Minimum v1 generic checks:
- evidence list is non-empty;
- evidence IDs are unique within the request;
- mutable external evidence that the rule treats as decision-critical has `content_digest` or immutable artifact/snapshot binding;
- if the rule declares freshness required, evidence carries `as_of` and request carries `evaluated_at`;
- a single executor `ActionReceipt` that merely repeats its own success/failure claim cannot self-verify without independently evaluated state/invariant evidence;
- unsupported/ambiguous evidence shape yields `Inconclusive` or typed rejection, never optimistic `Verified`.

## 4. Aggregate contract

For one exact canonical target, partition accepted receipts by outcome after deterministic ID ordering.

```text
verified_count > 0 && rejected_count > 0 -> conflicted
verified_count > 0                       -> verified
rejected_count > 0                       -> rejected
inconclusive_count > 0                   -> inconclusive
otherwise                                -> absent
```

`not_evaluated` receipts remain inspectable but never satisfy a checkpoint.

## 5. VerificationCheckpointV1 JSON shape

```json
{
  "version": { "major": 1, "minor": 0 },
  "id": "<checkpoint-id>",
  "label": "critical-effect-confirmation",
  "requirements": [
    {
      "target": { "kind": "action_attempt", "value": {} },
      "accepted_states": ["verified"]
    }
  ]
}
```

Rules:
- non-empty requirements;
- exact duplicate targets reject;
- `absent` and `conflicted` cannot be accepted satisfying states;
- checkpoint evaluation is derived from receipts and never mutates them.

## 6. ReconciliationRecordV1 JSON shape

```json
{
  "version": { "major": 1, "minor": 0 },
  "id": "<reconciliation-id>",
  "run_id": "<run-id>",
  "attempt": {},
  "action": {},
  "outcome": "still_unknown",
  "verification_receipts": ["<verification-id>"],
  "reconciled_at": null,
  "notes": null
}
```

Rules:
- `attempt` must bind `action` exactly using ECR-001 validation;
- attempt must be the exact durable attempt in the supplied ECR-002 run state;
- support receipt IDs are unique and non-empty;
- support receipts must exist in the ECR-004 journal and be relevant to the exact attempt/action claim;
- `effect_confirmed` requires non-conflicted conclusive evidence of effect presence;
- `no_effect_confirmed` requires non-conflicted conclusive evidence of effect absence;
- `still_unknown` is mandatory for absent, insufficient, or conflicting evidence;
- no outcome creates `ActionReceipt`;
- no outcome removes the attempt from ECR-002 `unresolved_attempts`, mutates `PreparedAttemptState`, appends an ECR-002 event, resumes/completes the existing run, or schedules execution.

## 7. Retry disposition contract

Inputs:
- exact `ActionIntent`;
- exact durable prior attempt;
- latest deterministic reconciliation view.

Output is one closed safety classification:

```text
duplicate_retry_blocked
reconciliation_required
semantically_retryable
semantically_retryable_same_key
requires_explicit_nonblind_path
```

This output is not authorization. A caller must still pass later owning authorization/execution gates.

`semantically_retryable` and `semantically_retryable_same_key` mean only that the reconciliation evidence does not itself prove a duplicate effect and that ECR-001 semantics may permit a future **new-attempt proposal**. They MUST NOT be interpreted as permission to call ECR-002 blind-retry helpers for the unresolved prior attempt or as proof that the existing ECR-002 run is resumable.

## 8. ECR-002 compatibility contract

For every reconciliation outcome, ECR-004 v1 MUST preserve the supplied ECR-002 execution state exactly for reconciliation purposes:

```text
prior attempt remains prepared
prior attempt remains unreceipted when it was unreceipted
prior attempt remains unresolved when it was unresolved
RunState unresolved_attempts is unchanged
RunPhase is unchanged
no RunEvent is emitted
no ActionReceipt is synthesized
```

Tests must prove ECR-002 `RunResumed`, `ExecutionCompleted`, and blind-retry guards remain fail-closed for the unresolved prior attempt after ECR-004 records `effect_confirmed` or `no_effect_confirmed`.

A future explicitly versioned ECR-002 repair/resolution protocol may consume ECR-004 evidence. That protocol is outside ECR-004 v1 and MUST NOT be simulated by side effects, adapters, or undocumented state mutation.

## 9. Journal digest contract

Domain separator:

```text
ecra/verification-journal-entry/v1\0
```

Digest material is RFC 8785/JCS canonical JSON of:

```json
{
  "version": { "major": 1, "minor": 0 },
  "sequence": 1,
  "previous_digest": null,
  "body": {}
}
```

Then:

```text
entry_digest = SHA-256(domain_separator || canonical_material)
```

Sequence starts at 1 and increments by exactly 1. Genesis requires `previous_digest = null`; successors require exact prior digest. The digest chain is an integrity mechanism only; no hostile complete-store rewrite resistance is claimed without a protected anchor owned elsewhere.

## 10. Journal body variants

```json
{ "kind": "verification_receipt", "value": { "receipt": {} } }
{ "kind": "checkpoint_defined", "value": { "checkpoint": {} } }
{ "kind": "reconciliation_recorded", "value": { "record": {} } }
```

Unknown variant/field/version rejects.

## 11. Persistence contract

- append-only canonical journal entries;
- ordinary SQL `UPDATE`/`DELETE` of canonical journal rows must be rejected by the store API and protected by schema triggers where practical;
- indexes/projections are rebuildable and non-authoritative;
- duplicate verification/checkpoint/reconciliation IDs reject;
- competing appenders use expected-head compare-and-append semantics so at most one wins for one expected head;
- crash before commit leaves no partial canonical entry;
- corruption, sequence break, previous-digest mismatch, entry-digest mismatch, unknown newer schema, and malformed canonical JSON fail closed.

## 12. Resource limits

Initial v1 maxima:

```text
MAX_EVIDENCE_REFS_PER_REQUEST = 32
MAX_RECEIPTS_PER_TARGET = 256
MAX_CHECKPOINT_REQUIREMENTS = 128
MAX_RECONCILIATION_RECEIPTS = 64
MAX_RULE_ID_BYTES = 128
MAX_CHECKPOINT_LABEL_BYTES = 256
MAX_NOTES_BYTES = 4096
MAX_JOURNAL_ENTRY_BYTES = 65536
MAX_QUERY_ENTRIES = 4096
```

Implementations may choose stricter limits only through documented compatibility review; widening requires explicit contract/version review.

## 13. Error contract

Machine-readable categories must cover at least:

```text
validation
verification
evidence
aggregation
reconciliation
persistence
compatibility
resource_limit
```

Machine-readable codes must distinguish at least:

```text
unsupported_version
invalid_target
invalid_evidence
evidence_insufficient
self_attesting_receipt
verification_conflict
duplicate_id
attempt_binding_mismatch
reconciliation_unresolved
retry_blocked
journal_sequence_mismatch
journal_digest_mismatch
store_corrupt
resource_limit_exceeded
```

Display strings are diagnostic only; callers must not parse them for behavior.

## 14. Security boundary

ECR-004 v1 accepts explicit evidence metadata/records and local fixture stores only. It does not fetch URLs, invoke providers/models/tools, execute processes, read browser state, authorize disclosure, validate identity assertions, store raw secrets, resolve ECR-002 run state, or execute retries.