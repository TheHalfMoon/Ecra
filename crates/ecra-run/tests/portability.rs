use ecra_core::EpochMillis;
use ecra_run::{RunEvent, RunEventEnvelope, RunReducer, export_ecra};
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

fn replay_history_from_genesis(genesis: RunEventEnvelope) -> Vec<RunEventEnvelope> {
    let mut history = vec![genesis];
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

fn replay_history() -> Vec<RunEventEnvelope> {
    replay_history_from_genesis(
        RunEventEnvelope::from_json_slice(include_bytes!(
            "../../../contracts/ecra-run-v1/valid/run-created-envelope.v1.json"
        ))
        .expect("genesis"),
    )
}

fn formatted_genesis_variants() -> Vec<RunEventEnvelope> {
    let lf = include_bytes!("../../../contracts/ecra-run-v1/valid/run-created-envelope.v1.json");
    let crlf = std::str::from_utf8(lf)
        .expect("UTF-8 fixture")
        .replace('\n', "\r\n")
        .into_bytes();
    let value: serde_json::Value = serde_json::from_slice(lf).expect("fixture JSON");
    let compact = serde_json::to_vec(&value).expect("compact JSON");

    vec![
        RunEventEnvelope::from_json_slice(lf).expect("LF envelope"),
        RunEventEnvelope::from_json_slice(&crlf).expect("CRLF envelope"),
        RunEventEnvelope::from_json_slice(&compact).expect("compact envelope"),
    ]
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

#[test]
fn json_formatting_variants_preserve_reducer_and_archive_output() {
    let variants = formatted_genesis_variants();
    let first_envelope = variants.first().expect("first variant");
    for variant in &variants[1..] {
        assert_eq!(variant, first_envelope);
    }

    let histories: Vec<Vec<RunEventEnvelope>> = variants
        .into_iter()
        .map(replay_history_from_genesis)
        .collect();
    let first_history = histories.first().expect("first history");
    let first_state = RunReducer::reduce(first_history)
        .expect("first state")
        .canonical_bytes()
        .expect("first canonical state");
    let first_archive = export_ecra(first_history, &[]).expect("first archive");

    for history in &histories[1..] {
        let state = RunReducer::reduce(history)
            .expect("variant state")
            .canonical_bytes()
            .expect("variant canonical state");
        assert_eq!(state, first_state);
        assert_eq!(export_ecra(history, &[]).expect("variant archive"), first_archive);
    }
}
