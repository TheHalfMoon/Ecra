# Specification: Verification & Reconciliation

**Feature:** ECR-004  
**Lifecycle target:** SPEC_READY → PLAN_READY → TASKS_READY  
**Depends on:** ECR-001 `CLOSED_CANONICAL`, ECR-002 `CLOSED_CANONICAL`  
**Constitution:** v1.1.0  
**Scope class:** independent verification, evidence aggregation, UNKNOWN reconciliation

## 1. Purpose

ECR-004 supplies the independent verification layer between executor-observed outcomes and later policy/product completion. It consumes canonical ECR-001 targets/evidence/`VerificationReceipt` semantics and ECR-002 durable run/attempt/UNKNOWN truth, records immutable verification and reconciliation evidence, and provides fail-closed reconciliation decisions without fabricating execution receipts or granting authority.

ECR-004 does **not** execute provider actions, authorize actions, approve retries, declassify information, authenticate principals, replace ECR-001/ECR-002 truth, or silently evolve the closed ECR-002 run-event v1 contract. A verifier establishes only an independent verification outcome about an exact target from explicit evidence.

For v1, reconciliation resolves an **effect claim**, not the missing provider receipt and not the ECR-002 run-state blocker. An ECR-002 prepared attempt that recovered without a real `ActionReceipt` remains durably unreceipted/unresolved in the original run even after ECR-004 records `effect_confirmed` or `no_effect_confirmed`. Any future same-run resume/retry integration requires an explicitly versioned owning contract; ECR-004 v1 does not counterfeit one.

## 2. Binding inherited invariants

```text
ActionIntent != ActionAttempt
ActionReceipt != VerificationReceipt
executor observed success != VERIFIED
executor observed failure != REJECTED
UNKNOWN execution-receipt state remains UNKNOWN without a real provider receipt
independent effect reconciliation != provider receipt recovery
Fact/Artifact/Receipt metadata != verification truth
VerificationReceipt is the authoritative verification record type
reconciliation != authorization
reconciliation != fabricated provider receipt
reconciliation evidence != ECR-002 RunState mutation
retry advisory != permission or ability to execute
external content/evidence != authority
```

ECR-004 MUST reuse ECR-001 `VerificationReceipt`, `VerificationTarget`, `VerificationMethod`, `VerificationOutcome`, `EvidenceRef`, `ActionRef`, `ActionAttemptRef`, `ReceiptId`, `FactId`, `ArtifactId`, `ClaimRef`, and ECR-002 durable attempt/recovery truth. It MUST NOT create a competing canonical verification receipt, rewrite `ActionReceipt` semantics, clear ECR-002 unresolved attempt state, or add an unversioned ECR-002 run event.

## 3. User stories

### US1 — Independently verify an exact execution target

As a runtime/product consumer, I need an independent verifier to evaluate the exact action, attempt, receipt, fact, artifact, or claim from explicit evidence so executor self-report cannot become completion truth.

Acceptance:
- the request binds one exact canonical `VerificationTarget`;
- verifier identity/method/evidence/outcome are captured in a canonical `VerificationReceipt`;
- non-`NotEvaluated` outcomes require evidence;
- a provider/executor receipt may be evidence but cannot verify itself merely by existing.

### US2 — Preserve UNKNOWN safely

As a recovery path, I need ambiguous external side effects to remain unresolved until reconciliation obtains independent evidence, so Ecra never blindly repeats a consequential action.

Acceptance:
- an unreceipted/unresolved attempt cannot be reclassified by notes, model output, or elapsed time;
- reconciliation binds the exact `RunId`, `ActionAttemptRef`, and underlying `ActionRef`;
- `still_unknown` means the effect claim remains unresolved;
- `effect_confirmed` / `no_effect_confirmed` resolve only the effect claim and do not fabricate the missing provider receipt;
- the original ECR-002 run remains unreceipted/unresolved in v1 and cannot be resumed by ECR-004.

### US3 — Reconcile without fabricating execution truth

As an operator, I need reconciliation to say whether an external effect is independently observed without inventing a missing provider receipt or rewriting the original run.

Acceptance:
- reconciliation records reference verification receipt IDs rather than synthesize `ActionReceipt`;
- `effect_confirmed`, `no_effect_confirmed`, and `still_unknown` are distinct;
- `effect_confirmed` yields a duplicate-retry-blocked advisory;
- `no_effect_confirmed` may yield a semantic advisory describing what ECR-001 retry/idempotency rules would require if a later owning runtime proposes another attempt;
- no advisory authorizes execution, appends an attempt, resumes the original ECR-002 run, or clears its unresolved set.

### US4 — Verify critical points before long-horizon completion

As a long-running workflow owner, I need explicit verification checkpoints for decision-critical outputs and side effects so a final fluent model summary cannot hide an earlier violated constraint.

Acceptance:
- checkpoint requirements bind exact targets and required verification outcomes;
- unsatisfied/rejected/inconclusive checkpoints prevent a `verified-complete` view;
- checkpoint evaluation is deterministic from explicit receipts and requirements;
- checkpoints grant no capability or approval;
- the derived `verified-complete` view does not change ECR-002 `RunPhase`.

### US5 — Decision-grade evidence remains inspectable

As a reviewer, I need verification decisions to preserve evidence identity, snapshot/digest/as-of information when available, method, verifier, and immutable target binding so I can audit why a result was accepted or rejected.

Acceptance:
- mutable external evidence used for a conclusive decision requires an immutable digest/snapshot reference or an explicit non-decision-grade classification;
- evidence age/freshness is explicit when relevant;
- notes cannot substitute for evidence;
- verification aggregation never rewrites source provenance.

### US6 — Conflicting verification remains visible

As a user, I need conflicting independent verifier results to remain visible instead of last-write-wins so uncertainty is not hidden.

Acceptance:
- all accepted receipts remain immutable records;
- deterministic aggregation distinguishes agreement, conflict, inconclusive-only, and absent evidence;
- conflict never silently becomes `Verified`;
- a later receipt does not delete or mutate an earlier result.

## 4. Functional requirements

### Canonical verification boundary

- **FR-001** Reuse ECR-001 `VerificationReceipt` as the only canonical independent verification record.
- **FR-002** Reuse ECR-001 `VerificationTarget`; no parallel action/attempt/receipt/fact/artifact/claim target namespace.
- **FR-003** `ActionReceipt` MUST remain executor-observed execution evidence and MUST NOT become verification by naming, flag, adapter, or serialization shortcut.
- **FR-004** `Fact`, `Artifact`, run state, UI metadata, notes, and model output MUST NOT gain an independent `verified` truth flag.
- **FR-005** A verification request MUST bind one exact target and explicit verifier input; no ambient target selection.
- **FR-006** The verifier method is evidence metadata, not a trust score that automatically determines outcome.
- **FR-007** Every `Verified`, `Rejected`, or `Inconclusive` receipt MUST carry at least one canonical `EvidenceRef`; `NotEvaluated` MUST NOT be promoted by notes alone.
- **FR-008** Verifier principal identity remains optional ECR-001 identity evidence; ECR-004 MUST NOT validate or mint ECR-031 assertions.
- **FR-009** Verification processing MUST be deterministic for identical canonical request/evidence/configuration inputs.
- **FR-010** Unknown fields, unsupported versions, duplicate IDs, target mismatch, and malformed evidence bindings MUST fail closed with typed errors.

### Evidence quality and decision-grade rules

- **FR-011** Evidence references remain references; ECR-004 v1 MUST NOT persist arbitrary raw remote/private payloads in its verification journal.
- **FR-012** Conclusive verification over mutable external state MUST require an immutable content digest/snapshot/artifact binding when available; otherwise the result MUST be classified non-decision-grade or inconclusive.
- **FR-013** Evidence freshness/as-of state MUST be explicit when the verification rule depends on time.
- **FR-014** A receipt-linked `ActionReceipt` may be supporting evidence but cannot alone prove its own success claim without an independent rule evaluating additional observable state or deterministic invariants.
- **FR-015** Model judgment MAY be represented only as ECR-001 `IndependentModelJudgment`; it MUST NOT outrank unavailable structured evidence by default or grant authority.
- **FR-016** Evidence classification/provenance remains owned by ECR-001; verification MUST NOT rewrite provenance to `Observed` or `UserProvided`.
- **FR-017** Verification notes are bounded non-authoritative metadata and MUST NOT contain raw secrets in committed fixtures/log output.

### Aggregation and checkpoints

- **FR-018** Define deterministic aggregation over immutable receipts for one exact target.
- **FR-019** Aggregation MUST distinguish at least `Absent`, `Verified`, `Rejected`, `Inconclusive`, and `Conflicted` views.
- **FR-020** Any simultaneous conclusive `Verified` and `Rejected` evidence MUST aggregate to `Conflicted`, never last-write-wins.
- **FR-021** `NotEvaluated` receipts MUST NOT satisfy a verification requirement.
- **FR-022** Define bounded `VerificationCheckpointV1` requirements over exact targets and required outcomes.
- **FR-023** A checkpoint set MUST evaluate deterministically and expose all unsatisfied/conflicted targets.
- **FR-024** Checkpoints MUST NOT contain capability, approval, policy grant, declassification, or execution authority.
- **FR-025** A `verified-complete` view is an ECR-004 derived view only and MUST NOT rewrite ECR-002 `RunPhase`, remove unresolved attempts, or fabricate a run event.

### UNKNOWN and reconciliation

- **FR-026** Reconciliation MUST bind exact `RunId`, `ActionAttemptRef`, and underlying `ActionRef` and reject cross-run/cross-attempt/cross-action evidence.
- **FR-027** Define `ReconciliationOutcomeV1` with exactly `effect_confirmed`, `no_effect_confirmed`, and `still_unknown` for v1.
- **FR-028** `effect_confirmed` requires conclusive evidence that the exact attempted effect occurred; it MUST produce a duplicate-retry-blocked advisory.
- **FR-029** `no_effect_confirmed` requires conclusive evidence that the exact attempted effect did not occur; absence of evidence alone is insufficient.
- **FR-030** `still_unknown` MUST preserve both the unresolved effect claim and the ECR-002 unresolved-attempt blocker.
- **FR-031** Reconciliation MUST NOT create a synthetic `ActionReceipt` for a provider response that was never observed.
- **FR-032** A reconciliation record MUST reference the verification receipts/evidence used to reach its outcome.
- **FR-033** ECR-004 MAY derive a non-authoritative `RetryAdvisoryV1` after reconciliation, but the advisory MUST be computed from exact ECR-001 `RetryClass` and `IdempotencyClass`, MUST state that the original ECR-002 run remains blocked, and MUST NOT prepare/schedule/authorize a new attempt.
- **FR-034** `RequiresExternalReconciliation` and `NeverBlindRetry` semantics MUST remain fail-closed. A later explicitly owned execution integration may consume ECR-004 evidence, but ECR-004 v1 provides no same-run bypass and no generic “retry allowed” boolean.
- **FR-035** Multiple reconciliation records for one attempt are append-only; disagreement or regression to unknown remains visible.

### Durability and boundaries

- **FR-036** ECR-004 v1 verification/reconciliation records MUST be append-only and independently addressable by stable typed IDs.
- **FR-037** ECR-004 persistence MUST be separate from and MUST NOT silently change the ECR-002 v1 run-event wire contract.
- **FR-038** Persisted records MUST bind canonical target/attempt IDs and verification receipt content strongly enough to detect mismatched substitution.
- **FR-039** ECR-004 v1 may persist only synthetic/non-sensitive fixture evidence metadata until downstream sensitive-storage/privacy gates authorize more.
- **FR-040** Recovery/reopen MUST reproduce the same aggregate verification and reconciliation view from persisted records.
- **FR-041** No browser/network/model/provider/process execution dependency may enter the trusted ECR-004 v1 crate; adapters that acquire live evidence belong to later owning slices.
- **FR-042** No unsafe Ecra-authored code is required for ECR-004 v1.
- **FR-043** Resource bounds MUST cover record count, evidence references per receipt, checkpoint count, notes, and reconciliation scan/evaluation work.
- **FR-044** Typed errors MUST distinguish malformed input, target mismatch, evidence insufficiency, conflict, persistence corruption, and unresolved reconciliation without parsing display text.
- **FR-045** ECR-004 MUST remain usable offline over committed synthetic evidence/records.
- **FR-046** ECR-004 v1 MUST NOT mutate the supplied ECR-002 `RunState`, remove an attempt from `unresolved_attempts`, convert an unreceipted attempt into receipted state, append `RunResumed`/`AttemptPrepared`/any ECR-002 event, or claim that `no_effect_confirmed` makes the original run operationally retryable. Same-run resolution requires a future explicitly versioned integration and is outside this v1 slice.

## 5. Success criteria

- **SC-001** Canonical verification fixtures reject all target/evidence/version mismatches and preserve `ActionReceipt != VerificationReceipt`.
- **SC-002** Identical verification inputs aggregate 1,000 times to byte-equivalent deterministic views.
- **SC-003** Conflicting verified/rejected receipts always aggregate to `Conflicted` with both receipts retained.
- **SC-004** UNKNOWN/reconciliation matrix proves no blind retry for unreceipted, unresolved, non-idempotent, unknown-idempotency, or still-unknown attempts.
- **SC-005** `effect_confirmed` always yields a duplicate-retry-blocked advisory; `no_effect_confirmed` yields only bounded semantic guidance and never execution authorization or same-run retry permission.
- **SC-006** Reconciliation never fabricates `ActionReceipt` and never mutates ECR-002 authoritative execution history.
- **SC-007** Restart/reopen yields identical aggregate/checkpoint/reconciliation views from the same append-only journal.
- **SC-008** Mutable external evidence without required digest/snapshot cannot produce a decision-grade conclusive result.
- **SC-009** Bounded hostile input cannot panic or produce unbounded record/evidence/checkpoint scans within v1 limits.
- **SC-010** Workspace build/fmt/Clippy/tests/rustdoc/offline plus ECR-001/ECR-002 regression suites pass on the exact implementation head.
- **SC-011** Architecture/dependency checks prove no model/browser/network/provider/process/policy/authorization dependency enters ECR-004 trusted code.
- **SC-012** Post-implementation traceability maps every FR/SC to code/tests/contracts with zero unowned MUST requirement.
- **SC-013** For fixtures covering every reconciliation outcome, canonical ECR-002 `RunState` before vs after ECR-004 evaluation is byte/field-equivalent, the original unreceipted attempt remains unresolved, and ECR-002's existing blind-retry/resume guards continue to reject that original run unless a real `ActionReceipt` is later recorded by its owning provider path.

## 6. Explicit non-goals

ECR-004 v1 does not:
- execute external actions or acquire browser/network/provider evidence;
- authorize/re-authorize actions or decide information disclosure;
- replace ECR-003 policy/approval semantics;
- validate ECR-031 identity assertions or hold trust-root secrets;
- create provider-specific success heuristics;
- claim verifier statistical accuracy beyond committed deterministic fixtures;
- persist real sensitive/private evidence payloads;
- change ECR-002 run-event v1 wire semantics;
- clear ECR-002 unresolved attempts, resume the original run, or operationally schedule a reconciled retry;
- define the future versioned run-integration contract that may consume reconciliation evidence;
- implement ECR-005 benchmark scoring, ECR-009 source independence, or ECR-028 public evaluation metrics.

## 7. Definition of done

ECR-004 is not `CLOSED_CANONICAL` until the full Spec Kit package is complete, all FR-001–FR-046 and SC-001–SC-013 are owned and implemented, constitutional G1–G15 are rechecked, exact-head ECR-004 plus ECR-001/ECR-002 regression gates pass, post-implementation analyze/convergence has zero unresolved MUST drift, the exact feature head is merged by an allowed non-rebase method, and required post-merge `main` evidence succeeds.