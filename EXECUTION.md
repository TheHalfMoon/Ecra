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

Current canonical `main` before ECR-004 planning merge: `f6d8eb6ff6a60aa0ad8a6f52686a62f12cd374b0`.

## Parallel eligible execution lanes

The dependency graph currently permits two separate lanes. They MUST remain separate branches/PRs and one lane must not counterfeit the other's missing evidence.

### Lane A — ECR-031 implementation

```text
Slice: ECR-031 — Identity, Trust Root & Sensitive Storage Foundations
Live branch: 031-identity-trust-root
Live PR: #4 — DRAFT / OPEN / NON-CANONICAL
Last verified live PR head: d5e7079f0bf9f7a8848fbee9829e0994985ff38c
Current task frontier: T064
Current state: BLOCKED_EXTERNAL_NATIVE_ACCEPTANCE
```

Phase checkpoints already verified on the implementation branch:

```text
Phase 1  0289596bb7cdbb81d5f03c445fd324e985294143  run 33161529028  SUCCESS
Phase 2  4ddb6da267ebc90647e27fde382385a9d2529452  run 33163366128  SUCCESS
Phase 3  7eaede3f9f10461c307c8900c021273a4dbffa03  run 33165941748  SUCCESS
Phase 4  217934d1f2c334b943349af87bcf40a4ad44b889  run 33196312711  SUCCESS
Phase 5  bd066fa501476ff4f7fe43d0f4153de1e8d2fc60  run 33198508505  SUCCESS
Phase 6  64c34744dd05b9850d8c9657a87e46913bd23412  run 33200973225  SUCCESS
```

Phase 7 non-native implementation evidence head `4f2c150d2e5fd882d8554cd32a8aea4d4c5da639`, run `33235282966`, passed locked build, fmt, strict Clippy, workspace tests, ECR-001/ECR-002 regressions, explicit ECR-031 targets, rustdoc, offline replay and all boundary/dependency/toolchain checks. Only the live macOS Data Protection Keychain acceptance step failed closed.

Trusted-runner readiness evidence `33235282975` / `33235454670` shows:

```text
interactive console user     available
runner == console user       yes
codesign/Xcode tools          available
code-signing identity         absent
local provisioning profile   absent
Xcode developer account      absent
Xcode development team       absent
```

T064 therefore requires an external macOS host prerequisite: configure a valid Apple developer account/team in Xcode for the same user that owns the self-hosted runner and allow Xcode to create/install an Apple Development code-signing identity and suitable provisioning profile for an app-like Data Protection Keychain acceptance host.

Repository approval alone cannot create or infer Apple account credentials, team identity, certificates or provisioning profiles. Do not substitute legacy file-based Keychain, ad-hoc signing, plaintext/file/environment/memory fallback, or weaken the live acceptance requirement.

ECR-031 remains:

```text
T064 [BLOCKED_EXTERNAL_NATIVE_ACCEPTANCE]
  ↓
T068
  ↓
T069–T074
  ↓
T075–T082
```

Do not move PR #4 out of Draft, merge it, start ECR-031 Phase 8, or claim `VERIFIED_ON_BRANCH`/`CLOSED_CANONICAL` while T064/T068 remain open.

### Lane B — ECR-004 planning / next implementation-eligible slice

```text
Slice: ECR-004 — Verification & Reconciliation
Planning branch: 004-verification-receipts
Planning PR: #5
Planning base: f6d8eb6ff6a60aa0ad8a6f52686a62f12cd374b0
Last converged planning head before this EXECUTION commit: 764dce1c089ead4138cb5faed07c3a706946eff3
Planning lifecycle: TASKS_READY candidate / non-canonical until planning merge
Analyze Pass: 3
Analyze result: ZERO_BLOCKING_PLANNING_DRIFT_FOUND
FR-001–FR-046: OWNED
SC-001–SC-013: OWNED
A-001: REMEDIATED by IC-001 / T011A
A-002: REMEDIATED by IC-002 / FR-046 / SC-013 / Phase 5 compatibility gates
Implementation authorized: NO
```

ECR-004 is independently eligible from ECR-001/ECR-002 and does not depend on ECR-031. Its package freezes these boundaries:

- reuse ECR-001 `VerificationReceipt` as the only canonical independent verification record;
- `ActionReceipt` remains executor-observed evidence and cannot self-verify;
- UNKNOWN reconciliation never fabricates an `ActionReceipt` or mutates ECR-002 run-event truth;
- aggregate/checkpoint/reconciliation views preserve conflict and grant no authority;
- retry disposition is safety advisory only and `semantically_retryable*` applies to a future new-attempt proposal, not same-run scheduling;
- every reconciliation outcome leaves ECR-002 prepared/unreceipted/unresolved state, `unresolved_attempts`, and `RunPhase` unchanged;
- same-run `RunResumed`, `ExecutionCompleted`, and blind-retry guards remain authoritative for the unresolved prior attempt;
- ECR-002 `RunEvent` v1 remains unchanged and no sidecar projection represents run resolution;
- ECR-004 uses a separate append-only verification journal with rebuildable projections;
- journal digest chaining is integrity/corruption detection only, not hostile complete-store tamper resistance;
- persisted v1 acceptance is synthetic/non-sensitive evidence metadata/references/digests only;
- no browser/network/model/provider/process/policy execution dependency enters `ecra-verify`;
- IC-001 permits only typed read-only accessors for already-existing ECR-001 `EvidenceRef` metadata, with unchanged ECR-001 wire/canonical semantics and mandatory ECR-001 regressions;
- IC-002 prohibits ECR-004 from clearing ECR-002 unresolved state or counterfeiting a run-repair protocol.

ECR-004 implementation MUST NOT start directly from the planning branch. Required order:

```text
1. Converge planning package/platform/index/EXECUTION/PR text on Analyze Pass 3.
2. Process PR #5 planning review findings to zero actionable blocking drift on the exact converged head.
3. Merge exact planning head by an allowed non-rebase method.
4. Freeze resulting canonical main SHA.
5. Require permanent ECR-001 and ECR-002 workflows SUCCESS on that exact SHA.
6. Create implementation branch 004-verification-receipts-impl from that exact green canonical SHA.
7. Execute specs/004-verification-receipts/tasks.md from T001 in dependency order.
8. Never claim ECR-004 CLOSED_CANONICAL until T052 merge plus T053 post-merge evidence.
```

## Dependency consequences

- ECR-003 remains implementation-blocked until ECR-031 is `CLOSED_CANONICAL`.
- ECR-005 requires ECR-003, ECR-004 and ECR-031 in addition to ECR-001/ECR-002, so ECR-004 alone cannot unlock it.
- ECR-004 may continue despite the ECR-031 native-host blocker because the roadmap intentionally gives it only ECR-001/ECR-002 dependencies.
- No other later slice may be pulled forward merely because one lane is blocked.

## CI architecture

The repository-scoped self-hosted macOS runner `macbook` is the trusted execution oracle for current Rust slices. Persistent personal runners must not execute untrusted fork PR code.

Closed ECR-001/ECR-002 workflows remain permanent push gates on `main`. ECR-031 has its own trusted push-only branch/main gate including live native acceptance. ECR-004 implementation must add its own trusted push-only workflow with explicit request/evidence/aggregate/checkpoint/reconcile/journal/store/boundary targets plus ECR-001/ECR-002 regressions and explicit unresolved-state compatibility acceptance.

Historical green cannot be reused after a content change to claim exact-head PASS.

## Execution rule

Follow each active slice's `tasks.md` dependency order. Fix actual CI/review blockers forward-only and immediately resume. Do not weaken tests/security boundaries to make gates green. No force-push, rebase or destructive history rewriting. Never mark PASS, MERGED, `VERIFIED_ON_BRANCH`, or `CLOSED_CANONICAL` without the exact evidence required by the owning package.