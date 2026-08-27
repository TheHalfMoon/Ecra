# Research: Durable Run, Ledger & Budgets

**Feature:** ECR-002  
**Status:** PLANNING_RESEARCH_COMPLETE  
**Date:** 2026-08-27

This research resolves the implementation-shaping questions for ECR-002. Sources are used as standards/dependency/concept references only; no donor source is copied into Ecra.

## 1. Decision summary

| Question | Decision |
|---|---|
| Authoritative run truth | Append-only, per-run ordered event history; mutable projections are rebuildable/non-authoritative |
| Local database | SQLite through `rusqlite` with bundled SQLite candidate, WAL, `synchronous=FULL`, `BEGIN IMMEDIATE`-equivalent writes |
| Alternative reviewed | `redb` is credible/crash-safe/pure Rust but rejected for ECR-002 because Ecra would have to build more schema/query/migration/constraint infrastructure itself |
| Attempt safety | Durable attempt preparation before provider invocation; missing receipt after recovery remains unresolved/UNKNOWN |
| Replay | Pure deterministic reducer over accepted ordered events; no OS clock/random/I/O in reducer |
| Integrity | Domain-separated SHA-256 previous-event chain; corruption/mutation evidence only, not hostile-rewriter authenticity |
| Budgets | Typed I-JSON-safe hard limits, optional soft limits, checked arithmetic, durable usage/exhaustion events, no ambient increase |
| Portable `.ecra` | Deterministic ZIP profile with stored entries; manifest + canonical events + content-addressed blobs; never live SQLite/WAL |
| Sensitive persistence | Synthetic/non-sensitive fixtures only in v1 acceptance; real sensitive state gated by ECR-031/ECR-003/ECR-025 |
| Distributed workflow engine | Explicit non-goal; Temporal/Restate are conceptual references, not dependencies/foundations |

## 2. SQLite durability and transaction semantics

Primary sources:
- SQLite WAL: https://www.sqlite.org/wal.html
- SQLite synchronous pragma: https://sqlite.org/pragma.html#pragma_synchronous
- SQLite transactions: https://www.sqlite.org/lang_transaction.html
- SQLite corruption guidance: https://www.sqlite.org/howtocorrupt.html

Findings:
- WAL mode permits concurrent readers with one writer and keeps committed changes in the WAL until checkpointed.
- SQLite documents that `synchronous=FULL` in WAL mode adds a WAL sync after each commit and is the setting that preserves transaction durability across OS crash/power loss under the VFS/storage assumptions.
- `synchronous=NORMAL` keeps WAL consistency but can lose a recently committed transaction on power loss/system crash, which is unsuitable for Ecra's durable-attempt preparation contract.
- `BEGIN IMMEDIATE` acquires the write transaction at begin time; another writer cannot simultaneously acquire the write transaction. This is useful for an atomic expected-head append protocol.
- SQLite explicitly warns that it must trust OS/hardware sync behavior; Ecra therefore must not make absolute physical-media durability claims.
- SQLite WAL files are part of persistent database state while live. A database file copied without its associated WAL can lose committed state or be invalid. Therefore a live SQLite database is not Ecra's interchange format.

Decision:
- ECR-002 v1 uses `journal_mode=WAL`, `synchronous=FULL`, `foreign_keys=ON`, strict schema/version checks, and eager write transactions.
- The adapter verifies key pragmas after opening rather than assuming they succeeded.
- WAL checkpoint policy may be operationally tuned, but checkpointing must never change event semantics or become required to call an event committed.

## 3. Rust SQLite adapter

Upstream:
- https://github.com/rusqlite/rusqlite
- release candidate studied: `rusqlite` 0.40.1

Observed upstream facts at planning time:
- `rusqlite` 0.40.1 uses MIT licensing.
- `libsqlite3-sys` is likewise MIT; the `bundled` feature compiles the embedded SQLite source, which upstream describes as public domain.
- Upstream 0.40.1 bundles SQLite 3.53.2 and recommends bundled SQLite for applications that control their own database because it avoids depending on a missing/old system SQLite.

Decision:
- Plan for exact locked `rusqlite = 0.40.1` with minimal required features plus `bundled` unless implementation-time lock/security review reveals a blocker.
- The native C SQLite boundary is intentionally outside `ecra-core`; it lives only in the ECR-002 I/O crate.
- Dependency lock/license/provenance and transitive review are implementation tasks, not pre-authorized by this research.

## 4. Pure-Rust alternative: redb

Upstream:
- https://github.com/cberner/redb
- design: https://github.com/cberner/redb/blob/master/docs/design.md

Observed:
- redb is a maintained, pure-Rust embedded ACID key-value database with MVCC and crash-safe durable commits.
- Its stable format and checksum/transaction design make it a legitimate option, not a strawman.
- It is MIT OR Apache-2.0.

Why rejected for ECR-002 v1:
- ECR-002 benefits from relational uniqueness/check constraints, ordered queries, explicit schema/user versioning, migration fixtures, and ordinary SQL inspection.
- With a KV engine Ecra would own more custom table/index/schema/migration code in the trusted runtime layer.
- SQLite is more directly aligned with the append-event + rebuildable projection model and is easier to inspect with standard tools.

Revisit trigger:
- measured SQLite write contention/latency or native-build/platform constraints that materially violate ECR-002 acceptance criteria.

## 5. Durable execution conceptual donors

### Temporal
References:
- https://docs.temporal.io/
- https://github.com/temporalio/documentation/tree/main/docs/encyclopedia/event-history

Useful principles:
- event history reconstructs workflow state after failure;
- replay logic must be deterministic;
- side-effecting/nondeterministic work belongs outside replay logic;
- retried activities require idempotency/unique operation identity;
- history/schema/code evolution needs explicit versioning.

Ecra adaptation:
- Ecra does not adopt Temporal's service, workflow language, server architecture, activity retry defaults, or cloud assumptions.
- ECR-002 reducer is a small local deterministic state reducer over Ecra events.
- ECR-001 retry semantics remain authoritative; Ecra does not blanket-retry an activity because a workflow framework normally would.

### Restate
References:
- https://docs.restate.dev/guides/request-lifecycle
- https://docs.restate.dev/foundations/key-concepts

Useful principles:
- persist invocation/journal intent before relying on progress;
- replay journaled results instead of repeating completed effects;
- durable steps expose the exact failure window around side effects;
- deterministic time/random helpers prevent replay divergence.

Ecra adaptation:
- Restate core is `REFERENCE_ONLY` because of its licensing/distribution posture already recorded in the donor ledger.
- No Restate code/protocol/storage model is copied.
- Ecra is stricter around ambiguous consequential side effects: if the provider effect may have happened but no receipt is durable, the attempt remains UNKNOWN and requires reconciliation rather than optimistic replay.

## 6. Attempt-before-effect protocol

The critical failure window is:

```text
prepare attempt durably
  ↓
invoke external provider
  ↓
receive executor result
  ↓
record ActionReceipt durably
```

Required recovery semantics:
- crash before durable preparation → provider invocation was not authorized by the ECR-002 durable-attempt guard;
- crash after preparation, before invocation → unresolved prepared attempt; no blind retry unless ECR-001 semantics permit it and runtime can prove no effect occurred;
- crash during/after provider effect but before receipt commit → UNKNOWN/reconciliation required;
- crash after receipt commit → receipt is replayed as executor-observed truth;
- no state maps missing receipt to success/failure.

ECR-004 later decides reconciliation evidence/sufficiency. ECR-002 only preserves the exact unresolved condition durably.

## 7. Event ledger and projection model

Decision:
- `run_events` is authoritative.
- identity is `(RunId, EventSequence)`.
- `run_heads` and current RunState are projections/caches and can be deleted/rebuilt.
- append transaction checks expected previous sequence/digest, inserts event, then updates projection atomically.
- database triggers reject ordinary UPDATE/DELETE against authoritative events as defense against accidental/in-process mutation.

Integrity chain:

```text
LedgerDigest_n = SHA-256(
  "ecra/run-event/v1\0" ||
  JCS({ run_id, sequence, previous_digest, event })
)
```

The exact byte contract belongs in `contracts/run-ledger-v1.md` and golden fixtures.

Security wording:
- detects broken continuity/mutation relative to the inspected chain;
- does not authenticate who wrote the ledger;
- a full-store attacker can rewrite events and recompute hashes;
- ECR-031 owns any protected MAC/signature/trust-root anchor.

## 8. Run state

Chosen v1 phases:

```text
created
running
suspended
cancellation_requested
cancelled
failed
execution_completed
```

`execution_completed` is intentionally not `verified`.

Suspension reasons are explicit and may include:
- user_pause;
- budget_exhausted;
- reconciliation_required;
- cancellation_in_progress;
- runtime_interruption;
- other_versioned_reason.

State is reducer-derived from events; mutable UI/database status does not independently transition the run.

## 9. Budgets

Constitution/platform-owned dimensions are represented as typed `BudgetDimension` values with I-JSON-safe integer amounts.

v1 dimensions:
- active wall milliseconds;
- steps;
- tool calls;
- model calls;
- input tokens;
- output tokens;
- cost microunits;
- process count;
- process milliseconds;
- output bytes;
- network requests;
- network bytes;
- storage/artifact bytes;
- recursion/delegation depth.

Rules:
- every configured dimension has a hard limit;
- optional soft limit <= hard;
- checked arithmetic only;
- no floating-point cost accounting;
- usage is event-sourced, not a mutable authoritative counter;
- known upper-bound work is preflighted against remaining hard budget;
- post-use accounting can suspend future work but cannot pretend an external effect did not occur;
- v1 has no ambient budget expansion. Policy-authorized revision is deferred until an owner contract exists.

Time:
- reducer never calls system time;
- runtime supplies typed absolute event timestamps and/or active-duration deltas from an explicit timing boundary;
- persisted duration is data, not recomputed from wall-clock timestamps during replay.

## 10. Portable `.ecra` artifact

References:
- ZIP implementation candidate: https://github.com/zip-rs/zip2
- docs: https://docs.rs/zip/8.6.0/zip/
- format basis documented by upstream: PKWARE APPNOTE 6.3.9.

Observed planning facts:
- stable `zip` release studied: 8.6.0, MIT, MSRV below Ecra's Rust 1.98.
- upstream exposes writer metadata controls intended to permit byte-identical archives across platforms.

Decision:
- `.ecra` v1 is a deterministic ZIP profile, not live SQLite.
- writer uses Stored/no compression only for deterministic/simple behavior.
- normalized metadata: fixed timestamp, fixed permissions/system marker, no comments, no encryption, no symlinks, stable sorted UTF-8 names.
- canonical layout:

```text
manifest.v1.json
events/0000000000000001.json
...
blobs/sha256/<64-lowercase-hex>
```

- JSON entries are RFC 8785 canonical bytes.
- reader rejects unsupported methods/features and malicious names before trusted materialization.
- entry count, per-entry bytes and total uncompressed bytes are hard-limited.
- duplicate names are invalid.
- archive manifest binds run id, event count, head ledger digest and blob digests/sizes.

Why ZIP rather than custom framing:
- well-specified interoperable container;
- mature Rust implementation/fuzzing;
- deterministic profile can be much smaller than the full ZIP feature set;
- avoids inventing a bespoke archive parser/framing protocol.

## 11. Sensitive-state gate

ECR-002 is allowed to prove durability with synthetic/non-sensitive fixtures and structural references only.

Forbidden in v1 acceptance fixtures:
- real browser cookies/session tokens;
- API credentials/passkeys/secret bytes;
- private user documents/medical/financial payloads;
- real authenticated identity assertion contents;
- real production approval/authorization secrets.

A plain SQLite file + hash chain is not protected sensitive storage. ECR-031/ECR-025 must define at-rest protection/authenticity before broad real sensitive persistence is enabled.

## 12. Migration/versioning

- SQLite `user_version`/schema metadata is explicitly checked.
- v1 initial creation is deterministic.
- newer unsupported schema fails closed.
- migration code must be transactionally applied and fixture-tested.
- event schema version is independent of database schema version.
- old event semantics are never silently replayed under incompatible reducer semantics; version compatibility is explicit.
- snapshots/projections never delete authoritative history in ECR-002 v1.

## 13. Rejected alternatives

### Ad-hoc append-only JSON/file log
Rejected: Ecra would own fsync/rename/torn-write/locking/crash protocol, raising audit and portability risk.

### Live SQLite database as `.ecra`
Rejected: WAL is part of live persistent state; byte representation is not a stable interchange contract; migration/export concerns would couple transport to store internals.

### Distributed workflow engine dependency
Rejected: ECR-002 is local-first and bounded. Temporal/Restate add server/runtime architecture far beyond the current requirement and can impose retry/identity semantics inconsistent with Ecra.

### `synchronous=NORMAL`
Rejected for authoritative attempt/event commits because SQLite documents possible loss of recent WAL commits on system crash/power loss.

### Claiming hash-chain tamper resistance
Rejected constitutionally. Stronger hostile-rewriter authenticity needs ECR-031 protected trust material.

## 14. Implementation-time dependency review candidates

These are candidates, not authorization:

| Package | Candidate | Planned feature posture | License observed | Purpose |
|---|---:|---|---|---|
| `rusqlite` | 0.40.1 | minimal + `bundled` | MIT | local SQLite adapter |
| bundled SQLite | 3.53.2 via candidate above | upstream bundled C source | public domain per upstream | database engine |
| `zip` | 8.6.0 | default-features=false; Stored-only profile | MIT | deterministic `.ecra` container |
| `tempfile` | exact lock TBD | dev-only | MIT OR Apache-2.0 upstream commonly; verify exact | crash/store/archive tests |

Existing ECR-001 dependencies remain governed by their locked ledger. No dependency is added to `ecra-core` by this plan.
