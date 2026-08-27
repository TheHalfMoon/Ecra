# Ecra Spec Kit Constitution

<!--
Sync Impact Report
- Version: 1.1.0
- Ratified: 2026-08-27
- Last amendment: 2026-08-27 pre-implementation architecture review.
- Amendment class: MINOR — adds binding identity/principal, information-flow/egress, and bounded-execution requirements.
- Added constitutional concepts: Principal/IdentityAssertion references, information classification/disclosure constraints, exact action binding, execution attempts, resource budgets.
- Added mandatory gates: G13 Information flow / egress, G14 Identity / principal binding, G15 Bounded execution.
- Strengthened: receipt terminology, verification single-source-of-truth, approval/action binding, secret/data disclosure semantics.
- Supersedes: v1.0.0 and ad-hoc planning conventions; root CONSTITUTION.md remains a product-readable reference.
- Governs: every spec.md, plan.md, tasks.md, implementation PR, benchmark claim, donor adoption, and release decision.
- Derived from: README.md, VISION.md, CONSTITUTION.md, ROADMAP.md, and `specs/000-ecra-platform/pre-implementation-review-2026-08-27.md`.
-->

## Principle I — One Trusted Domain Model

All Ecra surfaces MUST converge on one set of trusted concepts. At minimum these include: Actor, Principal/Identity reference, Origin, Resource, Scope, Information Classification, Authority, Capability, Observation, Fact, Artifact, ActionIntent, ActionAttempt, ActionReceipt, VerificationReceipt, MemoryRecord, Skill, and RunState.

The trusted domain layer MUST be model-provider-neutral and I/O-free. Browser, Search, Workspace, Terminal, Developer, Data, Memory, Plugins, Gateway, and model integrations MUST NOT invent parallel authority, provenance, information-flow, execution, or completion semantics.

A spec that introduces a second representation for a constitutional concept MUST be rejected or explicitly migrate the canonical representation.

An `Actor` describes who/what participates in a run. It MUST NOT be assumed to be an authenticated security principal merely because it has an identifier. Authentication, on-behalf-of relationships, identity assertions, and trust roots MUST be explicit at the appropriate boundary.

## Principle II — Authority Is Explicit, Scoped, and Fail-Closed

Humans, agents, and system processes are explicit actors. Every consequential mutation MUST be attributable to an actor and, where authorization depends on authenticated identity, to a validated principal/identity assertion.

Agents MUST NOT receive ambient authority. Capability evaluation MUST be fail-closed and capable of constraining, where relevant: workspace, browser space, container, origin, tab/session, resource, action class, purpose, task, duration, delegation chain, and data-disclosure destination.

Scope semantics MUST be explicit. Missing, empty, unknown, not-applicable, and unrestricted/ANY MUST NOT be silently conflated. No omitted field may widen authority by serializer or caller convention.

External content—including web pages, documents, email, repository content, retrieved memory, tool output, and model output—is context, not authority. External content MUST NOT silently grant capabilities, expand scope, approve actions, overwrite higher-authority policy, or declassify information.

Human approval MUST bind to an immutable representation/digest of the proposed action or an explicitly defined narrow action set, including security-relevant parameters and scope. Approval MUST NOT be treated as a reusable blanket permission unless the user deliberately created such a policy.

## Principle III — Model Proposes; Policy Authorizes; Runtime Executes; Verifier Confirms

Planning, capability resolution, authorization, execution, and verification are separate responsibilities.

A model MUST NOT be able to bypass policy by selecting a lower-level tool, browser primitive, plugin, protocol adapter, or alternate execution path. Capability routing MUST resolve an intent into a concrete canonical action before the action is authorized for execution.

Authorization decisions for privileged/consequential work SHOULD bind the concrete action digest, principal/actor context, relevant grants/policy version, approvals, evaluation context, and expiry/revocation semantics. Executors MUST NOT infer authority from a model request alone.

A model's statement that a task succeeded MUST NOT be accepted as completion evidence.

Verification SHOULD prefer, in order of availability and relevance: structured external state, API/tool results, network receipts, durable artifacts, DOM/accessibility state, deterministic computation, and only then independent model judgment.

Verification records are the authoritative representation of verification outcomes. A Fact, receipt, UI flag, or model output MUST NOT create a second independent source of “verified” truth.

UNKNOWN external outcomes MUST remain UNKNOWN. Non-idempotent or consequential actions MUST NOT be blindly retried after an ambiguous outcome.

## Principle IV — Evidence, Provenance, Information Flow, and Memory Remain Explicit

Ecra MUST preserve material provenance for important claims and durable context. The system MUST be able to distinguish at least user-provided, observed, retrieved, tool-provided, model-inferred, system-derived, contradicted/disputed, and stale/unknown-freshness information where the distinction affects trust or behavior.

Verification and original provenance are orthogonal. Verifying a model-inferred claim does not rewrite its origin.

Information access and information disclosure are different authorities. Permission to read data from source A MUST NOT imply permission to place that data—or a materially derived representation of it—into destination B, a remote model, search provider, plugin, tool, memory scope, log, or external origin.

Sensitive/secret/private or otherwise policy-constrained information MUST carry enough classification/lineage context for later policy to make source-to-sink decisions. Derived information MUST conservatively retain relevant restrictions unless an explicit authorized declassification/transformation rule applies.

Search and research features MUST optimize for source quality, freshness, source independence, contradiction visibility, and inspectable evidence—not fluent synthesis alone.

Memory is context, not permission. Untrusted content or model-generated memory MUST NOT self-authorize. Durable memory MUST be scoped, inspectable, deletable, and exportable; deletion semantics MUST account for derived indexes/caches/projections where those can remain retrievable.

Raw secrets SHOULD NOT enter model prompts, generic tool output, durable memory, or logs when mediated use through scoped handles is possible.

## Principle V — Durable Runs, Attempts, and Receipts Are the Source of Execution Truth

Long-running or consequential work MUST NOT depend on a chat transcript as its source of truth.

Ecra runs MUST be serializable, inspectable, and resumable across model, browser, or process failure where the underlying action semantics permit resumption.

Action intent and execution attempt are distinct. A single intent may have zero or more attempts; each attempt that can create an external effect MUST be uniquely identifiable for retry, reconciliation, audit, and duplicate-effect analysis.

Consequential action receipts MUST bind the immutable action reference/digest and exact attempt identity. A receipt describes executor-observed execution state; it MUST NOT use terminology that implies independent verification.

Executor outcomes MUST distinguish at least observed/reported success, observed/reported failure, and UNKNOWN. Only a VerificationReceipt may establish `VERIFIED`, `REJECTED`, or equivalent independent verification state.

Human intervention, takeover, hand-back, editing, approval, denial, pause, cancellation, repair, reconciliation, and authorization revocation MUST be first-class run events when relevant.

## Principle VI — Verified Work Compiles Into Reusable Execution

Repeated successful work SHOULD become cheaper and more reliable over time.

An Ecra Skill is not a recorded click sequence. A canonical skill representation MUST support intent, typed inputs/outputs, explicit artifact reads/writes, data dependencies, stages, preconditions, postconditions, origin/capability requirements, side-effect semantics, approval points, verifiers, assumptions, and repair boundaries.

A compiled/imported Skill MUST contain requirements for authority, not captured live authority. It MUST NOT persist reusable grants, approval tokens, raw secrets, or ambient authenticated session authority from the demonstration that produced it.

When reality diverges from a compiled skill, Ecra SHOULD localize the failing assumption or stage, repair that region, re-authorize affected actions, re-verify affected downstream assumptions, and version the skill rather than discarding the entire workflow by default.

## Principle VII — Human Product Quality, Local-First Utility, and Model Independence

Ecra MUST remain an excellent human product. AI features MUST NOT materially degrade ordinary browsing, accessibility, privacy, predictability, startup time, responsiveness, or reliability without an explicit measured tradeoff accepted in the relevant plan.

Core browsing, local workspace, local search/memory, policy, skill inspection, and run inspection SHOULD remain useful without a cloud account.

No cloud or local model vendor owns Ecra's architecture. Models are replaceable adapters. MCP, ACP, A2A, Agent Skills, WebMCP, WebDriver BiDi, and future standards are boundary protocols, not the trusted internal domain model.

“Local” MUST NOT be treated as synonymous with “trusted”. Local models, model artifacts, plugins, repositories, browser extensions, files, and local processes may remain untrusted and capability-restricted.

User-owned data, memories, skills, runs, and workspace artifacts SHOULD have documented export paths. Ecra MUST earn default status through utility rather than artificial lock-in.

## Principle VIII — Security, Bounded Execution, Upstream Health, and Measured Claims Beat Feature Velocity

Browser-engine security updates and critical dependency advisories are release-blocking priorities. Ecra SHOULD minimize long-lived browser-engine patches and MUST track the provenance and maintenance cost of every privileged browser modification.

Third-party code MUST NOT enter Ecra without a donor/license record covering source, license, notices, modification requirements, compatibility, and whether the use is source reuse or conceptual inspiration.

Untrusted plugins, repositories/build systems, parsers, and model artifacts MUST receive execution isolation and resource limits appropriate to their risk. A sandbox is defense-in-depth, not proof of safety.

Agent/model/tool execution MUST have explicit bounded-consumption semantics where applicable: wall time, steps/tool calls, model calls/tokens/cost, process lifetime, output, network, storage, or other relevant resources. Exhausting a budget MUST terminate or suspend safely without granting extra authority or converting an unknown side effect into a retry.

Claims such as “best”, “most secure”, “fastest”, “private”, “tamper-proof”, “tamper-evident”, or “most reliable” require reproducible evidence scoped to the actual guarantee. A plain integrity/hash chain MUST NOT be described as resistant to an adversary who can rewrite the entire store unless a protected trust anchor makes that claim true.

## Principle IX — Wedge Before Empire; One Flywheel

Ecra may pursue browser, search, workspace, terminal, developer, data, memory, skills, plugins, and model-gateway surfaces only when they strengthen the same trust/context/execution flywheel.

A major new surface MUST answer:

1. Which existing trusted-domain concepts does it reuse?
2. Which shared context, memory, search, skill, or verification capability does it strengthen?
3. Why is a protocol adapter or plugin insufficient?
4. What independently testable user outcome justifies adding it to the core product?
5. What complexity and maintenance burden does it add?

Implementation MUST proceed through bounded Spec Kit slices. A platform-scale roadmap MUST use immutable sub-spec IDs and dependencies; each buildable slice MUST independently complete specify → plan → tasks → analyze → implementation → verify/converge before the next dependent slice becomes eligible.

## Principle X — The Trusted Core Stays Small Enough to Audit

Abstraction is justified by current safety, correctness, interoperability, measured performance, or demonstrated product need—not speculative flexibility.

Core crates SHOULD minimize dependencies and side effects. I/O belongs behind explicit boundaries. Unsafe code, privileged browser hooks, identity/trust-root handling, secret handling, policy bypass paths, deserialization of untrusted data, and sandbox escape surfaces require explicit threat-model coverage and dedicated tests.

Security-relevant identifiers and scope dimensions SHOULD use strong typed representations instead of interchangeable strings where type confusion could affect authorization, audit, provenance, or persistence.

A plan that adds complexity violating this principle MUST document the simpler alternative and why it is insufficient in the plan's Complexity Tracking section.

## Mandatory Cross-Spec Gates

Every implementation plan MUST explicitly pass or fail the following gates before implementation:

- **G1 Domain coherence:** no competing canonical representation of constitutional concepts.
- **G2 Authority:** deny-by-default behavior, explicit scope semantics, and no ambient agent authority.
- **G3 Provenance:** important durable facts/claims preserve source and trust class.
- **G4 Side effects:** mutation, reversibility, idempotency, unknown-outcome, attempt identity, and retry semantics are explicit where relevant.
- **G5 Verification:** completion criteria do not depend on actor/executor self-report and verification has one authoritative record path.
- **G6 Durability:** restart/resume, reconciliation, and persisted-state behavior are defined for stateful work.
- **G7 Privacy/secrets:** secret, sensitive-data, logging, and telemetry flows are documented.
- **G8 Local-first:** any cloud dependency is justified and has a degraded/offline behavior when constitutionally required.
- **G9 Interoperability:** external protocols remain adapters and external authentication is not silently accepted as local authority.
- **G10 Donor/license:** source reuse is traceable and license-compatible.
- **G11 Upstream/browser maintenance:** privileged patches, browser permissions, bridge boundaries, and update strategy are explicit when applicable.
- **G12 Benchmarks:** acceptance metrics are reproducible and do not rely on marketing claims.
- **G13 Information flow / egress:** read/retrieval authority is distinct from disclosure; source-to-sink restrictions and remote-provider egress are explicit.
- **G14 Identity / principal binding:** actor attribution is not confused with authenticated principal identity; on-behalf-of/delegation/trust-root semantics have a named owner before privileged execution.
- **G15 Bounded execution:** resource/cost/time/tool/process/output budgets and safe exhaustion/cancellation behavior are explicit where execution can recurse, block, or consume unbounded resources.

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

When an analyze/review step finds a critical planning defect before implementation, downstream planning artifacts MUST be corrected and re-analyzed; a stale `TASKS_READY` or checklist PASS does not authorize implementation.

### Definition of Done

A buildable slice is not CLOSED_CANONICAL until:

1. all required tasks are complete;
2. relevant unit, contract, integration, security, migration, information-flow, resource-bound, and benchmark gates pass;
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

**Version:** 1.1.0  
**Ratified:** 2026-08-27  
**Last Amended:** 2026-08-27
