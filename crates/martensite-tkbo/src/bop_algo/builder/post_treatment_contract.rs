//! Native post-treatment has stage progress but no internal cancellation poll.
use super::{BOPAlgoBuilder, image_progress_contract::compound_structure};
use crate::bop_algo::progress_test_support::{Trace, indicator, pulse_indicator};
use std::sync::atomic::Ordering;

#[test]
fn post_treatment_reports_both_actual_correction_boundaries() {
    let structure = compound_structure();
    let mut builder = BOPAlgoBuilder::new();
    builder.shape = structure.shape(0).unwrap().clone();
    let (progress, trace) = indicator(0);
    builder
        .post_treat_progress_raw(&structure, &progress.start())
        .unwrap();
    assert_eq!(trace.queries.load(Ordering::SeqCst), 0);
    let events = trace.events.lock().unwrap();
    assert!(
        events
            .iter()
            .any(|(position, _)| *position == 0.5_f64.to_bits())
    );
    drop(events);
    record_post_treatment("", &builder, &trace);
}

#[test]
fn native_post_treatment_does_not_invent_a_cancellation_poll() {
    let structure = compound_structure();
    let mut builder = BOPAlgoBuilder::new();
    builder.shape = structure.shape(0).unwrap().clone();
    builder.set_non_destructive(true);
    let (progress, trace) = pulse_indicator(1);
    builder
        .post_treat_progress_raw(&structure, &progress.start())
        .unwrap();
    assert_eq!(trace.queries.load(Ordering::SeqCst), 0);
    assert!(!builder.options().has_errors());
    record_post_treatment("query_1", &builder, &trace);
}

fn record_post_treatment(cancel: &str, builder: &BOPAlgoBuilder, trace: &Trace) {
    let record = crate::bop_algo::progress_test_support::observation(
        "post_treatment",
        cancel,
        builder.options(),
        0,
        trace,
    );
    println!("{record}");
}
