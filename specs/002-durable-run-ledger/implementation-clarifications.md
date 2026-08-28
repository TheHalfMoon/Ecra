# ECR-002 Implementation Clarifications

**Status:** FOLDED_INTO_PRIMARY_CONTRACT  
**Created:** 2026-08-27  
**Folded:** 2026-08-28  
**Owner:** ECR-002 convergence

This file records the implementation-time underspecification that was resolved during ECR-002. It is retained as convergence history only and is no longer a competing normative contract. The exact rules below are now frozen in `data-model.md` and `contracts/run-ledger-v1.md`.

## C1 — Bounded diagnostic strings

The planning contract said `SuspensionReason::other.code` and `intervention_recorded.note` were bounded but did not freeze numeric byte limits.

ECR-002 v1 implementation fixed and the primary normative documents now contain:

```text
SuspensionReason::other.code   1..=256 UTF-8 bytes
intervention_recorded.note     0..=4096 UTF-8 bytes when present
```

Rationale:
- keep durable/parser-controlled diagnostic metadata explicitly bounded;
- preserve enough room for human/runtime diagnostics without turning free-form text into authority;
- use UTF-8 byte bounds so the wire/parser rule is deterministic across platforms;
- avoid silently accepting unbounded persisted metadata.

These limits do not make either field authentication, authorization, approval, policy, verification, or provider syntax.

## Convergence disposition

```text
fold C1 into data-model.md                    COMPLETE
fold C1 into contracts/run-ledger-v1.md      COMPLETE
update analyze/traceability evidence         COMPLETE / post-implementation-analyze.md + traceability-closure.md
clarification normative authority            NONE — historical record only
```
