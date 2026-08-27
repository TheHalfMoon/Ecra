# Ecra Platform Decision Log

**Status:** CANONICAL_PLANNING  
**Date:** 2026-08-27

This is a compact architectural decision index. It is not a substitute for detailed research/specs. Once a decision is referenced by implementation, changing it requires explicit rationale, affected-slice review, and migration/compatibility analysis where relevant.

| ID | Decision | Status | Rationale | Revisit trigger |
|---|---|---|---|---|
| D-001 | Ecra is a gateway/trust-execution platform, not a bundle of unrelated AI products | ACCEPTED | Unifies Browser/Search/Workspace/Terminal/Data/Models around one flywheel | evidence that shared substrate materially harms product fit |
| D-002 | Use Spec Kit “spec of specs” decomposition with immutable `ECR-###` IDs | ACCEPTED | Platform is too large for one feature cycle; preserves traceability | only if repository governance replaces Spec Kit process |
| D-003 | Rust is the trusted-core language | ACCEPTED | Safety/performance/auditability and user preference | evidence that a specific trusted-core requirement cannot be met safely in Rust |
| D-004 | Trusted domain kernel is zero-I/O and model/browser/protocol independent | ACCEPTED | Prevents accidental architecture coupling | constitutional amendment only |
| D-005 | Firefox/Gecko is preferred daily-browser foundation | ACCEPTED_DIRECTION | Mature human browser, Containers, standards alignment, avoids Chromium monoculture | stock Firefox prototype proves critical blocker or upstream constraints make maintenance untenable |
| D-006 | Do not start with a permanent deep Zen downstream fork | ACCEPTED | Avoid Mozilla→Zen→Ecra maintenance chain | selective upstream collaboration or a demonstrably cheaper maintenance path appears |
| D-007 | Prove execution with stock Firefox/WebDriver BiDi before maintained browser distribution | ACCEPTED | De-risks trust/runtime semantics before expensive browser fork | only if a blocker requires privileged integration earlier, with explicit plan amendment |
| D-008 | Rustwright is a strong Chromium/headless/ergonomics donor, not Firefox internal controller | ACCEPTED | Rustwright is Chromium-only; Ecra browser is Firefox-directed | Rustwright gains relevant Firefox/BiDi capability or browser foundation changes |
| D-009 | Human, Agent and System are explicit actors | ACCEPTED | Attribution/control/approval depend on actor identity | constitutional amendment only |
| D-010 | External content is context/data, never self-granting authority | ACCEPTED | Prompt injection cannot be solved by prompt wording alone | constitutional amendment only |
| D-011 | CapabilityRequest and CapabilityGrant are different domain types | ACCEPTED | Prevents request/authority type confusion | constitutional amendment only |
| D-012 | Model proposes; policy authorizes; runtime executes; verifier confirms | ACCEPTED | Separation of concerns/security | constitutional amendment only |
| D-013 | ActionReceipt and VerificationReceipt are distinct | ACCEPTED | Executor self-report is not independent success proof | constitutional amendment only |
| D-014 | UNKNOWN is a first-class external action outcome | ACCEPTED | Crash/network ambiguity must not create false certainty | constitutional amendment only |
| D-015 | Side-effect, idempotency and retry semantics are explicit before execution | ACCEPTED | Required for safe resume/retry | constitutional amendment only |
| D-016 | Search uses one evidence/provenance contract across web/local/workspace providers | ACCEPTED | Avoids source quality disappearing in synthesis | ECR-009 research may refine ranking/indexing, not remove evidence contract |
| D-017 | Memory is source-aware context and cannot self-authorize | ACCEPTED | Prevents memory poisoning/permission drift | constitutional amendment only |
| D-018 | Ecra Skill is typed executable IR, not recorded clicks or Agent Skill text | ACCEPTED | Enables deterministic replay, verification, repair | ECR-012 research may refine syntax, not semantic requirements |
| D-019 | Human and verified-agent demonstrations compile to the same Skill IR | ACCEPTED_DIRECTION | Makes human teaching and agent learning converge | compiler experiments show irreducible separate semantics |
| D-020 | Replay should use zero model calls when compatibility/preconditions hold | ACCEPTED_GOAL | Economic/reliability moat | empirical evidence shows unavoidable reasoning for a skill class; then classify that class explicitly |
| D-021 | Repair is localized and versioned rather than full re-exploration by default | ACCEPTED_GOAL | Efficiency/auditability | repair benchmarks prove whole-task regeneration superior for a defined class |
| D-022 | MCP/ACP/A2A/Agent Skills/WebMCP are adapters/standards, not internal trusted model | ACCEPTED | Standards evolve; core semantics must remain coherent | constitutional amendment only |
| D-023 | Plugins use capability-isolated sandbox/process boundaries | ACCEPTED_DIRECTION | Third-party code cannot inherit ambient authority | exact sandbox tier varies by platform/risk in ECR-017 |
| D-024 | Local models are first-class but no custom model training before data/eval moat exists | ACCEPTED | Runtime/context/skills provide nearer-term leverage | verified corpus/evals demonstrate a specialized model is the next bottleneck |
| D-025 | Local-first useful core; cloud is additive, not hidden prerequisite | ACCEPTED | Privacy/resilience/model independence | constitutional amendment only |
| D-026 | No hidden telemetry | ACCEPTED | Trust/privacy | constitutional amendment only |
| D-027 | User-owned runs/memory/skills should be exportable | ACCEPTED | Avoid lock-in; ecosystem trust | format details owned by ECR-029 |
| D-028 | Public superiority claims require reproducible benchmark evidence | ACCEPTED | Avoid marketing-driven architecture | constitutional amendment only |
| D-029 | Start ECR-001 with one production crate (`ecra-core`) | ACCEPTED_FOR_SLICE | Avoid speculative crate explosion | ECR-001 plan amendment with concrete current need |
| D-030 | ECR-001 normative contract uses JSON + RFC 8785 JCS canonicalization | ACCEPTED_FOR_V1 | Inspectable/cross-language/deterministic | compatibility/versioned migration evidence justifies new representation |
| D-031 | ECR-001 pins Rust current stable 1.98.x / Edition 2024 | ACCEPTED_FOR_SLICE | Greenfield current stable baseline | toolchain/security/upstream compatibility evidence |
| D-032 | Broad public web crawl/index is not required for initial Search wedge | ACCEPTED | First win can combine web providers + local/workspace evidence without Google-scale crawler | product/economics evidence justifies owning crawl/index infrastructure |
| D-033 | Mobile and team/enterprise multi-principal governance are deferred | ACCEPTED | Desktop single-user trust model first | browser wedge proves demand and required authority model is stable |
| D-034 | Brand `Ecra` is provisional until clearance | ACCEPTED | Existing name collisions/trademark risk observed | legal/brand clearance completes |

## Decision Change Process

A decision change proposal MUST include:

1. old decision and proposed replacement;
2. new evidence or constraint;
3. impacted ECR slices and dependency graph;
4. constitution analysis;
5. persistence/API/skill/browser migration impact;
6. donor/license impact;
7. benchmark/risk-register changes;
8. rollback plan if implementation already exists.

Avoid reopening accepted decisions because a new donor/framework appears. New technology is evidence only when it materially changes Ecra's constraints or measured outcomes.
