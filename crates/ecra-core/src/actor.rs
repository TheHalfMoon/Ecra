use serde::{Deserialize, Serialize};

use crate::ActorId;

/// Audit attribution class. Actor identity does not prove authentication.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorKind {
    Human,
    Agent,
    System,
}

/// An attributable participant in an Ecra run.
///
/// `label` is display-only metadata. It must never be interpreted as authority,
/// authentication, email ownership, model identity or policy input.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Actor {
    id: ActorId,
    kind: ActorKind,
    label: Option<String>,
}

impl Actor {
    #[must_use]
    pub fn new(id: ActorId, kind: ActorKind, label: Option<String>) -> Self {
        Self { id, kind, label }
    }

    #[must_use]
    pub const fn id(&self) -> ActorId {
        self.id
    }

    #[must_use]
    pub const fn kind(&self) -> ActorKind {
        self.kind
    }

    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}
