# Ecra Vision

## Mission

Build the default gateway for the AI era.

Ecra should be the place where humans and AI systems begin when they need to discover information, understand context, use software, work with data, collaborate, or safely cause change in the digital world.

The ambition is intentionally larger than an AI browser. The browser is the first distribution surface because it already sits between people and the web. The Ecra core should eventually sit between **intent and digital reality**.

## The Era Shift

The web era established a small number of default gateways:

- browsers became the interface to the web,
- search engines became the interface to information,
- operating systems became the interface to local software,
- developer platforms became the interface to building and shipping software.

The AI era creates a new gateway problem.

Humans and models now need to:

- search live information,
- distinguish trusted evidence from generated claims,
- operate authenticated software,
- preserve context across long tasks,
- collaborate with each other,
- access tools through common protocols,
- work with private and local data,
- execute actions safely,
- verify outcomes,
- and reuse successful procedures.

Today these capabilities are fragmented across browsers, search engines, agent frameworks, terminals, IDEs, MCP clients, memory products, automation systems, data platforms, and model providers.

Ecra's opportunity is to unify the gateway without collapsing the security boundaries between those systems.

## North Star

> **If a human or an AI needs to find, understand, build, or safely act on something digital, Ecra should be the best place to start.**

## The Ecra Flywheel

Ecra should become more valuable as more work passes through it.

```text
More human work + agent work
          ↓
More trusted trajectories and evidence
          ↓
Better context and reusable skills
          ↓
Faster, cheaper, more reliable execution
          ↓
Better experience for humans and models
          ↓
More work begins in Ecra
```

This flywheel must be built without depending on surveillance or hidden telemetry. User-owned/private trajectories can improve that user's Ecra locally; shared/public skills and benchmarks can improve the ecosystem explicitly.

## The Seven Moats

### 1. Trust Graph

Ecra knows where information came from, how it was observed, when it was observed, which actor produced it, and whether it was independently verified.

Search results, memories, model outputs, browser observations, files, API responses, and workflow results do not collapse into undifferentiated text.

### 2. Execution Graph

Ecra records how intent became action:

```text
intent → plan → capability → policy → action → receipt → verification
```

This produces an inspectable and reproducible execution history rather than opaque agent behavior.

### 3. Skill Graph

Successful human and agent work can be compiled into typed reusable workflows with explicit inputs, outputs, data dependencies, preconditions, postconditions, capabilities, and verifiers.

The goal is to convert expensive exploratory intelligence into deterministic reusable execution.

### 4. Workspace Memory

Ecra remembers projects, sources, artifacts, decisions, workflows, open questions, and verified facts across browser, search, terminal, repositories, files, and connected tools.

Memory is source-aware, scoped, inspectable, and deletable.

### 5. Human-Agent Collaboration

Humans and agents are explicit actors with visible control ownership, take-over/hand-back, scoped workspaces, shared tabs, approvals, and auditability.

The user is never reduced to a confirmation button attached to an autonomous process.

### 6. Model Independence

Ecra makes models more capable without binding the product to one vendor.

Frontier cloud models, local models, open models, and specialized models can all use the same Ecra context, tools, skills, memory, and verification substrate.

A smaller local model paired with high-quality Ecra context and skills should often outperform a larger model operating blindly.

### 7. Ecosystem Gateway

MCP, ACP, A2A, Agent Skills, WebMCP, plugins, APIs, and developer SDKs allow Ecra to become infrastructure that other agents and applications use.

The goal is not only to have users inside Ecra. The goal is for other AI systems to route trusted knowledge and execution through Ecra.

## Product Surfaces

### Ecra Browser

The primary human distribution surface.

- daily browsing,
- Spaces and Containers,
- human/agent/shared tabs,
- trusted search,
- take-over and hand-back,
- visible agent actions,
- scoped credentials,
- browser-native skills,
- WebMCP and WebDriver BiDi,
- agent workspaces.

### Ecra Search

A source-first answer and retrieval layer across:

- live web,
- official documentation,
- papers,
- files,
- repositories,
- local data,
- connected services,
- workspace memory,
- structured databases.

Ecra Search should optimize for provenance, freshness, contradiction detection, source quality, and inspectability rather than answer fluency alone.

### Ecra Workspace

A durable project context that joins browsing, research, files, tasks, code, data, memories, agents, and reusable skills.

A workspace is the unit of long-term context and authority.

### Ecra Terminal

A human-and-agent terminal connected to the same policy, context, memory, receipts, and verification substrate as the browser.

Terminal access is capability-scoped and sandboxable. It is not a hidden shell granted to every agent.

### Ecra Developer

Developer-oriented capabilities:

- repository understanding,
- terminal and test execution,
- browser QA,
- current documentation,
- code/search graphs,
- MCP/ACP tooling,
- reproducible agent runs,
- verified build/test/release workflows.

### Ecra Data

Trusted work across local files, SQL databases, notebooks, APIs, structured datasets, and analytics tools.

The same provenance and verification rules apply to data-derived claims.

### Ecra Memory

User-owned memory for projects and agents.

Memory is typed, scoped, versioned where necessary, source-aware, and resistant to untrusted content silently becoming authority.

### Ecra Skills

The executable knowledge layer.

Skills can be created from:

- human demonstrations,
- verified agent trajectories,
- hand-authored workflows,
- imported ecosystem skills.

### Ecra Gateway

The programmatic surface used by external agents, local models, IDEs, services, and applications.

Ecra Gateway exposes trusted search, context, skills, memory, and execution without giving external callers ambient access to the user's browser or data.

## The Local Model Thesis

Local models should be able to become dramatically more useful through Ecra.

A model does not need every fact in its weights if Ecra can provide:

```text
fresh search
+ trusted sources
+ structured context
+ durable memory
+ tools
+ skills
+ safe secrets
+ verification
```

Ecra should therefore be designed as a **knowledge and action augmentation layer** for models.

The long-term ideal:

> A local model opens Ecra and immediately gains access to the user's authorized world without gaining unrestricted access to the user's world.

## Search as a Gateway, Not a Page

Ecra Search is not merely a search-results UI.

Every surface should be searchable:

```text
web
open tabs
history
bookmarks
papers
files
repositories
terminal output
databases
workspace memory
previous agent runs
skills
connected tools
```

The same query can resolve against different scopes with explicit provenance.

Example:

> "What changed in our authentication flow since the incident?"

Ecra may combine:

- repository commits,
- issue/PR history,
- internal workspace notes,
- browser research,
- official dependency documentation,
- previous incident runs.

Each claim remains linked to its evidence.

## The Gateway Strategy

Ecra cannot become the default AI-era gateway by launching ten mediocre products simultaneously.

The platform strategy is sequential.

### Wedge

Win daily human-agent browsing and trusted research/work execution.

### Expansion

Once identity, authority, search, memory, skills, and execution exist in the browser, extend the same substrate into terminal, developer workflows, data, plugins, and local models.

### Infrastructure

Make Ecra useful even when the user is not looking at the Ecra UI by exposing its trusted substrate through protocols and SDKs.

### Ecosystem

Allow developers and organizations to publish skills, plugins, connectors, search providers, model adapters, verifiers, and workspace integrations.

## What Must Be True to Become Number One

Market leadership is not a feature. It requires compounding advantages.

Ecra must become:

1. **More useful every day** than a conventional browser plus separate AI tools.
2. **More trustworthy** than opaque autonomous agents.
3. **Cheaper over repeated work** because successful work compiles into reusable skills.
4. **Model-independent** so users do not need to migrate their digital environment when model leadership changes.
5. **Local-first enough** that developers and privacy-sensitive users can depend on it.
6. **Open enough** to become infrastructure for other agents and developers.
7. **Simple enough** that the core remains understandable and reliable.
8. **Fast enough** that AI features do not make normal browsing worse.
9. **Beautiful enough** to become a daily human product, not only infrastructure.
10. **Measurably better** through public, reproducible benchmarks instead of marketing claims.

## Non-Goal: Forced Exclusivity

Ecra does not win by preventing people from using other tools.

It wins when using Ecra becomes the rational default because it provides superior trust, context, interoperability, execution, and accumulated reusable knowledge.

The objective is **earned default status**, not artificial lock-in.

## End State

The desired end state is that humans say:

> "Open Ecra."

when they want to search, research, work, code, analyze, or delegate.

Developers say:

> "Expose it through Ecra."

when they want an application or data source to be safely usable by agents.

And models behave as though:

> "Use Ecra to obtain current trusted context and perform real-world work."

That is the platform Ecra is intended to become.
