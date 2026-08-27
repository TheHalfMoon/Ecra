# ECR-001 Status — Trusted Domain Kernel

**Slice:** ECR-001  
**Lifecycle:** FINAL_CONVERGENCE_VERIFICATION  
**Branch:** `001-trusted-domain-kernel`  
**PR:** `#1` — OPEN / DRAFT / mergeable at last live check  
**Latest fully verified implementation baseline:** `5dfe4c09b2abceeec14bc94b8e13d2dccddfd37c`  
**Phase 10 CI:** `33086490495` — success  
**Latest successful converged executable-gate head:** `a7f1ea27e55fe7d41d70a6101dd3f44502e260f0`  
**Latest successful converged executable-gate CI:** `33087744071` — success  
**Ledger parent head:** `d84f01ad4e1c71aac58a89fcdea8d67179fc89fe`

This is the active-slice execution ledger required by `AGENTS.md`. Normative semantics live in `spec.md`, converged `data-model.md`, `contracts/domain-v1.md`, and exact implementation truth. `VERIFIED_ON_BRANCH` is not `CLOSED_CANONICAL`.

## Current position

```text
Phases 1–10  VERIFIED_ON_BRANCH
Phase 11     T070–T072, T074–T075 complete; T073/T076 remain
Phase 12     T077–T079 complete; T080 BLOCKED_BY_EXTERNAL_GITHUB_HOSTED_RUNNER_ACCOUNT_STATE
```

## Verified implementation and convergence evidence

Phase 10 implementation head `5dfe4c09b2abceeec14bc94b8e13d2dccddfd37c`, CI `33086490495`, passed build, format, strict Clippy, full tests, rustdoc, offline replay, zero-unsafe and dependency-boundary gates.

The revised quickstart/CI surface was proven executable at `a7f1ea27e55fe7d41d70a6101dd3f44502e260f0`, run `33087744071`, including:

```text
Build locked workspace                     PASS
Format                                     PASS
Clippy                                     PASS
Full workspace tests                       PASS
Dedicated contract and security targets   PASS
Rustdoc tests                              PASS
Offline replay gate                        PASS
Unsafe code boundary                       PASS
Dependency boundary + cargo tree           PASS
```

Golden ActionDigest remains:

```text
6ccf10a5d1db36cb9637b4ed2ee6d3f0167c44eec585b543f58d86287710e3d4
```

## Convergence disposition

- T077 COMPLETE: primary data model + v1 contract match implemented version/error semantics and folded C1–C12.
- T078 COMPLETE: quickstart/tasks/STATUS/EXECUTION match the real gate surface and convergence phase.
- T079 COMPLETE: `traceability-closure-2026-08-27.md` maps FR-001–FR-055, SC-001–SC-020, constitution G1–G15 and P-001–P-035 to exact ECR-001 evidence or explicit downstream enforcement owners.
- Final semantic analyze: `final-convergence-analyze-2026-08-27.md` reports zero blocking drift, zero failed constitution gates, zero unowned FR/SC/pre-review blockers.

## External GitHub-hosted runner account-state blocker

No ECR-001 build/test failure is currently observable. GitHub-hosted jobs terminate before runner allocation and therefore produce no checkout/build/test steps or job logs.

Evidence:

1. Final-report head `3a6b2c8794d322aee06771a36dd46ed891a95c62`, run `33088269829`, was attempted four times. Every attempt ended pre-runner with `steps=[]`.
2. The previously successful run `33087744071` at known-good head `a7f1ea27…` was re-run and then failed in the same pre-runner/zero-step state. This proves the failure is not specific to the final documentation head.
3. Fresh standard-runner head `238f49a1…`, run `33089816127`, failed pre-runner with zero steps.
4. A bounded diagnostic switched only `runs-on` to GitHub's `ubuntu-slim` pool at `5a42bf77…`, run `33090299540`. It also failed pre-runner with zero steps. The workflow was immediately restored to `ubuntu-latest`; the slim experiment changed no product code, tests, fixtures, Cargo graph, or gate commands.
5. Restored-standard head `57c70a76…` produced both push/PR attempts and both remained zero-step, confirming the alternate runner pool did not solve the account-level block.
6. CI topology was then corrected at `d84f01ad…` to avoid duplicate feature-branch billing: one `pull_request` run for feature work, one `push` run on `main` for post-merge verification, plus `workflow_dispatch`, with concurrency cancellation. Run `33090580208` still failed pre-runner with zero steps.
7. GitHub check-run metadata for the failed hosted jobs contains one failure annotation per job, but the connected repository API does not expose the annotation body or billing settings endpoint.

The evidence rules out an ECR-001 source/test defect and rules out a single hosted-runner pool outage as the direct repository-side fix. GitHub's documented causes for private-repository hosted jobs being blocked before execution include exhausted included usage with no payable overage, payment/account billing problems, or an Actions budget that blocks further spend.

The connected GitHub repository tool cannot read or mutate personal-account Billing & Licensing settings. Do not weaken ECR-001 gates, merge without exact-head evidence, or make the private repository public merely to bypass billing.

## Required external remediation

For the repository owner account, inspect GitHub **Settings → Billing & Licensing → Budgets and alerts / Actions usage** and resolve whichever account-level condition is blocking GitHub-hosted Actions:

```text
- ensure the payment method/billing account is valid if overage is needed;
- ensure an Actions product/SKU budget is positive and not exhausted;
- ensure no broader overlapping budget is blocking usage;
- if included private-repository minutes are exhausted, authorize paid overage or wait for the billing-cycle reset;
- alternatively register an appropriately isolated self-hosted runner, then explicitly amend the workflow to target it.
```

A self-hosted runner is a possible recovery path because GitHub does not charge hosted-runner minutes for it, but no self-hosted runner is currently known/registered through the available repository API. Do not add an untrusted or persistent self-hosted runner merely to bypass the gate.

## CI usage hardening completed

To reduce recurrence after the account is unlocked, `.github/workflows/ecr-001.yml` now uses:

```text
pull_request        -> one feature-branch verification run
push: main          -> one post-merge canonical verification run
workflow_dispatch   -> explicit manual recovery/recheck
concurrency          -> cancels superseded runs for the same PR/ref
```

The executable gate itself is unchanged.

## Remaining ordered work

```text
1. resolve the GitHub account-level Actions billing/budget/usage block, or register an approved self-hosted runner
2. trigger/observe CI on the exact current PR head
3. require every revised quickstart/CI gate to PASS
4. record T073/T080 exact head/run evidence
5. run final exact-head CI if evidence-ledger finalization changes the head
6. inspect PR reviews/checks/readiness
7. make PR ready only after all pre-merge gates are satisfied
8. merge without force-push/rebase/destructive history rewriting
9. verify canonical main post-merge
10. only then T076 -> roadmap/platform/active status CLOSED_CANONICAL
```

## Closure boundary

ECR-001 does not implement authentication/trust roots, authorization/declassification, durable run lifecycle/budgets/persistence, verifier orchestration/reconciliation, browser/model/tool execution or protocol adapters. Those remain owned by ECR-031/ECR-003/ECR-002/ECR-004 and dependent slices.

No dependent ECR implementation is eligible until ECR-001 is `CLOSED_CANONICAL` after merge/post-merge evidence.
