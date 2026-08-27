# ECR-001 Status — Trusted Domain Kernel

**Slice:** ECR-001  
**Lifecycle:** CLOSED_CANONICAL  
**Branch:** `001-trusted-domain-kernel` — merged  
**PR:** `#1` — MERGED  
**Final feature head:** `1d3c319c3317d3572baad1784f18eea771c5ac6e`  
**Final feature-head CI:** `33098892820` — success  
**Merge commit:** `d1021616eae721e0b89bd5d4114531c4b9cc8a58`  
**Post-merge main CI:** `33099033214` — success  
**Review state:** all actionable Qodo threads resolved/outdated; CodeRabbit success before merge

This is the canonical closure ledger required by `AGENTS.md`. Normative semantics live in `spec.md`, converged `data-model.md`, `contracts/domain-v1.md`, implementation truth on canonical `main`, and the evidence recorded below.

## Canonical closure

```text
Phases 1–10  VERIFIED_ON_BRANCH_AND_MERGED
Phase 11     T070–T076 COMPLETE
Phase 12     T077–T080 COMPLETE
Phase 13     T081–T084 COMPLETE
Lifecycle    CLOSED_CANONICAL
```

T076 is satisfied because PR #1 merged from the exact fully verified feature head and the resulting canonical merge commit passed the complete post-merge main gate.

## Final pre-merge evidence

```text
Head:   1d3c319c3317d3572baad1784f18eea771c5ac6e
Run:    33098892820
Runner: macbook
Result: SUCCESS
```

The exact feature head passed checkout, pinned Rust installation, locked build, formatting, strict Clippy, full workspace tests, all eight dedicated contract/security targets, rustdoc tests, offline replay, unsafe boundary, dependency boundary and `cargo tree -p ecra-core`.

## Merge and post-merge evidence

```text
PR:           #1 — MERGED
Merge commit: d1021616eae721e0b89bd5d4114531c4b9cc8a58
Main run:     33099033214
Runner:       macbook
Result:       SUCCESS
```

Canonical `main` at the merge commit passed the same complete gate surface after the merge. This proves the merged implementation rather than only the feature branch.

## Review-remediation disposition

- strict public `Versioned<T>` deserialization rejects unsupported major/newer minor versions;
- typed compatibility errors remain available from `Versioned::from_json_slice`;
- Fact integer/canonical-decimal construction is fail-closed and strict-round-trippable;
- platform lifecycle state is synchronized;
- Phase 13 task entries include exact implementation/test path traceability;
- all actionable review threads were resolved or became outdated by the remediating commits before merge.

Golden ActionDigest remains:

```text
6ccf10a5d1db36cb9637b4ed2ee6d3f0167c44eec585b543f58d86287710e3d4
```

## CI security posture

Repository CI currently uses the approved repository-scoped self-hosted macOS runner `macbook` because GitHub-hosted private-repository execution is blocked by the owner account's `$0` Actions budget with stop-usage enabled.

```text
push: 001-trusted-domain-kernel -> trusted feature-head verification
push: main                      -> canonical-main verification
workflow_dispatch               -> explicit recovery/recheck
concurrency                      -> cancel superseded same-ref work
runs-on                          -> self-hosted
permissions                     -> contents: read
```

Do not restore untrusted `pull_request` execution on the persistent self-hosted machine without an explicit security design.

## Closure boundary

ECR-001 intentionally does not implement authentication/trust roots, authorization/declassification, durable run lifecycle/budgets/persistence, verifier orchestration/reconciliation, browser/model/tool execution or protocol adapters. Those remain owned by ECR-031/ECR-003/ECR-002/ECR-004 and dependent slices.

ECR-001 is now a closed dependency. Subsequent work must re-read the live platform roadmap and begin only the next genuinely eligible bounded slice.
