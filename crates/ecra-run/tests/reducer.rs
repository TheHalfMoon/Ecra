use ecra_core::EpochMillis;
use ecra_run::{
    BudgetAmount, BudgetDimension, RunErrorCode, RunEvent, RunEventEnvelope, RunPhase, RunReducer,
    SuspensionReason,
};

fn amount(value: u64) -> BudgetAmount {
    BudgetAmount::new(value).expect("safe budget amount")
}

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
        .unwrap_or_else(|| panic!("missing fixture for {kind}"))
}

fn push(history: &mut Vec<RunEventEnvelope>, next: RunEvent) {
    let previous = history.last().expect("history has genesis");
    let sequence = previous.sequence().checked_next().expect("next sequence");
    let recorded_at = EpochMillis::new(previous.recorded_at().get() + 1).expect("timestamp");
    history.push(
        RunEventEnvelope::new(
            previous.run_id(),
            sequence,
            recorded_at,
            Some(previous.event_digest().clone()),
            next,
        )
        .expect("valid successor envelope"),
    );
}

fn push_valid_budget_exhaustion(history: &mut Vec<RunEventEnvelope>) {
    push(
        history,
        RunEvent::ResourceUsageRecorded {
            dimension: BudgetDimension::ToolCalls,
            amount: amount(100),
        },
    );
    push(
        history,
        RunEvent::BudgetExhausted {
            dimension: BudgetDimension::ToolCalls,
            hard_limit: amount(100),
            cumulative_usage: amount(100),
        },
    );
}

fn history(kinds: &[&str]) -> Vec<RunEventEnvelope> {
    let mut history = vec![genesis()];
    for kind in kinds {
        push(&mut history, event(kind));
    }
    history
}

fn apply_kind(base: &[&str], candidate: &str) -> Result<RunPhase, RunErrorCode> {
    let mut history = history(base);
    let state = RunReducer::reduce(&history).expect("base state");

    if candidate == "budget_exhausted" && state.phase() == RunPhase::Running {
        push(
            &mut history,
            RunEvent::ResourceUsageRecorded {
                dimension: BudgetDimension::ToolCalls,
                amount: amount(100),
            },
        );
        let charged_state = RunReducer::reduce(&history).expect("charged running state");
        push(
            &mut history,
            RunEvent::BudgetExhausted {
                dimension: BudgetDimension::ToolCalls,
                hard_limit: amount(100),
                cumulative_usage: amount(100),
            },
        );
        return RunReducer::apply(&charged_state, history.last().expect("budget exhaustion"))
            .map(|next| next.phase())
            .map_err(|error| error.code());
    }

    push(&mut history, event(candidate));
    RunReducer::apply(&state, history.last().expect("candidate envelope"))
        .map(|next| next.phase())
        .map_err(|error| error.code())
}

#[test]
fn exact_phase_transition_matrix_accepts_only_v1_edges() {
    let phase_cases: [(&[&str], RunPhase); 7] = [
        (&[], RunPhase::Created),
        (&["run_started"], RunPhase::Running),
        (&["run_started", "run_suspended"], RunPhase::Suspended),
        (&["cancellation_requested"], RunPhase::CancellationRequested),
        (
            &["cancellation_requested", "run_cancelled"],
            RunPhase::Cancelled,
        ),
        (&["run_failed"], RunPhase::Failed),
        (
            &["run_started", "execution_completed"],
            RunPhase::ExecutionCompleted,
        ),
    ];

    let candidates = [
        "run_started",
        "run_suspended",
        "run_resumed",
        "cancellation_requested",
        "run_cancelled",
        "run_failed",
        "execution_completed",
        "budget_exhausted",
        "recovery_boundary",
    ];

    for (base, phase) in phase_cases {
        let allowed: &[&str] = match phase {
            RunPhase::Created => &["run_started", "cancellation_requested", "run_failed"],
            RunPhase::Running => &[
                "run_suspended",
                "cancellation_requested",
                "run_failed",
                "execution_completed",
                "budget_exhausted",
                "recovery_boundary",
            ],
            RunPhase::Suspended => &["run_resumed", "cancellation_requested", "run_failed"],
            RunPhase::CancellationRequested => {
                &["run_cancelled", "run_failed", "recovery_boundary"]
            }
            RunPhase::Cancelled | RunPhase::Failed | RunPhase::ExecutionCompleted => &[],
        };

        for candidate in candidates {
            let result = apply_kind(base, candidate);
            if allowed.contains(&candidate) {
                assert!(
                    result.is_ok(),
                    "{candidate} should be accepted from {phase:?}, got {result:?}"
                );
            } else {
                assert_eq!(
                    result,
                    Err(RunErrorCode::InvalidStateTransition),
                    "{candidate} should be rejected from {phase:?}"
                );
            }
        }
    }
}

#[test]
fn terminal_state_rejects_non_phase_events_too() {
    let mut completed = history(&["run_started", "execution_completed"]);
    let state = RunReducer::reduce(&completed).expect("completed state");
    push(&mut completed, event("intervention_recorded"));
    let error = RunReducer::apply(&state, completed.last().expect("event"))
        .expect_err("terminal state must reject every later v1 event");
    assert_eq!(error.code(), RunErrorCode::InvalidStateTransition);
}

#[test]
fn resumability_obeys_suspension_and_unresolved_blockers() {
    let paused = history(&["run_started", "run_suspended"]);
    let paused_state = RunReducer::reduce(&paused).expect("paused state");
    assert_eq!(paused_state.phase(), RunPhase::Suspended);
    assert!(matches!(
        paused_state.suspension(),
        Some(SuspensionReason::UserPause)
    ));
    assert_eq!(
        apply_kind(&["run_started", "run_suspended"], "run_resumed"),
        Ok(RunPhase::Running)
    );

    let mut exhausted = history(&["run_started"]);
    push_valid_budget_exhaustion(&mut exhausted);
    let exhausted_state = RunReducer::reduce(&exhausted).expect("budget suspended state");
    let mut attempt = exhausted;
    push(&mut attempt, event("run_resumed"));
    let error = RunReducer::apply(&exhausted_state, attempt.last().expect("resume"))
        .expect_err("budget exhaustion is not directly resumable in v1");
    assert_eq!(error.code(), RunErrorCode::InvalidStateTransition);
}

#[test]
fn recovery_without_inflight_attempt_suspends_as_runtime_interruption() {
    let recovered = history(&["run_started", "recovery_boundary"]);
    let state = RunReducer::reduce(&recovered).expect("recovered state");
    assert_eq!(state.phase(), RunPhase::Suspended);
    assert!(matches!(
        state.suspension(),
        Some(SuspensionReason::RuntimeInterruption)
    ));
    assert!(state.unresolved_attempts().is_empty());
}
