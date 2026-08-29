use std::collections::BTreeSet;

use ecra_core::{ActorId, EpochMillis, IdentityAssertionId, PrincipalId, PrincipalRef};
use ed25519_dalek::{Signer, SigningKey};

use crate::{
    ActorBinding, AssertionAttributes, AssertionAudience, AssertionAudienceService,
    AssertionIssuanceRequest, AssertionIssuer, AssertionNonceId, AssertionSignature, DelegationId,
    EnrolledPrincipalHandle, EnrollmentId, IdentityAssertionPayloadV1, IdentityAssertionV1,
    IdentityErrorCode, IdentityValidationContext, IssuerSession, KeyId, KeyStatus,
    OnBehalfOfBinding, ReplayValidationInput, TrustRootId, TrustStateDigest, VerifiedAssertionKey,
    VerifiedTrustSnapshot, validate_identity_assertion,
};

const ASSERTION_ID: &str = "00000000-0000-0000-0000-000000000001";
const TRUST_ROOT_ID: &str = "00000000-0000-0000-0000-000000000002";
const KEY_ID: &str = "00000000-0000-0000-0000-000000000003";
const PRINCIPAL_ID: &str = "00000000-0000-0000-0000-000000000004";
const ACTOR_ID: &str = "00000000-0000-0000-0000-000000000005";
const DELEGATION_ID: &str = "00000000-0000-0000-0000-000000000006";
const ENROLLMENT_ID: &str = "00000000-0000-0000-0000-000000000030";
const NONCE_ID: &str = "00000000-0000-0000-0000-000000000031";

struct Fixture {
    signing_key: SigningKey,
    snapshot: VerifiedTrustSnapshot,
    session: IssuerSession,
    actor: ActorId,
    audience: AssertionAudience,
}

fn fixture() -> Fixture {
    let signing_key = SigningKey::from_bytes(&[7u8; 32]);
    let key_id = KeyId::parse_str(KEY_ID).unwrap();
    let snapshot = VerifiedTrustSnapshot::from_authenticated_parts(
        EnrollmentId::parse_str(ENROLLMENT_ID).unwrap(),
        PrincipalRef::new(PrincipalId::parse_str(PRINCIPAL_ID).unwrap()),
        TrustRootId::parse_str(TRUST_ROOT_ID).unwrap(),
        1,
        vec![
            VerifiedAssertionKey::new(
                key_id,
                1,
                KeyStatus::Active,
                signing_key.verifying_key().to_bytes(),
            )
            .unwrap(),
        ],
        BTreeSet::new(),
        TrustStateDigest::from_bytes([9u8; 32]),
    )
    .unwrap();
    let handle = EnrolledPrincipalHandle::from_verified_snapshot(&snapshot);
    let session = IssuerSession::from_verified_state(
        handle,
        &snapshot,
        signing_key.clone(),
        EpochMillis::new(1_000).unwrap(),
    )
    .unwrap();
    Fixture {
        signing_key,
        snapshot,
        session,
        actor: ActorId::parse_str(ACTOR_ID).unwrap(),
        audience: AssertionAudience::new(AssertionAudienceService::EcraPolicyLocal, None),
    }
}

fn issue(
    fixture: &Fixture,
    nonce: Option<AssertionNonceId>,
    delegation: Option<DelegationId>,
) -> IdentityAssertionV1 {
    fixture
        .session
        .issue(AssertionIssuanceRequest::new(
            IdentityAssertionId::parse_str(ASSERTION_ID).unwrap(),
            fixture.actor,
            fixture.audience.clone(),
            EpochMillis::new(1_000).unwrap(),
            Some(EpochMillis::new(1_000).unwrap()),
            EpochMillis::new(2_000).unwrap(),
            nonce,
            AssertionAttributes::empty(),
            delegation,
        ))
        .unwrap()
}

fn context<'a>(
    fixture: &'a Fixture,
    evaluated_at: i64,
    actor: ActorId,
    audience: AssertionAudience,
    replay: ReplayValidationInput,
) -> IdentityValidationContext<'a> {
    IdentityValidationContext::new(
        EpochMillis::new(evaluated_at).unwrap(),
        actor,
        audience,
        Some(fixture.session.principal().id()),
        replay,
        &fixture.snapshot,
    )
}

fn sign_payload(
    signing_key: &SigningKey,
    payload: IdentityAssertionPayloadV1,
) -> IdentityAssertionV1 {
    let signature = signing_key.sign(&payload.signing_input().unwrap());
    let key_id = payload.issuer().key_id();
    payload
        .into_signed(AssertionSignature::from_bytes(key_id, signature.to_bytes()))
        .unwrap()
}

#[test]
fn signed_assertion_round_trips_and_validates() {
    let fixture = fixture();
    let assertion = issue(&fixture, None, None);
    let wire = serde_json::to_vec(&assertion).unwrap();
    let parsed = IdentityAssertionV1::from_json_slice(&wire).unwrap();
    let validated = validate_identity_assertion(
        &parsed,
        &context(
            &fixture,
            1_500,
            fixture.actor,
            fixture.audience.clone(),
            ReplayValidationInput::reusable_within_validity(),
        ),
    )
    .unwrap();
    assert_eq!(validated.principal(), fixture.session.principal());
    assert_eq!(
        validated.signing_key_id(),
        KeyId::parse_str(KEY_ID).unwrap()
    );
}

#[test]
fn wrong_signature_is_rejected() {
    let fixture = fixture();
    let assertion = issue(&fixture, None, None);
    let mut wire = serde_json::to_value(&assertion).unwrap();
    let signature = wire["signature"]["bytes_b64url"].as_str().unwrap();
    let mut mutated = signature.as_bytes().to_vec();
    mutated[0] = if mutated[0] == b'A' { b'B' } else { b'A' };
    wire["signature"]["bytes_b64url"] = String::from_utf8(mutated).unwrap().into();
    let parsed = IdentityAssertionV1::from_json_slice(&serde_json::to_vec(&wire).unwrap()).unwrap();
    let error = validate_identity_assertion(
        &parsed,
        &context(
            &fixture,
            1_500,
            fixture.actor,
            fixture.audience.clone(),
            ReplayValidationInput::reusable_within_validity(),
        ),
    )
    .unwrap_err();
    assert_eq!(error.code(), IdentityErrorCode::AssertionSignatureInvalid);
}

#[test]
fn wrong_issuer_key_and_subject_are_rejected_before_context_creation() {
    let fixture = fixture();
    let assertion = issue(&fixture, None, None);

    let mut wrong_key = serde_json::to_value(&assertion).unwrap();
    let other_key = "00000000-0000-0000-0000-000000000013";
    wrong_key["issuer"]["key_id"] = other_key.into();
    wrong_key["signature"]["key_id"] = other_key.into();
    let parsed =
        IdentityAssertionV1::from_json_slice(&serde_json::to_vec(&wrong_key).unwrap()).unwrap();
    let error = validate_identity_assertion(
        &parsed,
        &context(
            &fixture,
            1_500,
            fixture.actor,
            fixture.audience.clone(),
            ReplayValidationInput::reusable_within_validity(),
        ),
    )
    .unwrap_err();
    assert_eq!(error.code(), IdentityErrorCode::KeyNotFound);

    let mut wrong_subject = serde_json::to_value(&assertion).unwrap();
    wrong_subject["subject_principal_id"] = "00000000-0000-0000-0000-000000000014".into();
    let parsed =
        IdentityAssertionV1::from_json_slice(&serde_json::to_vec(&wrong_subject).unwrap()).unwrap();
    let error = validate_identity_assertion(
        &parsed,
        &context(
            &fixture,
            1_500,
            fixture.actor,
            fixture.audience.clone(),
            ReplayValidationInput::reusable_within_validity(),
        ),
    )
    .unwrap_err();
    assert_eq!(
        error.code(),
        IdentityErrorCode::TrustSnapshotStaleOrMismatched
    );
}

#[test]
fn exact_actor_audience_and_time_bindings_fail_closed() {
    let fixture = fixture();
    let assertion = issue(&fixture, None, None);
    let wrong_actor = ActorId::parse_str("00000000-0000-0000-0000-000000000015").unwrap();
    let error = validate_identity_assertion(
        &assertion,
        &context(
            &fixture,
            1_500,
            wrong_actor,
            fixture.audience.clone(),
            ReplayValidationInput::reusable_within_validity(),
        ),
    )
    .unwrap_err();
    assert_eq!(error.code(), IdentityErrorCode::AssertionActorMismatch);

    let wrong_audience = AssertionAudience::new(
        AssertionAudienceService::EcraPolicyLocal,
        Some(crate::AudienceInstanceId::new("other-instance".to_owned()).unwrap()),
    );
    let error = validate_identity_assertion(
        &assertion,
        &context(
            &fixture,
            1_500,
            fixture.actor,
            wrong_audience,
            ReplayValidationInput::reusable_within_validity(),
        ),
    )
    .unwrap_err();
    assert_eq!(error.code(), IdentityErrorCode::AssertionAudienceMismatch);

    let error = validate_identity_assertion(
        &assertion,
        &context(
            &fixture,
            999,
            fixture.actor,
            fixture.audience.clone(),
            ReplayValidationInput::reusable_within_validity(),
        ),
    )
    .unwrap_err();
    assert_eq!(error.code(), IdentityErrorCode::AssertionNotYetValid);

    let error = validate_identity_assertion(
        &assertion,
        &context(
            &fixture,
            2_001,
            fixture.actor,
            fixture.audience.clone(),
            ReplayValidationInput::reusable_within_validity(),
        ),
    )
    .unwrap_err();
    assert_eq!(error.code(), IdentityErrorCode::AssertionExpired);
}

#[test]
fn signed_cross_principal_delegation_is_rejected() {
    let fixture = fixture();
    let payload = IdentityAssertionPayloadV1::new(
        IdentityAssertionId::parse_str(ASSERTION_ID).unwrap(),
        AssertionIssuer::new(
            TrustRootId::parse_str(TRUST_ROOT_ID).unwrap(),
            KeyId::parse_str(KEY_ID).unwrap(),
        ),
        PrincipalId::parse_str(PRINCIPAL_ID).unwrap(),
        ActorBinding::new(fixture.actor),
        Some(OnBehalfOfBinding::new(
            PrincipalId::parse_str("00000000-0000-0000-0000-000000000014").unwrap(),
            DelegationId::parse_str(DELEGATION_ID).unwrap(),
        )),
        fixture.audience.clone(),
        EpochMillis::new(1_000).unwrap(),
        Some(EpochMillis::new(1_000).unwrap()),
        EpochMillis::new(2_000).unwrap(),
        None,
        AssertionAttributes::empty(),
    )
    .unwrap();
    let assertion = sign_payload(&fixture.signing_key, payload);
    let error = validate_identity_assertion(
        &assertion,
        &context(
            &fixture,
            1_500,
            fixture.actor,
            fixture.audience.clone(),
            ReplayValidationInput::reusable_within_validity(),
        ),
    )
    .unwrap_err();
    assert_eq!(error.code(), IdentityErrorCode::AssertionDelegationInvalid);
}

#[test]
fn replay_mode_requires_present_unseen_nonce() {
    let fixture = fixture();
    let no_nonce = issue(&fixture, None, None);
    let error = validate_identity_assertion(
        &no_nonce,
        &context(
            &fixture,
            1_500,
            fixture.actor,
            fixture.audience.clone(),
            ReplayValidationInput::single_use_nonce(false),
        ),
    )
    .unwrap_err();
    assert_eq!(error.code(), IdentityErrorCode::AssertionReplayRejected);

    let with_nonce = issue(
        &fixture,
        Some(AssertionNonceId::parse_str(NONCE_ID).unwrap()),
        None,
    );
    let error = validate_identity_assertion(
        &with_nonce,
        &context(
            &fixture,
            1_500,
            fixture.actor,
            fixture.audience.clone(),
            ReplayValidationInput::single_use_nonce(true),
        ),
    )
    .unwrap_err();
    assert_eq!(error.code(), IdentityErrorCode::AssertionReplayRejected);

    validate_identity_assertion(
        &with_nonce,
        &context(
            &fixture,
            1_500,
            fixture.actor,
            fixture.audience.clone(),
            ReplayValidationInput::single_use_nonce(false),
        ),
    )
    .unwrap();
}

#[test]
fn revoked_key_rejects_current_identity_validation() {
    let fixture = fixture();
    let key_id = KeyId::parse_str(KEY_ID).unwrap();
    let mut revoked = BTreeSet::new();
    revoked.insert(key_id);
    let snapshot = VerifiedTrustSnapshot::from_authenticated_parts(
        EnrollmentId::parse_str(ENROLLMENT_ID).unwrap(),
        PrincipalRef::new(PrincipalId::parse_str(PRINCIPAL_ID).unwrap()),
        TrustRootId::parse_str(TRUST_ROOT_ID).unwrap(),
        1,
        vec![
            VerifiedAssertionKey::new(
                key_id,
                1,
                KeyStatus::Revoked,
                fixture.signing_key.verifying_key().to_bytes(),
            )
            .unwrap(),
        ],
        revoked,
        TrustStateDigest::from_bytes([10u8; 32]),
    )
    .unwrap();
    let payload = IdentityAssertionPayloadV1::new(
        IdentityAssertionId::parse_str(ASSERTION_ID).unwrap(),
        AssertionIssuer::new(TrustRootId::parse_str(TRUST_ROOT_ID).unwrap(), key_id),
        PrincipalId::parse_str(PRINCIPAL_ID).unwrap(),
        ActorBinding::new(fixture.actor),
        None,
        fixture.audience.clone(),
        EpochMillis::new(1_000).unwrap(),
        Some(EpochMillis::new(1_000).unwrap()),
        EpochMillis::new(2_000).unwrap(),
        None,
        AssertionAttributes::empty(),
    )
    .unwrap();
    let assertion = sign_payload(&fixture.signing_key, payload);
    let validation_context = IdentityValidationContext::new(
        EpochMillis::new(1_500).unwrap(),
        fixture.actor,
        fixture.audience.clone(),
        Some(PrincipalId::parse_str(PRINCIPAL_ID).unwrap()),
        ReplayValidationInput::reusable_within_validity(),
        &snapshot,
    );
    let error = validate_identity_assertion(&assertion, &validation_context).unwrap_err();
    assert_eq!(error.code(), IdentityErrorCode::KeyRevoked);
}

#[test]
fn authenticated_snapshot_rejects_lifecycle_ambiguity() {
    let fixture = fixture();
    let first = VerifiedAssertionKey::new(
        KeyId::parse_str(KEY_ID).unwrap(),
        1,
        KeyStatus::Active,
        fixture.signing_key.verifying_key().to_bytes(),
    )
    .unwrap();
    let second = VerifiedAssertionKey::new(
        KeyId::parse_str("00000000-0000-0000-0000-000000000013").unwrap(),
        1,
        KeyStatus::Active,
        fixture.signing_key.verifying_key().to_bytes(),
    )
    .unwrap();
    let error = VerifiedTrustSnapshot::from_authenticated_parts(
        EnrollmentId::parse_str(ENROLLMENT_ID).unwrap(),
        PrincipalRef::new(PrincipalId::parse_str(PRINCIPAL_ID).unwrap()),
        TrustRootId::parse_str(TRUST_ROOT_ID).unwrap(),
        1,
        vec![first, second],
        BTreeSet::new(),
        TrustStateDigest::from_bytes([11u8; 32]),
    )
    .unwrap_err();
    assert_eq!(
        error.code(),
        IdentityErrorCode::TrustSnapshotLifecycleInvalid
    );
}

#[test]
fn validation_is_deterministic_for_one_thousand_identical_evaluations() {
    let fixture = fixture();
    let assertion = issue(
        &fixture,
        Some(AssertionNonceId::parse_str(NONCE_ID).unwrap()),
        Some(DelegationId::parse_str(DELEGATION_ID).unwrap()),
    );
    let validation_context = context(
        &fixture,
        1_500,
        fixture.actor,
        fixture.audience.clone(),
        ReplayValidationInput::single_use_nonce(false),
    );
    let first = validate_identity_assertion(&assertion, &validation_context).unwrap();
    let expected_bytes = first.canonical_bytes().unwrap();
    let expected_digest = first.digest_bytes().unwrap();
    for _ in 0..1_000 {
        let current = validate_identity_assertion(&assertion, &validation_context).unwrap();
        assert_eq!(current.canonical_bytes().unwrap(), expected_bytes);
        assert_eq!(current.digest_bytes().unwrap(), expected_digest);
    }
}
