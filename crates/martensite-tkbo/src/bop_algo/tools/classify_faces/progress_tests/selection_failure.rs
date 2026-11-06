//! A supplied void bound must fail only after the native initial task cancellation check.
use super::{Context, InParts, ProgressCompletion, classify_faces_with_progress};
use super::{fixtures::AnalyticalClassification, matrix::rendered_members, with_owned_workers_on};
use crate::bop_algo::progress_test_support as recording;
use martensite_tkernel::message::ProgressRange;
use std::sync::atomic::Ordering;

#[test]
fn actual_selection_failure_uses_native_worker_boundary_after_cancellation() {
    let mut records = Vec::new();
    for parallel in [false, true] {
        for cancel in ["", "before", "query_4"] {
            records.push(observe(parallel, cancel));
        }
    }
    let expected: Vec<&str> = include_str!("native-classifier-failure.jsonl")
        .lines()
        .collect();
    println!("{}", records.join("\n"));
    for (index, record) in records.iter().enumerate() {
        if index == 3 {
            assert_parallel_schedule(record, expected[index]);
        } else {
            assert_eq!(record, expected[index]);
        }
    }
}

fn observe(parallel: bool, cancel: &str) -> String {
    let mut fixture = AnalyticalClassification::new();
    fixture.bind_void_first();
    let before = fixture.snapshot();
    let context = Context::default();
    let (indicator, trace) = recording::profile_indicator(cancel);
    let (completion, parts) = perform_case(&fixture, &context, parallel, &indicator.start());
    let (failure_type, failure_message) = failure_fields(completion, &trace);
    assert_eq!(fixture.snapshot(), before);
    let members = rendered_members(&fixture.observed_membership(&parts));
    render_observation(
        parallel,
        cancel,
        &failure_type,
        &failure_message,
        parts.extent(),
        &members,
        &trace,
    )
}

fn failure_fields(completion: ProgressCompletion, trace: &recording::Trace) -> (String, String) {
    match completion {
        ProgressCompletion::Failed {
            cause: Some(cause), ..
        } => (
            cause.exception_type().to_owned(),
            cause.message().to_owned(),
        ),
        ProgressCompletion::Cancelled { .. } if trace.break_observed.load(Ordering::SeqCst) => {
            (String::new(), String::new())
        }
        other => {
            panic!("completion={other:?}; expected actual native failure or observed cancellation")
        }
    }
}

fn render_observation(
    parallel: bool,
    cancel: &str,
    failure_type: &str,
    failure_message: &str,
    entries: usize,
    members: &str,
    trace: &recording::Trace,
) -> String {
    let queries = trace.queries.load(Ordering::SeqCst);
    let cancelled = trace.cancelled.load(Ordering::SeqCst);
    let observed = trace.break_observed.load(Ordering::SeqCst);
    let events = recording::rendered_events(trace);
    format!(
        "{{\"kind\":\"one_void_box\",\"cancel_scope\":{cancel:?},\"parallel\":{parallel},\"queries\":{queries},\"cancelled\":{cancelled},\"break_observed\":{observed},\"failure_type\":{failure_type:?},\"failure_message\":{failure_message:?},\"entries\":{entries},\"members\":{members},\"events\":[{events}]}}"
    )
}

fn perform_case(
    fixture: &AnalyticalClassification,
    context: &Context,
    parallel: bool,
    range: &ProgressRange<'_>,
) -> (ProgressCompletion, InParts) {
    if parallel {
        return with_owned_workers_on(fixture, context, &|_| Ok(()), range);
    }
    let mut parts = InParts::default();
    let completion = classify_faces_with_progress(fixture.job(context, &mut parts, false), range);
    (completion, parts)
}

fn assert_parallel_schedule(actual: &str, native: &str) {
    let (prefix, _) = native.rsplit_once(",\"events\":").unwrap();
    let matches = include_str!("native-failure-schedules.jsonl")
        .lines()
        .any(|events| actual == format!("{prefix},\"events\":{events}}}"));
    assert!(
        matches,
        "actual={actual}; expected exact native fields and one independently validated E/NN scope schedule"
    );
}
