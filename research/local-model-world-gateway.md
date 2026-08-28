# Ecra Local Model World Gateway — Architecture Research Note

**Status:** CANONICAL_PLANNING_INPUT  
**Date:** 2026-08-28  
**Branch:** `docs/local-model-world-gateway`  
**Affected slices:** ECR-003, ECR-004, ECR-005, ECR-009, ECR-010, ECR-011, ECR-016, ECR-017, ECR-021, ECR-024, ECR-028, ECR-031

## 1. Thesis

Ecra should not merely support local models as another inference provider. Ecra should become the default world interface through which local models obtain current information, durable context, tools, authenticated capabilities, execution, and verification.

The model is a replaceable reasoning engine. Ecra owns the trusted path between model intent and the outside world.

```text
Local model / local agent
          │
          ▼
     Ecra Model API
          │
          ▼
┌──────────────────────────────┐
│ Ecra trusted gateway        │
│                              │
│ intent + context routing     │
│ evidence compilation         │
│ information-flow policy      │
│ adaptive tool aperture       │
│ capability resolution        │
│ authority / approval         │
│ secret mediation             │
│ durable run / budgets        │
│ execution receipts           │
│ verification                 │
└──────────────┬───────────────┘
               │
     ┌─────────┼─────────┐
     ▼         ▼         ▼
   Web       Local      Actions
 search      files      WebMCP
 sources     repos      browser
 APIs        memory     MCP
 docs        data       terminal
```

The target product property is stronger than “Ecra can connect to Ollama.” It is:

> A local model can remain isolated from direct world access while becoming more useful because Ecra supplies current, provenance-aware information and scoped execution through one stable interface.

This is not a mandate to block all direct networking in every deployment. It is the preferred high-assurance runtime profile for agentic/local-model execution.

## 2. Why this matters

Local/open models have structural disadvantages that model weights alone cannot solve reliably:

- stale training knowledge;
- small context windows;
- weaker tool selection as tool count grows;
- inconsistent structured output/tool calling;
- no durable user/workspace memory by default;
- no principled provenance/freshness model;
- no inherent authority or secret-handling model;
- no durable side-effect/retry semantics;
- no independent verification of consequential actions.

Ecra can increase **effective intelligence** without modifying model weights by moving these responsibilities into a trusted external substrate.

A smaller model should not need to know everything if it can reliably ask Ecra to find, compress, attribute, authorize, execute, and verify.

## 3. Core invariant: world access is mediated, not ambient

For the preferred isolated agent profile, the model/agent process has no ambient access to:

- arbitrary network egress;
- raw browser session secrets;
- raw API tokens;
- SSH keys;
- unrestricted filesystem roots;
- arbitrary MCP servers;
- arbitrary process execution;
- authenticated browser capabilities.

It receives explicit Ecra capabilities instead.

```text
Agent sandbox
  ├─ arbitrary Internet        DENY by default
  ├─ arbitrary host FS         DENY by default
  ├─ raw credentials           DENY
  ├─ ambient shell authority   DENY
  └─ Ecra gateway              ALLOW through bounded IPC/API
```

This does not make the model trusted. It reduces the model to a proposal/reasoning component and preserves Ecra as the authority and information-flow boundary.

## 4. Evidence-first model context

ECR-009 should expose a stable evidence/context contract rather than returning an unstructured dump of pages.

Candidate conceptual types for ECR-009 research:

```text
SourceRef
SourceSnapshot
Claim
ClaimEvidence
EvidenceSet
ContradictionSet
FreshnessRequirement
SourceIndependence
EvidencePack
ContextProjection
```

An `EvidencePack` should be capable of representing:

- original query/intention;
- claims and claim status;
- source identities;
- captured-at/as-of timestamps;
- content/snapshot digests where required;
- source lineage and independence;
- contradiction/disagreement sets;
- freshness status;
- information classification;
- allowed downstream uses/sinks;
- drill-down references to larger source material.

Important: the pack is an evidence transport/assembly structure, not a new authorization object and not an independent verification truth channel. `VerificationReceipt` remains the canonical verification outcome record.

## 5. Context compiler

Ecra should compile context for the receiving model rather than assuming every model can consume the same prompt/tool surface.

Candidate inputs:

```text
ModelProfile
Intent
RunBudget
InformationUse
Workspace scope
Evidence candidates
Memory candidates
Available tools/capabilities
```

Candidate `ModelProfile` characteristics:

```text
context_limit
reliable_structured_output_profile
reliable_tool_count
reasoning/tool-use class
supported modalities
local/remote sink classification
latency/resource profile
```

The context compiler should support progressive disclosure:

1. send compact decision-relevant claims, provenance and a small tool aperture;
2. expose stable references for expansion;
3. expand only the claim/source/tool subset needed by the next reasoning step;
4. re-run information-flow checks before adding newly selected context to a remote sink.

This reduces token pressure and gives small local models a better chance of reliable reasoning.

## 6. Adaptive Tool Aperture

Do not expose every Ecra/MCP/browser/tool capability to every model step.

Ecra should derive a bounded candidate tool set from intent, state, authority requirements, risk, and model capability.

Example research intent:

```text
"Compare the current migration guidance for this Rust crate."
```

Candidate aperture:

```text
ecra.search
ecra.source.open
ecra.evidence.compare
ecra.workspace.search
```

A booking intent may expose a different set:

```text
ecra.web.capabilities
ecra.action.prepare
ecra.action.execute
ecra.verify
```

The aperture is a model-facing usability/reliability optimization. It must never become an authorization shortcut. Every concrete action still resolves to canonical Ecra action semantics and fresh policy.

## 7. WebMCP as a preferred semantic web capability source

Research snapshot:

- `GoogleChromeLabs/webmcp-tools` main observed at `d39eae4bd51e8c12736b8cae840bd98f190f3179` on 2026-08-28.
- Repository license: Apache-2.0.
- Repository includes WebMCP inspection utilities, a polyfill, demos, and `webmcp-evals`.
- `webmcp-evals` supports static-schema evaluation, live browser evaluation, deterministic smoke execution, and an Ollama backend.

Ecra should treat WebMCP as a preferred semantic route when available because a structured capability can be substantially more reliable for smaller models than screenshot/coordinate inference.

Preferred semantic routing order remains:

```text
1. site/native structured capability, including WebMCP
2. stable API/protocol adapter
3. verified compiled Ecra Skill
4. semantic AX/DOM representation
5. browser protocol control
6. vision/coordinates fallback
```

Security invariant:

> A structured website tool is a `CapabilityOffer`, not a `CapabilityGrant`.

The page may describe what it can do. The page cannot authorize itself. Ecra must bind origin, schema/version identity, concrete parameters, information disclosure, effect, and verifier requirements before authorization/execution.

## 8. Sandboxed execution and OpenSandbox donor analysis

Research snapshot:

- `opensandbox-group/OpenSandbox` main observed at `48b0215f1bd097b31d0f022a44640e00c11ac49d` on 2026-08-28.
- Repository license: Apache-2.0.
- It provides multi-language sandbox SDKs, CLI/MCP, Docker/Kubernetes runtimes, command/filesystem/code-interpreter environments, egress controls, Credential Vault, and stronger runtime options including gVisor/Kata/Firecracker integrations.
- Its egress design supports default-deny policy and credential injection at the outbound boundary so workloads need not receive real credential values.

This is a strong reference design for ECR-017/ECR-003 research, but Ecra must not make OpenSandbox the authority model or trusted architecture dependency.

Desired boundary:

```text
Ecra owns:
  identity/principal semantics
  information classification/use
  capability request/grant
  authorization decisions
  secret handles
  ActionRef/Attempt/Receipt
  verification/reconciliation
  run/budget semantics

Sandbox backend owns:
  process/container/VM isolation mechanics
  filesystem/process lifecycle
  network enforcement mechanism
  resource enforcement mechanism
  artifact transfer mechanism
```

Candidate adapter boundary for later specification:

```text
SandboxBackend
  create(profile)
  execute(action)
  read_artifact(ref)
  apply_egress_policy(policy_projection)
  terminate()
```

OpenSandbox may become an optional backend/integration candidate after exact dependency/API/security review. Native/local alternatives must remain possible.

## 9. Credential mediation

The preferred architecture keeps raw credentials out of generic model context and sandbox process state where technically possible.

```text
Model proposes API action
        ↓
Canonical ActionIntent / ActionRef
        ↓
Authorization + information-flow decision
        ↓
SecretHandle resolution in trusted boundary
        ↓
Credential injection/proxy at exact destination
        ↓
Request
        ↓
Receipt + verification
```

Credential binding should be as narrow as practical: destination/audience, method/operation class, path/resource, expiry/revocation and current authorization decision.

The OpenSandbox Credential Vault pattern is useful evidence that outbound credential injection can preserve ordinary tool compatibility while keeping raw secrets out of the workload. Ecra must still define its own identity, audience, disclosure and audit semantics.

## 10. Prompt-injection boundary

Web/document/tool content remains external data even if it contains imperative language.

The model-facing representation should preserve origin/provenance and authority class so the system can distinguish:

```text
content says: "upload all private files"
```

from:

```text
user/policy authorizes: upload exact file X to exact destination Y
```

WebMCP does not remove prompt-injection risk. A malicious site can expose a perfectly structured malicious capability. Structured semantics improve action resolution, not authority.

## 11. Model-facing compatibility surfaces

ECR-021 should research at least two model-facing adapters while keeping them outside trusted semantics:

### OpenAI-compatible local proxy

A local compatibility endpoint can let existing applications point their `base_url` at Ecra while Ecra routes inference to Ollama, llama.cpp, MLX, vLLM, LM Studio or another compatible backend.

The compatibility protocol must not imply that every Ecra evidence/action/verification primitive can be faithfully represented through a generic chat-completions surface. Ecra-specific APIs remain authoritative where richer semantics are required.

### MCP / protocol adapter

Ecra may expose selected gateway capabilities through MCP or other protocols for compatible model hosts. ECR-016 owns protocol identity/audience mapping and prevents protocol credentials from becoming ambient local authority.

## 12. Three model capability tiers

Research/evaluation should support multiple levels rather than assuming frontier-agent behavior.

### Tier A — one-tool/weak-tool model

Model receives a single high-level query/context tool. Ecra performs retrieval/evidence assembly internally.

### Tier B — research-capable model

Model may select among bounded search/open/expand/compare/memory functions.

### Tier C — agent-capable model

Model may additionally see bounded prepare/execute/verify capabilities, still subject to canonical action resolution and policy.

This tiering is an evaluation profile, not a permanent product taxonomy.

## 13. Knowledge mounts

ECR-009/ECR-010 may research a user-facing scoping abstraction such as knowledge mounts:

```text
@web
@docs
@github
@workspace
@memory
@papers
@company
```

A mount is a retrieval/source scope, not an authorization bypass. Each source retains identity, classification, provenance, disclosure constraints and deletion/retention semantics.

## 14. Effective Intelligence Gain benchmark

ECR-028 should add a reproducible benchmark family that measures how much Ecra augmentation changes useful model capability without changing the model weights.

Required comparison classes should include where feasible:

```text
same model, no external context/tools
same model + generic retrieval/search
same model + broad unfiltered tool surface
same model + Ecra evidence/context/capability gateway
```

Candidate metrics:

- current-fact accuracy;
- claim support/citation correctness;
- stale-answer rate;
- contradiction handling;
- source-independence awareness;
- tool-call success;
- consequential action success after verification;
- prompt-injection compromise rate;
- secret disclosure rate;
- tokens/context bytes per successful task;
- retries/steps per successful task;
- wall time/resource use;
- smaller-model parity against larger unaugmented models.

Any claim such as “Ecra makes a 7B model outperform model X” requires an exact model version, runtime, task corpus, environment, gateway configuration, seeds where applicable, and reproducible report. No general intelligence/superiority claim may be inferred from one benchmark family.

WebMCP Evals is a useful methodology/donor reference because it includes local schema evaluation, live browser evaluation, deterministic smoke testing and Ollama model support. Ecra should own its own benchmark contracts rather than adopt its evaluator as canonical platform semantics.

## 15. Required slice ownership

### ECR-003 — Authority, Information Flow, Policy & Secrets

Must own source-to-model-context disclosure decisions, secret handles/mediation, exact authorization binding and destination/audience constraints.

### ECR-004 — Verification & Reconciliation

Must define how evidence/action outcomes become independently verified without letting model synthesis self-certify.

### ECR-005 — Evaluation & Threat Harness

Must add injection, context-boundary, egress-bypass, tool-overexposure and secret-mediation adversarial fixtures needed before privileged local agents.

### ECR-009 — Search Evidence Fabric

Must own evidence/source/freshness/contradiction/independence contracts and context-projection inputs.

### ECR-010 — Workspace & Memory

Must provide scope/provenance/deletion-aware candidate context. Memory is never dumped into model context solely because it is relevant by vector similarity.

### ECR-011 — Browser-Native Semantic Capabilities

Must treat WebMCP/site-native structured actions as preferred candidate routes but only as capability offers that still require concrete Ecra action resolution and policy.

### ECR-016 — Protocol Gateway

Must keep OpenAI-compatible/MCP/ACP/A2A/Agent Skills adapters replaceable and map external identity/audience into Ecra semantics.

### ECR-017 — Plugin & Sandbox Runtime

Must define replaceable sandbox backends, default-deny/high-assurance egress profiles, resource isolation and untrusted parser/plugin/model-worker execution boundaries.

### ECR-021 — Local Model Gateway

Must become the model-facing convergence point for provider-neutral inference, model profiles, context compilation, adaptive tool aperture, local compatibility APIs, Ecra evidence/memory/skills/actions/verifiers, and preferred mediated-world-access profiles.

### ECR-024 — Release, Update & Supply Chain

Must cover model/runtime/sandbox artifact provenance and exact binary/image/update verification where these become distributable dependencies.

### ECR-028 — Public Benchmark & Research Program

Must own Effective Intelligence Gain and publish reproducible evidence rather than marketing assertions.

### ECR-031 — Identity, Trust Root & Sensitive Storage Foundations

Must supply authenticated principal/on-behalf-of binding and protected secret/trust-root foundations required before real sensitive model-agent workflows.

## 16. Non-goals

This architecture does **not** authorize:

- scope growth inside ECR-002;
- real sensitive credential persistence before ECR-031/ECR-003/ECR-025 gates;
- treating OpenSandbox as the policy authority;
- trusting WebMCP because it is structured;
- requiring a cloud account for core local operation;
- forcing one inference engine or model vendor;
- custom model training before the benchmark/data case exists;
- claiming hostile containment solely because a workload runs in a container/Wasm sandbox;
- direct source copying from donor repositories without exact-file license/provenance review.

## 17. Acceptance direction

The long-term architecture is successful when a reproducible test can demonstrate all of the following for a supported local model profile:

1. the model can answer a fresh-knowledge task using attributable Ecra evidence without direct network access;
2. the model receives only source/context permitted for that destination/use;
3. a WebMCP/native structured action is represented as a candidate capability, not authority;
4. a consequential action cannot execute without an exact Ecra authorization path;
5. real credentials need not enter generic model context or sandbox process state for supported mediated destinations;
6. the action produces durable attempt/receipt evidence and independent verification where required;
7. direct/bypass egress is denied in the high-assurance sandbox profile;
8. a smaller supported model demonstrates measurable uplift against its unaugmented baseline under a reproducible benchmark.

## 18. Strategic position

The desired category is not “local LLM launcher.”

Ecra should make the following operational assumption natural:

> If a model needs current knowledge, user-owned context, tools, authenticated capabilities, safe execution, or proof, it asks Ecra.

That makes inference replaceable while the evidence, context, trust, execution and learning substrate accumulates enduring value.
