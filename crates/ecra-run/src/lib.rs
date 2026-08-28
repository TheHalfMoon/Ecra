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

pub mod archive;
pub mod budget;
pub mod digest;
pub mod error;
pub mod event;
pub mod migration;
mod recovery;
mod sqlite;
pub mod state;
pub mod store;

pub use archive::{
    ArchiveBlob, EcraRunManifest, MAX_ARCHIVE_ENTRIES, MAX_BLOB_COUNT, MAX_EVENT_COUNT,
    MAX_EVENT_ENTRY_BYTES, MAX_MANIFEST_BYTES, MAX_PATH_BYTES, MAX_SINGLE_BLOB_BYTES,
    MAX_TOTAL_UNCOMPRESSED_BYTES, ManifestBlobEntry, ManifestEventEntry, ValidatedEcraArchive,
    export_ecra, read_ecra,
};
pub use budget::{
    BudgetAmount, BudgetDimension, BudgetLimit, BudgetUsage, MAX_BUDGET_AMOUNT, RunBudget,
};
pub use digest::{LedgerDigest, LedgerDigestAlgorithm};
pub use error::{RunError, RunErrorCategory, RunErrorCode, RunErrorSummary};
pub use event::{
    AttemptUnknownCause, EventSequence, InterventionKind, MAX_EVENT_SEQUENCE,
    MAX_INTERVENTION_NOTE_BYTES, RecoveryReason, RunEvent, RunEventEnvelope,
};
pub use migration::ECR_RUN_SCHEMA_VERSION;
pub use recovery::{PreparedAttemptGuard, RecoveryResult, ensure_retry_allowed};
pub use sqlite::SqliteConfiguration;
pub use state::{
    MAX_SUSPENSION_OTHER_CODE_BYTES, PreparedAttemptState, RunPhase, RunReducer, RunState,
    SuspensionReason,
};
pub use store::{ExpectedRunHead, RunStore};

pub const ECR_002_CONTRACT_MAJOR: u16 = 1;
pub const ECR_002_CONTRACT_MINOR: u16 = 0;
