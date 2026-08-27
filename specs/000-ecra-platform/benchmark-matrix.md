# Ecra Cross-Phase Benchmark Matrix

**Date:** 2026-08-27  
**Status:** CANONICAL_PLANNING

Ecra does not make platform-superiority claims from a single browser benchmark. Evaluation must match the claim and test the trusted substrate as well as end-task success.

## Metric Families

### Correctness
- task success;
- constraint retention;
- artifact correctness;
- structured-output correctness;
- long-horizon completion.

### Trust / Information Quality
- evidence coverage;
- provenance coverage;
- unsupported-claim rate;
- freshness error rate;
- contradiction detection/visibility;
- primary-source usage where appropriate.

### Verification
- verifier false-positive rate;
- verifier false-negative rate;
- inconclusive calibration;
- critical-point violation detection;
- process-vs-outcome disagreement handling.

### Security
- prompt-injection attack success rate;
- cross-origin leakage;
- capability overreach;
- secret exposure;
- memory-poisoning persistence;
- plugin/sandbox escape fixtures;
- unauthorized retrieval/context inclusion.

### Durability
- crash/resume success;
- duplicate side-effect rate;
- UNKNOWN-outcome preservation;
- reconciliation accuracy;
- cancellation correctness.

### Human-Agent UX
- takeover latency;
- hand-back recovery;
- intervention precision;
- unnecessary approval rate;
- agent-focus theft/background-task disruption;
- user understanding of current authority/control owner.

### Reuse / Skill Economics
- compile yield;
- replay success;
- zero-model replay rate;
- model calls avoided;
- repair success;
- repair locality;
- skill-version stability;
- cost/time per repeated successful task.

### Browser Product Quality
- startup time;
- navigation/page interaction overhead vs upstream baseline;
- memory/CPU overhead;
- crash rate;
- extension/profile compatibility;
- accessibility;
- battery/energy impact when measurable.

### Model Augmentation
- workflow success with Ecra context/actions vs same model unaided;
- smaller/local model + Ecra vs larger unaided model on bounded workflows;
- token/cost/latency/privacy tradeoff;
- provenance and verification quality, not only answer accuracy.

## External Benchmark Families

External benchmarks are adapters, not the definition of Ecra quality. Exact versions/tasks must be pinned in reports.

| Family | Intended Ecra use | Not sufficient for |
|---|---|---|
| BrowserGym / AgentLab ecosystem | reproducible browser-agent evaluation harness | security, daily-browser UX, Ecra Skill economics |
| WebArena-Verified-style tasks | deterministic browser outcome verification | live-web drift, human collaboration |
| Online-Mind2Web | live-web robustness/site drift | consequential side effects or Ecra-specific security |
| OSWorld 2.0 | long-horizon constraint/state evaluation | browser-only product quality |
| WeaveBench-style GUI+CLI tasks | cross-surface developer/workflow evaluation | search trust or privacy |
| WASP / BrowseSafe / StepJack / AgentLAB families | prompt-injection/adversarial browsing | full origin/capability security alone |
| SOPBench / agent same-origin research | cross-origin leakage and information-flow boundaries | prompt injection alone |
| BrowseComp / research-search tasks | difficult retrieval/research | source-policy/compliance and workspace search |
| Mind2Web 2-style tasks | long-horizon research/action behavior | daily-browser product quality |

Dataset/benchmark licenses and terms must be reviewed before vendoring or redistributing fixtures.

## Internal Mandatory Suites

Ecra must own internal deterministic suites because external benchmarks cannot express its constitutional contracts.

### Suite A — Trusted Core Contracts
Owner: ECR-001

- valid/invalid domain fixtures;
- canonicalization;
- capability request/grant separation;
- action semantic invalid combinations;
- receipt/verification separation.

### Suite B — Run Durability
Owner: ECR-002/ECR-004

Fault injection at:
- before dispatch;
- after dispatch before acknowledgement;
- after external commit before receipt persistence;
- after receipt before verification;
- during human takeover/cancellation.

Measure duplicate side effects and UNKNOWN handling.

### Suite C — Authority / Origin Security
Owner: ECR-003/ECR-005/ECR-006

- cross-origin instructions;
- hidden/visible injected content;
- origin transitions;
- capability delegation/narrowing;
- secret handle misuse;
- retrieval scope leaks.

### Suite D — Browser Human/Agent Collaboration
Owner: ECR-008

- human tab and agent tab concurrency;
- shared-tab control ownership;
- takeover/hand-back;
- background agent work;
- approval UX;
- unauthorized container attempts.

### Suite E — Trusted Search
Owner: ECR-009

Curated cases with:
- primary vs secondary sources;
- stale vs current sources;
- contradictions;
- insufficient evidence;
- workspace/private + web combined queries;
- claim-to-source entailment checks.

### Suite F — Memory Poisoning and Recovery
Owner: ECR-010

- malicious page asks to persist instructions;
- stale fact returns after correction;
- contradictory facts;
- workspace isolation;
- deletion/export round-trip.

### Suite G — Skills
Owner: ECR-012–ECR-015

- human vs agent demonstration to same semantic IR;
- deterministic compatibility;
- zero-model replay;
- controlled UI/site drift;
- localized repair;
- policy invariant preservation across repair.

### Suite H — Gateway / Plugins
Owner: ECR-016/ECR-017

- protocol conformance;
- least-authority adapter exposure;
- plugin capability denial;
- resource exhaustion;
- malicious plugin fixtures;
- protocol caller cannot reach privileged browser bridge implicitly.

### Suite I — Developer / Data
Owner: ECR-018–ECR-020

- bounded shell/process lifetime;
- repo/test/browser QA chain;
- claim → computation/query → source data lineage;
- unknown/failed computation never emitted as verified result.

## Benchmark Report Requirements

Every published/internal decision-grade report must record:

```text
Ecra commit SHA
browser/version when relevant
OS/platform
model/provider/version when relevant
benchmark/dataset version
exact task IDs or selection procedure
configuration and capability policy
number of runs / seeds
success definition
verifier version
cost/token accounting method
known exclusions/failures
raw or reproducible result artifacts where licensing permits
```

## Claim Gates

### “More reliable”
Requires statistically meaningful improvement on relevant success + verifier false-positive + durability metrics, not only anecdotal demos.

### “More secure”
Requires scoped threat-model evidence and security benchmark improvements. No claim may imply immunity from prompt injection/sandbox escape.

### “Cheaper for repeated work”
Requires compile/replay cohorts showing model-call/token/cost reduction while maintaining verification success.

### “Better trusted search”
Requires claim-evidence/freshness/contradiction metrics, not answer preference alone.

### “Makes local models smarter/more capable”
Requires matched workflow experiments against the same model unaided and relevant larger-model baselines; claim scope must match evaluated tasks.

## Regression Policy

Once a benchmark becomes a release gate:

- baseline and tolerance are versioned;
- deliberate threshold changes require rationale;
- benchmark changes cannot be used to erase an unfavorable regression without reporting both old/new results;
- security/durability zero-tolerance invariants (e.g. unauthorized capability grant, duplicate consequential side effect in deterministic fixture) remain blockers regardless of average score.
