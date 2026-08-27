# Ecra Initial Platform Threat Model

**Status:** CANONICAL_PLANNING  
**Date:** 2026-08-27  
**Updated after:** pre-implementation architecture review  
**Constitution:** v1.1.0

Each privileged/exposed slice MUST refine this model with implementation-specific assets, boundaries, abuse cases and tests.

## 1. Security Objective

Allow humans/agents to find, understand and act across digital systems without granting models, web content, plugins, repositories, browser extensions, external protocols or local processes ambient authority over the user's identity, browser, secrets, information or OS.

Security is defined by:

```text
authenticated identity where required
+ explicit authority
+ explicit source→sink information flow
+ provenance
+ bounded execution
+ exact side-effect/attempt semantics
+ durable receipts
+ independent verification
```

Prompt-injection detection is defense-in-depth, not the authority boundary.

## 2. High-Value Assets

- authenticated browser sessions/cookies;
- identity assertions, authorization decisions, approval records;
- credentials/API tokens/passkeys/secret handles;
- user files/workspace/private search context;
- browser history/bookmarks/tabs/containers;
- long-term memory and derived indexes/summaries;
- repositories/source/build credentials;
- terminal/process/filesystem/network authority;
- database/data/analytics sources;
- compiled skills and repair history;
- run ledger, ActionRefs/attempts/receipts/verification evidence;
- local trust-root/key material and protected storage metadata;
- update/signing keys/binaries/SBOM/provenance;
- plugin/registry/extension trust metadata;
- local/cloud model artifacts, provider credentials and context.

## 3. Adversaries / Failure Sources

- malicious/compromised page/site/document/email/repository;
- indirect/multi-step prompt injection;
- malicious/compromised tool/MCP/A2A/WebMCP server;
- malicious plugin/skill/connector/browser extension;
- malicious/local/cloud model output, model artifact or provider;
- poisoned memory/index/search result;
- untrusted local process under the same user session;
- untrusted repository build/test/install script;
- malicious parser input/archive/PDF/media;
- supply-chain compromise in Rust/Firefox/model/plugin dependencies;
- malicious update/build artifact;
- adversarial evidence intended to capture verifier;
- user mistake/approval fatigue/UI spoofing;
- crash/network/race faults producing UNKNOWN external outcome;
- resource-exhaustion/recursive-agent/tool loops.

“Local” is not equivalent to trusted.

### Security-boundary disclaimer

Ecra aims to isolate unrelated/unprivileged local clients and reduce accidental leakage. A fully compromised kernel/user account/debugger or attacker holding equivalent OS key-store authority is outside guaranteed containment unless a later slice states a narrower protected-hardware guarantee.

## 4. Trust Boundaries

### TB-1 Actor ↔ Authenticated Principal

Actor attribution does not prove security identity.

Threats:
- self-selected ActorId treated as authentication;
- confused on-behalf-of delegation;
- stale/revoked identity assertion;
- privilege inherited from a human merely because an agent acts in their run.

Controls:
- Actor/Principal/IdentityAssertion type separation;
- ECR-031 assertion/trust-root/revocation;
- ECR-003 authorization bound to validated identity context.

### TB-2 Human ↔ Agent

Threats: overreach, approval confusion/replay, concurrent mutation, spoofed approval UI.

Controls: exact ActionRef approval binding, trusted browser chrome, visible control ownership/authority, pause/takeover/hand-back and durable intervention events.

### TB-3 Information Source ↔ Information Sink

Permission to read does not imply permission to disclose.

Threats:
- private workspace → remote search/model;
- secret data → logs/telemetry/plugin;
- cross-origin browser exfiltration;
- derived summary silently declassified;
- memory/retrieval crossing workspace/model scope.

Controls:
- InformationClassification/lineage;
- InformationUse declaration;
- source-to-sink ECR-003 policy/declassification;
- redaction/minimization;
- egress benchmark fixtures.

### TB-4 Web Content ↔ Browser/Agent

Page content is untrusted even inside authenticated tabs.

Threats: prompt injection, hidden instructions, cross-origin chaining, policy/memory manipulation, spoofed Ecra UI.

Controls: Origin provenance, content-is-data invariant, origin/scoped authority, information-flow gate, trusted browser chrome, adversarial harness.

### TB-5 Browser Privileged Bridge ↔ Rust Core

Threats:
- arbitrary local RPC/superuser surface;
- page/extension reaching bridge;
- same-user rogue process;
- replay/forged messages;
- overly broad remote-debug command.

Controls owned ECR-007:
- OS endpoint ACL/peer identity;
- ephemeral channel/session binding;
- sequence/replay protection;
- narrow versioned method set;
- strict validation;
- capability + information-flow policy in core;
- no generic remote-debug endpoint by default;
- teardown/revocation/fuzzing.

### TB-6 Browser Permission/Presence Boundary

WebAuthn/passkeys, credential fill, clipboard, file chooser/upload, downloads, camera/mic/geolocation, notifications, payment handlers, fullscreen and external protocol/site permission prompts have special semantics.

Controls: ECR-003/ECR-006/ECR-008 permission broker; some operations human-presence-only/non-delegable by default.

### TB-7 Browser Extensions ↔ Page/Browser/Core

Broad-permission extension may observe/modify content across origins.

Controls: extension trust tiers, compatibility restrictions, labeling of extension-modified observations where material, no implicit extension authority over Ecra core.

### TB-8 Model ↔ Trusted Runtime

Model output is untrusted proposal.

Threats: fabricated success, tool bypass, secret solicitation, malformed structured output, endless planning.

Controls: schemas; concrete ActionRef before policy; independent authorization/verification; no raw secrets by default; budgets.

### TB-9 External Protocol ↔ Gateway

MCP/ACP/A2A callers/servers may be untrusted.

Threats: state exposure, token passthrough/confused deputy, identity/audience mismatch, capability escalation.

Controls: version-pinned protocol research; external auth mapped to Ecra identity assertions/capabilities; audience/resource binding; no token passthrough as ambient authority; least-authority views.

### TB-10 Plugin / Parser / Untrusted Code ↔ Core/OS

Includes plugins, dangerous file parsers, repo build scripts and native model loaders.

Controls: Wasm/process/VM tier as justified; explicit capabilities; time/memory/output/network/fs limits; advisory monitoring; sandbox assumed fallible.

### TB-11 Persistence ↔ Runtime

Threats: malformed/newer state, rollback, forged receipts, poisoned memory, recomputed hash chain, sensitive plaintext leakage.

Controls:
- strict versioned schemas;
- integrity chaining with accurately scoped claim;
- ECR-031 protected authenticity/storage when required;
- migration/rollback controls;
- sensitive real-state gate before persistence;
- provenance/classification validation.

### TB-12 Search/Retrieval ↔ Context/Memory

Threats: SEO/source manipulation, stale citations, source-copy laundering, changed sources, cross-workspace leakage, private query egress, parser attacks, silent memory promotion.

Controls: source identity/lineage/independence, captured hash/as-of, source policy, information-flow authorization before remote query, scoped retrieval, candidate-memory transition, parser/resource isolation.

### TB-13 Local Model Artifacts ↔ Runtime

Threats: malicious executable loader/custom code, tokenizer/template manipulation, unsafe native libraries, resource/GPU exhaustion, license/provenance ambiguity.

Controls: ECR-021 artifact provenance/hash/license/trust policy; ECR-017/024 containment/supply-chain controls; bounded runtime; no `trust_remote_code`-equivalent default without explicit authorization.

## 5. High-Priority Attack Classes

### A1 Indirect Prompt Injection
Invariant: content can influence reasoning but never directly grant/declassify/approve.

### A2 Cross-Origin / Cross-Provider Exfiltration
Invariant: source read authority and sink call/write authority do not imply source→sink disclosure.

### A3 Identity / On-Behalf-Of Confusion
Invariant: ActorId is not authenticated Principal; delegation/proof/revocation are explicit.

### A4 Secret Exfiltration
Invariant: mediated handles/use permissions replace raw secret bytes when possible.

### A5 Approval Confusion / Replay
Invariant: approval binds exact ActionDigest, identity/context/scope, expiry and one-use/policy semantics.

### A6 Side-Effect Duplication
Invariant: exact ActionAttempt identity + UNKNOWN/reconciliation/idempotency prevent blind retry.

### A7 False Completion / Verifier Capture
Invariant: VerificationReceipt and decision-grade evidence, not executor/model statement.

### A8 Memory Poisoning / Deletion Residue
Invariant: memory provenance/classification; deleted memory does not remain retrievable through derived projections.

### A9 Skill Poisoning / Authority Capture / Repair Drift
Invariant: skills store required capabilities/approvals, never captured live grants/tokens/secrets; repairs re-authorize/re-verify.

### A10 Plugin / Extension / Supply-Chain Compromise
Invariant: third-party code receives bounded capabilities and provenance/release controls.

### A11 Retrieval Scope / Query Leakage
Invariant: context/query egress is authorized before remote provider invocation.

### A12 Resource Exhaustion / Infinite Delegation
Invariant: wall-time/step/tool/model/token/cost/process/output/network/storage/delegation budgets terminate/suspend safely.

### A13 Trusted-UI Spoofing
Invariant: page content cannot impersonate native approval/agent-authority/takeover chrome.

### A14 Untrusted Repo / Parser Execution
Invariant: inspection does not imply executing build hooks/parsers with ambient host authority.

## 6. Side-Effect Model

Policy must treat these dimensions independently:

```text
MutationDomain: none / local / external / unknown
Reversibility: not_applicable / reversible / conditional / irreversible / unknown
Idempotency: natural / keyed / non-idempotent / unknown
Retry: safe / same-key / reconcile / never-blind
```

High-impact examples include payment, send/publish, delete, push/merge/release, permissions/security changes, account settings and external workflow invocation.

A browser click or shell command is not inherently low risk; policy evaluates canonical intent/resource/information flow.

## 7. Security Invariants

1. No ambient agent authority.
2. Actor != authenticated Principal.
3. External content is not authority/declassification.
4. CapabilityRequest != CapabilityGrant.
5. Missing scope != ANY.
6. Read authority != disclosure authority.
7. Lower-level route cannot bypass semantic policy.
8. Raw secrets minimized/mediated.
9. Approval binds exact ActionRef/context.
10. ActionIntent != ActionAttempt.
11. UNKNOWN is first-class; no blind retry.
12. Executor receipt != verification.
13. Fact has no independent verified-truth channel.
14. Memory/skill/plugin/model/protocol does not self-authorize.
15. Browser Containers assist storage/session partitioning but do not replace Ecra policy/sandboxing.
16. Privileged bridge is narrow/authenticated/replay-resistant.
17. Resource budgets are explicit for recursive/executable surfaces.
18. Stored local bytes are validated; hostile-tamper claims require protected trust root.
19. Security updates outrank feature velocity.

## 8. Required Security Evidence

As slices mature, retain locally inspectable records for:
- validated identity/assertion references;
- authorization/disclosure decisions;
- denied capability/egress attempts;
- origin transitions;
- approval/takeover events;
- ActionRef/Attempt/Receipt;
- VerificationReceipt;
- UNKNOWN/reconciliation;
- budget exhaustion/cancellation;
- plugin/sandbox violations;
- schema/integrity failures;
- benchmark attack results.

Local logging does not authorize remote telemetry.

## 9. Privacy Threats

- unnecessary browser/workspace context sent to cloud model/search/tool;
- provider-retained prompts/tool output;
- copied/retained source content beyond allowed policy;
- support/crash bundles containing sensitive data;
- hidden telemetry;
- derived memory/index surviving deletion;
- cross-workspace context assembly;
- optional sync metadata leakage.

Owners: ECR-003/ECR-009/ECR-010/ECR-021/ECR-022/ECR-025/ECR-027/ECR-029/ECR-031.

## 10. Open Research With Named Owners

- browser IPC/process topology and extension trust tiers → ECR-007;
- browser special permission/presence semantics → ECR-003/ECR-006/ECR-008;
- identity assertions/trust root/key management/sensitive storage → ECR-031;
- WebMCP principal/tool binding → ECR-011/ECR-016;
- OS-specific sandbox tiers → ECR-017;
- plugin/skill signing/registry reputation → ECR-023;
- encrypted multi-device sync/key recovery → ECR-022;
- team/multi-principal governance → future roadmap amendment.

No other slice may silently settle these as incidental implementation detail.

## 11. Update Triggers

Update this threat model for any new:
- privileged process/IPC;
- authenticated principal/secret class;
- persistent sensitive data;
- remote provider/caller/protocol version;
- plugin/native/parser/repository execution path;
- browser privileged patch/permission;
- local-model executable artifact path;
- cross-workspace/team/sync feature;
- consequential action;
- public security/privacy/tamper claim.
