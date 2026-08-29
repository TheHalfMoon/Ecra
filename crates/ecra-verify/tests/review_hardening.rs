use ecra_core::{
    ActorId, EvidenceId, EvidenceKind, EvidenceRef, ReceiptId, VerificationId, VerificationMethod,
    VerificationOutcome, VerificationTarget,
};
use ecra_verify::{
    MAX_VERIFICATION_CHECKPOINT_BYTES, MAX_VERIFICATION_REQUEST_BYTES, VerificationCheckpointV1,
    VerificationRequestFieldsV1, VerificationRequestV1, VerifyErrorCode,
};

#[test]
fn oversized_request_json_fails_before_deserialization() {
    let input = vec![b' '; MAX_VERIFICATION_REQUEST_BYTES + 1];
    let error = VerificationRequestV1::from_json_slice(&input)
        .expect_err("oversized request input must fail closed");
    assert_eq!(error.code(), VerifyErrorCode::ResourceLimitExceeded);
}

#[test]
fn oversized_nested_request_reference_fails_complete_size_check() {
    let evidence = EvidenceRef::new(
        EvidenceId::parse_str("00000000-0000-0000-0000-000000090001").expect("evidence id"),
        EvidenceKind::ExternalState,
    )
    .with_external_ref("x".repeat(MAX_VERIFICATION_REQUEST_BYTES))
    .expect("non-empty external ref");
    let error = VerificationRequestV1::from_fields(VerificationRequestFieldsV1 {
        receipt_id: VerificationId::parse_str("00000000-0000-0000-0000-000000090002")
            .expect("verification id"),
        verifier: ActorId::parse_str("00000000-0000-0000-0000-000000000001").expect("actor id"),
        verifier_principal: None,
        target: VerificationTarget::Receipt(
            ReceiptId::parse_str("00000000-0000-0000-0000-000000090003").expect("receipt id"),
        ),
        method: VerificationMethod::Other,
        evidence: vec![evidence],
        proposed_outcome: VerificationOutcome::NotEvaluated,
        evaluated_at: None,
        rule_id: "review_hardening".to_owned(),
        notes: None,
    })
    .expect_err("oversized nested reference must fail complete request size check");
    assert_eq!(error.code(), VerifyErrorCode::ResourceLimitExceeded);
}

#[test]
fn oversized_checkpoint_json_fails_before_deserialization() {
    let input = vec![b' '; MAX_VERIFICATION_CHECKPOINT_BYTES + 1];
    let error = VerificationCheckpointV1::from_json_slice(&input)
        .expect_err("oversized checkpoint input must fail closed");
    assert_eq!(error.code(), VerifyErrorCode::ResourceLimitExceeded);
}
