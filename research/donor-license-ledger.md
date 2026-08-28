# Ecra Donor and License Ledger

**Status:** CANONICAL_IMPLEMENTATION_LEDGER  
**Created:** 2026-08-27  
**Updated:** 2026-08-28 for ECR-031 T001 implementation-time dependency disposition

This ledger separates **conceptual reference**, **dependency candidate**, **locked dependency**, and **source-reuse candidate**. Listing a project here never authorizes copying its source. Source reuse requires exact-file review, license compatibility, notice handling, and an implementation change that records what was copied/modified.

## Status Definitions

- `REFERENCE_ONLY` — study architecture/product/research; do not copy source under current plan.
- `DEPENDENCY_CANDIDATE` — may be used through normal package dependency after implementation-time license/security review.
- `LOCKED_DEPENDENCY` — exact release is present in the committed lockfile and reviewed for the owning slice.
- `SOURCE_REUSE_CANDIDATE` — selective source reuse may be considered only with exact provenance/notice handling.
- `FOUNDATION_CANDIDATE` — upstream project may become a maintained product foundation; requires dedicated upstream strategy.
- `BLOCKED_UNTIL_REVIEW` — licensing or maintenance conditions make source reuse unsuitable without explicit decision.

## Browser and Human Product

| Project | Role | Observed license | Status | Ecra use / constraint |
|---|---|---|---|---|
| Mozilla Firefox / Gecko | Production browser engine/foundation | MPL-2.0 and component-specific notices | FOUNDATION_CANDIDATE | Preferred browser foundation; ECR-007 owns exact upstream/distribution/notice analysis. |
| Zen Browser | Human productivity/browser UX donor | MPL-2.0 | SOURCE_REUSE_CANDIDATE | Prefer UX concepts and selective compatible patches over permanent deep downstream fork. Modified covered files remain subject to MPL obligations. |
| zen-browser/surfer | Firefox-fork build tooling reference/candidate | MPL-2.0 | DEPENDENCY/SOURCE_REUSE_CANDIDATE | Prototype/prerelease; useful for patch/build workflow ideas, not constitutional dependency. |
| Skyvern-AI/rustwright | Rust-native Chromium/CDP and agent-browser ergonomics | MIT | REFERENCE/DEPENDENCY_CANDIDATE | Strong donor for AX snapshots/CLI/CDP provider; not Ecra Firefox internal engine. |
| chromiumoxide | Rust CDP reference/provider | MIT/Apache-family per upstream review required before use | REFERENCE_ONLY pending exact license verification | Fallback/reference for Chromium provider. |
| Browser Use | Agent browser UX/behavior/evals | MIT | REFERENCE_ONLY | Do not architect Ecra as a Python port/fork. |
| Browserbase Stagehand | Hybrid deterministic/AI browser workflow ideas | MIT | REFERENCE_ONLY | Caching/self-healing/workflow ideas; Ecra differentiates through verified compiled Skill IR. |
| Tandem Browser | Human/agent shared-browser competitor/reference | verify exact upstream before source use | REFERENCE_ONLY | Product/UX benchmark only unless separately reviewed. |
| BrowserOS | OSS agent-browser competitor | AGPL-3.0 observed in discovery | BLOCKED_UNTIL_REVIEW | Competitor/reference; no source reuse under current distribution assumptions. |

## Rust Agent / Execution Architecture

| Project | Role | Observed license | Status | Ecra use / constraint |
|---|---|---|---|---|
| block/buzz | Rust event/identity/audit/protocol architecture | Apache-2.0 | REFERENCE/SOURCE_REUSE_CANDIDATE | Strong architectural donor; exact source reuse requires NOTICE/file provenance. |
| 0xPlaygrounds/rig | Serializable/Sans-I/O agent run concepts | MIT | REFERENCE/DEPENDENCY_CANDIDATE | ECR-002 may reuse patterns or dependency after exact fit review; Ecra owns canonical RunState. |
| aaif-goose/goose | Rust agent/provider/MCP/desktop ecosystem | Apache-2.0 | REFERENCE_ONLY | Study provider/extension/local-model patterns; avoid importing broad agent architecture into trusted core. |
| deepseek-ai/deepseek-harness | Replaceable capabilities/events/plugin tree | MIT observed in discovery | REFERENCE_ONLY | Conceptual architecture donor; TypeScript implementation is not Ecra core. |
| snarktank/ralph | Durable small-task/fresh-context iteration | MIT | REFERENCE_ONLY | Planning/execution loop concepts only. |

## State, Memory, Search, Graph

| Project | Role | Observed license | Status | Ecra use / constraint |
|---|---|---|---|---|
| tursodatabase/agentfs | Portable agent filesystem/state/audit model | MIT | REFERENCE/DEPENDENCY_CANDIDATE | Strong ECR-002/ECR-010 donor; Ecra does not inherit schema blindly. |
| Graphify-Labs/graphify | Provenance-aware graph extraction | Apache-2.0 | REFERENCE_ONLY | Key conceptual donor for EXTRACTED vs INFERRED-style provenance. |
| vitali87/code-graph-rag | Structural code graph/runtime relationships | MIT | REFERENCE_ONLY | ECR-019 structural context research. |
| upstash/context7 | Version-aware documentation context | MIT for repository components observed in discovery | REFERENCE/PROVIDER_CANDIDATE | External truth/provider ideas; hosted/private backend pieces are not assumed open source. |
| Tree-sitter | Incremental syntax parsing | MIT | DEPENDENCY_CANDIDATE | Likely ECR-019 dependency. |
| ast-grep | Structural search/rewrite | MIT observed in ecosystem | DEPENDENCY_CANDIDATE | Likely ECR-019 dependency after exact version/license review. |
| Tantivy | Embedded full-text search | MIT | DEPENDENCY_CANDIDATE | Preferred local text-index candidate for ECR-009. |
| petgraph | In-process graph structures | MIT/Apache-2.0 | DEPENDENCY_CANDIDATE | Candidate for local graph representation. |
| Qdrant | Vector database | Apache-2.0 | OPTIONAL_PROVIDER_CANDIDATE | Must not become local-core requirement. |

## Policy, Protocols, Sandboxing

| Project | Role | Observed license | Status | Ecra use / constraint |
|---|---|---|---|---|
| cedar-policy/cedar | Fine-grained authorization engine | Apache-2.0 | DEPENDENCY_CANDIDATE | ECR-003 adapter candidate; Ecra capability model remains independent. |
| Bytecode Alliance Wasmtime | WebAssembly/component runtime | Apache-2.0 WITH LLVM-exception / upstream notices | DEPENDENCY_CANDIDATE | ECR-017 plugin sandbox candidate; sandbox is not treated as infallible. |
| modelcontextprotocol/rust-sdk (`rmcp`) | MCP Rust SDK | mixed transition: Apache-2.0 for newly relicensed/new code; legacy MIT portions per upstream notice | DEPENDENCY_CANDIDATE | ECR-016 must pin reviewed version and preserve applicable notices. |
| agentclientprotocol/rust-sdk | ACP Rust SDK | verify exact version license before dependency | DEPENDENCY_CANDIDATE | ECR-016 only. |
| A2A Rust SDK | Agent-to-agent protocol | verify exact version license before dependency | DEPENDENCY_CANDIDATE | ECR-016 only. |
| Agent Skills specification | Portable skill knowledge format | verify spec/code licensing before bundled source use | REFERENCE/INTEROP | ECR-016 import/export semantics; Ecra Skill IR remains distinct. |

## Serialization / Trusted Core Candidates

| Project / standard | Role | License | Status | Ecra use / constraint |
|---|---|---|---|---|
| RFC 8785 JCS | Canonical JSON scheme | RFC text terms | STANDARD_REFERENCE | ECR-001 canonicalization contract. |
| `serde_jcs` | Rust RFC 8785 implementation | MIT OR Apache-2.0 | DEPENDENCY_CANDIDATE | ECR-001 candidate; exact release/security/maintenance review before lockfile. |
| Serde | Serialization | MIT OR Apache-2.0 | DEPENDENCY_CANDIDATE | ECR-001 likely dependency. |
| serde_json | JSON | MIT OR Apache-2.0 | DEPENDENCY_CANDIDATE | ECR-001 likely dependency. |
| thiserror | Typed error derivation | MIT OR Apache-2.0 | DEPENDENCY_CANDIDATE | ECR-001 likely dependency. |
| uuid | Strong UUID values | MIT OR Apache-2.0 | DEPENDENCY_CANDIDATE | ECR-001 candidate. |
| url | Standards-aware URL parsing | MIT OR Apache-2.0 | DEPENDENCY_CANDIDATE | ECR-001 `WebOrigin` parsing candidate. |

## ECR-001 Locked Dependency Review

The versions below are the exact packages committed in `Cargo.lock` for ECR-001. License expressions were checked against the exact package manifests/license files where available and the corresponding upstream project license. `scripts/check-core-deps.sh` separately fail-closes the production direct-dependency set and prohibited FR-050 runtime categories.

| Package | Exact locked version | Scope | License | ECR-001 role | Source reuse |
|---|---:|---|---|---|---|
| `serde` | 1.0.229 | runtime | MIT OR Apache-2.0 | typed serialization/deserialization | dependency API only; no source copied |
| `serde_json` | 1.0.151 | runtime | MIT OR Apache-2.0 | strict JSON wire parsing/value tests | dependency API only; no source copied |
| `serde_jcs` | 0.2.0 | runtime | MIT OR Apache-2.0 | RFC 8785 canonical JSON implementation | dependency API only; no source copied |
| `sha2` | 0.11.0 | runtime | MIT OR Apache-2.0 | SHA-256 security-binding digest | dependency API only; no source copied |
| `thiserror` | 2.0.20 | runtime | MIT OR Apache-2.0 | typed error derivation | dependency API only; no source copied |
| `url` | 2.5.8 | runtime | MIT OR Apache-2.0 | standards-aware web-origin parsing | dependency API only; no source copied |
| `uuid` | 1.26.0 | runtime | Apache-2.0 OR MIT | opaque UUID-backed ID values; generation features are not enabled | dependency API only; no source copied |
| `proptest` | 1.11.0 | dev-only | MIT OR Apache-2.0 | property/invariant tests | test dependency API only; no source copied |

### ECR-001 review notes

- `Cargo.lock` is the exact version authority; permissive version ranges in `crates/ecra-core/Cargo.toml` resolve reproducibly through the lockfile.
- Runtime direct dependencies are limited to the seven packages above; `proptest` is dev-only and is excluded from the runtime dependency boundary.
- ECR-001 does not vendor these crates or copy/adapt their implementation source into `crates/ecra-core`.
- The implementation uses public dependency APIs and independently written Ecra domain code. Repository review found no copied/adapted donor code requiring an exact-file provenance entry under the Source Reuse Rules below.
- `serde_jcs` is used as a dependency rather than copying canonicalization source; Ecra owns the wrapper, domain separator, normative fixtures and digest contract.
- `sha2` is used as a dependency rather than copying a SHA-256 implementation; Ecra owns the `SecurityDigest`/`ActionDigest` domain semantics.
- `uuid` is used for parsing/storage of strong opaque identifiers. ECR-001 does not treat UUID version/display text as authority and does not require random generation.
- Future dependency additions or feature changes require a new ledger delta and must pass the direct-dependency allowlist before merge.

## ECR-002 Locked Dependency Review

ECR-002 extends the workspace with `ecra-run` while leaving the ECR-001 `ecra-core` direct dependency boundary unchanged. The committed generated `Cargo.lock` has SHA-256 `b720472bf40a554ab61afb74eae95dd625bc6b2604e47a632991faea630e42c6`. `scripts/check-run-deps.sh` owns the ECR-002 direct-runtime allowlist and native SQLite exception; `scripts/check-run-unsafe.sh` keeps Ecra-authored Rust code under `#![forbid(unsafe_code)]`.

| Package / component | Exact locked version | Scope | License/status | ECR-002 role | Source/native boundary |
|---|---:|---|---|---|---|
| `ecra-core` | workspace path | runtime | Ecra source | imports ECR-001 trusted domain types | no duplication of ECR-001 semantics |
| `serde` | 1.0.229 | runtime | MIT OR Apache-2.0 | strict typed wire serialization | dependency API only |
| `serde_json` | 1.0.151 | runtime | MIT OR Apache-2.0 | JSON parsing/value support | dependency API only |
| `serde_jcs` | 0.2.0 | runtime | MIT OR Apache-2.0 | RFC 8785 canonical event/archive JSON | dependency API only |
| `sha2` | 0.11.0 | runtime | MIT OR Apache-2.0 | domain-separated ledger/content bindings | dependency API only |
| `thiserror` | 2.0.20 | runtime | MIT OR Apache-2.0 | typed run/store/archive errors | dependency API only |
| `rusqlite` | 0.40.2 | runtime | MIT | bounded SQLite adapter | dependency API only; `default-features = false`, `bundled` enabled |
| `libsqlite3-sys` | 0.38.2 | transitive native runtime | MIT | FFI/build boundary used by `rusqlite` | native dependency outside Ecra-authored Rust unsafe boundary |
| bundled SQLite | 3.53.2 | native runtime source embedded by `libsqlite3-sys` | public domain | local durable storage engine | compiled by dependency build; no SQLite source copied into Ecra |
| `zip` | 8.6.0 | runtime | MIT | strict Stored-only `.ecra` reader/writer substrate | dependency API only; `default-features = false` |
| `proptest` | 1.11.0 | dev-only | MIT OR Apache-2.0 | reducer/budget/archive properties | test dependency API only |
| `tempfile` | 3.27.0 | dev-only | MIT OR Apache-2.0 | isolated SQLite/archive tests | test dependency API only |

### ECR-002 review notes

- `rusqlite` and `zip` are exact-pinned in `crates/ecra-run/Cargo.toml`; the committed generated lockfile is the transitive version authority.
- The `bundled` `rusqlite` feature intentionally selects `libsqlite3-sys`'s embedded SQLite instead of ambient system SQLite. For `libsqlite3-sys 0.38.2`, the bundled amalgamation identifies SQLite `3.53.2`.
- `rusqlite` and `libsqlite3-sys` are MIT licensed; the bundled SQLite amalgamation is public domain according to upstream licensing documentation.
- `zip 8.6.0` is MIT licensed. Ecra disables its default features and owns the stricter ECR-002 archive profile: Stored entries only, deterministic metadata/order, hard parser limits, and fail-closed validation. Dependency capability does not widen the Ecra contract.
- The native SQLite C/FFI implementation is explicitly outside Ecra-authored `#![forbid(unsafe_code)]`; this is a reviewed native dependency boundary, not an unsafe exception inside `ecra-run` source.
- ECR-002 does not copy/adapt implementation source from rusqlite, libsqlite3-sys, SQLite, zip, Rig, AgentFS, Restate, or other donors into Ecra. Public dependency APIs and independently written Ecra code are used.
- No network, browser, model, provider, process-execution, telemetry, authentication, authorization, or verification dependency is authorized by this delta.
- Any feature/version/native-boundary change requires a new ledger delta plus exact-head dependency/unsafe gate evidence.

### ECR-002 Phase 8 locked-dependency evidence

The Phase 8 implementation candidate was gated on exact head `af5d8d580b29af450807b32281b79f04e17c1aa7` by ECR-002 CI run `33151825178`, job `98785357882`, with all build, rustfmt, strict Clippy, workspace tests, ECR-001 regression targets, ECR-002 contract targets, rustdoc, offline replay, core/run unsafe+dependency checks, and dependency-evidence steps successful.

```text
rustc 1.98.0 (88d9e12ae 2026-08-18)
cargo 1.98.0 (797e8a9bc 2026-08-05)
Cargo.lock SHA-256  b720472bf40a554ab61afb74eae95dd625bc6b2604e47a632991faea630e42c6
rusqlite             =0.40.2, default-features=false, features=["bundled"]
libsqlite3-sys       0.38.2
bundled SQLite       3.53.2
zip                   =8.6.0, default-features=false
archive profile       Stored-only; Ecra-owned deterministic metadata/order and fail-closed limits
```

The dependency-evidence step also re-read the direct runtime tree as `ecra-core`, `rusqlite`, `serde`, `serde_jcs`, `serde_json`, `sha2`, `thiserror`, and `zip`; it introduced no new runtime provider/network/process dependency. The exact final Phase 8 ledger head remains subject to its own full gate before T060–T066 are marked complete.

## ECR-031 T001 Accepted Dependency Candidates

T001 performed an implementation-time review on 2026-08-28 before manifest adoption. The entries below are **accepted direct candidates**, not yet `LOCKED_DEPENDENCY`: promotion to locked status requires the committed generated `Cargo.lock`, exact resolved tree, native-boundary evidence and Phase 1 exact-head CI/T010 disposition.

| Package / component | Exact direct version | Planned direct feature shape | License | Scope / boundary | T001 status |
|---|---:|---|---|---|---|
| `ed25519-dalek` | 3.0.0 | `default-features=false`, `fast`, `zeroize` | BSD-3-Clause | portable v1 Ed25519 sign/verify; no rand/serde/PKCS8/PEM/batch/hazmat | ACCEPTED_CANDIDATE |
| `chacha20poly1305` | 0.11.0 | `default-features=false`, `alloc`, `zeroize` | Apache-2.0 OR MIT | RFC 8439 AEAD; dependency-owned randomness disabled | ACCEPTED_CANDIDATE |
| `hkdf` | 0.13.0 | `default-features=false` | MIT OR Apache-2.0 | HKDF-SHA-256; optional generic `kdf` feature disabled | ACCEPTED_CANDIDATE |
| `sha2` | 0.11.0 | `default-features=false` | MIT OR Apache-2.0 | SHA-256; same release already locked by ECR-001/ECR-002 | ACCEPTED_CANDIDATE |
| `zeroize` | 1.9.0 | `default-features=false`, `alloc` | MIT OR Apache-2.0 | bounded secret wrapper; derive disabled | ACCEPTED_CANDIDATE |
| `getrandom` | 0.4.3 | `default-features=false` | MIT OR Apache-2.0 | explicit fallible system CSPRNG boundary; no `rand`, `wasm_js` or `sys_rng` | ACCEPTED_CANDIDATE |
| `security-framework` | 3.7.0 | macOS target only; `default-features=false`, `OSX_10_15` | MIT OR Apache-2.0 | Data Protection Keychain integration; native Security.framework/CoreFoundation FFI boundary | ACCEPTED_CANDIDATE |

All seven candidates declare MSRV 1.85 and are compatible with the repository's pinned Rust 1.98.0 toolchain. No source from these projects is copied or adapted; Ecra uses dependency APIs and independently written domain/security code.

### ECR-031 T001 advisory and native-boundary disposition

- RustSec RUSTSEC-2022-0093 affects `ed25519-dalek <2`; 3.0.0 is outside the affected range. `hazmat` remains disabled.
- RustSec RUSTSEC-2024-0344 for `curve25519-dalek` is patched in `>=4.1.3`; the accepted Ed25519 release requires curve25519-dalek 5.0.0.
- RustSec RUSTSEC-2021-0100 for `sha2` is patched in `>=0.9.8`; 0.11.0 is outside the affected range.
- RustSec RUSTSEC-2019-0029 for `chacha20` is patched in `>=0.2.3`; the advisory also states `chacha20poly1305` is unaffected by the overflow issue. The accepted AEAD release uses modern chacha20 0.10.
- `zeroize_derive` is intentionally not enabled; the old derive advisory does not justify adding the derive surface.
- the historical `security-framework` TLS-hostname advisory is patched long before 3.7.0; ECR-031 also disables the crate's Secure Transport/ALPN/session-ticket default features and uses it only for the Keychain path.
- `security-framework`/`security-framework-sys` are an explicit reviewed native/FFI boundary outside Ecra-authored `#![forbid(unsafe_code)]`.
- `getrandom` may use target-specific OS/FFI dependencies for entropy. This is accepted for `SecureRandom` only and does not constitute or authorize a Windows/Linux `TrustBackend`.

Current RustSec records were reviewed before this disposition. The eventual exact lockfile must still be rechecked as a graph during T008–T010; this table is not a permanent advisory waiver.

### Rejected/deferred ECR-031 dependency surface

- `rand`/`rand_chacha`: rejected as unnecessary ambient RNG abstraction; use the explicit `getrandom` boundary.
- OpenSSL, libsodium, `ring` or general crypto suites: rejected because they broaden algorithms/native capability beyond the frozen suite.
- Ed25519 batch/serde/PEM/PKCS8/hazmat/legacy features: rejected as unnecessary for the strict Ecra v1 wire/custody contract.
- `chacha20poly1305` default `getrandom` feature: rejected because nonce/randomness ownership must stay explicit.
- `security-framework` default Secure Transport/ALPN/session-ticket feature set: rejected as unrelated to Keychain custody.
- Secure Enclave signing as the portable v1 Ed25519 implementation: rejected by the frozen algorithm/custody contract.
- Windows DPAPI and Linux Secret Service crates: deferred; no platform backend crate may be added until the corresponding backend is implemented and evidenceable.

Primary implementation-time dependency review details and links are maintained in `specs/031-identity-trust-root/research.md` §23.

## Durable Execution References With Licensing Caution

| Project | Role | Observed license | Status | Ecra use / constraint |
|---|---|---|---|---|
| Restate core | Durable execution semantics | Business Source License 1.1 with delayed Apache change | REFERENCE_ONLY | Study exactly-once/durable execution semantics; do not copy core under current plan. |

## Research / Evaluation References (No Source Reuse Implied)

- BrowserGym / AgentLab — benchmark harness methodology.
- WebArena-Verified — deterministic browser verification concepts.
- Online-Mind2Web — live-web drift evaluation.
- OSWorld 2.0 / WeaveBench — long-horizon state/constraint evaluation.
- WASP / BrowseSafe / SOPBench / StepJack / AgentLAB — prompt-injection/origin/security research.
- Microsoft Universal Verifier / FaraGen / Echoverse — verifier/training-environment concepts.
- WebXSkill / ReUseIt / Hierarchical Memory Tree / Artic — reusable workflow/Skill IR research.
- WebMCP standards/work — semantic website capability surface and its trust implications.

These references may supply ideas, benchmark adapters, or standards compatibility. Paper text/code/datasets have separate licenses/terms and require exact review before redistribution.

## Source Reuse Rules

Before any copied/adapted code enters Ecra, the implementation change MUST record:

```text
upstream repository
exact commit/tag
exact file(s)/region(s)
upstream copyright holder/notice
license
whether file was modified
required NOTICE/source-offer obligations
Ecra destination path
reason source reuse is preferable to clean implementation/dependency
```

No commit may use “inspired by” to obscure copied source provenance.

## Dependency Review Rules

Before a candidate becomes a locked dependency:

1. verify exact release/tag and license files;
2. inspect security policy/advisories relevant to Ecra use;
3. record transitive-license compatibility where material;
4. minimize default features;
5. pin/lock reproducibly;
6. define update/advisory ownership;
7. confirm the dependency does not violate a constitutional boundary.

## Current Authorization

ECR-001 remains authorized only for its locked trusted-domain dependency set. ECR-002 is additionally authorized for the locked `ecra-run` dependency delta recorded above, including the bounded bundled-SQLite native boundary and Stored-only ZIP substrate. ECR-031 T001 has authorized only the accepted candidate/direct-feature set recorded above for Phase 1 adoption; those candidates are not `LOCKED_DEPENDENCY` until the generated lockfile and exact-head T009/T010 evidence converge. No authorization permits copied donor source, ambient network/provider execution, downstream sensitive-state persistence, general authorization/declassification policy, or independent verification/reconciliation behavior.