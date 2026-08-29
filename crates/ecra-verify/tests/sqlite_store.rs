use ecra_core::{
    ActionAttemptId, ActionAttemptRef, ActionDigest, ActionId, ActionRef, ActorId, ClaimRef,
    EvidenceId, EvidenceKind, EvidenceRef, RunId, SecurityDigest, VerificationId,
    VerificationMethod, VerificationOutcome, VerificationReceipt, VerificationTarget,
};
use ecra_verify::{
    CheckpointId, ExpectedVerificationHead, ReconciliationId, ReconciliationOutcomeV1,
    ReconciliationRecordFieldsV1, ReconciliationRecordV1, VerificationAggregateStateV1,
    VerificationAggregateViewV1, VerificationCheckpointFieldsV1, VerificationCheckpointV1,
    VerificationJournalBodyV1, VerificationJournalDigest, VerificationJournalEntryV1,
    VerificationJournalSequence, VerificationRequirementV1, VerificationStore, VerifyErrorCode,
};
use rusqlite::{Connection, params};
use tempfile::tempdir;

const SECRET_SENTINEL: &str = "ECRA_TEST_SECRET_DO_NOT_PERSIST";

fn target() -> VerificationTarget {
    VerificationTarget::Claim(ClaimRef::new("store", "replay").expect("claim target"))
}

fn receipt() -> VerificationReceipt {
    VerificationReceipt::new(
        VerificationId::parse_str("00000000-0000-0000-0000-000000091001")
            .expect("verification id"),
        ActorId::parse_str("00000000-0000-0000-0000-000000000001").expect("actor id"),
        target(),
        VerificationMethod::DeterministicComputation,
        VerificationOutcome::Verified,
        vec![EvidenceRef::new(
            EvidenceId::parse_str("00000000-0000-0000-0000-000000091002")
                .expect("evidence id"),
            EvidenceKind::Computation,
        )],
    )
    .expect("verification receipt")
}

fn checkpoint() -> VerificationCheckpointV1 {
    VerificationCheckpointV1::from_fields(VerificationCheckpointFieldsV1 {
        id: CheckpointId::parse_str("00000000-0000-0000-0000-000000091003")
            .expect("checkpoint id"),
        label: "synthetic replay checkpoint".to_owned(),
        requirements: vec![VerificationRequirementV1::new(
            target(),
            vec![VerificationAggregateStateV1::Verified],
        )
        .expect("verification requirement")],
    })
    .expect("verification checkpoint")
}

fn action_ref() -> ActionRef {
    ActionRef::new(
        ActionId::parse_str("00000000-0000-0000-0000-000000091010").expect("action id"),
        ActionDigest::new(SecurityDigest::sha256(b"synthetic-store-action")),
    )
}

fn reconciliation() -> ReconciliationRecordV1 {
    let action = action_ref();
    let attempt = ActionAttemptRef::new(
        ActionAttemptId::parse_str("00000000-0000-0000-0000-000000091011")
            .expect("attempt id"),
        action.clone(),
    );
    ReconciliationRecordV1::from_fields(ReconciliationRecordFieldsV1 {
        id: ReconciliationId::parse_str("00000000-0000-0000-0000-000000091012")
            .expect("reconciliation id"),
        run_id: RunId::parse_str("00000000-0000-0000-0000-000000091013").expect("run id"),
        attempt,
        action,
        outcome: ReconciliationOutcomeV1::StillUnknown,
        verification_receipts: Vec::new(),
        reconciled_at: None,
        notes: Some("synthetic effect remains unknown".to_owned()),
    })
    .expect("reconciliation record")
}

fn at(entry: &VerificationJournalEntryV1) -> ExpectedVerificationHead {
    ExpectedVerificationHead::At {
        sequence: entry.sequence(),
        digest: entry.entry_digest().clone(),
    }
}

fn trigger_sql() -> &'static str {
    "CREATE TRIGGER verification_journal_no_update
     BEFORE UPDATE ON verification_journal
     BEGIN
         SELECT RAISE(ABORT, 'verification journal is append-only');
     END;
     CREATE TRIGGER verification_journal_no_delete
     BEFORE DELETE ON verification_journal
     BEGIN
         SELECT RAISE(ABORT, 'verification journal is append-only');
     END;"
}

fn drop_triggers(connection: &Connection) {
    connection
        .execute_batch(
            "DROP TRIGGER verification_journal_no_update;
             DROP TRIGGER verification_journal_no_delete;",
        )
        .expect("drop append-only triggers for corruption fixture");
}

fn restore_triggers(connection: &Connection) {
    connection
        .execute_batch(trigger_sql())
        .expect("restore append-only triggers after corruption fixture");
}

#[test]
fn append_reopen_replay_preserves_derived_views() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("verification.sqlite");
    let mut store = VerificationStore::open(&path).expect("open store");

    let first = store
        .append(
            &ExpectedVerificationHead::Empty,
            VerificationJournalBodyV1::VerificationReceipt { receipt: receipt() },
        )
        .expect("append receipt");
    let second = store
        .append(
            &at(&first),
            VerificationJournalBodyV1::CheckpointDefined {
                checkpoint: checkpoint(),
            },
        )
        .expect("append checkpoint");
    store
        .append(
            &at(&second),
            VerificationJournalBodyV1::ReconciliationRecorded {
                record: reconciliation(),
            },
        )
        .expect("append reconciliation");

    let before = store.snapshot().expect("snapshot before reopen");
    let aggregate_before =
        VerificationAggregateViewV1::from_receipts(target(), before.receipts()).expect("aggregate");
    let checkpoint_before = before.checkpoints()[0]
        .evaluate(std::slice::from_ref(&aggregate_before))
        .expect("checkpoint evaluation");
    let aggregate_bytes_before = serde_jcs::to_vec(&aggregate_before).expect("aggregate JCS");
    let checkpoint_bytes_before = serde_jcs::to_vec(&checkpoint_before).expect("checkpoint JCS");
    drop(store);

    let reopened = VerificationStore::open(&path).expect("reopen store");
    let after = reopened.snapshot().expect("snapshot after reopen");
    assert_eq!(after, before);
    let entries = reopened.load_entries().expect("replayed entries");
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[1].previous_digest(), Some(entries[0].entry_digest()));
    assert_eq!(entries[2].previous_digest(), Some(entries[1].entry_digest()));

    let aggregate_after =
        VerificationAggregateViewV1::from_receipts(target(), after.receipts()).expect("aggregate");
    let checkpoint_after = after.checkpoints()[0]
        .evaluate(std::slice::from_ref(&aggregate_after))
        .expect("checkpoint evaluation");
    assert_eq!(
        serde_jcs::to_vec(&aggregate_after).expect("aggregate JCS"),
        aggregate_bytes_before
    );
    assert_eq!(
        serde_jcs::to_vec(&checkpoint_after).expect("checkpoint JCS"),
        checkpoint_bytes_before
    );
    assert_eq!(after.reconciliations(), before.reconciliations());
}

#[test]
fn stale_expected_head_allows_exactly_one_competing_append() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("verification.sqlite");
    let mut first_writer = VerificationStore::open(&path).expect("first writer");
    let mut second_writer = VerificationStore::open(&path).expect("second writer");

    let genesis = first_writer
        .append(
            &ExpectedVerificationHead::Empty,
            VerificationJournalBodyV1::VerificationReceipt { receipt: receipt() },
        )
        .expect("genesis append");
    let stale = at(&genesis);
    first_writer
        .append(
            &stale,
            VerificationJournalBodyV1::CheckpointDefined {
                checkpoint: checkpoint(),
            },
        )
        .expect("first competing append wins");
    let error = second_writer
        .append(
            &stale,
            VerificationJournalBodyV1::ReconciliationRecorded {
                record: reconciliation(),
            },
        )
        .expect_err("stale competing append must fail");
    assert_eq!(error.code(), VerifyErrorCode::JournalDigestMismatch);
    assert_eq!(first_writer.load_entries().expect("entries").len(), 2);
}

#[test]
fn schema_is_append_only_and_projection_indexes_are_rebuildable() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("verification.sqlite");
    let mut store = VerificationStore::open(&path).expect("open store");
    store
        .append(
            &ExpectedVerificationHead::Empty,
            VerificationJournalBodyV1::VerificationReceipt { receipt: receipt() },
        )
        .expect("append receipt");
    drop(store);

    let connection = Connection::open(&path).expect("direct connection");
    assert!(
        connection
            .execute(
                "UPDATE verification_journal SET entry_json = entry_json WHERE sequence = 1",
                [],
            )
            .is_err()
    );
    assert!(
        connection
            .execute("DELETE FROM verification_journal WHERE sequence = 1", [])
            .is_err()
    );
    connection
        .execute("DELETE FROM verification_receipt_index", [])
        .expect("projection deletion is allowed");
    connection
        .execute(
            "INSERT INTO verification_receipt_index (verification_id, sequence, target_key)
             VALUES ('00000000-0000-0000-0000-000000099999', 999, 'poison')",
            [],
        )
        .expect("poison projection fixture");
    drop(connection);

    let mut reopened = VerificationStore::open(&path).expect("reopen store");
    reopened.rebuild_projections().expect("rebuild projections");
    drop(reopened);
    let connection = Connection::open(&path).expect("inspect projections");
    let real: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM verification_receipt_index WHERE verification_id = ?1",
            params![receipt().id().to_string()],
            |row| row.get(0),
        )
        .expect("real projection count");
    let poison: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM verification_receipt_index WHERE verification_id = '00000000-0000-0000-0000-000000099999'",
            [],
            |row| row.get(0),
        )
        .expect("poison projection count");
    let journal: i64 = connection
        .query_row("SELECT COUNT(*) FROM verification_journal", [], |row| row.get(0))
        .expect("journal count");
    assert_eq!(real, 1);
    assert_eq!(poison, 0);
    assert_eq!(journal, 1);
}

#[test]
fn duplicate_identity_is_rejected_even_when_projection_is_missing() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("verification.sqlite");
    let mut store = VerificationStore::open(&path).expect("open store");
    let first = store
        .append(
            &ExpectedVerificationHead::Empty,
            VerificationJournalBodyV1::VerificationReceipt { receipt: receipt() },
        )
        .expect("append receipt");
    drop(store);

    let connection = Connection::open(&path).expect("direct connection");
    connection
        .execute("DELETE FROM verification_receipt_index", [])
        .expect("delete projection");
    drop(connection);

    let mut reopened = VerificationStore::open(&path).expect("reopen store");
    let error = reopened
        .append(
            &at(&first),
            VerificationJournalBodyV1::VerificationReceipt { receipt: receipt() },
        )
        .expect_err("authoritative duplicate identity must fail");
    assert_eq!(error.code(), VerifyErrorCode::DuplicateId);
}

#[test]
fn malformed_and_metadata_corruption_are_detected() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("verification.sqlite");
    let mut store = VerificationStore::open(&path).expect("open store");
    store
        .append(
            &ExpectedVerificationHead::Empty,
            VerificationJournalBodyV1::VerificationReceipt { receipt: receipt() },
        )
        .expect("append receipt");
    drop(store);

    let connection = Connection::open(&path).expect("direct connection");
    drop_triggers(&connection);
    connection
        .execute(
            "UPDATE verification_journal SET entry_json = 'not-json' WHERE sequence = 1",
            [],
        )
        .expect("corrupt entry json");
    restore_triggers(&connection);
    drop(connection);
    let reopened = VerificationStore::open(&path).expect("schema remains valid");
    let error = reopened
        .load_entries()
        .expect_err("malformed entry must fail replay");
    assert_eq!(error.code(), VerifyErrorCode::StoreCorrupt);
}

#[test]
fn previous_digest_and_sequence_chain_corruption_are_detected() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("verification.sqlite");
    let mut store = VerificationStore::open(&path).expect("open store");
    let first = store
        .append(
            &ExpectedVerificationHead::Empty,
            VerificationJournalBodyV1::VerificationReceipt { receipt: receipt() },
        )
        .expect("append receipt");
    store
        .append(
            &at(&first),
            VerificationJournalBodyV1::CheckpointDefined {
                checkpoint: checkpoint(),
            },
        )
        .expect("append checkpoint");
    drop(store);

    let wrong_previous = VerificationJournalDigest::new_sha256(
        "1111111111111111111111111111111111111111111111111111111111111111",
    )
    .expect("wrong digest fixture");
    let forged = VerificationJournalEntryV1::new(
        VerificationJournalSequence::new(2).expect("sequence"),
        Some(wrong_previous),
        VerificationJournalBodyV1::CheckpointDefined {
            checkpoint: checkpoint(),
        },
    )
    .expect("self-consistent forged entry");
    let forged_json = String::from_utf8(forged.canonical_bytes().expect("forged bytes"))
        .expect("forged UTF-8");
    let connection = Connection::open(&path).expect("direct connection");
    drop_triggers(&connection);
    connection
        .execute(
            "UPDATE verification_journal SET entry_json = ?1, entry_digest = ?2 WHERE sequence = 2",
            params![forged_json, forged.entry_digest().hex()],
        )
        .expect("forge previous digest");
    restore_triggers(&connection);
    drop(connection);
    let reopened = VerificationStore::open(&path).expect("reopen store");
    let error = reopened
        .load_entries()
        .expect_err("previous digest mismatch must fail replay");
    assert_eq!(error.code(), VerifyErrorCode::JournalDigestMismatch);

    let second_directory = tempdir().expect("tempdir");
    let second_path = second_directory.path().join("verification.sqlite");
    let mut store = VerificationStore::open(&second_path).expect("open store");
    let first = store
        .append(
            &ExpectedVerificationHead::Empty,
            VerificationJournalBodyV1::VerificationReceipt { receipt: receipt() },
        )
        .expect("append receipt");
    store
        .append(
            &at(&first),
            VerificationJournalBodyV1::CheckpointDefined {
                checkpoint: checkpoint(),
            },
        )
        .expect("append checkpoint");
    drop(store);
    let forged_gap = VerificationJournalEntryV1::new(
        VerificationJournalSequence::new(3).expect("gap sequence"),
        Some(first.entry_digest().clone()),
        VerificationJournalBodyV1::CheckpointDefined {
            checkpoint: checkpoint(),
        },
    )
    .expect("self-consistent gap entry");
    let forged_gap_json = String::from_utf8(forged_gap.canonical_bytes().expect("gap bytes"))
        .expect("gap UTF-8");
    let connection = Connection::open(&second_path).expect("direct connection");
    drop_triggers(&connection);
    connection
        .execute(
            "UPDATE verification_journal SET sequence = 3, entry_json = ?1, entry_digest = ?2 WHERE sequence = 2",
            params![forged_gap_json, forged_gap.entry_digest().hex()],
        )
        .expect("forge sequence gap");
    restore_triggers(&connection);
    drop(connection);
    let reopened = VerificationStore::open(&second_path).expect("reopen store");
    let error = reopened
        .load_entries()
        .expect_err("sequence gap must fail replay");
    assert_eq!(error.code(), VerifyErrorCode::JournalSequenceMismatch);
}

#[test]
fn synthetic_journal_never_contains_secret_sentinel() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("verification.sqlite");
    let mut store = VerificationStore::open(&path).expect("open store");
    store
        .append(
            &ExpectedVerificationHead::Empty,
            VerificationJournalBodyV1::VerificationReceipt { receipt: receipt() },
        )
        .expect("append synthetic receipt");
    let snapshot = store.snapshot().expect("snapshot");
    assert!(!format!("{snapshot:?}").contains(SECRET_SENTINEL));
    drop(store);

    let connection = Connection::open(&path).expect("direct connection");
    let journal: String = connection
        .query_row(
            "SELECT COALESCE(group_concat(entry_json, ''), '') FROM verification_journal",
            [],
            |row| row.get(0),
        )
        .expect("journal text");
    assert!(!journal.contains(SECRET_SENTINEL));
}
