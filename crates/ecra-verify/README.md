# ecra-verify

`ecra-verify` is the ECR-004 verification and reconciliation crate. It consumes canonical ECR-001 domain values, reads ECR-002 run/attempt state where reconciliation needs exact durable binding, and owns a separate local verification journal. It does not own authorization or provider execution.

## Exact v1 flow

The supported v1 flow is intentionally narrow:

1. Build a strict `VerificationRequestV1` for one exact canonical `VerificationTarget`.
2. Assess the request with `DecisionGradeRuleV1` using only supplied evidence metadata and supplied evaluation time.
3. Produce the canonical ECR-001 `VerificationReceipt` only when the requested outcome is supported by the decision-grade rules.
4. Derive `VerificationAggregateViewV1` from canonical receipts for one exact target.
5. Evaluate `VerificationCheckpointV1` from aggregate views when a critical decision point needs explicit satisfied/unsatisfied/conflicted targets.
6. For an unresolved ECR-002 action attempt, derive a `ReconciliationRecordV1` from exact supporting verification receipts without changing ECR-002 run truth.
7. Persist accepted synthetic/non-sensitive verification receipts, checkpoint definitions and reconciliation records in the separate append-only verification journal when durable local replay is required.

`VerificationReceipt` is the single canonical independent verification record. `ActionReceipt` remains executor-observed execution evidence and never self-promotes to verification.

## Verification requests and decision-grade evidence

`VerificationRequestV1` is strict, versioned and bounded. It binds the proposed canonical verification receipt ID, verifier, optional principal reference, exact target, verification method, evidence references, proposed outcome, explicit evaluation time when supplied, bounded rule ID and bounded notes.

Unknown fields, unsupported versions, duplicate evidence IDs and resource-limit violations fail closed.

A conclusive `Verified`, `Rejected` or `Inconclusive` result is not justified by a method label alone. Decision-grade assessment evaluates the supplied evidence shape. Mutable external references require an immutable artifact/content/snapshot binding when the rule requires one. Freshness-sensitive rules use explicit `as_of` metadata and the supplied evaluation time; there is no ambient clock or remote fetch inside verification semantics.

An execution receipt cannot independently verify itself. Independent model judgment also cannot outrank missing required independent evidence. Verification does not rewrite the original provenance, freshness or dispute axes of ECR-001 facts.

## Aggregation and checkpoints

`VerificationAggregateViewV1` is deterministic and target-exact. Its closed states are:

- `Absent`
- `Verified`
- `Rejected`
- `Inconclusive`
- `Conflicted`

All canonical supporting receipt IDs are retained. Receipt input ordering does not change the aggregate. `Verified + Rejected` is always `Conflicted`; there is no last-write-wins rule.

`VerificationCheckpointV1` is a bounded collection of unique exact-target requirements. A checkpoint evaluation reports explicit satisfied, unsatisfied and conflicted target sets. `Absent`, `Inconclusive` and `Conflicted` never satisfy a v1 requirement. A specialized negative requirement may explicitly accept `Rejected`.

A checkpoint is not authority. It contains no capability grant, approval, policy decision, declassification, secret handle or execution permission.

## Reconciliation and UNKNOWN outcomes

Reconciliation binds all of the following before deriving an outcome:

- exact ECR-002 `RunId`;
- exact durable unresolved `ActionAttemptRef`;
- the attempt's underlying exact `ActionRef`;
- exact canonical supporting `VerificationReceipt` IDs targeting that attempt.

The closed reconciliation outcomes are:

- `effect_confirmed` — explicit conclusive effect evidence exists;
- `no_effect_confirmed` — explicit conclusive no-effect evidence exists;
- `still_unknown` — evidence is absent, insufficient or conflicting.

Absence of an executor/provider receipt is never proof of no effect. `verification_receipts` is normally non-empty; the sole empty-support exception is `still_unknown` when no supporting verification receipt exists. Conclusive reconciliation always requires resolved supporting verification evidence with the required immutable binding.

Reconciliation is read-only with respect to ECR-002. For every reconciliation outcome, ECR-002 `RunState`, `PreparedAttemptState`, `RunPhase` and `unresolved_attempts` membership remain unchanged. ECR-004 does not construct or append ECR-002 `RunEvent` values and does not synthesize `ActionReceipt` values.

Therefore reconciliation does **not** repair, resume or complete the existing ECR-002 run. The same-run `RunResumed`, `ExecutionCompleted` and blind-retry guards remain authoritative.

## Retry disposition is advisory only

`RetryDispositionV1` classifies safety evidence after reconciliation. It can indicate duplicate block, reconciliation required, semantic retryability, exact-same-key semantic retryability, or an explicit nonblind path.

`SemanticallyRetryable` and `SemanticallyRetryableSameKey` mean only that a **future owning path may consider a new attempt proposal** under the canonical ECR-001 retry/idempotency semantics. They do not authorize, schedule or execute that attempt, do not resume the existing run, and do not override ECR-002 retry guards.

ECR-004 exposes no execution, scheduling, policy or authorization method.

## Verification journal and local store

`VerificationJournalEntryV1` is a strict versioned append-only envelope with:

- a positive bounded sequence;
- the exact prior entry digest for non-genesis entries;
- a typed verification-receipt/checkpoint/reconciliation body;
- repository-aligned JCS canonical material;
- a domain-separated SHA-256 entry digest.

`VerificationStore` persists the authoritative `verification_journal` separately from ECR-002 run storage. SQLite update/delete triggers block ordinary mutation of canonical journal rows. Receipt/checkpoint/reconciliation indexes are rebuildable projections only; deleting or poisoning a projection cannot redefine canonical journal truth.

Append uses expected-head compare-and-append semantics. Competing writers using the same prior head allow exactly one canonical successor. Replay validates sequence continuity, previous-digest linkage, entry digest/material consistency, row metadata and canonical identity uniqueness. Queries are bounded.

The v1 acceptance surface persists only synthetic/non-sensitive identifiers, metadata, references and digests. Raw secret/private payload persistence is outside ECR-004 v1 acceptance. Secret sentinels are tested against journal rows and derived debug material.

## Offline and dependency boundary

Semantic verification performs no browser, network, provider, model, process, policy or authorization execution. The crate can replay its local synthetic verification journal offline.

The implementation reuses the repository-approved ECR-001/ECR-002, Serde/JSON/JCS, SHA-256 and bundled SQLite dependency set. No copied donor source is required for ECR-004 v1.

`#![forbid(unsafe_code)]` applies to Ecra-authored `ecra-verify` Rust.

## Assurance claim and explicit non-claims

The journal digest chain provides deterministic integrity/corruption/substitution detection when the stored chain is replayed under its stated local assumptions. It can detect broken sequence/linkage/digest/material relationships.

ECR-004 v1 has no independently protected external root/head anchor. It therefore does **not** claim hostile full-store tamper resistance against an adversary able to rewrite the entire store consistently.

ECR-004 also does **not** claim:

- verifier infallibility;
- provider authenticity merely because provider-shaped evidence exists;
- cryptographic authenticity from a generic content digest alone;
- exactly-once external effects;
- that executor-observed success is independently verified;
- that reconciliation resolves ECR-002 unresolved run state;
- that semantic retryability grants permission to retry;
- that local data or local execution is automatically trusted.

Those guarantees, where needed, belong to their explicit owning security, identity, policy, provider or execution contracts rather than being inferred by `ecra-verify`.
