//! Exact original interval, interference and transition requests as direct owned Rust tests.
#[path = "replay_support/mod.rs"]
mod replay_support;
pub(super) use replay_support::{check_request_slice, replay};
use std::ops::Range;
const REQUESTS: &str = include_str!("fixtures/foundation_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/foundation_numeric/reference-expected.txt");

fn run_case(request: Range<usize>, expected: Range<usize>) {
    check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}

#[test]
fn original_foundation_request_and_expected_counts_are_exact() {
    assert_eq!(REQUESTS.lines().count(), 1261);
    assert_eq!(EXPECTATIONS.lines().count(), 1261);
}

#[test]
fn pure_foundation_dispatch_has_no_unknown_or_empty_operation() {
    assert_eq!(replay::execute_request("unknown_operation 0"), None);
    assert_eq!(replay::execute_request(" \t"), None);
}

#[path = "foundation_numeric/iv_edit_00.rs"]
mod iv_edit_00;

#[path = "foundation_numeric/iv_edit_01.rs"]
mod iv_edit_01;

#[path = "foundation_numeric/iv_edit_02.rs"]
mod iv_edit_02;

#[path = "foundation_numeric/iv_interval_00.rs"]
mod iv_interval_00;

#[path = "foundation_numeric/iv_interval_01.rs"]
mod iv_interval_01;

#[path = "foundation_numeric/iv_relate_00.rs"]
mod iv_relate_00;

#[path = "foundation_numeric/iv_relate_01.rs"]
mod iv_relate_01;

#[path = "foundation_numeric/iv_relate_02.rs"]
mod iv_relate_02;

#[path = "foundation_numeric/iv_relate_03.rs"]
mod iv_relate_03;

#[path = "foundation_numeric/iv_set_00.rs"]
mod iv_set_00;

#[path = "foundation_numeric/iv_set_01.rs"]
mod iv_set_01;

#[path = "foundation_numeric/iv_set_02.rs"]
mod iv_set_02;

#[path = "foundation_numeric/iv_set_03.rs"]
mod iv_set_03;

#[path = "foundation_numeric/iv_set_04.rs"]
mod iv_set_04;

#[path = "foundation_numeric/iv_set_05.rs"]
mod iv_set_05;

#[path = "foundation_numeric/iv_value_00.rs"]
mod iv_value_00;

#[path = "foundation_numeric/tb_interference_00.rs"]
mod tb_interference_00;

#[path = "foundation_numeric/tc_transition_00.rs"]
mod tc_transition_00;

#[path = "foundation_numeric/tc_transition_01.rs"]
mod tc_transition_01;

#[path = "foundation_numeric/tc_transition_02.rs"]
mod tc_transition_02;
