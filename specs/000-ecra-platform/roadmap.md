# Ecra Platform — Spec of Specs

**Status:** CANONICAL_PLANNING  
**Roadmap ID namespace:** `ECR-###`  
**Created:** 2026-08-27  
**Governed by:** `.specify/memory/constitution.md`

This document decomposes the Ecra platform into bounded, independently implementable Spec Kit slices. IDs are immutable once referenced by another slice, issue, commit, PR, contract, or benchmark report.

The roadmap is not a release calendar. It is an architectural dependency graph. A slice becomes implementation-eligible only when its dependencies are CLOSED_CANONICAL or the slice explicitly documents a safe fixture-only/mock boundary that does not counterfeit a dependency.

## Platform Objective

Build the default trusted gateway between human/model intent and digital information/action through a shared substrate for browser, search, workspace, memory, skills, terminal, developer workflows, data, plugins, and external agents/models.

## Ordering Rules

1. Security/trust semantics precede privileged autonomy.
2. Stock-browser prototypes precede a maintained browser distribution.
3. Evidence/search contracts precede broad “answer engine” claims.
4. Durable state precedes long-horizon autonomy.
5. Skill compilation requires verified trajectories; replay requires canonical skills; repair requires replay.
6. Terminal/Data/Developer reuse the browser trust model rather than inventing a second one.
7. Ecosystem/registry work follows stable extension contracts.
8. Custom model training is deferred until Ecra owns enough verified trajectory/evaluation data to justify it.

## Roadmap

| ID | Slice | Primary outcome | Depends on | Initial status | Sub-spec |
|---|---|---|---|---|---|
| ECR-001 | Trusted Domain Kernel | Versioned, zero-I/O domain types and invariants for actor/origin/capability/provenance/action/receipt/verification | — | PLANNED | `specs/001-trusted-domain-kernel/` |
| ECR-002 | Durable Run & Ledger | Serializable run machine, append-only/tamper-evident local ledger, portable `.ecra` run artifact | ECR-001 | PLANNED | `specs/002-durable-run-ledger/` |
| ECR-003 | Authority, Policy & Secrets | Fail-closed capability evaluation, approval binding, origin authority, secret handles, policy adapter | ECR-001, ECR-002 | PLANNED | `specs/003-authority-policy-secrets/` |
| ECR-004 | Verification & Side-Effect Semantics | Independent verifier framework, UNKNOWN handling, retry/idempotency classes, critical-point verification | ECR-001, ECR-002 | PLANNED | `specs/004-verification-receipts/` |
| ECR-005 | Evaluation & Threat Harness | Golden fixtures plus security/durability/verification benchmark harness used by later slices | ECR-001, ECR-002, ECR-003, ECR-004 | PLANNED | `specs/005-evaluation-threat-harness/` |
| ECR-006 | Stock Firefox / WebDriver BiDi Prototype | Bounded browser control against stock Firefox with observations, receipts, origin tracking and takeover events | ECR-001–ECR-005 | PLANNED | `specs/006-firefox-bidi-prototype/` |
| ECR-007 | Browser Foundation & Upstream Strategy | Reproducible Firefox-derived build, patch ledger, update/rebase policy, extension/profile compatibility contract | ECR-006 | PLANNED | `specs/007-browser-foundation/` |
| ECR-008 | Ecra Browser Wedge | Daily human browser with Spaces/Containers, human/agent/shared tabs, control ownership and visible authority | ECR-003, ECR-004, ECR-006, ECR-007 | PLANNED | `specs/008-browser-wedge/` |
| ECR-009 | Search Evidence Fabric | Shared evidence contract, provider abstraction, source ranking, freshness, contradiction handling, local index | ECR-001, ECR-002, ECR-004 | PLANNED | `specs/009-search-evidence-fabric/` |
| ECR-010 | Workspace & Memory | Durable scoped workspace, provenance-aware memories, aging/deletion/export, candidate-memory policy | ECR-001–ECR-004, ECR-009 | PLANNED | `specs/010-workspace-memory/` |
| ECR-011 | Browser-Native Semantic Capabilities | Capability router across WebMCP/native APIs/compiled skills/AX-DOM/BiDi/vision fallback | ECR-003, ECR-004, ECR-006, ECR-009 | PLANNED | `specs/011-semantic-capability-router/` |
| ECR-012 | Skill IR | Versioned typed executable workflow IR with artifacts/dataflow, capabilities, side effects, pre/postconditions and verifiers | ECR-001–ECR-004 | PLANNED | `specs/012-skill-ir/` |
| ECR-013 | Skill Compiler | Human or verified-agent trajectory → candidate skill → sandbox validation → versioned skill | ECR-005, ECR-010, ECR-011, ECR-012 | PLANNED | `specs/013-skill-compiler/` |
| ECR-014 | Deterministic Replay | Low/no-model execution of compatible skills with exact receipts and compatibility checks | ECR-012, ECR-013 | PLANNED | `specs/014-skill-replay/` |
| ECR-015 | Divergence & Repair | Assumption tracking, localized repair, downstream invalidation, re-verification and version promotion | ECR-014 | PLANNED | `specs/015-divergence-repair/` |
| ECR-016 | Protocol Gateway | MCP/ACP/A2A/Agent Skills adapters with least-authority external access | ECR-001–ECR-004, ECR-009, ECR-010, ECR-012 | PLANNED | `specs/016-protocol-gateway/` |
| ECR-017 | Plugin & Sandbox Runtime | Signed/versioned extension model, Wasm/process isolation, capability manifests, resource limits | ECR-003, ECR-004, ECR-005, ECR-016 | PLANNED | `specs/017-plugin-sandbox/` |
| ECR-018 | Terminal Execution | Human/agent terminal sessions using the same capability/receipt/verifier/run model | ECR-002–ECR-005, ECR-017 | PLANNED | `specs/018-terminal/` |
| ECR-019 | Developer Workspace | Repo graph/context, current docs, tests/builds, browser QA, code review/release evidence | ECR-009, ECR-010, ECR-016, ECR-018 | PLANNED | `specs/019-developer-workspace/` |
| ECR-020 | Data & Analytics | Files/SQL/API analytics with lineage, reproducible transformations, evidence-backed conclusions | ECR-004, ECR-009, ECR-010, ECR-017 | PLANNED | `specs/020-data-analytics/` |
| ECR-021 | Local Model Gateway | Provider-neutral local inference adapter using Ecra search/context/memory/skills/actions/verifiers | ECR-009–ECR-016 | PLANNED | `specs/021-local-model-gateway/` |
| ECR-022 | Optional Sync & Multi-Device | User-controlled encrypted sync for portable workspaces/memory/skills/policies without becoming core-required cloud | ECR-002, ECR-003, ECR-010, ECR-012 | DEFERRED | `specs/022-sync-multidevice/` |
| ECR-023 | Extension Registry & Trust | Discovery, signing, provenance, review metadata and compatibility for plugins/skills/connectors/verifiers | ECR-012, ECR-016, ECR-017 | DEFERRED | `specs/023-extension-registry/` |
| ECR-024 | Release, Update & Supply Chain | Reproducible builds, SBOM, signing, update channels, security response, dependency/license automation | ECR-005, ECR-007 | PLANNED | `specs/024-release-supply-chain/` |
| ECR-025 | Privacy, Telemetry & Diagnostics | Local diagnostics, explicit telemetry contracts, redaction, crash reporting, retention/export controls | ECR-002, ECR-003, ECR-008 | PLANNED | `specs/025-privacy-diagnostics/` |
| ECR-026 | Accessibility, Internationalization & Human UX Quality | Accessibility, keyboard/screen-reader behavior, localization and non-AI browser-quality gates | ECR-007, ECR-008 | PLANNED | `specs/026-accessibility-i18n/` |
| ECR-027 | Search/Content Compliance & Source Policy | robots/access policy, source licensing/attribution, caching/retention, publisher controls, safe downloads | ECR-009 | PLANNED | `specs/027-search-source-policy/` |
| ECR-028 | Public Benchmark & Research Program | Reproducible benchmark adapters/reports for web, security, long-horizon, search trust, local-model augmentation | ECR-005 plus relevant feature slices | PLANNED | `specs/028-benchmark-program/` |
| ECR-029 | Migration, Import & Export | Import/export for browser state, workspaces, runs, skills, memories and policies; avoid artificial lock-in | ECR-008, ECR-010, ECR-012 | DEFERRED | `specs/029-portability/` |
| ECR-030 | Ecosystem Gateway | Stable developer SDK/local API and production-quality third-party agent/model infrastructure surface | ECR-016, ECR-017, ECR-023, ECR-024, ECR-025 | DEFERRED | `specs/030-ecosystem-gateway/` |

## Critical Path

```text
ECR-001 Trusted Domain Kernel
  ↓
ECR-002 Durable Run & Ledger
  ├───────────────┐
  ↓               ↓
ECR-003 Policy   ECR-004 Verification
  └──────┬────────┘
         ↓
ECR-005 Evaluation Harness
         ↓
ECR-006 Stock Firefox Prototype
         ↓
ECR-007 Browser Foundation
         ↓
ECR-008 Browser Wedge

Parallel after the trusted substrate:
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

Ecra must be able to prove authority, receipts, restart semantics, and verification against fixtures before privileged browser integration exists. Otherwise browser code becomes the accidental source of trust semantics.

### Browser foundation is separated from browser product

A daily browser creates an enduring upstream maintenance obligation. Stock Firefox/BiDi proves product mechanics first; only then does Ecra earn the cost of a distribution/fork.

### Search is separated from memory

Search evidence is ephemeral/retrieved truth. Memory is durable user-owned context. Combining them before their authority/provenance contracts exist risks retrieved content silently becoming durable authority.

### Skill IR is separated from compiler/replay/repair

The IR must be stable and independently testable before generated skills or repair logic depend on it.

### Gateway is separated from protocols

MCP/ACP/A2A are adapters. The public Ecra gateway can stabilize only after internal trust/context/execution contracts have demonstrated compatibility.

## Cross-Cutting Work That Must Never Be “Later”

The following are not end-of-project cleanup items. Every affected slice MUST add or update them as part of its Definition of Done:

- threat model;
- donor/license ledger;
- dependency and advisory review;
- migrations/backward compatibility for persisted formats;
- redaction/secret handling;
- accessibility for user-facing flows;
- observability without hidden telemetry;
- cancellation/timeouts/resource bounds;
- benchmark/acceptance fixtures;
- exact documentation for privileged behavior;
- export/portability implications.

## Status Semantics

- `PLANNED` — decomposed but no implementation is authorized by this roadmap alone.
- `SPEC_READY` — complete `spec.md` with no unresolved blocking clarification.
- `PLAN_READY` — research/data model/contracts/plan complete and constitution gates pass.
- `TASKS_READY` — traceable executable `tasks.md` exists.
- `IMPLEMENTING` — implementation branch/PR active.
- `BLOCKED` — dependency or evidence gate prevents safe continuation.
- `CLOSED_CANONICAL` — exact implemented state satisfies spec, plan, tasks, tests, analysis/convergence and documentation.
- `DEFERRED` — intentionally outside current critical path; may not be pulled forward without explicit dependency/strategy review.

## First Eligible Slice

`ECR-001 Trusted Domain Kernel` is the only implementation slice with no dependency. Its complete Spec Kit package is maintained in `specs/001-trusted-domain-kernel/`.
