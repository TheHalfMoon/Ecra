# Ecra Execution Guide

> **Operational start-here document.** This file tells a human or coding agent what is active, what is next, and which repository documents govern the work.

## Source-of-truth order

Before any material implementation or architecture change, read in this order:

1. `.specify/memory/constitution.md` — binding governance and Definition of Done.
2. `EXECUTION.md` — current active slice, branch/PR status, phase ledger, and next eligible work.
3. `specs/000-ecra-platform/roadmap.md` — immutable ECR slice IDs and dependency graph.
4. `specs/000-ecra-platform/{architecture,threat-model,gap-audit,risk-register,benchmark-matrix,decision-log}.md` as relevant.
5. `specs/README.md` — navigation across Spec Kit packages.
6. The active slice package, especially `STATUS.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`, `plan.md`, `tasks.md`, `quickstart.md`, `analyze.md`, and checklists.
7. Exact current Git/GitHub branch, head, PR, CI, review, and changed-file truth.

If a status line in an older planning document conflicts with exact live repository evidence, update the stale planning document; never downgrade live evidence to match stale prose.

## Current execution truth

Active slice: **ECR-001 — Trusted Domain Kernel**  
Package: `specs/001-trusted-domain-kernel/`  
Implementation branch: `001-trusted-domain-kernel`  
PR: `#1` — draft until the entire ECR-001 slice satisfies closure gates.  
Latest fully verified implementation head: `946e95366ed681c724192cd01ece199d5e8f55a7`.

At that head ECR-001 CI run `33083362584` passed:

```text
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
cargo test --doc --workspace --locked
cargo test --workspace --locked --offline
bash scripts/check-core-deps.sh
```

Do not treat this SHA as permanent. Always re-read the branch before mutation.

## ECR-001 phase ledger

| Phase | Tasks | Outcome | Branch evidence |
|---|---:|---|---|
| 1 — Reproducible Rust Workspace | T001–T006 | `VERIFIED_ON_BRANCH` | workspace/toolchain/lints/CI/dependency boundary established |
| 2 — Version, Errors, IDs, Time, Canonicalization, Digests | T007–T014 | `VERIFIED_ON_BRANCH` | deterministic zero-I/O primitives and tests |
| 3 — Actors, Principals, Origins, Resources, Scope | T015–T023 | `VERIFIED_ON_BRANCH` | normative fixtures and Actor→Principal compile-fail coverage |
| 4 — Capability Request/Grant, Delegation, Time | T024–T028 | `VERIFIED_ON_BRANCH` | exact-head `992dd31c…` green |
| 5 — Information, Observation, Fact, Freshness, Evidence, Artifact | T029–T038 | `VERIFIED_ON_BRANCH` | exact-head `d29f700c…` green |
| 6 — Information Use / Source-to-Sink Intent | T039–T042 | `VERIFIED_ON_BRANCH` | exact-head `b0f4ae4c…`; CI `33075545972` green |
| 7 — Effects, Idempotency, Retry, Action Digest | T043–T051 | `VERIFIED_ON_BRANCH` | exact-head `ea177363…`; CI `33078470973` green |
| 8 — Attempts, Receipts, Independent Verification | T052–T057 | `VERIFIED_ON_BRANCH` | exact-head `0b273f41…`; CI `33080355344` green |
| 9 — Strict v1 Contract / Fixture Runner / Portability | T058–T062 | `VERIFIED_ON_BRANCH` | exact-head `946e9536…`; CI `33083362584` green |
| 10 — Cross-cutting Security / Architecture Gates | T063–T069 | `NEXT_ACTIVE_PHASE` | dependency/unsafe/error/free-form/provenance/architecture/offline convergence |
| 11 — Closure / Analyze / Canonicalization | T070–T076 | `BLOCKED_BY_PHASE_10` | final traceability, analyze, PR/merge/post-merge evidence |

`VERIFIED_ON_BRANCH` is not `CLOSED_CANONICAL`. The slice becomes `CLOSED_CANONICAL` only after all ECR-001 tasks, convergence/analyze, exact-head gates, PR merge, and required post-merge evidence are complete.

## Immediate next work

Continue ECR-001 from Phase 10 in `tasks.md`:

```text
T063 verify/strengthen dependency-boundary automation against FR-050 prohibited categories
T064 add explicit static/CI proof of zero unsafe code in addition to crate-level forbid lint
T065 cover every machine-readable ErrorCode/ErrorCategory without display-string parsing
T066 audit free-form metadata fields and prove/document non-authoritative semantics
T067 update canonical donor/license ledger for exact locked dependencies and no-source-copy provenance
T068 add ecra-core README architecture map and seven mandatory misuse warnings
T069 record exact-head offline/no-service-access evidence
```

Existing evidence already includes `scripts/check-core-deps.sh`, a CI dependency-boundary step, `#![forbid(unsafe_code)]`, and the offline replay gate. Do not duplicate them mechanically: verify the canonical task and FR wording, strengthen only gaps, then record exact evidence.

Required invariants remain:

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
```

Phase 10 remains zero-I/O trusted-core convergence. Do not add runtime orchestration, filesystem/process execution abstractions, network clients, clocks, policy engines, browser/model SDKs, or provider protocols.

`implementation-clarifications.md` contains bounded implementation resolutions discovered while completing Phases 5–8. These must be folded into primary canonical documents during Phase 11 before ECR-001 can close.

After every bounded implementation batch:

```text
implement
  ↓
format
  ↓
build --locked
  ↓
Clippy -D warnings
  ↓
tests + rustdoc
  ↓
offline replay
  ↓
dependency boundary
  ↓
inspect exact head / PR / CI
  ↓
fix defects before advancing
```

## Platform execution path

The detailed dependency graph is canonical in `specs/000-ecra-platform/roadmap.md`. For orientation, the intended program is:

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

The semantic capability ladder should prefer stronger interfaces over weaker ones:

```text
Native API / A2A / MCP
  ↓ unavailable
WebMCP
  ↓ unavailable
Verified compiled Ecra Skill
  ↓ unresolved
Accessibility tree / semantic DOM
  ↓ insufficient
WebDriver BiDi / CDP provider
  ↓ insufficient
Vision / coordinates
  ↓
Full computer-use fallback
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

### Cross-cutting / later program

ECR-022 through ECR-030 own sync, registry, supply chain, privacy/diagnostics, accessibility/i18n, source compliance, public benchmarks, portability, and ecosystem gateway. Their exact eligibility comes from the canonical roadmap dependencies, not from this simplified wave view.

## Constitutional architecture invariants

Never cross these boundaries merely to make progress faster:

- Actor attribution is not authenticated Principal identity.
- Web/model/tool/memory content is observation, not instruction authority.
- Missing or empty scope is never implicit wildcard.
- Resource locator text is non-authoritative metadata.
- CapabilityRequest is not CapabilityGrant.
- Read authority does not imply disclosure authority.
- Information classification/provenance/freshness do not grant permission.
- Fact has no mutable `verified` truth flag.
- ActionIntent is not ActionAttemptRef.
- ActionReceipt is not VerificationReceipt.
- Executor-observed success is not independent verification.
- UNKNOWN external outcomes are never silently coerced to success or failure.
- Generic ContentDigest is not an authenticity/security proof.
- Remote model/search/tool/protocol calls are information-disclosure boundaries.
- External protocols are adapters, not Ecra's internal trusted domain model.
- No unbounded agent/tool/process loops when runtime slices arrive.
- No source reuse without donor/license provenance.

## How to add or change the plan

For a new feature or major idea:

1. Map it to an existing `ECR-###` slice if possible.
2. If no slice owns it, amend the spec-of-specs with a new immutable ECR ID and explicit dependencies.
3. Update architecture/threat/gap/risk/benchmark documents if the idea changes a trust boundary, persistent data class, remote egress path, public claim, browser patch, protocol, or privileged capability.
4. Create the bounded Spec Kit package before implementation.
5. Run clarify/research/plan/tasks/analyze as required.
6. Only then implement from the first eligible task.

## Handoff rule

A continuation prompt should not need a private chat handoff. The next executor must be able to recover from repository truth by reading this file, the active `STATUS.md`, the roadmap, active tasks, exact branch/PR/CI, and any open reviews.

When work moves forward, update `EXECUTION.md` and the active slice `STATUS.md` in the same PR so the repository remains self-explanatory.
