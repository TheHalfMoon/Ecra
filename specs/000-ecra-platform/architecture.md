# Ecra Platform Architecture

**Status:** CANONICAL_PLANNING  
**Date:** 2026-08-27  
**Governed by:** `.specify/memory/constitution.md`

This document defines stable architectural boundaries for the platform roadmap. It does not freeze internal implementation details before their owning Spec Kit slices are planned.

## 1. Architectural Thesis

Ecra is one trusted substrate exposed through multiple human and machine surfaces.

```text
Humans / Agents / Local Models / Cloud Models / IDEs / Apps
                           │
                           ▼
                Ecra Surfaces & Adapters
                           │
                           ▼
                 Trusted Rust Substrate
                           │
             ┌─────────────┼─────────────┐
             ▼             ▼             ▼
          Browser        Local OS       External Services
          / Web          / Data         / Tools / Models
```

The trusted substrate owns semantics for authority, provenance, durable execution, receipts, verification, memory trust, and reusable skills. External surfaces/adapters do not redefine them.

## 2. Architectural Layers

### Layer A — Trusted Domain Kernel

Owned initially by ECR-001.

```text
Actor
Identity references
Origin
Resource/Scope
Capability request/grant representation
Observation
Fact / Provenance
Artifact references
ActionIntent
ActionReceipt
VerificationReceipt
Schema/version contracts
```

Properties:
- Rust;
- zero I/O;
- no browser/model/database/protocol dependency;
- strict validation;
- small enough to audit deeply.

### Layer B — Durable Execution and Trust Services

Owned by ECR-002–ECR-005.

```text
Run state machine
Event ledger
Policy authorization
Approval binding
Secret mediation contract
Verification orchestration
Retry/reconciliation semantics
Evaluation/threat harness
```

Properties:
- uses Layer A types;
- I/O behind explicit interfaces;
- policy is independent of model/planner/executor;
- executor receipts do not self-verify.

### Layer C — Capability Providers

Concrete ways Ecra can observe or act:

```text
Browser / WebDriver BiDi / privileged bridge
WebMCP / site-native semantic tools
Search providers
Terminal/process provider
Filesystem/data providers
MCP/ACP/A2A adapters
Plugin runtime
Model providers
Local model providers
```

Providers translate their native APIs into Ecra domain operations. They do not grant themselves authority.

### Layer D — Context, Search, Workspace, Memory

Owned by ECR-009/ECR-010 and extended by developer/data slices.

```text
Evidence fabric
Indexes/retrievers
Workspace scope
Memory lifecycle
Fact/provenance graph
Repository/code context
Data lineage
```

Retrieval is capability/scope-aware before context reaches a model. Retrieved material remains context/evidence, never permission.

### Layer E — Skill System

Owned by ECR-012–ECR-015.

```text
Skill IR
Compiler
Compatibility/precondition evaluation
Deterministic replay
Divergence detection
Localized repair
Version promotion/rollback
```

Skills invoke Layer C capabilities through Layer B policy/verification; they do not bypass the trusted substrate.

### Layer F — Human Product Surfaces

```text
Ecra Browser
Ecra Search
Workspace UI
Terminal / Developer UI
Data UI
Inspector
Approval/takeover surfaces
```

Human UI reads trustworthy execution state from the same run/receipt/policy objects used by machine interfaces.

### Layer G — External Gateway

```text
MCP
ACP
A2A
Agent Skills import/export
Rust SDK
Local API
future stable public APIs
```

Gateway callers receive explicit capabilities and scoped views. They never connect directly to the privileged browser bridge or raw user state by default.

## 3. Dependency Direction

The intended dependency direction is one-way:

```text
Human UI / External Adapters / Providers
              ↓
Application services / Skill runtime
              ↓
Run / Policy / Verification / Ledger
              ↓
Trusted Domain Kernel
```

Forbidden direction examples:

- `ecra-core` importing Firefox/BiDi types;
- policy logic importing a specific LLM SDK;
- search result types becoming the canonical Fact type;
- MCP tool schema becoming CapabilityGrant;
- browser UI state becoming durable run truth;
- plugin code calling privileged internals outside capability provider interfaces.

## 4. Process / Privilege Boundaries

Final process topology is owned by later slices, but planning assumes at least these logical trust zones:

### Browser Zone

Firefox/Gecko plus Ecra browser UI/privileged integration.

Privileged browser bridge should expose a narrow authenticated local protocol to the Rust substrate. A browser compromise or malicious page must not automatically obtain arbitrary Rust-core/OS/plugin authority.

### Trusted Core Zone

Rust services handling policy, run state, ledger, verification orchestration, secrets mediation and capability routing.

This zone is smaller and more privileged than generic model/plugin processes.

### Model Zone

Cloud/local model adapters are untrusted decision producers. Model output is proposal/context, not authorization.

### Plugin Zone

Third-party extensions run in capability-restricted Wasm/process sandboxes according to ECR-017. Plugin compromise must not equal core compromise by design.

### External Protocol Zone

MCP/ACP/A2A clients/servers are adapters. Remote/local callers authenticate/authorize through gateway policy rather than inheriting process trust.

### Storage Zone

Run/memory/workspace stores persist versioned records. Storage bytes are not trusted merely because they are local; migrations/integrity validation apply on load.

## 5. Browser Architecture Direction

Preferred sequence:

```text
Stock Firefox + WebDriver BiDi prototype
        ↓
Prove Actor → Capability → Action → Receipt → Verification
        ↓
Define privileged browser bridge
        ↓
Reproducible Firefox-derived Ecra distribution
        ↓
Human/Agent/Shared tabs + Spaces/Containers
```

Do not start by deep-forking Firefox/Zen.

### Firefox/Zen relationship

- Firefox/Gecko: engine/foundation candidate.
- Zen: UX/source donor candidate under MPL-2.0 where exact reuse is justified.
- Surfer: build/patch tooling candidate, not architectural core.
- Rustwright: Chromium/headless/provider/agent-ergonomics donor, not Firefox internal controller.

## 6. Capability Resolution

Ecra should choose the safest/highest-semantic execution mechanism compatible with authority and state:

```text
Intent
  ↓
Capability Router
  ├─ native trusted API/provider
  ├─ WebMCP/site semantic capability
  ├─ compiled Ecra Skill
  ├─ accessibility/DOM semantic action
  ├─ WebDriver BiDi/browser primitive
  ├─ vision/coordinate action
  └─ desktop/general fallback (future)
```

Selection considers:
- required authority;
- origin/principal trust;
- determinism;
- semantic fidelity;
- cost/latency;
- available verification;
- compatibility with current state.

A lower-level route never bypasses a denied higher-level action.

## 7. Search / Knowledge Architecture

One evidence fabric, multiple providers/indexes:

```text
Provider result
  ↓
Source identity + observation time
  ↓
Evidence/Fact mapping
  ↓
Quality/freshness/contradiction analysis
  ↓
Scoped retrieval/context assembly
  ↓
Synthesis
  ↓
Claim-to-evidence mapping
```

Possible provider/index implementations are replaceable. Vector search is optional, not the memory/search architecture itself.

## 8. Memory Architecture

```text
Observation/Retrieval/Model output
      ↓
Candidate durable memory
      ↓
provenance + scope + trust policy
      ↓
accepted MemoryRecord
      ↓
aging / contradiction / verification / deletion
```

Memory lookup is scope/capability aware. Memory never self-authorizes.

## 9. Skill Architecture

Ecra Skill IR is a dataflow/behavior program, not a macro recording.

```text
Intent
Stages
Typed inputs/outputs
Artifact reads/writes
Dependencies
Preconditions/postconditions
Capabilities/origins
Side-effect classes
Approval points
Verifiers
Assumptions
Repair boundaries
```

Human demonstrations and agent trajectories normalize into the same IR only after verification.

## 10. Data and Developer Architecture

Terminal, Developer and Data surfaces are capability providers/consumers over the same core:

```text
Browser action → ActionReceipt → Verification
Shell action   → ActionReceipt → Verification
SQL query      → ActionReceipt → Verification
Git push       → ActionReceipt → Verification
```

They do not get bespoke “agent mode” trust semantics.

## 11. Persistence Strategy Direction

- local-first;
- versioned schemas;
- SQLite-like portable run/workspace state is preferred for early slices when it fits;
- large blobs may be content-addressed outside the primary DB with explicit ArtifactRef lineage;
- append-only execution events for run truth;
- migrations required once persisted formats ship;
- exportability is strategic, not optional cleanup.

Exact database/schema belongs to ECR-002/ECR-010.

## 12. Observability Direction

Internal structured diagnostics should use Ecra-owned event/error categories. Export to tracing/OpenTelemetry/etc. belongs outside core domain types.

No hidden telemetry. Local inspector/doctor capability precedes optional remote telemetry.

## 13. Architecture Fitness Functions

Plans/CI should increasingly enforce:

- `ecra-core` remains zero-I/O;
- no privileged provider bypasses policy routing;
- all consequential executor results create receipts;
- all completion claims resolve through verifier state;
- no external protocol SDK leaks into domain types;
- persisted schema changes have migrations/fixtures;
- browser privileged patch inventory is bounded and current;
- user-facing flows remain accessible and functional with model features disabled when constitutionally required.

## 14. Architecture Change Rule

A change to dependency direction, trust zones, canonical domain ownership, browser foundation, or the distinction between receipts and verification is a platform architecture change. It requires:

1. explicit decision record;
2. affected spec-of-specs dependency review;
3. constitution check;
4. gap/risk register update;
5. migration/compatibility analysis if any persisted/public contract exists.
