use std::collections::{BTreeMap, BTreeSet};

use ecra_core::{ActionAttemptId, ActionAttemptRef};

use crate::state::PreparedAttemptState;

pub(crate) fn mark_unreceipted_attempts_unresolved(
    prepared_attempts: &mut BTreeMap<ActionAttemptId, PreparedAttemptState>,
    unresolved_attempts: &mut BTreeSet<ActionAttemptId>,
) -> Option<ActionAttemptRef> {
    let mut first_unresolved = None;
    for (attempt_id, prepared) in prepared_attempts {
        if prepared.receipt().is_none() {
            prepared.mark_unresolved();
            unresolved_attempts.insert(*attempt_id);
            if first_unresolved.is_none() {
                first_unresolved = Some(prepared.attempt().clone());
            }
        }
    }
    first_unresolved
}
