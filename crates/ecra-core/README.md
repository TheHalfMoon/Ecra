# ecra-core — ECR-001 Trusted Domain Kernel

`ecra-core` is Ecra's zero-I/O, provider-neutral trusted domain contract. It owns deterministic value objects, structural invariants, canonical serialization, security-binding digests, action/attempt/receipt identity, and the independent verification record shape. It does **not** authenticate principals, authorize actions or disclosures, execute tools/models/browsers, persist state, access secrets, or perform network/process/filesystem I/O.

Normative package: `../../specs/001-trusted-domain-kernel/`  
Normative v1 contract: `../../specs/001-trusted-domain-kernel/contracts/domain-v1.md`  
Normative fixtures: `../../contracts/ecra-domain-v1/`

## Architecture map

| Module | Primary types / responsibility | Owning requirements |
|---|---|---|
| `version` | `SchemaVersion`, `Versioned<T>`, strict compatibility dispatch | FR-001, FR-047 |
| `error` | `DomainError`, `ErrorCode`, `ErrorCategory` | FR-053 |
| `id` | opaque security/audit ID newtypes | FR-002, FR-003 |
| `time` | `EpochMillis`, `TemporalValidity`, caller-supplied `EvaluationContext` | FR-018, FR-049 |
| `canonical` | RFC 8785 JCS wrapper | FR-004, FR-051 |
| `digest` | `ContentDigest`, SHA-256 `SecurityDigest`, `ActionDigest` | FR-031, FR-032, FR-051 |
| `actor` | attribution-only `Actor` / `ActorKind` | FR-005–FR-007 |
| `identity` | opaque `PrincipalRef` / `IdentityAssertionRef` | FR-006 |
| `origin` | typed provenance/origin context and `WebOrigin` | FR-008, FR-009 |
| `resource` | stable `ResourceRef` identity plus non-authoritative locator | FR-010, FR-054 |
| `scope` | explicit `ScopeConstraint<T>`, `Scope`, `PurposeRef` | FR-011–FR-013, FR-054 |
| `capability` | distinct request/grant/delegation representation and non-authoritative request reason | FR-014–FR-019, FR-054, FR-055 |
| `information` | classification, tags, `InformationRef`, source-to-sink `InformationUse` | FR-027–FR-029, FR-034, FR-035, FR-054 |
| `evidence` | observation, fact, provenance, freshness, dispute, evidence references | FR-020–FR-029, FR-046, FR-054 |
| `artifact` | artifact identity, classification, digest, lineage and locator metadata | FR-029–FR-031, FR-054 |
| `action` | intent, effect/reversibility/idempotency/retry, parameters, `ActionRef`, attempts | FR-033, FR-036–FR-040, FR-048, FR-055 |
| `receipt` | executor-known `ActionReceipt` and conservative outcome | FR-041–FR-043, FR-054, FR-055 |
| `verification` | independent `VerificationReceipt`, target, method and outcome | FR-022, FR-044–FR-046, FR-054, FR-055 |

## Seven misuse warnings

These are contract boundaries, not style preferences:

1. **Actor != authenticated Principal.** `ActorId` is attribution; authentication/trust-root validity belongs downstream.
2. **Classification != permission.** `public/private/sensitive/secret/unknown` describes information, never authority.
3. **InformationUse != authorization.** A source-to-sink declaration states intended use/disclosure; ECR-003 decides whether it is permitted.
4. **Locator != resource security identity.** URLs, filesystem paths, storage locators and provider strings may alias or change; typed IDs remain identity.
5. **ActionDigest != signature or approval.** It is deterministic content binding over the exact versioned ActionIntent, not authorization, a MAC, or proof of trust.
6. **ActionReceipt != verification.** Executor-observed success/failure is separate from independent `VerificationReceipt` outcomes.
7. **UNKNOWN remains UNKNOWN.** Ambiguous external outcomes are never silently normalized to success/failure or made safe to retry.

## Canonical ActionDigest v1

The normative security binding is:

```text
UTF8("ecra/action-intent/v1\0")
|| RFC8785_JCS(Versioned<ActionIntent>)
  ↓ SHA-256
ActionDigest
```

The committed golden fixture and expected canonical bytes/hex are security-sensitive contract artifacts under `../../contracts/ecra-domain-v1/`.

## Free-form metadata boundary

Labels, capability-request reasons, purpose text, notes, locators, external/provider references, diagnostic messages and similar free-form metadata are non-authoritative. They must never be parsed to manufacture authentication, authorization, approval, resource identity or verification. See `../../specs/001-trusted-domain-kernel/free-form-field-audit.md` and `tests/non_authoritative_metadata.rs`.

## Runtime boundary

Production dependencies are fail-closed by `../../scripts/check-core-deps.sh`; any new direct runtime dependency requires explicit review. `#![forbid(unsafe_code)]` plus `../../scripts/check-core-unsafe.sh` enforce the zero-unsafe boundary. CI also runs the full workspace test suite offline after dependencies are available.

The production crate must not depend on async runtimes, networking, databases, browser control, model/provider SDKs, policy engines, protocol SDKs, process/filesystem execution frameworks, or telemetry exporters.

## Verification commands

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

`VERIFIED_ON_BRANCH` is not `CLOSED_CANONICAL`; closure requires the full ECR-001 traceability/analyze/merge/post-merge process defined by the active Spec Kit package.
