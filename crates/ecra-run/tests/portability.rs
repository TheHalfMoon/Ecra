use ecra_core::EpochMillis;
use ecra_run::{RunEvent, RunEventEnvelope, RunReducer};
use sha2::{Digest, Sha256};

fn event(kind: &str) -> RunEvent {
    let events: Vec<RunEvent> = serde_json::from_slice(include_bytes!(
        "../../../contracts/ecra-run-v1/valid/all-event-kinds.v1.json"
    ))
    .expect("event fixtures");
    events
        .into_iter()
        .find(|candidate| candidate.kind() == kind)
        .unwrap_or_else(|| panic!("missing fixture for {kind}"))
}

fn replay_history() -> Vec<RunEventEnvelope> {
    let mut history = vec![
        RunEventEnvelope::from_json_slice(include_bytes!(
            "../../../contracts/ecra-run-v1/valid/run-created-envelope.v1.json"
        ))
        .expect("genesis"),
    ];
    for kind in [
        "run_started",
        "resource_usage_recorded",
        "attempt_prepared",
        "receipt_recorded",
        "intervention_recorded",
        "run_suspended",
    ] {
        let previous = history.last().expect("history");
        history.push(
            RunEventEnvelope::new(
                previous.run_id(),
                previous.sequence().checked_next().expect("next sequence"),
                EpochMillis::new(previous.recorded_at().get() + 1).expect("timestamp"),
                Some(previous.event_digest().clone()),
                event(kind),
            )
            .expect("successor"),
        );
    }
    history
}

#[test]
fn identical_accepted_history_reduces_1000_times_to_identical_canonical_state() {
    let history = replay_history();
    let first = RunReducer::reduce(&history).expect("first replay");
    let canonical = first.canonical_bytes().expect("canonical state");
    let digest = Sha256::digest(&canonical);

    for _ in 0..1_000 {
        let replay = RunReducer::reduce(&history).expect("deterministic replay");
        let replay_bytes = replay.canonical_bytes().expect("canonical replay");
        assert_eq!(replay_bytes, canonical);
        assert_eq!(Sha256::digest(&replay_bytes), digest);
    }
}
