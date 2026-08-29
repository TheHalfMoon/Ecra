# Threat Model: ECR-004 Verification & Reconciliation

## Assets

- integrity of canonical `VerificationReceipt` records;
- exact target/action/attempt binding;
- visibility of conflicting verifier outcomes;
- UNKNOWN preservation and blind-retry prevention;
- integrity/order of ECR-004 journal entries;
- evidence identity/digest/as-of metadata;
- checkpoint requirements and derived completion views;
- non-authoritative boundary between reconciliation and execution authorization;
- preservation of ECR-002 unresolved execution state unless a real ECR-002 receipt/versioned repair protocol changes it.

## Trust boundaries

```text
untrusted evidence metadata / external references
                ↓
strict ECR-004 parser + bounds
                ↓
pure evidence rule / receipt construction
                ↓
canonical ECR-001 VerificationReceipt
                ↓
append-only ECR-004 journal
                ↓
aggregate/checkpoint/reconciliation derived views

ECR-002 RunState/ActionAttempt truth ───────┘
                ↑
read-only compatibility boundary; no run-state mutation
```

Provider/browser/model/network acquisition is outside ECR-004 v1. Inputs crossing into ECR-004 are data, not authority.

## Adversaries and failure classes

### TM-001 Executor self-verifies

**Attack:** executor success receipt is treated as independent verification.

**Mitigation:** canonical type separation; generic decision-grade rule rejects a self-attesting receipt-only basis; architecture tests forbid receipt->verified shortcuts.

### TM-002 Wrong-target evidence substitution

**Attack:** valid evidence for action/attempt A is used to verify B.

**Mitigation:** exact `VerificationTarget` plus reconciliation `RunId`/`ActionAttemptRef`/`ActionRef` binding; target mismatch is typed fail-closed error.

### TM-003 Last-write-wins hides conflict

**Attack:** later `Verified` overwrites earlier `Rejected` or vice versa.

**Mitigation:** append-only receipts; deterministic aggregate `Conflicted`; all receipt IDs retained.

### TM-004 Absence of evidence becomes no-effect proof

**Attack:** missing provider receipt/404/timeout is interpreted as proof that an external side effect did not happen.

**Mitigation:** `no_effect_confirmed` requires explicit conclusive evidence; absent/insufficient evidence remains `still_unknown`.

### TM-005 Reconciliation fabricates execution receipt

**Attack:** independent observation is converted into a fake provider `ActionReceipt` so ECR-002 looks clean.

**Mitigation:** reconciliation stores only canonical verification receipt IDs and effect outcome; no API creates `ActionReceipt`.

### TM-006 Blind retry after UNKNOWN

**Attack:** unresolved external attempt is retried due timeout, note, elapsed time, model confidence, or a misread `no_effect_confirmed` advisory.

**Mitigation:** `still_unknown` -> `reconciliation_required`; ECR-001 retry/idempotency semantics remain mandatory; no ECR-004 result authorizes execution; `semantically_retryable*` applies only to a future new-attempt proposal and never clears the existing ECR-002 unresolved state.

### TM-007 Duplicate side effect after effect confirmed

**Attack:** reconciled `effect_confirmed` attempt is retried.

**Mitigation:** derived `duplicate_retry_blocked` disposition; exact-attempt tests across all retry classes; existing ECR-002 unresolved state remains untouched.

### TM-008 Mutable external evidence changes later

**Attack:** a URL/resource changes after verification while old result is presented as decision-grade current truth.

**Mitigation:** conclusive mutable evidence requires digest/snapshot binding when required; explicit `as_of`/evaluation time for freshness-sensitive rules; no live fetch in ECR-004.

### TM-009 Malicious evidence text becomes authority

**Attack:** evidence notes/content says “approved”, “verified”, or contains prompt injection that changes runtime behavior.

**Mitigation:** evidence is structured data only; no policy/approval parser; no model/tool execution in trusted crate; bounded opaque notes/rule IDs.

### TM-010 VerificationReceipt metadata becomes provenance rewrite

**Attack:** verified result changes original source class to trusted/user-provided.

**Mitigation:** verification aggregation never mutates ECR-001 provenance/freshness/dispute objects.

### TM-011 Journal row mutation/deletion

**Attack:** local corruption or ordinary code alters prior verification truth.

**Mitigation:** append-only store APIs, optional DB triggers, sequence/previous-digest/content-digest validation, projection rebuild tests.

**Boundary:** an attacker able to rewrite the entire local store can recompute the unprotected chain. No hostile tamper-resistance claim is made until a future protected-anchor integration is authorized.

### TM-012 Projection poisoning

**Attack:** index row points to wrong target/entry and changes aggregate view.

**Mitigation:** canonical journal replay is authoritative; indexes are rebuildable; load path validates indexed entry identity/target before use.

### TM-013 Duplicate IDs / ambiguous records

**Attack:** two receipts/reconciliation records share one stable ID with different content.

**Mitigation:** unique indexes plus canonical duplicate-content mismatch rejection.

### TM-014 Oversized evidence/checkpoint/journal input

**Attack:** memory/CPU exhaustion through huge arrays/notes or scan amplification.

**Mitigation:** strict count/byte/query limits, checked arithmetic, bounds before expensive materialization.

### TM-015 Verification method laundering

**Attack:** caller labels model output as structured external state or relies on method enum as proof.

**Mitigation:** method alone never determines outcome; rule validates evidence shape/binding. Provider/source attestation belongs to later adapters/evaluation slices.

### TM-016 Compromised verifier

**Attack:** malicious verifier emits false but structurally valid evidence-backed receipt.

**Mitigation:** ECR-004 records verifier identity/method/evidence and preserves conflicts; statistical verifier trust/corroboration metrics belong to ECR-005/ECR-009/ECR-028. ECR-004 does not claim one verifier is infallible.

### TM-017 Secret leakage in notes/evidence payloads

**Attack:** raw secrets/private content enter journal/logs through notes or generic evidence blobs.

**Mitigation:** ECR-004 v1 stores references/digests only, bounds notes, uses synthetic fixtures, and adds sentinel/redaction scans. Real sensitive evidence persistence is outside authorization.

### TM-018 ECR-004 becomes authorization bypass

**Attack:** `Verified`/`no_effect_confirmed` is interpreted as permission to execute/retry/disclose.

**Mitigation:** no capability/policy/approval output types; retry disposition explicitly non-authoritative; architecture tests reject ECR-003 concepts/dependencies.

### TM-019 Reconciliation bypasses ECR-002 unresolved-state guards

**Attack:** a consumer treats `no_effect_confirmed` as if it removed the prior attempt from `RunState::unresolved_attempts`, marks the prepared attempt resolved, calls `RunResumed`/`ExecutionCompleted`, or invokes blind retry in the same run.

**Mitigation:** ECR-004 exposes no run-state mutation/event/receipt API; reconciliation functions take ECR-002 state read-only; Phase 5 tests compare pre/post run state and prove `unresolved_attempts` is unchanged; ECR-002 regression tests prove resume/completion/retry guards still reject the unresolved attempt after every reconciliation outcome. Any future repair/resolution transition requires explicit ECR-002 versioned ownership.

## Security claims allowed

ECR-004 v1 may claim:
- deterministic exact-target verification aggregation from supplied records;
- append-only local journal behavior under tested store APIs;
- corruption/substitution detection under stated local integrity assumptions;
- fail-closed UNKNOWN reconciliation and blind-retry safety semantics;
- no executor-receipt self-verification shortcut;
- reconciliation preserves ECR-002 unresolved execution-state guards.

ECR-004 v1 may NOT claim:
- verifier infallibility or calibrated accuracy;
- hostile full-store tamper resistance;
- remote evidence authenticity beyond supplied bindings;
- authorization, authentication, declassification, or safe provider execution;
- protection of real sensitive evidence payloads;
- exactly-once external effects;
- that `no_effect_confirmed` resumes/completes the same ECR-002 run or clears its unresolved attempt.