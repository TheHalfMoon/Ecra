# Ecra Donor and License Ledger

**Status:** CANONICAL_PLANNING  
**Created:** 2026-08-27

This ledger separates **conceptual reference**, **dependency candidate**, and **source-reuse candidate**. Listing a project here never authorizes copying its source. Source reuse requires exact-file review, license compatibility, notice handling, and an implementation change that records what was copied/modified.

## Status Definitions

- `REFERENCE_ONLY` — study architecture/product/research; do not copy source under current plan.
- `DEPENDENCY_CANDIDATE` — may be used through normal package dependency after implementation-time license/security review.
- `SOURCE_REUSE_CANDIDATE` — selective source reuse may be considered only with exact provenance/notice handling.
- `FOUNDATION_CANDIDATE` — upstream project may become a maintained product foundation; requires dedicated upstream strategy.
- `BLOCKED_UNTIL_REVIEW` — licensing or maintenance conditions make source reuse unsuitable without explicit decision.

## Browser and Human Product

| Project | Role | Observed license | Status | Ecra use / constraint |
|---|---|---|---|---|
| Mozilla Firefox / Gecko | Production browser engine/foundation | MPL-2.0 and component-specific notices | FOUNDATION_CANDIDATE | Preferred browser foundation; ECR-007 owns exact upstream/distribution/notice analysis. |
| Zen Browser | Human productivity/browser UX donor | MPL-2.0 | SOURCE_REUSE_CANDIDATE | Prefer UX concepts and selective compatible patches over permanent deep downstream fork. Modified covered files remain subject to MPL obligations. |
| zen-browser/surfer | Firefox-fork build tooling reference/candidate | MPL-2.0 | DEPENDENCY/SOURCE_REUSE_CANDIDATE | Prototype/prerelease; useful for patch/build workflow ideas, not constitutional dependency. |
| Skyvern-AI/rustwright | Rust-native Chromium/CDP and agent-browser ergonomics | MIT | REFERENCE/DEPENDENCY_CANDIDATE | Strong donor for AX snapshots/CLI/CDP provider; not Ecra Firefox internal engine. |
| chromiumoxide | Rust CDP reference/provider | MIT/Apache-family per upstream review required before use | REFERENCE_ONLY pending exact license verification | Fallback/reference for Chromium provider. |
| Browser Use | Agent browser UX/behavior/evals | MIT | REFERENCE_ONLY | Do not architect Ecra as a Python port/fork. |
| Browserbase Stagehand | Hybrid deterministic/AI browser workflow ideas | MIT | REFERENCE_ONLY | Caching/self-healing/workflow ideas; Ecra differentiates through verified compiled Skill IR. |
| Tandem Browser | Human/agent shared-browser competitor/reference | verify exact upstream before source use | REFERENCE_ONLY | Product/UX benchmark only unless separately reviewed. |
| BrowserOS | OSS agent-browser competitor | AGPL-3.0 observed in discovery | BLOCKED_UNTIL_REVIEW | Competitor/reference; no source reuse under current distribution assumptions. |

## Rust Agent / Execution Architecture

| Project | Role | Observed license | Status | Ecra use / constraint |
|---|---|---|---|---|
| block/buzz | Rust event/identity/audit/protocol architecture | Apache-2.0 | REFERENCE/SOURCE_REUSE_CANDIDATE | Strong architectural donor; exact source reuse requires NOTICE/file provenance. |
| 0xPlaygrounds/rig | Serializable/Sans-I/O agent run concepts | MIT | REFERENCE/DEPENDENCY_CANDIDATE | ECR-002 may reuse patterns or dependency after exact fit review; Ecra owns canonical RunState. |
| aaif-goose/goose | Rust agent/provider/MCP/desktop ecosystem | Apache-2.0 | REFERENCE_ONLY | Study provider/extension/local-model patterns; avoid importing broad agent architecture into trusted core. |
| deepseek-ai/deepseek-harness | Replaceable capabilities/events/plugin tree | MIT observed in discovery | REFERENCE_ONLY | Conceptual architecture donor; TypeScript implementation is not Ecra core. |
| snarktank/ralph | Durable small-task/fresh-context iteration | MIT | REFERENCE_ONLY | Planning/execution loop concepts only. |

## State, Memory, Search, Graph

| Project | Role | Observed license | Status | Ecra use / constraint |
|---|---|---|---|---|
| tursodatabase/agentfs | Portable agent filesystem/state/audit model | MIT | REFERENCE/DEPENDENCY_CANDIDATE | Strong ECR-002/ECR-010 donor; Ecra does not inherit schema blindly. |
| Graphify-Labs/graphify | Provenance-aware graph extraction | Apache-2.0 | REFERENCE_ONLY | Key conceptual donor for EXTRACTED vs INFERRED-style provenance. |
| vitali87/code-graph-rag | Structural code graph/runtime relationships | MIT | REFERENCE_ONLY | ECR-019 structural context research. |
| upstash/context7 | Version-aware documentation context | MIT for repository components observed in discovery | REFERENCE/PROVIDER_CANDIDATE | External truth/provider ideas; hosted/private backend pieces are not assumed open source. |
| Tree-sitter | Incremental syntax parsing | MIT | DEPENDENCY_CANDIDATE | Likely ECR-019 dependency. |
| ast-grep | Structural search/rewrite | MIT observed in ecosystem | DEPENDENCY_CANDIDATE | Likely ECR-019 dependency after exact version/license review. |
| Tantivy | Embedded full-text search | MIT | DEPENDENCY_CANDIDATE | Preferred local text-index candidate for ECR-009. |
| petgraph | In-process graph structures | MIT/Apache-2.0 | DEPENDENCY_CANDIDATE | Candidate for local graph representation. |
| Qdrant | Vector database | Apache-2.0 | OPTIONAL_PROVIDER_CANDIDATE | Must not become local-core requirement. |

## Policy, Protocols, Sandboxing

| Project | Role | Observed license | Status | Ecra use / constraint |
|---|---|---|---|---|
| cedar-policy/cedar | Fine-grained authorization engine | Apache-2.0 | DEPENDENCY_CANDIDATE | ECR-003 adapter candidate; Ecra capability model remains independent. |
| Bytecode Alliance Wasmtime | WebAssembly/component runtime | Apache-2.0 WITH LLVM-exception / upstream notices | DEPENDENCY_CANDIDATE | ECR-017 plugin sandbox candidate; sandbox is not treated as infallible. |
| modelcontextprotocol/rust-sdk (`rmcp`) | MCP Rust SDK | mixed transition: Apache-2.0 for newly relicensed/new code; legacy MIT portions per upstream notice | DEPENDENCY_CANDIDATE | ECR-016 must pin reviewed version and preserve applicable notices. |
| agentclientprotocol/rust-sdk | ACP Rust SDK | verify exact version license before dependency | DEPENDENCY_CANDIDATE | ECR-016 only. |
| A2A Rust SDK | Agent-to-agent protocol | verify exact version license before dependency | DEPENDENCY_CANDIDATE | ECR-016 only. |
| Agent Skills specification | Portable skill knowledge format | verify spec/code licensing before bundled source use | REFERENCE/INTEROP | ECR-016 import/export semantics; Ecra Skill IR remains distinct. |

## Serialization / Trusted Core Candidates

| Project / standard | Role | License | Status | Ecra use / constraint |
|---|---|---|---|---|
| RFC 8785 JCS | Canonical JSON scheme | RFC text terms | STANDARD_REFERENCE | ECR-001 canonicalization contract. |
| `serde_jcs` | Rust RFC 8785 implementation | MIT OR Apache-2.0 | DEPENDENCY_CANDIDATE | ECR-001 candidate; exact release/security/maintenance review before lockfile. |
| Serde | Serialization | MIT OR Apache-2.0 | DEPENDENCY_CANDIDATE | ECR-001 likely dependency. |
| serde_json | JSON | MIT OR Apache-2.0 | DEPENDENCY_CANDIDATE | ECR-001 likely dependency. |
| thiserror | Typed error derivation | MIT OR Apache-2.0 | DEPENDENCY_CANDIDATE | ECR-001 likely dependency. |
| uuid | Strong UUID values | MIT OR Apache-2.0 | DEPENDENCY_CANDIDATE | ECR-001 candidate. |
| url | Standards-aware URL parsing | MIT OR Apache-2.0 | DEPENDENCY_CANDIDATE | ECR-001 `WebOrigin` parsing candidate. |

## Durable Execution References With Licensing Caution

| Project | Role | Observed license | Status | Ecra use / constraint |
|---|---|---|---|---|
| Restate core | Durable execution semantics | Business Source License 1.1 with delayed Apache change | REFERENCE_ONLY | Study exactly-once/durable execution semantics; do not copy core under current plan. |

## Research / Evaluation References (No Source Reuse Implied)

- BrowserGym / AgentLab — benchmark harness methodology.
- WebArena-Verified — deterministic browser verification concepts.
- Online-Mind2Web — live-web drift evaluation.
- OSWorld 2.0 / WeaveBench — long-horizon state/constraint evaluation.
- WASP / BrowseSafe / SOPBench / StepJack / AgentLAB — prompt-injection/origin/security research.
- Microsoft Universal Verifier / FaraGen / Echoverse — verifier/training-environment concepts.
- WebXSkill / ReUseIt / Hierarchical Memory Tree / Artic — reusable workflow/Skill IR research.
- WebMCP standards/work — semantic website capability surface and its trust implications.

These references may supply ideas, benchmark adapters, or standards compatibility. Paper text/code/datasets have separate licenses/terms and require exact review before redistribution.

## Source Reuse Rules

Before any copied/adapted code enters Ecra, the implementation change MUST record:

```text
upstream repository
exact commit/tag
exact file(s)/region(s)
upstream copyright holder/notice
license
whether file was modified
required NOTICE/source-offer obligations
Ecra destination path
reason source reuse is preferable to clean implementation/dependency
```

No commit may use “inspired by” to obscure copied source provenance.

## Dependency Review Rules

Before a candidate becomes a locked dependency:

1. verify exact release/tag and license files;
2. inspect security policy/advisories relevant to Ecra use;
3. record transitive-license compatibility where material;
4. minimize default features;
5. pin/lock reproducibly;
6. define update/advisory ownership;
7. confirm the dependency does not violate a constitutional boundary.

## Current Authorization

At planning time, this ledger authorizes **research and planning only**. ECR-001 tasks may add the small Rust dependency candidates listed in its `research.md` after implementation-time review. No browser/donor source code has been authorized for copying into Ecra yet.
