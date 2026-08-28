# Ecra Platform Decision Log

**Status:** CANONICAL_PLANNING_V2  
**Date:** 2026-08-27  
**Updated:** 2026-08-28 — local-model world gateway decisions

This is a compact architecture decision index. It is not a substitute for owning specs/research. Once referenced by implementation, a change requires evidence, affected-slice review and compatibility/migration analysis where applicable.

| ID | Decision | Status | Rationale | Revisit trigger |
|---|---|---|---|---|
| D-001 | Ecra is a gateway/trust-execution platform, not unrelated AI products | ACCEPTED | One flywheel across Browser/Search/Workspace/Terminal/Data/Models | evidence shared substrate materially harms product fit |
| D-002 | Spec Kit spec-of-specs with immutable ECR IDs | ACCEPTED | platform too large for one feature cycle | governance replacement only |
| D-003 | Rust is trusted-core language | ACCEPTED | safety/performance/auditability | specific trusted requirement cannot be met safely |
| D-004 | Trusted domain kernel is zero-I/O and provider-independent | ACCEPTED | prevents architecture coupling | constitution amendment |
| D-005 | Firefox/Gecko preferred daily-browser foundation | ACCEPTED_DIRECTION | mature browser/standards/Containers/non-Chromium | ECR-006/007 evidence shows untenable blocker |
| D-006 | No permanent deep Zen downstream fork by default | ACCEPTED | avoid Mozilla→Zen→Ecra maintenance chain | demonstrably cheaper sustainable path |
| D-007 | Stock Firefox/WebDriver BiDi before maintained distribution | ACCEPTED | de-risk execution semantics first | privileged dependency truly required earlier |
| D-008 | Rustwright is Chromium/headless/ergonomics donor, not Firefox internal controller | ACCEPTED | Chromium-only today | relevant capability/foundation changes |
| D-009 | Human/Agent/System are explicit actors | ACCEPTED | attribution/control | constitution amendment |
| D-010 | External content is data/context, not self-authority | ACCEPTED | prompt injection not solved by prompt wording | constitution amendment |
| D-011 | CapabilityRequest != CapabilityGrant | ACCEPTED | prevent request/authority confusion | constitution amendment |
| D-012 | Model proposes; policy authorizes; runtime executes; verifier confirms | ACCEPTED | security separation | constitution amendment |
| D-013 | ActionReceipt != VerificationReceipt | ACCEPTED | executor self-report not proof | constitution amendment |
| D-014 | UNKNOWN is first-class external outcome | ACCEPTED | honest crash/network ambiguity | constitution amendment |
| D-015 | Effect/idempotency/retry semantics explicit before execution | ACCEPTED | safe resume/retry | constitution amendment |
| D-016 | One evidence/provenance contract across search providers | ACCEPTED | prevent source-quality loss in synthesis | ECR-009 may refine, not remove |
| D-017 | Memory is source-aware context and cannot self-authorize | ACCEPTED | memory poisoning/permission drift | constitution amendment |
| D-018 | Ecra Skill is typed executable IR, not recorded clicks/Agent Skill text | ACCEPTED | deterministic replay/verification/repair | syntax may evolve, semantics not silently removed |
| D-019 | Human and verified-agent demos target same Skill IR | ACCEPTED_DIRECTION | converge teaching/learning | experiments prove irreducible separate semantics |
| D-020 | Replay targets zero model calls when compatibility holds | ACCEPTED_GOAL | economic/reliability moat | defined class proves unavoidable reasoning |
| D-021 | Repair localized/versioned by default | ACCEPTED_GOAL | auditability/efficiency | benchmark-defined class favors re-exploration |
| D-022 | MCP/ACP/A2A/Agent Skills/WebMCP are adapters, not internal model | ACCEPTED | standards evolve | constitution amendment |
| D-023 | Plugins use capability-isolated sandbox/process boundaries | ACCEPTED_DIRECTION | no ambient third-party authority | exact tier varies ECR-017 |
| D-024 | Local models first-class; no custom training before data/eval moat | ACCEPTED | runtime/context/skills nearer leverage | verified corpus shows model bottleneck |
| D-025 | Useful local-first core; cloud additive | ACCEPTED | privacy/resilience/model independence | constitution amendment |
| D-026 | No hidden telemetry | ACCEPTED | trust/privacy | constitution amendment |
| D-027 | User runs/memory/skills should be exportable | ACCEPTED | anti-lock-in/ecosystem trust | exact formats ECR-029 |
| D-028 | Superiority claims require reproducible evidence | ACCEPTED | prevent marketing-driven architecture | constitution amendment |
| D-029 | ECR-001 starts with one `ecra-core` crate | ACCEPTED_FOR_SLICE | avoid speculative crate explosion | concrete current need + plan amendment |
| D-030 | ECR-001 uses JSON + RFC 8785 JCS normative v1 | ACCEPTED_FOR_V1 | inspectable/cross-language/deterministic | versioned migration evidence |
| D-031 | ECR-001 uses Rust 1.98.x / Edition 2024 planning baseline | ACCEPTED_FOR_SLICE | greenfield current stable | toolchain/security evidence |
| D-032 | Google-scale crawler/index not required for initial Search wedge | ACCEPTED | web providers + local/workspace evidence can win first | product/economics evidence |
| D-033 | Mobile and team/multi-principal product governance deferred | ACCEPTED | desktop/single-user trust first | browser wedge + identity model stable |
| D-034 | Brand `Ecra` provisional until clearance | ACCEPTED | observed collision/trademark risk | legal/brand clearance |
| D-035 | Actor attribution is distinct from authenticated Principal/IdentityAssertion | ACCEPTED | runtime actor cannot self-authenticate | constitution amendment |
| D-036 | Permission to read information does not imply permission to disclose it | ACCEPTED | capability-only model misses cross-origin/provider exfiltration | constitution amendment |
| D-037 | Scope wildcard is explicit; missing/empty never means ANY | ACCEPTED | fail-closed authorization cannot rely on caller convention | constitution amendment |
| D-038 | Consequential authorization/approval/receipts bind exact `ActionRef { ActionId, ActionDigest }`; attempts have distinct ActionAttemptId | ACCEPTED | prevent parameter substitution and retry ambiguity | versioned contract migration only |
| D-039 | VerificationReceipt is the sole canonical verification-outcome record; Fact has no independent verified truth flag | ACCEPTED | prevent state divergence | constitution amendment |
| D-040 | Mutation domain and reversibility are orthogonal; idempotency/retry remain separate | ACCEPTED | local destructive actions can exceed external reversible risk | versioned contract migration |
| D-041 | Add ECR-031 Identity, Trust Root & Sensitive Storage Foundations | ACCEPTED | identity/key/storage cannot be hidden in Cedar/browser/SQLite | dependency evidence allows safe consolidation without semantic loss |
| D-042 | Firefox Containers are site-data/session partitions, not complete Ecra agent sandbox | ACCEPTED | avoid overclaiming browser primitive | upstream Firefox semantics materially change |
| D-043 | Browser special permissions use an Ecra permission broker; some operations may require human presence/non-delegability | ACCEPTED_DIRECTION | passkeys/clipboard/files/camera/payment differ from generic clicks | ECR-006/008 empirical browser constraints |
| D-044 | Remote model/search/tool/plugin/protocol calls are information-egress boundaries | ACCEPTED | private query/context leakage exists before remote execution result | constitution amendment |
| D-045 | Runs/providers require explicit bounded-consumption semantics | ACCEPTED | agent loops create denial-of-wallet/service/resource risk | bounded surface proven impossible/irrelevant in owning spec |
| D-046 | Local model artifacts/loaders are untrusted by default | ACCEPTED | local does not remove executable/supply-chain risk | independently verified safe artifact class may receive narrower policy |
| D-047 | Inspecting an untrusted repo is distinct from executing its build/test/install hooks | ACCEPTED | developer tooling can execute arbitrary code | explicitly trusted repo policy may grant scoped execution |
| D-048 | Protocol authentication maps to Ecra identity/capabilities; token passthrough is not ambient authority | ACCEPTED_DIRECTION | confused-deputy/audience risks | protocol standard revision with equivalent stronger semantics |
| D-049 | Trusted Search tracks source identity/lineage/independence and captured as-of/hash when decision-grade | ACCEPTED_DIRECTION | copied citations are not independent corroboration; sources change | ECR-009 may refine representation |
| D-050 | Skill IR stores authority requirements, never captured live grants/approval tokens/raw secrets/session authority | ACCEPTED | one demonstration must not create permanent privilege | constitution amendment |
| D-051 | Plain append-only/hash chaining is not described as hostile tamper resistance without a protected trust anchor | ACCEPTED | attacker able to rewrite whole store can recompute chain | protected anchor/signature design may justify stronger scoped claim |
| D-052 | Trusted approval/authority/takeover UI must be distinguishable from page content | ACCEPTED_DIRECTION | web pages can spoof agent/security prompts | ECR-008 UX research can refine trusted chrome |
| D-053 | Derived memory indexes/caches/summaries are deletion-aware and preferably rebuildable/non-authoritative | ACCEPTED_DIRECTION | deletion cannot leave retrievable hidden copy | storage design may refine exact projection model |
| D-054 | High-assurance local-model/agent profiles use Ecra-mediated world access instead of ambient network/credential/tool authority | ACCEPTED_DIRECTION | makes Ecra the enforceable information/action gateway and reduces model compromise blast radius | ECR-017/021 experiments show material unsupported platform constraints |
| D-055 | WebMCP/site-native structured tools are origin-bound `CapabilityOffer`s, never `CapabilityGrant`s | ACCEPTED | structured semantics improve resolution but a page cannot authorize itself | standards change only if equivalent authority separation remains |
| D-056 | ECR-021 owns model-aware context compilation and adaptive tool aperture; narrowing context/tools never substitutes for policy | ACCEPTED_DIRECTION | small/local models benefit from progressive disclosure and bounded tool sets | ECR-021/028 evidence shows no measurable reliability/resource benefit |
| D-057 | Sandbox backends are replaceable enforcement mechanisms; Ecra owns identity, information flow, authorization, action/receipt and verification truth | ACCEPTED | prevents container/network policy from becoming a second trust model | constitution-level trust-model revision only |
| D-058 | Local-model value claims use reproducible Effective Intelligence Gain comparisons against the same-model baseline | ACCEPTED_DIRECTION | measures augmentation rather than marketing model size/provider narratives | ECR-028 may refine metrics while preserving reproducibility/same-model baseline |

## Decision Change Process

A proposal changes an accepted decision only with:
1. old and proposed decision;
2. new evidence/constraint;
3. affected ECR dependency graph;
4. constitution analysis;
5. persistence/API/skill/browser migration impact;
6. donor/license impact;
7. benchmark/risk changes;
8. rollback plan if implementation exists.

A new donor/framework is not by itself evidence to reopen a decision.
