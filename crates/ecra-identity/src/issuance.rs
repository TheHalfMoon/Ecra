use ecra_core::{ActorId, EpochMillis, IdentityAssertionId, PrincipalRef};
use ed25519_dalek::{Signer, SigningKey};

use crate::{
    ActorBinding, AssertionAttributes, AssertionAudience, AssertionSignature, DelegationId,
    EnrolledPrincipalHandle, IdentityAssertionPayloadV1, IdentityAssertionV1, IdentityError,
    IdentityErrorCategory, IdentityErrorCode, KeyId, KeyStatus, OnBehalfOfBinding, TrustRootId,
    TrustStateDigest, VerifiedTrustSnapshot,
};

/// Caller-controlled issuance data deliberately has no subject-principal field.
/// The subject is always sourced from the authenticated `IssuerSession`.
pub struct AssertionIssuanceRequest {
    assertion_id: IdentityAssertionId,
    actor_id: ActorId,
    audience: AssertionAudience,
    issued_at: EpochMillis,
    not_before: Option<EpochMillis>,
    expires_at: EpochMillis,
    nonce: Option<crate::AssertionNonceId>,
    attributes: AssertionAttributes,
    on_behalf_of_delegation: Option<DelegationId>,
}

impl AssertionIssuanceRequest {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        assertion_id: IdentityAssertionId,
        actor_id: ActorId,
        audience: AssertionAudience,
        issued_at: EpochMillis,
        not_before: Option<EpochMillis>,
        expires_at: EpochMillis,
        nonce: Option<crate::AssertionNonceId>,
        attributes: AssertionAttributes,
        on_behalf_of_delegation: Option<DelegationId>,
    ) -> Self {
        Self {
            assertion_id,
            actor_id,
            audience,
            issued_at,
            not_before,
            expires_at,
            nonce,
            attributes,
            on_behalf_of_delegation,
        }
    }
}

/// Opaque, non-serializable issuance capability bound to one authenticated local
/// principal, one trust root and one currently active assertion-signing key.
/// It is not an authorization or capability grant.
pub struct IssuerSession {
    principal: PrincipalRef,
    trust_root_id: TrustRootId,
    key_id: KeyId,
    trust_state_digest: TrustStateDigest,
    session_created_at: EpochMillis,
    signing_key: SigningKey,
}

impl IssuerSession {
    pub(crate) fn from_verified_state(
        handle: EnrolledPrincipalHandle,
        snapshot: &VerifiedTrustSnapshot,
        signing_key: SigningKey,
        session_created_at: EpochMillis,
    ) -> Result<Self, IdentityError> {
        if handle.principal() != snapshot.principal()
            || handle.enrollment_id() != snapshot.enrollment_id()
            || handle.trust_root_id() != snapshot.trust_root_id()
            || handle.trust_state_generation() != snapshot.generation()
            || handle.trust_state_digest() != snapshot.trust_state_digest()
        {
            return Err(IdentityError::new(
                IdentityErrorCategory::Issuance,
                IdentityErrorCode::IssuerSessionUnavailable,
                Some("verified_handle_snapshot_mismatch"),
            ));
        }
        let active_key = snapshot.active_assertion_key().ok_or_else(|| {
            IdentityError::new(
                IdentityErrorCategory::KeyState,
                IdentityErrorCode::KeyNotActive,
                Some("assertion_signing_key"),
            )
        })?;
        if !matches!(active_key.status(), KeyStatus::Active)
            || signing_key.verifying_key().to_bytes() != active_key.public_key()
        {
            return Err(IdentityError::new(
                IdentityErrorCategory::Issuance,
                IdentityErrorCode::IssuerSessionUnavailable,
                Some("assertion_signing_key_binding"),
            ));
        }

        Ok(Self {
            principal: handle.principal(),
            trust_root_id: handle.trust_root_id(),
            key_id: active_key.key_id(),
            trust_state_digest: handle.trust_state_digest(),
            session_created_at,
            signing_key,
        })
    }

    #[must_use]
    pub const fn principal(&self) -> PrincipalRef {
        self.principal
    }

    #[must_use]
    pub const fn trust_root_id(&self) -> TrustRootId {
        self.trust_root_id
    }

    #[must_use]
    pub const fn key_id(&self) -> KeyId {
        self.key_id
    }

    #[must_use]
    pub const fn trust_state_digest(&self) -> TrustStateDigest {
        self.trust_state_digest
    }

    #[must_use]
    pub const fn session_created_at(&self) -> EpochMillis {
        self.session_created_at
    }

    pub fn issue(
        &self,
        request: AssertionIssuanceRequest,
    ) -> Result<IdentityAssertionV1, IdentityError> {
        let on_behalf_of = request
            .on_behalf_of_delegation
            .map(|delegation_id| OnBehalfOfBinding::new(self.principal.id(), delegation_id));
        let payload = IdentityAssertionPayloadV1::new(
            request.assertion_id,
            crate::AssertionIssuer::new(self.trust_root_id, self.key_id),
            self.principal.id(),
            ActorBinding::new(request.actor_id),
            on_behalf_of,
            request.audience,
            request.issued_at,
            request.not_before,
            request.expires_at,
            request.nonce,
            request.attributes,
        )?;
        let signing_input = payload.signing_input()?;
        let signature = self.signing_key.sign(&signing_input);
        payload.into_signed(AssertionSignature::from_bytes(self.key_id, signature.to_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use ecra_core::{ActorId, EpochMillis, IdentityAssertionId, PrincipalId, PrincipalRef};
    use ed25519_dalek::SigningKey;

    use super::*;
    use crate::{
        AssertionAudienceService, EnrollmentId, TrustStateDigest, VerifiedAssertionKey,
        VerifiedTrustSnapshot,
    };

    fn fixture() -> (IssuerSession, SigningKey) {
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
            signing_key.clone(),
            EpochMillis::new(1_000).unwrap(),
        )
        .unwrap();
        (session, signing_key)
    }

    #[test]
    fn issuance_sources_subject_from_session_and_binds_delegation_to_same_principal() {
        let (session, _) = fixture();
        let request = AssertionIssuanceRequest::new(
            IdentityAssertionId::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
            ActorId::parse_str("00000000-0000-0000-0000-000000000005").unwrap(),
            AssertionAudience::new(AssertionAudienceService::EcraPolicyLocal, None),
            EpochMillis::new(1_000).unwrap(),
            Some(EpochMillis::new(1_000).unwrap()),
            EpochMillis::new(2_000).unwrap(),
            None,
            AssertionAttributes::empty(),
            Some(DelegationId::parse_str("00000000-0000-0000-0000-000000000006").unwrap()),
        );
        let assertion = session.issue(request).unwrap();
        assert_eq!(
            assertion.payload().subject_principal_id(),
            session.principal().id()
        );
        assert_eq!(
            assertion.payload().on_behalf_of().unwrap().principal_id(),
            session.principal().id()
        );
    }

    #[test]
    fn issuer_session_rejects_key_not_bound_to_verified_snapshot() {
        let (session, _) = fixture();
        let _ = session;

        let trusted_key = SigningKey::from_bytes(&[7u8; 32]);
        let wrong_key = SigningKey::from_bytes(&[8u8; 32]);
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
                trusted_key.verifying_key().to_bytes(),
            )
            .unwrap()],
            BTreeSet::new(),
            TrustStateDigest::from_bytes([9u8; 32]),
        )
        .unwrap();
        let handle = EnrolledPrincipalHandle::from_verified_snapshot(&snapshot);
        let error = IssuerSession::from_verified_state(
            handle,
            &snapshot,
            wrong_key,
            EpochMillis::new(1_000).unwrap(),
        )
        .err()
        .unwrap();
        assert_eq!(error.code(), IdentityErrorCode::IssuerSessionUnavailable);
    }
}
