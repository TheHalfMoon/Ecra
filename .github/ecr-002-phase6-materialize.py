from pathlib import Path


def replace_once(text: str, old: str, new: str, label: str) -> str:
    if text.count(old) != 1:
        raise SystemExit(f"{label} replacement count={text.count(old)}")
    return text.replace(old, new, 1)


recovery = Path("crates/ecra-run/src/recovery.rs")
recovery.write_text(r'''use std::collections::{BTreeMap, BTreeSet};

use ecra_core::{
    ActionAttemptId, ActionAttemptRef, ActionIntent, IdempotencyClass, RetryClass, RunId,
};

use crate::state::PreparedAttemptState;
use crate::{
    EventSequence, LedgerDigest, RunError, RunErrorCategory, RunErrorCode, RunPhase, RunState,
};

/// Proof returned by the store only after `attempt_prepared` commits durably.
///
/// This value does not authorize provider execution. A later owning executor may
/// use it only as evidence that the exact attempt is present in durable run truth.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreparedAttemptGuard {
    run_id: RunId,
    attempt: ActionAttemptRef,
    committed_sequence: EventSequence,
    committed_digest: LedgerDigest,
}

impl PreparedAttemptGuard {
    pub(crate) fn new(
        run_id: RunId,
        attempt: ActionAttemptRef,
        committed_sequence: EventSequence,
        committed_digest: LedgerDigest,
    ) -> Self {
        Self {
            run_id,
            attempt,
            committed_sequence,
            committed_digest,
        }
    }

    #[must_use]
    pub const fn run_id(&self) -> RunId {
        self.run_id
    }

    #[must_use]
    pub fn attempt(&self) -> &ActionAttemptRef {
        &self.attempt
    }

    #[must_use]
    pub const fn committed_sequence(&self) -> EventSequence {
        self.committed_sequence
    }

    #[must_use]
    pub fn committed_digest(&self) -> &LedgerDigest {
        &self.committed_digest
    }
}

/// Recovery result after an explicit recovery boundary is durably appended.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecoveryResult {
    unreceipted_attempts: Vec<ActionAttemptRef>,
    state: RunState,
}

impl RecoveryResult {
    pub(crate) fn new(unreceipted_attempts: Vec<ActionAttemptRef>, state: RunState) -> Self {
        Self {
            unreceipted_attempts,
            state,
        }
    }

    #[must_use]
    pub fn unreceipted_attempts(&self) -> &[ActionAttemptRef] {
        &self.unreceipted_attempts
    }

    #[must_use]
    pub const fn state(&self) -> &RunState {
        &self.state
    }
}

pub(crate) fn scan_unreceipted_attempts(state: &RunState) -> Vec<ActionAttemptRef> {
    state
        .prepared_attempts()
        .values()
        .filter(|prepared| prepared.receipt().is_none())
        .map(|prepared| prepared.attempt().clone())
        .collect()
}

/// Refuse blind retries unless the exact ECR-001 action semantics permit them.
///
/// This is a runtime safety guard, not an authorization or reconciliation
/// decision. Exact action binding ensures `requires_same_idempotency_key` is
/// evaluated against the same `ActionIntent`, including its key reference.
pub fn ensure_retry_allowed(
    state: &RunState,
    intent: &ActionIntent,
    prior_attempt: &ActionAttemptRef,
) -> Result<(), RunError> {
    prior_attempt.validate_for(intent).map_err(|_| {
        attempt_error(
            RunErrorCode::AttemptBindingMismatch,
            "retry candidate does not bind the exact ActionIntent",
        )
    })?;

    let prepared = state
        .prepared_attempts()
        .get(&prior_attempt.id())
        .ok_or_else(|| {
            attempt_error(
                RunErrorCode::AttemptBindingMismatch,
                "retry candidate has no matching prepared attempt",
            )
        })?;
    if prepared.attempt() != prior_attempt {
        return Err(attempt_error(
            RunErrorCode::AttemptBindingMismatch,
            "retry candidate conflicts with the durable attempt binding",
        ));
    }

    if state.phase() != RunPhase::Running {
        return Err(blind_retry_error(
            "blind retry requires a running run phase",
        ));
    }
    if state.has_hard_budget_blocker() {
        return Err(RunError::new(
            RunErrorCategory::Budget,
            RunErrorCode::BudgetExhausted,
            "blind retry is blocked by an exhausted hard budget",
        ));
    }
    if prepared.receipt().is_none()
        || prepared.unresolved()
        || state.unresolved_attempts().contains(&prior_attempt.id())
    {
        return Err(blind_retry_error(
            "blind retry is forbidden while the prior attempt is unreceipted or unresolved",
        ));
    }

    if matches!(
        intent.idempotency().class(),
        IdempotencyClass::NonIdempotent | IdempotencyClass::Unknown
    ) {
        return Err(blind_retry_error(
            "blind retry is forbidden for non-idempotent or unknown idempotency semantics",
        ));
    }

    match intent.retry() {
        RetryClass::Safe | RetryClass::RequiresSameIdempotencyKey => Ok(()),
        RetryClass::RequiresExternalReconciliation | RetryClass::NeverBlindRetry => Err(
            blind_retry_error("ECR-001 retry class requires reconciliation or forbids blind retry"),
        ),
    }
}

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

fn blind_retry_error(message: impl Into<String>) -> RunError {
    RunError::new(
        RunErrorCategory::Recovery,
        RunErrorCode::BlindRetryForbidden,
        message,
    )
}

fn attempt_error(code: RunErrorCode, message: impl Into<String>) -> RunError {
    RunError::new(RunErrorCategory::Attempt, code, message)
}
''')

lib = Path("crates/ecra-run/src/lib.rs")
text = lib.read_text()
anchor = "pub use migration::ECR_RUN_SCHEMA_VERSION;\n"
text = replace_once(
    text,
    anchor,
    anchor + "pub use recovery::{PreparedAttemptGuard, RecoveryResult, ensure_retry_allowed};\n",
    "lib recovery export",
)
lib.write_text(text)

store = Path("crates/ecra-run/src/store.rs")
text = store.read_text()
text = replace_once(
    text,
    "use ecra_core::{ContentDigest, RunId, to_jcs_vec};\n",
    "use ecra_core::{ActionAttemptRef, ActionReceipt, ContentDigest, EpochMillis, RunId, to_jcs_vec};\n",
    "store ecra_core import",
)
text = replace_once(
    text,
    '''use crate::{\n    BudgetAmount, EventSequence, LedgerDigest, RunError, RunErrorCategory, RunErrorCode,\n    RunEventEnvelope, RunPhase, RunReducer, RunState, SqliteConfiguration,\n};\n''',
    '''use crate::{\n    BudgetAmount, EventSequence, LedgerDigest, PreparedAttemptGuard, RecoveryReason, RecoveryResult,\n    RunError, RunErrorCategory, RunErrorCode, RunEvent, RunEventEnvelope, RunPhase, RunReducer,\n    RunState, SqliteConfiguration,\n};\n''',
    "store crate imports",
)
anchor = '''    pub fn sqlite_configuration(&self) -> Result<SqliteConfiguration, RunError> {\n        read_configuration(&self.connection)\n    }\n\n'''
methods = r'''    pub fn prepare_attempt(
        &mut self,
        run_id: RunId,
        expected: &ExpectedRunHead,
        attempt: ActionAttemptRef,
        recorded_at: EpochMillis,
    ) -> Result<PreparedAttemptGuard, RunError> {
        let envelope = successor_envelope(
            run_id,
            expected,
            recorded_at,
            RunEvent::AttemptPrepared {
                attempt: attempt.clone(),
            },
        )?;
        self.append(expected, &envelope)?;
        Ok(PreparedAttemptGuard::new(
            run_id,
            attempt,
            envelope.sequence(),
            envelope.event_digest().clone(),
        ))
    }

    pub fn record_receipt(
        &mut self,
        run_id: RunId,
        expected: &ExpectedRunHead,
        receipt: ActionReceipt,
        recorded_at: EpochMillis,
    ) -> Result<RunState, RunError> {
        let envelope = successor_envelope(
            run_id,
            expected,
            recorded_at,
            RunEvent::ReceiptRecorded { receipt },
        )?;
        self.append(expected, &envelope)
    }

    pub fn recover(
        &mut self,
        run_id: RunId,
        expected: &ExpectedRunHead,
        reason: RecoveryReason,
        recorded_at: EpochMillis,
    ) -> Result<RecoveryResult, RunError> {
        let current = self.load_state(run_id)?.ok_or_else(|| {
            RunError::new(
                RunErrorCategory::Recovery,
                RunErrorCode::RecoveryRequired,
                "recovery requires an existing durable run",
            )
        })?;
        let unreceipted_attempts = crate::recovery::scan_unreceipted_attempts(&current);
        let envelope = successor_envelope(
            run_id,
            expected,
            recorded_at,
            RunEvent::RecoveryBoundary { reason },
        )?;
        let state = self.append(expected, &envelope)?;
        Ok(RecoveryResult::new(unreceipted_attempts, state))
    }

'''
text = replace_once(text, anchor, anchor + methods, "store phase6 methods")
text = replace_once(
    text,
    "\nfn authoritative_head(\n",
    r'''
fn successor_envelope(
    run_id: RunId,
    expected: &ExpectedRunHead,
    recorded_at: EpochMillis,
    event: RunEvent,
) -> Result<RunEventEnvelope, RunError> {
    let ExpectedRunHead::At { sequence, digest } = expected else {
        return Err(RunError::new(
            RunErrorCategory::Ledger,
            RunErrorCode::LedgerHeadMismatch,
            "non-genesis store operation requires an existing expected run head",
        ));
    };
    RunEventEnvelope::new(
        run_id,
        sequence.checked_next()?,
        recorded_at,
        Some(digest.clone()),
        event,
    )
}

fn authoritative_head(
''',
    "store successor helper",
)
store.write_text(text)

attempts = Path("crates/ecra-run/tests/attempts.rs")
text = attempts.read_text()
text = replace_once(
    text,
    "use ecra_core::{ActionAttemptRef, ActionReceipt, EpochMillis};\n",
    '''use ecra_core::{\n    ActionAttemptId, ActionAttemptRef, ActionIntent, ActionOutcome, ActionReceipt, ActorId,\n    EpochMillis, ReceiptId,\n};\n''',
    "attempts core imports",
)
text = replace_once(
    text,
    "use ecra_run::{RunErrorCode, RunEvent, RunEventEnvelope, RunPhase, RunReducer, SuspensionReason};\n",
    '''use ecra_run::{\n    RunErrorCode, RunEvent, RunEventEnvelope, RunPhase, RunReducer, SuspensionReason,\n    ensure_retry_allowed,\n};\n''',
    "attempts run imports",
)
text += r'''

fn intent_with_semantics(
    retry: &str,
    idempotency: serde_json::Value,
    effect: serde_json::Value,
) -> ActionIntent {
    let mut value: serde_json::Value = serde_json::from_str(include_str!(
        "../../../contracts/ecra-domain-v1/valid/action-digest-golden.json"
    ))
    .expect("golden action intent JSON");
    value["retry"] = serde_json::Value::String(retry.to_owned());
    value["idempotency"] = idempotency;
    value["effect"] = effect;
    serde_json::from_value(value).expect("valid ECR-001 retry fixture")
}

fn attempt_for_intent(intent: &ActionIntent, id: &str) -> ActionAttemptRef {
    let id: ActionAttemptId = serde_json::from_str(&format!("\"{id}\""))
        .expect("attempt id");
    ActionAttemptRef::new(id, intent.action_ref().expect("action ref"))
}

fn receipt_for_attempt(attempt: ActionAttemptRef, id: &str) -> ActionReceipt {
    let receipt_id: ReceiptId = serde_json::from_str(&format!("\"{id}\""))
        .expect("receipt id");
    let actor: ActorId = serde_json::from_str("\"00000000-0000-0000-0000-000000000001\"")
        .expect("actor id");
    ActionReceipt::new(
        receipt_id,
        attempt,
        actor,
        ActionOutcome::ExecutorObservedSuccess,
    )
}

fn state_with_received_attempt(
    intent: &ActionIntent,
    attempt_id: &str,
    receipt_id: &str,
) -> (ecra_run::RunState, ActionAttemptRef) {
    let attempt = attempt_for_intent(intent, attempt_id);
    let receipt = receipt_for_attempt(attempt.clone(), receipt_id);
    let mut history = running();
    push(
        &mut history,
        RunEvent::AttemptPrepared {
            attempt: attempt.clone(),
        },
    );
    push(&mut history, RunEvent::ReceiptRecorded { receipt });
    (
        RunReducer::reduce(&history).expect("received attempt state"),
        attempt,
    )
}

#[test]
fn retry_guard_preserves_all_ecr001_retry_classes() {
    let none_effect = serde_json::json!({"mutation":"none","reversibility":"not_applicable"});
    let safe = intent_with_semantics(
        "safe",
        serde_json::json!({"class":"naturally_idempotent","key_ref":null}),
        none_effect.clone(),
    );
    let (safe_state, safe_attempt) = state_with_received_attempt(
        &safe,
        "00000000-0000-0000-0000-000000000410",
        "00000000-0000-0000-0000-000000000510",
    );
    ensure_retry_allowed(&safe_state, &safe, &safe_attempt).expect("safe retry allowed");

    let keyed = intent_with_semantics(
        "requires_same_idempotency_key",
        serde_json::json!({"class":"idempotent_with_key","key_ref":"phase6-key"}),
        serde_json::json!({"mutation":"local","reversibility":"reversible"}),
    );
    let (keyed_state, keyed_attempt) = state_with_received_attempt(
        &keyed,
        "00000000-0000-0000-0000-000000000411",
        "00000000-0000-0000-0000-000000000511",
    );
    ensure_retry_allowed(&keyed_state, &keyed, &keyed_attempt)
        .expect("same-key retry allowed for exact bound intent");

    let reconcile = intent_with_semantics(
        "requires_external_reconciliation",
        serde_json::json!({"class":"naturally_idempotent","key_ref":null}),
        serde_json::json!({"mutation":"external","reversibility":"reversible"}),
    );
    let (reconcile_state, reconcile_attempt) = state_with_received_attempt(
        &reconcile,
        "00000000-0000-0000-0000-000000000412",
        "00000000-0000-0000-0000-000000000512",
    );
    let error = ensure_retry_allowed(&reconcile_state, &reconcile, &reconcile_attempt)
        .expect_err("reconciliation retry class must not blind retry");
    assert_eq!(error.code(), RunErrorCode::BlindRetryForbidden);

    let never = intent_with_semantics(
        "never_blind_retry",
        serde_json::json!({"class":"naturally_idempotent","key_ref":null}),
        none_effect,
    );
    let (never_state, never_attempt) = state_with_received_attempt(
        &never,
        "00000000-0000-0000-0000-000000000413",
        "00000000-0000-0000-0000-000000000513",
    );
    let error = ensure_retry_allowed(&never_state, &never, &never_attempt)
        .expect_err("never-blind retry class must be refused");
    assert_eq!(error.code(), RunErrorCode::BlindRetryForbidden);
}

#[test]
fn unresolved_attempt_blocks_blind_retry_even_for_naturally_idempotent_safe_action() {
    let intent = intent_with_semantics(
        "safe",
        serde_json::json!({"class":"naturally_idempotent","key_ref":null}),
        serde_json::json!({"mutation":"none","reversibility":"not_applicable"}),
    );
    let attempt = attempt_for_intent(&intent, "00000000-0000-0000-0000-000000000414");
    let mut history = running();
    push(
        &mut history,
        RunEvent::AttemptPrepared {
            attempt: attempt.clone(),
        },
    );
    push(&mut history, event("recovery_boundary"));
    let state = RunReducer::reduce(&history).expect("recovered unresolved state");
    let error = ensure_retry_allowed(&state, &intent, &attempt)
        .expect_err("unresolved attempt must block blind retry");
    assert_eq!(error.code(), RunErrorCode::BlindRetryForbidden);
}

#[test]
fn multiple_attempts_for_one_action_keep_receipts_isolated() {
    let intent = intent_with_semantics(
        "safe",
        serde_json::json!({"class":"naturally_idempotent","key_ref":null}),
        serde_json::json!({"mutation":"none","reversibility":"not_applicable"}),
    );
    let first = attempt_for_intent(&intent, "00000000-0000-0000-0000-000000000415");
    let second = attempt_for_intent(&intent, "00000000-0000-0000-0000-000000000416");
    let first_receipt = receipt_for_attempt(
        first.clone(),
        "00000000-0000-0000-0000-000000000515",
    );

    let mut history = running();
    push(
        &mut history,
        RunEvent::AttemptPrepared {
            attempt: first.clone(),
        },
    );
    push(
        &mut history,
        RunEvent::AttemptPrepared {
            attempt: second.clone(),
        },
    );
    push(
        &mut history,
        RunEvent::ReceiptRecorded {
            receipt: first_receipt.clone(),
        },
    );
    let state = RunReducer::reduce(&history).expect("two-attempt state");
    assert_eq!(
        state.prepared_attempts().get(&first.id()).unwrap().receipt(),
        Some(&first_receipt)
    );
    assert!(
        state
            .prepared_attempts()
            .get(&second.id())
            .unwrap()
            .receipt()
            .is_none()
    );
}
'''
attempts.write_text(text)

sqlite = Path("crates/ecra-run/tests/sqlite_store.rs")
text = sqlite.read_text()
text = replace_once(
    text,
    "use std::path::Path;\n" if text.startswith("use std::path::Path;\n") else "use ecra_core::{ContentDigest, EpochMillis, RunId, to_jcs_vec};\n",
    "use std::path::Path;\nuse std::sync::{Arc, Barrier};\nuse std::thread;\n" if text.startswith("use std::path::Path;\n") else "use std::sync::{Arc, Barrier};\nuse std::thread;\n\nuse ecra_core::{ContentDigest, EpochMillis, RunId, to_jcs_vec};\n",
    "sqlite_store concurrency imports",
)
text += r'''

#[test]
fn attempt_guard_and_receipt_store_apis_commit_exact_bound_truth() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("attempt-guard.db");
    let mut store = RunStore::open(&path).expect("open store");
    let created = genesis();
    store
        .append(&ExpectedRunHead::Genesis, &created)
        .expect("append genesis");
    let started = successor(&created, fixture_event("run_started"));
    store
        .append(&expected(&created), &started)
        .expect("append started");

    let attempt = match fixture_event("attempt_prepared") {
        RunEvent::AttemptPrepared { attempt } => attempt,
        _ => unreachable!(),
    };
    let guard = store
        .prepare_attempt(
            created.run_id(),
            &expected(&started),
            attempt.clone(),
            EpochMillis::new(started.recorded_at().get() + 1).unwrap(),
        )
        .expect("durably prepare attempt");
    assert_eq!(guard.run_id(), created.run_id());
    assert_eq!(guard.attempt(), &attempt);
    let history = store.load_history(created.run_id()).expect("prepared history");
    assert!(matches!(history.last().unwrap().event(), RunEvent::AttemptPrepared { attempt: found } if found == &attempt));

    let receipt = match fixture_event("receipt_recorded") {
        RunEvent::ReceiptRecorded { receipt } => receipt,
        _ => unreachable!(),
    };
    let receipt_expected = ExpectedRunHead::At {
        sequence: guard.committed_sequence(),
        digest: guard.committed_digest().clone(),
    };
    let state = store
        .record_receipt(
            created.run_id(),
            &receipt_expected,
            receipt.clone(),
            EpochMillis::new(started.recorded_at().get() + 2).unwrap(),
        )
        .expect("durably record receipt");
    assert_eq!(
        state
            .prepared_attempts()
            .get(&attempt.id())
            .unwrap()
            .receipt(),
        Some(&receipt)
    );
}

#[test]
fn two_connections_competing_on_one_expected_head_allow_exactly_one_append() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("concurrency.db");
    let mut initializer = RunStore::open(&path).expect("initializer");
    let created = genesis();
    initializer
        .append(&ExpectedRunHead::Genesis, &created)
        .expect("append genesis");
    let started = successor(&created, fixture_event("run_started"));
    initializer
        .append(&expected(&created), &started)
        .expect("append started");
    drop(initializer);

    let candidate = successor(&started, fixture_event("intervention_recorded"));
    let expected_head = expected(&started);
    let barrier = Arc::new(Barrier::new(3));
    let path = Arc::new(path);

    let spawn_writer = |barrier: Arc<Barrier>| {
        let path = Arc::clone(&path);
        let expected_head = expected_head.clone();
        let candidate = candidate.clone();
        thread::spawn(move || {
            let mut store = RunStore::open(path.as_path()).expect("writer connection");
            barrier.wait();
            store.append(&expected_head, &candidate)
        })
    };

    let first = spawn_writer(Arc::clone(&barrier));
    let second = spawn_writer(Arc::clone(&barrier));
    barrier.wait();
    let results = [first.join().expect("first writer"), second.join().expect("second writer")];
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    let errors: Vec<_> = results
        .iter()
        .filter_map(|result| result.as_ref().err())
        .collect();
    assert_eq!(errors.len(), 1);
    assert!(matches!(
        errors[0].code(),
        RunErrorCode::LedgerHeadMismatch | RunErrorCode::StoreBusy
    ));

    let store = RunStore::open(path.as_path()).expect("reopen store");
    let history = store.load_history(created.run_id()).expect("final history");
    assert_eq!(history.len(), 3);
    assert_eq!(history[2], candidate);
}
'''
sqlite.write_text(text)

crash = Path("crates/ecra-run/tests/crash_recovery.rs")
crash.write_text(r'''use std::path::Path;
use std::process::Command;

use ecra_core::EpochMillis;
use ecra_run::{
    ExpectedRunHead, RecoveryReason, RunEvent, RunEventEnvelope, RunPhase, RunStore,
    SuspensionReason,
};
use tempfile::tempdir;

const CHILD_FLAG: &str = "ECRA_RUN_CRASH_WRITER_CHILD";
const CHILD_PATH: &str = "ECRA_RUN_CRASH_DB_PATH";
const ATTEMPT_CHILD_FLAG: &str = "ECRA_RUN_ATTEMPT_CRASH_CHILD";
const ATTEMPT_MODE: &str = "ECRA_RUN_ATTEMPT_CRASH_MODE";
const EFFECT_MARKER: &str = "ECRA_RUN_EFFECT_MARKER";

fn genesis() -> RunEventEnvelope {
    RunEventEnvelope::from_json_slice(include_bytes!(
        "../../../contracts/ecra-run-v1/valid/run-created-envelope.v1.json"
    ))
    .expect("genesis fixture")
}

fn event(kind: &str) -> RunEvent {
    let events: Vec<RunEvent> = serde_json::from_slice(include_bytes!(
        "../../../contracts/ecra-run-v1/valid/all-event-kinds.v1.json"
    ))
    .expect("event fixtures");
    events
        .into_iter()
        .find(|candidate| candidate.kind() == kind)
        .unwrap_or_else(|| panic!("missing fixture {kind}"))
}

fn successor(previous: &RunEventEnvelope, event: RunEvent) -> RunEventEnvelope {
    RunEventEnvelope::new(
        previous.run_id(),
        previous.sequence().checked_next().expect("next sequence"),
        EpochMillis::new(previous.recorded_at().get() + 1).expect("timestamp"),
        Some(previous.event_digest().clone()),
        event,
    )
    .expect("successor")
}

fn expected_for(envelope: &RunEventEnvelope) -> ExpectedRunHead {
    ExpectedRunHead::At {
        sequence: envelope.sequence(),
        digest: envelope.event_digest().clone(),
    }
}

fn expected_state(state: &ecra_run::RunState) -> ExpectedRunHead {
    ExpectedRunHead::At {
        sequence: state.last_sequence(),
        digest: state.last_digest().clone(),
    }
}

fn initialize_running(path: &Path) -> RunEventEnvelope {
    let mut store = RunStore::open(path).expect("open initialization store");
    let created = genesis();
    store
        .append(&ExpectedRunHead::Genesis, &created)
        .expect("append genesis");
    let started = successor(&created, event("run_started"));
    store
        .append(&expected_for(&created), &started)
        .expect("append started");
    started
}

#[test]
fn crash_writer_child() {
    if std::env::var_os(CHILD_FLAG).is_none() {
        return;
    }
    let path = std::env::var_os(CHILD_PATH).expect("child database path");
    let mut store = RunStore::open(&path).expect("child open store");
    store
        .append(&ExpectedRunHead::Genesis, &genesis())
        .expect("child durable append");
    std::process::abort();
}

#[test]
fn attempt_crash_child() {
    if std::env::var_os(ATTEMPT_CHILD_FLAG).is_none() {
        return;
    }
    let path = std::env::var_os(CHILD_PATH).expect("attempt child database path");
    let mode = std::env::var(ATTEMPT_MODE).expect("attempt crash mode");
    let marker = std::env::var_os(EFFECT_MARKER).expect("effect marker path");
    let mut store = RunStore::open(&path).expect("attempt child open store");
    let state = store
        .load_state(genesis().run_id())
        .expect("load running state")
        .expect("running state exists");
    if mode == "A" {
        std::process::abort();
    }

    let attempt = match event("attempt_prepared") {
        RunEvent::AttemptPrepared { attempt } => attempt,
        _ => unreachable!(),
    };
    let guard = store
        .prepare_attempt(
            state.run_id(),
            &expected_state(&state),
            attempt,
            EpochMillis::new(10_000).unwrap(),
        )
        .expect("durable attempt preparation");
    if mode == "B" {
        std::process::abort();
    }
    if mode == "C" {
        std::fs::write(marker, b"synthetic-external-effect").expect("write effect marker");
        std::process::abort();
    }
    if mode == "D" {
        let receipt = match event("receipt_recorded") {
            RunEvent::ReceiptRecorded { receipt } => receipt,
            _ => unreachable!(),
        };
        let expected = ExpectedRunHead::At {
            sequence: guard.committed_sequence(),
            digest: guard.committed_digest().clone(),
        };
        store
            .record_receipt(
                state.run_id(),
                &expected,
                receipt,
                EpochMillis::new(10_001).unwrap(),
            )
            .expect("durable receipt commit");
        std::process::abort();
    }
    panic!("unsupported crash matrix mode {mode}");
}

#[test]
fn committed_event_survives_process_abort_with_wal_and_full() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("crash.db");
    let executable = std::env::current_exe().expect("current test binary");
    let status = Command::new(executable)
        .arg("--exact")
        .arg("crash_writer_child")
        .arg("--nocapture")
        .arg("--test-threads=1")
        .env(CHILD_FLAG, "1")
        .env(CHILD_PATH, &path)
        .status()
        .expect("spawn crash writer child");
    assert!(!status.success(), "child must terminate by abort");

    let store = RunStore::open(&path).expect("reopen after process abort");
    let configuration = store.sqlite_configuration().expect("configuration");
    assert_eq!(configuration.journal_mode(), "wal");
    assert_eq!(configuration.synchronous(), 2);
    let created = genesis();
    let history = store
        .load_history(created.run_id())
        .expect("recover committed history");
    assert_eq!(history, vec![created]);
}

#[test]
fn crash_matrix_a_through_d_preserves_attempt_truth_without_fabrication_or_retry() {
    let executable = std::env::current_exe().expect("current test binary");
    let attempt = match event("attempt_prepared") {
        RunEvent::AttemptPrepared { attempt } => attempt,
        _ => unreachable!(),
    };
    let receipt = match event("receipt_recorded") {
        RunEvent::ReceiptRecorded { receipt } => receipt,
        _ => unreachable!(),
    };

    for mode in ["A", "B", "C", "D"] {
        let directory = tempdir().expect("tempdir");
        let path = directory.path().join(format!("attempt-{mode}.db"));
        let marker = directory.path().join(format!("effect-{mode}.marker"));
        initialize_running(&path);
        let status = Command::new(&executable)
            .arg("--exact")
            .arg("attempt_crash_child")
            .arg("--nocapture")
            .arg("--test-threads=1")
            .env(ATTEMPT_CHILD_FLAG, "1")
            .env(ATTEMPT_MODE, mode)
            .env(CHILD_PATH, &path)
            .env(EFFECT_MARKER, &marker)
            .status()
            .expect("spawn attempt crash child");
        assert!(!status.success(), "mode {mode} child must abort");

        let mut store = RunStore::open(&path).expect("reopen attempt store");
        let state = store
            .load_state(genesis().run_id())
            .expect("load post-crash state")
            .expect("post-crash state exists");
        match mode {
            "A" => {
                assert!(state.prepared_attempts().is_empty());
                assert_eq!(state.phase(), RunPhase::Running);
            }
            "B" | "C" => {
                let prepared = state
                    .prepared_attempts()
                    .get(&attempt.id())
                    .expect("prepared attempt survives crash");
                assert!(prepared.receipt().is_none());
                assert!(!prepared.unresolved());
                assert_eq!(marker.exists(), mode == "C");

                let recovery = store
                    .recover(
                        state.run_id(),
                        &expected_state(&state),
                        RecoveryReason::ProcessRestart,
                        EpochMillis::new(20_000).unwrap(),
                    )
                    .expect("append explicit recovery boundary");
                assert_eq!(recovery.unreceipted_attempts(), [attempt.clone()]);
                assert_eq!(recovery.state().phase(), RunPhase::Suspended);
                assert!(recovery.state().unresolved_attempts().contains(&attempt.id()));
                assert!(matches!(
                    recovery.state().suspension(),
                    Some(SuspensionReason::ReconciliationRequired { attempt: found }) if found == &attempt
                ));
                assert!(
                    recovery
                        .state()
                        .prepared_attempts()
                        .get(&attempt.id())
                        .unwrap()
                        .receipt()
                        .is_none()
                );
                let history = store.load_history(state.run_id()).expect("recovery history");
                assert_eq!(
                    history
                        .iter()
                        .filter(|envelope| matches!(envelope.event(), RunEvent::AttemptPrepared { .. }))
                        .count(),
                    1,
                    "recovery must not retry or create another attempt"
                );
                assert!(
                    !history
                        .iter()
                        .any(|envelope| matches!(envelope.event(), RunEvent::ReceiptRecorded { .. })),
                    "recovery must not fabricate a receipt"
                );
            }
            "D" => {
                let prepared = state
                    .prepared_attempts()
                    .get(&attempt.id())
                    .expect("received attempt survives crash");
                assert_eq!(prepared.receipt(), Some(&receipt));
                assert!(!prepared.unresolved());
                assert_eq!(state.phase(), RunPhase::Running);
            }
            _ => unreachable!(),
        }
    }
}
''')
