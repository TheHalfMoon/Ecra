use ecra_core::{ClaimRef, VerificationTarget};
use ecra_verify::{
    CheckpointId, VerificationAggregateStateV1, VerificationCheckpointFieldsV1,
    VerificationCheckpointV1, VerificationRequirementV1,
};

#[test]
fn checkpoint_exposes_no_authority_or_execution_surface() {
    let checkpoint = VerificationCheckpointV1::from_fields(VerificationCheckpointFieldsV1 {
        id: CheckpointId::parse_str("00000000-0000-0000-0000-000000006101").expect("checkpoint id"),
        label: "verification only".to_owned(),
        requirements: vec![
            VerificationRequirementV1::new(
                VerificationTarget::Claim(
                    ClaimRef::new("checkpoint", "boundary").expect("claim target"),
                ),
                vec![VerificationAggregateStateV1::Verified],
            )
            .expect("requirement"),
        ],
    })
    .expect("checkpoint");

    let value = serde_json::to_value(checkpoint).expect("serialize checkpoint");
    let object = value.as_object().expect("checkpoint object");
    for prohibited in [
        "capability_grant",
        "approval",
        "authorization",
        "policy_decision",
        "declassification",
        "secret_handle",
        "executor",
        "execute",
        "schedule",
        "resume",
        "retry_allowed",
    ] {
        assert!(
            !object.contains_key(prohibited),
            "prohibited field: {prohibited}"
        );
    }
}

#[test]
fn checkpoint_module_has_no_run_state_mutation_or_authority_dependency() {
    let source = include_str!("../src/checkpoint.rs");
    for prohibited in [
        "RunState",
        "RunPhase",
        "PreparedAttemptState",
        "RunEvent",
        "CapabilityGrant",
        "ActionReceipt::new",
        "execute(",
        "schedule(",
        "resume(",
    ] {
        assert!(
            !source.contains(prohibited),
            "checkpoint source contains prohibited surface: {prohibited}"
        );
    }
}
