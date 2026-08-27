# Ecra Platform Risk Register

**Date:** 2026-08-27  
**Status:** CANONICAL_PLANNING

This register covers risks that can invalidate Ecra's platform thesis even if individual features appear to work. Scores are qualitative and must be re-evaluated as implementation evidence arrives.

| ID | Risk | Likelihood | Impact | Primary mitigation | Owning slices |
|---|---|---:|---:|---|---|
| R-001 | Scope explosion turns Ecra into disconnected browser/search/IDE/data products | High | Critical | Spec-of-specs ordering, Wedge Before Empire, no surface without flywheel test | all, especially ECR-008/ECR-009/ECR-018–020 |
| R-002 | Firefox downstream maintenance consumes the project | High | Critical | Stock Firefox prototype first, minimal privileged patch surface, patch ledger, release/update program | ECR-006/ECR-007/ECR-024 |
| R-003 | Prompt injection or external content expands agent authority | High | Critical | content-is-data rule, origin-aware capability system, independent policy, adversarial harness | ECR-003/ECR-005/ECR-006/ECR-011 |
| R-004 | Cross-origin information leakage through agent reasoning/actions | High | Critical | agent-origin authority model, SOP-style tests, capability re-evaluation on origin transition | ECR-003/ECR-005/ECR-006/ECR-008 |
| R-005 | Secrets leak into model context/logs/memory | Medium | Critical | secret handles, origin/field-bound mediation, redaction, telemetry contracts | ECR-003/ECR-025 |
| R-006 | Ambiguous external side effects are duplicated after crash/retry | Medium | Critical | UNKNOWN outcome, idempotency/retry classes, reconciliation before retry, durable receipts | ECR-002/ECR-004 |
| R-007 | Verification becomes a second LLM that repeats planner mistakes | High | High | deterministic evidence preference, independent verifier architecture, false-positive benchmark gates | ECR-004/ECR-005/ECR-028 |
| R-008 | Human browsing quality is degraded by agent infrastructure | Medium | Critical | no-model daily browse path, performance budgets, upstream parity tests, feature isolation | ECR-007/ECR-008/ECR-026/ECR-028 |
| R-009 | Approval UX becomes unusable due to permission fatigue | High | High | risk-based scoped policy, exact-action approvals, measure unnecessary approval rate | ECR-003/ECR-008/ECR-028 |
| R-010 | Memory becomes a poisoned, stale, contradictory text swamp | High | High | typed provenance, candidate-memory pipeline, staleness/contradiction states, adversarial tests | ECR-005/ECR-010 |
| R-011 | Search synthesizes unsupported or stale claims despite citations | High | Critical | claim-to-evidence mapping, freshness, contradiction visibility, source quality classification | ECR-009/ECR-027/ECR-028 |
| R-012 | Search/source ingestion creates copyright, access-policy, retention or publisher conflicts | Medium | High | explicit source/content compliance spec before broad crawling/indexing | ECR-027 |
| R-013 | Skill compiler learns brittle UI traces rather than reusable procedures | High | High | typed artifact/dataflow IR, pre/postconditions, semantic capability router, sandbox replay | ECR-011–ECR-014 |
| R-014 | Skill repair silently changes behavior/authority | Medium | Critical | localized repair boundaries, downstream invalidation, policy re-check, versioned promotion | ECR-015 |
| R-015 | Model vendor integration leaks into trusted core | Medium | High | provider-neutral interfaces, protocol adapters, dependency boundary tests | ECR-001/ECR-016/ECR-021 |
| R-016 | Local models are marketed as capable without enough quality | Medium | High | narrow reproducible uplift experiments; no general superiority claim | ECR-021/ECR-028 |
| R-017 | Plugin ecosystem creates sandbox escape/supply-chain path | High | Critical | capability manifests, sandboxing, signing/reputation, advisory monitoring, resource limits | ECR-017/ECR-023/ECR-024 |
| R-018 | MCP/ACP/A2A adapters accidentally expose entire user state | Medium | Critical | gateway capability grants, adapter isolation, no direct privileged bridge exposure | ECR-016/ECR-030 |
| R-019 | Persisted schema evolves without migrations and breaks user data/runs/skills | Medium | Critical | versioned schemas, migration fixtures, compatibility policy, export/rollback | ECR-002/ECR-010/ECR-012/ECR-024/ECR-029 |
| R-020 | Telemetry/crash diagnostics violate privacy thesis | Medium | Critical | no hidden telemetry, explicit consent, redaction, local diagnostics first | ECR-025 |
| R-021 | Cross-platform browser behavior diverges and doubles complexity | High | High | CI/release matrix, platform abstractions only where proven, parity fixtures | ECR-007/ECR-024/ECR-026 |
| R-022 | Browser extension compatibility breaks daily-user adoption | Medium | High | extension smoke matrix and upstream-compatible browser architecture | ECR-007/ECR-008 |
| R-023 | Accessibility/i18n become late rewrites | Medium | High | dedicated slice plus acceptance criteria in each user-facing feature | ECR-026 + cross-cutting |
| R-024 | Donor licenses/attribution contaminate desired distribution model | Medium | High | copy nothing without donor/license record; separate conceptual inspiration from source reuse | ECR-024 + every donor-using slice |
| R-025 | Brand/name conflict blocks launch or creates confusion | Medium | High | trademark/domain/package clearance before public brand commitment | founder/legal workstream |
| R-026 | Public benchmark claims are gamed or non-reproducible | Medium | High | publish exact harness/version/tasks, frozen reports, multiple benchmark families | ECR-005/ECR-028 |
| R-027 | Ecosystem cold start prevents gateway network effects | High | Medium | first-party high-value skills/connectors, standard protocols, import/export, strong daily product wedge | ECR-016/ECR-023/ECR-030 |
| R-028 | Artificial lock-in damages trust and adoption | Low/Medium | High | portable runs/memory/skills, standards, explicit export path | ECR-010/ECR-016/ECR-029 |
| R-029 | Business model conflicts with local-first/no-hidden-telemetry principles | Medium | High | constitutional governance; monetization cannot silently create cloud-required core | founder/product + constitution |
| R-030 | Training a custom model distracts from runtime/data moat | Medium | High | defer training until benchmark/data flywheel demonstrates justified specialized target | future research after ECR-021/ECR-028 |
| R-031 | Privileged browser bridge becomes an unauditable superuser API | Medium | Critical | tiny authenticated IPC contract, least-privilege operations, fuzzing, threat-model updates | ECR-007/ECR-008/ECR-005 |
| R-032 | WebMCP/native structured tools are trusted merely because they are structured | Medium | Critical | bind tools to origin/principal/provenance; policy applies equally to semantic and low-level execution | ECR-003/ECR-011 |
| R-033 | Background agents create race conditions with human actions | Medium | High | explicit control ownership, conflict detection, takeover/handback as run events | ECR-002/ECR-008 |
| R-034 | Context assembly leaks data across workspaces/containers | Medium | Critical | scope-aware retrieval, provenance, capability checks before retrieval/model context inclusion | ECR-009/ECR-010/ECR-003 |
| R-035 | Data/terminal surfaces create a second trust model | Medium | Critical | constitutional gate: reuse Actor/Capability/Receipt/Verifier, no parallel agent runtime | ECR-018–ECR-020 |

## Risk Acceptance Rules

- A **Critical** impact risk cannot be accepted implicitly by an implementation PR.
- A plan touching a Critical risk must name its prevention/detection tests and rollback/containment strategy where applicable.
- Security boundaries cannot be substituted with prompt wording.
- “We will monitor later” is not a mitigation for authority, secret, side-effect, persistence, or browser-update risks.
- Risk status changes should cite evidence (tests, benchmark report, production incident, upstream change, or validated design), not optimism.

## Release Blockers

Regardless of feature status, release is blocked by:

- known critical browser-engine/security dependency exposure without accepted upstream mitigation;
- known capability escape granting unauthorized agent authority;
- known secret exposure to generic model/log/memory paths;
- known duplicate non-idempotent side effects caused by Ecra retry/resume semantics;
- known verifier false-positive path that marks a consequential failed action successful in the release-critical workflow set;
- unsigned/unverified update path once Ecra distributes privileged browser binaries;
- license obligations for included donor code that are unknown or unmet.
