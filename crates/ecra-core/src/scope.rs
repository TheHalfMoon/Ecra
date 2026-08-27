use serde::{Deserialize, Deserializer, Serialize, de};

use crate::{
    BrowserSpaceId, ContainerId, DomainError, ResourceId, SessionId, TabId, TaskId, WebOrigin,
    WorkspaceId,
};

/// Explicit scope algebra. Absence or an empty collection never means wildcard.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum ScopeConstraint<T> {
    NotApplicable,
    Exact(T),
    OneOf(Vec<T>),
    AnyExplicit,
}

impl<T> ScopeConstraint<T> {
    #[must_use]
    pub const fn not_applicable() -> Self {
        Self::NotApplicable
    }

    #[must_use]
    pub const fn exact(value: T) -> Self {
        Self::Exact(value)
    }

    pub fn one_of(values: Vec<T>) -> Result<Self, DomainError> {
        if values.is_empty() {
            return Err(DomainError::InvalidScope(
                "one_of requires at least one value".to_owned(),
            ));
        }
        Ok(Self::OneOf(values))
    }

    #[must_use]
    pub const fn any_explicit() -> Self {
        Self::AnyExplicit
    }
}

#[derive(Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case", deny_unknown_fields)]
enum ScopeConstraintWire<T> {
    NotApplicable,
    Exact(T),
    OneOf(Vec<T>),
    AnyExplicit,
}

impl<'de, T> Deserialize<'de> for ScopeConstraint<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match ScopeConstraintWire::<T>::deserialize(deserializer)? {
            ScopeConstraintWire::NotApplicable => Ok(Self::NotApplicable),
            ScopeConstraintWire::Exact(value) => Ok(Self::Exact(value)),
            ScopeConstraintWire::OneOf(values) => Self::one_of(values),
            ScopeConstraintWire::AnyExplicit => Ok(Self::AnyExplicit),
        }
        .map_err(de::Error::custom)
    }
}

/// Structured purpose metadata. Purpose text does not grant authority.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PurposeRef {
    namespace: String,
    name: String,
}

impl PurposeRef {
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let namespace = namespace.into();
        let name = name.into();
        if namespace.is_empty() || name.is_empty() {
            return Err(DomainError::InvalidScope(
                "purpose namespace and name must be non-empty".to_owned(),
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
struct PurposeRefWire {
    namespace: String,
    name: String,
}

impl<'de> Deserialize<'de> for PurposeRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = PurposeRefWire::deserialize(deserializer)?;
        Self::new(wire.namespace, wire.name).map_err(de::Error::custom)
    }
}

/// Canonical structural scope carried by requests, grants and actions.
///
/// Every security dimension is present and explicit. Start from
/// [`Scope::not_applicable`] and set only dimensions that are meaningful.
/// `any_explicit` is the only wildcard representation. ECR-003 later owns
/// intersection, subset, narrowing and authorization semantics.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scope {
    workspace: ScopeConstraint<WorkspaceId>,
    browser_space: ScopeConstraint<BrowserSpaceId>,
    container: ScopeConstraint<ContainerId>,
    tab: ScopeConstraint<TabId>,
    session: ScopeConstraint<SessionId>,
    task: ScopeConstraint<TaskId>,
    origins: ScopeConstraint<WebOrigin>,
    resources: ScopeConstraint<ResourceId>,
    purpose: Option<PurposeRef>,
}

impl Scope {
    #[must_use]
    pub const fn not_applicable() -> Self {
        Self {
            workspace: ScopeConstraint::NotApplicable,
            browser_space: ScopeConstraint::NotApplicable,
            container: ScopeConstraint::NotApplicable,
            tab: ScopeConstraint::NotApplicable,
            session: ScopeConstraint::NotApplicable,
            task: ScopeConstraint::NotApplicable,
            origins: ScopeConstraint::NotApplicable,
            resources: ScopeConstraint::NotApplicable,
            purpose: None,
        }
    }

    #[must_use]
    pub fn with_workspace(mut self, value: ScopeConstraint<WorkspaceId>) -> Self {
        self.workspace = value;
        self
    }

    #[must_use]
    pub fn with_browser_space(mut self, value: ScopeConstraint<BrowserSpaceId>) -> Self {
        self.browser_space = value;
        self
    }

    #[must_use]
    pub fn with_container(mut self, value: ScopeConstraint<ContainerId>) -> Self {
        self.container = value;
        self
    }

    #[must_use]
    pub fn with_tab(mut self, value: ScopeConstraint<TabId>) -> Self {
        self.tab = value;
        self
    }

    #[must_use]
    pub fn with_session(mut self, value: ScopeConstraint<SessionId>) -> Self {
        self.session = value;
        self
    }

    #[must_use]
    pub fn with_task(mut self, value: ScopeConstraint<TaskId>) -> Self {
        self.task = value;
        self
    }

    #[must_use]
    pub fn with_origins(mut self, value: ScopeConstraint<WebOrigin>) -> Self {
        self.origins = value;
        self
    }

    #[must_use]
    pub fn with_resources(mut self, value: ScopeConstraint<ResourceId>) -> Self {
        self.resources = value;
        self
    }

    #[must_use]
    pub fn with_purpose(mut self, value: PurposeRef) -> Self {
        self.purpose = Some(value);
        self
    }

    #[must_use]
    pub const fn workspace(&self) -> &ScopeConstraint<WorkspaceId> {
        &self.workspace
    }

    #[must_use]
    pub const fn origins(&self) -> &ScopeConstraint<WebOrigin> {
        &self.origins
    }

    #[must_use]
    pub const fn resources(&self) -> &ScopeConstraint<ResourceId> {
        &self.resources
    }

    #[must_use]
    pub const fn purpose(&self) -> Option<&PurposeRef> {
        self.purpose.as_ref()
    }
}
