use ecra_core::{
    ActionAttemptId, ActionAttemptRef, ActionIntent, ActionOutcome, ActionReceipt, ActorId,
    EpochMillis, ReceiptId,
};
use ecra_run::{
    RunErrorCode, RunEvent, RunEventEnvelope, RunPhase, RunReducer, SuspensionReason,
    ensure_retry_allowed,
};

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

fn intent_with_semantics(
    retry: &str,
    idempotency: serde_json::Value,
    effect: serde_json::Value,
) -> ActionIntent {
    let mut value: serde_json::Value = serde_json::from_str(include_str!(
        "../../../contracts/ecra-domain-v1/valid/action-digest-golden.json"
    ))
    .expect("golden action intent JSON");
    value["retry"] = serde_json::Value::String(retry.to_owned());
    value["idempotency"] = idempotency;
    value["effect"] = effect;
    serde_json::from_value(value).expect("valid ECR-001 retry fixture")
}

fn attempt_for_intent(intent: &ActionIntent, id: &str) -> ActionAttemptRef {
    let id: ActionAttemptId = serde_json::from_str(&format!("\"{id}\"")).expect("attempt id");
    ActionAttemptRef::new(id, intent.action_ref().expect("action ref"))
}

fn receipt_for_attempt(attempt: ActionAttemptRef, id: &str) -> ActionReceipt {
    let receipt_id: ReceiptId = serde_json::from_str(&format!("\"{id}\"")).expect("receipt id");
    let actor: ActorId =
        serde_json::from_str("\"00000000-0000-0000-0000-000000000001\"").expect("actor id");
    ActionReceipt::new(
        receipt_id,
        attempt,
        actor,
        ActionOutcome::ExecutorObservedSuccess,
    )
}

fn state_with_received_attempt(
    intent: &ActionIntent,
    attempt_id: &str,
    receipt_id: &str,
) -> (ecra_run::RunState, ActionAttemptRef) {
    let attempt = attempt_for_intent(intent, attempt_id);
    let receipt = receipt_for_attempt(attempt.clone(), receipt_id);
    let mut history = running();
    push(
        &mut history,
        RunEvent::AttemptPrepared {
            attempt: attempt.clone(),
        },
    );
    push(&mut history, RunEvent::ReceiptRecorded { receipt });
    (
        RunReducer::reduce(&history).expect("received attempt state"),
        attempt,
    )
}

#[test]
fn retry_guard_preserves_all_ecr001_retry_classes() {
    let none_effect = serde_json::json!({"mutation":"none","reversibility":"not_applicable"});
    let safe = intent_with_semantics(
        "safe",
        serde_json::json!({"class":"naturally_idempotent","key_ref":null}),
        none_effect.clone(),
    );
    let (safe_state, safe_attempt) = state_with_received_attempt(
        &safe,
        "00000000-0000-0000-0000-000000000410",
        "00000000-0000-0000-0000-000000000510",
    );
    ensure_retry_allowed(&safe_state, &safe, &safe_attempt).expect("safe retry allowed");

    let keyed = intent_with_semantics(
        "requires_same_idempotency_key",
        serde_json::json!({"class":"idempotent_with_key","key_ref":"phase6-key"}),
        serde_json::json!({"mutation":"local","reversibility":"reversible"}),
    );
    let (keyed_state, keyed_attempt) = state_with_received_attempt(
        &keyed,
        "00000000-0000-0000-0000-000000000411",
        "00000000-0000-0000-0000-000000000511",
    );
    ensure_retry_allowed(&keyed_state, &keyed, &keyed_attempt)
        .expect("same-key retry allowed for exact bound intent");

    let reconcile = intent_with_semantics(
        "requires_external_reconciliation",
        serde_json::json!({"class":"naturally_idempotent","key_ref":null}),
        serde_json::json!({"mutation":"external","reversibility":"reversible"}),
    );
    let (reconcile_state, reconcile_attempt) = state_with_received_attempt(
        &reconcile,
        "00000000-0000-0000-0000-000000000412",
        "00000000-0000-0000-0000-000000000512",
    );
    let error = ensure_retry_allowed(&reconcile_state, &reconcile, &reconcile_attempt)
        .expect_err("reconciliation retry class must not blind retry");
    assert_eq!(error.code(), RunErrorCode::BlindRetryForbidden);

    let never = intent_with_semantics(
        "never_blind_retry",
        serde_json::json!({"class":"naturally_idempotent","key_ref":null}),
        none_effect,
    );
    let (never_state, never_attempt) = state_with_received_attempt(
        &never,
        "00000000-0000-0000-0000-000000000413",
        "00000000-0000-0000-0000-000000000513",
    );
    let error = ensure_retry_allowed(&never_state, &never, &never_attempt)
        .expect_err("never-blind retry class must be refused");
    assert_eq!(error.code(), RunErrorCode::BlindRetryForbidden);
}

#[test]
fn unresolved_attempt_blocks_blind_retry_even_for_naturally_idempotent_safe_action() {
    let intent = intent_with_semantics(
        "safe",
        serde_json::json!({"class":"naturally_idempotent","key_ref":null}),
        serde_json::json!({"mutation":"none","reversibility":"not_applicable"}),
    );
    let attempt = attempt_for_intent(&intent, "00000000-0000-0000-0000-000000000414");
    let mut history = running();
    push(
        &mut history,
        RunEvent::AttemptPrepared {
            attempt: attempt.clone(),
        },
    );
    push(&mut history, event("recovery_boundary"));
    let state = RunReducer::reduce(&history).expect("recovered unresolved state");
    let error = ensure_retry_allowed(&state, &intent, &attempt)
        .expect_err("unresolved attempt must block blind retry");
    assert_eq!(error.code(), RunErrorCode::BlindRetryForbidden);
}

#[test]
fn multiple_attempts_for_one_action_keep_receipts_isolated() {
    let intent = intent_with_semantics(
        "safe",
        serde_json::json!({"class":"naturally_idempotent","key_ref":null}),
        serde_json::json!({"mutation":"none","reversibility":"not_applicable"}),
    );
    let first = attempt_for_intent(&intent, "00000000-0000-0000-0000-000000000415");
    let second = attempt_for_intent(&intent, "00000000-0000-0000-0000-000000000416");
    let first_receipt = receipt_for_attempt(first.clone(), "00000000-0000-0000-0000-000000000515");

    let mut history = running();
    push(
        &mut history,
        RunEvent::AttemptPrepared {
            attempt: first.clone(),
        },
    );
    push(
        &mut history,
        RunEvent::AttemptPrepared {
            attempt: second.clone(),
        },
    );
    push(
        &mut history,
        RunEvent::ReceiptRecorded {
            receipt: first_receipt.clone(),
        },
    );
    let state = RunReducer::reduce(&history).expect("two-attempt state");
    assert_eq!(
        state
            .prepared_attempts()
            .get(&first.id())
            .unwrap()
            .receipt(),
        Some(&first_receipt)
    );
    assert!(
        state
            .prepared_attempts()
            .get(&second.id())
            .unwrap()
            .receipt()
            .is_none()
    );
}
