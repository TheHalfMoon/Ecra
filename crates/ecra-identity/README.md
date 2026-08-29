# ecra-identity

`ecra-identity` is the single bounded trusted crate for ECR-031 — Identity, Trust Root & Sensitive Storage Foundations.

## Ownership

This crate owns only the ECR-031 trust substrate:

- Ecra-local principal enrollment/bootstrap boundaries;
- authenticated identity assertions and on-behalf-of identity binding;
- process-local issuer-session semantics;
- trust-root/key generation, retirement and revocation state;
- authenticated `ProtectedTrustStateV1` authority;
- native secret-custody abstraction and evidence-bearing backend capabilities;
- protected envelope primitives;
- protected-anchor primitives;
- redacted/zeroizing handling for bounded secret materialization.

It reuses ECR-001 identifiers/domain types and must not create a competing actor/principal/capability model.

## Security boundary

A validated identity context proves identity/trust context. It does **not** authorize an action.

The following remain outside this crate:

- capability grants, authorization, approval, declassification and secret-use mediation — ECR-003;
- independent external outcome verification/reconciliation — ECR-004;
- browser/model/tool/provider/process execution;
- MCP/ACP/A2A or other protocol gateway behavior — ECR-016;
- local-model world access — ECR-021;
- multi-device recovery/sync — ECR-022;
- telemetry/privacy product policy — ECR-025;
- general portability/export — ECR-029.

## Production fail-closed rules

- `#![forbid(unsafe_code)]` is mandatory for Ecra-authored Rust in this crate.
- No plaintext file, environment-variable, ordinary database or generic in-memory production fallback may replace an unavailable/locked/unsupported native trust backend.
- `PrincipalId` must never be derived from OS username, email, display label, Actor label, path or protocol identity string.
- Ordinary metadata must never reactivate a retired/revoked key or substitute for authenticated protected trust state.
- No public production API may mint an assertion for a caller-selected arbitrary principal.
- Portable v1 signing is software Ed25519 protected at rest by the native backend; this is not a Secure Enclave/non-exportable/hardware-backed signing claim.
- Platform assurance claims are evidence-scoped. macOS is the initial v1 native acceptance platform; Windows/Linux must not be called verified without their own native evidence.

## Phase ordering

Implementation follows `specs/031-identity-trust-root/tasks.md` exactly. Phase 1 establishes this crate, the reviewed dependency surface and trusted CI before semantic cryptographic/native implementation begins.

No donor implementation source is copied into this crate by default. Dependency and source-reuse disposition is maintained in `research/donor-license-ledger.md`.
