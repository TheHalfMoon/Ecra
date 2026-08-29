# ECR-004 Status — Verification & Reconciliation

**Slice:** ECR-004  
**Lifecycle:** IMPLEMENTING  
**Dependencies:** ECR-001 `CLOSED_CANONICAL`, ECR-002 `CLOSED_CANONICAL`  
**Canonical implementation base:** `4fb61f8b41267983fc460c666fddd7781d91653c`  
**Implementation branch:** `004-verification-receipts-impl`  
**Implementation PR:** #6 (Draft)  
**Constitution:** v1.1.0

ECR-004 planning became canonical through merged PR #5. The exact canonical planning head `4fb61f8b41267983fc460c666fddd7781d91653c` then passed both required dependency regressions:

```text
ECR-001 CI  33237289643  SUCCESS  exact head 4fb61f8b41267983fc460c666fddd7781d91653c
ECR-002 CI  33237289693  SUCCESS  exact head 4fb61f8b41267983fc460c666fddd7781d91653c
```

Therefore implementation branch `004-verification-receipts-impl` was legally created from that exact canonical SHA.

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

## Phase 1 — workspace, dependency and CI boundary

T001–T004 are implemented on the branch:

- T001 exact dependency/license/advisory/MSRV admission is recorded in `research.md` and `research/donor-license-ledger.md`;
- T002 added dependency-minimal `crates/ecra-verify` with `#![forbid(unsafe_code)]` and explicit pure-logic/local-journal separation;
- T003 added `scripts/check-verify-unsafe.sh` and `scripts/check-verify-deps.sh`;
- T004 added permanent trusted push-only `.github/workflows/ecr-004.yml` with `contents: read` only;
- the temporary branch-only lockfile bootstrap changed only `Cargo.lock` and was removed before the Phase 1 acceptance head.

T005 exact-head evidence:

```text
HEAD   e223ba5fbf8c375c580e7a93f524be3fd4c311fa
RUN    33237728338
JOB    99061549466
RESULT SUCCESS
```

Every required step succeeded on that exact head: locked metadata/build, rustfmt, strict Clippy, workspace tests, ECR-001 regressions, ECR-002 regressions, ECR-004 Phase 1 targets, rustdoc, offline replay, ECR-001/ECR-002/ECR-004 unsafe/dependency boundaries, and ECR-004 dependency/toolchain evidence.

Phase 1 is therefore `VERIFIED_ON_BRANCH`; this is not `CLOSED_CANONICAL` for ECR-004.

## Current execution state

```text
CURRENT_TASK                    T006
CURRENT_STATE                   IMPLEMENTING
IMPLEMENTATION_BASE             4fb61f8b41267983fc460c666fddd7781d91653c
IMPLEMENTATION_BRANCH           004-verification-receipts-impl
IMPLEMENTATION_PR               6_DRAFT
T001                             COMPLETE
T002                             COMPLETE
T003                             COMPLETE
T004                             COMPLETE
T005                             COMPLETE_EXACT_HEAD
PHASE_1_HEAD                     e223ba5fbf8c375c580e7a93f524be3fd4c311fa
PHASE_1_RUN                      33237728338
PHASE_1_JOB                      99061549466
PHASE_1_RESULT                   SUCCESS
T006                             ELIGIBLE
T007_PLUS                        NOT_REACHED
```

## Canonical next order

```text
T006 IDs/version/errors
  ↓
T007 strict VerificationRequestV1
  ↓
T008 fixtures/tests
  ↓
T009 request -> canonical VerificationReceipt
  ↓
T010 architecture/type boundaries
  ↓
T011 exact-head Phase 2 gate
  ↓
T011A canonical EvidenceRef read-only accessors + unchanged ECR-001 semantics
```

No Phase 3 evidence logic starts before T011A completes and its exact regression evidence exists.

## Parallel ECR-031 boundary

ECR-031 remains a separate Draft implementation PR with the native macOS provisioning blocker at T064/T068. ECR-004 does not depend on ECR-031 and must not absorb its identity/protected-storage scope or use its blocker to authorize real sensitive evidence persistence.

ECR-005 remains blocked by its complete dependency set.