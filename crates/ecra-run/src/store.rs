use std::path::Path;

use ecra_core::{ContentDigest, RunId, to_jcs_vec};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use sha2::{Digest, Sha256};

use crate::migration::ensure_schema;
use crate::sqlite::{map_sqlite, open_configured, read_configuration};
use crate::{
    BudgetAmount, EventSequence, LedgerDigest, RunError, RunErrorCategory, RunErrorCode,
    RunEventEnvelope, RunPhase, RunReducer, RunState, SqliteConfiguration,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpectedRunHead {
    Genesis,
    At {
        sequence: EventSequence,
        digest: LedgerDigest,
    },
}

pub struct RunStore {
    connection: Connection,
}

impl RunStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, RunError> {
        let mut connection = open_configured(path)?;
        ensure_schema(&mut connection)?;
        Ok(Self { connection })
    }

    pub fn sqlite_configuration(&self) -> Result<SqliteConfiguration, RunError> {
        read_configuration(&self.connection)
    }

    pub fn append(
        &mut self,
        expected: &ExpectedRunHead,
        envelope: &RunEventEnvelope,
    ) -> Result<RunState, RunError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| map_sqlite("begin immediate append transaction", error))?;
        let run_id = envelope.run_id();
        let actual_head = authoritative_head(&transaction, run_id)?;
        validate_expected_head(expected, actual_head.as_ref())?;

        let history = load_history_from(&transaction, run_id)?;
        let next_state = if history.is_empty() {
            RunReducer::reduce(std::slice::from_ref(envelope))?
        } else {
            let current = RunReducer::reduce(&history)?;
            RunReducer::apply(&current, envelope)?
        };

        let event_json =
            to_jcs_vec(envelope).map_err(|error| RunError::serialization(error.to_string()))?;
        let sequence = i64::try_from(envelope.sequence().get()).map_err(|_| {
            RunError::new(
                RunErrorCategory::Storage,
                RunErrorCode::StorageError,
                "event sequence does not fit SQLite INTEGER",
            )
        })?;
        let previous_digest = envelope.previous_digest().map(LedgerDigest::hex);
        transaction
            .execute(
                "INSERT INTO run_events \
                 (run_id, sequence, event_digest, previous_digest, event_json) \
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    run_id.to_string(),
                    sequence,
                    envelope.event_digest().hex(),
                    previous_digest,
                    event_json
                ],
            )
            .map_err(|error| map_sqlite("insert authoritative run event", error))?;

        write_projection(&transaction, &next_state)?;
        transaction
            .commit()
            .map_err(|error| map_sqlite("commit append transaction", error))?;
        Ok(next_state)
    }

    pub fn load_history(&self, run_id: RunId) -> Result<Vec<RunEventEnvelope>, RunError> {
        load_history_from(&self.connection, run_id)
    }

    pub fn load_state(&self, run_id: RunId) -> Result<Option<RunState>, RunError> {
        let history = self.load_history(run_id)?;
        if history.is_empty() {
            Ok(None)
        } else {
            RunReducer::reduce(&history).map(Some)
        }
    }

    pub fn rebuild_projection(&mut self, run_id: RunId) -> Result<Option<RunState>, RunError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| map_sqlite("begin projection rebuild transaction", error))?;
        let history = load_history_from(&transaction, run_id)?;
        let state = if history.is_empty() {
            None
        } else {
            Some(RunReducer::reduce(&history)?)
        };

        transaction
            .execute(
                "DELETE FROM run_heads WHERE run_id = ?1",
                params![run_id.to_string()],
            )
            .map_err(|error| map_sqlite("delete stale run projection", error))?;
        if let Some(state) = &state {
            write_projection(&transaction, state)?;
        }
        transaction
            .commit()
            .map_err(|error| map_sqlite("commit projection rebuild", error))?;
        Ok(state)
    }

    pub fn put_blob(
        &mut self,
        digest: &ContentDigest,
        declared_size: BudgetAmount,
        bytes: &[u8],
        remaining_storage: Option<BudgetAmount>,
    ) -> Result<(), RunError> {
        let actual_size = BudgetAmount::new(u64::try_from(bytes.len()).map_err(|_| {
            RunError::new(
                RunErrorCategory::Storage,
                RunErrorCode::StorageError,
                "blob byte length does not fit u64",
            )
        })?)?;
        if actual_size != declared_size {
            return Err(RunError::new(
                RunErrorCategory::Integrity,
                RunErrorCode::StorageError,
                "declared blob size does not match materialized bytes",
            ));
        }
        if remaining_storage.is_some_and(|remaining| declared_size > remaining) {
            return Err(RunError::new(
                RunErrorCategory::Budget,
                RunErrorCode::BudgetPreflightExceeded,
                "blob storage exceeds remaining storage budget",
            ));
        }
        verify_content_digest(digest, bytes)?;
        let key = content_digest_key(digest);
        let size = i64::try_from(declared_size.get()).map_err(|_| {
            RunError::new(
                RunErrorCategory::Storage,
                RunErrorCode::StorageError,
                "blob size does not fit SQLite INTEGER",
            )
        })?;

        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| map_sqlite("begin blob transaction", error))?;
        let existing: Option<(i64, Vec<u8>)> = transaction
            .query_row(
                "SELECT byte_size, bytes FROM artifact_blobs WHERE content_digest = ?1",
                params![key],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()
            .map_err(|error| map_sqlite("read existing artifact blob", error))?;
        if let Some((existing_size, existing_bytes)) = existing {
            if existing_size != size || existing_bytes != bytes {
                return Err(RunError::new(
                    RunErrorCategory::Integrity,
                    RunErrorCode::StorageError,
                    "content-addressed blob row conflicts with existing bytes",
                ));
            }
            transaction
                .commit()
                .map_err(|error| map_sqlite("commit idempotent blob transaction", error))?;
            return Ok(());
        }

        transaction
            .execute(
                "INSERT INTO artifact_blobs (content_digest, byte_size, bytes) VALUES (?1, ?2, ?3)",
                params![content_digest_key(digest), size, bytes],
            )
            .map_err(|error| map_sqlite("insert artifact blob", error))?;
        transaction
            .commit()
            .map_err(|error| map_sqlite("commit blob transaction", error))?;
        Ok(())
    }

    pub fn get_blob(&self, digest: &ContentDigest) -> Result<Option<Vec<u8>>, RunError> {
        let key = content_digest_key(digest);
        let row: Option<(i64, Vec<u8>)> = self
            .connection
            .query_row(
                "SELECT byte_size, bytes FROM artifact_blobs WHERE content_digest = ?1",
                params![key],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()
            .map_err(|error| map_sqlite("read artifact blob", error))?;
        let Some((declared_size, bytes)) = row else {
            return Ok(None);
        };
        let actual_size = i64::try_from(bytes.len()).map_err(|_| {
            RunError::new(
                RunErrorCategory::Integrity,
                RunErrorCode::StorageError,
                "stored blob length does not fit SQLite INTEGER",
            )
        })?;
        if declared_size != actual_size {
            return Err(RunError::new(
                RunErrorCategory::Integrity,
                RunErrorCode::StorageError,
                "stored blob size does not match materialized bytes",
            ));
        }
        verify_content_digest(digest, &bytes)?;
        Ok(Some(bytes))
    }
}

fn authoritative_head(
    connection: &Connection,
    run_id: RunId,
) -> Result<Option<(EventSequence, LedgerDigest)>, RunError> {
    let row: Option<(i64, String)> = connection
        .query_row(
            "SELECT sequence, event_digest FROM run_events WHERE run_id = ?1 \
             ORDER BY sequence DESC LIMIT 1",
            params![run_id.to_string()],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
        .map_err(|error| map_sqlite("read authoritative run head", error))?;
    row.map(|(sequence, digest)| {
        Ok((
            event_sequence_from_sql(sequence)?,
            LedgerDigest::new_sha256(digest)?,
        ))
    })
    .transpose()
}

fn validate_expected_head(
    expected: &ExpectedRunHead,
    actual: Option<&(EventSequence, LedgerDigest)>,
) -> Result<(), RunError> {
    let matches = match (expected, actual) {
        (ExpectedRunHead::Genesis, None) => true,
        (ExpectedRunHead::At { sequence, digest }, Some((actual_sequence, actual_digest))) => {
            sequence == actual_sequence && digest == actual_digest
        }
        _ => false,
    };
    if matches {
        Ok(())
    } else {
        Err(RunError::new(
            RunErrorCategory::Ledger,
            RunErrorCode::LedgerHeadMismatch,
            "authoritative run head does not match caller expectation",
        ))
    }
}

fn load_history_from(
    connection: &Connection,
    run_id: RunId,
) -> Result<Vec<RunEventEnvelope>, RunError> {
    let mut statement = connection
        .prepare(
            "SELECT sequence, event_digest, previous_digest, event_json \
             FROM run_events WHERE run_id = ?1 ORDER BY sequence ASC",
        )
        .map_err(|error| map_sqlite("prepare run history load", error))?;
    let rows = statement
        .query_map(params![run_id.to_string()], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, Vec<u8>>(3)?,
            ))
        })
        .map_err(|error| map_sqlite("query run history", error))?;

    let mut history = Vec::new();
    for row in rows {
        let (sequence_sql, digest_sql, previous_sql, event_json) =
            row.map_err(|error| map_sqlite("read run history row", error))?;
        let sequence = event_sequence_from_sql(sequence_sql)?;
        let envelope = RunEventEnvelope::from_json_slice(&event_json)?;
        if envelope.run_id() != run_id || envelope.sequence() != sequence {
            return Err(RunError::ledger_chain_invalid(
                "stored run-event row identity does not match canonical envelope",
            ));
        }
        if envelope.event_digest().hex() != digest_sql {
            return Err(RunError::ledger_digest_mismatch(
                "stored run-event digest column does not match canonical envelope",
            ));
        }
        if envelope.previous_digest().map(LedgerDigest::hex) != previous_sql.as_deref() {
            return Err(RunError::ledger_chain_invalid(
                "stored previous_digest column does not match canonical envelope",
            ));
        }
        if let Some(previous) = history.last() {
            envelope.validate_successor(previous)?;
        }
        history.push(envelope);
    }
    Ok(history)
}

fn write_projection(connection: &Connection, state: &RunState) -> Result<(), RunError> {
    let state_json = state.canonical_bytes()?;
    let sequence = i64::try_from(state.last_sequence().get()).map_err(|_| {
        RunError::new(
            RunErrorCategory::Storage,
            RunErrorCode::StorageError,
            "projection sequence does not fit SQLite INTEGER",
        )
    })?;
    connection
        .execute(
            "INSERT INTO run_heads (run_id, last_sequence, last_digest, phase, state_json) \
             VALUES (?1, ?2, ?3, ?4, ?5) \
             ON CONFLICT(run_id) DO UPDATE SET \
             last_sequence = excluded.last_sequence, \
             last_digest = excluded.last_digest, \
             phase = excluded.phase, \
             state_json = excluded.state_json",
            params![
                state.run_id().to_string(),
                sequence,
                state.last_digest().hex(),
                phase_name(state.phase()),
                state_json
            ],
        )
        .map_err(|error| map_sqlite("publish run projection", error))?;
    Ok(())
}

fn phase_name(phase: RunPhase) -> &'static str {
    match phase {
        RunPhase::Created => "created",
        RunPhase::Running => "running",
        RunPhase::Suspended => "suspended",
        RunPhase::CancellationRequested => "cancellation_requested",
        RunPhase::Cancelled => "cancelled",
        RunPhase::Failed => "failed",
        RunPhase::ExecutionCompleted => "execution_completed",
    }
}

fn event_sequence_from_sql(value: i64) -> Result<EventSequence, RunError> {
    let value = u64::try_from(value).map_err(|_| {
        RunError::new(
            RunErrorCategory::Ledger,
            RunErrorCode::InvalidEventSequence,
            "stored event sequence is negative",
        )
    })?;
    EventSequence::new(value)
}

fn content_digest_key(digest: &ContentDigest) -> String {
    format!("{}:{}", digest.algorithm(), digest.hex())
}

fn verify_content_digest(digest: &ContentDigest, bytes: &[u8]) -> Result<(), RunError> {
    if digest.algorithm() != "sha256" || digest.hex().len() != 64 {
        return Err(RunError::new(
            RunErrorCategory::Integrity,
            RunErrorCode::StorageError,
            "ECR-002 blob storage supports only canonical sha256 ContentDigest values",
        ));
    }
    let actual_hex = sha256_hex(bytes);
    if actual_hex != digest.hex() {
        return Err(RunError::new(
            RunErrorCategory::Integrity,
            RunErrorCode::StorageError,
            "content digest does not match blob bytes",
        ));
    }
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let digest = Sha256::digest(bytes);
    let mut hex = String::with_capacity(64);
    for byte in digest {
        hex.push(char::from(HEX[usize::from(byte >> 4)]));
        hex.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    hex
}
