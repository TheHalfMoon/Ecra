# Ecra Platform Architecture

**Status:** CANONICAL_PLANNING  
**Date:** 2026-08-27  
**Governed by:** `.specify/memory/constitution.md` v1.1.0

This document defines stable platform boundaries. It intentionally does not freeze implementation internals owned by later Spec Kit slices.

## 1. Architectural Thesis

Ecra is one trusted context/execution substrate exposed through multiple human and machine surfaces.

```text
Humans / Agents / Local Models / Cloud Models / IDEs / Apps
                           │
                           ▼
                  Ecra Surfaces/Adapters
                           │
                           ▼
                 Trusted Rust Substrate
                           │
          ┌────────────────┼────────────────┐
          ▼                ▼                ▼
      Browser/Web       Local OS/Data    Remote Services
```

The substrate owns canonical semantics for attribution, authenticated-principal references, authority, information flow, provenance, durable execution, receipts, verification, memory trust and reusable skills.

## 2. Core Layering

### Layer A — Trusted Domain Kernel (ECR-001)

Zero-I/O Rust value objects:

```text
Actor vs Principal/IdentityAssertion references
Origin / Resource identity / explicit Scope
InformationClassification / InformationUse
CapabilityRequest / CapabilityGrant
Observation / Fact / Evidence / Artifact
ActionIntent / ActionDigest / ActionRef / ActionAttemptRef
Effect / Idempotency / Retry
ActionReceipt / VerificationReceipt
Version / Canonicalization / Errors
```

No browser/model/database/protocol/policy/authentication dependency.

### Layer B — Durable Execution (ECR-002)

```text
Run state machine
ActionAttempt lifecycle
Append-only integrity-chained events
Cancellation / reconciliation hooks
Resource/time/model/tool/output budgets
Portable synthetic/non-sensitive .ecra run artifact first
```

A hash chain provides integrity/corruption evidence under its stated assumptions; hostile tamper resistance is not claimed without a protected trust anchor.

### Layer C — Identity / Trust Root (ECR-031)

```text
IdentityAssertion validation
Actor ↔ Principal / on-behalf-of binding
Local trust root
Key issue/rotation/revocation
Protected authenticity/MAC/signature envelope primitives
Sensitive local-storage protection contracts
```

ActorId alone never authenticates a principal.

### Layer D — Authority / Information Flow / Secrets (ECR-003)

```text
Capability narrowing/intersection
Source-to-sink information-flow policy
Declassification decisions
Human approval binding
Secret handles/mediation
Immutable AuthorizationDecision / execution lease
Revocation / expiry / policy-version binding
```

An executor consumes an authorization decision bound to a concrete ActionRef; it never treats the model proposal or capability request itself as authority.

### Layer E — Verification / Reconciliation (ECR-004)

```text
Verifier framework
Evidence sufficiency
Critical-point verification
UNKNOWN reconciliation
Decision-grade immutable evidence rules
VerificationReceipt aggregation/views
```

VerificationReceipt is the authoritative verification record. Fact/UI/receipt/model output does not maintain a second verified-truth channel.

### Layer F — Evaluation / Threat Harness (ECR-005)

Security, information-flow, durability, budget-exhaustion, verification and later browser benchmark fixtures.

### Layer G — Capability Providers

```text
Firefox/WebDriver BiDi/privileged bridge
WebMCP/site-native capabilities
Search providers
Terminal/process/filesystem/data providers
MCP/ACP/A2A adapters
Plugins
Cloud/local model providers
```

Providers translate native APIs into Ecra actions/evidence. They never grant themselves authority and never redefine trusted domain types.

### Layer H — Search / Context / Workspace / Memory

One evidence + information-flow fabric over multiple indexes/providers. Retrieval remains scoped/classified, remote provider queries are disclosure boundaries, and memory cannot self-authorize.

### Layer I — Skill System

Skill IR/Compiler/Replay/Repair uses fresh policy and verification on execution. A Skill stores authority **requirements**, never captured live grants, approvals, secrets or session authority.

### Layer J — Human Product / External Gateway

Browser/Search/Workspace/Terminal/Developer/Data UI and MCP/ACP/A2A/SDK surfaces consume the same trusted objects.

## 3. Dependency Direction

```text
Human UI / External Adapters / Providers
               ↓
Application services / Skill runtime
               ↓
Run / Identity / Policy / Verification / Ledger
               ↓
Trusted Domain Kernel
```

Forbidden examples:
- `ecra-core` imports Firefox/MCP/LLM/Cedar/SQLite types;
- browser UI state becomes run truth;
- protocol token becomes CapabilityGrant;
- search result becomes canonical Fact without provenance mapping;
- plugin/model output becomes authorization;
- Resource locator string is compared as canonical security identity;
- Fact carries independent verified truth in parallel with VerificationReceipt.

## 4. Two Independent Security Questions

Every privileged path must answer both:

### 4.1 May this principal perform this operation on this resource?

Capability/authorization question.

### 4.2 May this information flow from these sources into this destination/use?

Information-flow/egress question.

Example:

```text
Agent may READ private workspace file        = yes
Agent may CALL remote search provider        = yes
Private file text → remote provider query    = NOT implied
```

This distinction applies to model context, search queries, plugins, logs, telemetry, memory, MCP/A2A calls, terminal/network actions and browser origins.

## 5. Trust / Process Zones

### Browser Zone

Firefox/Gecko + Ecra browser chrome/privileged integration. Browser/web content is untrusted relative to trusted core.

Firefox Containers are useful **site-data/session partitions** (cookies/storage) and an input to Ecra scope; they are not the entire Ecra sandbox/authorization boundary.

### Trusted Core Zone

Rust run/identity/policy/verification/secret mediation/capability routing. Smaller and more privileged than models/plugins.

### Model Zone

Cloud and local model processes are untrusted proposal generators. “Local” is not “trusted”; model artifacts/loaders/configuration can be hostile.

### Plugin / Parser / Untrusted-Code Zone

Third-party plugins, dangerous parsers, untrusted repository build scripts and other executable content require capability/resource-limited sandbox/process isolation appropriate to risk.

### Protocol Zone

MCP/ACP/A2A/WebMCP adapters. External authentication maps into Ecra identity assertions and capabilities; tokens are not blindly passed through between resources/services.

### Storage Zone

Local bytes are untrusted on read and version/integrity validated. Real sensitive persistence waits for relevant ECR-031/ECR-003/ECR-025 contracts.

### Local OS Adversary Boundary

Ecra aims to defend against unrelated/unprivileged local clients and accidental leakage through narrow OS endpoints/ACLs. A fully compromised user account/kernel/debugger or attacker holding equivalent key-store authority is outside guaranteed containment; threat claims must state that boundary.

## 6. Concrete Action Resolution Before Policy

The capability router is **not** an authorization bypass.

```text
Intent
  ↓
Candidate semantic route resolution
  ├ native API
  ├ WebMCP
  ├ compiled Skill
  ├ AX/DOM
  ├ WebDriver BiDi
  ├ vision/coordinates
  └ future desktop
  ↓
Canonical ActionIntent + ActionDigest
  ↓
Identity + capability + information-flow policy
  ↓
AuthorizationDecision/lease
  ↓
Execution attempt
  ↓
Receipt
  ↓
Verification
```

Route choice considers semantic fidelity, risk, available evidence, cost/latency and state; lower-level execution never bypasses a denied semantic operation.

## 7. Browser Architecture Direction

```text
Stock Firefox + WebDriver BiDi
        ↓
Prove trusted execution model
        ↓
Research browser permission broker + bridge threat model
        ↓
Narrow privileged Ecra bridge
        ↓
Firefox-derived Ecra distribution
        ↓
Human / Agent / Shared browsing UX
```

### Privileged bridge requirements (owned ECR-007)

Research/contract must cover:
- OS endpoint ACL/peer identity;
- ephemeral channel/session binding;
- anti-replay/message sequencing;
- least-method surface;
- strict input validation/versioning;
- no generic remote-debug/superuser endpoint by default;
- revocation/connection teardown;
- logs that exclude secret payloads by default.

### Browser permission broker (ECR-006/ECR-008/ECR-003)

Agent use of these is not ordinary click authority:
- WebAuthn/passkeys/user verification;
- password/credential fill;
- clipboard;
- file chooser/upload/download;
- notifications/popups;
- camera/microphone/geolocation;
- payment handlers;
- fullscreen/external protocol handlers;
- browser/site permission prompts.

Some capabilities may require current human presence and remain non-delegable by default.

### Extensions

ECR-007/ECR-008 define extension compatibility **and trust tiers**. Extension-modified content cannot be assumed equivalent to pristine web origin content; broad-permission extensions are a threat input.

### Trusted chrome / anti-spoofing

Approval, agent-active state, authority and takeover controls must live in browser chrome or another unspoofable trusted surface distinguishable from page content.

Background agent tabs must additionally restrict/mediate focus, audio, fullscreen, permission prompts, notifications, popups, downloads and clipboard effects.

## 8. Search / Trusted Knowledge Architecture

```text
Provider/local source
  ↓
Source identity + lineage/independence
  ↓
Captured observation timestamp/hash/as-of metadata
  ↓
Classification + provider-egress decision
  ↓
Evidence/Fact mapping
  ↓
Quality/freshness/contradiction/source-copy analysis
  ↓
Scoped retrieval/context assembly
  ↓
Synthesis
  ↓
Claim→evidence map
```

Remote search is an information disclosure. Query rewriting/context enrichment must pass egress policy before provider call.

Search must model copied/citation-laundered sources; N pages repeating one upstream claim are not N independent confirmations.

Dangerous document/archive/repository parsing belongs behind parser/resource isolation as appropriate.

## 9. Memory Architecture

```text
Observation/Retrieval/Model output
       ↓
Candidate memory + classification/provenance
       ↓
Policy/trust transition
       ↓
Accepted MemoryRecord
       ↓
Derived indexes/summaries/embeddings (rebuildable projections)
       ↓
aging / contradiction / deletion
```

Deletion must propagate so derived retrievable projections do not silently preserve deleted memory. Derived indexes should be rebuildable/non-authoritative where practical.

## 10. Skill Architecture

Skill IR contains:

```text
Intent
Stages
Typed inputs/outputs
Artifact/data reads/writes
Information-flow requirements
Preconditions/postconditions
Capability requirements
Effect/idempotency/retry semantics
Approval points
Verifiers
Assumptions
Repair boundaries
```

It never contains reusable approval tokens, capability grants, secret bytes or authenticated session authority captured from demonstration.

Replay resolves current ActionIntent → fresh policy/authorization each time. Repair cannot silently weaken policy or classifications.

## 11. Terminal / Developer / Data

All reuse the same trust model.

Untrusted repository inspection is distinct from executing repository code. Build scripts/package install hooks/test commands can execute arbitrary code and use ECR-018/ECR-017 isolation/authority.

Data claims maintain claim→query/transformation→source lineage plus information-flow restrictions on remote tools/models.

## 12. Local Model Gateway

Local model adapter research must include:
- artifact/source/license/provenance;
- hashes/signatures where available;
- tokenizer/chat-template provenance;
- executable custom-loader / `trust_remote_code`-style risks;
- native-library/GPU/resource isolation;
- model/output classifications;
- prompt/context egress even when another local/remote component is used.

Custom training remains deferred until data/eval evidence justifies it.

## 13. Persistence Direction

- local-first;
- versioned schemas/migrations;
- append-only run truth;
- content-addressed large artifacts where appropriate;
- portable exports;
- integrity chaining without overstating hostile tamper resistance;
- protected sensitive state only after trust-root/storage design;
- real secret values avoided in generic run artifacts when handles/references suffice.

## 14. Runtime Resource Budgets

ECR-002 defines RunBudget/ResourceBudget semantics; providers map concrete usage into them.

Relevant dimensions may include:
- wall time;
- run/step/tool/model call counts;
- tokens/cost;
- process lifetime/count;
- stdout/stderr/output bytes;
- network requests/bytes;
- storage/artifact bytes;
- recursion/delegation depth.

Budget exhaustion suspends/fails safely and never grants more authority or blindly retries unknown side effects.

## 15. Architecture Fitness Functions

Plans/CI increasingly enforce:
- `ecra-core` zero-I/O/no unsafe;
- Actor != Principal;
- no implicit scope wildcard;
- no provider bypass of action + egress policy;
- every consequential attempt has exact ActionRef/Attempt/Receipt;
- verification only through verifier records;
- real remote disclosures are classified/authorized;
- no protocol SDK/type leaks into domain kernel;
- persisted schema changes have migration fixtures;
- browser patch/bridge/permission inventory stays bounded;
- no sensitive state persisted before owning protection gates;
- extension/plugin/model/repo/parser untrusted-code boundaries are explicit;
- ordinary human browsing remains functional without AI/model dependency.

## 16. Architecture Change Rule

Changing dependency direction, trust zones, canonical domain ownership, information-flow model, identity/principal separation, ActionRef/attempt/receipt binding, verification ownership or browser foundation requires:

1. decision-log update;
2. affected roadmap dependency review;
3. constitution check;
4. threat/gap/risk/benchmark update;
5. migration/compatibility analysis if any persisted/public contract exists.
