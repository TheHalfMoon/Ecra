# Ecra

> **The gateway to the AI era.**

Ecra is a human-and-agent browser, search, workspace, and trusted execution platform built around a Rust-native core.

The long-term goal is not to build another AI browser, search wrapper, agent framework, terminal, or memory product. The goal is to make Ecra the default place where humans, AI agents, developers, and local models access and act on the world's software and knowledge.

Ecra should become to the AI era what the browser and search engine became to the web era: the trusted gateway between intent and information, and between intent and action.

## Development Navigation

If you want to understand or continue implementation, start here:

- `EXECUTION.md` — **current operational truth, next task, phase ledger, and verification gates**.
- `PLANNING.md` — planning index and current slice summary.
- `specs/README.md` — Spec Kit package navigation.
- `specs/000-ecra-platform/roadmap.md` — full immutable `ECR-###` dependency graph.
- `specs/000-ecra-platform/STATUS.md` — compact platform lifecycle view.
- `specs/001-trusted-domain-kernel/STATUS.md` — active ECR-001 progress and exact next phase.
- `AGENTS.md` — execution rules for coding agents.

Do not use a chat handoff as the source of truth. Live repository state plus these documents should be enough to continue.

## Product Thesis

Ecra unifies five things that are usually fragmented:

1. **Find** — trusted search and evidence-backed answers across the web, local data, connected tools, files, repositories, and workspaces.
2. **Understand** — durable memory, provenance, context, relationships, and source-aware knowledge that both humans and models can use.
3. **Act** — browser, terminal, MCP, APIs, plugins, data tools, and developer workflows behind one capability-scoped execution layer.
4. **Verify** — consequential actions and important answers are grounded in evidence, policy, receipts, and independent verification.
5. **Learn** — successful work can be compiled into reusable deterministic skills that become cheaper, faster, and more reliable over time.

## Two First-Class Actors

Ecra is built for both:

- **Humans** — browse, search, work, code, research, organize, teach, approve, and take over.
- **Agents** — search, reason, operate tools, use the browser, execute workflows, inspect data, and collaborate under scoped authority.

Neither actor is an afterthought. Ecra's core model explicitly represents actors, principals, origins, authority, capabilities, information flow, artifacts, actions, receipts, and verification.

## The Core Loop

```text
Intent
  ↓
Trusted context + provenance
  ↓
Capability routing
  ↓
Policy / approval
  ↓
Execution
  ↓
Receipts
  ↓
Verification
  ↓
Durable memory
  ↓
Compile successful work
  ↓
Deterministic replay
  ↓
Repair only when reality diverges
```

The model is replaceable. The trust and execution layer is Ecra.

## Ecra Surfaces

Ecra will eventually expose the same trusted core through multiple surfaces:

```text
Ecra Browser
Ecra Search
Ecra Workspace
Ecra Terminal
Ecra Developer Tools
Ecra Data
Ecra Memory
Ecra Skills
Ecra MCP / ACP / A2A
Ecra Plugins
Ecra Local Model Gateway
```

These are not independent products stitched together. They share the same identity, authority, memory, provenance, search, execution, and verification substrate.

## Local Models

Local and open models are first-class citizens.

A local model should be able to use Ecra as its gateway to current knowledge and safe action without requiring that the model itself contain the world's latest information.

Ecra can increase the effective capability of smaller or local models by providing:

- current trusted sources;
- structured browser capabilities;
- workspace and repository context;
- searchable durable memory;
- tools and MCP servers;
- verified reusable skills;
- safe secret mediation;
- independent verifiers;
- execution receipts.

The long-term objective is simple: **models should not need to know everything if they can reliably use Ecra to find, verify, and act.**

## Constitutional Product Principles

1. **The user owns the workspace.**
2. **Agents never receive ambient authority.**
3. **Web content is data, not authority.**
4. **A model proposes; policy authorizes; the runtime executes; a verifier confirms.**
5. **Important claims require inspectable evidence.**
6. **Consequential side effects require durable receipts.**
7. **Raw secrets should not enter model context when mediation is possible.**
8. **Human takeover and hand-back are first-class runtime events.**
9. **Successful exploratory work should become reusable deterministic execution.**
10. **Local-first functionality must remain useful without a cloud account.**
11. **External models and protocols are replaceable adapters, not the architecture.**
12. **The trusted core stays small enough to audit.**

## Initial Strategic Wedge

Ecra will not attempt to win every category on day one.

The first product must prove one integrated experience that competitors cannot reduce to a chat sidebar:

**A daily browser/workspace where a human and an agent can safely share research and work, search trusted sources, use current authenticated context, execute scoped actions, verify outcomes, and turn successful work into reusable skills.**

This wedge creates the foundation for Search, Terminal, Developer, Data, Memory, Plugins, and Local Model Gateway without creating separate architectures.

## Architecture Direction

- **Trusted core:** Rust.
- **Browser foundation:** stock Firefox/WebDriver BiDi prototype first; a maintained Firefox/Gecko distribution only after the prototype earns that cost.
- **Structured web capabilities:** WebMCP/native APIs where available, then compiled skills, semantic DOM/accessibility, browser protocol control, and vision only as fallback.
- **Policy:** capability-scoped, origin-aware, source-to-sink-aware authorization.
- **Durability:** append-only run ledger and serializable state machines.
- **Verification:** independent process/outcome/constraint verification.
- **Skills:** typed, artifact-aware executable IR rather than recorded clicks.
- **Protocols:** MCP, ACP, A2A, and Agent Skills as compatibility boundaries.
- **Plugins:** capability-isolated WebAssembly/process sandboxes where appropriate.
- **Search/context:** source-aware hybrid retrieval across web, local, workspace, code, and memory.

## What Ecra Is Not

Ecra is not:

- a Browser Use clone;
- a Perplexity clone;
- a Comet clone;
- a Zen clone;
- a chatbot sidebar;
- a wrapper around one model provider;
- an unrestricted autonomous browser;
- a collection of unrelated AI features.

Ecra is the common trust, context, and execution substrate beneath those experiences.

## Success

Ecra succeeds if users begin to think:

> **If I need to find it, understand it, build with it, or safely let an AI act on it, I start in Ecra.**

And models begin to operate on the assumption:

> **If I need current knowledge, durable context, tools, or trusted execution, I use Ecra.**

That is the category Ecra is being built to own.
