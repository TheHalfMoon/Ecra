# Ecra Platform Planning Gap Audit

**Date:** 2026-08-27  
**Status:** CANONICAL_PLANNING  
**Purpose:** Ensure the platform roadmap covers architectural, product, security, operational, legal, evaluation, and lifecycle obligations before implementation expands.

This audit reviews gaps that are easy to omit when planning an “AI-era gateway”. A gap is considered covered only when it has a named owning roadmap slice and an expected acceptance artifact. “Covered” does not mean implemented.

## Coverage Legend

- **COVERED** — owning roadmap slice exists and expected acceptance evidence is named.
- **CROSS-CUTTING** — every affected slice must address it; one additional slice may own shared infrastructure.
- **DEFERRED** — explicitly outside the critical path, with a future owner.
- **OPEN-RESEARCH** — no irreversible implementation choice is allowed until research resolves the question.

## 1. Core Trust and Execution

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| Canonical Actor/Origin/Capability types | COVERED | ECR-001 | versioned domain schema + unit/contract tests |
| Human/agent/system attribution | COVERED | ECR-001/ECR-002 | event fixtures preserving actor attribution |
| Delegation and capability narrowing | COVERED | ECR-003 | fail-closed policy tests |
| Origin transition authority re-evaluation | COVERED | ECR-003/ECR-006 | cross-origin integration tests |
| Approval scope/expiry/replay prevention | COVERED | ECR-003 | approval-binding contract tests |
| Side-effect risk/reversibility/idempotency classification | COVERED | ECR-004 | normative action semantics matrix |
| UNKNOWN external outcome handling | COVERED | ECR-004 | fault-injection tests |
| Blind retry prevention | COVERED | ECR-004 | non-idempotent retry tests |
| Independent completion verification | COVERED | ECR-004 | verifier false-completion fixtures |
| Critical-point verification during long tasks | COVERED | ECR-004/ECR-005 | checkpoint violation scenarios |
| Human takeover/hand-back semantics | COVERED | ECR-002/ECR-006/ECR-008 | exact state-transition tests |
| Cancellation/pause/timeouts | CROSS-CUTTING | ECR-002 + each executor | cancellation safety tests |

## 2. Durability, Persistence, and Portability

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| Serializable run state | COVERED | ECR-002 | restart/resume tests |
| Append-only run ledger | COVERED | ECR-002 | monotonic event sequence tests |
| Tamper evidence/integrity | COVERED | ECR-002 | corruption/tamper detection tests |
| Portable `.ecra` run artifact | COVERED | ECR-002 | open/inspect on supported platforms |
| Schema versioning | COVERED | ECR-002 | version field + compatibility tests |
| Persisted-format migration policy | CROSS-CUTTING | ECR-002/ECR-010/ECR-012/ECR-024 | migration fixtures before format changes |
| Large blob/content-addressed artifact strategy | COVERED | ECR-002 | artifact storage contract |
| Backup/export | COVERED | ECR-010/ECR-029 | export/import round-trip |
| Optional encrypted sync | DEFERRED | ECR-022 | threat model + crypto/sync protocol before implementation |
| Data deletion semantics | COVERED | ECR-010/ECR-025/ECR-029 | deletion/export tests |

## 3. Browser Foundation and Human Product

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| Stock Firefox control before fork | COVERED | ECR-006 | fixture/live bounded BiDi flows |
| WebDriver BiDi capability mapping | COVERED | ECR-006 | capability matrix |
| Privileged browser bridge boundary | COVERED | ECR-007/ECR-008 | authenticated IPC contract + threat model |
| Firefox upstream update strategy | COVERED | ECR-007/ECR-024 | documented update/rebase cadence and emergency process |
| Patch inventory/minimization | COVERED | ECR-007 | machine-readable patch/provenance ledger |
| Browser extension compatibility | COVERED | ECR-007 | compatibility smoke suite |
| Profile/session compatibility and migration | COVERED | ECR-007/ECR-029 | import/migration tests |
| Containers as security/state isolation | COVERED | ECR-008 | unauthorized cross-container tests |
| Human/agent/shared tab ownership | COVERED | ECR-008 | conflict/takeover tests |
| Background agent tabs without focus theft | COVERED | ECR-008 | UX integration tests |
| Agent visible state/authority | COVERED | ECR-008 | user-flow acceptance tests |
| Normal browsing without model dependency | COVERED | ECR-008 | model-off daily browse smoke suite |
| Startup/performance regressions | COVERED | ECR-007/ECR-008/ECR-028 | budgeted browser performance benchmarks |
| Accessibility | COVERED | ECR-026 | keyboard/screen-reader/accessibility checks |
| Internationalization/localization | COVERED | ECR-026 | locale/RTL/string-extraction gates |
| Cross-platform parity | COVERED | ECR-007/ECR-008/ECR-024 | Windows/macOS/Linux release matrix |
| Mobile browser | DEFERRED | future roadmap amendment | explicit non-goal until desktop wedge proves value |

## 4. Search, Trusted Information, and Web Content

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| Unified evidence contract across providers | COVERED | ECR-009 | contract tests |
| Source type and primary-source classification | COVERED | ECR-009 | ranking fixtures |
| Freshness and observation time | COVERED | ECR-009 | stale-result tests |
| Claim-to-source mapping | COVERED | ECR-009 | evidence coverage metrics |
| Contradiction detection/visibility | COVERED | ECR-009 | contradictory-source fixtures |
| Local/private search | COVERED | ECR-009 | offline search test |
| Hybrid lexical/structural/semantic retrieval | COVERED | ECR-009 | retrieval benchmark matrix |
| Search provider abstraction | COVERED | ECR-009 | provider contract |
| Result caching and invalidation | COVERED | ECR-009/ECR-027 | cache freshness tests |
| robots/access/publisher policy | COVERED | ECR-027 | source-policy contract |
| Attribution/licensing/copyright-aware retention | COVERED | ECR-027 | policy + metadata requirements |
| Download safety/malware boundary | COVERED | ECR-027/ECR-017 | quarantined/sandbox download flow |
| Search abuse/rate limiting/provider quotas | COVERED | ECR-009/ECR-016 | quota/backoff tests |
| Ranking transparency | COVERED | ECR-009 | inspectable ranking/source metadata |
| Public web crawl/index at Google scale | DEFERRED | future strategy | not required for initial trusted metasearch/retrieval wedge |

## 5. Workspace and Memory

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| Workspace as authority/context scope | COVERED | ECR-010 | isolation tests |
| Typed/provenance-aware memories | COVERED | ECR-010 | memory schema + retrieval tests |
| Candidate memory vs accepted memory | COVERED | ECR-010 | trust-transition tests |
| Memory aging/staleness | COVERED | ECR-010 | time-based fixtures |
| Memory contradiction/update | COVERED | ECR-010 | conflict/version tests |
| Memory poisoning resistance | COVERED | ECR-005/ECR-010 | adversarial fixtures |
| Export/delete | COVERED | ECR-010/ECR-029 | round-trip + deletion tests |
| Cross-device memory | DEFERRED | ECR-022 | encrypted sync spec |
| Shared/team workspace authorization | DEFERRED | roadmap amendment after single-user model is stable | multi-principal policy model required first |

## 6. Skills, Learning, Replay, Repair

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| Typed Skill IR | COVERED | ECR-012 | versioned schema + parser/validator tests |
| Artifact reads/writes/dataflow | COVERED | ECR-012 | static validation tests |
| Explicit side-effect semantics | COVERED | ECR-012 | invalid-skill rejection tests |
| Preconditions/postconditions | COVERED | ECR-012 | compatibility evaluation tests |
| Origin/capability requirements | COVERED | ECR-012 | authority validation tests |
| Human demo → skill | COVERED | ECR-013 | deterministic fixture compilation |
| Agent run → skill | COVERED | ECR-013 | same IR acceptance tests |
| Sandbox validation before promotion | COVERED | ECR-013/ECR-017 | promotion gate |
| Deterministic/no-model replay | COVERED | ECR-014 | zero-model replay benchmark |
| Compatibility/determinism checks | COVERED | ECR-014 | environment drift fixtures |
| Localized divergence repair | COVERED | ECR-015 | stage-local repair tests |
| Downstream invalidation | COVERED | ECR-015 | dependency invalidation tests |
| Skill versioning/rollback | COVERED | ECR-012/ECR-015 | version/rollback tests |
| Skill trust/signing/registry | DEFERRED/COVERED | ECR-023 | signing/reputation contract before public registry |

## 7. Models, Context, and Agent Runtime

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| Provider-neutral model interface | COVERED | ECR-002/ECR-016/ECR-021 | mock/provider contract tests |
| Local model adapter | COVERED | ECR-021 | reproducible local inference experiments |
| Context assembly/provenance | COVERED | ECR-009/ECR-010 | context trace inspection |
| Context compaction for long runs | COVERED | ECR-002/ECR-010 | resume/compaction invariants |
| Token/cost budgets | COVERED | ECR-002/ECR-028 | usage accounting metrics |
| Model fallback | COVERED | ECR-021 | explicit fallback policy tests |
| Model output as untrusted proposal | COVERED | Constitution/ECR-003 | policy bypass tests |
| Custom model training | DEFERRED | future research spec | only after verified proprietary/open trajectory corpus exists |

## 8. Protocols and Ecosystem

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| MCP client/server | COVERED | ECR-016 | conformance/interoperability tests |
| ACP integration | COVERED | ECR-016 | protocol tests |
| A2A integration | COVERED | ECR-016 | protocol tests |
| Agent Skills import/export | COVERED | ECR-016 | round-trip fixtures |
| WebMCP | COVERED | ECR-011/ECR-016 | trust/provenance tests |
| Stable local Ecra API/SDK | DEFERRED/COVERED | ECR-030 | compatibility policy before GA |
| Plugin capability manifest | COVERED | ECR-017 | deny-by-default tests |
| Plugin sandbox/resource limits | COVERED | ECR-017 | escape/resource-exhaustion tests |
| Plugin/skill signing | DEFERRED/COVERED | ECR-023 | signature/key lifecycle spec |
| Registry malware/reputation/review | DEFERRED | ECR-023 | registry threat model |
| Extension version compatibility | COVERED | ECR-017/ECR-023 | compatibility metadata/tests |

## 9. Terminal, Developer, and Data

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| Shell/process authority | COVERED | ECR-018 | cwd/fs/network/process scope tests |
| Process-tree cleanup/timeouts | COVERED | ECR-018 | kill/cancellation tests |
| Output bounds | COVERED | ECR-018 | bounded-output tests |
| Repository structural context | COVERED | ECR-019 | parsing/index benchmark |
| Build/test receipts | COVERED | ECR-019 | reproducibility tests |
| Browser QA tied to code changes | COVERED | ECR-019 | end-to-end fixture |
| Developer release workflow verification | COVERED | ECR-019 | signed/verified release fixture |
| SQL/data source lineage | COVERED | ECR-020 | claim → query → source trace |
| Data quality/uncertainty | COVERED | ECR-020 | warnings/validation tests |
| Notebook/runtime isolation | COVERED | ECR-020/ECR-017 | sandbox tests |
| Generated-number provenance | COVERED | ECR-020 | unsupported-number rejection/flagging |

## 10. Privacy, Security Operations, and Supply Chain

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| Threat model ownership | CROSS-CUTTING | ECR-005 + every privileged slice | updated threat model in same change |
| No hidden telemetry | COVERED | ECR-025 | documented defaults + network tests |
| Redaction/logging policy | COVERED | ECR-025 | secret/PII fixture tests |
| Crash reporting consent | COVERED | ECR-025 | opt-in/opt-out behavior tests |
| At-rest protection for sensitive local state | OPEN-RESEARCH | ECR-003/ECR-025 | platform key-store/encryption design before sensitive persistence |
| Dependency advisory monitoring | COVERED | ECR-024 | CI/security workflow |
| SBOM | COVERED | ECR-024 | release artifact |
| Reproducible builds | COVERED | ECR-024 | reproducibility report |
| Binary/update signing | COVERED | ECR-024 | verification test |
| Emergency browser security updates | COVERED | ECR-007/ECR-024 | documented SLA/process |
| Donor license provenance | CROSS-CUTTING | ECR-024 + every donor-using slice | donor/license ledger update |
| Unsafe Rust policy | COVERED | ECR-001/ECR-024 | lint/review rule + exceptions ledger |
| Fuzzing/property tests on parsers/policy formats | COVERED | ECR-005 | fuzz/property test harness |

## 11. Operations and Product Lifecycle

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| CI baseline | COVERED | ECR-001/ECR-024 | fmt/clippy/test/audit gates |
| Release channels | COVERED | ECR-024 | nightly/beta/stable policy |
| Automatic updates | COVERED | ECR-024 | signed update flow |
| Rollback | COVERED | ECR-024 | rollback drill |
| Feature flags | COVERED | affected product slices | explicit default/retirement policy |
| Persisted schema downgrade/rollback behavior | CROSS-CUTTING | state-owning slices/ECR-024 | migration rollback docs/tests |
| Diagnostics/doctor command | COVERED | ECR-025 | local diagnostic bundle tests |
| Support bundle redaction | COVERED | ECR-025 | redaction fixtures |
| Experimental feature labeling | COVERED | ECR-024/ECR-028 | release/documentation policy |

## 12. Product Strategy and Adoption

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| Initial wedge defined | COVERED | ECR-008 | browser daily-use acceptance plan |
| Search differentiation vs generic answer engine | COVERED | ECR-009/ECR-028 | workspace/evidence benchmark |
| Skill economics moat | COVERED | ECR-013–ECR-015/ECR-028 | compile/replay/repair cost metrics |
| Local-model augmentation thesis | COVERED | ECR-021/ECR-028 | reproducible comparative evaluation |
| External-agent value thesis | COVERED | ECR-016/ECR-030 | third-party workflow evaluation |
| Lock-in avoidance/export | COVERED | Constitution/ECR-029 | export/interoperability tests |
| Branding/trademark clearance | OPEN-RESEARCH | founder/legal workstream | clearance before public launch; not an engineering claim |
| Business model | OPEN-RESEARCH | product/business workstream | must not invalidate local-first/privacy constitution |
| Mobile | DEFERRED | future product spec | desktop proof first |
| Team/enterprise governance | DEFERRED | future spec | single-user authority model first |

## 13. Benchmark and Claim Gaps

The following metrics must exist before Ecra makes corresponding public claims:

| Claim area | Required metrics | Owner |
|---|---|---|
| Reliable agent execution | task success, constraint retention, crash/resume, duplicate side-effect rate | ECR-005/ECR-028 |
| Trusted answers | evidence coverage, provenance coverage, freshness, unsupported claim rate, contradiction visibility | ECR-009/ECR-028 |
| Secure browser agency | prompt-injection ASR, cross-origin leakage, capability overreach, secret exposure, memory poisoning | ECR-005/ECR-028 |
| Human-agent collaboration | takeover latency, intervention precision, unnecessary approval rate, human correction recovery | ECR-008/ECR-028 |
| Reusable skills | compile yield, replay success, model calls avoided, repair success, cost per repeated task | ECR-013–ECR-015/ECR-028 |
| Browser quality | startup, navigation latency, memory/CPU, crash rate, extension compatibility, accessibility | ECR-007/ECR-008/ECR-026/ECR-028 |
| Local model uplift | matched workflow success/cost/privacy tradeoff against unaided larger models | ECR-021/ECR-028 |

## 14. Explicitly Rejected Planning Shortcuts

The roadmap MUST NOT silently assume any of the following:

- that an LLM judge is sufficient for success verification;
- that a model-visible page is trusted instruction;
- that a browser profile is an acceptable agent permission boundary;
- that “retry” is safe after ambiguous external side effects;
- that a vector database is the memory architecture;
- that MCP is the internal domain model;
- that a local model is inherently private if tools/context leak data;
- that Wasm automatically makes plugins safe;
- that browser automation success proves daily-browser quality;
- that a recorded workflow is deterministic or reusable;
- that a citation proves a claim is supported;
- that a search result may be retained/reused without source/retention policy;
- that optional cloud services may become hidden prerequisites;
- that a Firefox fork may defer upstream security/update engineering;
- that “open source” means code can be copied without license/provenance review;
- that ecosystem/network effects justify artificial data lock-in.

## Audit Result

**Planning coverage:** COMPLETE ENOUGH TO BEGIN ECR-001 SPEC-DRIVEN IMPLEMENTATION.

No known foundational category is intentionally left ownerless. Items marked OPEN-RESEARCH or DEFERRED are explicit and cannot be silently implemented as assumptions. This audit must be amended whenever a new major surface, persistent data class, privileged capability, external protocol, or public product claim enters scope.
