# Ecra Platform Planning Gap Audit

**Date:** 2026-08-27  
**Status:** CANONICAL_PLANNING_V2  
**Review source:** `pre-implementation-review-2026-08-27.md`  
**Purpose:** ensure every material product/security/operational gap has an owner and required evidence before implementation expands.

“Owned” means planned responsibility exists; it does not mean implemented or safe yet.

## Legend

- **OWNED** — named slice(s) + required evidence exist.
- **CROSS-CUTTING** — every affected slice must address it; a shared owner may supply infrastructure.
- **OPEN-RESEARCH** — irreversible implementation choice blocked until owning slice research resolves it.
- **DEFERRED** — intentionally outside current critical path with future owner/trigger.
- **GATED** — early fixture work may proceed, but real sensitive/privileged use is blocked until named gate closes.

## 1. Trusted Domain / Identity / Authority

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| Actor Human/Agent/System attribution | OWNED | ECR-001/ECR-002 | typed contract + event fixtures |
| Actor vs authenticated Principal | OWNED | ECR-001/ECR-031 | non-interchangeable types + assertion validation tests |
| Human↔agent on-behalf-of binding | OWNED | ECR-031/ECR-003 | identity assertion/delegation fixtures |
| Identity/key issuance, rotation, revocation | OPEN-RESEARCH | ECR-031 | trust-root/key-lifecycle spec + tests |
| Request != Grant | OWNED | ECR-001 | type/ID-confusion tests |
| Explicit scope algebra | OWNED | ECR-001 | no empty/missing wildcard; explicit ANY fixtures |
| Capability narrowing/intersection | OWNED | ECR-003 | fail-closed subset/intersection property tests |
| Immutable authorization decision/lease | OWNED | ECR-003 | binds ActionRef + principal + grants/policy/approval + expiry/revocation |
| Authorization TOCTOU / revocation | OWNED | ECR-003/ECR-031 | stale-decision/revocation race tests |
| Approval binding / replay prevention | OWNED | ECR-003 | exact ActionDigest/context/one-use/expiry tests |
| Resource identity vs locator aliases | OWNED | ECR-001 + providers | ResourceId contract; provider canonicalization/alias tests |
| Free-form metadata accidentally parsed as authority | CROSS-CUTTING | ECR-001/ECR-005 | adversarial string/property tests |

## 2. Information Flow / Privacy / Secrets

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| Read authority != disclosure authority | OWNED | ECR-001/ECR-003 | InformationUse contract + source→sink denial tests |
| Data classification/taint lineage | OWNED | ECR-001/ECR-003 | public/private/sensitive/secret/unknown + derivation tests |
| Conservative derived-data inheritance | OWNED | ECR-003 | join/transform/declassification property tests |
| Remote model context egress | OWNED | ECR-003/ECR-021/ECR-025 | explicit disclosure decision + redaction/minimization tests |
| Remote search query/context egress | OWNED | ECR-003/ECR-009/ECR-025 | provider-call denial/redaction fixtures |
| Plugin/protocol/log/telemetry data egress | OWNED | ECR-003/ECR-016/ECR-017/ECR-025 | source→sink tests |
| Secret raw-value minimization | OWNED | ECR-003 | secret-handle contract + no-model/log raw secret tests |
| Passkeys/user-presence credentials | OWNED | ECR-003/ECR-006/ECR-008 | non-delegable/presence-required permission tests |
| Hidden telemetry | OWNED | ECR-025 | network-default/offline/consent tests |
| Diagnostic/crash bundle redaction | OWNED | ECR-025 | adversarial secret/PII fixtures |
| Provider retention/privacy disclosure | OWNED | ECR-021/ECR-025/ECR-027 | documented provider policy metadata + UI/egress behavior |
| At-rest protection for real sensitive state | GATED/OPEN-RESEARCH | ECR-031/ECR-025 | key/storage envelope spec before real sensitive persistence |
| Fully compromised local OS/user account | BOUNDARY | platform threat model | explicitly out of guaranteed containment unless future hardware-backed design says otherwise |

## 3. Action / Side Effects / Durability / Budgets

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| Mutation vs reversibility conflation | OWNED | ECR-001 | orthogonal EffectProfile contract tests |
| Idempotency / retry classes | OWNED | ECR-001/ECR-004 | exhaustive semantic matrix |
| Exact action approval/receipt binding | OWNED | ECR-001/ECR-003 | ActionDigest mutation tests + approval fixture |
| Intent vs execution attempt | OWNED | ECR-001/ECR-002 | multiple-attempt/one-intent fixtures |
| UNKNOWN outcome | OWNED | ECR-001/ECR-002/ECR-004 | crash/network fault injection |
| Blind retry prevention | OWNED | ECR-004 | non-idempotent UNKNOWN reconciliation tests |
| Duplicate external side effects | OWNED | ECR-002/ECR-004 | attempt/idempotency/reconciliation scenarios |
| Serializable RunState | OWNED | ECR-002 | restart/resume tests |
| Append-only event truth | OWNED | ECR-002 | sequence/immutability tests |
| Hash-chain overclaim | OWNED | ECR-002/ECR-031 | explicitly scoped integrity claim; hostile tamper tests only with protected anchor |
| Portable `.ecra` artifact | OWNED/GATED | ECR-002/ECR-029 | synthetic first; sensitive content gate + import/export tests |
| Large content/blob storage | OWNED | ECR-002 | content-addressed ArtifactRef contract |
| Schema migration / downgrade | CROSS-CUTTING | state-owning slices/ECR-024 | forward/backward migration fixtures |
| Wall-time / step / tool/model call budgets | OWNED | ECR-002/ECR-005 | budget exhaustion/cancellation tests |
| Token/cost budgets | OWNED | ECR-002/ECR-028 | exact accounting + hard/soft limit tests |
| Process/output/network/storage budgets | OWNED | ECR-002/ECR-017/ECR-018 | exhaustion/cleanup tests |
| Delegation/recursion depth | OWNED | ECR-002/ECR-003/ECR-016 | recursive-agent/tool termination tests |

## 4. Verification / Evidence Truth

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| Executor receipt vs verifier result | OWNED | ECR-001/ECR-004 | type separation + false-completion fixtures |
| Fact duplicate verified truth | OWNED | ECR-001/ECR-004 | Fact has no independent verified flag; VerificationReceipt aggregation tests |
| Critical-point verification | OWNED | ECR-004/ECR-005 | long-task constraint violation scenarios |
| Mutable external evidence | OWNED | ECR-001/ECR-004/ECR-009 | snapshot/hash/as-of requirements for decision-grade verification |
| Verifier false positives/negatives | OWNED | ECR-005/ECR-028 | labeled verifier corpus/metrics |
| Verifier capture by malicious evidence | OWNED | ECR-004/ECR-005 | adversarial evidence tests |
| Independent source corroboration | OWNED | ECR-009/ECR-028 | source-lineage/copy-cascade fixtures |

## 5. Firefox / Browser / Human Product

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| Stock Firefox/BiDi before fork | OWNED | ECR-006 | bounded live/fixture prototype |
| Firefox foundation maintenance cost | OWNED | ECR-007 | patch/update/rebase/security-SLA ledger |
| Privileged browser IPC authentication | OPEN-RESEARCH | ECR-007 | OS ACL/peer identity/channel-binding/replay/fuzz contract |
| Generic remote-debug superuser path | OWNED | ECR-007 | prohibited-by-default contract/tests |
| Containers mistaken as full sandbox | OWNED | ECR-008 | storage/session partitioning wording + Ecra-policy isolation tests |
| Agent origin transitions | OWNED | ECR-003/ECR-006 | re-authorization/disclosure tests |
| Browser permission broker | OWNED | ECR-003/ECR-006/ECR-008 | clipboard/file/camera/mic/location/notifications/payment/WebAuthn matrix |
| Human-presence/non-delegable operations | OWNED | ECR-003/ECR-008 | policy + trusted-chrome acceptance tests |
| Browser extension trust tiers | OWNED | ECR-007/ECR-008 | compatibility/trust/modified-content tests |
| Trusted approval/agent UI anti-spoofing | OWNED | ECR-008 | native chrome spoofing tests/user study |
| Human/agent/shared tab ownership | OWNED | ECR-008 | conflict/takeover/hand-back tests |
| Background focus theft | OWNED | ECR-008 | no unexpected focus changes |
| Background audio/fullscreen/popups/permissions/notifications/download/clipboard | OWNED | ECR-008 | background-effect policy tests |
| Normal browsing without model | OWNED | ECR-008 | model-off smoke suite |
| Extension/profile migration compatibility | OWNED | ECR-007/ECR-029 | import/smoke tests |
| Accessibility | OWNED | ECR-026 | keyboard/screen-reader/contrast/semantic checks |
| i18n/RTL/localization | OWNED | ECR-026 | extraction/locale/RTL tests |
| Cross-platform desktop parity | OWNED | ECR-007/ECR-024 | Windows/macOS/Linux matrix |
| Mobile | DEFERRED | future amendment | desktop wedge evidence first |

## 6. Search / Trusted Information / Content

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| Unified evidence contract | OWNED | ECR-009 | provider contract tests |
| Primary/official/community/source type | OWNED | ECR-009 | ranking/source metadata fixtures |
| Source identity + copy lineage/independence | OWNED | ECR-009 | citation-laundering/copy-cascade fixtures |
| Source snapshot/hash/as-of | OWNED | ECR-009 | changed-source detection tests |
| Freshness basis | OWNED | ECR-001/ECR-009 | stale/current/unknown benchmark |
| Contradiction visibility | OWNED | ECR-009 | conflicting-primary-source fixtures |
| Claim→source coverage | OWNED | ECR-009/ECR-028 | evidence coverage metrics |
| Local/private search | OWNED | ECR-009 | offline/local search test |
| Query egress/minimization | OWNED | ECR-003/ECR-009 | private-query remote-provider denial/redaction |
| Hybrid retrieval | OWNED | ECR-009 | lexical/structural/semantic benchmark |
| Cache invalidation | OWNED | ECR-009/ECR-027 | freshness/change tests |
| robots/access/publisher policy | OWNED | ECR-027 | source access policy contract |
| copyright/license/attribution/retention | OWNED | ECR-027 | metadata/policy compliance fixtures |
| Safe download handling | OWNED | ECR-017/ECR-027 | quarantine/sandbox tests |
| Dangerous parser/archive/PDF/media workloads | OWNED | ECR-017/ECR-027 | resource-bounded hostile corpus |
| Provider quotas/backoff | OWNED | ECR-009 | quota/rate/backoff tests |
| Google-scale crawler/index | DEFERRED | future strategy | not required for first trusted search wedge |

## 7. Workspace / Memory

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| Workspace as context + authority scope | OWNED | ECR-010 | cross-workspace isolation tests |
| Candidate vs accepted memory | OWNED | ECR-010 | trust-transition tests |
| Provenance/classification memory | OWNED | ECR-010 | source/label retrieval tests |
| Memory self-authorization | OWNED | ECR-003/ECR-010 | poisoned memory permission tests |
| Aging/staleness/conflict/versioning | OWNED | ECR-010 | temporal/conflict fixtures |
| Delete from derived FTS/vector/cache/summary | OWNED | ECR-010/ECR-029 | deletion propagation + rebuild tests |
| Export/import portability | OWNED | ECR-029 | round trip |
| Cross-device encrypted sync | DEFERRED | ECR-022 | trust/key recovery spec first |
| Team/shared multi-principal governance | DEFERRED | future spec | single-user identity/policy stable first |

## 8. Skills / Replay / Repair

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| Typed artifact/dataflow Skill IR | OWNED | ECR-012 | schema/static validation |
| Preconditions/postconditions/assumptions | OWNED | ECR-012 | compatibility fixtures |
| Capability requirements | OWNED | ECR-012 | validation tests |
| Captured grants/approval tokens/secrets in skill | OWNED | ECR-012/ECR-013 | must be rejected/stripped; negative fixtures |
| Information-flow requirements in skill | OWNED | ECR-012 | source/sink static validation |
| Human demo→skill | OWNED | ECR-013 | deterministic compilation fixture |
| Agent run→same IR | OWNED | ECR-013 | equivalence fixture |
| Sandbox validation before promotion | OWNED | ECR-013/ECR-017 | promotion gate |
| Zero/no-model replay | OWNED | ECR-014 | replay benchmark |
| Fresh authorization on replay | OWNED | ECR-003/ECR-014 | expired/revoked grant tests |
| Localized repair | OWNED | ECR-015 | drift corpus |
| Repair policy/label weakening | OWNED | ECR-015 | negative policy-drift tests |
| Skill version/rollback | OWNED | ECR-012/ECR-015 | version fixtures |
| Registry signing/reputation | DEFERRED | ECR-023 | dedicated threat model |

## 9. Protocols / Plugins / Models

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| MCP/ACP/A2A adapters | OWNED | ECR-016 | version-pinned conformance tests |
| MCP audience/resource/issuer binding | OWNED | ECR-016/ECR-031 | auth mapping + confused-deputy tests |
| Token passthrough | OWNED | ECR-016 | prohibited-by-default tests |
| Agent Skills import/export | OWNED | ECR-016 | round-trip without conflating Ecra Skill IR |
| WebMCP origin/tool/principal binding | OPEN-RESEARCH | ECR-011/ECR-016 | provenance/authority contract |
| Plugin capability manifest | OWNED | ECR-017 | deny-by-default tests |
| Wasm/process/VM sandbox tiers | OPEN-RESEARCH | ECR-017 | OS-specific threat/perf tests |
| Sandbox escape/resource exhaustion | OWNED | ECR-017/ECR-005 | hostile fixtures/advisory response |
| Plugin/skill signing/registry | DEFERRED | ECR-023 | key/reputation/rollback design |
| Local model provider-neutral interface | OWNED | ECR-021 | mock/provider tests |
| Model artifact provenance/hash/license | OWNED | ECR-021/ECR-024 | manifest verification |
| Executable custom loader / `trust_remote_code` | OWNED | ECR-017/ECR-021 | deny-by-default/sandbox tests |
| Tokenizer/chat-template provenance | OWNED | ECR-021 | manifest/digest tests |
| GPU/native library/resource isolation | OWNED | ECR-017/ECR-021 | resource/fault tests |
| Custom model training | DEFERRED | future research | verified corpus/evaluation bottleneck first |

## 10. Terminal / Developer / Data

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| Shell/process authority | OWNED | ECR-018 | cwd/fs/network/process scope tests |
| Process tree cleanup/timeouts/output limits | OWNED | ECR-018 | cancellation/resource tests |
| Untrusted repo inspection vs execution | OWNED | ECR-019/ECR-017/ECR-018 | build-hook/package-script sandbox tests |
| Repository structural/current-doc context | OWNED | ECR-019 | retrieval benchmark |
| Build/test/browser-QA receipts | OWNED | ECR-019 | reproducible workflow fixture |
| Git push/release high-impact gates | OWNED | ECR-003/ECR-019 | approval/verification tests |
| SQL/data lineage | OWNED | ECR-020 | claim→query→source trace |
| Data quality/uncertainty | OWNED | ECR-020 | validation/warning tests |
| Notebook/runtime isolation | OWNED | ECR-017/ECR-020 | sandbox tests |
| Data egress to model/tools | OWNED | ECR-003/ECR-020 | information-flow tests |

## 11. Supply Chain / Operations / Lifecycle

| Gap | Status | Owner | Required evidence |
|---|---|---|---|
| CI baseline | OWNED | ECR-001/ECR-024 | fmt/clippy/test/audit |
| Dependency advisory monitoring | OWNED | ECR-024 | CI/security automation |
| SBOM | OWNED | ECR-024 | release artifact |
| Artifact-specific reproducibility | OWNED | ECR-024 | separate Rust/Firefox distribution reports |
| Signing/provenance/update channels | OWNED | ECR-024/ECR-031 | verify/rollback drill |
| Emergency Firefox security update path | OWNED | ECR-007/ECR-024 | SLA/process test drill |
| Donor/source provenance | CROSS-CUTTING | every donor-using slice/ECR-024 | exact ledger entries |
| Feature flags/experimental labeling | OWNED | affected slices/ECR-024 | default/retirement policy |
| Doctor/support bundle | OWNED | ECR-025 | redaction/offline diagnostic tests |
| Brand/trademark clearance | OPEN-RESEARCH | founder/legal | before public launch/rename commitment |
| Business model vs local-first/privacy | OPEN-RESEARCH | product/business | must not require constitutional regression |

## 12. Benchmark / Claim Coverage

| Claim | Required evidence | Owner |
|---|---|---|
| secure agent execution | capability + egress + prompt injection + identity + UI-spoof tests | ECR-005/ECR-028 |
| reliable long-horizon work | task/constraint/crash-resume/duplicate-effect/budget metrics | ECR-005/ECR-028 |
| trusted search | evidence/provenance/freshness/source-independence/unsupported-claim metrics | ECR-009/ECR-028 |
| better human-agent collaboration | takeover/correction/approval-fatigue/trusted-UI metrics | ECR-008/ECR-028 |
| cheaper reusable work | compile/replay/repair/model-call/cost metrics | ECR-013–ECR-015/ECR-028 |
| local model uplift | matched workflow quality/cost/privacy/resource comparison | ECR-021/ECR-028 |
| private/local-first | network/telemetry/offline/export/deletion evidence | ECR-025/ECR-029 |
| tamper-resistant/tamper-evident | explicitly scoped integrity/adversary model + protected-anchor tests where claimed | ECR-002/ECR-031/ECR-028 |

## 13. Audit Decision

The pre-implementation review found no major category without an owner after this revision.

Remaining `OPEN-RESEARCH` items are deliberately owned and do **not** block ECR-001 unless they would be smuggled into its implementation. ECR-001's own critical modeling gaps are covered by its revised Spec Kit package and still require a final analyze pass before implementation.
