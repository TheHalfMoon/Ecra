use rusqlite::{Connection, TransactionBehavior};

use crate::{RunError, RunErrorCategory, RunErrorCode};

pub const ECR_RUN_SCHEMA_VERSION: i64 = 1;

const SCHEMA_V1: &str = r#"
CREATE TABLE run_events (
    run_id TEXT NOT NULL,
    sequence INTEGER NOT NULL CHECK(sequence >= 1 AND sequence <= 9007199254740991),
    event_digest TEXT NOT NULL,
    previous_digest TEXT,
    event_json BLOB NOT NULL,
    PRIMARY KEY(run_id, sequence),
    UNIQUE(run_id, event_digest)
) STRICT;

CREATE TRIGGER run_events_no_update
BEFORE UPDATE ON run_events
BEGIN
    SELECT RAISE(ABORT, 'run_events are append-only');
END;

CREATE TRIGGER run_events_no_delete
BEFORE DELETE ON run_events
BEGIN
    SELECT RAISE(ABORT, 'run_events are append-only');
END;

CREATE TABLE run_heads (
    run_id TEXT PRIMARY KEY,
    last_sequence INTEGER NOT NULL CHECK(last_sequence >= 1 AND last_sequence <= 9007199254740991),
    last_digest TEXT NOT NULL,
    phase TEXT NOT NULL,
    state_json BLOB NOT NULL
) STRICT;

CREATE TABLE artifact_blobs (
    content_digest TEXT PRIMARY KEY,
    byte_size INTEGER NOT NULL CHECK(byte_size >= 0 AND byte_size <= 9007199254740991),
    bytes BLOB NOT NULL,
    CHECK(length(bytes) = byte_size)
) STRICT;

PRAGMA user_version = 1;
"#;

pub(crate) fn ensure_schema(connection: &mut Connection) -> Result<(), RunError> {
    let version: i64 = connection
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .map_err(|error| migration_error("read SQLite user_version", error))?;

    match version {
        ECR_RUN_SCHEMA_VERSION => validate_v1_schema(connection),
        0 => migrate_v0_to_v1(connection),
        value if value > ECR_RUN_SCHEMA_VERSION => Err(RunError::new(
            RunErrorCategory::Migration,
            RunErrorCode::UnsupportedStoreVersion,
            format!(
                "SQLite schema user_version {value} is newer than supported version {ECR_RUN_SCHEMA_VERSION}"
            ),
        )),
        value => Err(RunError::new(
            RunErrorCategory::Migration,
            RunErrorCode::UnsupportedStoreVersion,
            format!("unsupported SQLite schema user_version {value}"),
        )),
    }
}

fn migrate_v0_to_v1(connection: &mut Connection) -> Result<(), RunError> {
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|error| migration_error("begin v0-to-v1 migration", error))?;
    transaction
        .execute_batch(SCHEMA_V1)
        .map_err(|error| migration_error("apply v1 schema", error))?;
    transaction
        .commit()
        .map_err(|error| migration_error("commit v1 schema", error))?;
    validate_v1_schema(connection)
}

fn validate_v1_schema(connection: &Connection) -> Result<(), RunError> {
    let object_count: i64 = connection
        .query_row(
            "SELECT count(*) FROM sqlite_schema WHERE \
             (type = 'table' AND name IN ('run_events', 'run_heads', 'artifact_blobs')) OR \
             (type = 'trigger' AND name IN ('run_events_no_update', 'run_events_no_delete'))",
            [],
            |row| row.get(0),
        )
        .map_err(|error| migration_error("inspect v1 schema objects", error))?;
    if object_count != 5 {
        return Err(RunError::new(
            RunErrorCategory::Migration,
            RunErrorCode::MigrationFailed,
            format!("v1 schema object count mismatch: expected 5, found {object_count}"),
        ));
    }

    let version: i64 = connection
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .map_err(|error| migration_error("verify SQLite user_version", error))?;
    if version != ECR_RUN_SCHEMA_VERSION {
        return Err(RunError::new(
            RunErrorCategory::Migration,
            RunErrorCode::MigrationFailed,
            format!(
                "v1 schema user_version mismatch: expected {ECR_RUN_SCHEMA_VERSION}, found {version}"
            ),
        ));
    }
    Ok(())
}

fn migration_error(context: &str, error: rusqlite::Error) -> RunError {
    RunError::new(
        RunErrorCategory::Migration,
        RunErrorCode::MigrationFailed,
        format!("{context}: {error}"),
    )
}
