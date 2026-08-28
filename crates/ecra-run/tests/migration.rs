use ecra_run::{ECR_RUN_SCHEMA_VERSION, RunErrorCode, RunStore};
use rusqlite::Connection;
use tempfile::tempdir;

#[test]
fn empty_store_migrates_transactionally_to_schema_v1() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("migrate-v0.db");
    let connection = Connection::open(&path).expect("open raw fixture");
    connection
        .execute_batch(include_str!(
            "../../../contracts/ecra-run-v1/migrations/v0-empty.sql"
        ))
        .expect("apply v0 fixture");
    drop(connection);

    let store = RunStore::open(&path).expect("migrate to v1");
    let configuration = store.sqlite_configuration().expect("configuration");
    assert_eq!(configuration.journal_mode(), "wal");
    assert_eq!(configuration.synchronous(), 2);
    assert!(configuration.foreign_keys());
    assert!(!configuration.trusted_schema());
    drop(store);

    let connection = Connection::open(&path).expect("inspect migrated store");
    let version: i64 = connection
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .expect("user_version");
    assert_eq!(version, ECR_RUN_SCHEMA_VERSION);
    let objects: i64 = connection
        .query_row(
            "SELECT count(*) FROM sqlite_schema WHERE \
             (type = 'table' AND name IN ('run_events','run_heads','artifact_blobs')) OR \
             (type = 'trigger' AND name IN ('run_events_no_update','run_events_no_delete'))",
            [],
            |row| row.get(0),
        )
        .expect("schema objects");
    assert_eq!(objects, 5);
}

#[test]
fn newer_store_version_is_rejected_without_mutation() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("newer.db");
    let connection = Connection::open(&path).expect("open raw store");
    connection
        .pragma_update(None, "user_version", 2_i64)
        .expect("set newer version");
    drop(connection);

    let error = match RunStore::open(&path) {
        Ok(_) => panic!("newer store must fail closed"),
        Err(error) => error,
    };
    assert_eq!(error.code(), RunErrorCode::UnsupportedStoreVersion);

    let connection = Connection::open(&path).expect("reopen raw store");
    let version: i64 = connection
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .expect("user_version");
    assert_eq!(version, 2);
}

#[test]
fn failed_v0_migration_rolls_back_user_version_and_new_objects() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("broken-v0.db");
    let connection = Connection::open(&path).expect("open raw store");
    connection
        .execute_batch("CREATE TABLE run_events(x TEXT); PRAGMA user_version = 0;")
        .expect("create conflicting predecessor");
    drop(connection);

    let error = match RunStore::open(&path) {
        Ok(_) => panic!("conflicting migration must fail"),
        Err(error) => error,
    };
    assert_eq!(error.code(), RunErrorCode::MigrationFailed);

    let connection = Connection::open(&path).expect("inspect rollback");
    let version: i64 = connection
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .expect("user_version");
    assert_eq!(version, 0);
    let heads: i64 = connection
        .query_row(
            "SELECT count(*) FROM sqlite_schema WHERE type='table' AND name='run_heads'",
            [],
            |row| row.get(0),
        )
        .expect("run_heads existence");
    assert_eq!(heads, 0);
}
