use std::process::Command;

use ecra_run::{ExpectedRunHead, RunEventEnvelope, RunStore};
use tempfile::tempdir;

const CHILD_FLAG: &str = "ECRA_RUN_CRASH_WRITER_CHILD";
const CHILD_PATH: &str = "ECRA_RUN_CRASH_DB_PATH";

fn genesis() -> RunEventEnvelope {
    RunEventEnvelope::from_json_slice(include_bytes!(
        "../../../contracts/ecra-run-v1/valid/run-created-envelope.v1.json"
    ))
    .expect("genesis fixture")
}

#[test]
fn crash_writer_child() {
    if std::env::var_os(CHILD_FLAG).is_none() {
        return;
    }
    let path = std::env::var_os(CHILD_PATH).expect("child database path");
    let mut store = RunStore::open(&path).expect("child open store");
    store
        .append(&ExpectedRunHead::Genesis, &genesis())
        .expect("child durable append");
    std::process::abort();
}

#[test]
fn committed_event_survives_process_abort_with_wal_and_full() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("crash.db");
    let executable = std::env::current_exe().expect("current test binary");
    let status = Command::new(executable)
        .arg("--exact")
        .arg("crash_writer_child")
        .arg("--nocapture")
        .arg("--test-threads=1")
        .env(CHILD_FLAG, "1")
        .env(CHILD_PATH, &path)
        .status()
        .expect("spawn crash writer child");
    assert!(!status.success(), "child must terminate by abort");

    let store = RunStore::open(&path).expect("reopen after process abort");
    let configuration = store.sqlite_configuration().expect("configuration");
    assert_eq!(configuration.journal_mode(), "wal");
    assert_eq!(configuration.synchronous(), 2);
    let created = genesis();
    let history = store
        .load_history(created.run_id())
        .expect("recover committed history");
    assert_eq!(history, vec![created]);
}
