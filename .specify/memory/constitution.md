# Ecra Spec Kit Constitution

<!--
Sync Impact Report
- Version: 1.0.0
- Ratified: 2026-08-27
- Supersedes: ad-hoc planning conventions and the root CONSTITUTION.md as the machine-governed Spec Kit source.
- Governs: every spec.md, plan.md, tasks.md, implementation PR, benchmark claim, donor adoption, and release decision.
- Derived from: README.md, VISION.md, CONSTITUTION.md, ROADMAP.md.
-->

## Principle I — One Trusted Domain Model

All Ecra surfaces MUST converge on one set of trusted concepts: Actor, Identity, Origin, Authority, Capability, Observation, Fact, Artifact, ActionIntent, ActionReceipt, VerificationReceipt, MemoryRecord, Skill, and RunState.

The trusted domain layer MUST be model-provider-neutral and I/O-free. Browser, Search, Workspace, Terminal, Developer, Data, Memory, Plugins, Gateway, and model integrations MUST NOT invent parallel authority, provenance, execution, or completion semantics.

A spec that introduces a second representation for a constitutional concept MUST be rejected or explicitly migrate the canonical representation.

## Principle II — Authority Is Explicit, Scoped, and Fail-Closed

Humans, agents, and system processes are explicit actors. Every consequential mutation MUST be attributable to an actor.

Agents MUST NOT receive ambient authority. Capability evaluation MUST be fail-closed and capable of constraining, where relevant: workspace, browser space, container, origin, tab/session, resource, action class, purpose, task, duration, and delegation chain.

External content—including web pages, documents, email, repository content, retrieved memory, tool output, and model output—is context, not authority. External content MUST NOT silently grant capabilities, expand scope, approve actions, or overwrite higher-authority policy.

Human approval MUST bind to a specific proposed action or narrowly defined action set. Approval MUST NOT be treated as a reusable blanket permission unless the user explicitly created such a policy.

## Principle III — Model Proposes; Policy Authorizes; Runtime Executes; Verifier Confirms

Planning, authorization, execution, and verification are separate responsibilities.

A model MUST NOT be able to bypass policy by selecting a lower-level tool, browser primitive, plugin, protocol adapter, or alternate execution path.

A model's statement that a task succeeded MUST NOT be accepted as completion evidence.

Verification SHOULD prefer, in order of availability and relevance: structured external state, API/tool results, network receipts, durable artifacts, DOM/accessibility state, deterministic computation, and only then independent model judgment.

UNKNOWN external outcomes MUST remain UNKNOWN. Non-idempotent or consequential actions MUST NOT be blindly retried after an ambiguous outcome.

## Principle IV — Evidence, Provenance, and Memory Remain Distinct from Generated Text

Ecra MUST preserve material provenance for important claims and durable context. The system MUST be able to distinguish at least: user-provided, observed, retrieved, tool-provided, model-inferred, verified, contradicted, and stale information where the distinction affects trust or behavior.

Search and research features MUST optimize for source quality, freshness, contradiction visibility, and inspectable evidence—not fluent synthesis alone.

Memory is context, not permission. Untrusted content or model-generated memory MUST NOT self-authorize. Durable memory MUST be scoped, inspectable, deletable, and exportable.

Raw secrets SHOULD NOT enter model prompts, generic tool output, durable memory, or logs when mediated use through scoped handles is possible.

## Principle V — Durable Runs and Receipts Are the Source of Execution Truth

Long-running or consequential work MUST NOT depend on a chat transcript as its source of truth.

Ecra runs MUST be serializable, inspectable, and resumable across model, browser, or process failure where the underlying action semantics permit resumption.

Consequential actions MUST emit durable receipts sufficient to determine what was intended, what was attempted, what external evidence exists, and whether the outcome is CONFIRMED_SUCCESS, CONFIRMED_FAILURE, or UNKNOWN.

Human intervention, takeover, hand-back, editing, approval, denial, pause, cancellation, and repair MUST be first-class run events.

## Principle VI — Verified Work Compiles Into Reusable Execution

Repeated successful work SHOULD become cheaper and more reliable over time.

An Ecra Skill is not a recorded click sequence. A canonical skill representation MUST support: intent, typed inputs/outputs, explicit artifact reads/writes, data dependencies, stages, preconditions, postconditions, origin/capability requirements, side-effect semantics, approval points, verifiers, assumptions, and repair boundaries.

When reality diverges from a compiled skill, Ecra SHOULD localize the failing assumption or stage, repair that region, re-verify affected downstream assumptions, and version the skill rather than discarding the entire workflow by default.

## Principle VII — Human Product Quality, Local-First Utility, and Model Independence

Ecra MUST remain an excellent human product. AI features MUST NOT materially degrade ordinary browsing, accessibility, privacy, predictability, startup time, responsiveness, or reliability without an explicit measured tradeoff accepted in the relevant plan.

Core browsing, local workspace, local search/memory, policy, skill inspection, and run inspection SHOULD remain useful without a cloud account.

No cloud or local model vendor owns Ecra's architecture. Models are replaceable adapters. MCP, ACP, A2A, Agent Skills, WebMCP, WebDriver BiDi, and future standards are boundary protocols, not the trusted internal domain model.

User-owned data, memories, skills, runs, and workspace artifacts SHOULD have documented export paths. Ecra MUST earn default status through utility rather than artificial lock-in.

## Principle VIII — Security, Upstream Health, and Measured Claims Beat Feature Velocity

Browser-engine security updates and critical dependency advisories are release-blocking priorities. Ecra SHOULD minimize long-lived browser-engine patches and MUST track the provenance and maintenance cost of every privileged browser modification.

Third-party code MUST NOT enter Ecra without a donor/license record covering source, license, notices, modification requirements, compatibility, and whether the use is source reuse or conceptual inspiration.

Untrusted plugins MUST receive explicit capabilities and SHOULD execute in a sandboxed component/process boundary appropriate to their risk.

Claims such as “best”, “most secure”, “fastest”, “private”, or “most reliable” require reproducible evidence. Plans MUST identify the benchmark or acceptance evidence that would justify the claim.

## Principle IX — Wedge Before Empire; One Flywheel

Ecra may pursue browser, search, workspace, terminal, developer, data, memory, skills, plugins, and model-gateway surfaces only when they strengthen the same trust/context/execution flywheel.

A major new surface MUST answer:

1. Which existing trusted-domain concepts does it reuse?
2. Which shared context, memory, search, skill, or verification capability does it strengthen?
3. Why is a protocol adapter or plugin insufficient?
4. What independently testable user outcome justifies adding it to the core product?
5. What complexity and maintenance burden does it add?

Implementation MUST proceed through bounded Spec Kit slices. A platform-scale roadmap MUST use immutable sub-spec IDs and dependencies; each buildable slice MUST independently complete specify → plan → tasks → implementation → analyze/converge before the next dependent slice becomes eligible.

## Principle X — The Trusted Core Stays Small Enough to Audit

Abstraction is justified by current safety, correctness, interoperability, measured performance, or demonstrated product need—not speculative flexibility.

Core crates SHOULD minimize dependencies and side effects. I/O belongs behind explicit boundaries. Unsafe code, privileged browser hooks, secret handling, policy bypass paths, deserialization of untrusted data, and sandbox escape surfaces require explicit threat-model coverage and dedicated tests.

A plan that adds complexity violating this principle MUST document the simpler alternative and why it is insufficient in the plan's Complexity Tracking section.

## Mandatory Cross-Spec Gates

Every implementation plan MUST explicitly pass or fail the following gates before implementation:

- **G1 Domain coherence:** no competing canonical representation of constitutional concepts.
- **G2 Authority:** deny-by-default behavior and no ambient agent authority.
- **G3 Provenance:** important durable facts/claims preserve source and trust class.
- **G4 Side effects:** unknown-outcome and retry semantics are explicit.
- **G5 Verification:** completion criteria do not depend on actor self-report.
- **G6 Durability:** restart/resume behavior is defined for stateful work.
- **G7 Privacy/secrets:** secret and telemetry flows are documented.
- **G8 Local-first:** any cloud dependency is justified and has a degraded/offline behavior when constitutionally required.
- **G9 Interoperability:** external protocols remain adapters.
- **G10 Donor/license:** source reuse is traceable and license-compatible.
- **G11 Upstream/browser maintenance:** privileged patches and update strategy are explicit when applicable.
- **G12 Benchmarks:** acceptance metrics are reproducible and do not rely on marketing claims.

A failed constitutional gate blocks implementation unless the constitution itself is amended through governance.

## Governance

### Authority

This file is the canonical Spec Kit governance source for Ecra. Root-level vision documents remain product-readable references; if they conflict with this file on a MUST-level rule, this file governs until the conflict is deliberately resolved.

### Spec Kit Lifecycle

For platform-scale work, Ecra uses the Spec Kit “spec of specs” pattern. Each immutable roadmap slice is independently specified and implemented. Buildable work MUST have, at minimum:

```text
spec.md
research.md
plan.md
data-model.md (when data/state exists)
contracts/ (when interfaces or schemas exist)
quickstart.md or verification guide
tasks.md
```

Tasks MUST be traceable to user stories, functional requirements, contracts, or acceptance criteria and MUST contain exact target paths once implementation structure is known.

### Definition of Done

A buildable slice is not CLOSED_CANONICAL until:

1. all required tasks are complete;
2. relevant unit, contract, integration, security, migration, and benchmark gates pass;
3. no unresolved MUST-level constitution violation remains;
4. acceptance criteria are demonstrated on the exact implemented state;
5. docs/contracts reflect implementation truth;
6. donor/license records are current;
7. `/speckit.analyze`-equivalent traceability review finds no critical spec/plan/task drift;
8. convergence work, if any, is appended and completed;
9. the final state is reproducible from repository instructions.

### Amendments

Constitution changes require rationale, impact analysis, and semantic versioning:

- **MAJOR** — removal/redefinition of a binding principle or incompatible governance change.
- **MINOR** — new binding principle/gate or materially stronger requirements.
- **PATCH** — clarification with no semantic change.

Every amendment MUST update the Sync Impact Report.

**Version:** 1.0.0  
**Ratified:** 2026-08-27  
**Last Amended:** 2026-08-27
