#[test]
fn archive_module_has_no_raw_sqlite_export_dependency() {
    let source = include_str!("../src/archive.rs");
    for forbidden in ["rusqlite", "Connection", "-wal", "-shm", "SQLite format 3"] {
        assert!(
            !source.contains(forbidden),
            "archive production source must remain logical-content-only: {forbidden}"
        );
    }
}
