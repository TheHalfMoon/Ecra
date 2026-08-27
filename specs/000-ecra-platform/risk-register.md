# Ecra Platform Risk Register

**Date:** 2026-08-27  
**Status:** CANONICAL_PLANNING_V2  
**Review source:** `pre-implementation-review-2026-08-27.md`

Scores are qualitative and must be revised from evidence, not optimism.

| ID | Risk | Likelihood | Impact | Primary mitigation | Owning slices |
|---|---|---:|---:|---|---|
| R-001 | Scope explosion turns Ecra into disconnected browser/search/IDE/data products | High | Critical | Spec-of-specs ordering, Wedge Before Empire, no surface without flywheel test | all, especially ECR-008/ECR-009/ECR-018–020 |
| R-002 | Firefox downstream maintenance consumes the project | High | Critical | stock Firefox prototype first, minimal patch surface, patch ledger, update program | ECR-006/ECR-007/ECR-024 |
| R-003 | Prompt injection/external content expands authority | High | Critical | content-is-data, independent policy, adversarial harness | ECR-003/ECR-005/ECR-006/ECR-011 |
| R-004 | Cross-origin/provider information leakage through allowed actions | High | Critical | source→sink information-flow policy, explicit InformationUse, egress tests | ECR-001/ECR-003/ECR-005/ECR-009 |
| R-005 | Secrets leak to models/logs/memory/tools | Medium | Critical | secret handles, classification/egress policy, redaction | ECR-003/ECR-025 |
| R-006 | Ambiguous external effects duplicate after crash/retry | Medium | Critical | exact attempts, UNKNOWN, idempotency/reconciliation, durable receipts | ECR-001/ECR-002/ECR-004 |
| R-007 | Verification becomes a second LLM repeating planner error | High | High | deterministic evidence preference, independent verifier, FP/FN gates | ECR-004/ECR-005/ECR-028 |
| R-008 | Agent infrastructure degrades normal browser quality | Medium | Critical | no-model browse path, performance budgets, upstream parity | ECR-007/ECR-008/ECR-026/ECR-028 |
| R-009 | Approval UX becomes unusable / users rubber-stamp | High | High | risk/scoped policy, exact approvals, unnecessary-approval metric | ECR-003/ECR-008/ECR-028 |
| R-010 | Memory becomes poisoned/stale/contradictory swamp | High | High | candidate-memory transition, provenance/classification/freshness, adversarial tests | ECR-005/ECR-010 |
| R-011 | Search synthesizes unsupported/stale claims despite citations | High | Critical | claim→evidence, freshness, contradiction, source identity/independence | ECR-009/ECR-027/ECR-028 |
| R-012 | Search ingestion creates copyright/access/retention/publisher conflicts | Medium | High | explicit source/content compliance spec before broad crawling/indexing | ECR-027 |
| R-013 | Skill compiler learns brittle UI traces instead of reusable procedures | High | High | typed artifact/dataflow IR, semantic capabilities, sandbox replay | ECR-011–ECR-014 |
| R-014 | Skill repair silently broadens behavior/authority/classification | Medium | Critical | localized repair, invalidation, fresh policy/egress check, versioned promotion | ECR-015 |
| R-015 | Model vendor integration leaks into trusted core | Medium | High | provider-neutral interfaces, adapter boundaries, dependency tests | ECR-001/ECR-016/ECR-021 |
| R-016 | Local-model marketing exceeds measured capability | Medium | High | narrow reproducible uplift experiments, no general superiority claim | ECR-021/ECR-028 |
| R-017 | Plugin ecosystem creates sandbox escape/supply-chain path | High | Critical | capability manifests, sandbox tiers, signing/reputation, limits, advisories | ECR-017/ECR-023/ECR-024 |
| R-018 | MCP/ACP/A2A adapters expose excessive user state | Medium | Critical | gateway grants, identity/audience mapping, scoped state views | ECR-016/ECR-030/ECR-031 |
| R-019 | Persisted schemas evolve without migration and break user state | Medium | Critical | versioning, migration fixtures, compatibility/export/rollback | ECR-002/ECR-010/ECR-012/ECR-024/ECR-029 |
| R-020 | Telemetry/crash diagnostics violate privacy thesis | Medium | Critical | no hidden telemetry, consent, redaction, local diagnostics first | ECR-025 |
| R-021 | Cross-platform browser behavior diverges and multiplies complexity | High | High | CI/release matrix, minimal abstractions, parity fixtures | ECR-007/ECR-024/ECR-026 |
| R-022 | Browser extension compatibility/trust breaks adoption/security | Medium | High/Critical | extension smoke + trust tiers; broad extensions treated as threat input | ECR-007/ECR-008 |
| R-023 | Accessibility/i18n become late rewrites | Medium | High | dedicated slice + per-feature acceptance criteria | ECR-026 + cross-cutting |
| R-024 | Donor licenses/attribution contaminate distribution model | Medium | High | copy nothing without exact donor/license record | ECR-024 + donor-using slices |
| R-025 | Brand/name conflict blocks launch | Medium | High | trademark/domain/package clearance before public commitment | founder/legal |
| R-026 | Public benchmarks are gamed/non-reproducible | Medium | High | exact harness/version/tasks/reports + multiple benchmark families | ECR-005/ECR-028 |
| R-027 | Ecosystem cold start prevents gateway network effects | High | Medium | first-party valuable skills/connectors, standards, strong daily wedge | ECR-016/ECR-023/ECR-030 |
| R-028 | Artificial lock-in damages trust/adoption | Low/Medium | High | portable runs/memory/skills, standards, export | ECR-010/ECR-016/ECR-029 |
| R-029 | Business model conflicts with local-first/privacy constitution | Medium | High | constitutional governance; no silent cloud-required core | founder/product |
| R-030 | Custom-model training distracts from runtime/data moat | Medium | High | defer until benchmark/data flywheel identifies justified bottleneck | future after ECR-021/ECR-028 |
| R-031 | Privileged browser bridge becomes unauditable superuser API | Medium | Critical | OS peer auth/ACL, ephemeral binding, replay protection, tiny method set, fuzzing | ECR-007/ECR-008/ECR-005 |
| R-032 | WebMCP/native structured tools are trusted merely because structured | Medium | Critical | bind origin/principal/provenance; same policy/egress gates as low-level actions | ECR-003/ECR-011/ECR-016 |
| R-033 | Background agents race/conflict with human actions | Medium | High | control ownership, conflict detection, takeover/handback events | ECR-002/ECR-008 |
| R-034 | Context assembly leaks across workspaces/containers | Medium | Critical | scoped retrieval + information-flow checks before context/model inclusion | ECR-003/ECR-009/ECR-010 |
| R-035 | Data/terminal/developer surfaces create a second trust model | Medium | Critical | reuse Actor/Principal/Capability/InformationFlow/Receipt/Verifier | ECR-018–ECR-020 |
| R-036 | Actor attribution is mistaken for authenticated principal identity | Medium | Critical | separate types, ECR-031 assertions/trust roots, authorization requires identity context | ECR-001/ECR-031/ECR-003 |
| R-037 | Implicit/empty scope is interpreted as unrestricted | Medium | Critical | explicit ScopeConstraint algebra; only `any_explicit` is wildcard | ECR-001/ECR-003 |
| R-038 | Approval/receipt binds ActionId but action parameters change | Medium | Critical | ActionDigest/ActionRef canonical binding + mismatch rejection | ECR-001/ECR-003 |
| R-039 | Retry attempts cannot be distinguished in audit/reconciliation | Medium | Critical | ActionAttemptId + exact ActionRef per receipt | ECR-001/ECR-002/ECR-004 |
| R-040 | Fact and verifier maintain divergent “verified” state | Medium | Critical | VerificationReceipt single source of verification truth | ECR-001/ECR-004 |
| R-041 | Resource locator aliases bypass policy | Medium | Critical | stable ResourceId; locator explicitly non-authoritative; provider identity resolution | ECR-001/ECR-003/providers |
| R-042 | Unbounded agent/model/tool loops cause cost/resource denial of wallet/service | High | High/Critical | RunBudget/ResourceBudget, circuit breakers, cancellation/fault tests | ECR-002/ECR-005/ECR-017/ECR-028 |
| R-043 | Browser page spoofs Ecra approval/agent state UI | Medium | Critical | unspoofable trusted chrome, no native approval affordance from page content | ECR-008/ECR-005 |
| R-044 | Local model artifact/custom loader compromises host or exhausts GPU/RAM | Medium | Critical | artifact provenance/hash/license, custom-code deny-by-default, sandbox/resource limits | ECR-017/ECR-021/ECR-024 |
| R-045 | Untrusted repo build/test/install hooks execute with developer ambient authority | High | Critical | inspect-vs-execute trust levels, structured process authority, sandbox | ECR-017/ECR-018/ECR-019 |
| R-046 | Source/citation laundering makes copied claims look independently corroborated | High | High | source lineage/independence/copy-cascade detection + benchmark | ECR-009/ECR-028 |
| R-047 | Deleted memory remains retrievable from vector/FTS/cache/summary projections | Medium | High | deletion propagation and rebuildable derived projections | ECR-010/ECR-029 |
| R-048 | Skill compilation captures approval/grant/secret from demonstration | Medium | Critical | Skill IR stores requirements only; reject captured live authority/tokens/secrets | ECR-012/ECR-013 |
| R-049 | Browser Containers are mistaken for complete agent sandbox | Medium | Critical | describe as storage/session partitions; Ecra policy/process boundaries enforce authority | ECR-006/ECR-008 |
| R-050 | Same-user rogue process forges/replays privileged browser IPC | Medium | Critical | peer ACL/identity, ephemeral channel binding, sequence/replay protection | ECR-007 |
| R-051 | Special browser permissions (passkeys/clipboard/files/camera/etc.) inherit generic click authority | Medium | Critical | browser permission broker + human-presence/non-delegable policies | ECR-003/ECR-006/ECR-008 |
| R-052 | Plain hash chain is marketed as hostile tamper resistance | Medium | High | scoped integrity wording; protected trust-root MAC/signature/anchor for stronger claim | ECR-002/ECR-031/ECR-028 |
| R-053 | Sensitive state persisted before key/storage protection design | Medium | Critical | synthetic/non-sensitive persistence first; sensitive-state gate | ECR-002/ECR-031/ECR-025 |
| R-054 | Remote protocol token passthrough creates confused deputy | Medium | Critical | version-pinned auth mapping, audience/resource binding, no token passthrough | ECR-016/ECR-031 |
| R-055 | Malicious PDF/archive/parser input escapes search/content pipeline | Medium | Critical | parser isolation/resource limits/quarantine/hostile corpora | ECR-017/ECR-027/ECR-005 |

## Risk Acceptance Rules

- A **Critical** risk cannot be accepted implicitly by a feature PR.
- Any plan touching a Critical risk must name prevention/detection tests and containment/rollback where applicable.
- Security boundaries cannot be replaced by prompt wording, “local-only”, Containers, citations, Wasm, or a model refusal alone.
- “Monitor later” is not mitigation for identity, authority, information flow, secrets, side effects, persistence, privileged browser IPC or supply chain.
- Risk status changes cite tests, benchmark reports, incidents, upstream changes or validated design evidence.

## Release Blockers

Regardless of feature status, release is blocked by a known unresolved path for:

- critical browser/dependency vulnerability without accepted mitigation;
- unauthorized capability or information-flow escape;
- Actor/self-asserted identity being accepted as authenticated authority;
- raw secret exposure to generic model/log/memory/tool paths;
- duplicate consequential non-idempotent side effects caused by Ecra retry/resume;
- verifier false-positive marking release-critical consequential failure successful;
- page-spoofed privileged approval surface in a release-critical flow;
- unbounded executable/provider loop with no effective circuit breaker in a release-critical flow;
- unsigned/unverified privileged update path once Ecra distributes browser binaries;
- unknown/unmet source/donor license obligations.
