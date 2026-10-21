//! Retained numeric requests replayed against frozen answers with direct Rust execution.
#[allow(dead_code)]
#[path = "common/helix_replay/mod.rs"]
mod replay;
#[path = "common/helix_replay/replay_compare.rs"]
mod replay_compare;
use std::ops::Range;

const REQUESTS: &str = include_str!("fixtures/retained_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/retained_numeric/reference-expected.txt");

fn run_case(request: Range<usize>, expected: Range<usize>) {
    let input = &REQUESTS[request];
    let actual = replay::execute_request(input).unwrap();
    let expected = &EXPECTATIONS[expected];
    assert_eq!(
        replay_compare::canonical_nan_tokens(&actual),
        replay_compare::canonical_nan_tokens(expected),
        "request: {input}"
    );
}

#[test]
fn frozen_corpus_has_exactly_the_original_request_and_result_counts() {
    assert_eq!(REQUESTS.lines().count(), 347);
    assert_eq!(EXPECTATIONS.lines().count(), 347);
}

#[test]
fn direct_dispatch_rejects_unknown_operations_and_preserves_empty_input() {
    assert_eq!(replay::execute_request(" \t").unwrap(), "");
    assert_eq!(
        replay::execute_request("unknown_operation 0"),
        Err("Unknown probe operation: unknown_operation".to_owned())
    );
}

#[path = "numeric_corpus/hx_appr_curve3d.rs"]
mod hx_appr_curve3d;

#[path = "numeric_corpus/hx_appr_helix.rs"]
mod hx_appr_helix;

#[path = "numeric_corpus/hx_coil.rs"]
mod hx_coil;

#[path = "numeric_corpus/hx_curve.rs"]
mod hx_curve;

#[path = "numeric_corpus/hx_helix.rs"]
mod hx_helix;

#[path = "numeric_corpus/hx_placement.rs"]
mod hx_placement;

#[path = "numeric_corpus/hxb_build.rs"]
mod hxb_build;
