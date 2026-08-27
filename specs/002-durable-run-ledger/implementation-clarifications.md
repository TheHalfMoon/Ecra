# ECR-002 Implementation Clarifications

**Status:** ACTIVE_IMPLEMENTATION_CLARIFICATIONS  
**Created:** 2026-08-27  
**Owner:** ECR-002 convergence

These clarifications exist only where implementation exposed a real numeric underspecification. They must be folded into `data-model.md` and `contracts/run-ledger-v1.md` before `CLOSED_CANONICAL` so this file does not remain a competing normative contract.

## C1 — Bounded diagnostic strings

The planning contract says `SuspensionReason::other.code` and `intervention_recorded.note` are bounded but did not freeze numeric byte limits.

ECR-002 v1 implementation therefore fixes:

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

## Convergence requirement

Before ECR-002 closure:

```text
fold C1 into data-model.md
fold C1 into contracts/run-ledger-v1.md
update analyze/traceability evidence
mark this file FOLDED_INTO_PRIMARY_CONTRACT
```
