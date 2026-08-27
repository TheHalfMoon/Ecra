# Ecra Execution Guide

> **Operational start-here document.** This file tells a human or coding agent what is active, what is next, and which repository documents govern the work.

## Source-of-truth order

Before any material implementation or architecture change, read in this order:

1. `.specify/memory/constitution.md` — binding governance and Definition of Done.
2. `EXECUTION.md` — current active slice, branch/PR status, phase ledger, and next eligible work.
3. `specs/000-ecra-platform/roadmap.md` — immutable ECR slice IDs and dependency graph.
4. `specs/000-ecra-platform/STATUS.md` — compact platform lifecycle truth.
5. platform architecture/threat/gap/risk/benchmark/decision documents as relevant.
6. `specs/README.md` — package navigation.
7. Active slice package, especially `STATUS.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`, `implementation-clarifications.md`, `plan.md`, `quickstart.md`, `tasks.md`, analyze artifacts, and checklists.
8. Exact current GitHub branch/head, PR, CI, review, and changed-file truth.

If prose conflicts with exact live evidence, update stale prose; never downgrade repository truth to match an old status line.

## Current execution truth

Active slice: **ECR-001 — Trusted Domain Kernel**  
Package: `specs/001-trusted-domain-kernel/`  
Implementation branch: `001-trusted-domain-kernel`  
PR: `#1` — OPEN / DRAFT / mergeable at the last live check.  
Lifecycle: `IMPLEMENTING_CONVERGENCE`.

Latest fully verified implementation head before docs convergence:

```text
5dfe4c09b2abceeec14bc94b8e13d2dccddfd37c
```

Exact-head Phase 10 CI:

```text
run 33086490495 — success
```

That run passed:

```text
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
cargo test --doc --workspace --locked
cargo test --workspace --locked --offline
bash scripts/check-core-unsafe.sh
bash scripts/check-core-deps.sh
```

Do not treat any SHA here as permanent. Re-read PR/head/CI before mutation.

## ECR-001 phase ledger

| Phase | Tasks | Outcome | Evidence / next state |
|---|---:|---|---|
| 1 — Reproducible Rust Workspace | T001–T006 | `VERIFIED_ON_BRANCH` | workspace/toolchain/lints/CI established |
| 2 — Version, Errors, IDs, Time, Canonicalization, Digests | T007–T014 | `VERIFIED_ON_BRANCH` | deterministic zero-I/O primitives |
| 3 — Actors, Principals, Origins, Resources, Scope | T015–T023 | `VERIFIED_ON_BRANCH` | typed authority boundaries/fixtures |
| 4 — Capability Request/Grant, Delegation, Time | T024–T028 | `VERIFIED_ON_BRANCH` | request/grant separation and temporal shape |
| 5 — Information, Observation, Fact, Freshness, Evidence, Artifact | T029–T038 | `VERIFIED_ON_BRANCH` | provenance/classification/freshness/artifact contracts |
| 6 — Information Use / Source-to-Sink Intent | T039–T042 | `VERIFIED_ON_BRANCH` | CI `33075545972` |
| 7 — Effects, Idempotency, Retry, Action Digest | T043–T051 | `VERIFIED_ON_BRANCH` | CI `33078470973` |
| 8 — Attempts, Receipts, Independent Verification | T052–T057 | `VERIFIED_ON_BRANCH` | CI `33080355344` |
| 9 — Strict v1 Contract / Fixture Runner / Portability | T058–T062 | `VERIFIED_ON_BRANCH` | CI `33083362584` |
| 10 — Cross-cutting Security / Architecture Gates | T063–T069 | `VERIFIED_ON_BRANCH` | exact head `5dfe4c09…`, CI `33086490495` |
| 11 — Traceability / Analyze / Closure | T070–T076 | `PARTIAL` | T074 analyze found blocking canonical drift; T075 activated convergence |
| 12 — Convergence | T077–T080 | `ACTIVE` | T077 complete on branch; T078 active, T079 next, T080 final exact-head convergence gate |

`VERIFIED_ON_BRANCH` is not `CLOSED_CANONICAL`.

## Why Phase 12 exists

Post-implementation analyze artifact:

```text
specs/001-trusted-domain-kernel/post-implementation-analyze-2026-08-27.md
Decision: CONVERGENCE_REQUIRED
```

The implementation/test suite was stronger and more precise than several planning-era canonical documents. Closure is blocked until the primary contract, data model, verification guide and execution ledgers match the implemented v1 semantics and complete traceability proves no remaining MUST-level drift.

## Immediate next work

Follow `specs/001-trusted-domain-kernel/tasks.md` in order:

```text
T077 DONE_ON_BRANCH
  Fold C1–C12 plus real version/error semantics into primary data-model + contract.

T078 ACTIVE
  Converge quickstart + active STATUS + EXECUTION + task ledger to live truth.

T079 NEXT
  Produce one exact traceability artifact for FR-001–FR-055,
  SC-001–SC-020, constitution G1–G15, and all ECR-001-owned
  pre-implementation-review findings, with downstream deferrals explicit.

T080 BLOCKED_BY_T078_T079
  Run revised quickstart, exact-head CI and analyze-equivalent review.
  Only zero blocking drift can authorize PR readiness.
```

The active slice status is:

```text
specs/001-trusted-domain-kernel/STATUS.md
```

There is no root `STATUS.md`; do not invent one. `AGENTS.md` names both platform and active-slice status files explicitly.

## Current ECR-001 invariants

Never cross these boundaries to make convergence faster:

```text
Actor != authenticated Principal
CapabilityRequest != CapabilityGrant
classification != permission
InformationUse != authorization
locator != resource security identity
ActionDigest != signature/approval
ActionIntent != ActionAttemptRef != ActionReceipt != VerificationReceipt
executor_observed_success != verified
UNKNOWN remains UNKNOWN
ContentDigest != ActionDigest/security proof
free-form reason/label/notes/provider text != authority
```

Additional converged v1 facts:
- malformed/missing strict Versioned envelope -> `serialization_failed`;
- unsupported major/minor use dedicated compatibility codes;
- machine API is exactly 16 ErrorCategory variants / 19 ErrorCode variants;
- ActionParametersRef binds every non-empty payload reference with SecurityDigest;
- `ActionSemantics` is construction-only; wire JSON keeps effect/idempotency/retry flat;
- verified/rejected/inconclusive VerificationReceipt requires evidence; not_evaluated may have none;
- repository inner-body fixtures do not weaken the public `Versioned<T>` wire contract.

## Convergence verification gate

After every material convergence batch, CI must remain capable of passing:

```bash
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
cargo test --doc --workspace --locked
cargo test --workspace --locked --offline
bash scripts/check-core-unsafe.sh
bash scripts/check-core-deps.sh
```

Before T080 closure analysis, also run every dedicated contract/security target named in the revised `quickstart.md`.

No later ECR implementation becomes eligible after a failed ECR-001 gate.

## Closure sequence

ECR-001 closure order is:

```text
finish T077–T079
  ↓
run T080 exact-head quickstart + CI + analyze
  ↓ zero blocking drift
finish T070–T073 traceability/closure evidence disposition
  ↓
make PR ready only if governance/readiness criteria pass
  ↓
review/fix exact current head as required
  ↓
merge without force-push/rebase/destructive history rewriting
  ↓
post-merge exact-main verification
  ↓
update platform roadmap/status + EXECUTION
  ↓
CLOSED_CANONICAL
  ↓
identify next dependency-eligible ECR slice
```

T076 must not set roadmap `CLOSED_CANONICAL` before the merge/post-merge evidence required by the Definition of Done.

## Platform execution path

The canonical dependency graph is `specs/000-ecra-platform/roadmap.md`. High-level orientation remains:

### Wave A — Trusted substrate

```text
ECR-001 Trusted Domain Kernel
  ↓
ECR-002 Durable Run / Ledger / Budgets
  ├── ECR-031 Identity / Trust Root / Sensitive Storage
  └── ECR-004 Verification / Reconciliation
  ↓
ECR-003 Authority / Information Flow / Policy / Secrets
  ↓
ECR-005 Evaluation & Threat Harness
```

### Wave B — Browser wedge

```text
ECR-006 Stock Firefox / WebDriver BiDi Prototype
  ↓
ECR-007 Browser Foundation / Upstream Strategy
  ↓
ECR-008 Ecra Browser Wedge
```

### Wave C — Trusted knowledge and context

```text
ECR-009 Search Evidence Fabric
  ↓
ECR-010 Workspace & Memory
  ↓
ECR-011 Browser-Native Semantic Capabilities
```

### Wave D — Learn once, replay cheaply

```text
ECR-012 Skill IR
  ↓
ECR-013 Skill Compiler
  ↓
ECR-014 Deterministic Replay
  ↓
ECR-015 Divergence & Repair
```

### Wave E — Agent/developer ecosystem

```text
ECR-016 Protocol Gateway
ECR-017 Plugin & Sandbox Runtime
  ↓
ECR-018 Terminal Execution
ECR-019 Developer Workspace
ECR-020 Data & Analytics
ECR-021 Local Model Gateway
```

ECR-022 through ECR-031 remain governed by the canonical roadmap, not this orientation summary.

## Handoff rule

A continuation prompt must not require private chat state. The next executor should recover by reading:

1. this file;
2. `specs/001-trusted-domain-kernel/STATUS.md`;
3. active `tasks.md` + converged contract/data model/quickstart;
4. post-implementation analyze and traceability artifacts;
5. live PR #1 head, CI, reviews and changed-file truth.

Update `EXECUTION.md` and active `STATUS.md` whenever execution materially advances.