# Ecra Roadmap

This roadmap is designed around a single strategic objective:

> **Earn the position of default gateway for the AI era.**

The roadmap intentionally separates **platform ambition** from **implementation order**. Ecra will not attempt to win browser, search, terminal, developer tools, data, memory, plugins, and model infrastructure as independent products at the same time.

Every phase must strengthen the same gateway substrate.

## Phase 0 — Category and Contracts

### Goal

Define what Ecra is before implementation creates accidental architecture.

### Deliverables

- Product thesis and vision.
- Constitution.
- Threat model.
- Actor / authority / capability model.
- Origin security model.
- Observation / fact / provenance model.
- Action and receipt model.
- Durable run schema.
- Verifier contract.
- Ecra Skill IR specification.
- Search/evidence contract.
- Memory contract.
- Browser foundation decision record.
- Protocol boundary strategy.
- Donor and license ledger.
- Benchmark strategy.

### Exit Gate

No foundational concept has two competing canonical representations.

The architecture can explain the full path:

```text
human/model intent
→ context/evidence
→ capability resolution
→ policy
→ execution
→ receipt
→ verification
→ durable state
→ optional skill compilation
```

without reference to a specific model vendor.

---

## Phase 1 — Trusted Rust Substrate

### Goal

Build the smallest useful Ecra core before building a custom browser distribution.

### Initial crates

```text
ecra-core
ecra-run
ecra-ledger
ecra-policy
ecra-verify
ecra-skill
ecra-protocols
ecra-cli
```

### Deliverables

- Typed Actor, Origin, Capability, Action, Observation, Fact, Artifact, Receipt.
- Serializable run state machine.
- Append-only local run ledger.
- SQLite-backed portable run artifact.
- Policy evaluation boundary.
- Approval events.
- Deterministic verifier interface.
- CLI inspect/resume/verify primitives.
- Model-provider-neutral interfaces.

### Exit Gates

- Run survives process restart.
- No non-idempotent action is blindly retried after unknown outcome.
- Human/agent/system actions remain attributable.
- Verification can reject an agent's false completion claim.
- Core operates without a cloud account.

---

## Phase 2 — Stock Firefox Execution Prototype

### Goal

Prove the human-agent execution model against stock Firefox before maintaining a browser fork.

### Deliverables

- WebDriver BiDi integration.
- Firefox Remote Agent experiments.
- Accessibility/DOM observations.
- Navigation, reading, forms, downloads, tab management.
- Browser action receipts.
- Origin tracking.
- Human takeover/hand-back state transitions.
- Dedicated profile/container experiments.
- Initial prompt-injection test corpus.

### Exit Gates

- Agent can complete bounded browser tasks with exact receipts.
- Human can take over and return control without corrupting run state.
- Origin transitions trigger authority re-evaluation.
- Browser session failure can resume without duplicating consequential effects.

---

## Phase 3 — Ecra Browser Wedge

### Goal

Ship a daily browser that humans would choose even without autonomous delegation.

### Foundation

- Firefox / Gecko.
- Minimal upstream patch surface.
- Selective UX inspiration/code from compatible Firefox ecosystem donors where justified.

### Core experiences

- Vertical tab / workspace-oriented UX.
- Spaces.
- Containers as real state/security isolation.
- Human tabs.
- Agent tabs.
- Shared tabs.
- Visible control owner.
- Watch / Pause / Take Over / Hand Back.
- Agent workspace.
- Scoped browser authority.
- Secret mediation.

### Exit Gates

- Normal browsing performance and reliability remain competitive with upstream Firefox.
- Users can understand when and where an agent has authority.
- Sensitive spaces/containers remain inaccessible to unauthorized agents.
- Daily use does not require an AI model call.

---

## Phase 4 — Ecra Search: Trusted Answer and Retrieval Fabric

### Goal

Make Ecra the best place to begin information-seeking inside the user's authorized world.

### Search scopes

```text
live web
official documentation
papers
open tabs
history/bookmarks
files
repositories
workspace memory
previous Ecra runs
skills
connected tools
structured data
```

### Deliverables

- Source ranking and provenance.
- Freshness timestamps.
- Contradiction detection.
- Evidence-backed synthesis.
- Search scope controls.
- Official/primary-source preference where appropriate.
- Local full-text search.
- Hybrid structured/semantic retrieval.
- Inspectable answer-to-source mapping.

### Product Requirement

Search is not a detached search page. Search must be available anywhere context exists.

### Exit Gates

- Important synthesized claims have inspectable evidence coverage.
- Search can distinguish stale, inferred, and verified information.
- Workspace/private search remains useful locally.
- Ecra demonstrates measurable value over generic web-only answer engines for multi-source workspace questions.

---

## Phase 5 — Ecra Memory and Workspace

### Goal

Turn temporary sessions into durable user-owned context without creating an untrusted memory swamp.

### Deliverables

- Workspace as long-term scope for context and authority.
- Project facts, decisions, artifacts, sources, questions, and workflows.
- Provenance-aware memory.
- Candidate-memory review/policy.
- Memory deletion/export.
- Memory aging/staleness.
- Distinction between user facts, observations, model inferences, and verified facts.
- Cross-surface retrieval from Browser and Search.

### Exit Gates

- Untrusted web content cannot silently become high-authority durable memory.
- A returning user can resume a real project without reconstructing context manually.
- Memory can be inspected and exported independently of a model provider.

---

## Phase 6 — Ecra Skill Compiler

### Goal

Create the primary economic and reliability moat.

### Input trajectories

- human demonstration,
- verified exploratory agent run,
- hand-authored workflow,
- imported compatible skill.

### Skill IR requirements

- intent,
- typed inputs/outputs,
- artifacts,
- explicit reads/writes,
- data dependencies,
- stages,
- preconditions,
- postconditions,
- capability requirements,
- origins,
- approval points,
- side-effect semantics,
- deterministic verifiers,
- repair boundaries.

### Pipeline

```text
Demonstrate / Explore
        ↓
Verified trajectory
        ↓
Semantic normalization
        ↓
Intent → Stage → Action
        ↓
Artifact/dataflow inference
        ↓
Skill IR
        ↓
Sandbox replay
        ↓
Verification
        ↓
Versioned reusable skill
```

### Exit Gates

- A meaningful percentage of repeated successful tasks can run with zero model calls while the environment remains compatible.
- Skill execution is independently verifiable.
- Human demonstrations and agent demonstrations compile to the same IR.

### North-Star Metrics

- Compile Yield.
- Replay Success.
- Model Calls Avoided.
- Cost per Successful Repeated Task.

---

## Phase 7 — Divergence Detection and Repair

### Goal

Make compiled skills resilient to real-world change.

### Deliverables

- Explicit assumption tracking.
- Stage-local divergence detection.
- Repair planner.
- Candidate patch generation.
- Sandbox re-execution.
- Downstream assumption invalidation.
- Versioned skill promotion.

### Exit Gates

- Most common website/UI drift can be repaired without regenerating the entire workflow.
- Repairs cannot silently bypass capability/approval policies.
- Skill history remains auditable.

### North-Star Metric

- Repair Success per model call and per unit cost.

---

## Phase 8 — Ecra Terminal and Developer

### Goal

Extend the same trusted substrate from the web into software development and local execution.

### Ecra Terminal

- human/agent shared sessions,
- policy-scoped filesystem/network/process capabilities,
- bounded output/process lifetime,
- sandbox providers,
- durable receipts,
- takeover and approval.

### Ecra Developer

- repository context,
- structural search,
- current documentation,
- browser QA,
- tests/build execution,
- code-review evidence,
- reproducible release workflows,
- MCP/ACP integration.

### Exit Gate

Terminal and developer execution use the same Actor/Capability/Receipt/Verifier model as the browser rather than a parallel agent system.

---

## Phase 9 — Ecra Data

### Goal

Make Ecra the trusted gateway for analytical and structured-data work.

### Surfaces

- CSV/Parquet/files,
- SQL,
- APIs,
- local notebooks/runtimes,
- connected warehouses/databases,
- charts/reports,
- data-derived evidence.

### Requirements

- source lineage,
- query/tool receipts,
- reproducible transformations where practical,
- explicit uncertainty/data-quality warnings,
- no generated number without traceable computation/evidence.

### Exit Gate

A factual analytical conclusion can be traced from claim → calculation/query → source data.

---

## Phase 10 — Ecra Gateway and Ecosystem

### Goal

Make Ecra useful even when the user is not inside the Ecra UI.

### External interfaces

- MCP server/client.
- ACP agent/proxy/client integration.
- A2A compatibility.
- Agent Skills import/export.
- WebMCP support.
- Rust SDK.
- stable local API.

### Ecosystem extensions

- search providers,
- connectors,
- model adapters,
- verifiers,
- plugins,
- skills,
- workspace integrations,
- browser capability providers.

### Security rule

External callers receive explicit gateway capabilities, never implicit access to the user's entire Ecra state.

### Exit Gate

At least one major workflow is demonstrably better by having a third-party agent/model use Ecra as infrastructure rather than operating directly against raw tools/web context.

---

## Phase 11 — Ecra Local Model Gateway

### Goal

Use Ecra to raise the effective intelligence of local and open models.

### Capabilities

- current trusted search,
- source-aware context,
- workspace memory,
- skill invocation,
- browser/terminal/data actions through policy,
- secret mediation,
- independent verification.

### Key Experiment

Measure whether smaller/local models augmented by Ecra can match or outperform larger unaided models on selected real workflows at lower privacy/cost tradeoffs.

### Exit Gate

Publish reproducible evidence showing where Ecra augmentation materially improves local-model capability.

---

## Phase 12 — Ecosystem Flywheel

### Goal

Turn Ecra from product into standard infrastructure.

### Flywheel

```text
More users/models
      ↓
More useful skills/connectors/verifiers
      ↓
More tasks Ecra can handle safely
      ↓
More reasons to start in Ecra
      ↓
More ecosystem developers
```

### Requirements

- portable skills,
- clear extension contracts,
- high-quality registry/discovery,
- security review/signing paths,
- trust/reputation metadata,
- reproducible benchmarks,
- backward-compatible public interfaces.

---

# Cross-Phase Benchmark Program

Ecra tracks at least these dimensions from early development:

## Correctness

- Task Success.
- Constraint Retention.
- Artifact Correctness.

## Trust

- Evidence Coverage.
- Provenance Coverage.
- Unsupported Claim Rate.

## Verification

- Verifier False Positive Rate.
- Verifier False Negative Rate.
- Critical-Point Violation Rate.

## Security

- Prompt-Injection Attack Success Rate.
- Cross-Origin Leakage.
- Capability Overreach.
- Secret Exposure Rate.
- Memory-Poisoning Survival.

## Durability

- Crash/Resume Success.
- Duplicate Side-Effect Rate.
- Unknown-Outcome Handling Accuracy.

## Reuse

- Compile Yield.
- Replay Success.
- Repair Success.
- Skill Version Stability.

## Economics

- Model Calls per Successful Task.
- Tokens per Successful Task.
- Cost per Successful Task.
- Cost per Repeated Task.
- Time per Successful Task.

## Human-Agent UX

- Takeover Latency.
- Unnecessary Approval Rate.
- Intervention Precision.
- Human Correction Recovery.

# Strategic Kill Criteria

Ecra should stop or redesign a major direction when evidence shows that it:

- creates a second trusted core,
- requires ambient agent authority,
- degrades human browser quality materially,
- cannot be verified independently,
- creates vendor/model lock-in without overwhelming benefit,
- adds complexity without strengthening the gateway flywheel,
- or is better delivered as a plugin/protocol integration than a core surface.

# The Sequence That Matters

The intended compounding path is:

```text
Daily Browser
    ↓
Trusted Search
    ↓
Workspace + Memory
    ↓
Verified Skills
    ↓
Replay + Repair
    ↓
Terminal + Developer + Data
    ↓
Gateway for external agents/models
    ↓
Local-model augmentation
    ↓
Open ecosystem
```

The sequence is deliberate.

Ecra earns the right to expand by proving each layer strengthens the next.
