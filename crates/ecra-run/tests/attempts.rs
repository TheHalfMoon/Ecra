use ecra_core::{ActionAttemptRef, ActionReceipt, EpochMillis};
use ecra_run::{RunErrorCode, RunEvent, RunEventEnvelope, RunPhase, RunReducer, SuspensionReason};

fn genesis() -> RunEventEnvelope {
    RunEventEnvelope::from_json_slice(include_bytes!(
        "../../../contracts/ecra-run-v1/valid/run-created-envelope.v1.json"
    ))
    .expect("genesis fixture")
}

fn fixtures() -> Vec<RunEvent> {
    serde_json::from_slice(include_bytes!(
        "../../../contracts/ecra-run-v1/valid/all-event-kinds.v1.json"
    ))
    .expect("event fixtures")
}

fn event(kind: &str) -> RunEvent {
    fixtures()
        .into_iter()
        .find(|candidate| candidate.kind() == kind)
        .unwrap_or_else(|| panic!("missing fixture for {kind}"))
}

fn fixture_attempt() -> ActionAttemptRef {
    match event("attempt_prepared") {
        RunEvent::AttemptPrepared { attempt } => attempt,
        _ => unreachable!(),
    }
}

fn fixture_receipt() -> ActionReceipt {
    match event("receipt_recorded") {
        RunEvent::ReceiptRecorded { receipt } => receipt,
        _ => unreachable!(),
    }
}

fn push(history: &mut Vec<RunEventEnvelope>, next: RunEvent) {
    let previous = history.last().expect("history has genesis");
    history.push(
        RunEventEnvelope::new(
            previous.run_id(),
            previous.sequence().checked_next().expect("next sequence"),
            EpochMillis::new(previous.recorded_at().get() + 1).expect("timestamp"),
            Some(previous.event_digest().clone()),
            next,
        )
        .expect("valid envelope"),
    );
}

fn running() -> Vec<RunEventEnvelope> {
    let mut history = vec![genesis()];
    push(&mut history, event("run_started"));
    history
}

#[test]
fn exact_attempt_can_be_prepared_once_and_receipt_binds_it() {
    let mut history = running();
    push(&mut history, event("attempt_prepared"));
    push(&mut history, event("receipt_recorded"));

    let state = RunReducer::reduce(&history).expect("attempt + receipt reduces");
    let attempt = fixture_attempt();
    let prepared = state
        .prepared_attempts()
        .get(&attempt.id())
        .expect("prepared attempt projection");
    assert_eq!(prepared.attempt(), &attempt);
    let receipt = fixture_receipt();
    assert_eq!(prepared.receipt(), Some(&receipt));
    assert!(!prepared.unresolved());
    assert!(state.unresolved_attempts().is_empty());
}

#[test]
fn duplicate_and_conflicting_attempt_identity_fail_with_typed_codes() {
    let mut history = running();
    push(&mut history, event("attempt_prepared"));
    let state = RunReducer::reduce(&history).expect("prepared state");

    let duplicate = RunEvent::AttemptPrepared {
        attempt: fixture_attempt(),
    };
    push(&mut history, duplicate);
    let error = RunReducer::apply(&state, history.last().expect("duplicate"))
        .expect_err("duplicate attempt must fail");
    assert_eq!(error.code(), RunErrorCode::DuplicateAttempt);

    let conflicting: ActionAttemptRef = serde_json::from_str(
        r#"{"id":"00000000-0000-0000-0000-000000000302","action":{"id":"00000000-0000-0000-0000-000000000399","digest":{"algorithm":"sha256","hex":"2222222222222222222222222222222222222222222222222222222222222222"}}}"#,
    )
    .expect("conflicting attempt fixture");
    let mut conflict_history = running();
    push(&mut conflict_history, event("attempt_prepared"));
    let conflict_state = RunReducer::reduce(&conflict_history).expect("prepared state");
    push(
        &mut conflict_history,
        RunEvent::AttemptPrepared {
            attempt: conflicting,
        },
    );
    let error = RunReducer::apply(&conflict_state, conflict_history.last().expect("conflict"))
        .expect_err("conflicting binding must fail");
    assert_eq!(error.code(), RunErrorCode::AttemptBindingMismatch);
}

#[test]
fn receipt_for_unprepared_or_cross_bound_attempt_fails_closed() {
    let mut history = running();
    let state = RunReducer::reduce(&history).expect("running state");
    push(&mut history, event("receipt_recorded"));
    let error = RunReducer::apply(&state, history.last().expect("receipt"))
        .expect_err("receipt without preparation must fail");
    assert_eq!(error.code(), RunErrorCode::ReceiptBindingMismatch);

    let mut prepared_history = running();
    push(&mut prepared_history, event("attempt_prepared"));
    let prepared_state = RunReducer::reduce(&prepared_history).expect("prepared state");
    let mut receipt_json = serde_json::to_value(fixture_receipt()).expect("receipt json");
    receipt_json["attempt"]["action"]["id"] = "00000000-0000-0000-0000-000000000399".into();
    let wrong_receipt: ActionReceipt =
        serde_json::from_value(receipt_json).expect("cross-bound receipt fixture");
    push(
        &mut prepared_history,
        RunEvent::ReceiptRecorded {
            receipt: wrong_receipt,
        },
    );
    let error = RunReducer::apply(
        &prepared_state,
        prepared_history.last().expect("wrong receipt"),
    )
    .expect_err("cross-bound receipt must fail");
    assert_eq!(error.code(), RunErrorCode::ReceiptBindingMismatch);
}

#[test]
fn distinct_attempt_ids_for_one_action_remain_distinct_and_ordered() {
    let first = fixture_attempt();
    let second: ActionAttemptRef = serde_json::from_str(
        r#"{"id":"00000000-0000-0000-0000-000000000304","action":{"id":"00000000-0000-0000-0000-000000000301","digest":{"algorithm":"sha256","hex":"1111111111111111111111111111111111111111111111111111111111111111"}}}"#,
    )
    .expect("second attempt fixture");

    let mut history = running();
    push(
        &mut history,
        RunEvent::AttemptPrepared {
            attempt: second.clone(),
        },
    );
    push(
        &mut history,
        RunEvent::AttemptPrepared {
            attempt: first.clone(),
        },
    );

    let state = RunReducer::reduce(&history).expect("distinct attempts reduce");
    assert_eq!(state.prepared_attempts().len(), 2);
    assert!(state.prepared_attempts().contains_key(&first.id()));
    assert!(state.prepared_attempts().contains_key(&second.id()));
    let ids: Vec<_> = state.prepared_attempts().keys().copied().collect();
    assert!(ids.windows(2).all(|pair| pair[0] < pair[1]));
}

#[test]
fn recovery_boundary_marks_missing_receipt_unknown_and_blocks_completion() {
    let mut history = running();
    push(&mut history, event("attempt_prepared"));
    push(&mut history, event("recovery_boundary"));
    let state = RunReducer::reduce(&history).expect("recovery state");

    let attempt = fixture_attempt();
    let prepared = state
        .prepared_attempts()
        .get(&attempt.id())
        .expect("attempt remains projected");
    assert!(prepared.receipt().is_none());
    assert!(prepared.unresolved());
    assert!(state.unresolved_attempts().contains(&attempt.id()));
    assert_eq!(state.phase(), RunPhase::Suspended);
    assert!(matches!(
        state.suspension(),
        Some(SuspensionReason::ReconciliationRequired { attempt: found }) if found == &attempt
    ));

    let mut completion = history;
    push(&mut completion, event("execution_completed"));
    let error = RunReducer::apply(&state, completion.last().expect("completion"))
        .expect_err("suspended unresolved run cannot complete");
    assert_eq!(error.code(), RunErrorCode::InvalidStateTransition);
}

#[test]
fn reconciliation_request_requires_exact_unresolved_attempt() {
    let mut history = running();
    push(&mut history, event("attempt_prepared"));
    let prepared_state = RunReducer::reduce(&history).expect("prepared state");
    push(&mut history, event("reconciliation_requested"));
    let error = RunReducer::apply(&prepared_state, history.last().expect("request"))
        .expect_err("prepared but not unresolved attempt cannot request reconciliation");
    assert_eq!(error.code(), RunErrorCode::UnresolvedAttempt);

    let mut recovered = running();
    push(&mut recovered, event("attempt_prepared"));
    push(&mut recovered, event("recovery_boundary"));
    let recovered_state = RunReducer::reduce(&recovered).expect("recovered state");
    push(&mut recovered, event("reconciliation_requested"));
    let next = RunReducer::apply(&recovered_state, recovered.last().expect("request"))
        .expect("exact unresolved request accepted");
    assert_eq!(next.phase(), RunPhase::Suspended);
    assert_eq!(
        next.unresolved_attempts(),
        recovered_state.unresolved_attempts()
    );
}
