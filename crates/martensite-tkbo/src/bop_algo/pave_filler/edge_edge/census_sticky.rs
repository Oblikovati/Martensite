//! Partial sticky comparison: final observations and scope bits, never projected live per-query DS.
use super::{BOPAlgoPaveFiller, final_state_json, shapes, trace};
use crate::bop_algo::{ProgressCancellation, ProgressCompletion};
use martensite_tkernel::standard::Failure;
use std::collections::HashSet;

#[test]
fn serial_sticky_edge_matrix_matches_97_native_final_observations_and_scope_bits() {
    let mut seen = HashSet::new();
    let mut mismatches = Vec::new();
    for line in include_str!("census_sticky97.expected").lines() {
        let (profile, target, expected) = reference_case(line);
        assert!(
            seen.insert((profile, target)),
            "EE {profile}/{target}; expected unique fixture identity"
        );
        let actual = sticky_observation(profile, target);
        println!("{actual}");
        if actual != expected {
            mismatches.push((profile, target));
        }
    }
    assert_census_outcome(seen.len(), &mismatches);
}

fn assert_census_outcome(count: usize, mismatches: &[(&str, usize)]) {
    assert_eq!(
        count, 97,
        "expected all97 qualified Native serial identities"
    );
    assert!(
        mismatches.is_empty(),
        "actual differing EE cases {mismatches:?}; expected exact final fields/query returns/full scope bits; raw rows printed above"
    );
}

fn reference_case(line: &str) -> (&str, usize, &str) {
    let fields: Vec<&str> = line.splitn(3, '|').collect();
    let [profile, target, expected]: [&str; 3] = fields.try_into().unwrap_or_else(|actual| {
        panic!("EE fixture {actual:?}; expected profile|target|observation")
    });
    let target = target.parse::<usize>().unwrap_or_else(|cause| {
        panic!("EE target {target:?} ({cause}); expected a nonnegative query ordinal")
    });
    assert!(
        [
            "none", "cross", "shared", "endpoint", "triple", "far", "flagged"
        ]
        .contains(&profile),
        "EE profile {profile:?}; expected one of seven qualified census fixtures"
    );
    (profile, target, expected)
}

fn sticky_observation(profile: &str, target: usize) -> String {
    let mut filler = shapes::configured_census(profile);
    let (indicator, trace) = trace::sticky_indicator(target);
    let observed = ProgressCancellation::default();
    let range = indicator.start();
    let outcome = filler.perform_ee_with_progress(&range, &observed);
    drop(range);
    let row = sticky_row_json(profile, target, &filler, &trace);
    append_returned_failure(row, outcome)
}

fn sticky_row_json(
    profile: &str,
    target: usize,
    filler: &BOPAlgoPaveFiller,
    trace: &trace::SerialEETrace,
) -> String {
    let state = final_state_json(filler);
    let polls = trace.query_count();
    let events = trace.event_json();
    let responses = trace.responses_json();
    let configured = target != 0;
    let stopped = trace.break_observed();
    format!(
        "{{\"profile\":\"{profile}\",\"parallel\":false,\"cancel_target\":{target},\"request_configured\":{configured},\"break_observed\":{stopped},\"query_returns\":{responses},{state},\"poll_count\":{polls},\"events\":{events}}}"
    )
}

fn append_returned_failure(
    mut row: String,
    outcome: Result<Vec<ProgressCompletion>, Failure>,
) -> String {
    let Err(cause) = outcome else {
        return row;
    };
    assert_eq!(
        row.pop(),
        Some('}'),
        "EE observation {row:?}; expected a JSON object terminator"
    );
    let kind = failure_hex(cause.exception_type().as_bytes());
    let what = failure_hex(cause.what());
    let stack = failure_hex(cause.stack_bytes());
    // Encoding keeps the actual bytes even when the failure is not UTF-8 text.
    row.push_str(&format!(",\"returned_failure\":{{\"type_hex\":\"{kind}\",\"what_hex\":\"{what}\",\"stack_hex\":\"{stack}\"}}}}"));
    row
}

fn failure_hex(bytes: &[u8]) -> String {
    let fields: Vec<String> = bytes.iter().map(|value| format!("{value:02x}")).collect();
    // No character replacement can erase an offending message or stack byte.
    fields.join("")
}

#[test]
fn returned_ee_failure_encoding_keeps_exact_bytes_and_success_row() {
    let cause = Failure::with_type_bytes("Owned_ReportFailure", &[255, b'"'], &[254, b'\n']);
    let baseline = r#"{"owned":true}"#;
    let failed = append_returned_failure(baseline.to_owned(), Err(cause));
    let following = append_returned_failure(baseline.to_owned(), Ok(Vec::new()));
    assert!(
        failed.contains("\"what_hex\":\"ff22\""),
        "actual {failed}; expected unchanged message bytes"
    );
    assert!(
        failed.contains("\"stack_hex\":\"fe0a\""),
        "actual {failed}; expected unchanged stack bytes"
    );
    assert_eq!(
        following, baseline,
        "expected following observation to remain publishable"
    );
}
