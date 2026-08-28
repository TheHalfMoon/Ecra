use ecra_identity::IdentityAssertionV1;

#[test]
fn phase3_public_wire_rejects_invalid_corpus() {
    for fixture in [
        include_bytes!("../../../contracts/ecra-identity-v1/invalid/assertion-unknown-field.json")
            .as_slice(),
        include_bytes!("../../../contracts/ecra-identity-v1/invalid/assertion-unsupported-version.json")
            .as_slice(),
        include_bytes!("../../../contracts/ecra-identity-v1/invalid/assertion-malformed-signature.json")
            .as_slice(),
    ] {
        assert!(IdentityAssertionV1::from_json_slice(fixture).is_err());
    }
}

#[test]
fn stateful_phase3_validation_corpus_stays_inside_the_crate_boundary() {
    let source = include_str!("../src/phase3_tests.rs");
    for required_test in [
        "wrong_signature_is_rejected",
        "wrong_issuer_key_and_subject_are_rejected_before_context_creation",
        "exact_actor_audience_and_time_bindings_fail_closed",
        "signed_cross_principal_delegation_is_rejected",
        "replay_mode_requires_present_unseen_nonce",
        "revoked_key_rejects_current_identity_validation",
        "authenticated_snapshot_rejects_lifecycle_ambiguity",
        "validation_is_deterministic_for_one_thousand_identical_evaluations",
    ] {
        assert!(source.contains(required_test), "missing crate-private validation test: {required_test}");
    }
    assert!(source.contains("for _ in 0..1_000"));
}
