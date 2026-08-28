use ecra_core::{EpochMillis, PrincipalId};
use ecra_identity::{EnrollmentId, EnrollmentRecord, TrustRootId};

fn block<'a>(source: &'a str, start: &str, end: &str) -> &'a str {
    let start_index = source.find(start).expect("start marker");
    let tail = &source[start_index..];
    let end_index = tail.find(end).expect("end marker");
    &tail[..end_index]
}

#[test]
fn ordinary_enrollment_metadata_never_becomes_bootstrap_authority() {
    let record = EnrollmentRecord::new(
        EnrollmentId::parse_str("00000000-0000-0000-0000-000000000030").unwrap(),
        PrincipalId::parse_str("00000000-0000-0000-0000-000000000004").unwrap(),
        TrustRootId::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
        EpochMillis::new(1_000).unwrap(),
    );
    let value = serde_json::to_value(record).unwrap();
    let object = value.as_object().unwrap();
    assert!(object.contains_key("principal_id"));
    assert!(object.contains_key("trust_root_id"));
    assert!(!object.contains_key("secret"));
    assert!(!object.contains_key("private_key"));
    assert!(!object.contains_key("backend_secret_ref"));
}

#[test]
fn bootstrap_transaction_orders_fail_closed_crash_boundaries() {
    let source = include_str!("../src/bootstrap.rs");
    let transaction = block(
        source,
        "pub(crate) fn bootstrap_or_reopen_local_principal(",
        "fn enrolled_handle_from_authenticated(",
    );

    let store_exists = transaction.find("ProtectedTrustStateStore::store_exists").unwrap();
    let open_existing = transaction.find("ProtectedTrustStateStore::open_existing").unwrap();
    let marker_exists = transaction
        .find("ProtectedTrustStateStore::bootstrap_marker_exists")
        .unwrap();
    let incomplete = transaction.find("return Err(incomplete_bootstrap_error())").unwrap();
    let marker_write = transaction
        .find("ProtectedTrustStateStore::write_bootstrap_marker")
        .unwrap();
    let randomness = transaction.find("let principal_id =").unwrap();

    assert!(store_exists < open_existing);
    assert!(open_existing < marker_exists);
    assert!(marker_exists < incomplete);
    assert!(incomplete < marker_write);
    assert!(marker_write < randomness);

    let creation = &transaction[marker_write..];
    let first_protect = creation.find("backend.protect_secret").unwrap();
    let publish = creation.find("store.publish").unwrap();
    let reopen = creation.find("store.open_authenticated").unwrap();
    let marker_clear = creation
        .find("ProtectedTrustStateStore::clear_bootstrap_marker")
        .unwrap();
    assert!(first_protect < publish);
    assert!(publish < reopen);
    assert!(reopen < marker_clear);
}

#[test]
fn protected_state_atomic_replace_flushes_before_and_after_rename() {
    let source = include_str!("../src/store.rs");
    let atomic = block(source, "fn atomic_replace_path(", "fn temp_path_for(");
    let write = atomic.find("file.write_all(bytes)").unwrap();
    let file_sync = atomic.find("file.sync_all()").unwrap();
    let rename = atomic.find("fs::rename(&temp_path, path)").unwrap();
    let parent_sync = atomic.find("sync_parent_directory(parent)").unwrap();
    assert!(write < file_sync);
    assert!(file_sync < rename);
    assert!(rename < parent_sync);
}

#[test]
fn bootstrap_transaction_stays_non_public_until_native_backend_integration() {
    let source = include_str!("../src/bootstrap.rs");
    assert!(source.contains("pub(crate) fn bootstrap_or_reopen_local_principal("));
    assert!(!source.contains("pub fn bootstrap_or_reopen_local_principal("));
    assert!(source.contains("complete_bootstrap_reopens_same_principal_without_reminting"));
    assert!(source.contains("partial_marker_blocks_silent_identity_remint"));
    assert!(source.contains("unavailable_backend_fails_before_creating_partial_marker"));
}
