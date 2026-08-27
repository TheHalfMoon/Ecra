# Contract: ECR-002 Run Ledger v1

**Feature:** ECR-002 — Durable Run, Ledger & Budgets  
**Contract version:** 1.0  
**Status:** NORMATIVE_PLANNING  
**Depends on:** ECR-001 canonical domain contract

This contract defines the public persisted/interchange semantics for the ECR-002 run ledger. It does not authorize execution, authenticate a principal, independently verify an outcome, or protect secrets at rest.

## 1. Public crate boundary

Planned crate:

```text
crates/ecra-run
```

`ecra-run` may depend on `ecra-core`; `ecra-core` must not depend on `ecra-run` or any SQLite/ZIP/runtime dependency.

Public capability families:

```text
RunEventEnvelope / RunEvent / RunState
RunBudget / BudgetLimit / BudgetDimension / BudgetAmount
LedgerDigest / EventSequence
RunReducer
RunStore / ExpectedRunHead
EcraArchiveWriter / EcraArchiveReader / EcraRunManifest
RunError / RunErrorCategory / RunErrorCode
```

Provider execution is deliberately absent. A later executor receives an already-durable `ActionAttemptRef` guard from this layer before invoking a provider.

## 2. Version envelope

Every persisted run event and archive manifest carries ECR-001 `SchemaVersion`.

Supported v1:

```json
{"major":1,"minor":0}
```

Rules:
- unsupported major -> `unsupported_major_version`;
- newer unsupported minor -> `unsupported_minor_version`;
- malformed/missing version -> `serialization_failed`;
- unknown fields in strict persisted values -> `serialization_failed`;
- no decoder may silently reinterpret a newer version as v1.

Database schema version and event/manifest wire versions are independent.

## 3. RunEventEnvelope wire shape

```json
{
  "schema_version":{"major":1,"minor":0},
  "run_id":"<RunId>",
  "sequence":1,
  "recorded_at":0,
  "previous_digest":null,
  "event":{"kind":"run_created","value":{...}},
  "event_digest":{"algorithm":"sha256","hex":"<64 lowercase hex>"}
}
```

Strict rules:
- sequence 1 is genesis and requires `previous_digest = null`;
- sequence n>1 requires `previous_digest` equal to event n-1 digest;
- sequence is authoritative order; timestamps never reorder events;
- sequence is in `1..=9_007_199_254_740_991`;
- `recorded_at` uses ECR-001 `EpochMillis` validation;
- event/body types reject unknown fields.

## 4. Ledger digest

Strong type:

```text
LedgerDigest != ContentDigest != ActionDigest != SecurityDigest
```

Domain separator:

```text
b"ecra/run-event/v1\0"
```

Digest preimage is the RFC 8785 canonical JSON bytes of exactly:

```json
{
  "schema_version":{"major":1,"minor":0},
  "run_id":"<RunId>",
  "sequence":1,
  "recorded_at":0,
  "previous_digest":null,
  "event":{"kind":"run_created","value":{...}}
}
```

Then:

```text
SHA-256(domain_separator || canonical_bytes)
```

`event_digest` itself is excluded from its own preimage. A fixed golden fixture must lock this byte/digest contract.

Integrity guarantee:
- chain continuity and inspected-content mutation are detectable;
- the digest is not a signature/MAC/authentication/authorization/verification proof;
- an attacker who can rewrite the whole store can recompute the chain;
- stronger authenticity awaits an ECR-031 protected anchor.

## 5. Run phases and transition matrix

Phases:

```text
created
running
suspended
cancellation_requested
cancelled
failed
execution_completed
```

Terminal:

```text
cancelled | failed | execution_completed
```

`execution_completed` is runtime execution completion only and never means `VERIFIED`.

Allowed phase-changing transitions:

| Event | From | To | Additional conditions |
|---|---|---|---|
| `run_created` | none | created | sequence 1 only |
| `run_started` | created | running | no unresolved attempt |
| `run_suspended` | running | suspended | explicit reason |
| `run_resumed` | suspended | running | suspension must be resumable; no unresolved attempt; no hard-budget blocker |
| `cancellation_requested` | created/running/suspended | cancellation_requested | records request only |
| `run_cancelled` | cancellation_requested | cancelled | no prepared-without-receipt attempt |
| `run_failed` | created/running/suspended/cancellation_requested | failed | no prepared-without-receipt attempt |
| `execution_completed` | running | execution_completed | no unresolved/prepared-without-receipt attempt; hard budgets not exceeded |
| `budget_exhausted` | running | suspended | reason becomes budget_exhausted(dimension) |
| `recovery_boundary` | running/cancellation_requested | suspended | reconciliation reason if unresolved exists, otherwise runtime_interruption |

Non-phase-changing events may still modify derived state only where explicitly defined.

Terminal phases reject all later v1 events. Reopening requires a future versioned migration/repair contract.

## 6. Suspension reasons

```json
{"kind":"user_pause"}
{"kind":"budget_exhausted","dimension":"tool_calls"}
{"kind":"reconciliation_required","attempt":{...}}
{"kind":"cancellation_in_progress"}
{"kind":"runtime_interruption"}
{"kind":"other","code":"bounded.non-empty.code"}
```

Resumable in v1:
- `user_pause`;
- `runtime_interruption` only when no unresolved attempt or budget blocker remains.

Not directly resumable in v1:
- `budget_exhausted`;
- `reconciliation_required`;
- `cancellation_in_progress`.

ECR-002 v1 has no policy event that increases a budget or clears a reconciliation requirement by fiat.

## 7. Run events

Strict tagged wire form:

```json
{"kind":"<snake_case kind>","value":{...}}
```

### `run_created`

```json
{"actor":"<ActorId>","budget":{...}}
```

Only sequence 1.

### `run_started`

```json
{}
```

### `run_suspended`

```json
{"reason":{...}}
```

### `run_resumed`

```json
{}
```

### `cancellation_requested`

```json
{"actor":"<ActorId>"}
```

Attribution is not authenticated-principal proof.

### `run_cancelled`

```json
{}
```

### `run_failed`

```json
{"error":{"category":"<category>","code":"<code>","message":null}}
```

The diagnostic message is non-authoritative.

### `execution_completed`

```json
{}
```

### `attempt_prepared`

```json
{"attempt":{...ActionAttemptRef...}}
```

Must be durably committed before provider invocation.

### `receipt_recorded`

```json
{"receipt":{...ActionReceipt...}}
```

Must match the exact prepared attempt and ActionRef. At most one receipt per attempt in v1.

### `recovery_boundary`

```json
{"reason":"process_restart"}
```

or

```json
{"reason":"explicit_recovery"}
```

The recovering runtime appends this after loading and validating the durable ledger. The reducer never invents durable events during replay.

### `attempt_marked_unknown`

```json
{
  "attempt":{...ActionAttemptRef...},
  "cause":"interrupted_before_receipt"
}
```

Allowed causes:

```text
interrupted_before_receipt
provider_ambiguous
other
```

This is executor-side ambiguity, not verification.

### `reconciliation_requested`

```json
{"attempt":{...ActionAttemptRef...}}
```

Durable handoff for ECR-004; ECR-002 does not decide reconciliation evidence.

### `resource_usage_recorded`

```json
{"dimension":"tool_calls","amount":1}
```

Usage is charged using checked arithmetic.

### `budget_soft_limit_reached`

```json
{"dimension":"tool_calls","soft_limit":80,"cumulative_usage":80}
```

Valid only on the first crossing of the configured soft limit.

### `budget_exhausted`

```json
{"dimension":"tool_calls","hard_limit":100,"cumulative_usage":100}
```

Valid only when cumulative usage is at/above hard limit and hard value matches the run budget. It blocks further governed work.

### `intervention_recorded`

```json
{
  "actor":"<ActorId>",
  "kind":"takeover",
  "note":null
}
```

Kinds:

```text
takeover
hand_back
pause_request
edit
denial
note
```

`note` is bounded diagnostic text and never authority/approval.

## 8. Attempt protocol

For any action that may create an external effect:

```text
1. construct exact ECR-001 ActionAttemptRef
2. append + COMMIT attempt_prepared
3. only after commit succeeds, allow provider invocation
4. provider returns executor-observed result
5. append + COMMIT receipt_recorded
```

Failure windows:

| Failure point | Durable truth | Allowed recovery |
|---|---|---|
| before step 2 commit | no attempt established | provider must not have been invoked through this guard |
| after step 2, before known provider invocation | prepared/no receipt | unresolved; no blind retry without proof allowed by retry semantics |
| during/after provider effect, before step 5 | prepared/no receipt | UNKNOWN + reconciliation required |
| after step 5 commit | exact receipt available | replay receipt; do not repeat effect |

A recovery boundary marks every prepared-without-receipt attempt unresolved. Missing receipt never becomes success or failure.

## 9. Retry guard

ECR-002 does not redefine ECR-001 retry semantics.

A scheduling helper must refuse retry when:
- the prior attempt is unresolved and the ActionIntent retry mode is `reconcile` or `never_blind`;
- idempotency is `non_idempotent` or `unknown` and absence of effect cannot be proven;
- a hard budget is exhausted;
- the run is not `running`.

A refusal is typed durable/runtime evidence, not a policy authorization decision.

## 10. Budget contract

Dimensions exactly:

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

`BudgetAmount`:

```text
0..=9_007_199_254_740_991
```

`RunBudget` is a non-empty list of unique `BudgetLimit` values:

```json
{
  "limits":[
    {"dimension":"tool_calls","soft":80,"hard":100}
  ]
}
```

Rules:
- hard is mandatory;
- soft is optional and `soft <= hard`;
- duplicate dimensions invalid;
- floating-point and negative values invalid;
- all cumulative arithmetic uses checked addition;
- omitted dimension means ECR-002 has no configured limit for that dimension, not that the caller obtained authorization to consume it;
- cost uses integer microunits only.

Preflight:

```text
known_upper_bound <= hard_limit - current_usage
```

If false, the work must not start.

Post-use charging may reveal an overshoot after an already-observed external effect. The usage and exhaustion are recorded; the effect is never rewritten or automatically retried.

## 11. SQLite store contract

Database schema version v1 is `1`.

Required open configuration:

```text
journal_mode = WAL
synchronous  = FULL
foreign_keys = ON
trusted_schema = OFF
```

The adapter reads back the requested values and fails closed if the critical durability/security configuration did not take effect.

All authoritative writes use an eager write transaction equivalent to:

```sql
BEGIN IMMEDIATE;
```

No provider/network/model/browser/process invocation occurs while a store transaction is open.

### `run_events`

Planned DDL semantics:

```sql
CREATE TABLE run_events (
  run_id TEXT NOT NULL,
  sequence INTEGER NOT NULL CHECK(sequence >= 1 AND sequence <= 9007199254740991),
  event_digest TEXT NOT NULL,
  previous_digest TEXT,
  event_json BLOB NOT NULL,
  PRIMARY KEY(run_id, sequence),
  UNIQUE(run_id, event_digest)
) STRICT;
```

Triggers reject `UPDATE` and `DELETE` on `run_events` through ordinary application access.

### `run_heads`

```sql
CREATE TABLE run_heads (
  run_id TEXT PRIMARY KEY,
  last_sequence INTEGER NOT NULL,
  last_digest TEXT NOT NULL,
  phase TEXT NOT NULL,
  state_json BLOB NOT NULL
) STRICT;
```

Projection only; safe to delete/rebuild from `run_events`.

### `artifact_blobs`

```sql
CREATE TABLE artifact_blobs (
  content_digest TEXT PRIMARY KEY,
  byte_size INTEGER NOT NULL CHECK(byte_size >= 0 AND byte_size <= 9007199254740991),
  bytes BLOB NOT NULL
) STRICT;
```

Blob digest and declared size are verified before commit. Real sensitive blobs are outside ECR-002 v1 product authorization.

## 12. Atomic append and expected head

Caller supplies:

```text
ExpectedRunHead::Genesis
```

or

```text
ExpectedRunHead::At { sequence, digest }
```

Append algorithm:

```text
BEGIN IMMEDIATE
  read current head
  require current == expected
  require next sequence == current + 1 (or 1 for genesis)
  validate envelope/version/event/bindings/budget/state transition
  recompute and verify ledger digest
  INSERT authoritative event
  reduce previous state + event
  UPSERT derived head/state
COMMIT
```

Mismatch -> `ledger_head_mismatch`. Two concurrent writers using the same expected head cannot both commit the next sequence.

## 13. Projection rebuild

Rebuild algorithm:
- read authoritative events ordered by `(run_id, sequence)`;
- strict-parse and chain-verify every event;
- reduce from empty state;
- replace projection only after full validation succeeds;
- never mutate authoritative events during rebuild.

A corrupted history fails before publishing a rebuilt projection.

## 14. Schema migration

Rules:
- database schema version is checked before normal reads/writes;
- v1 creation is deterministic;
- version > supported -> fail closed;
- every future migration is transactional and fixture-tested;
- migration failure rolls back and leaves the prior store unchanged;
- event wire version is validated independently;
- no migration may rewrite historical event meaning silently.

## 15. `.ecra` v1 archive profile

`.ecra` is a ZIP container but only this strict profile is accepted.

Canonical entry layout:

```text
manifest.v1.json
events/0000000000000001.json
events/0000000000000002.json
...
blobs/sha256/<64-lowercase-hex>
```

Writer rules:
- compression method: Stored only;
- no encryption;
- UTF-8 forward-slash paths only;
- no absolute path, `.` or `..` path segment, backslash or NUL;
- no directories/symlinks;
- no archive/file comments;
- fixed DOS-compatible timestamp `1980-01-01T00:00:00`;
- fixed regular-file permissions `0600`;
- stable system/metadata selection supported by the library;
- entry order: manifest first, events ascending, blobs lexicographic;
- JSON entries are RFC 8785 canonical bytes;
- duplicate entry names forbidden.

Reader hard limits:

```text
MAX_ARCHIVE_ENTRIES             = 16_384
MAX_EVENT_COUNT                 = 10_000
MAX_BLOB_COUNT                  = 6_000
MAX_MANIFEST_BYTES              = 8 MiB
MAX_EVENT_ENTRY_BYTES           = 4 MiB
MAX_SINGLE_BLOB_BYTES           = 64 MiB
MAX_TOTAL_UNCOMPRESSED_BYTES    = 512 MiB
MAX_PATH_BYTES                  = 512
```

These are v1 parser safety limits, not user quota/budget grants. Future changes require contract/version review.

Reader validates metadata/path/method/count/size before allocating/materializing trusted output and applies a running total limit while streaming.

## 16. Archive manifest

Strict form:

```json
{
  "schema_version":{"major":1,"minor":0},
  "run_id":"<RunId>",
  "event_count":2,
  "head_digest":{"algorithm":"sha256","hex":"..."},
  "events":[
    {
      "sequence":1,
      "path":"events/0000000000000001.json",
      "ledger_digest":{"algorithm":"sha256","hex":"..."},
      "byte_size":123,
      "content_digest":{"algorithm":"sha256","hex":"..."}
    }
  ],
  "blobs":[]
}
```

Manifest event entries are ordered by sequence and must exactly cover `1..=event_count`.

Import validates:
1. ZIP safety/profile/limits;
2. strict manifest schema/version;
3. exact allowed-entry set equals manifest declarations;
4. event/blob byte size and ContentDigest;
5. event strict parsing and run-id/sequence binding;
6. full ledger-chain/digest verification;
7. manifest head digest matches final event;
8. reducer/state validation.

Only then may the caller import into a local store.

## 17. Deterministic export

For identical logical run events/blobs, export bytes must be identical.

Determinism inputs include:
- canonical JSON bytes;
- fixed ZIP timestamp/permissions/system metadata;
- Stored method;
- sorted entry order;
- no runtime clock/random/archive comments;
- manifest generated from logical content only.

The archive is not signed/encrypted in ECR-002. Protected authenticity/confidentiality belongs to later slices.

## 18. Sensitive-state gate

The v1 implementation and committed fixtures may contain only synthetic/non-sensitive values. The API must document that durable structural capability is not product authorization to persist real secrets/private browser/workspace payloads.

No acceptance test may use real credentials, browser cookies, API keys, private documents, PHI, financial records, or real authenticated assertion material.

## 19. Typed errors

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

Required v1 machine codes:

```text
unsupported_major_version
unsupported_minor_version
invalid_event_sequence
invalid_event
invalid_state_transition
duplicate_attempt
attempt_binding_mismatch
receipt_binding_mismatch
unresolved_attempt
blind_retry_forbidden
ledger_head_mismatch
ledger_chain_invalid
ledger_digest_mismatch
store_configuration_invalid
store_busy
storage_error
unsupported_store_version
migration_failed
invalid_budget
budget_overflow
budget_preflight_exceeded
budget_exhausted
archive_path_invalid
archive_duplicate_entry
archive_feature_unsupported
archive_limit_exceeded
archive_manifest_invalid
archive_digest_mismatch
recovery_required
serialization_failed
```

Display strings are diagnostics only. Tests branch on category/code.

## 20. Dependency and side-effect boundary

Allowed production I/O for `ecra-run` v1:
- explicitly requested local SQLite file operations;
- explicitly requested `.ecra` read/write operations.

Forbidden:
- network;
- telemetry;
- browser/model/tool/provider calls;
- shell/process execution in production library code;
- OS clock/random inside deterministic reducer/digest/archive canonicalization.

`ecra-run` itself must use `#![forbid(unsafe_code)]`. Native SQLite is isolated behind reviewed `rusqlite/libsqlite3-sys`; no unsafe code is added to Ecra-owned production Rust.
