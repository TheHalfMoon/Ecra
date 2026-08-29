# Ecra Execution Guide

> **Operational start-here document.** Recover live work from this file, platform roadmap/status, active slice packages, and exact GitHub truth; do not depend on private chat state. Live repository/PR/Actions truth overrides stale prose.

## Source-of-truth order

1. `.specify/memory/constitution.md`
2. `EXECUTION.md`
3. `specs/000-ecra-platform/roadmap.md`
4. `specs/000-ecra-platform/STATUS.md`
5. relevant platform architecture/threat/gap/risk/benchmark/decision artifacts
6. `specs/README.md`
7. active slice package(s)
8. exact live branch/head, PR, CI, reviews and changed files

Stale prose must be updated to live evidence, never the reverse.

## Canonically closed foundation

```text
ECR-001 — Trusted Domain Kernel: CLOSED_CANONICAL
Closure ledger head: 85e4bf657b6c33e3f88d83e92e7a35279d177349
Closure CI: 33099434232 — SUCCESS

ECR-002 — Durable Run, Ledger & Budgets: CLOSED_CANONICAL
Final closure-convergence main head: aadc19c972e619222d426674d7542dd9c00dbe44
ECR-002 closure CI: 33155302100 — SUCCESS
ECR-001 regression on closure head: 33155302026 — SUCCESS
```

ECR-004 implementation base is canonical `main` `4fb61f8b41267983fc460c666fddd7781d91653c`, where the permanent ECR-001/ECR-002 workflows both passed before implementation branch creation.

## Parallel trusted-substrate lanes

The dependency graph permits ECR-031 and ECR-004 to progress independently. They remain separate branches/PRs and neither may counterfeit the other's missing evidence.

### Lane A — ECR-031

```text
Slice: ECR-031 — Identity, Trust Root & Sensitive Storage Foundations
Branch: 031-identity-trust-root
PR: #4 — DRAFT / OPEN / NON-CANONICAL
State: BLOCKED_EXTERNAL_NATIVE_ACCEPTANCE
```

Non-native Phase 1–7 work has passed its owning gates, but native macOS Data Protection Keychain acceptance remains externally blocked because the trusted runner user has no valid Apple Development signing identity, suitable provisioning profile, configured developer account or development team.

Repository approval cannot create/infer those Apple credentials. Do not substitute legacy file-based Keychain, ad-hoc signing, plaintext/file/environment/memory fallback, or weakened acceptance. ECR-031 remains non-canonical until its own live package/PR evidence proves otherwise.

### Lane B — ECR-004

```text
Slice: ECR-004 — Verification & Reconciliation
Branch: 004-verification-receipts-impl
Review PR: #7 — READY / OPEN / NON-CANONICAL
Superseded Draft PR: #6 — CLOSED / NOT MERGED
Canonical implementation base: 4fb61f8b41267983fc460c666fddd7781d91653c
Lifecycle: IMPLEMENTING_MERGE_READY
Current frontier: T052 after exact live governance-head gate
```

PR #6 was replaced as the review container only because the connected ready-for-review GraphQL mutation was incompatible with the live GitHub schema. PR #7 opened over the same branch; no implementation diff was altered by that container replacement.

Verified implementation/review checkpoints:

```text
Phase 1  e223ba5fbf8c375c580e7a93f524be3fd4c311fa  run 33237728338  SUCCESS
Phase 2  40c18b4bcf1e6c124587cdfbc0e423822eb5b138  run 33245650032  SUCCESS
T011A    75cac2aed9099d7ba82295c442b37764b284302c  run 33245970650  SUCCESS
Phase 3  f5181ca4f903f2d039463b03b3e328b1fa9c30dd  run 33246658250  SUCCESS
Phase 4  412de3f481d84154c5c2a85f11c6a6da0c89e35a  run 33247226826  SUCCESS
Phase 5  fb3fdf1ce113a55d3d7276f54681a7f55dc542b3  run 33247815573  SUCCESS
Phase 6  18ad19ae4b4f4d5f48270485af666e7204b95a0e  run 33249643366  SUCCESS
T040     815b95ed0f95513e583aa077f04e863998d0d425  run 33250068524  SUCCESS
T041     2a86dd909abfcb9d8658eab589787eb376a73004  run 33250250973  SUCCESS
T043     67207e1bc91434555bfe31997f4af9f641324a76  run 33250358128  SUCCESS
T045     90ed1bbeafea72ee655bc58a96e94696096f360e  run 33251037913  SUCCESS
Pre-review T050  882b4ef7358aef6c416dd1b9dd67602e86334a06  run 33251589848  SUCCESS
T051 remediation  fde10b37c17f8113b81c78cf87c0de717909ab59  run 33255382842  SUCCESS
```

The T051 remediation job `99108056542` passed locked metadata/build, rustfmt, strict Clippy, workspace tests, complete ECR-001 regressions, complete ECR-002 regressions, every explicit ECR-004 quickstart target including review hardening, dedicated ECR-002 unresolved-state compatibility acceptance, rustdoc, offline replay, all unsafe/dependency boundaries and dependency evidence.

T046/T047 traceability/constitution closure and T048/T049 convergence remain satisfied. Cubic produced 19 findings on PR #7; all valid findings were repaired forward-only, all review threads are resolved, and Cubic reports all findings addressed. CodeRabbit commit status is successful and exposes no actionable blocker. Review-only non-actionable findings were resolved with explicit rationale rather than weakening the required architecture/source/sentinel boundaries. T051 is complete.

Because the T051 task/status/execution ledger convergence itself changes the branch head, the exact live governance-converged head must pass the complete ECR-004 workflow before T052 uses it as the expected merge head.

## ECR-004 frozen boundaries

- ECR-001 `VerificationReceipt` is the only canonical independent verification record.
- `ActionReceipt` remains executor-observed evidence and cannot self-verify, even when the receipt itself has an immutable digest/artifact binding.
- UNKNOWN reconciliation never fabricates `ActionReceipt` or mutates ECR-002 run-event truth.
- aggregate/checkpoint/reconciliation views preserve conflict and grant no authority.
- `semantically_retryable*` is advisory only for a future new-attempt proposal and must revalidate any supplied reconciliation record against canonical supporting receipts.
- every reconciliation outcome leaves the original ECR-002 prepared/unreceipted/unresolved state and `unresolved_attempts` unchanged.
- `RunPhase` is unchanged by every ECR-004 reconciliation outcome.
- same-run `RunResumed`, `ExecutionCompleted`, and blind-retry guards remain authoritative.
- ECR-002 `RunEvent` v1 remains unchanged; no sidecar projection represents run resolution.
- ECR-004 uses a separate append-only verification journal with rebuildable projections.
- journal chaining is local integrity/corruption/substitution detection only, not hostile full-store tamper resistance.
- persisted v1 acceptance is synthetic/non-sensitive evidence metadata/references/digests only.
- no browser/network/model/provider/process/policy/authorization/identity/telemetry runtime dependency enters `ecra-verify`.
- ECR-004 exposes no provider execution, authorization, declassification, identity validation, secret-storage or same-run repair API.

## ECR-004 remaining closure order

```text
complete exact-head ECR-004 + ECR-001 + ECR-002 gate on the live governance-converged PR #7 head
  ↓
re-check PR #7 exact head, reviews, threads, checks and mergeability
  ↓
T052 merge that exact expected head by allowed non-rebase method
     then require ECR-004 + ECR-001 + ECR-002 workflows on canonical main
  ↓
T053 mark CLOSED_CANONICAL only from post-merge evidence;
     update roadmap/status/index/EXECUTION and re-evaluate dependent eligibility
```

No merge, PASS or `CLOSED_CANONICAL` claim may be inferred from historical green.

## Dependency consequences

- ECR-003 remains blocked until ECR-031 is `CLOSED_CANONICAL`.
- ECR-005 requires ECR-003, ECR-004 and ECR-031 in addition to ECR-001/ECR-002; ECR-004 closure alone will not make ECR-005 implementation-eligible.
- ECR-004 may finish despite the ECR-031 external native-host blocker because ECR-031 is not an ECR-004 dependency.
- No later slice may be pulled forward merely because one lane is blocked.

## CI architecture

The repository-scoped self-hosted macOS runner `macbook` is the trusted execution oracle for current Rust slices. Persistent personal runners must not execute untrusted fork PR code.

ECR-001/ECR-002 workflows remain permanent push gates on `main`. ECR-004 has a permanent trusted push-only branch/main workflow that includes all explicit quickstart targets, ECR-001/ECR-002 regressions, unresolved-state compatibility, rustdoc/offline, boundaries and dependency evidence.

Historical green cannot be reused after a content change to claim exact-head PASS.

## Execution rule

Follow each active slice's `tasks.md` dependency order. Fix actual CI/review blockers forward-only and immediately resume. Do not weaken tests/security boundaries to make gates green. No force-push, rebase or destructive history rewriting. Never mark PASS, MERGED, `VERIFIED_ON_BRANCH`, or `CLOSED_CANONICAL` without the exact evidence required by the owning package.