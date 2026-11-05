//! EF controls distinguish zero-poll shortcuts, real candidate cancellation and full success.
use super::progress_fixture::crossing_filler;
use super::*;
use crate::bop_algo::progress_test_support::{before_indicator, indicator, pulse_indicator};
use crate::bop_algo::{ProgressCancellation, ProgressCompletion};
use std::sync::atomic::Ordering;

#[test]
fn ef_glue_full_does_not_poll_an_already_cancelled_indicator() {
    let mut filler = crossing_filler(false);
    filler.set_glue(Glue::Full);
    let (indicator, trace) = before_indicator();
    let range = indicator.start();
    let observed = ProgressCancellation::default();
    let outcomes = filler.perform_ef_with_progress(&range, &observed).unwrap();
    assert!(
        outcomes.is_empty(),
        "actual {outcomes:?}; expected no EF workers for GlueFull"
    );
    assert_eq!(trace.queries.load(Ordering::SeqCst), 0);
    assert!(!observed.is_cancelled());
}

#[test]
fn ef_candidate_poll_cancels_before_advancing_the_actual_cursor() {
    let mut filler = crossing_filler(false);
    let (indicator, trace) = before_indicator();
    let range = indicator.start();
    let observed = ProgressCancellation::default();
    let outcomes = filler.perform_ef_with_progress(&range, &observed).unwrap();
    assert!(
        outcomes.is_empty(),
        "actual {outcomes:?}; expected no executed EF workers"
    );
    assert_eq!(trace.queries.load(Ordering::SeqCst), 1);
    assert!(observed.is_cancelled());
    assert!(filler.progress_pair_cursor().unwrap().more());
    assert_eq!(filler.structure_mut().unwrap().interf_ef().len(), 0);
}

#[test]
fn ef_ranged_success_keeps_real_crossing_results_in_both_execution_modes() {
    for parallel in [false, true] {
        assert_crossing_success(parallel);
    }
}

fn assert_crossing_success(parallel: bool) {
    let mut filler = crossing_filler(parallel);
    let (indicator, trace) = indicator(0);
    let range = indicator.start();
    let observed = ProgressCancellation::default();
    let outcomes = filler.perform_ef_with_progress(&range, &observed).unwrap();
    assert!(
        !outcomes.is_empty(),
        "actual {outcomes:?}; expected a real EF task completion"
    );
    assert!(
        outcomes
            .iter()
            .all(|item| matches!(item, ProgressCompletion::Completed)),
        "actual {outcomes:?}; expected all reached children completed"
    );
    assert_eq!(filler.structure_mut().unwrap().interf_ef().len(), 1);
    assert!(!observed.is_cancelled());
    assert!(trace.queries.load(Ordering::SeqCst) >= 4);
}

#[test]
fn ef_empty_pair_set_does_not_poll_or_invent_worker_completion() {
    let mut filler = BOPAlgoPaveFiller::default();
    filler.add_argument(&super::progress_fixture::crossing_edge());
    filler.add_argument(&super::progress_fixture::crossing_edge());
    filler.initialize().unwrap();
    let (indicator, trace) = before_indicator();
    let range = indicator.start();
    let observed = ProgressCancellation::default();
    let outcomes = filler.perform_ef_with_progress(&range, &observed).unwrap();
    assert!(
        outcomes.is_empty(),
        "actual {outcomes:?}; expected no EF candidate tasks"
    );
    assert_eq!(trace.queries.load(Ordering::SeqCst), 0);
    assert!(!observed.is_cancelled());
}

#[test]
fn ef_common_part_stop_does_not_enter_post_treatment() {
    let mut filler = crossing_filler(false);
    let (indicator, trace) = pulse_indicator(6);
    let range = indicator.start();
    let observed = ProgressCancellation::default();
    let outcomes = filler.perform_ef_with_progress(&range, &observed).unwrap();
    assert_eq!(trace.queries.load(Ordering::SeqCst), 6);
    assert!(observed.is_cancelled());
    assert_eq!(outcomes, vec![ProgressCompletion::Completed]);
    let structure = filler.structure_mut().unwrap();
    assert_eq!(structure.interf_ef().len(), 0);
    assert_eq!(structure.interf_ve().len(), 0);
}
