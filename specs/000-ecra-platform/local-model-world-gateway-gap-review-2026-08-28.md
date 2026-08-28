# Local Model World Gateway — Planning Gap Review

**Date:** 2026-08-28  
**Status:** CANONICAL_PLANNING_SUPPLEMENT  
**Scope:** downstream planning only; no ECR-002 scope expansion  
**Primary research:** `research/local-model-world-gateway.md`

This supplement records the gap review triggered by studying WebMCP tooling and OpenSandbox against Ecra's existing platform architecture. It supplements `gap-audit.md` without changing immutable ECR IDs.

## Result

**NO_NEW_ECR_REQUIRED.**

The existing dependency graph can own the world-gateway direction if responsibilities are made explicit across ECR-003/004/005/009/010/011/016/017/021/024/028/031. The review found planning gaps in the older wording, but no reason to pull downstream implementation into ECR-002.

## Gap Matrix

| Gap | Prior state | Resolution | Owner | Required future evidence |
|---|---|---|---|---|
| Local model gateway could be read as only an inference-provider adapter | PARTIAL | ECR-021 is now explicitly the model-facing convergence point for context compilation, tool aperture, compatibility adapters and mediated world access | ECR-021 | provider-neutral mocks; context/tool profiles; end-to-end local-model fixture |
| “Ecra is the gate” lacked an enforceable runtime profile | MISSING | high-assurance profile may deny direct network/credential/ambient-tool access and expose bounded Ecra APIs | ECR-017/ECR-021/ECR-005 | direct-egress, process, filesystem and alternate-channel bypass tests |
| Local model context was not explicitly treated as an information sink | PARTIAL | local/remote model context is subject to scope/classification/use policy | ECR-003/ECR-009/ECR-010/ECR-021 | cross-workspace/principal/use denial fixtures |
| Search output lacked a named model-agnostic evidence/context assembly boundary | PARTIAL | ECR-009 research now owns `EvidencePack` and `ContextProjection` concepts | ECR-009 | canonical contract; lineage/freshness/contradiction/drill-down fixtures |
| Compression could destroy provenance or claim meaning | MISSING | context compiler must preserve source/classification lineage and support progressive disclosure | ECR-009/ECR-021/ECR-005 | golden compression, claim support and drill-down tests |
| Broad tool exposure could overwhelm weaker local models | MISSING | Adaptive Tool Aperture is an explicit ECR-021/ECR-011 evaluation/design responsibility | ECR-011/ECR-016/ECR-021/ECR-028 | same-model broad-vs-bounded tool evaluations; wrong-tool rate |
| WebMCP structured tools could be mistaken for trusted authority | PARTIAL | structured site capabilities are origin-bound `CapabilityOffer`s, never grants | ECR-011/ECR-003/ECR-016 | malicious structured-tool corpus; origin/schema/parameter mutation tests |
| Route fallback could bypass a denied semantic action | OWNED but implicit for WebMCP | same concrete ActionRef/policy gate applies regardless of WebMCP/API/Skill/DOM/BiDi/vision route | ECR-011/ECR-003/ECR-005 | denied semantic action cannot execute via lower-level fallback |
| Sandbox backend could become a second trust model | MISSING | sandbox is a replaceable enforcement mechanism; Ecra owns identity/info-flow/authorization/action/receipt/verification | ECR-017/ECR-003/ECR-031 | conformance tests across backends; stale-policy/revocation tests |
| Credential mediation lacked an explicit local-agent architecture target | PARTIAL | secret handles resolve at trusted outbound/provider boundary with narrow audience/destination/operation binding where supported | ECR-003/ECR-017/ECR-031/ECR-025 | no raw secret in model/workload/log; wrong-destination/path/method denial |
| OpenAI-compatible/MCP adapters could collapse richer Ecra semantics | MISSING | compatibility protocols remain adapters; Ecra-specific evidence/action/verification contracts remain authoritative | ECR-016/ECR-021 | lossy-adapter tests/documented unsupported semantic mappings |
| Model artifact/runtime isolation and world-access isolation were conflated | PARTIAL | ECR-021 owns model artifact/profile semantics; ECR-017 owns execution/isolation backend mechanics | ECR-017/ECR-021/ECR-024 | malicious loader/resource tests + independent egress tests |
| Memory relevance could become automatic model disclosure | PARTIAL | relevant memory remains candidate context subject to scope/provenance/classification/freshness/use policy | ECR-003/ECR-010/ECR-021 | vector-similar but disallowed memory never reaches context |
| Local-model uplift had no canonical comparison design | PARTIAL | ECR-028 now owns Effective Intelligence Gain against same-model baselines | ECR-005/ECR-021/ECR-028 | exact model/runtime/task/config reports; no generalization beyond evidence |
| Donor adoption could create architecture capture | OWNED generally | WebMCP-tools and OpenSandbox are ledgered as reference/integration candidates only; no source reuse or policy ownership implied | ECR-011/ECR-017/ECR-024 | exact version/license/advisory/API review before dependency/integration |

## Donor-Specific Conclusions

### GoogleChromeLabs/webmcp-tools

Useful for:
- structured website capability ergonomics;
- developer inspection/debugging patterns;
- static schema evaluation;
- live browser evaluation;
- deterministic smoke testing;
- local-model/Ollama tool-call evaluation methodology.

Not adopted as:
- Ecra authority semantics;
- Ecra action/receipt/verification model;
- a reason to trust page-provided tool descriptions.

### opensandbox-group/OpenSandbox

Useful for:
- sandbox lifecycle/API patterns;
- command/filesystem/code execution isolation;
- default-deny egress mechanics;
- stronger runtime integration references;
- outbound credential-injection pattern.

Not adopted as:
- Ecra identity or policy authority;
- Ecra run/receipt/verification truth;
- a mandatory trusted-core dependency;
- proof that every supported OS/backend can provide identical containment.

## Dependency Review

No dependency edge changes are required.

ECR-021 already depends on ECR-009–ECR-017 and ECR-024. Those dependencies transitively gate ECR-021 behind the evidence, authority, verification, identity, protocol, skill and sandbox contracts required for safe privileged local-agent use.

Adding a new ECR solely for the context compiler or information gateway would duplicate ownership and weaken the existing convergence design.

## ECR-002 Boundary Check

The active ECR-002 implementation remains unchanged in scope.

This review does **not** authorize ECR-002 to add:
- model providers;
- search;
- WebMCP;
- network access;
- secret mediation;
- sandbox backends;
- authorization/policy;
- verification decisions;
- real sensitive persistence.

ECR-002 continues to own durable run/attempt/ledger/budget/portable synthetic artifact semantics only.

## Planning Closure Criteria

This review is considered integrated only when the same boundaries appear in:

- `research/local-model-world-gateway.md`;
- `specs/000-ecra-platform/roadmap.md`;
- `specs/000-ecra-platform/architecture.md`;
- `specs/000-ecra-platform/decision-log.md`;
- `specs/000-ecra-platform/threat-model.md`;
- `specs/000-ecra-platform/risk-register.md`;
- `research/donor-license-ledger.md`.

Implementation remains governed by each downstream Spec Kit package when it becomes dependency-eligible.
