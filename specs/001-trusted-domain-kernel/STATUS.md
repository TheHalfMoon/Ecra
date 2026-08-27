# ECR-001 Status — Trusted Domain Kernel

**Slice:** ECR-001  
**Lifecycle:** REVIEW_REMEDIATION  
**Branch:** `001-trusted-domain-kernel`  
**PR:** `#1` — OPEN / DRAFT / mergeable at last live check  
**Latest pre-review exact-head:** `12c7029dbde30d2d860fe70447f79b6432ff2f96`  
**Latest pre-review exact-head CI:** `33095782152` — success  
**Review state:** three actionable findings discovered after Ready transition; remediation active

This is the active-slice execution ledger required by `AGENTS.md`. Normative semantics live in `spec.md`, converged `data-model.md`, `contracts/domain-v1.md`, and exact implementation truth. `VERIFIED_ON_BRANCH` is not `CLOSED_CANONICAL`.

## Current position

```text
Phases 1–10  VERIFIED_ON_BRANCH
Phase 11     T070–T075 complete; T076 waits for merge + post-merge evidence
Phase 12     T077–T080 complete on branch
Phase 13     T081–T084 review remediation active
Current gate complete remediation -> exact-head CI -> close review threads -> Ready
```

## Review findings being remediated

1. **High — strict version validation bypass:** derived `Deserialize` on `Versioned<T>` allowed ordinary Serde callers to bypass `SchemaVersion::validate_supported`.
2. **High — wire-unsafe Fact numeric construction:** public `FactValue::Integer(i64)` allowed API-created integers outside the I-JSON exact-integer range to serialize even though strict deserialization rejected them. Remediation also closes the analogous canonical-decimal construction gap rather than leaving a sibling invariant bypass.
3. **Medium — platform lifecycle drift:** platform `STATUS.md` said `IMPLEMENTING` while the authoritative roadmap still said `TASKS_READY`.

The implementation now uses a validating public `Deserialize` path for `Versioned<T>`, validated Fact numeric wrappers/constructors, and synchronized platform lifecycle docs. These changes are not considered verified until the complete exact-head gate passes.

## Prior verification evidence

The approved repository-scoped self-hosted macOS runner `macbook` replaced unavailable paid GitHub-hosted execution without making the repository public or adding a payment method.

```text
Head:   12c7029dbde30d2d860fe70447f79b6432ff2f96
Run:    33095782152
Runner: macbook
Result: SUCCESS
```

That evidence predates Phase 13 source remediation and therefore cannot authorize merge of the new head.

## CI security posture

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
1. finish T081–T083 remediation
2. run the full exact-head gate on the remediation head
3. inspect every review thread and resolve only findings proven fixed
4. mark T081–T084 complete only with exact evidence
5. return PR #1 to Ready and observe final review/check state
6. merge only with clean exact-head evidence and no actionable review blocker
7. verify canonical main with the post-merge self-hosted workflow
8. only then T076 -> platform/roadmap/active status CLOSED_CANONICAL
9. re-read roadmap and begin the next genuinely eligible ECR slice
```

## Closure boundary

ECR-001 does not implement authentication/trust roots, authorization/declassification, durable run lifecycle/budgets/persistence, verifier orchestration/reconciliation, browser/model/tool execution or protocol adapters. Those remain owned by ECR-031/ECR-003/ECR-002/ECR-004 and dependent slices.

No dependent ECR implementation is eligible until ECR-001 is `CLOSED_CANONICAL` after merge/post-merge evidence.
