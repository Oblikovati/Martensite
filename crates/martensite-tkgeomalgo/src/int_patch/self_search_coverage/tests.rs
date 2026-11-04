use super::*;
use martensite_tkernel::standard::Failure;

#[test]
fn pending_scheduled_work_is_not_finished() {
    let mut receipt = SurfaceSelfSearchReceipt::default();
    receipt.begin();
    receipt.discovered(2, 0);
    receipt.sections.visited = 1;
    receipt.sections.finished = 1;
    receipt.finish(&Ok(()));
    assert_eq!(receipt.termination, SelfSearchTermination::Incomplete);
}
#[test]
fn accounted_empty_discovery_can_finish() {
    let mut receipt = SurfaceSelfSearchReceipt::default();
    receipt.begin();
    receipt.discovered(0, 0);
    receipt.finish(&Ok(()));
    assert_eq!(receipt.termination, SelfSearchTermination::Finished);
    assert_eq!(receipt.discovery, SelfSearchDiscovery::SampledPolyhedron);
}
#[test]
fn unknown_inner_termination_stays_incomplete() {
    let mut receipt = SurfaceSelfSearchReceipt::default();
    receipt.begin();
    receipt.discovered(0, 0);
    receipt.observe_walking(crate::int_walk::WalkingSearchObservation {
        inner_termination_unobserved: true,
        ..Default::default()
    });
    receipt.finish(&Ok(()));
    assert_eq!(receipt.termination, SelfSearchTermination::Incomplete);
    assert_eq!(receipt.calls_with_unobserved_inner_work, 1);
}
#[test]
fn actual_limit_observations_are_retained_without_success_credit() {
    let mut receipt = SurfaceSelfSearchReceipt::default();
    receipt.begin();
    receipt.discovered(0, 0);
    receipt.observe_walking(limited_walking_observation());
    receipt.finish(&Ok(()));
    assert_eq!(receipt.marching_steps, 17);
    assert_eq!(
        receipt.limits,
        SelfSearchLimitCounts {
            no_append: 1,
            repeated_start: 2,
            point_count: 3
        }
    );
    assert_eq!(receipt.termination, SelfSearchTermination::Incomplete);
}
fn limited_walking_observation() -> crate::int_walk::WalkingSearchObservation {
    crate::int_walk::WalkingSearchObservation {
        steps: 17,
        no_append_limit: 1,
        repeated_start_limit: 2,
        point_count_limit: 3,
        ..Default::default()
    }
}

#[test]
fn errors_and_cancellation_remain_distinct() {
    let failure = Err(Failure::with_message(
        "ExpectedTestFailure",
        "owned failure control",
    ));
    let mut receipt = SurfaceSelfSearchReceipt::default();
    receipt.begin();
    receipt.finish(&failure);
    assert_eq!(receipt.termination, SelfSearchTermination::Failed);
    receipt.begin();
    receipt.observe_walking(crate::int_walk::WalkingSearchObservation {
        cancelled: true,
        ..Default::default()
    });
    receipt.finish(&failure);
    assert_eq!(receipt.termination, SelfSearchTermination::Cancelled);
}
#[test]
fn returned_counts_do_not_hide_pruned_failed_or_cancelled_work() {
    let mut counts = SelfSearchWorkCounts::default();
    counts.entered();
    counts.returned(&Ok(()), false);
    assert_eq!(counts.scheduled, 1);
    assert_eq!(counts.visited, 1);
    assert_eq!(counts.finished, 1);
}
#[test]
fn failed_and_cancelled_returns_do_not_count_as_finished() {
    let mut counts = SelfSearchWorkCounts::default();
    counts.entered();
    counts.returned(
        &Err(Failure::with_message(
            "ExpectedTestFailure",
            "owned control",
        )),
        false,
    );
    counts.entered();
    counts.returned(&Ok(()), true);
    assert_eq!(
        (counts.failed, counts.cancelled, counts.finished),
        (1, 1, 0)
    );
}
