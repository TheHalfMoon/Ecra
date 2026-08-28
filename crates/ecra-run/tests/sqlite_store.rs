use ecra_core::{ContentDigest, EpochMillis, RunId, to_jcs_vec};
use ecra_run::{
    BudgetAmount, ExpectedRunHead, LedgerDigest, RunErrorCode, RunEvent, RunEventEnvelope, RunPhase,
    RunStore,
};
use rusqlite::{Connection, params};
use sha2::{Digest, Sha256};
use tempfile::tempdir;

fn genesis() -> RunEventEnvelope {
    RunEventEnvelope::from_json_slice(include_bytes!(
        "../../../contracts/ecra-run-v1/valid/run-created-envelope.v1.json"
    ))
    .expect("genesis fixture")
}

fn fixture_event(kind: &str) -> RunEvent {
    let events: Vec<RunEvent> = serde_json::from_slice(include_bytes!(
        "../../../contracts/ecra-run-v1/valid/all-event-kinds.v1.json"
    ))
    .expect("event fixtures");
    events
        .into_iter()
        .find(|candidate| candidate.kind() == kind)
        .unwrap_or_else(|| panic!("missing fixture {kind}"))
}

fn successor(previous: &RunEventEnvelope, event: RunEvent) -> RunEventEnvelope {
    RunEventEnvelope::new(
        previous.run_id(),
        previous.sequence().checked_next().expect("next sequence"),
        EpochMillis::new(previous.recorded_at().get() + 1).expect("timestamp"),
        Some(previous.event_digest().clone()),
        event,
    )
    .expect("successor envelope")
}

fn expected(envelope: &RunEventEnvelope) -> ExpectedRunHead {
    ExpectedRunHead::At {
        sequence: envelope.sequence(),
        digest: envelope.event_digest().clone(),
    }
}

fn sha256_digest(bytes: &[u8]) -> ContentDigest {
    let hex = format!("{:x}", Sha256::digest(bytes));
    ContentDigest::new("sha256", hex).expect("content digest")
}

#[test]
fn configured_store_appends_atomically_and_loads_authoritative_history() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("store.db");
    let mut store = RunStore::open(&path).expect("open store");
    let configuration = store.sqlite_configuration().expect("configuration");
    assert_eq!(configuration.journal_mode(), "wal");
    assert_eq!(configuration.synchronous(), 2);
    assert!(configuration.foreign_keys());
    assert!(!configuration.trusted_schema());

    let created = genesis();
    let created_state = store
        .append(&ExpectedRunHead::Genesis, &created)
        .expect("append genesis");
    assert_eq!(created_state.phase(), RunPhase::Created);

    let duplicate_error = store
        .append(&ExpectedRunHead::Genesis, &created)
        .expect_err("stale genesis expectation must fail");
    assert_eq!(duplicate_error.code(), RunErrorCode::LedgerHeadMismatch);

    let started = successor(&created, fixture_event("run_started"));
    let running_state = store
        .append(&expected(&created), &started)
        .expect("append run_started");
    assert_eq!(running_state.phase(), RunPhase::Running);

    let history = store.load_history(created.run_id()).expect("load history");
    assert_eq!(history, vec![created.clone(), started.clone()]);
    let derived = store
        .load_state(created.run_id())
        .expect("load state")
        .expect("state exists");
    assert_eq!(derived.canonical_bytes().unwrap(), running_state.canonical_bytes().unwrap());
}

#[test]
fn projection_can_be_deleted_and_rebuilt_from_events_byte_equivalently() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("projection.db");
    let mut store = RunStore::open(&path).expect("open store");
    let created = genesis();
    store
        .append(&ExpectedRunHead::Genesis, &created)
        .expect("append genesis");
    let started = successor(&created, fixture_event("run_started"));
    let state = store
        .append(&expected(&created), &started)
        .expect("append started");
    let canonical = state.canonical_bytes().expect("canonical state");

    let raw = Connection::open(&path).expect("raw projection connection");
    raw.execute(
        "DELETE FROM run_heads WHERE run_id = ?1",
        params![created.run_id().to_string()],
    )
    .expect("delete projection only");
    let count: i64 = raw
        .query_row(
            "SELECT count(*) FROM run_heads WHERE run_id = ?1",
            params![created.run_id().to_string()],
            |row| row.get(0),
        )
        .expect("projection count");
    assert_eq!(count, 0);

    let authoritative = store
        .load_state(created.run_id())
        .expect("derive without projection")
        .expect("state exists");
    assert_eq!(authoritative.canonical_bytes().unwrap(), canonical);
    let rebuilt = store
        .rebuild_projection(created.run_id())
        .expect("rebuild projection")
        .expect("rebuilt state");
    assert_eq!(rebuilt.canonical_bytes().unwrap(), canonical);

    let projected: Vec<u8> = raw
        .query_row(
            "SELECT state_json FROM run_heads WHERE run_id = ?1",
            params![created.run_id().to_string()],
            |row| row.get(0),
        )
        .expect("rebuilt projection bytes");
    assert_eq!(projected, canonical);
}

#[test]
fn authoritative_events_reject_ordinary_update_and_delete() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("append-only.db");
    let mut store = RunStore::open(&path).expect("open store");
    let created = genesis();
    store
        .append(&ExpectedRunHead::Genesis, &created)
        .expect("append genesis");
    drop(store);

    let raw = Connection::open(&path).expect("raw connection");
    assert!(
        raw.execute(
            "UPDATE run_events SET event_digest = event_digest WHERE run_id = ?1",
            params![created.run_id().to_string()],
        )
        .is_err()
    );
    assert!(
        raw.execute(
            "DELETE FROM run_events WHERE run_id = ?1",
            params![created.run_id().to_string()],
        )
        .is_err()
    );
    let count: i64 = raw
        .query_row("SELECT count(*) FROM run_events", [], |row| row.get(0))
        .expect("event count");
    assert_eq!(count, 1);
}

#[test]
fn malformed_or_chain_corrupt_authoritative_rows_fail_closed() {
    let directory = tempdir().expect("tempdir");
    let malformed_path = directory.path().join("malformed.db");
    let mut store = RunStore::open(&malformed_path).expect("open malformed store");
    let created = genesis();
    store
        .append(&ExpectedRunHead::Genesis, &created)
        .expect("append genesis");
    drop(store);
    let raw = Connection::open(&malformed_path).expect("raw malformed connection");
    raw.execute(
        "INSERT INTO run_events (run_id, sequence, event_digest, previous_digest, event_json) \
         VALUES (?1, 2, ?2, ?3, ?4)",
        params![
            created.run_id().to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000",
            created.event_digest().hex(),
            b"{}".as_slice()
        ],
    )
    .expect("inject malformed row");
    drop(raw);
    let store = RunStore::open(&malformed_path).expect("reopen malformed store");
    let error = store
        .load_history(created.run_id())
        .expect_err("malformed row must fail");
    assert_eq!(error.code(), RunErrorCode::SerializationFailed);

    let chain_path = directory.path().join("chain.db");
    let mut store = RunStore::open(&chain_path).expect("open chain store");
    let created = genesis();
    store
        .append(&ExpectedRunHead::Genesis, &created)
        .expect("append genesis");
    drop(store);
    let wrong_previous = LedgerDigest::new_sha256(
        "1111111111111111111111111111111111111111111111111111111111111111",
    )
    .expect("wrong digest");
    let corrupt = RunEventEnvelope::new(
        created.run_id(),
        created.sequence().checked_next().expect("next sequence"),
        EpochMillis::new(created.recorded_at().get() + 1).expect("timestamp"),
        Some(wrong_previous),
        fixture_event("run_started"),
    )
    .expect("locally valid but cross-row corrupt envelope");
    let bytes = to_jcs_vec(&corrupt).expect("canonical corrupt envelope");
    let raw = Connection::open(&chain_path).expect("raw chain connection");
    raw.execute(
        "INSERT INTO run_events (run_id, sequence, event_digest, previous_digest, event_json) \
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            created.run_id().to_string(),
            i64::try_from(corrupt.sequence().get()).unwrap(),
            corrupt.event_digest().hex(),
            corrupt.previous_digest().unwrap().hex(),
            bytes
        ],
    )
    .expect("inject chain-corrupt row");
    drop(raw);
    let store = RunStore::open(&chain_path).expect("reopen chain store");
    let error = store
        .load_history(created.run_id())
        .expect_err("chain corruption must fail");
    assert_eq!(error.code(), RunErrorCode::LedgerChainInvalid);
}

#[test]
fn synthetic_content_addressed_blobs_validate_size_digest_and_storage_budget() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("blobs.db");
    let mut store = RunStore::open(&path).expect("open store");
    let bytes = b"synthetic-ecr-002-artifact";
    let digest = sha256_digest(bytes);
    let size = BudgetAmount::new(u64::try_from(bytes.len()).unwrap()).unwrap();

    store
        .put_blob(&digest, size, bytes, Some(size))
        .expect("put exact synthetic blob");
    assert_eq!(store.get_blob(&digest).expect("get blob"), Some(bytes.to_vec()));
    store
        .put_blob(&digest, size, bytes, Some(size))
        .expect("idempotent put");

    let too_small = BudgetAmount::new(size.get() - 1).unwrap();
    let error = store
        .put_blob(&digest, size, bytes, Some(too_small))
        .expect_err("storage preflight must fail");
    assert_eq!(error.code(), RunErrorCode::BudgetPreflightExceeded);

    let wrong_size = BudgetAmount::new(size.get() - 1).unwrap();
    let error = store
        .put_blob(&digest, wrong_size, bytes, None)
        .expect_err("declared size mismatch must fail");
    assert_eq!(error.code(), RunErrorCode::StorageError);

    let wrong_digest = sha256_digest(b"other synthetic bytes");
    let error = store
        .put_blob(&wrong_digest, size, bytes, None)
        .expect_err("digest mismatch must fail");
    assert_eq!(error.code(), RunErrorCode::StorageError);
}

#[test]
fn store_returns_none_for_unknown_run_and_blob() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("empty.db");
    let store = RunStore::open(&path).expect("open store");
    let run_id = RunId::parse_str("00000000-0000-0000-0000-000000009999").unwrap();
    assert!(store.load_history(run_id).unwrap().is_empty());
    assert!(store.load_state(run_id).unwrap().is_none());
    let digest = sha256_digest(b"not stored");
    assert!(store.get_blob(&digest).unwrap().is_none());
}
