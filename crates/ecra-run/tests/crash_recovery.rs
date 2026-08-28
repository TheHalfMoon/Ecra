use std::path::Path;
use std::process::Command;

use ecra_core::EpochMillis;
use ecra_run::{
    ExpectedRunHead, RecoveryReason, RunEvent, RunEventEnvelope, RunPhase, RunStore,
    SuspensionReason,
};
use tempfile::tempdir;

const CHILD_FLAG: &str = "ECRA_RUN_CRASH_WRITER_CHILD";
const CHILD_PATH: &str = "ECRA_RUN_CRASH_DB_PATH";
const ATTEMPT_CHILD_FLAG: &str = "ECRA_RUN_ATTEMPT_CRASH_CHILD";
const ATTEMPT_MODE: &str = "ECRA_RUN_ATTEMPT_CRASH_MODE";
const EFFECT_MARKER: &str = "ECRA_RUN_EFFECT_MARKER";

fn genesis() -> RunEventEnvelope {
    RunEventEnvelope::from_json_slice(include_bytes!(
        "../../../contracts/ecra-run-v1/valid/run-created-envelope.v1.json"
    ))
    .expect("genesis fixture")
}

fn event(kind: &str) -> RunEvent {
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
    .expect("successor")
}

fn expected_for(envelope: &RunEventEnvelope) -> ExpectedRunHead {
    ExpectedRunHead::At {
        sequence: envelope.sequence(),
        digest: envelope.event_digest().clone(),
    }
}

fn expected_state(state: &ecra_run::RunState) -> ExpectedRunHead {
    ExpectedRunHead::At {
        sequence: state.last_sequence(),
        digest: state.last_digest().clone(),
    }
}

fn initialize_running(path: &Path) -> RunEventEnvelope {
    let mut store = RunStore::open(path).expect("open initialization store");
    let created = genesis();
    store
        .append(&ExpectedRunHead::Genesis, &created)
        .expect("append genesis");
    let started = successor(&created, event("run_started"));
    store
        .append(&expected_for(&created), &started)
        .expect("append started");
    started
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
fn attempt_crash_child() {
    if std::env::var_os(ATTEMPT_CHILD_FLAG).is_none() {
        return;
    }
    let path = std::env::var_os(CHILD_PATH).expect("attempt child database path");
    let mode = std::env::var(ATTEMPT_MODE).expect("attempt crash mode");
    let marker = std::env::var_os(EFFECT_MARKER).expect("effect marker path");
    let mut store = RunStore::open(&path).expect("attempt child open store");
    let state = store
        .load_state(genesis().run_id())
        .expect("load running state")
        .expect("running state exists");
    if mode == "A" {
        std::process::abort();
    }

    let attempt = match event("attempt_prepared") {
        RunEvent::AttemptPrepared { attempt } => attempt,
        _ => unreachable!(),
    };
    let guard = store
        .prepare_attempt(
            state.run_id(),
            &expected_state(&state),
            attempt,
            EpochMillis::new(10_000).unwrap(),
        )
        .expect("durable attempt preparation");
    if mode == "B" {
        std::process::abort();
    }
    if mode == "C" {
        std::fs::write(marker, b"synthetic-external-effect").expect("write effect marker");
        std::process::abort();
    }
    if mode == "D" {
        let receipt = match event("receipt_recorded") {
            RunEvent::ReceiptRecorded { receipt } => receipt,
            _ => unreachable!(),
        };
        let expected = ExpectedRunHead::At {
            sequence: guard.committed_sequence(),
            digest: guard.committed_digest().clone(),
        };
        store
            .record_receipt(
                state.run_id(),
                &expected,
                receipt,
                EpochMillis::new(10_001).unwrap(),
            )
            .expect("durable receipt commit");
        std::process::abort();
    }
    panic!("unsupported crash matrix mode {mode}");
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

#[test]
fn crash_matrix_a_through_d_preserves_attempt_truth_without_fabrication_or_retry() {
    let executable = std::env::current_exe().expect("current test binary");
    let attempt = match event("attempt_prepared") {
        RunEvent::AttemptPrepared { attempt } => attempt,
        _ => unreachable!(),
    };
    let receipt = match event("receipt_recorded") {
        RunEvent::ReceiptRecorded { receipt } => receipt,
        _ => unreachable!(),
    };

    for mode in ["A", "B", "C", "D"] {
        let directory = tempdir().expect("tempdir");
        let path = directory.path().join(format!("attempt-{mode}.db"));
        let marker = directory.path().join(format!("effect-{mode}.marker"));
        initialize_running(&path);
        let status = Command::new(&executable)
            .arg("--exact")
            .arg("attempt_crash_child")
            .arg("--nocapture")
            .arg("--test-threads=1")
            .env(ATTEMPT_CHILD_FLAG, "1")
            .env(ATTEMPT_MODE, mode)
            .env(CHILD_PATH, &path)
            .env(EFFECT_MARKER, &marker)
            .status()
            .expect("spawn attempt crash child");
        assert!(!status.success(), "mode {mode} child must abort");

        let mut store = RunStore::open(&path).expect("reopen attempt store");
        let state = store
            .load_state(genesis().run_id())
            .expect("load post-crash state")
            .expect("post-crash state exists");
        match mode {
            "A" => {
                assert!(state.prepared_attempts().is_empty());
                assert_eq!(state.phase(), RunPhase::Running);
            }
            "B" | "C" => {
                let prepared = state
                    .prepared_attempts()
                    .get(&attempt.id())
                    .expect("prepared attempt survives crash");
                assert!(prepared.receipt().is_none());
                assert!(!prepared.unresolved());
                assert_eq!(marker.exists(), mode == "C");

                let recovery = store
                    .recover(
                        state.run_id(),
                        &expected_state(&state),
                        RecoveryReason::ProcessRestart,
                        EpochMillis::new(20_000).unwrap(),
                    )
                    .expect("append explicit recovery boundary");
                assert_eq!(
                    recovery.unreceipted_attempts(),
                    std::slice::from_ref(&attempt)
                );
                assert_eq!(recovery.state().phase(), RunPhase::Suspended);
                assert!(
                    recovery
                        .state()
                        .unresolved_attempts()
                        .contains(&attempt.id())
                );
                assert!(matches!(
                    recovery.state().suspension(),
                    Some(SuspensionReason::ReconciliationRequired { attempt: found }) if found == &attempt
                ));
                assert!(
                    recovery
                        .state()
                        .prepared_attempts()
                        .get(&attempt.id())
                        .unwrap()
                        .receipt()
                        .is_none()
                );
                let history = store
                    .load_history(state.run_id())
                    .expect("recovery history");
                assert_eq!(
                    history
                        .iter()
                        .filter(|envelope| matches!(
                            envelope.event(),
                            RunEvent::AttemptPrepared { .. }
                        ))
                        .count(),
                    1,
                    "recovery must not retry or create another attempt"
                );
                assert!(
                    !history.iter().any(|envelope| matches!(
                        envelope.event(),
                        RunEvent::ReceiptRecorded { .. }
                    )),
                    "recovery must not fabricate a receipt"
                );
            }
            "D" => {
                let prepared = state
                    .prepared_attempts()
                    .get(&attempt.id())
                    .expect("received attempt survives crash");
                assert_eq!(prepared.receipt(), Some(&receipt));
                assert!(!prepared.unresolved());
                assert_eq!(state.phase(), RunPhase::Running);
            }
            _ => unreachable!(),
        }
    }
}
