# Ecra Pre-Implementation Architecture Review — 2026-08-27

**Review type:** Spec Kit analyze-style platform review before first implementation  
**Repository state reviewed:** `7a2f3aeabb2d5bf0ecdb39d65afc8f7956c03198` and planning artifacts on `main`  
**Implementation state:** no application/source implementation has started  
**Decision:** **DO_NOT_START_ECR_001_UNTIL_BLOCKERS_BELOW_ARE_REMEDIATED**

## Scope

This review checks the platform constitution, spec-of-specs roadmap, architecture, threat model, gap audit, risk register, benchmark matrix, donor ledger, AGENTS.md, and the complete ECR-001 Spec Kit package for inconsistencies, underspecification, missing security semantics, premature decisions, and future migration traps.

The review follows GitHub Spec Kit's `analyze` intent: identify inconsistencies, ambiguity, missing requirement/task coverage, and constitution conflicts before implementation. Remediation is performed as a separate planning change after this report.

## Summary

The current plan is unusually strong for a greenfield agent/browser platform. Its strongest properties are: one trusted domain model, request/grant separation, receipts vs verification, UNKNOWN side-effect handling, durable-run-first sequencing, stock Firefox before fork, verified Skill IR, and explicit benchmark/security gates.

However, several foundational semantics are still missing or ambiguous. If ECR-001 v1 were implemented unchanged, later policy/browser/search work would either need parallel security types or an early breaking migration. The highest-risk gap is that the plan controls **what an agent may do** more strongly than **what information an agent may disclose or combine**.

## Findings

| ID | Severity | Category | Finding | Why it matters | Required remediation |
|---|---|---|---|---|---|
| P-001 | **CRITICAL** | Information flow | No canonical data-classification / taint / disclosure model exists. | An agent may legitimately read private data under one scope and then disclose it through another allowed capability. Action permissions alone do not prevent cross-origin/workspace exfiltration. | Add constitutional information-flow/egress invariant; add ECR-001 data-label / information-flow references; ECR-003 must authorize disclosure/declassification separately from read access. |
| P-002 | **CRITICAL** | Identity | `Actor` is used as the capability principal, but authenticated identity, on-behalf-of binding, and principal assertions are deferred/undefined. | Runtime actor identity and security principal identity are not the same concept. This becomes dangerous for delegated agents and external protocols. | Add canonical identity/principal references to ECR-001 and explicit authentication/identity-assertion ownership before policy; add a dedicated trust-root/identity slice or bounded ECR-003 prerequisite. |
| P-003 | **CRITICAL** | Scope semantics | `Scope` uses optional strings/lists without normative empty/absent/ANY semantics. | In authorization models, omitted/empty fields can accidentally mean either “nothing” or “unrestricted”. A fail-closed system cannot leave this implicit. | Replace ambiguous option/list semantics with explicit scope-constraint variants; require explicit ANY and explicit NOT_APPLICABLE/NONE semantics. |
| P-004 | **CRITICAL** | Action binding | Receipts/approvals are described as binding exact actions, but the ECR-001 data model primarily references `ActionId`, not a security digest of the immutable action body. | If an action record is replaced or interpreted differently under the same ID, approvals/receipts can bind the wrong parameters. | Add `ActionDigest` / `ActionRef { id, digest }`; later authorization decisions and approvals bind the digest plus policy/grant context. |
| P-005 | **CRITICAL** | Retry/audit | No execution-attempt identity exists. Multiple attempts of one ActionIntent cannot be unambiguously distinguished in receipts. | Safe retry/reconciliation and duplicate-side-effect investigation require distinguishing intent from attempt. | Add `ActionAttemptId`; every receipt references exact action + digest + attempt. ECR-002 owns attempt lifecycle. |
| P-006 | **CRITICAL** | Verification truth | `Fact.trust_state = verified` and separate `VerificationReceipt` create two potential sources of verification truth. | State can drift: a Fact may say verified while verification records disagree or are revoked/superseded. | Make verification records authoritative. Remove `verified` as an independently mutable Fact truth state or require an explicit derived assessment linked to VerificationIds. |
| P-007 | **HIGH** | Side effects | `SideEffectClass` conflates mutation location and reversibility; local mutation has no reversible/irreversible dimension. | Deleting a local file can be more destructive than an external reversible update. Policy/retry/approval need orthogonal semantics. | Split effect domain/mutation from reversibility; add explicit high-impact/unknown semantics only where policy-neutral. |
| P-008 | **HIGH** | Typed identity | Security-sensitive `workspace_id`, `container_id`, `tab_id`, `session_id`, `task_id`, etc. are generic strings while other IDs are strong newtypes. | String confusion undermines the purpose of the trusted type layer. | Add strong ID/reference types for scope dimensions used in security decisions. |
| P-009 | **HIGH** | Resource identity | `ResourceRef.locator: string` can alias the same resource through multiple representations and is easy to misuse in policy comparisons. | URL normalization, path aliases/symlinks, case sensitivity, and provider-specific addressing can produce policy bypasses. | Make locator explicitly non-authoritative; add stable `ResourceId`/provider-resolved identity and require policy to compare canonical provider-backed identity/constraints. |
| P-010 | **HIGH** | Authorization TOCTOU | Plan mentions expiry and approval binding but not a canonical `AuthorizationDecision`/lease bound to action digest, policy version, grants, approval, evaluation context, and revocation state. | Authority may change between check and execution. | ECR-003 must emit an immutable authorization decision/lease consumed by executor; decision expires/revalidates and supports revocation. |
| P-011 | **HIGH** | Outcome naming | `CONFIRMED_SUCCESS` / `CONFIRMED_FAILURE` on `ActionReceipt` sound independently verified even though receipts explicitly are not verification. | Terminology can lead UI/callers to treat executor self-report as confirmation. | Rename execution outcomes to executor-observed/reported success/failure/unknown; reserve `VERIFIED` for VerificationReceipt. |
| P-012 | **HIGH** | Freshness | Fact freshness is `current/stale/unknown` without a normative temporal basis (`as_of`, source observed/published/effective time, assessment time). | Trusted search cannot explain why a fact is “current”. | Add temporal metadata/assessment references so freshness is inspectable and recalculable rather than a naked enum. |
| P-013 | **HIGH** | Ledger integrity | Roadmap says “tamper-evident ledger” before defining a trust root/key model. A plain hash chain is recomputable by an attacker who can rewrite the store. | This can create false security claims. | ECR-002 terminology must distinguish corruption/integrity chaining from adversarial authenticity; hostile tamper evidence requires protected MAC/signature/anchor semantics owned by identity/trust-root work. |
| P-014 | **HIGH** | Sensitive persistence | ECR-002 precedes final at-rest encryption/key-management research, yet portable run artifacts could later contain private context. | Sensitive data could be persisted before storage protection semantics exist. | ECR-002 must be fixture/non-secret only until trust-root/sensitive-storage gate closes; real authenticated browser data cannot be persisted before that gate. |
| P-015 | **HIGH** | Browser isolation | Firefox Containers are described in places as a security isolation boundary. They primarily partition site data/cookies; Ecra still must enforce agent authority and process/extension boundaries. | Overclaiming Containers as a sandbox creates unsafe assumptions. | Reword as browser storage/session partitioning plus an Ecra authority boundary; add tests proving Ecra policy, not Containers alone, blocks cross-container agent access. |
| P-016 | **HIGH** | Browser permissions | No dedicated plan item yet covers WebAuthn/passkeys, clipboard, file picker, downloads, notifications, camera/mic/geolocation, payment handlers, browser permission prompts, or user-presence requirements. | These are privileged browser capabilities with very different automation/security semantics. | Add a browser permission broker requirement to ECR-006/ECR-008/ECR-003; some capabilities must remain human-presence-only by policy. |
| P-017 | **HIGH** | Browser extensions | Extension compatibility is planned, but compromised/broad-permission extensions are an explicit adversary with no compatibility/trust tier. | An extension can observe/modify pages across normal browser boundaries and undermine agent assumptions. | ECR-007/008 must define extension trust tiers, compatibility restrictions, and how extension-produced/modified content is labeled. |
| P-018 | **HIGH** | Browser IPC | “Authenticated local IPC” is underspecified against same-user local processes/replay/credential theft. | A privileged bridge can become Ecra's superuser API. | ECR-007 research must cover OS endpoint ACL/peer identity, ephemeral channel binding, replay protection, message sequencing, strict methods, and no remote-debug endpoint by default. |
| P-019 | **HIGH** | Resource budgets | Token/cost metrics exist, but no mandatory runtime budget/circuit-breaker semantics for model calls, tool loops, wall time, network, processes, or output volume. | OWASP agent guidance treats unbounded consumption/recursive tool abuse as a core risk. | Add RunBudget/ResourceBudget ownership to ECR-002; ECR-005 tests termination under exhausted budgets. |
| P-020 | **HIGH** | Search privacy | Search provider abstraction lacks an explicit query/context egress gate. | A private workspace query may leak confidential text to a remote search provider before model policy is considered. | ECR-009/ECR-003/ECR-025 must authorize remote search/query disclosure using information-flow labels and redact/minimize by default. |
| P-021 | **HIGH** | Search trust | Source ranking does not yet explicitly model source independence, citation laundering, source snapshots/content hashes, or source-change detection. | Ten copied secondary pages are not ten independent corroborating sources. Citations can point to content that later changes. | ECR-009 must track source entity/lineage, captured observation hash/time, and independence signals; benchmark source laundering/copy cascades. |
| P-022 | **HIGH** | Memory lifecycle | Deletion/export requirements do not explicitly require removal from derived indexes, caches, embeddings, summaries, and generated memory projections. | “Delete memory” can leave retrievable derived copies. | ECR-010/ECR-029 must define deletion propagation and rebuildable/non-authoritative derived indexes. |
| P-023 | **HIGH** | Skill authority | Skill compilation does not explicitly forbid capturing grants/approvals/secrets from a demonstration as reusable authority. | A user approving one transaction must not teach a workflow permanent permission to transact. | ECR-012/013: skills store required capabilities/approval points, never live grants, approval tokens, secret bytes, or inherited ambient authority. |
| P-024 | **HIGH** | Local model security | ECR-021 focuses on capability uplift but not model artifact provenance, executable custom code, tokenizer/template supply chain, model license, GPU/resource isolation, or `trust_remote_code`-style risk. | “Local” does not mean trusted. Model artifacts can carry executable loaders or unsafe configuration. | Add local-model artifact/security contract and benchmark/run containment to ECR-021/ECR-024/ECR-017. |
| P-025 | **HIGH** | Developer execution | ECR-019 does not yet distinguish inspecting an untrusted repository from executing its build/test/install scripts. | `cargo test`, npm scripts, setup hooks, and build scripts can execute arbitrary code. | Developer workspace must have repository trust levels; untrusted project execution uses ECR-018/ECR-017 sandbox policy and structured process invocation. |
| P-026 | **HIGH** | Protocol identity | Protocol plan is version-neutral but latest MCP auth explicitly relies on audience/resource binding and forbids token passthrough; A2A likewise separates authn/authz. | Generic “MCP support” without identity/audience rules recreates confused-deputy risk. | ECR-016 research must pin protocol spec versions and map external authentication to Ecra identity assertions/capabilities without token passthrough. |
| P-027 | **MEDIUM** | Local adversary model | Threat model lists an untrusted local process/user but does not state what happens after full OS/user-account compromise. | Ecra cannot credibly defend secrets from an attacker with equivalent user/debug/keystore authority. | Document security boundary: defend against unprivileged/unrelated local clients and accidental exposure; fully compromised OS/account is out of guaranteed containment. |
| P-028 | **MEDIUM** | Actor identity | Actor ID collision across kinds is listed as an edge case but has no invariant. | Same ActorId reused for Human and Agent can break audit attribution. | Require ActorId to identify one immutable actor kind within a trust domain; conflicting definitions are invalid at run/store level. |
| P-029 | **MEDIUM** | Digest agility | `ContentDigest.algorithm` is an arbitrary string with no minimum cryptographic policy. | Weak digest choices could later be mistaken for security integrity. | Separate content-checksum metadata from security digest policy; define allowed security algorithms in owning slices and validate algorithm-specific encoding. |
| P-030 | **MEDIUM** | Evidence integrity | Verification may reference mutable external evidence without requiring a captured digest/snapshot when evidence materially determines a consequential result. | Evidence can change after the verification decision. | ECR-004/ECR-009 require immutable capture/hash or explicit “live external state as-of” metadata for decision-grade verification. |
| P-031 | **MEDIUM** | UI trust | Agent/approval UI visibility is required, but no anti-spoofing invariant distinguishes trusted browser chrome from web content. | A page can imitate Ecra approval/status UI and socially engineer users. | ECR-008 must define trusted/unspoofable control surface cues and restrict web content from invoking native approval affordances. |
| P-032 | **MEDIUM** | Special background effects | “Background agent tabs without focus theft” omits audio, notifications, popups, downloads, clipboard, permission prompts, and fullscreen effects. | Background tasks can still interrupt or deceive users. | Add background-agent effect restrictions and user-visible event policy to ECR-008. |
| P-033 | **MEDIUM** | Parsing attack surface | Search/file/repository ingestion plans do not explicitly sandbox dangerous document/archive/parser workloads. | Trusted search will ingest adversarial PDFs, archives, media, repos, and generated files. | ECR-009/ECR-017/ECR-019/ECR-027 must classify parser execution and enforce resource/sandbox limits. |
| P-034 | **MEDIUM** | Release provenance | “Reproducible builds” is stated broadly without separating Ecra-owned Rust artifacts from large Firefox-derived distribution reproducibility. | A universal bit-for-bit promise may be infeasible and later be quietly weakened. | ECR-024 must define artifact-specific reproducibility/SLSA/provenance targets and report what is and is not reproducible. |
| P-035 | **MEDIUM** | Spec Kit quality | ECR-001 checklist currently says PASS, but this review found multiple missing security semantics; status therefore became stale. | Starting implementation based on a stale PASS violates Spec Kit's purpose. | Mark ECR-001 planning as REWORK/BLOCKED until spec→plan→tasks/checklist are reconciled, then rerun analysis. |

## External Evidence That Strengthens These Findings

- NIST's 2026 agent identity/authorization work explicitly calls out agent identification, authentication, key lifecycle/revocation, least privilege, on-behalf-of delegation, binding human and agent identities, tamper-verifiable audit, and sensitivity of aggregated data.
- OWASP's current AI Agent Security guidance explicitly includes tool/privilege abuse, data exfiltration, memory poisoning, high-impact actions, approval manipulation, sensitive data exposure, supply-chain risk, and unbounded/recursive consumption.
- Firefox Containers partition site data/cookies and are useful isolation primitives, but Ecra must not treat them as the whole agent/core sandbox.
- MCP 2026-07-28 hardens authorization around issuer validation, resource/audience binding and credentials; token passthrough is forbidden because of confused-deputy risk.
- Spec Kit's official `analyze` workflow requires blocking critical spec/plan/task inconsistencies before implementation and rerunning downstream artifacts after spec changes.

## What Is Strong and Should Not Be Reopened

The review found no reason to reverse these current decisions:

- Rust as the trusted-core language.
- one zero-I/O `ecra-core` crate for ECR-001;
- external protocols as adapters;
- stock Firefox/BiDi prototype before a maintained Firefox distribution;
- Firefox as preferred daily-browser direction, subject to ECR-006 evidence;
- no deep Zen fork as default;
- request/grant separation;
- external content is not authority;
- UNKNOWN external action outcome;
- receipt vs verification separation;
- local-first/model-independent architecture;
- Skill IR rather than recorded-click macros;
- benchmark-before-superlative policy;
- no custom model training before verified data/evaluation evidence.

## Required Before First Code

1. Amend constitution for information-flow/egress and authenticated-principal semantics.
2. Rework ECR-001 domain v1 for explicit scope semantics, typed scope/resource/action-attempt IDs, action digest binding, data classification/flow references, orthogonal side-effect semantics, and single-source verification truth.
3. Add/assign identity/trust-root work before privileged browser/persistence of real sensitive data.
4. Correct Containers/ledger terminology so planning does not promise security properties primitives do not provide alone.
5. Add resource budgets, browser permission broker, search-query egress, skill non-transferable authority, and local-model artifact security to the owning slices.
6. Regenerate/update ECR-001 research/data model/contract/plan/tasks/checklist.
7. Run a fresh analyze-style consistency review. Only then return ECR-001 to `TASKS_READY`.

## Review Decision

**Current state at the time of this report: `PLANNING_REWORK_REQUIRED`.**

No implementation should start from the pre-review ECR-001 v1 contract.
