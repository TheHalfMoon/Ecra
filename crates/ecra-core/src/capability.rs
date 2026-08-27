use serde::{Deserialize, Deserializer, Serialize, de};

use crate::{
    ActorId, CapabilityGrantId, CapabilityRequestId, DomainError, EvaluationContext,
    IdentityAssertionRef, PrincipalRef, ResourceRef, Scope, TemporalValidity,
};

/// Provider-neutral operation identity such as `browser/read`.
///
/// The strings are names only; they are never parsed as Cedar, MCP, Firefox or
/// model-provider policy expressions.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OperationRef {
    namespace: String,
    name: String,
}

impl OperationRef {
    pub fn new(namespace: impl Into<String>, name: impl Into<String>) -> Result<Self, DomainError> {
        let namespace = namespace.into();
        let name = name.into();
        if namespace.is_empty() || name.is_empty() {
            return Err(DomainError::InvalidCapability(
                "operation namespace and name must be non-empty".to_owned(),
            ));
        }
        Ok(Self { namespace, name })
    }

    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OperationRefWire {
    namespace: String,
    name: String,
}

impl<'de> Deserialize<'de> for OperationRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = OperationRefWire::deserialize(deserializer)?;
        Self::new(wire.namespace, wire.name).map_err(de::Error::custom)
    }
}

/// Structural provenance for a delegated capability grant.
///
/// This records ancestry only. It does not prove that the child is a valid
/// subset of the parent, that the parent exists, or that either grant is live.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DelegationRef {
    parent_grant: CapabilityGrantId,
    depth: u16,
}

impl DelegationRef {
    pub fn new(parent_grant: CapabilityGrantId, depth: u16) -> Result<Self, DomainError> {
        if depth == 0 {
            return Err(DomainError::InvalidCapability(
                "delegation depth must be greater than zero".to_owned(),
            ));
        }
        Ok(Self {
            parent_grant,
            depth,
        })
    }

    #[must_use]
    pub const fn parent_grant(self) -> CapabilityGrantId {
        self.parent_grant
    }

    #[must_use]
    pub const fn depth(self) -> u16 {
        self.depth
    }
}

/// A request for authority. A request is never itself authority.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityRequest {
    id: CapabilityRequestId,
    principal: PrincipalRef,
    operation: OperationRef,
    target: ResourceRef,
    scope: Scope,
    temporal: Option<TemporalValidity>,
    requested_by: ActorId,
    identity_assertion: Option<IdentityAssertionRef>,
    reason: Option<String>,
}

impl CapabilityRequest {
    #[must_use]
    pub fn new(
        id: CapabilityRequestId,
        principal: PrincipalRef,
        operation: OperationRef,
        target: ResourceRef,
        scope: Scope,
        requested_by: ActorId,
    ) -> Self {
        Self {
            id,
            principal,
            operation,
            target,
            scope,
            temporal: None,
            requested_by,
            identity_assertion: None,
            reason: None,
        }
    }

    #[must_use]
    pub fn with_temporal(mut self, temporal: TemporalValidity) -> Self {
        self.temporal = Some(temporal);
        self
    }

    pub fn with_identity_assertion(
        mut self,
        assertion: IdentityAssertionRef,
    ) -> Result<Self, DomainError> {
        if assertion.principal() != self.principal.id() {
            return Err(DomainError::InvalidCapability(
                "identity assertion principal does not match requested principal".to_owned(),
            ));
        }
        self.identity_assertion = Some(assertion);
        Ok(self)
    }

    #[must_use]
    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    #[must_use]
    pub const fn id(&self) -> CapabilityRequestId {
        self.id
    }

    #[must_use]
    pub const fn principal(&self) -> PrincipalRef {
        self.principal
    }

    #[must_use]
    pub const fn temporal(&self) -> Option<TemporalValidity> {
        self.temporal
    }

    #[must_use]
    pub fn is_temporally_valid_at(&self, context: EvaluationContext) -> bool {
        self.temporal.is_none_or(|validity| validity.contains(context))
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CapabilityRequestWire {
    id: CapabilityRequestId,
    principal: PrincipalRef,
    operation: OperationRef,
    target: ResourceRef,
    scope: Scope,
    temporal: Option<TemporalValidity>,
    requested_by: ActorId,
    identity_assertion: Option<IdentityAssertionRef>,
    reason: Option<String>,
}

impl<'de> Deserialize<'de> for CapabilityRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = CapabilityRequestWire::deserialize(deserializer)?;
        let mut request = Self::new(
            wire.id,
            wire.principal,
            wire.operation,
            wire.target,
            wire.scope,
            wire.requested_by,
        );
        if let Some(temporal) = wire.temporal {
            request = request.with_temporal(temporal);
        }
        if let Some(assertion) = wire.identity_assertion {
            request = request
                .with_identity_assertion(assertion)
                .map_err(de::Error::custom)?;
        }
        if let Some(reason) = wire.reason {
            request = request.with_reason(reason);
        }
        Ok(request)
    }
}

/// A structurally represented authority grant.
///
/// ECR-001 validates only its shape. Authorization, revocation, parent
/// existence and subset/narrowing semantics belong to ECR-003/ECR-031.
///
/// A request cannot be converted into a grant by type conversion:
///
/// ```compile_fail
/// use ecra_core::{CapabilityGrant, CapabilityRequest};
///
/// fn convert(request: CapabilityRequest) -> CapabilityGrant {
///     request.into()
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityGrant {
    id: CapabilityGrantId,
    principal: PrincipalRef,
    operation: OperationRef,
    target: ResourceRef,
    scope: Scope,
    temporal: Option<TemporalValidity>,
    issued_by: ActorId,
    parent_grant: Option<CapabilityGrantId>,
    delegation_depth: Option<u16>,
}

impl CapabilityGrant {
    #[must_use]
    pub fn new(
        id: CapabilityGrantId,
        principal: PrincipalRef,
        operation: OperationRef,
        target: ResourceRef,
        scope: Scope,
        issued_by: ActorId,
        delegation: Option<DelegationRef>,
    ) -> Self {
        let (parent_grant, delegation_depth) = match delegation {
            Some(value) => (Some(value.parent_grant()), Some(value.depth())),
            None => (None, None),
        };
        Self {
            id,
            principal,
            operation,
            target,
            scope,
            temporal: None,
            issued_by,
            parent_grant,
            delegation_depth,
        }
    }

    #[must_use]
    pub fn with_temporal(mut self, temporal: TemporalValidity) -> Self {
        self.temporal = Some(temporal);
        self
    }

    #[must_use]
    pub const fn id(&self) -> CapabilityGrantId {
        self.id
    }

    #[must_use]
    pub const fn principal(&self) -> PrincipalRef {
        self.principal
    }

    #[must_use]
    pub const fn temporal(&self) -> Option<TemporalValidity> {
        self.temporal
    }

    #[must_use]
    pub fn delegation(&self) -> Option<DelegationRef> {
        match (self.parent_grant, self.delegation_depth) {
            (Some(parent), Some(depth)) => DelegationRef::new(parent, depth).ok(),
            _ => None,
        }
    }

    #[must_use]
    pub fn is_temporally_valid_at(&self, context: EvaluationContext) -> bool {
        self.temporal.is_none_or(|validity| validity.contains(context))
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CapabilityGrantWire {
    id: CapabilityGrantId,
    principal: PrincipalRef,
    operation: OperationRef,
    target: ResourceRef,
    scope: Scope,
    temporal: Option<TemporalValidity>,
    issued_by: ActorId,
    parent_grant: Option<CapabilityGrantId>,
    delegation_depth: Option<u16>,
}

impl<'de> Deserialize<'de> for CapabilityGrant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = CapabilityGrantWire::deserialize(deserializer)?;
        let delegation = match (wire.parent_grant, wire.delegation_depth) {
            (None, None) => None,
            (Some(parent), Some(depth)) => {
                Some(DelegationRef::new(parent, depth).map_err(de::Error::custom)?)
            }
            _ => {
                return Err(de::Error::custom(
                    "parent_grant and delegation_depth must appear together",
                ));
            }
        };
        let mut grant = Self::new(
            wire.id,
            wire.principal,
            wire.operation,
            wire.target,
            wire.scope,
            wire.issued_by,
            delegation,
        );
        if let Some(temporal) = wire.temporal {
            grant = grant.with_temporal(temporal);
        }
        Ok(grant)
    }
}
