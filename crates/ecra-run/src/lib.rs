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

pub mod budget;
pub mod digest;
pub mod error;
pub mod event;
pub mod state;

pub use budget::{
    BudgetAmount, BudgetDimension, BudgetLimit, MAX_BUDGET_AMOUNT, RunBudget,
};
pub use digest::{LedgerDigest, LedgerDigestAlgorithm};
pub use error::{RunError, RunErrorCategory, RunErrorCode, RunErrorSummary};
pub use event::{
    AttemptUnknownCause, EventSequence, InterventionKind, MAX_EVENT_SEQUENCE,
    MAX_INTERVENTION_NOTE_BYTES, RecoveryReason, RunEvent, RunEventEnvelope,
};
pub use state::{MAX_SUSPENSION_OTHER_CODE_BYTES, RunPhase, SuspensionReason};

pub const ECR_002_CONTRACT_MAJOR: u16 = 1;
pub const ECR_002_CONTRACT_MINOR: u16 = 0;
