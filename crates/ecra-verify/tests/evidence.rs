use ecra_core::{
    ActionAttemptId, ActionAttemptRef, ActionIntent, ActorId, ContentDigest, EpochMillis,
    EvidenceId, EvidenceKind, EvidenceRef, Fact, FactId, ReceiptId, VerificationId,
    VerificationMethod, VerificationOutcome, VerificationReceipt, VerificationTarget,
};
use ecra_verify::{
    DecisionGradeReasonV1, DecisionGradeRuleV1, DecisionGradeStatusV1, FreshnessRuleV1,
    MAX_RECEIPTS_PER_TARGET, VerificationAggregateViewV1, VerificationRequestFieldsV1,
    VerificationRequestV1, VerifyErrorCode, assess_request, verify_request,
};
use proptest::prelude::*;

fn verification_id(value: usize) -> VerificationId {
    VerificationId::parse_str(&format!("00000000-0000-0000-0000-{:012}", 1000 + value))
        .expect("verification id")
}

fn evidence_id(value: usize) -> EvidenceId {
    EvidenceId::parse_str(&format!("00000000-0000-0000-0000-{:012}", 2000 + value))
        .expect("evidence id")
}

fn target_receipt() -> ReceiptId {
    ReceiptId::parse_str("00000000-0000-0000-0000-000000003000").expect("receipt id")
}

fn digest() -> ContentDigest {
    ContentDigest::new(
        "sha256",
        "1111111111111111111111111111111111111111111111111111111111111111",
    )
    .expect("content digest")
}

fn request(
    evidence: Vec<EvidenceRef>,
    outcome: VerificationOutcome,
    method: VerificationMethod,
    evaluated_at: Option<i64>,
) -> VerificationRequestV1 {
    VerificationRequestV1::from_fields(VerificationRequestFieldsV1 {
        receipt_id: verification_id(1),
        verifier: ActorId::parse_str("00000000-0000-0000-0000-000000000001").expect("actor id"),
        verifier_principal: None,
        target: VerificationTarget::Receipt(target_receipt()),
        method,
        evidence,
        proposed_outcome: outcome,
        evaluated_at: evaluated_at.map(|value| EpochMillis::new(value).expect("time")),
        rule_id: "decision_grade_fixture".to_owned(),
        notes: None,
    })
    .expect("request")
}

#[test]
fn mutable_external_reference_without_immutable_binding_cannot_be_conclusive() {
    let evidence = EvidenceRef::new(evidence_id(1), EvidenceKind::ExternalState)
        .with_external_ref("provider:mutable-state")
        .expect("external ref");
    let request = request(
        vec![evidence],
        VerificationOutcome::Verified,
        VerificationMethod::StructuredExternalState,
        None,
    );

    let assessment = assess_request(&request, DecisionGradeRuleV1::standard()).expect("assessment");
    assert_eq!(assessment.status(), DecisionGradeStatusV1::NonDecisionGrade);
    assert!(
        assessment
            .reasons()
            .contains(&DecisionGradeReasonV1::MissingImmutableBinding)
    );
    let error = verify_request(&request, DecisionGradeRuleV1::standard())
        .expect_err("non-decision-grade evidence must not yield Verified");
    assert_eq!(error.code(), VerifyErrorCode::EvidenceInsufficient);
}

#[test]
fn immutable_external_evidence_can_produce_conclusive_canonical_receipt() {
    let evidence = EvidenceRef::new(evidence_id(2), EvidenceKind::ExternalState)
        .with_external_ref("provider:snapshot-v1")
        .expect("external ref")
        .with_content_digest(digest());
    let request = request(
        vec![evidence],
        VerificationOutcome::Verified,
        VerificationMethod::StructuredExternalState,
        None,
    );

    let assessment = assess_request(&request, DecisionGradeRuleV1::standard()).expect("assessment");
    assert!(assessment.is_decision_grade());
    let receipt =
        verify_request(&request, DecisionGradeRuleV1::standard()).expect("verified receipt");
    assert_eq!(receipt.outcome(), VerificationOutcome::Verified);
    assert_eq!(receipt.target(), request.target());
}

#[test]
fn freshness_is_explicit_and_uses_only_supplied_times() {
    let no_as_of =
        EvidenceRef::new(evidence_id(3), EvidenceKind::ExternalState).with_content_digest(digest());
    let request_missing = request(
        vec![no_as_of],
        VerificationOutcome::Verified,
        VerificationMethod::StructuredExternalState,
        Some(10_000),
    );
    let rule = DecisionGradeRuleV1::new(true, Some(FreshnessRuleV1::new(1_000)));
    let missing = assess_request(&request_missing, rule).expect("assessment");
    assert!(
        missing
            .reasons()
            .contains(&DecisionGradeReasonV1::MissingFreshness)
    );

    let fresh = EvidenceRef::new(evidence_id(4), EvidenceKind::ExternalState)
        .with_content_digest(digest())
        .with_as_of(EpochMillis::new(9_500).expect("as-of"));
    let fresh_request = request(
        vec![fresh],
        VerificationOutcome::Verified,
        VerificationMethod::StructuredExternalState,
        Some(10_000),
    );
    assert!(
        assess_request(&fresh_request, rule)
            .expect("fresh assessment")
            .is_decision_grade()
    );

    let stale = EvidenceRef::new(evidence_id(5), EvidenceKind::ExternalState)
        .with_content_digest(digest())
        .with_as_of(EpochMillis::new(8_000).expect("as-of"));
    let stale_request = request(
        vec![stale],
        VerificationOutcome::Verified,
        VerificationMethod::StructuredExternalState,
        Some(10_000),
    );
    assert!(
        assess_request(&stale_request, rule)
            .expect("stale assessment")
            .reasons()
            .contains(&DecisionGradeReasonV1::EvidenceStale)
    );

    let future = EvidenceRef::new(evidence_id(6), EvidenceKind::ExternalState)
        .with_content_digest(digest())
        .with_as_of(EpochMillis::new(10_001).expect("as-of"));
    let future_request = request(
        vec![future],
        VerificationOutcome::Verified,
        VerificationMethod::StructuredExternalState,
        Some(10_000),
    );
    assert!(
        assess_request(&future_request, rule)
            .expect("future assessment")
            .reasons()
            .contains(&DecisionGradeReasonV1::EvidenceFromFuture)
    );
}

#[test]
fn self_attesting_execution_receipt_is_rejected_even_when_immutably_bound() {
    let evidence = EvidenceRef::new(evidence_id(7), EvidenceKind::NetworkReceipt)
        .with_receipt(target_receipt())
        .with_content_digest(digest());
    let request = request(
        vec![evidence],
        VerificationOutcome::Verified,
        VerificationMethod::NetworkReceipt,
        None,
    );
    let error = assess_request(&request, DecisionGradeRuleV1::standard())
        .expect_err("receipt cannot verify itself even with an immutable binding");
    assert_eq!(error.code(), VerifyErrorCode::SelfAttestingReceipt);
}

#[test]
fn action_attempt_network_receipt_cannot_self_verify() {
    let intent: ActionIntent = serde_json::from_str(include_str!(
        "../../../contracts/ecra-domain-v1/valid/action-digest-golden.json"
    ))
    .expect("action intent");
    let attempt = ActionAttemptRef::new(
        ActionAttemptId::parse_str("00000000-0000-0000-0000-000000003100")
            .expect("attempt id"),
        intent.action_ref().expect("action ref"),
    );
    let request = VerificationRequestV1::from_fields(VerificationRequestFieldsV1 {
        receipt_id: verification_id(2),
        verifier: ActorId::parse_str("00000000-0000-0000-0000-000000000001").expect("actor id"),
        verifier_principal: None,
        target: VerificationTarget::ActionAttempt(attempt),
        method: VerificationMethod::NetworkReceipt,
        evidence: vec![
            EvidenceRef::new(evidence_id(10), EvidenceKind::NetworkReceipt)
                .with_content_digest(digest()),
        ],
        proposed_outcome: VerificationOutcome::Verified,
        evaluated_at: None,
        rule_id: "action_attempt_self_attestation".to_owned(),
        notes: None,
    })
    .expect("request");

    let error = verify_request(&request, DecisionGradeRuleV1::standard())
        .expect_err("executor network receipt cannot independently verify its action attempt");
    assert_eq!(error.code(), VerifyErrorCode::SelfAttestingReceipt);
}

#[test]
fn model_judgment_does_not_outrank_missing_independent_evidence() {
    let evidence =
        EvidenceRef::new(evidence_id(8), EvidenceKind::ModelJudgment).with_content_digest(digest());
    let request = request(
        vec![evidence],
        VerificationOutcome::Verified,
        VerificationMethod::IndependentModelJudgment,
        None,
    );
    let assessment = assess_request(&request, DecisionGradeRuleV1::standard()).expect("assessment");
    assert!(
        assessment
            .reasons()
            .contains(&DecisionGradeReasonV1::ModelJudgmentRequiresIndependentEvidence)
    );
    assert!(!assessment.is_decision_grade());
}

#[test]
fn verification_of_fact_target_does_not_mutate_fact_assessment_axes() {
    let fact: Fact = serde_json::from_str(include_str!(
        "../../../contracts/ecra-domain-v1/valid/fact-model-inferred.json"
    ))
    .expect("fact fixture");
    let fact_json = serde_json::to_value(&fact).expect("fact JSON");
    let fact_id = FactId::parse_str(
        fact_json["id"]
            .as_str()
            .expect("fact fixture has canonical id"),
    )
    .expect("fact id");
    let before = serde_json::to_vec(&fact).expect("serialize fact");

    let request = VerificationRequestV1::from_fields(VerificationRequestFieldsV1 {
        receipt_id: verification_id(3),
        verifier: ActorId::parse_str("00000000-0000-0000-0000-000000000001").expect("actor id"),
        verifier_principal: None,
        target: VerificationTarget::Fact(fact_id),
        method: VerificationMethod::DeterministicComputation,
        evidence: vec![
            EvidenceRef::new(evidence_id(9), EvidenceKind::Computation)
                .with_content_digest(digest()),
        ],
        proposed_outcome: VerificationOutcome::Verified,
        evaluated_at: None,
        rule_id: "fact_non_mutation".to_owned(),
        notes: None,
    })
    .expect("fact-targeted request");
    let receipt =
        verify_request(&request, DecisionGradeRuleV1::standard()).expect("verification");
    assert_eq!(receipt.target(), &VerificationTarget::Fact(fact_id));

    let after = serde_json::to_vec(&fact).expect("serialize fact after verification");
    assert_eq!(before, after);
}

#[test]
fn aggregate_exact_receipt_max_is_accepted_and_max_plus_one_is_typed() {
    let target = VerificationTarget::Receipt(target_receipt());
    let receipts = (0..MAX_RECEIPTS_PER_TARGET)
        .map(|index| {
            VerificationReceipt::new(
                verification_id(10_000 + index),
                ActorId::parse_str("00000000-0000-0000-0000-000000000001").expect("actor id"),
                target.clone(),
                VerificationMethod::Other,
                VerificationOutcome::NotEvaluated,
                Vec::new(),
            )
            .expect("receipt")
        })
        .collect::<Vec<_>>();
    let aggregate = VerificationAggregateViewV1::from_receipts(target.clone(), &receipts)
        .expect("exact aggregate max must be accepted");
    assert_eq!(aggregate.receipt_ids().len(), MAX_RECEIPTS_PER_TARGET);

    let mut over = receipts;
    over.push(
        VerificationReceipt::new(
            verification_id(20_000),
            ActorId::parse_str("00000000-0000-0000-0000-000000000001").expect("actor id"),
            target.clone(),
            VerificationMethod::Other,
            VerificationOutcome::NotEvaluated,
            Vec::new(),
        )
        .expect("over-limit receipt"),
    );
    let error = VerificationAggregateViewV1::from_receipts(target, &over)
        .expect_err("receipt max+1 must fail");
    assert_eq!(error.code(), VerifyErrorCode::ResourceLimitExceeded);
}

proptest! {
    #[test]
    fn arbitrary_bounded_evidence_json_never_panics(bytes in proptest::collection::vec(any::<u8>(), 0..=4_096)) {
        let _ = serde_json::from_slice::<EvidenceRef>(&bytes);
    }
}
