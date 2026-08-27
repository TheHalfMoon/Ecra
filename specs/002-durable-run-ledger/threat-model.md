# Threat Model: ECR-002 Durable Run, Ledger & Budgets

**Feature:** ECR-002  
**Status:** PLANNING_SECURITY_COMPLETE  
**Parent:** `specs/000-ecra-platform/threat-model.md`

## 1. Security objective

Preserve durable local execution truth across crashes/restarts without allowing persistence, replay, archives, budgets or attempt bookkeeping to fabricate authority, verification, identity, or side-effect certainty.

## 2. Assets

- authoritative run event history;
- exact ActionRef/ActionAttemptRef bindings;
- executor ActionReceipts;
- UNKNOWN/unresolved attempt state;
- run phase and cancellation/intervention events;
- resource budget limits/usage/exhaustion evidence;
- local SQLite database + WAL as one persistence unit;
- `.ecra` manifest/events/blobs;
- schema/event versions and migrations;
- event-chain continuity/digests.

Real secrets/sensitive user payloads are not authorized ECR-002 acceptance assets; their persistence remains gated.

## 3. Adversaries/failure sources

- process crash/kill at any transaction/provider boundary;
- OS/system crash or power loss within SQLite/VFS/storage assumptions;
- concurrent local writer races;
- malformed/corrupted local database bytes;
- accidental application UPDATE/DELETE of authoritative history;
- whole-store malicious rewriter;
- crafted `.ecra` archive: traversal, duplicate names, huge sizes/counts, unsupported compression/encryption, symlinks, malformed manifest/event, digest mismatch;
- malformed/newer schema/event version;
- integer overflow/resource-accounting bugs;
- caller attempting to retry unresolved consequential work;
- caller treating Actor as authenticated principal or receipt/execution-completed as verification;
- hidden network/telemetry or real-sensitive fixtures entering the slice.

## 4. Trust boundaries

### TB-1 Runtime caller -> reducer

Input events are untrusted until strict schema/version/state validation completes.

Controls:
- strict Serde forms;
- typed ECR-001 refs;
- pure reducer;
- typed errors;
- invalid transition/attempt/budget tests.

### TB-2 Runtime caller -> SQLite store

Callers may race, crash, pass stale expected heads, or try to mutate history.

Controls:
- `BEGIN IMMEDIATE` equivalent;
- expected-head compare;
- primary/unique/check constraints;
- UPDATE/DELETE-deny triggers;
- full event validation before append;
- projection rebuild rather than history mutation.

### TB-3 Durable attempt -> external provider boundary

ECR-002 does not invoke the provider but must make it impossible for a compliant executor to claim a durable attempt was prepared after the fact.

Controls:
- committed `attempt_prepared` guard before invocation;
- exact attempt/action binding;
- crash tests;
- prepared-without-receipt -> UNKNOWN/reconciliation required;
- blind retry guard.

### TB-4 SQLite/WAL -> runtime

Local bytes are untrusted on read.

Controls:
- supported schema check;
- WAL/FULL/foreign_keys/trusted_schema assertions;
- strict event parsing and chain validation;
- projection recomputation;
- migration fixtures.

### TB-5 `.ecra` bytes -> trusted logical import

Archives are adversarial parser input.

Controls:
- strict Stored-only profile;
- path normalization/rejection before extraction;
- no symlinks/directories/encryption;
- entry count/per-entry/total size caps;
- duplicate name rejection;
- manifest whitelist of entries;
- ContentDigest + LedgerDigest validation;
- no generic extract-to-directory API.

### TB-6 Budget caller -> resource accounting

Callers may omit dimensions, overflow counters, under-declare upper bounds, or attempt to continue after exhaustion.

Controls:
- typed dimensions/amounts;
- checked arithmetic;
- exact preflight;
- durable usage/exhaustion;
- run suspension on hard exhaustion;
- no v1 budget-increase API.

## 5. High-priority abuse cases

### A1 Duplicate external side effect after crash

Invariant: prepared-without-receipt never becomes safe-to-retry merely because the process restarted.

Evidence: crash boundary tests B/C and retry guard tests.

### A2 Silent event history fork

Invariant: two writers using the same expected head cannot both append sequence n+1.

Evidence: multi-connection concurrency test.

### A3 Projection becomes truth

Invariant: deleting/rebuilding projection yields identical derived state; corrupted history blocks rebuild.

Evidence: rebuild property/integration tests.

### A4 Hash-chain overclaim

Invariant: docs/API names never describe LedgerDigest as hostile tamper resistance/signature/MAC.

Evidence: documentation/static wording audit and whole-chain recomputation adversarial fixture showing the limit.

### A5 Archive traversal/resource bomb

Invariant: unsafe path/feature/size/count is rejected before trusted materialization.

Evidence: malicious archive corpus and streaming limit tests.

### A6 Budget overflow/bypass

Invariant: no wraparound or implicit unlimited widening.

Evidence: boundary/property tests at 0, hard, MAX_SAFE_INTEGER and overflow.

### A7 False completion

Invariant: `execution_completed` and ActionReceipt remain executor/runtime observations, never VerificationReceipt.

Evidence: type/API tests and docs.

### A8 Sensitive persistence before protection

Invariant: committed fixtures and product-facing scope remain synthetic/non-sensitive; no claim of encrypted/protected storage.

Evidence: fixture audit + README/threat model boundary.

## 6. SQLite durability claim

ECR-002 requires WAL + `synchronous=FULL`, but the claim is scoped to SQLite's documented VFS/storage assumptions. Ecra does not claim immunity to lying hardware, broken filesystems, malicious whole-store rewrite or all forms of physical corruption.

Copying only the main DB while a live WAL exists is invalid as export. `.ecra` is the portable format.

## 7. Native dependency boundary

`rusqlite/libsqlite3-sys` introduces a C/native dependency in the ECR-002 I/O layer. Ecra-owned `ecra-run` Rust still forbids unsafe code. Dependency/advisory/version review is required before merge and remains separate from the zero-I/O `ecra-core` boundary.

## 8. Security test matrix

| Threat | Required test/evidence |
|---|---|
| stale/concurrent append | two-connection expected-head race |
| crash before/after preparation/receipt | child-process crash matrix |
| missing receipt | unresolved + reconciliation-required recovery |
| blind retry | negative retry-guard tests for reconcile/never-blind/non-idempotent/unknown |
| event mutation/gap/reorder | chain corpus |
| ordinary UPDATE/DELETE | SQLite trigger rejection |
| newer/malformed DB/event | compatibility/migration fixtures |
| projection corruption/removal | full rebuild equality |
| budget overflow | property tests |
| hard exhaustion | durable suspension event fixture |
| archive traversal/backslash/absolute | malicious name corpus |
| archive duplicate/method/encryption/symlink | profile rejection corpus |
| archive count/size bomb | streaming limits |
| manifest/content/ledger digest mismatch | import rejection |
| hidden network/provider I/O | static production-source/dependency audit |
| unsafe Ecra-owned code | crate lint + CI source scan |
| real sensitive fixture | repository fixture audit |

## 9. Explicit residual risks / later owners

- protected authenticity/anti-rollback/keyed ledger anchor -> ECR-031;
- authorization/revocation/approval/budget-increase policy -> ECR-003;
- independent UNKNOWN reconciliation and verification sufficiency -> ECR-004;
- hostile sandbox/provider/process execution -> ECR-017/ECR-018;
- telemetry/redaction/protected sensitive diagnostics -> ECR-025;
- broader import/export/deletion portability -> ECR-029.

ECR-002 must expose durable hooks for these owners without implementing their policy prematurely.
