# Ecra Platform — Spec of Specs

**Status:** CANONICAL_PLANNING  
**Roadmap ID namespace:** `ECR-###`  
**Created:** 2026-08-27  
**Governed by:** `.specify/memory/constitution.md`

This document decomposes Ecra into bounded, independently implementable Spec Kit slices. IDs are immutable once referenced by another slice, issue, commit, PR, contract, or benchmark report. Numeric order is an identifier namespace, not a license to ignore explicit dependencies.

The roadmap is not a release calendar. It is an architectural dependency graph. A slice becomes implementation-eligible only when its dependencies are `CLOSED_CANONICAL`, or the slice explicitly authorizes bounded fixture-only/research work that cannot counterfeit a missing dependency.

## Platform Objective

Build the default trusted gateway between human/model intent and digital information/action through a shared substrate for browser, search, workspace, memory, skills, terminal, developer workflows, data, plugins, and external agents/models.

## Ordering Rules

1. Security/trust semantics precede privileged autonomy.
2. Identity/principal and information-flow semantics precede real sensitive privileged execution.
3. Durable fixture/local state precedes long-horizon autonomy; sensitive persistence additionally requires the trust-root/storage gate.
4. Stock-browser prototypes precede a maintained browser distribution.
5. Evidence/search contracts precede broad “answer engine” claims.
6. Skill compilation requires verified trajectories; replay requires canonical skills; repair requires replay.
7. Terminal/Data/Developer reuse the same Actor/Principal/Capability/InformationFlow/Receipt/Verifier model rather than inventing a second one.
8. Ecosystem/registry work follows stable extension contracts.
9. Custom model training is deferred until Ecra owns enough verified trajectory/evaluation data to justify it.
10. A remote provider call (model, search, tool, protocol) is an information-disclosure boundary and must be authorized as such.

## Roadmap

| ID | Slice | Primary outcome | Depends on | Status | Sub-spec |
|---|---|---|---|---|---|
| ECR-001 | Trusted Domain Kernel | Versioned zero-I/O domain types/invariants for actor/principal refs, origin/resource/scope, information labels, capability request/grant, provenance, action/action-attempt refs, receipts and verification | — | CLOSED_CANONICAL | `specs/001-trusted-domain-kernel/` |
| ECR-002 | Durable Run, Ledger & Budgets | Serializable run machine, unique execution attempts, append-only integrity-chained local ledger, portable `.ecra` fixture/run artifact, cancellation/resource budgets | ECR-001 | CLOSED_CANONICAL | `specs/002-durable-run-ledger/` |
| ECR-003 | Authority, Information Flow, Policy & Secrets | Fail-closed capability and source-to-sink disclosure evaluation, immutable authorization decision/lease, approval binding, origin authority, secret handles, policy adapter | ECR-001, ECR-002, ECR-031 | PLANNED | `specs/003-authority-policy-secrets/` |
| ECR-004 | Verification & Reconciliation | Independent verifier framework, executor-observed vs verified outcomes, UNKNOWN handling, reconciliation, critical-point verification, immutable decision-grade evidence | ECR-001, ECR-002 | TASKS_READY | `specs/004-verification-receipts/` |
| ECR-005 | Evaluation & Threat Harness | Golden fixtures plus security/information-flow/durability/resource-bound/verification benchmark harness used by later slices | ECR-001, ECR-002, ECR-003, ECR-004, ECR-031 | PLANNED | `specs/005-evaluation-threat-harness/` |
| ECR-006 | Stock Firefox / WebDriver BiDi Prototype | Bounded browser control against stock Firefox with observations, receipts, origin transitions, permission brokerage experiments and takeover events | ECR-001–ECR-005, ECR-031 | PLANNED | `specs/006-firefox-bidi-prototype/` |
| ECR-007 | Browser Foundation & Upstream Strategy | Reproducible/traceable Firefox-derived build, patch ledger, IPC threat contract, update/rebase policy, extension/profile compatibility/trust model | ECR-006 | PLANNED | `specs/007-browser-foundation/` |
| ECR-008 | Ecra Browser Wedge | Daily human browser with Spaces/Containers as session partitions plus Ecra authority isolation, human/agent/shared tabs, trusted chrome, control ownership, permissions and visible authority | ECR-003, ECR-004, ECR-006, ECR-007, ECR-031 | PLANNED | `specs/008-browser-wedge/` |
| ECR-009 | Search Evidence Fabric | Shared evidence contract, provider/egress abstraction, source identity/independence, snapshots/freshness, contradiction handling, local index | ECR-001, ECR-002, ECR-003, ECR-004 | PLANNED | `specs/009-search-evidence-fabric/` |
| ECR-010 | Workspace & Memory | Durable scoped workspace, provenance/information-flow-aware memories, derived-index lifecycle, aging/deletion/export, candidate-memory policy | ECR-001–ECR-004, ECR-009, ECR-031 | PLANNED | `specs/010-workspace-memory/` |
| ECR-011 | Browser-Native Semantic Capabilities | Capability router across WebMCP/native APIs/compiled skills/AX-DOM/BiDi/vision fallback, with concrete action resolution before authorization | ECR-003, ECR-004, ECR-006, ECR-009 | PLANNED | `specs/011-semantic-capability-router/` |
| ECR-012 | Skill IR | Versioned typed executable workflow IR with artifacts/dataflow, capability requirements (not captured grants), disclosure constraints, side effects, pre/postconditions and verifiers | ECR-001–ECR-004 | PLANNED | `specs/012-skill-ir/` |
| ECR-013 | Skill Compiler | Human or verified-agent trajectory → authority-free candidate skill → sandbox validation → versioned skill | ECR-005, ECR-010, ECR-011, ECR-012 | PLANNED | `specs/013-skill-compiler/` |
| ECR-014 | Deterministic Replay | Low/no-model execution of compatible skills with fresh authorization, exact attempts/receipts and compatibility checks | ECR-012, ECR-013 | PLANNED | `specs/014-skill-replay/` |
| ECR-015 | Divergence & Repair | Assumption tracking, localized repair, downstream invalidation, re-authorization/re-verification and version promotion | ECR-014 | PLANNED | `specs/015-divergence-repair/` |
| ECR-016 | Protocol Gateway | Version-pinned MCP/ACP/A2A/Agent Skills adapters with explicit external identity/audience mapping and least-authority state exposure | ECR-001–ECR-004, ECR-009, ECR-010, ECR-012, ECR-031 | PLANNED | `specs/016-protocol-gateway/` |
| ECR-017 | Plugin & Sandbox Runtime | Signed/versioned extension model, Wasm/process isolation, capability manifests, parser/native tiers, resource limits | ECR-003, ECR-004, ECR-005, ECR-016 | PLANNED | `specs/017-plugin-sandbox/` |
| ECR-018 | Terminal Execution | Human/agent terminal sessions using the same principal/capability/information-flow/receipt/verifier/run model with bounded process trees | ECR-002–ECR-005, ECR-017, ECR-031 | PLANNED | `specs/018-terminal/` |
| ECR-019 | Developer Workspace | Repo graph/context, current docs, trust-tiered repository inspection/execution, tests/builds, browser QA, code review/release evidence | ECR-009, ECR-010, ECR-016, ECR-018 | PLANNED | `specs/019-developer-workspace/` |
| ECR-020 | Data & Analytics | Files/SQL/API analytics with lineage, scoped disclosure, reproducible transformations, evidence-backed conclusions | ECR-004, ECR-009, ECR-010, ECR-017, ECR-031 | PLANNED | `specs/020-data-analytics/` |
| ECR-021 | Local Model Gateway | Provider-neutral local inference with model-artifact provenance/security, bounded execution, and Ecra search/context/memory/skills/actions/verifiers | ECR-009–ECR-017, ECR-024 | PLANNED | `specs/021-local-model-gateway/` |
| ECR-022 | Optional Sync & Multi-Device | User-controlled encrypted sync for portable workspaces/memory/skills/policies without becoming core-required cloud | ECR-002, ECR-003, ECR-010, ECR-012, ECR-031 | DEFERRED | `specs/022-sync-multidevice/` |
| ECR-023 | Extension Registry & Trust | Discovery, signing, provenance, review metadata and compatibility for plugins/skills/connectors/verifiers | ECR-012, ECR-016, ECR-017 | DEFERRED | `specs/023-extension-registry/` |
| ECR-024 | Release, Update & Supply Chain | Artifact-specific reproducibility/provenance targets, SBOM, signing, update channels, security response, dependency/license automation | ECR-005, ECR-007 | PLANNED | `specs/024-release-supply-chain/` |
| ECR-025 | Privacy, Telemetry & Diagnostics | Local diagnostics, explicit telemetry/remote-egress contracts, redaction, crash reporting, retention/export controls | ECR-002, ECR-003, ECR-008, ECR-031 | PLANNED | `specs/025-privacy-diagnostics/` |
| ECR-026 | Accessibility, Internationalization & Human UX Quality | Accessibility, keyboard/screen-reader behavior, localization and non-AI browser-quality gates | ECR-007, ECR-008 | PLANNED | `specs/026-accessibility-i18n/` |
| ECR-027 | Search/Content Compliance & Source Policy | robots/access policy, source licensing/attribution, caching/retention, publisher controls, parser/download safety | ECR-009, ECR-017 | PLANNED | `specs/027-search-source-policy/` |
| ECR-028 | Public Benchmark & Research Program | Reproducible benchmark adapters/reports for web, security, information flow, long-horizon, search trust and local-model augmentation | ECR-005 plus relevant feature slices | PLANNED | `specs/028-benchmark-program/` |
| ECR-029 | Migration, Import & Export | Import/export/deletion propagation for browser state, workspaces, runs, skills, memories, derived indexes and policies | ECR-008, ECR-010, ECR-012 | DEFERRED | `specs/029-portability/` |
| ECR-030 | Ecosystem Gateway | Stable developer SDK/local API and production-quality third-party agent/model infrastructure surface | ECR-016, ECR-017, ECR-023, ECR-024, ECR-025 | DEFERRED | `specs/030-ecosystem-gateway/` |
| ECR-031 | Identity, Trust Root & Sensitive Storage Foundations | Identity/principal assertions and on-behalf-of binding; device/user-local trust root; key lifecycle/revocation; protected sensitive-storage/authenticity envelope semantics | ECR-001, ECR-002 | TASKS_READY | `specs/031-identity-trust-root/` |

## Critical Path

```text
ECR-001 Trusted Domain Kernel [CLOSED_CANONICAL]
  ↓
ECR-002 Durable Run, Ledger & Budgets [CLOSED_CANONICAL]
  ├───────────────────────────────┐
  ↓                               ↓
ECR-031 Identity / Trust Root     ECR-004 Verification
[TASKS_READY; exact-head CI gate] [TASKS_READY; canonical-planning gate]
  ↓
ECR-003 Authority / Information Flow / Policy
  └───────────┬──────────┘
              ↓
ECR-005 Evaluation & Threat Harness
              ↓
ECR-006 Stock Firefox Prototype
              ↓
ECR-007 Browser Foundation
              ↓
ECR-008 Browser Wedge

Parallel after trusted policy/verification:
ECR-009 Search → ECR-010 Workspace/Memory

Convergence:
ECR-011 Semantic Capability Router
ECR-012 Skill IR
        ↓
ECR-013 Compiler → ECR-014 Replay → ECR-015 Repair
        ↓
ECR-016 Protocol Gateway / ECR-017 Plugin Runtime
        ↓
ECR-018 Terminal → ECR-019 Developer
ECR-020 Data
ECR-021 Local Model Gateway
```

## Why These Slices Exist

### Trust is separated from browser UX

Ecra must be able to prove identity/principal binding, information-flow constraints, authority, receipts, restart semantics, budgets and verification against fixtures before privileged browser integration exists. Otherwise browser code becomes the accidental source of trust semantics.

### Identity/trust root is explicit

`ActorId` is audit attribution, not proof of authenticated identity. Privileged execution needs explicit identity assertions, on-behalf-of relationships, revocation and protected key/storage semantics. These are owned by ECR-031 rather than being hidden inside Cedar, Firefox, MCP or a database.

### Browser foundation is separated from browser product

A daily browser creates an enduring upstream maintenance obligation. Stock Firefox/BiDi proves product mechanics first; only then does Ecra earn the cost of a distribution/fork. Firefox Containers are useful site-data/session partitions; Ecra authority policy remains the agent security boundary.

### Search is separated from memory

Search evidence is ephemeral/retrieved truth. Memory is durable user-owned context. Remote search is also a data-egress boundary. Combining these before authority/provenance/disclosure contracts exist risks retrieved content becoming durable authority or private context leaking to providers.

### Skill IR is separated from compiler/replay/repair

The IR must be stable and independently testable before generated skills or repair logic depend on it. Skills describe required authority; they never embed captured live grants/approvals/secrets from a demonstration.

### Gateway is separated from protocols

MCP/ACP/A2A are adapters. Their authentication/token semantics are mapped into Ecra identity assertions and capabilities; protocol credentials are not passed through or treated as ambient local authority.

## Sensitive-Data Progression Rule

ECR-002 may persist only synthetic/non-sensitive fixtures and local test runs in its v1 acceptance/product authorization. ECR-031 defines the protected local identity/trust/storage substrate, but its `TASKS_READY` planning state alone does not authorize downstream slices to persist real authenticated browser secrets, sensitive workspace content, or equivalent high-value state. ECR-004's independently ready planning package likewise authorizes only synthetic/non-sensitive evidence metadata/reference persistence in its v1 acceptance. Downstream sensitive-state use remains gated by implemented ECR-031 plus relevant ECR-003/ECR-025 contracts.

A hash/integrity chain may detect accidental/local corruption under its stated assumptions. Do not claim hostile tamper resistance unless a protected trust anchor, MAC/signature or external anchor supports the claim.

## Cross-Cutting Work That Must Never Be “Later”

Every affected slice MUST add/update as part of Definition of Done:

- threat model;
- identity/principal boundary where relevant;
- information classification and source-to-sink/remote-egress analysis;
- donor/license ledger;
- dependency/advisory review;
- migrations/backward compatibility for persisted formats;
- redaction/secret handling;
- accessibility for user-facing flows;
- observability without hidden telemetry;
- cancellation/timeouts/resource budgets;
- benchmark/acceptance fixtures;
- exact documentation for privileged behavior;
- export/deletion/portability implications.

## Status Semantics

- `PLANNED` — decomposed but no implementation is authorized by this roadmap alone.
- `PLANNING_REWORK` — review found blocking planning defects; implementation is forbidden until corrected and re-analyzed.
- `SPEC_READY` — complete `spec.md` with no unresolved blocking clarification.
- `PLAN_READY` — research/data model/contracts/plan complete and constitution gates pass.
- `TASKS_READY` — traceable executable `tasks.md` exists and the latest analyze pass has no critical planning defect. Repository execution rules may still require an exact-head CI gate before the implementation branch is created.
- `IMPLEMENTING` — implementation branch/PR active.
- `BLOCKED` — dependency or evidence gate prevents safe continuation.
- `CLOSED_CANONICAL` — exact implemented state satisfies spec, plan, tasks, tests, analysis/convergence and documentation.
- `DEFERRED` — intentionally outside current critical path; may not be pulled forward without explicit dependency/strategy review.

## Current Slice

`ECR-001 Trusted Domain Kernel` and `ECR-002 Durable Run, Ledger & Budgets` are `CLOSED_CANONICAL`. ECR-002's final closure-convergence head `aadc19c972e619222d426674d7542dd9c00dbe44` passed ECR-002 CI `33155302100` and ECR-001 regression CI `33155302026`.

`ECR-031 Identity, Trust Root & Sensitive Storage Foundations` is `TASKS_READY` in canonical planning. Its live implementation branch/PR state is tracked by `EXECUTION.md`, its slice `STATUS.md`, PR #4 and exact Actions truth rather than by this architectural row.

`ECR-004 Verification & Reconciliation` is now `TASKS_READY`: FR-001–FR-045 and SC-001–SC-012 are owned, Analyze Pass 2 found zero blocking planning drift after IC-001 remediated the single Pass-1 blocker, and G1–G15 pass/are explicitly N/A. This planning state must become canonical and the exact resulting `main` head must pass required ECR-001/ECR-002 regressions before an ECR-004 implementation branch is created.

ECR-003 remains implementation-blocked until ECR-031 is `CLOSED_CANONICAL`.