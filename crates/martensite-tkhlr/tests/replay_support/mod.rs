//! Single owned request execution and exact token comparison for ordinary tests.
#[allow(dead_code)]
#[path = "../common/foundation_replay/mod.rs"]
pub mod replay;
#[path = "../common/foundation_replay/replay_compare.rs"]
pub mod replay_compare;
pub fn check_request_slice(input: &str, expected: &str) {
    check_request_slice_with(input, expected, replay::execute_request);
}

pub fn check_request_slice_with(
    input: &str,
    expected: &str,
    execute: impl FnOnce(&str) -> Option<String>,
) {
    let actual = execute(input).unwrap();
    assert_eq!(
        replay_compare::canonical_nan_tokens(&actual),
        replay_compare::canonical_nan_tokens(expected),
        "request: {input}"
    );
}
