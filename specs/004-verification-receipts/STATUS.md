# ECR-004 Status — Verification & Reconciliation

**Slice:** ECR-004  
**Lifecycle:** IMPLEMENTING  
**Dependencies:** ECR-001 `CLOSED_CANONICAL`, ECR-002 `CLOSED_CANONICAL`  
**Canonical implementation base:** `4fb61f8b41267983fc460c666fddd7781d91653c`  
**Implementation branch:** `004-verification-receipts-impl`  
**Implementation PR:** pending first-diff PR creation  
**Constitution:** v1.1.0

ECR-004 planning became canonical through merged PR #5. The exact canonical planning head `4fb61f8b41267983fc460c666fddd7781d91653c` then passed both required dependency regressions:

```text
ECR-001 CI  33237289643  SUCCESS  exact head 4fb61f8b41267983fc460c666fddd7781d91653c
ECR-002 CI  33237289693  SUCCESS  exact head 4fb61f8b41267983fc460c666fddd7781d91653c
```

Therefore implementation branch `004-verification-receipts-impl` was legally created from that exact canonical SHA. No implementation work may claim a later phase until its exact task/gate evidence exists.

## Frozen v1 boundaries

- reuse ECR-001 `VerificationReceipt` as the only canonical independent verification record;
- `ActionReceipt` remains executor-observed execution evidence and never self-verifies;
- no second `verified` flag on Fact/Artifact/run metadata;
- exact target/evidence/verifier/method/outcome binding;
- deterministic aggregate states: `Absent`, `Verified`, `Rejected`, `Inconclusive`, `Conflicted`;
- critical verification checkpoints are requirements, not authority;
- exact ECR-002 UNKNOWN attempt reconciliation produces `effect_confirmed`, `no_effect_confirmed`, or `still_unknown` without fabricating `ActionReceipt`;
- retry disposition is fail-closed advisory metadata for a future new-attempt proposal only, never execution authorization or same-run scheduling;
- every reconciliation outcome leaves ECR-002 `RunState`, prepared-attempt receipt/unresolved state, `unresolved_attempts`, and `RunPhase` unchanged;
- ECR-002 `RunEvent` v1 wire contract is unchanged and no run-resolution event is introduced;
- ECR-004 uses a separate append-only verification journal with rebuildable indexes;
- no sidecar projection represents or mutates ECR-002 run resolution;
- journal hash chaining is corruption/substitution detection only, not hostile complete-store tamper resistance;
- acceptance persists synthetic/non-sensitive evidence metadata/references/digests only;
- no browser/network/model/provider/process/policy/identity-backend execution dependency enters v1.

## Analyze history

### Pass 1 — A-001

IC-001 authorizes only read-only accessors for already-existing canonical ECR-001 `EvidenceRef` metadata, with no wire/canonical/validation change and full ECR-001 regressions. T011A owns the implementation prerequisite.

### Pass 2 review — A-002

IC-002 + FR-046 + SC-013 freeze a read-only compatibility boundary: ECR-004 records effect truth and advisory new-attempt semantics only. It does not clear ECR-002 unresolved state, append an ECR-002 event/receipt, resume/complete the existing run, or schedule a retry.

### Analyze Pass 3

```text
FR_TOTAL=46
FR_OWNED=46
FR_UNOWNED=0
SC_TOTAL=13
SC_OWNED=13
SC_UNOWNED=0
MUST_LEVEL_PLANNING_GAPS=0
FAILED_CONSTITUTION_GATES=0
CROSS_ARTIFACT_BLOCKING_CONTRADICTIONS=0
RESULT=ZERO_BLOCKING_PLANNING_DRIFT_FOUND
```

## T001 dependency admission

Implementation-time review is recorded in `research.md` on this branch.

Accepted runtime boundary:

```text
ecra-core        workspace path
ecra-run         workspace path
serde            1.0.229 / derive
serde_json       1.0.151
serde_jcs        0.2.0
sha2             0.11.0
thiserror        2.0.20
uuid             1.26.0 / serde only
rusqlite         =0.40.2 / default-features=false / bundled
```

Accepted dev-only boundary:

```text
proptest         1.11.0
tempfile         3.27.0
```

Rejected for ECR-004: ZIP, URL parsing, network/browser/model/provider/process/protocol/policy/telemetry/identity-backend runtimes, duplicate canonicalization/hash/DB libraries, and donor source reuse.

Current advisory review explicitly covers the existing `libsqlite3-sys`/`sha2` historical advisories and the August 2026 malicious `arrayref`/`proc-macro1` supply-chain campaign. No matching `arrayref`/`proc-macro1` dependency path was found in the authorization state.

T001 is not marked complete until the donor/license ledger delta is committed on this branch.

## Current execution state

```text
CURRENT_TASK                    T001
CURRENT_STATE                   IMPLEMENTING
IMPLEMENTATION_BASE             4fb61f8b41267983fc460c666fddd7781d91653c
IMPLEMENTATION_BRANCH           004-verification-receipts-impl
IMPLEMENTATION_PR               PENDING_FIRST_DIFF_CREATION
T001_RESEARCH_REVIEW            RECORDED
T001_DONOR_LEDGER_DELTA         PENDING
T002                            NOT_REACHED
PHASE_1_EXACT_HEAD_GATE         NOT_REACHED
```

## Canonical next order

```text
1. Complete T001 donor/license ledger delta.
2. Open Draft implementation PR from this branch to main.
3. T002 add dependency-minimal ecra-verify workspace crate.
4. T003 add unsafe/dependency boundary scripts.
5. T004 add permanent ECR-004 trusted push-only CI.
6. T005 require exact-head Phase 1 gate SUCCESS before T006.
```

## Parallel ECR-031 boundary

ECR-031 remains a separate Draft implementation PR with the native macOS provisioning blocker at T064/T068. ECR-004 does not depend on ECR-031 and must not absorb its identity/protected-storage scope or use its blocker to authorize real sensitive evidence persistence.

ECR-005 remains blocked by its complete dependency set.