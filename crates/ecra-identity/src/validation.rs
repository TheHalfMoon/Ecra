use ecra_core::{
    ActorId, EpochMillis, IdentityAssertionRef, PrincipalId, PrincipalRef, to_jcs_vec,
};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::{
    AssertionAudience, DelegationId, IdentityAssertionDigest, IdentityAssertionV1, IdentityError,
    IdentityErrorCategory, IdentityErrorCode, KeyId, KeyStatus, TrustRootId, TrustStateDigest,
    VerifiedTrustSnapshot,
};

pub const VALIDATED_IDENTITY_CONTEXT_DOMAIN: &[u8] = b"ecra.validated-identity-context.v1\n";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReplayMode {
    ReusableWithinValidity,
    SingleUseNonce,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReplayValidationInput {
    mode: ReplayMode,
    nonce_seen: bool,
}

impl ReplayValidationInput {
    #[must_use]
    pub const fn reusable_within_validity() -> Self {
        Self {
            mode: ReplayMode::ReusableWithinValidity,
            nonce_seen: false,
        }
    }

    #[must_use]
    pub const fn single_use_nonce(nonce_seen: bool) -> Self {
        Self {
            mode: ReplayMode::SingleUseNonce,
            nonce_seen,
        }
    }

    #[must_use]
    pub const fn mode(self) -> ReplayMode {
        self.mode
    }

    #[must_use]
    pub const fn nonce_seen(self) -> bool {
        self.nonce_seen
    }
}

pub struct IdentityValidationContext<'a> {
    evaluated_at: EpochMillis,
    expected_actor_id: ActorId,
    expected_audience: AssertionAudience,
    expected_principal_id: Option<PrincipalId>,
    replay_state: ReplayValidationInput,
    trust_snapshot: &'a VerifiedTrustSnapshot,
}

impl<'a> IdentityValidationContext<'a> {
    #[must_use]
    pub const fn new(
        evaluated_at: EpochMillis,
        expected_actor_id: ActorId,
        expected_audience: AssertionAudience,
        expected_principal_id: Option<PrincipalId>,
        replay_state: ReplayValidationInput,
        trust_snapshot: &'a VerifiedTrustSnapshot,
    ) -> Self {
        Self {
            evaluated_at,
            expected_actor_id,
            expected_audience,
            expected_principal_id,
            replay_state,
            trust_snapshot,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ValidatedOnBehalfOf {
    principal_id: PrincipalId,
    delegation_id: DelegationId,
}

impl ValidatedOnBehalfOf {
    #[must_use]
    pub const fn principal_id(self) -> PrincipalId {
        self.principal_id
    }

    #[must_use]
    pub const fn delegation_id(self) -> DelegationId {
        self.delegation_id
    }
}

/// Pure identity/trust evidence. This type deliberately contains no capability,
/// approval, declassification, authorization decision, lease or secret bytes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ValidatedIdentityContext {
    assertion_ref: IdentityAssertionRef,
    principal: PrincipalRef,
    actor_id: ActorId,
    issuer_trust_root_id: TrustRootId,
    signing_key_id: KeyId,
    audience: AssertionAudience,
    on_behalf_of: Option<ValidatedOnBehalfOf>,
    evaluated_at: EpochMillis,
    assertion_digest: IdentityAssertionDigest,
    trust_state_digest: TrustStateDigest,
}

impl ValidatedIdentityContext {
    #[must_use]
    pub const fn assertion_ref(&self) -> IdentityAssertionRef {
        self.assertion_ref
    }

    #[must_use]
    pub const fn principal(&self) -> PrincipalRef {
        self.principal
    }

    #[must_use]
    pub const fn actor_id(&self) -> ActorId {
        self.actor_id
    }

    #[must_use]
    pub const fn issuer_trust_root_id(&self) -> TrustRootId {
        self.issuer_trust_root_id
    }

    #[must_use]
    pub const fn signing_key_id(&self) -> KeyId {
        self.signing_key_id
    }

    #[must_use]
    pub fn audience(&self) -> &AssertionAudience {
        &self.audience
    }

    #[must_use]
    pub const fn on_behalf_of(&self) -> Option<ValidatedOnBehalfOf> {
        self.on_behalf_of
    }

    #[must_use]
    pub const fn evaluated_at(&self) -> EpochMillis {
        self.evaluated_at
    }

    #[must_use]
    pub const fn assertion_digest(&self) -> IdentityAssertionDigest {
        self.assertion_digest
    }

    #[must_use]
    pub const fn trust_state_digest(&self) -> TrustStateDigest {
        self.trust_state_digest
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, IdentityError> {
        to_jcs_vec(self).map_err(|_| {
            IdentityError::new(
                IdentityErrorCategory::IdentityValidation,
                IdentityErrorCode::CanonicalizationFailed,
                Some("validated_identity_context"),
            )
        })
    }

    pub fn digest_bytes(&self) -> Result<[u8; 32], IdentityError> {
        let canonical = self.canonical_bytes()?;
        let mut hasher = Sha256::new();
        hasher.update(VALIDATED_IDENTITY_CONTEXT_DOMAIN);
        hasher.update(canonical);
        let digest = hasher.finalize();
        let mut output = [0u8; 32];
        output.copy_from_slice(&digest);
        Ok(output)
    }
}

pub fn validate_identity_assertion(
    assertion: &IdentityAssertionV1,
    context: &IdentityValidationContext<'_>,
) -> Result<ValidatedIdentityContext, IdentityError> {
    let payload = assertion.payload();
    let issuer = payload.issuer();
    let snapshot = context.trust_snapshot;

    if issuer.trust_root_id() != snapshot.trust_root_id()
        || payload.subject_principal_id() != snapshot.principal().id()
    {
        return Err(validation_error(
            IdentityErrorCode::TrustSnapshotStaleOrMismatched,
            "trust_snapshot_binding",
        ));
    }

    let key = snapshot.assertion_key(issuer.key_id()).ok_or_else(|| {
        IdentityError::new(
            IdentityErrorCategory::KeyState,
            IdentityErrorCode::KeyNotFound,
            Some("assertion_signing_key"),
        )
    })?;
    if snapshot.is_revoked(key.key_id()) || matches!(key.status(), KeyStatus::Revoked) {
        return Err(IdentityError::new(
            IdentityErrorCategory::KeyState,
            IdentityErrorCode::KeyRevoked,
            Some("assertion_signing_key"),
        ));
    }
    if assertion.signature().algorithm() != key.algorithm() {
        return Err(validation_error(
            IdentityErrorCode::UnsupportedAlgorithm,
            "signature_algorithm_binding",
        ));
    }

    let verifying_key = VerifyingKey::from_bytes(&key.public_key()).map_err(|_| {
        validation_error(
            IdentityErrorCode::TrustSnapshotLifecycleInvalid,
            "assertion_verifying_key",
        )
    })?;
    let signature_bytes = assertion.signature().decoded_bytes()?;
    let signature = Signature::from_slice(&signature_bytes).map_err(|_| {
        validation_error(
            IdentityErrorCode::AssertionSignatureInvalid,
            "assertion_signature",
        )
    })?;
    verifying_key
        .verify(&assertion.signing_input()?, &signature)
        .map_err(|_| {
            validation_error(
                IdentityErrorCode::AssertionSignatureInvalid,
                "assertion_signature",
            )
        })?;

    if let Some(expected_principal) = context.expected_principal_id
        && expected_principal != payload.subject_principal_id()
    {
        return Err(validation_error(
            IdentityErrorCode::AssertionPrincipalMismatch,
            "expected_principal",
        ));
    }
    if context.expected_actor_id != payload.actor_binding().actor_id() {
        return Err(validation_error(
            IdentityErrorCode::AssertionActorMismatch,
            "expected_actor",
        ));
    }
    if context.expected_audience != *payload.audience() {
        return Err(validation_error(
            IdentityErrorCode::AssertionAudienceMismatch,
            "expected_audience",
        ));
    }
    if let Some(not_before) = payload.not_before()
        && context.evaluated_at.get() < not_before.get()
    {
        return Err(validation_error(
            IdentityErrorCode::AssertionNotYetValid,
            "not_before",
        ));
    }
    if context.evaluated_at.get() > payload.expires_at().get() {
        return Err(validation_error(
            IdentityErrorCode::AssertionExpired,
            "expires_at",
        ));
    }

    let on_behalf_of = match payload.on_behalf_of() {
        Some(binding) => {
            if binding.principal_id() != payload.subject_principal_id() {
                return Err(validation_error(
                    IdentityErrorCode::AssertionDelegationInvalid,
                    "on_behalf_of_principal",
                ));
            }
            Some(ValidatedOnBehalfOf {
                principal_id: binding.principal_id(),
                delegation_id: binding.delegation_id(),
            })
        }
        None => None,
    };

    if matches!(context.replay_state.mode(), ReplayMode::SingleUseNonce)
        && (payload.nonce().is_none() || context.replay_state.nonce_seen())
    {
        return Err(validation_error(
            IdentityErrorCode::AssertionReplayRejected,
            "replay_state",
        ));
    }

    Ok(ValidatedIdentityContext {
        assertion_ref: IdentityAssertionRef::new(
            payload.assertion_id(),
            payload.subject_principal_id(),
        ),
        principal: PrincipalRef::new(payload.subject_principal_id()),
        actor_id: payload.actor_binding().actor_id(),
        issuer_trust_root_id: issuer.trust_root_id(),
        signing_key_id: key.key_id(),
        audience: payload.audience().clone(),
        on_behalf_of,
        evaluated_at: context.evaluated_at,
        assertion_digest: assertion.digest()?,
        trust_state_digest: snapshot.trust_state_digest(),
    })
}

fn validation_error(code: IdentityErrorCode, context: &'static str) -> IdentityError {
    IdentityError::new(
        IdentityErrorCategory::IdentityValidation,
        code,
        Some(context),
    )
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use ecra_core::{IdentityAssertionId, PrincipalId};
    use ed25519_dalek::SigningKey;

    use super::*;
    use crate::{
        AssertionAttributes, AssertionAudienceService, AssertionIssuanceRequest, EnrollmentId,
        EnrolledPrincipalHandle, IssuerSession, KeyStatus, TrustStateDigest, VerifiedAssertionKey,
    };

    #[test]
    fn issued_assertion_validates_without_ambient_state() {
        let signing_key = SigningKey::from_bytes(&[7u8; 32]);
        let key_id = KeyId::parse_str("00000000-0000-0000-0000-000000000003").unwrap();
        let snapshot = VerifiedTrustSnapshot::from_authenticated_parts(
            EnrollmentId::parse_str("00000000-0000-0000-0000-000000000030").unwrap(),
            PrincipalRef::new(
                PrincipalId::parse_str("00000000-0000-0000-0000-000000000004").unwrap(),
            ),
            TrustRootId::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
            1,
            vec![VerifiedAssertionKey::new(
                key_id,
                1,
                KeyStatus::Active,
                signing_key.verifying_key().to_bytes(),
            )
            .unwrap()],
            BTreeSet::new(),
            TrustStateDigest::from_bytes([9u8; 32]),
        )
        .unwrap();
        let handle = EnrolledPrincipalHandle::from_verified_snapshot(&snapshot);
        let session = IssuerSession::from_verified_state(
            handle,
            &snapshot,
            signing_key,
            EpochMillis::new(1_000).unwrap(),
        )
        .unwrap();
        let actor = ActorId::parse_str("00000000-0000-0000-0000-000000000005").unwrap();
        let audience = AssertionAudience::new(AssertionAudienceService::EcraPolicyLocal, None);
        let assertion = session
            .issue(AssertionIssuanceRequest::new(
                IdentityAssertionId::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
                actor,
                audience.clone(),
                EpochMillis::new(1_000).unwrap(),
                Some(EpochMillis::new(1_000).unwrap()),
                EpochMillis::new(2_000).unwrap(),
                None,
                AssertionAttributes::empty(),
                None,
            ))
            .unwrap();
        let validation_context = IdentityValidationContext::new(
            EpochMillis::new(1_500).unwrap(),
            actor,
            audience,
            Some(session.principal().id()),
            ReplayValidationInput::reusable_within_validity(),
            &snapshot,
        );
        let validated = validate_identity_assertion(&assertion, &validation_context).unwrap();
        assert_eq!(validated.principal(), session.principal());
        assert_eq!(validated.actor_id(), actor);
    }
}
