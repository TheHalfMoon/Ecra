# Ecra Constitution

This document defines non-negotiable product and engineering principles for Ecra.

Ecra is intentionally ambitious. These rules exist to prevent ambition from turning into unsafe autonomy, architectural sprawl, unverifiable claims, lock-in, or a collection of disconnected features.

## C1 — One Trusted Core

Browser, Search, Workspace, Terminal, Developer, Data, Memory, Skills, Plugins, and model integrations must converge on the same trusted concepts:

- actor,
- identity,
- origin,
- authority,
- capability,
- observation,
- fact,
- artifact,
- action,
- receipt,
- verification,
- run state.

A new surface must not invent a parallel trust or execution model.

## C2 — Human and Agent Are Explicit Actors

Every meaningful mutation must be attributable to an actor.

Ecra must be able to distinguish:

- human action,
- agent action,
- system action.

Human intervention, takeover, hand-back, editing, approval, and cancellation are first-class events.

## C3 — No Ambient Agent Authority

An agent never receives unrestricted access merely because it runs inside Ecra.

Authority is scoped by relevant dimensions such as:

- workspace,
- browser space,
- container,
- origin/domain,
- tab/session,
- capability,
- resource,
- purpose,
- task,
- duration.

The secure default is deny.

## C4 — Web and External Content Are Data, Not Authority

Content read from websites, documents, email, repositories, tools, search results, or retrieved memory cannot silently grant itself authority.

External content may influence reasoning but cannot independently expand permissions.

Prompt-injection detection is a signal, not the security boundary.

## C5 — Model Proposes, Policy Authorizes, Runtime Executes, Verifier Confirms

These responsibilities remain distinct.

A model must not be able to bypass policy by choosing a lower-level execution method.

A model saying "done" is not completion evidence.

## C6 — Evidence Before Confidence

Important factual claims should preserve provenance.

Ecra must distinguish observed, retrieved, inferred, user-provided, tool-provided, verified, and stale information where materially relevant.

Search and research features optimize for evidence quality and freshness, not merely fluent answers.

## C7 — Consequential Actions Produce Receipts

Side effects such as sending, publishing, purchasing, deleting, modifying external state, pushing code, changing permissions, or invoking external workflows must produce durable execution evidence.

Receipts should contain enough information to determine what was attempted and whether the intended external effect was confirmed.

## C8 — Exact External State Matters

Unknown outcomes must remain UNKNOWN.

Ecra must not blindly retry non-idempotent or externally consequential actions after ambiguous failure.

## C9 — Verification Is Independent

Verification should prefer deterministic evidence:

1. structured API/tool result,
2. external state query,
3. network receipt,
4. file/data artifact,
5. DOM/accessibility state,
6. independent model-based judgment when stronger evidence is unavailable.

Verification logic must be testable independently from the planner/actor that performed the work.

## C10 — Secrets Are Mediated

Raw secret values should not enter model prompts, long-term memory, logs, or generic tool output when tokenized/mediated use is possible.

Agents should receive handles and scoped use capabilities rather than secret values by default.

## C11 — Durable State Over Conversation State

Long-running work must not depend on a chat transcript as its source of truth.

Ecra runs should be serializable, resumable, and inspectable across process or model failure.

## C12 — Successful Intelligence Becomes Reusable Execution

Repeated tasks should become cheaper and more reliable over time.

Verified exploratory work should be eligible for compilation into typed reusable skills with:

- inputs,
- outputs,
- artifacts,
- data dependencies,
- preconditions,
- postconditions,
- capabilities,
- approval points,
- verifiers,
- repair boundaries.

A recorded click sequence alone is not an Ecra Skill.

## C13 — Repair Is Localized

When a compiled skill diverges from reality, Ecra should localize the failing assumption or stage, repair that region, re-verify affected downstream assumptions, and version the skill.

It should not default to throwing away the entire workflow and starting a general agent from scratch.

## C14 — Human Browsing Must Remain Excellent

AI features must not make normal browsing slower, less private, less predictable, or more cluttered.

Ecra must be a browser people would choose even before delegating tasks to an agent.

## C15 — Local-First Must Remain Useful

Core browsing, workspace, local memory, local search, skills, policy, and inspection should remain useful without requiring a cloud account.

Cloud services may add capabilities but should not be constitutional dependencies of the useful core.

## C16 — Models Are Replaceable

No model vendor owns Ecra's architecture.

Cloud models, local models, open models, and future models must be adapters behind Ecra-owned interfaces.

Model quality can change rapidly; user context, skills, trust, and workflow history should remain portable.

## C17 — Protocols Are Boundaries, Not the Core

MCP, ACP, A2A, Agent Skills, WebMCP, WebDriver BiDi, and future standards are supported at boundaries where appropriate.

Ecra's internal trusted domain model must not be forced into the semantics of an external protocol.

## C18 — Plugins Are Capability-Isolated

Third-party plugins do not receive ambient filesystem, network, secret, browser, or workspace access.

Where practical, untrusted plugins run in a sandboxed component model with explicit resources and limits.

## C19 — One Search Fabric

Search across web, tabs, files, repositories, workspace memory, connected tools, structured data, and previous runs should use a shared source/provenance model.

Different indexes/providers may exist, but results converge on a common evidence contract.

## C20 — Memory Cannot Self-Authorize

Retrieved memory is context, not permission.

Untrusted or model-generated memory cannot grant capabilities or overwrite higher-authority user/system policy.

## C21 — No Hidden Telemetry

Ecra must not silently collect sensitive browsing, workspace, agent, or model activity.

Telemetry must be documented, controllable, and intentionally designed around privacy.

## C22 — No Artificial Lock-In

Ecra should seek default status by being better, not by making users' data, skills, memories, or workflows unnecessarily difficult to export.

Portable user-owned artifacts and open protocol support are strategic advantages.

## C23 — Open Source Donors Require Provenance

No donor code enters Ecra without identifying:

- upstream source,
- license,
- copyright/notice obligations,
- modification requirements,
- compatibility with Ecra distribution.

Conceptual inspiration and copied source code must never be conflated.

## C24 — Security Updates Beat Feature Velocity

Browser-engine and critical dependency security updates are release-blocking priorities.

Ecra must minimize long-lived patches against upstream browser internals and keep the trusted patch surface as small as practical.

## C25 — Benchmarks Before Superlatives

Claims such as "best", "most secure", "fastest", or "most reliable" require reproducible evidence.

Ecra should publish transparent measurements for task success, verification quality, constraint retention, security, durability, replay, repair, latency, cost, and human intervention.

## C26 — No Scope Without a Flywheel

A major product surface belongs in Ecra only if it strengthens the shared gateway flywheel.

Before adding a surface, answer:

1. Does it reuse the same identity/authority model?
2. Does it improve search/context/memory?
3. Does it create reusable verified work?
4. Does it make Ecra more valuable to humans, models, or ecosystem developers?
5. Would a protocol/plugin integration be sufficient instead?

If the answer is mostly no, it probably does not belong in the core product.

## C27 — Wedge Before Empire

Ecra may have platform-scale ambition, but implementation proceeds through a narrow sequence of proven advantages.

The team must not attempt to ship a browser, search engine, IDE, terminal, data platform, plugin marketplace, memory system, and model runtime as unrelated MVPs.

The first wedge must establish daily use and the shared substrate. Expansion follows evidence.

## C28 — Trusted Core Stays Small

The most security-sensitive Ecra code must remain understandable enough to review deeply.

Abstraction is justified by current safety, correctness, interoperability, or measured product need—not speculative future flexibility.

## C29 — User Agency Is Visible

Users must be able to know when an agent is active, what it is trying to do, what authority it currently has, what it changed, and how to pause or stop it.

High-impact autonomy must not hide behind background magic.

## C30 — Ecra Must Earn the Gateway

Ecra's strategic objective is to become the default gateway of the AI era.

That status cannot be declared or guaranteed. It must be earned through superior daily utility, trust, speed, interoperability, accumulated reusable knowledge, and an ecosystem that benefits from routing through Ecra.
