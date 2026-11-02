use super::*;

fn observe_exits(exits: &[Exit]) -> NumericalSearchCounts {
    let mut counts = NumericalSearchCounts::default();
    for &termination in exits {
        counts.scheduled += 1;
        counts.returned(
            &Ok::<(), Failure>(()),
            RootSearchObservation {
                termination,
                iterations_entered: 2,
                iteration_limit: 10,
                ..RootSearchObservation::default()
            },
        );
    }
    counts
}
#[test]
fn local_solver_exits_remain_distinct() {
    let counts = observe_exits(&[
        Exit::Accepted,
        Exit::LocalDecline,
        Exit::EvaluationDeclined,
        Exit::Divergence,
        Exit::IterationLimit,
        Exit::Running,
    ]);
    assert_eq!(counts.accepted, 1);
    assert_eq!(counts.locally_declined, 1);
    assert_eq!(counts.evaluation_declined, 1);
    assert_eq!(counts.divergent, 1);
    assert_eq!(counts.exhausted, 1);
    assert_eq!(counts.unobserved, 1);
    assert_eq!(counts.iterations_entered, 12);
    assert!(!counts.all_accepted());
}
#[test]
fn error_and_residual_decline_cannot_be_accepted_work() {
    let mut counts = NumericalSearchCounts {
        scheduled: 1,
        ..Default::default()
    };
    counts.returned(
        &Err::<(), _>(Failure::with_message("test", "actual error")),
        RootSearchObservation::default(),
    );
    assert_eq!(counts.failed, 1);
    assert!(!counts.all_accepted());
    accumulate_residual_decline(&mut counts);
}
fn accumulate_residual_decline(counts: &mut NumericalSearchCounts) {
    let actual = NumericalSearchCounts {
        scheduled: 1,
        accepted: 1,
        residual_declined: 1,
        ..Default::default()
    };
    counts.accumulate(actual);
    assert_eq!(counts.scheduled, 2);
    assert_eq!(counts.residual_declined, 1);
    assert!(!counts.all_accepted());
}
#[test]
fn accepted_work_remains_owned_after_accumulation() {
    let mut counts = NumericalSearchCounts::default();
    assert!(counts.all_accepted());
    let actual = NumericalSearchCounts {
        scheduled: 2,
        accepted: 2,
        iterations_entered: 3,
        ..Default::default()
    };
    counts.accumulate(actual);
    assert!(counts.all_accepted());
    assert_eq!(counts, actual);
    counts.tangency_declined = 1;
    assert!(!counts.all_accepted());
    assert_eq!(actual.tangency_declined, 0);
}
