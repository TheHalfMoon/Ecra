use std::{collections::BTreeSet, path::Path};

use ecra_core::VerificationReceipt;
use rusqlite::{Connection, TransactionBehavior, params};

use crate::{
    ReconciliationRecordV1, VerificationCheckpointV1, VerificationJournalBodyV1,
    VerificationJournalDigest, VerificationJournalEntryV1, VerificationJournalSequence,
    VerifyError, VerifyErrorCategory, VerifyErrorCode,
};

pub const ECR_VERIFY_SCHEMA_VERSION: i64 = 1;
pub const MAX_MATERIALIZED_JOURNAL_ENTRIES: usize = 4_096;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpectedVerificationHead {
    Empty,
    At {
        sequence: VerificationJournalSequence,
        digest: VerificationJournalDigest,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerificationSnapshotV1 {
    receipts: Vec<VerificationReceipt>,
    checkpoints: Vec<VerificationCheckpointV1>,
    reconciliations: Vec<ReconciliationRecordV1>,
}

impl VerificationSnapshotV1 {
    #[must_use]
    pub fn receipts(&self) -> &[VerificationReceipt] {
        &self.receipts
    }

    #[must_use]
    pub fn checkpoints(&self) -> &[VerificationCheckpointV1] {
        &self.checkpoints
    }

    #[must_use]
    pub fn reconciliations(&self) -> &[ReconciliationRecordV1] {
        &self.reconciliations
    }
}

pub struct VerificationStore {
    connection: Connection,
}

impl VerificationStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, VerifyError> {
        let connection =
            Connection::open(path).map_err(|_| store_error("open verification store"))?;
        ensure_schema(&connection)?;
        Ok(Self { connection })
    }

    pub fn head(&self) -> Result<Option<ExpectedVerificationHead>, VerifyError> {
        let entries = self.load_entries()?;
        Ok(entries.last().map(|entry| ExpectedVerificationHead::At {
            sequence: entry.sequence(),
            digest: entry.entry_digest().clone(),
        }))
    }

    pub fn append(
        &mut self,
        expected: &ExpectedVerificationHead,
        body: VerificationJournalBodyV1,
    ) -> Result<VerificationJournalEntryV1, VerifyError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| store_error("begin verification append transaction"))?;
        let existing = load_entries_from(&transaction)?;
        let actual = existing
            .last()
            .map(|entry| (entry.sequence(), entry.entry_digest().clone()));
        validate_expected_head(expected, actual.as_ref())?;
        reject_duplicate_identity(&existing, &body)?;

        let (sequence, previous_digest) = match actual {
            None => (VerificationJournalSequence::new(1)?, None),
            Some((sequence, digest)) => (sequence.checked_next()?, Some(digest)),
        };
        let entry = VerificationJournalEntryV1::new(sequence, previous_digest, body)?;
        let bytes = entry.canonical_bytes()?;
        if bytes.len() > crate::MAX_VERIFICATION_JOURNAL_ENTRY_BYTES {
            return Err(VerifyError::new(
                VerifyErrorCategory::ResourceLimit,
                VerifyErrorCode::ResourceLimitExceeded,
                "verification journal entry exceeds the v1 byte limit",
            ));
        }
        let entry_json = String::from_utf8(bytes).map_err(|_| {
            store_error("canonical verification journal bytes were not valid UTF-8")
        })?;
        let sequence_i64 = i64::try_from(entry.sequence().get()).map_err(|_| {
            VerifyError::new(
                VerifyErrorCategory::ResourceLimit,
                VerifyErrorCode::ResourceLimitExceeded,
                "verification journal sequence does not fit SQLite INTEGER",
            )
        })?;
        transaction
            .execute(
                "INSERT INTO verification_journal (sequence, entry_json, entry_digest) VALUES (?1, ?2, ?3)",
                params![sequence_i64, entry_json, entry.entry_digest().hex()],
            )
            .map_err(|_| store_error("insert authoritative verification journal entry"))?;
        insert_projection(&transaction, &entry)?;
        transaction
            .commit()
            .map_err(|_| store_error("commit verification append transaction"))?;
        Ok(entry)
    }

    pub fn load_entries(&self) -> Result<Vec<VerificationJournalEntryV1>, VerifyError> {
        load_entries_from(&self.connection)
    }

    pub fn snapshot(&self) -> Result<VerificationSnapshotV1, VerifyError> {
        let entries = self.load_entries()?;
        let mut receipts = Vec::new();
        let mut checkpoints = Vec::new();
        let mut reconciliations = Vec::new();
        for entry in entries {
            match entry.body() {
                VerificationJournalBodyV1::VerificationReceipt { receipt } => {
                    receipts.push(receipt.clone());
                }
                VerificationJournalBodyV1::CheckpointDefined { checkpoint } => {
                    checkpoints.push(checkpoint.clone());
                }
                VerificationJournalBodyV1::ReconciliationRecorded { record } => {
                    reconciliations.push(record.clone());
                }
            }
        }
        Ok(VerificationSnapshotV1 {
            receipts,
            checkpoints,
            reconciliations,
        })
    }

    pub fn rebuild_projections(&mut self) -> Result<(), VerifyError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| store_error("begin verification projection rebuild transaction"))?;
        let entries = load_entries_from(&transaction)?;
        transaction
            .execute_batch(
                "DELETE FROM verification_receipt_index;
                 DELETE FROM checkpoint_index;
                 DELETE FROM reconciliation_index;",
            )
            .map_err(|_| store_error("clear verification projection indexes"))?;
        for entry in &entries {
            insert_projection(&transaction, entry)?;
        }
        transaction
            .commit()
            .map_err(|_| store_error("commit verification projection rebuild"))?;
        Ok(())
    }
}

fn ensure_schema(connection: &Connection) -> Result<(), VerifyError> {
    let user_version: i64 = connection
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .map_err(|_| store_error("read verification store schema version"))?;
    if user_version > ECR_VERIFY_SCHEMA_VERSION {
        return Err(VerifyError::new(
            VerifyErrorCategory::Compatibility,
            VerifyErrorCode::UnsupportedVersion,
            "verification store schema is newer than supported v1",
        ));
    }
    if user_version == ECR_VERIFY_SCHEMA_VERSION {
        verify_schema_marker(connection)?;
        return Ok(());
    }
    if user_version != 0 {
        return Err(VerifyError::new(
            VerifyErrorCategory::Compatibility,
            VerifyErrorCode::UnsupportedVersion,
            "verification store schema version is unsupported",
        ));
    }

    let transaction = connection
        .unchecked_transaction()
        .map_err(|_| store_error("begin verification schema initialization"))?;
    transaction
        .execute_batch(
            "CREATE TABLE verification_meta (
                 singleton INTEGER PRIMARY KEY CHECK(singleton = 1),
                 schema_version INTEGER NOT NULL
             );
             CREATE TABLE verification_journal (
                 sequence INTEGER PRIMARY KEY,
                 entry_json TEXT NOT NULL,
                 entry_digest TEXT NOT NULL UNIQUE
             );
             CREATE TABLE verification_receipt_index (
                 verification_id TEXT PRIMARY KEY,
                 sequence INTEGER NOT NULL,
                 target_key TEXT NOT NULL
             );
             CREATE TABLE checkpoint_index (
                 checkpoint_id TEXT PRIMARY KEY,
                 sequence INTEGER NOT NULL
             );
             CREATE TABLE reconciliation_index (
                 reconciliation_id TEXT PRIMARY KEY,
                 run_id TEXT NOT NULL,
                 attempt_id TEXT NOT NULL,
                 sequence INTEGER NOT NULL
             );
             CREATE TRIGGER verification_journal_no_update
             BEFORE UPDATE ON verification_journal
             BEGIN
                 SELECT RAISE(ABORT, 'verification journal is append-only');
             END;
             CREATE TRIGGER verification_journal_no_delete
             BEFORE DELETE ON verification_journal
             BEGIN
                 SELECT RAISE(ABORT, 'verification journal is append-only');
             END;",
        )
        .map_err(|_| store_error("create verification schema v1"))?;
    transaction
        .execute(
            "INSERT INTO verification_meta (singleton, schema_version) VALUES (1, ?1)",
            params![ECR_VERIFY_SCHEMA_VERSION],
        )
        .map_err(|_| store_error("write verification schema marker"))?;
    transaction
        .pragma_update(None, "user_version", ECR_VERIFY_SCHEMA_VERSION)
        .map_err(|_| store_error("set verification store user_version"))?;
    transaction
        .commit()
        .map_err(|_| store_error("commit verification schema initialization"))?;
    Ok(())
}

fn verify_schema_marker(connection: &Connection) -> Result<(), VerifyError> {
    let marker: i64 = connection
        .query_row(
            "SELECT schema_version FROM verification_meta WHERE singleton = 1",
            [],
            |row| row.get(0),
        )
        .map_err(|_| store_error("read verification schema marker"))?;
    if marker != ECR_VERIFY_SCHEMA_VERSION {
        return Err(store_error(
            "verification schema marker does not match user_version",
        ));
    }
    let triggers: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master
             WHERE type = 'trigger'
               AND name IN ('verification_journal_no_update', 'verification_journal_no_delete')",
            [],
            |row| row.get(0),
        )
        .map_err(|_| store_error("inspect verification append-only triggers"))?;
    if triggers != 2 {
        return Err(store_error(
            "verification journal append-only triggers are missing or corrupt",
        ));
    }
    Ok(())
}

fn validate_expected_head(
    expected: &ExpectedVerificationHead,
    actual: Option<&(VerificationJournalSequence, VerificationJournalDigest)>,
) -> Result<(), VerifyError> {
    let matches = match (expected, actual) {
        (ExpectedVerificationHead::Empty, None) => true,
        (
            ExpectedVerificationHead::At { sequence, digest },
            Some((actual_sequence, actual_digest)),
        ) => sequence == actual_sequence && digest == actual_digest,
        _ => false,
    };
    if !matches {
        return Err(VerifyError::new(
            VerifyErrorCategory::Persistence,
            VerifyErrorCode::JournalDigestMismatch,
            "expected verification journal head does not match authoritative head",
        ));
    }
    Ok(())
}

fn reject_duplicate_identity(
    entries: &[VerificationJournalEntryV1],
    body: &VerificationJournalBodyV1,
) -> Result<(), VerifyError> {
    let duplicate = entries.iter().any(|entry| match (entry.body(), body) {
        (
            VerificationJournalBodyV1::VerificationReceipt { receipt: existing },
            VerificationJournalBodyV1::VerificationReceipt { receipt: proposed },
        ) => existing.id() == proposed.id(),
        (
            VerificationJournalBodyV1::CheckpointDefined {
                checkpoint: existing,
            },
            VerificationJournalBodyV1::CheckpointDefined {
                checkpoint: proposed,
            },
        ) => existing.id() == proposed.id(),
        (
            VerificationJournalBodyV1::ReconciliationRecorded { record: existing },
            VerificationJournalBodyV1::ReconciliationRecorded { record: proposed },
        ) => existing.id() == proposed.id(),
        _ => false,
    });
    if duplicate {
        return Err(VerifyError::new(
            VerifyErrorCategory::Persistence,
            VerifyErrorCode::DuplicateId,
            "verification journal identity already exists",
        ));
    }
    Ok(())
}

fn insert_projection(
    connection: &Connection,
    entry: &VerificationJournalEntryV1,
) -> Result<(), VerifyError> {
    let sequence = i64::try_from(entry.sequence().get()).map_err(|_| {
        store_error("verification projection sequence does not fit SQLite INTEGER")
    })?;
    match entry.body() {
        VerificationJournalBodyV1::VerificationReceipt { receipt } => {
            let target = serde_jcs::to_vec(receipt.target()).map_err(|_| {
                store_error("canonicalize verification receipt target for projection")
            })?;
            let target_key = String::from_utf8(target)
                .map_err(|_| store_error("verification target projection is not UTF-8"))?;
            connection
                .execute(
                    "INSERT INTO verification_receipt_index (verification_id, sequence, target_key) VALUES (?1, ?2, ?3)",
                    params![receipt.id().to_string(), sequence, target_key],
                )
                .map_err(|_| store_error("insert verification receipt projection"))?;
        }
        VerificationJournalBodyV1::CheckpointDefined { checkpoint } => {
            connection
                .execute(
                    "INSERT INTO checkpoint_index (checkpoint_id, sequence) VALUES (?1, ?2)",
                    params![checkpoint.id().to_string(), sequence],
                )
                .map_err(|_| store_error("insert checkpoint projection"))?;
        }
        VerificationJournalBodyV1::ReconciliationRecorded { record } => {
            connection
                .execute(
                    "INSERT INTO reconciliation_index (reconciliation_id, run_id, attempt_id, sequence) VALUES (?1, ?2, ?3, ?4)",
                    params![
                        record.id().to_string(),
                        record.run_id().to_string(),
                        record.attempt().id().to_string(),
                        sequence
                    ],
                )
                .map_err(|_| store_error("insert reconciliation projection"))?;
        }
    }
    Ok(())
}

fn load_entries_from(
    connection: &Connection,
) -> Result<Vec<VerificationJournalEntryV1>, VerifyError> {
    let limit = i64::try_from(MAX_MATERIALIZED_JOURNAL_ENTRIES + 1)
        .map_err(|_| store_error("verification materialization limit is invalid"))?;
    let mut statement = connection
        .prepare(
            "SELECT sequence, entry_json, entry_digest FROM verification_journal ORDER BY sequence LIMIT ?1",
        )
        .map_err(|_| store_error("prepare verification journal replay"))?;
    let rows = statement
        .query_map(params![limit], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })
        .map_err(|_| store_error("query verification journal replay"))?;

    let mut entries = Vec::new();
    for row in rows {
        let (stored_sequence, entry_json, stored_digest) =
            row.map_err(|_| store_error("read verification journal replay row"))?;
        let entry = VerificationJournalEntryV1::from_json_slice(entry_json.as_bytes())
            .map_err(|_| store_error("verification journal contains an invalid entry"))?;
        let stored_sequence = u64::try_from(stored_sequence)
            .map_err(|_| store_error("stored verification sequence is negative"))?;
        if entry.sequence().get() != stored_sequence || entry.entry_digest().hex() != stored_digest {
            return Err(store_error(
                "verification journal row metadata does not match authoritative entry JSON",
            ));
        }
        entries.push(entry);
    }
    if entries.len() > MAX_MATERIALIZED_JOURNAL_ENTRIES {
        return Err(VerifyError::new(
            VerifyErrorCategory::ResourceLimit,
            VerifyErrorCode::ResourceLimitExceeded,
            "verification journal query exceeds the v1 materialization limit",
        ));
    }
    validate_chain(&entries)?;
    Ok(entries)
}

fn validate_chain(entries: &[VerificationJournalEntryV1]) -> Result<(), VerifyError> {
    let mut previous: Option<&VerificationJournalEntryV1> = None;
    let mut identities = BTreeSet::new();
    for entry in entries {
        match previous {
            None => {
                if entry.sequence().get() != 1 || entry.previous_digest().is_some() {
                    return Err(VerifyError::new(
                        VerifyErrorCategory::Persistence,
                        VerifyErrorCode::JournalSequenceMismatch,
                        "verification journal does not begin at canonical genesis sequence",
                    ));
                }
            }
            Some(prior) => {
                if entry.sequence() != prior.sequence().checked_next()? {
                    return Err(VerifyError::new(
                        VerifyErrorCategory::Persistence,
                        VerifyErrorCode::JournalSequenceMismatch,
                        "verification journal contains a sequence gap or reorder",
                    ));
                }
                if entry.previous_digest() != Some(prior.entry_digest()) {
                    return Err(VerifyError::new(
                        VerifyErrorCategory::Persistence,
                        VerifyErrorCode::JournalDigestMismatch,
                        "verification journal previous digest does not match prior entry",
                    ));
                }
            }
        }
        let identity = match entry.body() {
            VerificationJournalBodyV1::VerificationReceipt { receipt } => {
                format!("verification:{}", receipt.id())
            }
            VerificationJournalBodyV1::CheckpointDefined { checkpoint } => {
                format!("checkpoint:{}", checkpoint.id())
            }
            VerificationJournalBodyV1::ReconciliationRecorded { record } => {
                format!("reconciliation:{}", record.id())
            }
        };
        if !identities.insert(identity) {
            return Err(VerifyError::new(
                VerifyErrorCategory::Persistence,
                VerifyErrorCode::DuplicateId,
                "verification journal contains a duplicate canonical identity",
            ));
        }
        previous = Some(entry);
    }
    Ok(())
}

fn store_error(diagnostic: &'static str) -> VerifyError {
    VerifyError::new(
        VerifyErrorCategory::Persistence,
        VerifyErrorCode::StoreCorrupt,
        diagnostic,
    )
}
