#![forbid(unsafe_code)]

//! Durable local execution truth for Ecra.
//!
//! ECR-002 owns run-event durability, deterministic state reduction, resource
//! accounting, local SQLite persistence, crash/recovery bookkeeping and the
//! bounded `.ecra` interchange format. It builds on `ecra-core` instead of
//! redefining trusted-domain values.
//!
//! This crate does **not** authenticate principals, authorize actions,
//! declassify information, independently verify outcomes, execute providers,
//! protect real secrets at rest, or claim hostile-tamper resistance.
//!
//! The initial Phase 1 scaffold intentionally exposes no persistence or runtime
//! API until the corresponding contract tasks are implemented and verified.

/// ECR-002 implementation marker used by foundation smoke tests.
pub const ECR_002_CONTRACT_MAJOR: u16 = 1;
