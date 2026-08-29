# ECR-004 Status — Verification & Reconciliation

**Slice:** ECR-004  
**Lifecycle:** TASKS_READY_CANDIDATE / PLANNING_NON_CANONICAL  
**Dependencies:** ECR-001 `CLOSED_CANONICAL`, ECR-002 `CLOSED_CANONICAL`  
**Planning base:** `f6d8eb6ff6a60aa0ad8a6f52686a62f12cd374b0`  
**Planning branch:** `004-verification-receipts`  
**Constitution:** v1.1.0

ECR-004 is independently planning-eligible from ECR-001/ECR-002. This branch contains planning only and does not authorize implementation until the package is merged to canonical `main` and the exact merged planning state passes the required ECR-001/ECR-002 regression gates.

## Planning package

```text
spec.md
research.md
data-model.md
contracts/verification-reconciliation-v1.md
threat-model.md
plan.md
tasks.md
quickstart.md
implementation-clarifications.md
analyze.md
checklists/requirements.md
STATUS.md
```

## Frozen v1 boundaries

- reuse ECR-001 `VerificationReceipt` as the only canonical independent verification record;
- `ActionReceipt` remains executor-observed execution evidence and never self-verifies;
- no second `verified` flag on Fact/Artifact/run metadata;
- exact target/evidence/verifier/method/outcome binding;
- deterministic aggregate states: `Absent`, `Verified`, `Rejected`, `Inconclusive`, `Conflicted`;
- critical verification checkpoints are requirements, not authority;
- exact ECR-002 UNKNOWN attempt reconciliation produces `effect_confirmed`, `no_effect_confirmed`, or `still_unknown` without fabricating `ActionReceipt`;
- retry disposition is fail-closed safety metadata, never execution authorization;
- ECR-002 `RunEvent` v1 wire contract is unchanged;
- ECR-004 uses a separate append-only verification journal with rebuildable indexes;
- journal hash chaining is corruption/substitution detection only, not hostile complete-store tamper resistance;
- acceptance persists synthetic/non-sensitive evidence metadata/references/digests only;
- no browser/network/model/provider/process/policy/identity-backend execution dependency enters v1.

## Analyze result

Pass 1 found one blocking planning issue: canonical ECR-001 `EvidenceRef` keeps decision-grade metadata private and exposes only `id()`/`kind()`.

IC-001 resolves this without a competing evidence model: implementation may add only read-only accessors for the existing artifact/observation/receipt/external-ref/content-digest/as-of fields, with no wire/canonical/validation change and full ECR-001 regressions. `tasks.md` T011A owns the prerequisite before decision-grade evidence logic.

Pass 2 result:

```text
PASS_1_BLOCKERS_FOUND=1
PASS_1_BLOCKERS_REMEDIATED=1
FR_TOTAL=45
FR_OWNED=45
FR_UNOWNED=0
SC_TOTAL=12
SC_OWNED=12
SC_UNOWNED=0
MUST_LEVEL_PLANNING_GAPS=0
FAILED_CONSTITUTION_GATES=0
CROSS_ARTIFACT_BLOCKING_CONTRADICTIONS=0
RESULT=ZERO_BLOCKING_PLANNING_DRIFT_FOUND
```

## Planned implementation architecture

After planning becomes canonical and exact dependency regressions pass, create a fresh implementation branch from that exact canonical head and add one `crates/ecra-verify` crate:

```text
error.rs
ids.rs
request.rs
evidence.rs
aggregate.rs
checkpoint.rs
reconcile.rs
journal.rs
store.rs
```

The crate consumes canonical ECR-001/ECR-002 types and keeps pure verification logic separate from local sidecar journal I/O.

## Current execution state

```text
CURRENT_TASK                    PLANNING_CONVERGENCE_AND_REVIEW
CURRENT_STATE                   TASKS_READY_CANDIDATE_NON_CANONICAL
PLANNING_BASE                   f6d8eb6ff6a60aa0ad8a6f52686a62f12cd374b0
PLANNING_BRANCH                 004-verification-receipts
ANALYZE_RESULT                  ZERO_BLOCKING_PLANNING_DRIFT_FOUND
IMPLEMENTATION_AUTHORIZED       NO
NEXT_IF_PLANNING_MERGED_GREEN   CREATE_IMPLEMENTATION_BRANCH_FROM_EXACT_CANONICAL_HEAD
```

## Canonical next steps

1. converge platform lifecycle/index documentation with this package;
2. open the ECR-004 planning PR against current canonical `main`;
3. process all actionable planning-review findings;
4. merge the exact planning head by an allowed non-rebase method;
5. require ECR-001 and ECR-002 permanent workflows to succeed on the exact resulting canonical `main` head;
6. create the ECR-004 implementation branch from that exact eligible head;
7. execute `tasks.md` in dependency order.

## Parallel ECR-031 boundary

ECR-031 is a separate active implementation PR and currently has a native macOS provisioning prerequisite. ECR-004 does not depend on ECR-031, so planning may proceed independently. ECR-004 must not absorb ECR-031 identity/protected-storage scope or use its blocker as justification to persist real sensitive evidence.

ECR-005 remains blocked by its complete dependency set and does not become eligible merely because ECR-004 planning is ready.