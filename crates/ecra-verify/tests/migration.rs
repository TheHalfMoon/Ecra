use ecra_verify::{VerificationStore, VerifyErrorCode};
use rusqlite::Connection;
use tempfile::tempdir;

#[test]
fn unsupported_store_version_preserves_existing_local_data() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("unsupported.sqlite");
    let connection = Connection::open(&path).expect("create fixture");
    connection
        .execute_batch(
            "CREATE TABLE marker (value TEXT NOT NULL);
             INSERT INTO marker (value) VALUES ('preserve');
             PRAGMA user_version = 2;",
        )
        .expect("prepare fixture");
    drop(connection);

    let error = VerificationStore::open(&path)
        .err()
        .expect("unsupported schema must fail");
    assert_eq!(error.code(), VerifyErrorCode::UnsupportedVersion);

    let connection = Connection::open(&path).expect("inspect fixture");
    let version: i64 = connection
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .expect("user version");
    let value: String = connection
        .query_row("SELECT value FROM marker", [], |row| row.get(0))
        .expect("marker value");
    let journal_count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='verification_journal'",
            [],
            |row| row.get(0),
        )
        .expect("journal table count");
    assert_eq!(version, 2);
    assert_eq!(value, "preserve");
    assert_eq!(journal_count, 0);
}

#[test]
fn initialization_failure_rolls_back_version_and_new_tables() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("rollback.sqlite");
    let connection = Connection::open(&path).expect("create fixture");
    connection
        .execute_batch(
            "CREATE TABLE verification_meta (wrong TEXT NOT NULL);
             INSERT INTO verification_meta (wrong) VALUES ('existing');
             PRAGMA user_version = 0;",
        )
        .expect("prepare conflicting fixture");
    drop(connection);

    let error = VerificationStore::open(&path)
        .err()
        .expect("initialization must fail");
    assert_eq!(error.code(), VerifyErrorCode::StoreCorrupt);

    let connection = Connection::open(&path).expect("inspect rollback");
    let version: i64 = connection
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .expect("user version");
    let original: String = connection
        .query_row("SELECT wrong FROM verification_meta", [], |row| row.get(0))
        .expect("original row");
    let journal_count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='verification_journal'",
            [],
            |row| row.get(0),
        )
        .expect("journal table count");
    assert_eq!(version, 0);
    assert_eq!(original, "existing");
    assert_eq!(journal_count, 0);
}

#[test]
fn schema_marker_mismatch_fails_closed() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("marker-mismatch.sqlite");
    let connection = Connection::open(&path).expect("create fixture");
    connection
        .execute_batch(
            "CREATE TABLE verification_meta (
                 singleton INTEGER PRIMARY KEY CHECK(singleton = 1),
                 schema_version INTEGER NOT NULL
             );
             INSERT INTO verification_meta (singleton, schema_version) VALUES (1, 0);
             PRAGMA user_version = 1;",
        )
        .expect("prepare marker mismatch");
    drop(connection);

    let error = VerificationStore::open(&path)
        .err()
        .expect("marker mismatch must fail");
    assert_eq!(error.code(), VerifyErrorCode::StoreCorrupt);
}
