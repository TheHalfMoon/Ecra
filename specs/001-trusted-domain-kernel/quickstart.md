# Quickstart / Verification Guide: ECR-001 Trusted Domain Kernel

This guide verifies the converged ECR-001 implementation. It requires no browser, model API key, cloud account, database, secret store, network service, external process runtime, or system clock access by the trusted core.

## Prerequisites

- repository-pinned Rust toolchain (`1.98.0` at current v1 convergence);
- Git;
- dependencies available once for the initial locked build; after that the explicit offline gate must pass;
- no external credentials.

## Expected Slice

```text
Cargo.toml
Cargo.lock
rust-toolchain.toml
crates/ecra-core/
contracts/ecra-domain-v1/
scripts/check-core-deps.sh
scripts/check-core-unsafe.sh
specs/001-trusted-domain-kernel/
```

## Canonical Full Gate

Run from repository root on the exact feature head:

```bash
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
cargo test --doc --workspace --locked
cargo test --workspace --locked --offline
bash scripts/check-core-unsafe.sh
bash scripts/check-core-deps.sh
```

Expected: all commands exit 0. A passing earlier SHA does not authorize a later head.

## Dedicated Contract / Security Targets

The full workspace suite includes these targets, but closure evidence must also make them visible by name:

```bash
cargo test -p ecra-core --test valid_fixtures --locked
cargo test -p ecra-core --test invalid_fixtures --locked
cargo test -p ecra-core --test contract_fixtures --locked
cargo test -p ecra-core --test canonicalization --locked
cargo test -p ecra-core --test action_digest --locked
cargo test -p ecra-core --test properties --locked
cargo test -p ecra-core --test portability --locked
cargo test -p ecra-core --test non_authoritative_metadata --locked
```

Expected:
- every committed valid fixture is represented by the typed manifest and round-trips;
- every committed invalid fixture is represented by the typed manifest and fails with the expected machine-readable code/category;
- fixture directory/manifest drift fails the suite;
- canonical fixtures are byte-stable under RFC 8785 JCS;
- ActionDigest fixtures are exact/stable and sensitive to security-relevant field changes;
- property/type-confusion tests preserve constitutional invariants;
- portability inputs produce the same typed values/canonical bytes/digests;
- authority-looking free-form metadata cannot replace typed identity, scope, grant, receipt binding or verification outcome.

## Versioned Wire / Fixture Convention

The public persisted/interchange contract is `Versioned<T>`:

```json
{
  "schema_version": { "major": 1, "minor": 0 },
  "value": {}
}
```

Most repository semantic fixture files may store inner `T` bodies for readability. The fixture runner must wrap/round-trip those bodies through the v1 envelope and separately test full-envelope compatibility and unknown-field cases.

Expected machine behavior:
- unsupported major -> `unsupported_major_version`;
- unsupported newer minor -> `unsupported_minor_version`;
- malformed/missing strict envelope or unknown strict field -> `serialization_failed`.

## Offline / Zero-I/O Gate

After dependencies are available:

```bash
cargo test --workspace --locked --offline
```

Expected: no test requires network, browser, model, DB, external process, secret service, remote clock or cloud account.

The portability/static-source tests and dependency boundary complement this runtime evidence; offline success alone is not proof of every zero-I/O architectural restriction.

## Unsafe Boundary Gate

```bash
bash scripts/check-core-unsafe.sh
```

Expected:
- crate root keeps `#![forbid(unsafe_code)]`;
- no Ecra-owned unsafe block or unsafe-code allowance exists in the trusted core;
- the script fails closed if the boundary is violated.

Do not replace this repository gate with a manual grep-only claim.

## Dependency Boundary Gate

```bash
bash scripts/check-core-deps.sh
cargo tree -p ecra-core
```

Production graph must not include async runtime, HTTP/network, database/storage driver, browser automation, model/provider SDK, policy engine, MCP/ACP/A2A SDK, process/filesystem execution abstraction or telemetry exporter.

The script enforces the reviewed direct runtime allowlist and prohibited transitive categories/names. Pure serialization/UUID/URL/JCS/SHA-256/error dependencies must match the donor/license ledger and locked graph.

## Normative Fixture Review

Primary contract:

```text
specs/001-trusted-domain-kernel/contracts/domain-v1.md
```

Primary data model:

```text
specs/001-trusted-domain-kernel/data-model.md
```

Fixture corpus:

```text
contracts/ecra-domain-v1/{valid,invalid}/
```

Reviewers must sample actual JSON plus expected canonical/digest outputs. Fixtures are contract artifacts, not disposable test data.

## Security-Sensitive Manual Checks

Before closure, demonstrate from code/tests/fixtures:

1. `ActorId` cannot implicitly become `PrincipalId`; actor attribution is not authentication.
2. `CapabilityRequestId`/`CapabilityRequest` cannot implicitly become `CapabilityGrantId`/`CapabilityGrant`.
3. `CapabilityRequest.reason` is non-authoritative and cannot alter principal/operation/target/scope/temporal authority shape.
4. `ScopeConstraint.one_of([])` is rejected and missing/empty scope does not become `any_explicit`.
5. unrestricted scope is represented explicitly in canonical JSON.
6. Resource locator/free-form provider text cannot become stable security identity or permission.
7. web/model/tool/memory text cannot grant permission or alter typed scope/origin/identity by content.
8. private/sensitive/secret classifications round-trip and `unknown` never normalizes to public.
9. Observation payload references remain bounded references, not embedded arbitrary blobs.
10. Fact value integers respect I-JSON exact range and decimals use canonical decimal-string form.
11. freshness basis kind/time are paired.
12. artifact byte size uses canonical non-negative decimal text; artifact lineage uses stable typed IDs.
13. source->remote-provider `InformationUse` is representable but cannot become CapabilityGrant/authorization.
14. Fact has no independent verified truth flag; a model-inferred Fact remains model-inferred while a separate VerificationReceipt can be VERIFIED.
15. local irreversible mutation and external reversible mutation are representable without conflating location/reversibility.
16. ActionParametersRef binds every non-empty parameter reference with SecurityDigest.
17. keyed idempotency without key is rejected; classes that must not carry a key reject it.
18. effect x idempotency x retry combinations match the fail-closed v1 matrix.
19. exact normative `ActionDigest` matches expected SHA-256/JCS/domain-separated fixture.
20. changing every tested security-relevant ActionIntent field changes ActionDigest.
21. `ActionRef` with wrong digest is rejected.
22. two attempts for one ActionIntent have distinct ActionAttemptIds.
23. every ActionReceipt binds exact ActionRef + ActionAttemptId when validated against an intent.
24. receipt completion cannot precede start.
25. executor-observed success is not VERIFIED.
26. ActionReceipt cannot deserialize/cast as VerificationReceipt.
27. UNKNOWN receipt outcome round-trips unchanged.
28. verified/rejected/inconclusive verification requires evidence; not_evaluated may have none.
29. ClaimRef, ErrorSummary, notes, external references and other free-form metadata do not become authority or proof.
30. canonicalization is a fixed point and ContentDigest cannot implicitly substitute for ActionDigest/security binding.

## Constitution v1.1.0 Review

Confirm applicable G1-G15 results still match exact implementation. In particular:
- G1 one trusted domain model;
- G2 explicit fail-closed scope/no ambient authority;
- G3 provenance retained;
- G4 effect/idempotency/retry/attempt semantics explicit;
- G5 one authoritative verification record path;
- G7 privacy/secrets boundary remains zero-I/O/no secret handling;
- G8 local-first/no cloud dependency;
- G9 protocols remain outside internal trusted model;
- G10 donor/license ledger current;
- G13 information-flow/egress represented separately from read authority;
- G14 Actor/Principal separation;
- G15 N/A for runtime budgets remains justified because ECR-001 performs no recursive/runtime execution.

Any failed applicable gate blocks closure.

## Pre-Implementation Review Closure

Review:

```text
specs/000-ecra-platform/pre-implementation-review-2026-08-27.md
```

For every ECR-001-owned finding, the traceability/closure artifact must name exact code/test/fixture evidence. Findings owned by later slices remain explicitly assigned/deferred; ECR-001 value types do not counterfeit downstream enforcement.

## Closure Evidence

The implementation traceability/closure evidence must record:
- exact feature head SHA;
- toolchain version;
- changed-file list or compare range;
- full gate results;
- dedicated contract/security target results;
- offline result;
- dependency-boundary and unsafe-code evidence;
- valid/invalid fixture counts and manifest agreement;
- canonical ActionDigest golden result;
- donor/license delta;
- FR-001-FR-055 + SC-001-SC-020 traceability result;
- constitution G1-G15 re-check;
- ECR-001-owned pre-implementation review mapping;
- `/speckit.analyze`-equivalent result;
- convergence task disposition.

`VERIFIED_ON_BRANCH` is not `CLOSED_CANONICAL`. Closure additionally requires ready/reviewed PR state as required by repository governance, merge, required post-merge verification on canonical `main`, and truthful roadmap/execution advancement.