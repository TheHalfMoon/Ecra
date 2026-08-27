# ECR-001 Status — Trusted Domain Kernel

**Slice:** ECR-001  
**Lifecycle:** READY_FOR_MERGE_PENDING_EXACT_HEAD_LEDGER_CI  
**Branch:** `001-trusted-domain-kernel`  
**PR:** `#1` — OPEN / READY / mergeable at last live check  
**Latest verified remediation head:** `face8d7448afc617a6c04e53237b066bf2ef5b63`  
**Latest verified remediation CI:** `33097623599` — success  
**Review state:** all three actionable findings remediated; original threads resolved/outdated; CodeRabbit success on remediation head

This is the active-slice execution ledger required by `AGENTS.md`. Normative semantics live in `spec.md`, converged `data-model.md`, `contracts/domain-v1.md`, and exact implementation truth. `VERIFIED_ON_BRANCH` is not `CLOSED_CANONICAL`.

## Current position

```text
Phases 1–10  VERIFIED_ON_BRANCH
Phase 11     T070–T075 complete; T076 waits for merge + post-merge evidence
Phase 12     T077–T080 complete on branch
Phase 13     T081–T084 complete on verified remediation head
Current gate exact-head CI on this final ledger-finalization commit, then final live review/merge audit
```

## Phase 13 remediation disposition

1. **T081 COMPLETE — strict version validation.** `Versioned<T>` no longer exposes a derived bypassing `Deserialize`; ordinary Serde deserialization validates supported schema versions, while `Versioned::from_json_slice` preserves typed `unsupported_major_version` / `unsupported_minor_version` errors.
2. **T082 COMPLETE — wire-safe Fact numerics.** Fact integers and canonical decimals use validated construction paths. API-created values outside the I-JSON exact-integer range or canonical decimal form fail closed, and regression tests prove constructed values serialize and strict-round-trip.
3. **T083 COMPLETE — lifecycle synchronization.** Platform `STATUS.md` and canonical `roadmap.md` both report ECR-001 as `IMPLEMENTING`; active status/EXECUTION/tasks carry the more specific review/merge phase without claiming canonical closure.
4. **T084 COMPLETE on remediation head.** Full exact-head CI passed, all three review threads are resolved/outdated, PR #1 was returned to Ready, and CodeRabbit completed with success and no new actionable thread.

## Exact-head remediation evidence

```text
Head:   face8d7448afc617a6c04e53237b066bf2ef5b63
Run:    33097623599
Runner: macbook
Rust:   1.98.0-aarch64-apple-darwin
Result: SUCCESS
```

The full required surface passed:

```text
Checkout exact branch head                    PASS
Install/verify pinned Rust 1.98.0             PASS
Build locked workspace                        PASS
Format                                        PASS
Strict Clippy                                 PASS
Full workspace tests                          PASS
8 dedicated contract/security targets         PASS
Rustdoc tests                                 PASS
Offline replay gate                           PASS
Unsafe code boundary                          PASS
Dependency boundary + cargo tree              PASS
```

Regression evidence includes:

```text
ordinary_serde_versioned_deserialization_is_strict        PASS
versioned_json_dispatch_preserves_typed_compatibility_errors PASS
fact_numeric_construction_is_wire_safe                    PASS
```

Golden ActionDigest remains:

```text
6ccf10a5d1db36cb9637b4ed2ee6d3f0167c44eec585b543f58d86287710e3d4
```

## CI security posture

The repository uses the approved repository-scoped self-hosted macOS runner `macbook` because GitHub-hosted private-repository execution is blocked by the owner account's `$0` Actions budget with stop-usage enabled.

```text
push: 001-trusted-domain-kernel -> exact feature-head verification
push: main                      -> post-merge canonical verification
workflow_dispatch               -> explicit recovery/recheck
concurrency                      -> cancel superseded same-ref work
runs-on                          -> self-hosted
permissions                      -> contents: read
```

Do not restore `pull_request` execution on this persistent self-hosted machine without an explicit untrusted-code design. Do not weaken gates, force-push, rebase, or treat branch verification as canonical closure.

## Remaining ordered work

```text
1. require the full exact-head gate to PASS on this documentation/status ledger-finalization head
2. re-read PR #1 head, main, reviews, threads, checks and mergeability
3. require no new actionable review blocker on the exact final head
4. merge without rebase/force/destructive history rewriting using the exact expected head
5. verify canonical main with the post-merge self-hosted workflow
6. only then T076 -> platform/roadmap/active status CLOSED_CANONICAL
7. re-read roadmap/dependencies and begin the next genuinely eligible ECR slice
```

## Closure boundary

ECR-001 does not implement authentication/trust roots, authorization/declassification, durable run lifecycle/budgets/persistence, verifier orchestration/reconciliation, browser/model/tool execution or protocol adapters. Those remain owned by ECR-031/ECR-003/ECR-002/ECR-004 and dependent slices.

No dependent ECR implementation is eligible until ECR-001 is `CLOSED_CANONICAL` after merge/post-merge evidence.
