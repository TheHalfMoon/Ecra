use ecra_core::EpochMillis;
use ecra_run::{
    BudgetAmount, BudgetDimension, BudgetLimit, BudgetUsage, MAX_BUDGET_AMOUNT, RunBudget,
    RunErrorCode, RunEvent, RunEventEnvelope, RunPhase, RunReducer, SuspensionReason,
};
use proptest::prelude::*;

fn amount(value: u64) -> BudgetAmount {
    BudgetAmount::new(value).expect("safe budget amount")
}

fn genesis() -> RunEventEnvelope {
    RunEventEnvelope::from_json_slice(include_bytes!(
        "../../../contracts/ecra-run-v1/valid/run-created-envelope.v1.json"
    ))
    .expect("genesis fixture")
}

fn fixture_event(kind: &str) -> RunEvent {
    let events: Vec<RunEvent> = serde_json::from_slice(include_bytes!(
        "../../../contracts/ecra-run-v1/valid/all-event-kinds.v1.json"
    ))
    .expect("event fixtures");
    events
        .into_iter()
        .find(|event| event.kind() == kind)
        .unwrap_or_else(|| panic!("missing fixture event {kind}"))
}

fn push(history: &mut Vec<RunEventEnvelope>, event: RunEvent) {
    let previous = history.last().expect("history has head");
    history.push(
        RunEventEnvelope::new(
            previous.run_id(),
            previous.sequence().checked_next().expect("next sequence"),
            EpochMillis::new(previous.recorded_at().get() + 1).expect("timestamp"),
            Some(previous.event_digest().clone()),
            event,
        )
        .expect("valid event envelope"),
    );
}

fn running() -> Vec<RunEventEnvelope> {
    let mut history = vec![genesis()];
    push(&mut history, RunEvent::RunStarted {});
    history
}

fn charge(dimension: BudgetDimension, value: u64) -> RunEvent {
    RunEvent::ResourceUsageRecorded {
        dimension,
        amount: amount(value),
    }
}

#[test]
fn budget_dimension_v1_contract_is_exact_and_closed() {
    let names: Vec<String> = BudgetDimension::ALL
        .iter()
        .map(|dimension| serde_json::to_string(dimension).expect("dimension json"))
        .collect();
    assert_eq!(
        names,
        [
            "\"active_wall_millis\"",
            "\"steps\"",
            "\"tool_calls\"",
            "\"model_calls\"",
            "\"input_tokens\"",
            "\"output_tokens\"",
            "\"cost_microunits\"",
            "\"process_count\"",
            "\"process_millis\"",
            "\"output_bytes\"",
            "\"network_requests\"",
            "\"network_bytes\"",
            "\"storage_bytes\"",
            "\"recursion_depth\"",
        ]
    );
}

#[test]
fn malformed_duplicate_and_out_of_range_budgets_fail_closed() {
    assert_eq!(
        RunBudget::new(vec![])
            .expect_err("empty budget invalid")
            .code(),
        RunErrorCode::InvalidBudget
    );

    let first = BudgetLimit::new(BudgetDimension::ToolCalls, Some(amount(1)), amount(2))
        .expect("first limit");
    let duplicate =
        BudgetLimit::new(BudgetDimension::ToolCalls, None, amount(3)).expect("duplicate candidate");
    assert_eq!(
        RunBudget::new(vec![first, duplicate])
            .expect_err("duplicate dimension invalid")
            .code(),
        RunErrorCode::InvalidBudget
    );
    assert_eq!(
        BudgetLimit::new(BudgetDimension::Steps, Some(amount(3)), amount(2))
            .expect_err("soft > hard invalid")
            .code(),
        RunErrorCode::InvalidBudget
    );
    assert_eq!(
        BudgetAmount::new(MAX_BUDGET_AMOUNT + 1)
            .expect_err("out of range invalid")
            .code(),
        RunErrorCode::InvalidBudget
    );
    assert!(serde_json::from_str::<BudgetAmount>("-1").is_err());
    assert!(serde_json::from_str::<BudgetAmount>("1.5").is_err());
    assert!(serde_json::from_str::<BudgetDimension>("\"unknown_units\"").is_err());
}

#[test]
fn cumulative_usage_remaining_and_preflight_are_checked() {
    let budget = RunBudget::new(vec![
        BudgetLimit::new(BudgetDimension::ToolCalls, Some(amount(80)), amount(100)).expect("limit"),
    ])
    .expect("budget");
    let mut usage = BudgetUsage::default();
    assert_eq!(
        budget.remaining(&usage, BudgetDimension::ToolCalls),
        Some(amount(100))
    );
    let (previous, cumulative) = usage
        .charge(BudgetDimension::ToolCalls, amount(40))
        .expect("charge");
    assert_eq!(previous, BudgetAmount::ZERO);
    assert_eq!(cumulative, amount(40));
    assert_eq!(
        budget.remaining(&usage, BudgetDimension::ToolCalls),
        Some(amount(60))
    );
    assert!(
        budget
            .preflight(&usage, BudgetDimension::ToolCalls, amount(60))
            .is_ok()
    );
    assert_eq!(
        budget
            .preflight(&usage, BudgetDimension::ToolCalls, amount(61))
            .expect_err("preflight must refuse oversize unit")
            .code(),
        RunErrorCode::BudgetPreflightExceeded
    );
    assert!(
        budget
            .preflight(
                &usage,
                BudgetDimension::NetworkBytes,
                amount(MAX_BUDGET_AMOUNT)
            )
            .is_ok()
    );

    let mut overflow = BudgetUsage::default();
    overflow
        .charge(BudgetDimension::Steps, amount(MAX_BUDGET_AMOUNT))
        .expect("max charge");
    assert_eq!(
        overflow
            .charge(BudgetDimension::Steps, amount(1))
            .expect_err("I-JSON cumulative overflow")
            .code(),
        RunErrorCode::BudgetOverflow
    );
}

#[test]
fn all_fourteen_dimensions_cover_zero_soft_hard_max_and_overflow_boundaries() {
    for dimension in BudgetDimension::ALL {
        let zero_budget = RunBudget::new(vec![
            BudgetLimit::new(dimension, None, amount(0)).expect("zero hard limit"),
        ])
        .expect("zero budget");
        let zero_usage = BudgetUsage::default();
        assert_eq!(
            zero_budget.remaining(&zero_usage, dimension),
            Some(BudgetAmount::ZERO)
        );
        assert_eq!(
            zero_budget
                .preflight(&zero_usage, dimension, amount(1))
                .expect_err("zero hard limit blocks positive preflight")
                .code(),
            RunErrorCode::BudgetPreflightExceeded
        );
        assert_eq!(
            zero_budget.hard_exhaustion(dimension, BudgetAmount::ZERO),
            Some((BudgetAmount::ZERO, BudgetAmount::ZERO))
        );

        let budget = RunBudget::new(vec![
            BudgetLimit::new(dimension, Some(amount(1)), amount(2)).expect("limit"),
        ])
        .expect("budget");
        let mut usage = BudgetUsage::default();
        assert_eq!(usage.get(dimension), BudgetAmount::ZERO);
        let (previous, soft) = usage.charge(dimension, amount(1)).expect("soft charge");
        assert_eq!(
            budget.soft_crossing(dimension, previous, soft),
            Some((amount(1), amount(1)))
        );
        let (_, hard) = usage.charge(dimension, amount(1)).expect("hard charge");
        assert_eq!(
            budget.hard_exhaustion(dimension, hard),
            Some((amount(2), amount(2)))
        );

        let max_budget = RunBudget::new(vec![
            BudgetLimit::new(dimension, None, amount(MAX_BUDGET_AMOUNT)).expect("max limit"),
        ])
        .expect("max budget");
        let mut max_usage = BudgetUsage::default();
        max_usage
            .charge(dimension, amount(MAX_BUDGET_AMOUNT))
            .expect("max safe charge");
        assert_eq!(
            max_budget.remaining(&max_usage, dimension),
            Some(BudgetAmount::ZERO)
        );
        assert_eq!(
            max_usage
                .charge(dimension, amount(1))
                .expect_err("overflow")
                .code(),
            RunErrorCode::BudgetOverflow
        );
    }
}

#[test]
fn reducer_validates_first_soft_crossing_and_exact_hard_exhaustion_evidence() {
    let mut history = running();
    push(&mut history, charge(BudgetDimension::ToolCalls, 79));
    push(&mut history, charge(BudgetDimension::ToolCalls, 1));
    push(
        &mut history,
        RunEvent::BudgetSoftLimitReached {
            dimension: BudgetDimension::ToolCalls,
            soft_limit: amount(80),
            cumulative_usage: amount(80),
        },
    );
    push(&mut history, charge(BudgetDimension::ToolCalls, 20));
    push(
        &mut history,
        RunEvent::BudgetExhausted {
            dimension: BudgetDimension::ToolCalls,
            hard_limit: amount(100),
            cumulative_usage: amount(100),
        },
    );

    let state = RunReducer::reduce(&history).expect("budget history reduces");
    assert_eq!(state.phase(), RunPhase::Suspended);
    assert_eq!(
        state.usage_for(BudgetDimension::ToolCalls),
        Some(amount(100))
    );
    assert!(matches!(
        state.suspension(),
        Some(SuspensionReason::BudgetExhausted {
            dimension: BudgetDimension::ToolCalls
        })
    ));

    let mut duplicate_soft = running();
    push(&mut duplicate_soft, charge(BudgetDimension::ToolCalls, 80));
    push(
        &mut duplicate_soft,
        RunEvent::BudgetSoftLimitReached {
            dimension: BudgetDimension::ToolCalls,
            soft_limit: amount(80),
            cumulative_usage: amount(80),
        },
    );
    let accepted = RunReducer::reduce(&duplicate_soft).expect("first soft signal");
    push(
        &mut duplicate_soft,
        RunEvent::BudgetSoftLimitReached {
            dimension: BudgetDimension::ToolCalls,
            soft_limit: amount(80),
            cumulative_usage: amount(80),
        },
    );
    assert_eq!(
        RunReducer::apply(&accepted, duplicate_soft.last().expect("duplicate soft"))
            .expect_err("soft signal is first-crossing only")
            .code(),
        RunErrorCode::InvalidBudget
    );
}

#[test]
fn recursive_tool_loop_stops_deterministically_at_hard_budget() {
    let mut history = running();
    for observed in 1..=100 {
        push(&mut history, charge(BudgetDimension::ToolCalls, 1));
        if observed == 80 {
            push(
                &mut history,
                RunEvent::BudgetSoftLimitReached {
                    dimension: BudgetDimension::ToolCalls,
                    soft_limit: amount(80),
                    cumulative_usage: amount(80),
                },
            );
        }
    }
    push(
        &mut history,
        RunEvent::BudgetExhausted {
            dimension: BudgetDimension::ToolCalls,
            hard_limit: amount(100),
            cumulative_usage: amount(100),
        },
    );
    let state = RunReducer::reduce(&history).expect("bounded loop history");
    assert_eq!(state.phase(), RunPhase::Suspended);

    let mut attempted_more = history;
    push(&mut attempted_more, charge(BudgetDimension::ToolCalls, 1));
    assert_eq!(
        RunReducer::apply(&state, attempted_more.last().expect("extra work"))
            .expect_err("hard budget must stop further governed work")
            .code(),
        RunErrorCode::InvalidStateTransition
    );
}

#[test]
fn budget_exhaustion_preserves_unresolved_attempt_truth() {
    let mut history = running();
    push(&mut history, fixture_event("attempt_prepared"));
    push(&mut history, fixture_event("attempt_marked_unknown"));
    let before = RunReducer::reduce(&history).expect("unresolved running state");
    assert_eq!(before.unresolved_attempts().len(), 1);
    let prepared_before = before.prepared_attempts().clone();

    push(&mut history, charge(BudgetDimension::ToolCalls, 100));
    push(
        &mut history,
        RunEvent::BudgetExhausted {
            dimension: BudgetDimension::ToolCalls,
            hard_limit: amount(100),
            cumulative_usage: amount(100),
        },
    );
    let after = RunReducer::reduce(&history).expect("budget exhaustion with unresolved attempt");
    assert_eq!(after.phase(), RunPhase::Suspended);
    assert_eq!(after.unresolved_attempts(), before.unresolved_attempts());
    assert_eq!(after.prepared_attempts(), &prepared_before);
}

#[test]
fn hard_budget_blocks_further_governed_work_before_exhaustion_evidence() {
    let mut history = running();
    push(&mut history, charge(BudgetDimension::ToolCalls, 100));
    let state = RunReducer::reduce(&history).expect("hard-blocked running state");
    assert!(state.has_hard_budget_blocker());

    let mut extra_usage = history.clone();
    push(&mut extra_usage, charge(BudgetDimension::ToolCalls, 1));
    assert_eq!(
        RunReducer::apply(&state, extra_usage.last().expect("extra usage"))
            .expect_err("usage after hard exhaustion must stop")
            .code(),
        RunErrorCode::BudgetExhausted
    );

    let mut extra_attempt = history;
    push(&mut extra_attempt, fixture_event("attempt_prepared"));
    assert_eq!(
        RunReducer::apply(&state, extra_attempt.last().expect("extra attempt"))
            .expect_err("attempt after hard exhaustion must stop")
            .code(),
        RunErrorCode::BudgetExhausted
    );
}

#[test]
fn hard_exhaustion_evidence_must_match_limit_and_current_usage() {
    let mut history = running();
    push(&mut history, charge(BudgetDimension::ToolCalls, 100));
    let state = RunReducer::reduce(&history).expect("hard-blocked state");
    push(
        &mut history,
        RunEvent::BudgetExhausted {
            dimension: BudgetDimension::ToolCalls,
            hard_limit: amount(99),
            cumulative_usage: amount(100),
        },
    );
    assert_eq!(
        RunReducer::apply(&state, history.last().expect("bad hard evidence"))
            .expect_err("mismatched hard limit must fail")
            .code(),
        RunErrorCode::InvalidBudget
    );
}

proptest! {
    #[test]
    fn checked_accounting_never_wraps(a in 0_u64..=MAX_BUDGET_AMOUNT, b in 0_u64..=MAX_BUDGET_AMOUNT) {
        let left = amount(a);
        let right = amount(b);
        match a.checked_add(b).filter(|sum| *sum <= MAX_BUDGET_AMOUNT) {
            Some(sum) => prop_assert_eq!(left.checked_add(right).expect("safe sum").get(), sum),
            None => prop_assert_eq!(left.checked_add(right).expect_err("overflow").code(), RunErrorCode::BudgetOverflow),
        }
    }
}
