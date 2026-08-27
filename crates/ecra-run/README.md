# ecra-run

`ecra-run` is the ECR-002 local durable-execution layer above `ecra-core`.

## Owns

- append-only ordered run-event history;
- deterministic RunState reduction and rebuildable projections;
- durable ActionAttempt preparation/receipt bookkeeping;
- UNKNOWN/unresolved crash-recovery evidence;
- typed resource budgets and exhaustion evidence;
- bounded local SQLite persistence and migrations;
- deterministic synthetic/non-sensitive `.ecra` interchange.

## Does not own

- authentication, principal assertions, trust roots or protected keys;
- authorization, approvals, declassification or secret mediation;
- independent verification/reconciliation decisions;
- browser/model/tool/process provider execution;
- network/cloud/distributed workflow execution;
- product authorization to persist real sensitive user state.

## Misuse warnings

1. **Actor attribution is not Principal authentication.** Persisting an `ActorId` does not authenticate anyone.
2. **A durable attempt is not authorization.** `AttemptPrepared` records execution bookkeeping only.
3. **A receipt is not verification.** Executor-observed success never becomes `VERIFIED` here.
4. **Missing receipt remains UNKNOWN.** Recovery must not fabricate failure/success or blindly retry.
5. **Projection is not history.** Derived state may be deleted/rebuilt; authoritative events are the source of run truth.
6. **LedgerDigest is not hostile-tamper protection.** A full-store rewriter can recompute an unkeyed chain until ECR-031 supplies a protected anchor.
7. **A budget is not authority.** Remaining quota never grants permission to perform or disclose anything.
8. **`.ecra` is not a secret container.** ECR-002 v1 fixtures/import-export are synthetic/non-sensitive and provide no confidentiality/authenticity envelope.

## Dependency boundary

`ecra-run` may depend on `ecra-core`, serialization/digest/error libraries, a narrowly configured SQLite adapter and a narrowly configured ZIP library. It must never add SQLite/ZIP/runtime dependencies to `ecra-core`.

Ecra-owned production Rust uses `#![forbid(unsafe_code)]`. Bundled SQLite is a reviewed native dependency boundary outside Ecra-owned Rust source.
