# Ecra Cross-Phase Benchmark Matrix

**Date:** 2026-08-27  
**Status:** CANONICAL_PLANNING_V2  
**Review source:** `pre-implementation-review-2026-08-27.md`

Ecra does not make platform-superiority claims from one browser benchmark. Evaluation must match the exact claim and test the trusted substrate, information flow and human product as well as end-task success.

## Metric Families

### Correctness / Long Horizon
- task success;
- constraint retention;
- artifact correctness;
- structured-output correctness;
- critical-point violation rate;
- long-horizon completion.

### Trust / Information Quality
- evidence/provenance coverage;
- unsupported-claim rate;
- freshness error/calibration;
- contradiction visibility;
- primary/official source usage where appropriate;
- source independence/copy-cascade rate;
- citation/source-change detection.

### Identity / Authority
- unauthenticated Actor→Principal acceptance rate (target zero);
- implicit wildcard scope rate (zero);
- capability overreach;
- stale/revoked authorization acceptance;
- approval replay/ActionDigest mismatch acceptance;
- delegation/on-behalf-of confusion rate.

### Information Flow / Privacy
- cross-origin leakage;
- cross-workspace leakage;
- private-query remote-provider leakage;
- unauthorized model-context inclusion;
- secret exposure;
- log/telemetry disclosure violations;
- derived-data declassification errors;
- deletion-residue retrieval rate.

### Verification
- false-positive / false-negative rate;
- inconclusive calibration;
- executor-success vs verified-outcome disagreement handling;
- mutable-evidence/source-change handling;
- critical-point violation detection;
- verifier-capture/adversarial-evidence rate.

### Durability / Side Effects
- crash/resume success;
- duplicate side-effect rate;
- UNKNOWN preservation;
- reconciliation accuracy;
- ActionAttempt/receipt audit completeness;
- cancellation correctness.

### Bounded Execution
- wall-time budget enforcement;
- max step/tool/model-call enforcement;
- token/cost budget enforcement;
- process-tree cleanup;
- output/network/storage limit enforcement;
- recursive delegation termination;
- safe behavior when budget is exhausted during UNKNOWN side effect.

### Human-Agent UX / Trusted UI
- takeover/hand-back latency and recovery;
- intervention precision;
- unnecessary approval rate / approval fatigue;
- user understanding of current actor/control/authority;
- trusted-chrome spoof resistance;
- focus/audio/fullscreen/popup/permission/notification/download/clipboard disruption rate.

### Reuse / Skill Economics
- compile yield;
- replay success;
- zero-model replay rate;
- model calls/tokens/cost avoided;
- repair success/locality;
- authorization/policy preservation after repair;
- captured-authority rejection rate;
- skill-version stability.

### Browser Product Quality
- startup/navigation/UI latency vs upstream baseline;
- memory/CPU/energy overhead;
- crash rate;
- extension/profile compatibility;
- permission behavior;
- accessibility/i18n;
- security-update lag/patch delta size.

### Local Model Augmentation
- same local model + Ecra vs unaided;
- smaller/local + Ecra vs larger unaided on bounded workflows;
- token/cost/latency/privacy/resource tradeoff;
- artifact/provenance/security setup failure rate;
- verification/evidence quality.

## External Benchmark Families

External benchmarks are adapters, not Ecra's definition of quality. Pin exact versions/tasks/licenses in reports.

| Family | Intended use | Not sufficient for |
|---|---|---|
| BrowserGym / AgentLab | reproducible browser-agent harness | Ecra security/daily-browser UX/skill economics |
| WebArena-Verified style | deterministic outcome evaluation | live drift/human collaboration/egress |
| Online-Mind2Web | live-web robustness | consequential side-effect security |
| OSWorld 2.0 | long-horizon constraints/state | trusted search/browser product quality |
| WeaveBench-style GUI+CLI | cross-surface workflows | search trust/privacy |
| WASP / BrowseSafe / StepJack / AgentLAB families | prompt-injection/adversarial content | identity/source-to-sink authorization alone |
| SOPBench / agent same-origin research | cross-origin information flow | prompt injection alone |
| BrowseComp / research search | hard retrieval/research | source compliance/private workspace search |
| Mind2Web 2-style | long-horizon browser/research action | normal daily-browser quality |

## Internal Mandatory Suites

### Suite A — Trusted Domain Contract (ECR-001)

Must include:
- Actor/Principal/IdentityAssertion type separation;
- explicit ScopeConstraint wildcard tests;
- ResourceId vs locator;
- Request/Grant distinct IDs/types;
- information classification + InformationUse shape;
- Fact without duplicate verified truth;
- freshness basis;
- mutation/reversibility/idempotency/retry matrix;
- fixed ActionDigest mutation tests;
- multiple attempts per action;
- receipt vs verification separation;
- strict schema/canonicalization/dependency/unsafe tests.

Zero-tolerance failures: implicit wildcard, request→grant, Actor→Principal authentication, ActionDigest mismatch accepted, receipt→verification cast.

### Suite B — Identity / Trust Root (ECR-031)

- invalid/expired/revoked assertions;
- actor↔principal/on-behalf-of mismatch;
- key rotation/revocation;
- local trust-root rollback/replay;
- protected-storage authenticity/decryption failure;
- OS peer/client identity assumptions.

### Suite C — Authority / Egress (ECR-003/ECR-005)

- capability intersection/narrowing;
- ActionRef-bound approvals;
- stale policy/grant/revocation;
- private source + allowed remote tool but denied disclosure;
- derived sensitive data;
- remote model/search/plugin/log/telemetry sinks;
- secret-handle misuse;
- origin transitions.

Zero-tolerance deterministic fixtures: unauthorized grant or unauthorized disclosure.

### Suite D — Run Durability / Budgets (ECR-002/ECR-004/ECR-005)

Fault injection:
- before dispatch;
- after dispatch before acknowledgement;
- after external commit before receipt;
- after receipt before verification;
- during cancellation/takeover;
- at each budget-exhaustion boundary.

Measure duplicate side effects, UNKNOWN handling, reconciliation and process/resource cleanup.

### Suite E — Browser Security + Collaboration (ECR-006–ECR-008)

- cross-origin injected instructions;
- authenticated IPC spoof/replay attempts;
- Containers vs Ecra authority isolation;
- human/agent/shared tab concurrency;
- takeover/hand-back;
- trusted-chrome spoof attempts;
- WebAuthn/passkey/clipboard/file/camera/mic/location/payment/permission broker;
- broad browser-extension interference;
- background focus/audio/fullscreen/popup/notification/download/clipboard effects;
- normal model-off browsing.

### Suite F — Trusted Search (ECR-009)

Curated cases:
- primary vs secondary sources;
- copied/citation-laundered source families;
- stale/current/changed source snapshots;
- contradictions;
- insufficient evidence;
- private workspace + web combined query;
- private query denied/redacted before remote provider;
- claim→source entailment and evidence coverage;
- malicious/oversized documents through parser boundary.

### Suite G — Memory (ECR-010)

- malicious page asks to persist instructions;
- sensitive derived memory;
- stale fact after correction;
- contradiction/versioning;
- cross-workspace retrieval;
- delete propagates through FTS/vector/cache/summary projections;
- export/import round trip.

### Suite H — Skills / Replay / Repair (ECR-012–ECR-015)

- human vs agent demo to same semantic IR;
- captured grant/approval/secret rejected;
- dataflow/information-flow static validation;
- zero-model replay;
- fresh authorization after expiry/revocation;
- controlled UI/site drift;
- local repair;
- downstream invalidation;
- policy/classification invariants preserved after repair.

### Suite I — Protocol / Plugin (ECR-016/ECR-017)

- pinned protocol conformance;
- MCP resource/audience/issuer identity mapping;
- token-passthrough/confused-deputy negatives;
- least-authority state views;
- plugin capability denial;
- sandbox escape/resource exhaustion fixtures;
- protocol caller cannot reach privileged browser bridge implicitly.

### Suite J — Terminal / Developer / Data (ECR-018–ECR-020)

- bounded shell/process tree/output/network;
- inspect untrusted repo without executing it;
- sandbox malicious build/test/install hook;
- Git/release high-impact approval + verification;
- browser QA tied to code state;
- data claim → computation/query → source lineage;
- data egress restrictions;
- failed/UNKNOWN computation never emitted as verified conclusion.

### Suite K — Local Model Artifact / Uplift (ECR-021)

- model manifest/source/license/hash;
- tokenizer/template integrity;
- custom-loader/custom-code denial by default;
- GPU/RAM/time limits;
- malformed/malicious model artifact handling;
- matched workflow uplift experiments with provenance/verification metrics.

## Benchmark Report Requirements

Decision-grade report records:

```text
Ecra exact commit SHA
active spec/contract version
browser/version + patch delta when relevant
OS/platform/hardware
model/provider/artifact hash/version when relevant
protocol spec version when relevant
benchmark/dataset version + license
exact task IDs/selection procedure
identity/capability/information-flow policy configuration
resource budgets
number of runs/seeds
success + verification definition
verifier version/evidence policy
cost/token/resource accounting
known exclusions/failures
raw/reproducible artifacts where licensing permits
```

## Claim Gates

### “More reliable”
Requires task success plus verifier FP, durability, UNKNOWN/duplicate-effect and bounded-execution evidence.

### “More secure”
Requires a scoped threat model and identity/authority/egress/prompt-injection/trusted-UI/supply-chain tests. Never imply immunity.

### “Private”
Requires measured network/telemetry/remote-provider egress behavior, deletion/export evidence and explicit limitations under local OS compromise.

### “Tamper-evident/resistant”
Requires exact adversary model. A recomputable hash chain alone cannot support hostile-tamper-resistance wording.

### “Better trusted search”
Requires evidence/freshness/contradiction/source-independence and private-query egress metrics, not answer preference alone.

### “Cheaper repeated work”
Requires compile/replay cohorts showing cost/model-call reduction at maintained verification success.

### “Makes local models more capable”
Requires matched same-model unaided comparison plus relevant larger baseline, resource/privacy tradeoff and scoped claim wording.

## Regression Policy

Once a benchmark is a release gate:
- baseline/tolerance is versioned;
- threshold change requires rationale;
- harness change cannot erase an unfavorable regression without dual reporting;
- zero-tolerance security/durability invariants remain blockers regardless of averages.
