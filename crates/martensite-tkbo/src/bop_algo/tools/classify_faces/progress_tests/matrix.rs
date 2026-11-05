//! Native-first counts retain requests, observed stops, raw maps and callback scopes separately.
use super::{
    Context, InParts, ProgressCompletion, classify_faces_with_progress,
    fixtures::AnalyticalClassification,
};
use crate::bop_algo::progress_test_support as recording;
use std::sync::atomic::Ordering;

const RECIPES: [&str; 5] = [
    "two",
    "no_candidate",
    "empty_solid",
    "empty_faces",
    "empty_solids",
];
const PROFILES: [&str; 12] = [
    "",
    "before",
    "Classification of faces relatively solids",
    "query_1",
    "query_2",
    "query_3",
    "query_4",
    "query_5",
    "query_6",
    "query_7",
    "query_8",
    "query_9",
];

#[test]
fn direct_classification_matches_native_poll_and_scope_matrix() {
    let mut records = Vec::new();
    for recipe in RECIPES {
        for cancel in PROFILES {
            records.push(observe(recipe, cancel, false));
        }
        records.push(observe(recipe, "", true));
    }
    let actual = records.join("\n") + "\n";
    println!("{actual}");
    assert_eq!(actual, include_str!("native-classifier.jsonl"));
}

fn observe(recipe: &str, cancel: &str, parallel: bool) -> String {
    let fixture = AnalyticalClassification::for_recipe(recipe);
    let context = Context::default();
    let mut parts = InParts::default();
    let (indicator, trace) = recording::profile_indicator(cancel);
    let completion = classify_faces_with_progress(
        fixture.job(&context, &mut parts, parallel),
        &indicator.start(),
    );
    let observed = trace.break_observed.load(Ordering::SeqCst);
    assert_observed_completion(&completion, observed);
    let members = rendered_members(&fixture.observed_membership(&parts));
    render_observation(recipe, cancel, parallel, parts.extent(), &members, &trace)
}

/// Render only declared input indices; `rendered_members(&members)`.
pub(super) fn rendered_members(members: &[Vec<usize>]) -> String {
    let rows: Vec<String> = members
        .iter()
        .map(|row| {
            let indices: Vec<String> = row.iter().map(usize::to_string).collect();
            format!("[{}]", indices.join(","))
        })
        .collect();
    format!("[{}]", rows.join(","))
}

fn render_observation(
    recipe: &str,
    cancel: &str,
    parallel: bool,
    entries: usize,
    members: &str,
    trace: &recording::Trace,
) -> String {
    let queries = trace.queries.load(Ordering::SeqCst);
    let cancelled = trace.cancelled.load(Ordering::SeqCst);
    let observed = trace.break_observed.load(Ordering::SeqCst);
    let events = recording::rendered_events(trace);
    format!(
        "{{\"kind\":{recipe:?},\"cancel_scope\":{cancel:?},\"parallel\":{parallel},\"queries\":{queries},\"cancelled\":{cancelled},\"break_observed\":{observed},\"entries\":{entries},\"members\":{members},\"events\":[{events}]}}"
    )
}

fn assert_observed_completion(completion: &ProgressCompletion, observed: bool) {
    assert_eq!(
        matches!(completion, ProgressCompletion::Cancelled { .. }),
        observed
    );
    assert!(
        matches!(
            completion,
            ProgressCompletion::Completed | ProgressCompletion::Cancelled { .. }
        ),
        "completion={completion:?}; expected Completed or actually observed Cancelled"
    );
}
