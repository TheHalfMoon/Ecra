use serde::{Deserialize, Deserializer, Serialize, de};

use crate::{
    ActionDigest, ActionId, ActorId, ArtifactId, DomainError, EpochMillis, IdentityAssertionRef,
    InformationUse, OperationRef, PrincipalRef, ResourceRef, Scope, SecurityDigest, Versioned,
    to_jcs_vec,
};

const ACTION_INTENT_V1_DOMAIN: &[u8] = b"ecra/action-intent/v1\0";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationDomain {
    None,
    Local,
    External,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Reversibility {
    NotApplicable,
    Reversible,
    Conditional,
    Irreversible,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EffectProfile {
    mutation: MutationDomain,
    reversibility: Reversibility,
}

impl EffectProfile {
    pub fn new(
        mutation: MutationDomain,
        reversibility: Reversibility,
    ) -> Result<Self, DomainError> {
        let valid = match mutation {
            MutationDomain::None => reversibility == Reversibility::NotApplicable,
            MutationDomain::Local | MutationDomain::External => {
                reversibility != Reversibility::NotApplicable
            }
            MutationDomain::Unknown => reversibility == Reversibility::Unknown,
        };
        if !valid {
            return Err(DomainError::InvalidAction(
                "mutation and reversibility combination is not permitted by ECR-001 v1".to_owned(),
            ));
        }
        Ok(Self {
            mutation,
            reversibility,
        })
    }

    #[must_use]
    pub const fn mutation(self) -> MutationDomain {
        self.mutation
    }

    #[must_use]
    pub const fn reversibility(self) -> Reversibility {
        self.reversibility
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EffectProfileWire {
    mutation: MutationDomain,
    reversibility: Reversibility,
}

impl<'de> Deserialize<'de> for EffectProfile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = EffectProfileWire::deserialize(deserializer)?;
        Self::new(wire.mutation, wire.reversibility).map_err(de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdempotencyClass {
    NaturallyIdempotent,
    IdempotentWithKey,
    NonIdempotent,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IdempotencySpec {
    class: IdempotencyClass,
    key_ref: Option<String>,
}

impl IdempotencySpec {
    pub fn new(class: IdempotencyClass, key_ref: Option<String>) -> Result<Self, DomainError> {
        match class {
            IdempotencyClass::NaturallyIdempotent
            | IdempotencyClass::NonIdempotent
            | IdempotencyClass::Unknown => {
                if key_ref.is_some() {
                    return Err(DomainError::InvalidAction(
                        "this idempotency class must not carry key_ref".to_owned(),
                    ));
                }
            }
            IdempotencyClass::IdempotentWithKey => {
                if key_ref.as_ref().is_none_or(String::is_empty) {
                    return Err(DomainError::InvalidAction(
                        "idempotent_with_key requires a non-empty key_ref".to_owned(),
                    ));
                }
            }
        }
        Ok(Self { class, key_ref })
    }

    #[must_use]
    pub const fn class(&self) -> IdempotencyClass {
        self.class
    }

    #[must_use]
    pub fn key_ref(&self) -> Option<&str> {
        self.key_ref.as_deref()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct IdempotencySpecWire {
    class: IdempotencyClass,
    key_ref: Option<String>,
}

impl<'de> Deserialize<'de> for IdempotencySpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = IdempotencySpecWire::deserialize(deserializer)?;
        Self::new(wire.class, wire.key_ref).map_err(de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetryClass {
    Safe,
    RequiresSameIdempotencyKey,
    RequiresExternalReconciliation,
    NeverBlindRetry,
}

fn validate_retry(
    effect: EffectProfile,
    idempotency: &IdempotencySpec,
    retry: RetryClass,
) -> Result<(), DomainError> {
    let allowed = match retry {
        RetryClass::Safe => {
            idempotency.class() == IdempotencyClass::NaturallyIdempotent
                && effect.mutation() != MutationDomain::Unknown
        }
        RetryClass::RequiresSameIdempotencyKey => {
            idempotency.class() == IdempotencyClass::IdempotentWithKey
                && effect.mutation() != MutationDomain::Unknown
        }
        RetryClass::RequiresExternalReconciliation => {
            matches!(
                effect.mutation(),
                MutationDomain::External | MutationDomain::Unknown
            )
        }
        RetryClass::NeverBlindRetry => true,
    };

    if !allowed {
        return Err(DomainError::InvalidAction(
            "effect, idempotency and retry combination is not permitted by ECR-001 v1".to_owned(),
        ));
    }
    Ok(())
}

/// Validated grouping of the three orthogonal action safety axes.
///
/// This helper exists only to keep construction explicit and Clippy-clean; its
/// fields remain separately serialized in [`ActionIntent`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ActionSemantics {
    effect: EffectProfile,
    idempotency: IdempotencySpec,
    retry: RetryClass,
}

impl ActionSemantics {
    pub fn new(
        effect: EffectProfile,
        idempotency: IdempotencySpec,
        retry: RetryClass,
    ) -> Result<Self, DomainError> {
        validate_retry(effect, &idempotency, retry)?;
        Ok(Self {
            effect,
            idempotency,
            retry,
        })
    }

    #[must_use]
    pub const fn effect(&self) -> EffectProfile {
        self.effect
    }

    #[must_use]
    pub const fn idempotency(&self) -> &IdempotencySpec {
        &self.idempotency
    }

    #[must_use]
    pub const fn retry(&self) -> RetryClass {
        self.retry
    }
}

/// Exact parameter binding for an action intent. References are location only;
/// every non-empty parameter set carries a strong security digest.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ActionParametersRef {
    None,
    BoundArtifact {
        artifact: ArtifactId,
        binding_digest: SecurityDigest,
    },
    BoundExternal {
        external_ref: String,
        binding_digest: SecurityDigest,
    },
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum ActionParametersRefWire {
    None,
    BoundArtifact {
        artifact: ArtifactId,
        binding_digest: SecurityDigest,
    },
    BoundExternal {
        external_ref: String,
        binding_digest: SecurityDigest,
    },
}

impl<'de> Deserialize<'de> for ActionParametersRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match ActionParametersRefWire::deserialize(deserializer)? {
            ActionParametersRefWire::None => Ok(Self::None),
            ActionParametersRefWire::BoundArtifact {
                artifact,
                binding_digest,
            } => Ok(Self::BoundArtifact {
                artifact,
                binding_digest,
            }),
            ActionParametersRefWire::BoundExternal {
                external_ref,
                binding_digest,
            } if !external_ref.is_empty() => Ok(Self::BoundExternal {
                external_ref,
                binding_digest,
            }),
            ActionParametersRefWire::BoundExternal { .. } => Err(de::Error::custom(
                "bound_external action parameters require non-empty external_ref",
            )),
        }
    }
}

/// Opaque address of an action parameter for information lineage only.
/// The path is descriptive metadata and never authority or provider syntax.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActionParameterRef {
    action: ActionId,
    path: String,
}

impl ActionParameterRef {
    pub fn new(action: ActionId, path: impl Into<String>) -> Result<Self, DomainError> {
        let path = path.into();
        if path.is_empty() {
            return Err(DomainError::InvalidAction(
                "action parameter path must be non-empty".to_owned(),
            ));
        }
        Ok(Self { action, path })
    }

    #[must_use]
    pub const fn action(&self) -> ActionId {
        self.action
    }

    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ActionParameterRefWire {
    action: ActionId,
    path: String,
}

impl<'de> Deserialize<'de> for ActionParameterRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ActionParameterRefWire::deserialize(deserializer)?;
        Self::new(wire.action, wire.path).map_err(de::Error::custom)
    }
}

/// Proposed action before authorization or execution.
///
/// The operation names intent only; it is not a capability grant. ECR-003 owns
/// authorization and information-flow policy, while ECR-002 owns execution.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActionIntent {
    id: ActionId,
    actor: ActorId,
    principal: Option<PrincipalRef>,
    identity_assertion: Option<IdentityAssertionRef>,
    operation: OperationRef,
    target: ResourceRef,
    scope: Scope,
    parameters: ActionParametersRef,
    information_use: Vec<InformationUse>,
    effect: EffectProfile,
    idempotency: IdempotencySpec,
    retry: RetryClass,
    created_at: Option<EpochMillis>,
    correlation_id: Option<String>,
}

impl ActionIntent {
    #[must_use]
    pub fn new(
        id: ActionId,
        actor: ActorId,
        operation: OperationRef,
        target: ResourceRef,
        scope: Scope,
        parameters: ActionParametersRef,
        semantics: ActionSemantics,
    ) -> Self {
        Self {
            id,
            actor,
            principal: None,
            identity_assertion: None,
            operation,
            target,
            scope,
            parameters,
            information_use: Vec::new(),
            effect: semantics.effect,
            idempotency: semantics.idempotency,
            retry: semantics.retry,
            created_at: None,
            correlation_id: None,
        }
    }

    pub fn with_principal(mut self, principal: PrincipalRef) -> Result<Self, DomainError> {
        if self
            .identity_assertion
            .is_some_and(|assertion| assertion.principal() != principal.id())
        {
            return Err(DomainError::InvalidAction(
                "identity assertion principal does not match action principal".to_owned(),
            ));
        }
        self.principal = Some(principal);
        Ok(self)
    }

    pub fn with_identity_assertion(
        mut self,
        assertion: IdentityAssertionRef,
    ) -> Result<Self, DomainError> {
        if self
            .principal
            .is_some_and(|principal| assertion.principal() != principal.id())
        {
            return Err(DomainError::InvalidAction(
                "identity assertion principal does not match action principal".to_owned(),
            ));
        }
        self.identity_assertion = Some(assertion);
        Ok(self)
    }

    #[must_use]
    pub fn with_information_use(mut self, information_use: Vec<InformationUse>) -> Self {
        self.information_use = information_use;
        self
    }

    #[must_use]
    pub fn with_created_at(mut self, created_at: EpochMillis) -> Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn with_correlation_id(
        mut self,
        correlation_id: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let correlation_id = correlation_id.into();
        if correlation_id.is_empty() {
            return Err(DomainError::InvalidAction(
                "action correlation_id must be non-empty".to_owned(),
            ));
        }
        self.correlation_id = Some(correlation_id);
        Ok(self)
    }

    #[must_use]
    pub const fn id(&self) -> ActionId {
        self.id
    }

    #[must_use]
    pub const fn effect(&self) -> EffectProfile {
        self.effect
    }

    #[must_use]
    pub const fn idempotency(&self) -> &IdempotencySpec {
        &self.idempotency
    }

    #[must_use]
    pub const fn retry(&self) -> RetryClass {
        self.retry
    }

    pub fn digest(&self) -> Result<ActionDigest, DomainError> {
        let canonical = to_jcs_vec(&Versioned::v1(self))?;
        let mut binding = Vec::with_capacity(ACTION_INTENT_V1_DOMAIN.len() + canonical.len());
        binding.extend_from_slice(ACTION_INTENT_V1_DOMAIN);
        binding.extend_from_slice(&canonical);
        Ok(ActionDigest::new(SecurityDigest::sha256(&binding)))
    }

    pub fn action_ref(&self) -> Result<ActionRef, DomainError> {
        Ok(ActionRef::new(self.id, self.digest()?))
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ActionIntentWire {
    id: ActionId,
    actor: ActorId,
    principal: Option<PrincipalRef>,
    identity_assertion: Option<IdentityAssertionRef>,
    operation: OperationRef,
    target: ResourceRef,
    scope: Scope,
    parameters: ActionParametersRef,
    information_use: Vec<InformationUse>,
    effect: EffectProfile,
    idempotency: IdempotencySpec,
    retry: RetryClass,
    created_at: Option<EpochMillis>,
    correlation_id: Option<String>,
}

impl<'de> Deserialize<'de> for ActionIntent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ActionIntentWire::deserialize(deserializer)?;
        let semantics = ActionSemantics::new(wire.effect, wire.idempotency, wire.retry)
            .map_err(de::Error::custom)?;
        let mut value = Self::new(
            wire.id,
            wire.actor,
            wire.operation,
            wire.target,
            wire.scope,
            wire.parameters,
            semantics,
        );
        if let Some(principal) = wire.principal {
            value = value.with_principal(principal).map_err(de::Error::custom)?;
        }
        if let Some(assertion) = wire.identity_assertion {
            value = value
                .with_identity_assertion(assertion)
                .map_err(de::Error::custom)?;
        }
        value = value.with_information_use(wire.information_use);
        if let Some(created_at) = wire.created_at {
            value = value.with_created_at(created_at);
        }
        if let Some(correlation_id) = wire.correlation_id {
            value = value
                .with_correlation_id(correlation_id)
                .map_err(de::Error::custom)?;
        }
        Ok(value)
    }
}

/// Immutable action identity used by approvals, attempts, receipts and audit.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionRef {
    id: ActionId,
    digest: ActionDigest,
}

impl ActionRef {
    #[must_use]
    pub const fn new(id: ActionId, digest: ActionDigest) -> Self {
        Self { id, digest }
    }

    #[must_use]
    pub const fn id(&self) -> ActionId {
        self.id
    }

    #[must_use]
    pub const fn digest(&self) -> &ActionDigest {
        &self.digest
    }

    pub fn validate_for(&self, intent: &ActionIntent) -> Result<(), DomainError> {
        if self.id != intent.id() || self.digest != intent.digest()? {
            return Err(DomainError::InvalidAction(
                "action reference does not bind the exact action intent".to_owned(),
            ));
        }
        Ok(())
    }
}
