#![forbid(unsafe_code)]

//! Independent verification and reconciliation foundations for Ecra.
//!
//! ECR-004 consumes canonical ECR-001 verification/evidence values and
//! read-only ECR-002 run/attempt truth. It does not authorize execution,
//! fabricate provider receipts, mutate ECR-002 run state, or acquire remote
//! evidence.
//!
//! Semantic verification modules remain pure. Local journal I/O is isolated
//! behind the later `store` module and must not leak provider/runtime behavior
//! into verification logic.

/// Marker exposed by the Phase 1 scaffold so downstream gates can prove the
/// crate is linked without implying semantic verification is implemented.
#[must_use]
pub const fn phase_one_scaffold() -> bool {
    true
}
