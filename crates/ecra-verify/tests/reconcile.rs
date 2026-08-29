use ecra_core::{
    ActionAttemptId, ActionAttemptRef, ActionIntent, ActionRef, ActorId, ArtifactId, EpochMillis,
    EvidenceId, EvidenceKind, EvidenceRef, RunId, VerificationId, VerificationMethod,
    VerificationOutcome, VerificationReceipt, VerificationTarget,
};
use ecra_run::{
    RecoveryReason, RunEvent, RunEventEnvelope, RunReducer, RunState, ensure_retry_allowed,
};
use ecra_verify::{
    ReconciliationId, ReconciliationInputV1, ReconciliationOutcomeV1,
    ReconciliationRecordFieldsV1, ReconciliationRecordV1, RetryDispositionV1, VerifyErrorCode,
    reconcile, retry_disposition,
};

fn genesis() -> RunEventEnvelope {
    RunEventEnvelope::from_json_slice(include_bytes!(
        "../../../contracts/ecra-run-v1/valid/run-created-envelope.v1.json"
    ))
    .expect("genesis fixture")
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

fn unresolved_history(attempt: &ActionAttemptRef) -> (Vec<RunEventEnvelope>, RunState) {
    let mut history = vec![genesis()];
    push(&mut history, RunEvent::RunStarted {});
    push(
        &mut history,
        RunEvent::AttemptPrepared {
            attempt: attempt.clone(),
        },
    );
    push(
        &mut history,
        RunEvent::RecoveryBoundary {
            reason: RecoveryReason::ProcessRestart,
        },
    );
    let state = RunReducer::reduce(&history).expect("unresolved state");
    (history, state)
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
    serde_json::from_value(value).expect("valid ECR-001 action semantics")
}

fn safe_intent() -> ActionIntent {
    intent_with_semantics(
        "safe",
        serde_json::json!({"class":"naturally_idempotent","key_ref":null}),
        serde_json::json!({"mutation":"none","reversibility":"not_applicable"}),
    )
}

fn attempt_for_intent(intent: &ActionIntent, tail: u64) -> ActionAttemptRef {
    let id = ActionAttemptId::parse_str(&format!("00000000-0000-0000-0000-{tail:012}"))
        .expect("attempt id");
    ActionAttemptRef::new(id, intent.action_ref().expect("action ref"))
}

fn verification_id(tail: u64) -> VerificationId {
    VerificationId::parse_str(&format!("00000000-0000-0000-0000-{tail:012}"))
        .expect("verification id")
}

fn evidence_id(tail: u64) -> EvidenceId {
    EvidenceId::parse_str(&format!("00000000-0000-0000-0000-{tail:012}"))
        .expect("evidence id")
}

fn artifact_id(tail: u64) -> ArtifactId {
    ArtifactId::parse_str(&format!("00000000-0000-0000-0000-{tail:012}"))
        .expect("artifact id")
}

fn receipt(
    tail: u64,
    attempt: &ActionAttemptRef,
    outcome: VerificationOutcome,
    immutable: bool,
) -> VerificationReceipt {
    let mut evidence = EvidenceRef::new(evidence_id(70_000 + tail), EvidenceKind::Other);
    if immutable {
        evidence = evidence.with_artifact(artifact_id(80_000 + tail));
    }
    VerificationReceipt::new(
        verification_id(60_000 + tail),
        ActorId::parse_str("00000000-0000-0000-0000-000000000001").expect("actor id"),
        VerificationTarget::ActionAttempt(attempt.clone()),
        VerificationMethod::ArtifactValidation,
        outcome,
        vec![evidence],
    )
    .expect("verification receipt")
}

fn reconciliation_id(tail: u64) -> ReconciliationId {
    ReconciliationId::parse_str(&format!("00000000-0000-0000-0000-{tail:012}"))
        .expect("reconciliation id")
}

fn reconcile_with(
    state: &RunState,
    attempt: &ActionAttemptRef,
    support: Vec<VerificationId>,
    available: &[VerificationReceipt],
    tail: u64,
) -> ReconciliationRecordV1 {
    reconcile(
        ReconciliationInputV1 {
            id: reconciliation_id(tail),
            run_id: state.run_id(),
            attempt: attempt.clone(),
            action: attempt.action().clone(),
            verification_receipts: support,
            reconciled_at: None,
            notes: Some("synthetic reconciliation fixture".to_owned()),
        },
        state,
        available,
    )
    .expect("reconciliation")
}

#[test]
fn all_outcomes_preserve_ecr002_state_and_same_run_guards() {
    let intent = safe_intent();
    let attempt = attempt_for_intent(&intent, 50_001);
    let (history, state) = unresolved_history(&attempt);
    let before = state.canonical_bytes().expect("canonical state before");

    let verified = receipt(1, &attempt, VerificationOutcome::Verified, true);
    let rejected = receipt(2, &attempt, VerificationOutcome::Rejected, true);
    let cases = [
        (
            vec![verified.id()],
            vec![verified],
            ReconciliationOutcomeV1::EffectConfirmed,
        ),
        (
            vec![rejected.id()],
            vec![rejected],
            ReconciliationOutcomeV1::NoEffectConfirmed,
        ),
        (Vec::new(), Vec::new(), ReconciliationOutcomeV1::StillUnknown),
    ];

    for (index, (support, available, expected)) in cases.into_iter().enumerate() {
        let record = reconcile_with(
            &state,
            &attempt,
            support,
            &available,
            51_000 + u64::try_from(index).expect("index"),
        );
        assert_eq!(record.outcome(), expected);
        assert_eq!(state.canonical_bytes().expect("canonical state after"), before);
        assert!(state.unresolved_attempts().contains(&attempt.id()));
        let prepared = state
            .prepared_attempts()
            .get(&attempt.id())
            .expect("prepared attempt");
        assert!(prepared.unresolved());
        assert!(prepared.receipt().is_none());

        let mut resumed = history.clone();
        push(&mut resumed, RunEvent::RunResumed {});
        assert!(RunReducer::apply(&state, resumed.last().expect("resume event")).is_err());

        let mut completed = history.clone();
        push(&mut completed, RunEvent::ExecutionCompleted {});
        assert!(RunReducer::apply(&state, completed.last().expect("completion event")).is_err());

        assert!(ensure_retry_allowed(&state, &intent, &attempt).is_err());
    }
}

#[test]
fn exact_run_attempt_action_and_support_binding_fail_closed() {
    let intent = safe_intent();
    let attempt = attempt_for_intent(&intent, 52_001);
    let (_, state) = unresolved_history(&attempt);
    let verified = receipt(3, &attempt, VerificationOutcome::Verified, true);

    let wrong_run = RunId::parse_str("00000000-0000-0000-0000-000000099999")
        .expect("wrong run id");
    let error = reconcile(
        ReconciliationInputV1 {
            id: reconciliation_id(52_101),
            run_id: wrong_run,
            attempt: attempt.clone(),
            action: attempt.action().clone(),
            verification_receipts: vec![verified.id()],
            reconciled_at: None,
            notes: None,
        },
        &state,
        std::slice::from_ref(&verified),
    )
    .expect_err("cross-run reconciliation must fail");
    assert_eq!(error.code(), VerifyErrorCode::AttemptBindingMismatch);

    let mut action_value = serde_json::to_value(attempt.action()).expect("action json");
    action_value["id"] = "00000000-0000-0000-0000-000000099998".into();
    let wrong_action: ActionRef = serde_json::from_value(action_value).expect("wrong action ref");
    let error = reconcile(
        ReconciliationInputV1 {
            id: reconciliation_id(52_102),
            run_id: state.run_id(),
            attempt: attempt.clone(),
            action: wrong_action,
            verification_receipts: vec![verified.id()],
            reconciled_at: None,
            notes: None,
        },
        &state,
        std::slice::from_ref(&verified),
    )
    .expect_err("cross-action reconciliation must fail");
    assert_eq!(error.code(), VerifyErrorCode::AttemptBindingMismatch);

    let missing = verification_id(69_999);
    let error = reconcile(
        ReconciliationInputV1 {
            id: reconciliation_id(52_103),
            run_id: state.run_id(),
            attempt: attempt.clone(),
            action: attempt.action().clone(),
            verification_receipts: vec![missing],
            reconciled_at: None,
            notes: None,
        },
        &state,
        std::slice::from_ref(&verified),
    )
    .expect_err("missing support id must fail");
    assert_eq!(error.code(), VerifyErrorCode::InvalidEvidence);

    let error = reconcile(
        ReconciliationInputV1 {
            id: reconciliation_id(52_104),
            run_id: state.run_id(),
            attempt: attempt.clone(),
            action: attempt.action().clone(),
            verification_receipts: vec![verified.id(), verified.id()],
            reconciled_at: None,
            notes: None,
        },
        &state,
        std::slice::from_ref(&verified),
    )
    .expect_err("duplicate support ids must fail");
    assert_eq!(error.code(), VerifyErrorCode::DuplicateId);

    let other_attempt = ActionAttemptRef::new(
        ActionAttemptId::parse_str("00000000-0000-0000-0000-000000052999")
            .expect("other attempt id"),
        attempt.action().clone(),
    );
    let cross_target = receipt(4, &other_attempt, VerificationOutcome::Verified, true);
    let error = reconcile(
        ReconciliationInputV1 {
            id: reconciliation_id(52_105),
            run_id: state.run_id(),
            attempt: attempt.clone(),
            action: attempt.action().clone(),
            verification_receipts: vec![cross_target.id()],
            reconciled_at: None,
            notes: None,
        },
        &state,
        &[cross_target],
    )
    .expect_err("cross-target support must fail");
    assert_eq!(error.code(), VerifyErrorCode::InvalidTarget);
}

#[test]
fn unknown_support_rules_follow_ic003_without_fabrication() {
    let intent = safe_intent();
    let attempt = attempt_for_intent(&intent, 53_001);
    let (_, state) = unresolved_history(&attempt);

    let absent = reconcile_with(&state, &attempt, Vec::new(), &[], 53_101);
    assert_eq!(absent.outcome(), ReconciliationOutcomeV1::StillUnknown);
    assert!(absent.verification_receipts().is_empty());

    let verified = receipt(5, &attempt, VerificationOutcome::Verified, true);
    let rejected = receipt(6, &attempt, VerificationOutcome::Rejected, true);
    let conflict = reconcile_with(
        &state,
        &attempt,
        vec![rejected.id(), verified.id()],
        &[verified.clone(), rejected.clone()],
        53_102,
    );
    assert_eq!(conflict.outcome(), ReconciliationOutcomeV1::StillUnknown);
    assert_eq!(conflict.verification_receipts().len(), 2);
    assert!(conflict.verification_receipts().contains(&verified.id()));
    assert!(conflict.verification_receipts().contains(&rejected.id()));

    let weak = receipt(7, &attempt, VerificationOutcome::Verified, false);
    let insufficient = reconcile_with(
        &state,
        &attempt,
        vec![weak.id()],
        std::slice::from_ref(&weak),
        53_103,
    );
    assert_eq!(
        insufficient.outcome(),
        ReconciliationOutcomeV1::StillUnknown
    );
    assert_eq!(insufficient.verification_receipts(), &[weak.id()]);

    for outcome in [
        ReconciliationOutcomeV1::EffectConfirmed,
        ReconciliationOutcomeV1::NoEffectConfirmed,
    ] {
        let error = ReconciliationRecordV1::from_fields(ReconciliationRecordFieldsV1 {
            id: reconciliation_id(53_200),
            run_id: state.run_id(),
            attempt: attempt.clone(),
            action: attempt.action().clone(),
            outcome,
            verification_receipts: Vec::new(),
            reconciled_at: None,
            notes: None,
        })
        .expect_err("conclusive empty support must fail");
        assert_eq!(error.code(), VerifyErrorCode::EvidenceInsufficient);
    }
}

fn no_effect_record(
    intent: &ActionIntent,
    attempt_tail: u64,
    receipt_tail: u64,
    record_tail: u64,
) -> (RunState, ActionAttemptRef, ReconciliationRecordV1) {
    let attempt = attempt_for_intent(intent, attempt_tail);
    let (_, state) = unresolved_history(&attempt);
    let rejected = receipt(receipt_tail, &attempt, VerificationOutcome::Rejected, true);
    let record = reconcile_with(
        &state,
        &attempt,
        vec![rejected.id()],
        std::slice::from_ref(&rejected),
        record_tail,
    );
    assert_eq!(record.outcome(), ReconciliationOutcomeV1::NoEffectConfirmed);
    assert!(state.unresolved_attempts().contains(&attempt.id()));
    (state, attempt, record)
}

#[test]
fn retry_disposition_matrix_is_advisory_and_fail_closed() {
    let safe = safe_intent();
    let (safe_state, safe_attempt, safe_record) = no_effect_record(&safe, 54_001, 10, 54_101);
    assert_eq!(
        retry_disposition(&safe, &safe_attempt, &safe_state, Some(&safe_record), None)
            .expect("safe advisory"),
        RetryDispositionV1::SemanticallyRetryable
    );

    let keyed = intent_with_semantics(
        "requires_same_idempotency_key",
        serde_json::json!({"class":"idempotent_with_key","key_ref":"phase5-key"}),
        serde_json::json!({"mutation":"local","reversibility":"reversible"}),
    );
    let (keyed_state, keyed_attempt, keyed_record) =
        no_effect_record(&keyed, 54_002, 11, 54_102);
    assert_eq!(
        retry_disposition(
            &keyed,
            &keyed_attempt,
            &keyed_state,
            Some(&keyed_record),
            Some("phase5-key"),
        )
        .expect("same-key advisory"),
        RetryDispositionV1::SemanticallyRetryableSameKey
    );
    assert_eq!(
        retry_disposition(
            &keyed,
            &keyed_attempt,
            &keyed_state,
            Some(&keyed_record),
            Some("mutated-key"),
        )
        .expect("mutated key advisory"),
        RetryDispositionV1::RequiresExplicitNonblindPath
    );

    for (index, intent) in [
        intent_with_semantics(
            "requires_external_reconciliation",
            serde_json::json!({"class":"naturally_idempotent","key_ref":null}),
            serde_json::json!({"mutation":"external","reversibility":"reversible"}),
        ),
        intent_with_semantics(
            "requires_external_reconciliation",
            serde_json::json!({"class":"unknown","key_ref":null}),
            serde_json::json!({"mutation":"external","reversibility":"conditional"}),
        ),
        intent_with_semantics(
            "never_blind_retry",
            serde_json::json!({"class":"naturally_idempotent","key_ref":null}),
            serde_json::json!({"mutation":"none","reversibility":"not_applicable"}),
        ),
        intent_with_semantics(
            "never_blind_retry",
            serde_json::json!({"class":"non_idempotent","key_ref":null}),
            serde_json::json!({"mutation":"local","reversibility":"irreversible"}),
        ),
        intent_with_semantics(
            "never_blind_retry",
            serde_json::json!({"class":"unknown","key_ref":null}),
            serde_json::json!({"mutation":"unknown","reversibility":"unknown"}),
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let offset = u64::try_from(index).expect("index");
        let (state, attempt, record) = no_effect_record(
            &intent,
            54_010 + offset,
            20 + offset,
            54_110 + offset,
        );
        assert_eq!(
            retry_disposition(&intent, &attempt, &state, Some(&record), None)
                .expect("nonblind advisory"),
            RetryDispositionV1::RequiresExplicitNonblindPath
        );
        assert!(state.unresolved_attempts().contains(&attempt.id()));
    }

    let effect_receipt = receipt(30, &safe_attempt, VerificationOutcome::Verified, true);
    let effect_record = reconcile_with(
        &safe_state,
        &safe_attempt,
        vec![effect_receipt.id()],
        std::slice::from_ref(&effect_receipt),
        54_130,
    );
    assert_eq!(
        retry_disposition(
            &safe,
            &safe_attempt,
            &safe_state,
            Some(&effect_record),
            None,
        )
        .expect("duplicate advisory"),
        RetryDispositionV1::DuplicateRetryBlocked
    );

    let unknown_record = reconcile_with(&safe_state, &safe_attempt, Vec::new(), &[], 54_131);
    assert_eq!(
        retry_disposition(
            &safe,
            &safe_attempt,
            &safe_state,
            Some(&unknown_record),
            None,
        )
        .expect("unknown advisory"),
        RetryDispositionV1::ReconciliationRequired
    );
    assert_eq!(
        retry_disposition(&safe, &safe_attempt, &safe_state, None, None)
            .expect("missing reconciliation advisory"),
        RetryDispositionV1::ReconciliationRequired
    );
}

#[test]
fn strict_record_json_rejects_unknown_fields_and_outcome_tampering() {
    let intent = safe_intent();
    let attempt = attempt_for_intent(&intent, 55_001);
    let (_, state) = unresolved_history(&attempt);
    let verified = receipt(40, &attempt, VerificationOutcome::Verified, true);
    let record = reconcile_with(
        &state,
        &attempt,
        vec![verified.id()],
        std::slice::from_ref(&verified),
        55_101,
    );
    let mut value = serde_json::to_value(&record).expect("record json");
    value["unexpected"] = true.into();
    assert!(
        ReconciliationRecordV1::from_json_slice(
            &serde_json::to_vec(&value).expect("unknown field bytes")
        )
        .is_err()
    );

    let mut tampered = serde_json::to_value(&record).expect("record json");
    tampered["outcome"] = "no_effect_confirmed".into();
    let parsed = ReconciliationRecordV1::from_json_slice(
        &serde_json::to_vec(&tampered).expect("tampered bytes"),
    )
    .expect("static record parsing does not have run evidence");
    let error = parsed
        .validate_against(&state, std::slice::from_ref(&verified))
        .expect_err("tampered outcome must fail against evidence");
    assert_eq!(error.code(), VerifyErrorCode::VerificationConflict);
}
