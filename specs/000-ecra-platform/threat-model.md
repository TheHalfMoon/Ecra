# Ecra Initial Platform Threat Model

**Status:** CANONICAL_PLANNING  
**Date:** 2026-08-27

This is the initial platform-level threat model. Each privileged or externally exposed slice MUST refine it with implementation-specific assets, trust boundaries, attack paths, and tests.

## 1. Security Objective

Ecra should allow humans and agents to find, understand, and act across digital systems without granting agents, models, web content, plugins, or external protocols ambient authority over the user's browser, secrets, data, or operating system.

Security is defined primarily by **authority containment, provenance, explicit side-effect semantics, durable evidence, and independent verification**—not by prompt wording or attack detection alone.

## 2. Assets

High-value assets include:

- authenticated browser sessions/cookies;
- credentials, API keys, tokens, passkeys and secret handles;
- user files and workspace data;
- browser history/bookmarks/tabs;
- private search/retrieval context;
- long-term memory and accepted facts;
- repositories/source code;
- terminal/process/filesystem authority;
- databases and analytical data;
- approval decisions and policy configuration;
- compiled skills/workflows;
- run ledger and receipts;
- update/signing keys and distributed binaries;
- plugin/registry trust metadata;
- model/provider credentials and outputs.

## 3. Potential Adversaries / Failure Sources

- malicious website/page content;
- compromised legitimate website;
- indirect prompt injection in page/document/email/repo/tool content;
- malicious or compromised MCP/A2A/tool server;
- malicious plugin/skill/connector;
- malicious model output or compromised model provider;
- accidental model/tool hallucination;
- malicious retrieved memory or poisoned index;
- local untrusted process/user on the same machine;
- compromised browser extension;
- supply-chain compromise in Rust/npm/Firefox dependencies;
- malicious update/build artifact;
- adversarial data intended to confuse verifier/search/memory;
- user mistake or ambiguous approval;
- race/crash/network fault producing unknown external outcomes.

Ecra does not assume that “local” equals trusted.

## 4. Trust Boundaries

### TB-1 Human ↔ Agent

Agent is not equivalent to user identity. Human control/approval is explicit and scoped.

Threats:
- agent overreach;
- misleading approval request;
- stale blanket approval reuse;
- human/agent concurrent mutation.

Controls:
- Actor identity;
- exact-action approval binding;
- control ownership;
- visible authority;
- durable intervention events.

### TB-2 Web Content ↔ Browser/Agent

Page content is untrusted data even when rendered inside an authenticated tab.

Threats:
- prompt injection;
- data exfiltration;
- cross-origin instruction chaining;
- hidden content/invisible instructions;
- content attempting to modify memory or policy.

Controls:
- Origin provenance;
- content-is-data invariant;
- capability re-evaluation;
- agent same-origin policy concepts;
- context scoping;
- adversarial harness.

### TB-3 Browser Privileged Bridge ↔ Rust Core

Browser integration is high privilege.

Threats:
- arbitrary local RPC execution;
- page/extension reaching privileged bridge;
- confused deputy;
- bridge method surface expansion;
- unauthenticated/forged calls.

Controls:
- narrow authenticated local IPC;
- explicit operation types;
- capability/policy check in Rust core;
- fuzzing/strict parsing;
- no generic “execute arbitrary browser command” privileged API where avoidable.

### TB-4 Model ↔ Trusted Runtime

Model output is untrusted proposal.

Threats:
- fabricated success;
- tool-call injection;
- secret solicitation;
- bypass via lower-level tools;
- malicious structured output.

Controls:
- strict schemas;
- policy independent of model;
- capability router;
- receipt/verification separation;
- no raw secrets by default.

### TB-5 External Protocol ↔ Gateway

MCP/ACP/A2A/Agent Skills callers/servers may be untrusted.

Threats:
- broad state exposure;
- tool schema confusion;
- server prompt injection;
- capability escalation;
- confused-deputy delegation.

Controls:
- explicit gateway grants;
- adapters outside trusted domain;
- per-provider identity/origin;
- request/grant distinction;
- least-authority resource views.

### TB-6 Plugin ↔ Core/OS

Threats:
- sandbox escape;
- denial of service;
- network/filesystem exfiltration;
- secret access;
- malicious updates.

Controls:
- Wasm/process isolation;
- explicit capability manifest;
- resource/time/output limits;
- signing/reputation later;
- advisory monitoring;
- treat sandbox as defense layer, not proof of safety.

### TB-7 Persistence ↔ Runtime

Stored data can be stale, malformed, maliciously modified, or from a newer incompatible version.

Threats:
- deserialization abuse;
- rollback of policy/memory state;
- forged receipts;
- schema confusion;
- poisoned memory.

Controls:
- versioned strict schemas;
- integrity/tamper evidence for run ledger;
- migrations;
- provenance/trust state;
- signed/encrypted sync only after dedicated design.

### TB-8 Search/Retrieval ↔ Context/Memory

Threats:
- malicious SEO/source manipulation;
- stale results;
- citation laundering;
- cross-workspace leakage;
- retrieved prompt injection;
- silent durable-memory promotion.

Controls:
- source identity/freshness;
- evidence contracts;
- capability/scope-aware retrieval;
- candidate-memory boundary;
- contradiction handling;
- source/content policy.

## 5. High-Priority Attack Classes

### A1 — Indirect Prompt Injection

Attacker places instructions in external content intended for the model/agent.

Security invariant:
> Content may affect reasoning but cannot directly grant authority or bypass policy.

Required tests:
- visible/hidden page injection;
- injected tool output;
- document/repo injection;
- multi-page/multi-step instruction chains;
- attempts to persist malicious memory.

Owners: ECR-003/ECR-005/ECR-006/ECR-010/ECR-028.

### A2 — Cross-Origin Exfiltration

Agent reads private origin A and is induced to send data to origin B.

Invariant:
> Access to information at one origin does not automatically authorize disclosure/action at another origin.

Owners: ECR-003/ECR-005/ECR-006/ECR-008/ECR-011.

### A3 — Secret Exfiltration

Agent/plugin/content attempts to obtain raw credential material.

Invariant:
> Where mediated use is possible, model/plugin receives a handle/use permission, not raw secret bytes.

Owners: ECR-003/ECR-025.

### A4 — Approval Confusion / Replay

An approval for one action is reused for a different action, changed parameters, origin, or later time.

Invariant:
> Approval binds to exact/narrow action digest/scope, issuer and expiry/one-use semantics.

Owner: ECR-003.

### A5 — Side-Effect Duplication

Crash/network ambiguity causes repeated purchase/send/publish/delete/etc.

Invariant:
> UNKNOWN persists and unsafe retries require reconciliation or same idempotency key where valid.

Owners: ECR-002/ECR-004.

### A6 — False Completion / Verifier Capture

Actor/executor says “done” or manipulated evidence convinces verifier incorrectly.

Invariant:
> Verification is independently represented and prefers deterministic external evidence.

Owners: ECR-004/ECR-005/ECR-028.

### A7 — Memory Poisoning

Untrusted content becomes durable trusted memory or overrides corrected facts.

Invariant:
> Memory has provenance/trust/freshness and cannot self-authorize.

Owner: ECR-010.

### A8 — Skill Poisoning / Repair Drift

A learned/imported skill contains malicious capabilities or repair broadens behavior silently.

Invariant:
> Skill validation includes capabilities/origins/side effects/verifiers and repaired versions re-run policy/verification gates.

Owners: ECR-012–ECR-015/ECR-023.

### A9 — Plugin / Supply-Chain Compromise

Malicious dependency/plugin/update executes with excessive authority.

Invariant:
> Third-party code receives bounded capabilities and releases are reproducible/signed with tracked provenance.

Owners: ECR-017/ECR-023/ECR-024.

### A10 — Retrieval Scope Leakage

Private workspace A data appears in context/search for workspace B or an unauthorized model/provider.

Invariant:
> Retrieval/context assembly is authority- and workspace-scoped before model invocation.

Owners: ECR-003/ECR-009/ECR-010/ECR-016.

## 6. Side-Effect Risk Classes

Canonical action semantics are owned by ECR-001/ECR-004. Security policy should consider at least:

```text
read-only
local mutation
reversible external mutation
irreversible/destructive external mutation
unknown
```

High-impact examples:
- payment/purchase;
- send/publish/post;
- delete;
- push/merge/release;
- permissions/access control;
- account/security setting changes;
- external workflow invocation.

“Low-level” browser click or shell command is not inherently low-risk; policy evaluates intended/targeted capability and context.

## 7. Security Invariants

1. No ambient agent authority.
2. External content is not authority.
3. CapabilityRequest is not CapabilityGrant.
4. Lower-level tool choice cannot bypass policy.
5. Raw secret material is minimized/mediated.
6. Cross-origin disclosure/action is independently authorized.
7. Consequential action has durable receipt.
8. UNKNOWN is not silently retried or coerced.
9. Executor receipt is not verification.
10. Memory/skills/plugins do not self-authorize.
11. External protocols do not imply local trust.
12. Model/provider compromise should not automatically become core-policy compromise.
13. Browser compromise should not automatically grant arbitrary OS/core/plugin authority by design.
14. Stored data is validated/versioned on load.
15. Security-critical updates outrank feature velocity.

## 8. Verification and Detection

Preventive controls are preferred, but Ecra also needs evidence for detection:

- denied capability attempts;
- origin transitions;
- policy decisions;
- approval events;
- action receipts;
- verification outcomes;
- UNKNOWN/reconciliation states;
- plugin resource/capability violations;
- schema/integrity failures;
- explicit security benchmark results.

Logging these events locally does not authorize remote telemetry.

## 9. Privacy Threats

Privacy risk exists even without an attacker:

- sending unnecessary browser/workspace context to cloud models;
- retaining excessive search/page content;
- hidden telemetry;
- diagnostic bundles containing secrets/PII;
- sync exposing metadata;
- model providers retaining prompts/tool results;
- context assembly crossing workspace boundaries.

Owners: ECR-009/ECR-010/ECR-021/ECR-022/ECR-025/ECR-027.

## 10. Open Research Requiring Dedicated Slice Resolution

- exact browser privileged IPC/authentication/process topology (ECR-007);
- at-rest encryption/key management for sensitive local state (ECR-003/ECR-025);
- WebMCP origin/principal/tool trust-binding semantics (ECR-011);
- sandbox composition on Windows/macOS/Linux and Wasm/process/VM tiers (ECR-017);
- plugin/skill signing and registry reputation model (ECR-023);
- encrypted multi-device sync/key recovery (ECR-022);
- team/multi-principal workspace authority model (future roadmap amendment).

No implementation may silently choose these as incidental details in another slice.

## 11. Threat-Model Update Triggers

Update this document when any change introduces:

- a new privileged process/IPC path;
- new secret class;
- new persistent user data class;
- new protocol/remote caller;
- new plugin/native-code execution path;
- new browser patch exposing privileged behavior;
- new cross-workspace/team sharing;
- new sync/cloud storage;
- new side-effecting capability;
- new public security/privacy claim.
