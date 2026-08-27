# Data Model: Durable Run, Ledger & Budgets

**Feature:** ECR-002  
**Status:** PLAN_MODEL  
**Depends on:** ECR-001 canonical public types

This model adds durable execution concepts without redefining ECR-001 Actor, ActionRef/ActionAttemptRef/ActionReceipt, VerificationReceipt, ArtifactRef, ContentDigest, EpochMillis, or RunId semantics.

## 1. RunEventEnvelope

```text
RunEventEnvelope {
  schema_version: SchemaVersion,
  run_id: RunId,
  sequence: EventSequence,
  recorded_at: EpochMillis,
  previous_digest: Option<LedgerDigest>,
  event: RunEvent,
  event_digest: LedgerDigest
}
```

Rules:
- sequence starts at 1 and is contiguous per run;
- sequence is the ordering authority; recorded_at is audit data only;
- genesis requires no previous digest; every later event binds the exact previous digest;
- event digest is recomputed from versioned canonical material and must match;
- strict public parsing rejects unknown fields and unsupported versions.

## 2. EventSequence and BudgetAmount

```text
EventSequence: 1..=9_007_199_254_740_991
BudgetAmount:  0..=9_007_199_254_740_991
```

Both are strong integer wrappers. All arithmetic is checked. No floating-point budget/cost representation exists in v1.

## 3. LedgerDigest

```text
LedgerDigest {
  algorithm: sha256,
  hex: 64 lowercase hex chars
}
```

Domain separator:

```text
b"ecra/run-event/v1\0"
```

Digest material:

```json
{
  "schema_version":{"major":1,"minor":0},
  "run_id":"<RunId>",
  "sequence":1,
  "recorded_at":0,
  "previous_digest":null,
  "event":{"kind":"run_created","value":{}}
}
```

Exact event value varies by event kind. Canonical bytes use the ECR-001 RFC 8785 canonicalization boundary.

`LedgerDigest` is integrity-chain material only. It is not authentication, authorization, signature, verification, or hostile-rewriter protection.

## 4. RunPhase

```text
created
running
suspended
cancellation_requested
cancelled
failed
execution_completed
```

Terminal in v1:

```text
cancelled
failed
execution_completed
```

`execution_completed` means runtime execution completed; it does not mean independently verified.

## 5. SuspensionReason

```text
user_pause
budget_exhausted(BudgetDimension)
reconciliation_required(ActionAttemptRef)
cancellation_in_progress
runtime_interruption
other(code: bounded non-empty string)
```

The `other` code is state-machine/diagnostic data only and cannot encode authority.

## 6. RunState

Derived/rebuildable value:

```text
RunState {
  run_id: RunId,
  phase: RunPhase,
  actor: ActorId,
  budget: RunBudget,
  usage: BudgetUsage,
  prepared_attempts: ordered map<ActionAttemptId, PreparedAttemptState>,
  unresolved_attempts: ordered set<ActionAttemptId>,
  last_sequence: EventSequence,
  last_digest: LedgerDigest,
  suspension: Option<SuspensionReason>
}
```

`RunState` is computed solely from an accepted event stream. A serialized database projection is a cache, not independent truth.

## 7. RunBudget

```text
RunBudget {
  limits: non-empty unique list<BudgetLimit>
}

BudgetLimit {
  dimension: BudgetDimension,
  soft: Option<BudgetAmount>,
  hard: BudgetAmount
}
```

Rules:
- one limit per dimension;
- hard always explicit;
- soft <= hard;
- omitted dimension means ECR-002 has no configured limit for that dimension, not that authority/cost is unlimited;
- future provider slices must define mandatory dimensions before starting bounded work.

## 8. BudgetDimension

Exact v1 enum:

```text
active_wall_millis
steps
tool_calls
model_calls
input_tokens
output_tokens
cost_microunits
process_count
process_millis
output_bytes
network_requests
network_bytes
storage_bytes
recursion_depth
```

New dimensions require contract/version evolution. There is no free-form dimension.

## 9. BudgetUsage

```text
BudgetUsage: ordered map<BudgetDimension, BudgetAmount>
```

Computed from `resource_usage_recorded` events using checked addition.

## 10. PreparedAttemptState

```text
PreparedAttemptState {
  attempt: ActionAttemptRef,
  prepared_at_sequence: EventSequence,
  receipt: Option<ActionReceipt>,
  unresolved: bool
}
```

Rules:
- attempt is durably prepared before provider invocation;
- an attempt may receive at most one receipt in v1;
- receipt must bind the exact same ActionAttemptRef/ActionRef;
- after a recovery boundary, prepared-without-receipt becomes unresolved;
- missing receipt never implies success/failure.

## 11. RunEvent

Strict tagged enum:

```text
run_created
run_started
run_suspended
run_resumed
cancellation_requested
run_cancelled
run_failed
execution_completed
attempt_prepared
receipt_recorded
recovery_boundary
attempt_marked_unknown
reconciliation_requested
resource_usage_recorded
budget_soft_limit_reached
budget_exhausted
intervention_recorded
```

### RunCreated

```text
{
  actor: ActorId,
  budget: RunBudget
}
```

Only valid as sequence 1.

### RunStarted

Empty body. Transition created -> running.

### RunSuspended

```text
{ reason: SuspensionReason }
```

### RunResumed

Empty body. Only valid from a resumable suspended state with no blocking unresolved attempt or budget condition.

### CancellationRequested

```text
{ actor: ActorId }
```

Records attribution/request only.

### RunCancelled

Empty body. Terminal; requires cancellation_requested and no prepared-without-receipt attempt.

### RunFailed

```text
{ error: RunErrorSummary }
```

Runtime failure only; not verification rejection.

### ExecutionCompleted

Empty body. Requires no unresolved/prepared-without-receipt attempt and no hard-budget blocker.

### AttemptPrepared

```text
{ attempt: ActionAttemptRef }
```

Must commit before provider invocation.

### ReceiptRecorded

```text
{ receipt: ActionReceipt }
```

Exact attempt/action binding validated.

### RecoveryBoundary

```text
{ reason: process_restart | explicit_recovery }
```

The recovering runtime explicitly appends this after loading/validating history. Replay itself appends nothing.

### AttemptMarkedUnknown

```text
{
  attempt: ActionAttemptRef,
  cause: interrupted_before_receipt | provider_ambiguous | other
}
```

Executor ambiguity only; no VerificationReceipt synthesized.

### ReconciliationRequested

```text
{ attempt: ActionAttemptRef }
```

Durable hook for ECR-004.

### ResourceUsageRecorded

```text
{
  dimension: BudgetDimension,
  amount: BudgetAmount
}
```

### BudgetSoftLimitReached

```text
{
  dimension: BudgetDimension,
  soft_limit: BudgetAmount,
  cumulative_usage: BudgetAmount
}
```

Valid only on the first threshold crossing.

### BudgetExhausted

```text
{
  dimension: BudgetDimension,
  hard_limit: BudgetAmount,
  cumulative_usage: BudgetAmount
}
```

Transitions running -> suspended with a non-resumable v1 budget blocker.

### InterventionRecorded

```text
{
  actor: ActorId,
  kind: takeover | hand_back | pause_request | edit | denial | note,
  note: Option<bounded string>
}
```

Records attribution/context only. It is not authentication, approval or authorization.

## 12. RunErrorSummary

```text
RunErrorSummary {
  category: RunErrorCategory,
  code: RunErrorCode,
  message: Option<String>
}
```

The message is diagnostic/non-authoritative. Runtime branches on category/code.

## 13. Error taxonomy

Categories:

```text
compatibility
event
state
attempt
ledger
storage
migration
budget
archive
integrity
recovery
serialization
```

Codes are frozen by `contracts/run-ledger-v1.md`; unknown codes require version evolution rather than display-string parsing.

## 14. SQLite schema v1

### Metadata

Database schema version is `1`, checked before normal operations.

### `run_events`

```text
run_id TEXT NOT NULL
sequence INTEGER NOT NULL
event_digest TEXT NOT NULL
previous_digest TEXT NULL
event_json BLOB NOT NULL
PRIMARY KEY (run_id, sequence)
UNIQUE (run_id, event_digest)
```

`event_json` contains the full canonical envelope. Indexed columns are redundant validated fields for expected-head checks.

UPDATE/DELETE are rejected by triggers for ordinary application use.

### `run_heads`

```text
run_id TEXT PRIMARY KEY
last_sequence INTEGER NOT NULL
last_digest TEXT NOT NULL
phase TEXT NOT NULL
state_json BLOB NOT NULL
```

Derived/rebuildable projection only.

### `artifact_blobs`

```text
content_digest TEXT PRIMARY KEY
byte_size INTEGER NOT NULL
bytes BLOB NOT NULL
```

Digest/size validated before commit. ECR-001 `ArtifactRef` remains the domain identity.

## 15. Atomic append

```text
BEGIN IMMEDIATE
  read current run head
  require expected head matches
  validate next envelope/event/binding/budget/transition
  recompute ledger digest
  INSERT authoritative event
  reduce prior state + event
  UPSERT derived run head/state
COMMIT
```

A transaction contains no provider/browser/model/network/process call.

## 16. `.ecra` manifest v1

```text
EcraRunManifest {
  schema_version: SchemaVersion(1,0),
  run_id: RunId,
  event_count: EventSequence,
  head_digest: LedgerDigest,
  events: ordered list<ManifestEventEntry>,
  blobs: ordered list<ManifestBlobEntry>
}
```

`ManifestEventEntry`:

```text
sequence
path
ledger_digest
byte_size
content_digest
```

`ManifestBlobEntry`:

```text
path
content_digest
byte_size
```

All paths are canonical relative UTF-8 forward-slash paths.

## 17. Deterministic archive profile

Writer invariants:
- ZIP Stored only;
- fixed timestamp `1980-01-01T00:00:00`;
- fixed regular-file permission `0600`;
- no comments/encryption/symlinks/directory entries;
- sorted deterministic entry order;
- canonical JCS JSON.

Reader limits:

```text
MAX_ARCHIVE_ENTRIES          = 16_384
MAX_EVENT_COUNT              = 10_000
MAX_BLOB_COUNT               = 6_000
MAX_MANIFEST_BYTES           = 8 MiB
MAX_EVENT_ENTRY_BYTES        = 4 MiB
MAX_SINGLE_BLOB_BYTES        = 64 MiB
MAX_TOTAL_UNCOMPRESSED_BYTES = 512 MiB
MAX_PATH_BYTES               = 512
```

These are parser safety limits, not budget grants.

## 18. Migration model

Database schema version and event wire version are independent.

```text
DB schema v1 -> future version: transactional migration + fixed fixture
Event v1 -> future version: explicit compatible decoder/migration or fail closed
```

Migration never silently changes historical event meaning. Projections may be rebuilt; authoritative history is not compacted/deleted in ECR-002 v1.

## 19. Sensitive-data boundary

ECR-002 v1 proves durability using synthetic/non-sensitive state only. It does not define encryption-at-rest, key storage, authenticated ledger anchors, secret mediation or real-sensitive-state authorization. Those remain owned by ECR-031/ECR-003/ECR-025.
