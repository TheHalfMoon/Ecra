use std::any::TypeId;

use ecra_core::{
    ActionReceipt, ActorId, ReceiptId, VerificationId, VerificationMethod, VerificationOutcome,
    VerificationReceipt, VerificationTarget,
};
use ecra_verify::{VerificationRequestFieldsV1, VerificationRequestV1};

#[test]
fn execution_and_domain_types_have_no_parallel_verified_field() {
    let receipt_source = include_str!("../../ecra-core/src/receipt.rs");
    let evidence_source = include_str!("../../ecra-core/src/evidence.rs");
    let artifact_source = include_str!("../../ecra-core/src/artifact.rs");

    assert!(!receipt_source.contains("    verified:"));
    assert!(!evidence_source.contains("    verified:"));
    assert!(!artifact_source.contains("    verified:"));
    assert_ne!(TypeId::of::<ActionReceipt>(), TypeId::of::<VerificationReceipt>());
}

#[test]
fn verification_request_exposes_no_authority_surface() {
    let request = VerificationRequestV1::from_fields(VerificationRequestFieldsV1 {
        receipt_id: VerificationId::parse_str("00000000-0000-0000-0000-000000000801")
            .expect("verification id"),
        verifier: ActorId::parse_str("00000000-0000-0000-0000-000000000001")
            .expect("actor id"),
        verifier_principal: None,
        target: VerificationTarget::Receipt(
            ReceiptId::parse_str("00000000-0000-0000-0000-000000000802")
                .expect("receipt id"),
        ),
        method: VerificationMethod::Other,
        evidence: Vec::new(),
        proposed_outcome: VerificationOutcome::NotEvaluated,
        evaluated_at: None,
        rule_id: "boundary".to_owned(),
        notes: Some("diagnostic metadata only".to_owned()),
    })
    .expect("request");

    let value = serde_json::to_value(request).expect("serialize request");
    let object = value.as_object().expect("request object");
    for prohibited in [
        "verified",
        "capability_grant",
        "approval",
        "authorization",
        "policy_decision",
        "declassification",
        "executor",
        "execute",
        "retry_allowed",
    ] {
        assert!(!object.contains_key(prohibited), "prohibited field: {prohibited}");
    }
}
