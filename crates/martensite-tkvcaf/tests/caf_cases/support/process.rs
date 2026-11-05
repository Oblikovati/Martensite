//! Runs each case on the shared case contract and grades its stream with this group's rules.
use martensite_boolean_cases::support::{
    process::case_stream,
    raster_session::KEEP_IMAGES_ENV,
    rules::{Rule, grade},
};

/// A run with this set keeps each case's stream and image in that folder, for comparison.
pub const KEEP_ARTIFACTS_ENV: &str = KEEP_IMAGES_ENV;

/// Runs `body` as the libtest case `identity` and panics unless its stream earns a passing verdict.
/// Example: `run_case("bugs::a", 1000, BUG_RULES, || println!("TEST COMPLETED"))`.
pub fn run_case(identity: &str, cpu_seconds: i64, rules: &[Rule], body: impl FnOnce()) {
    let stream = case_stream(identity, cpu_seconds, body)
        .unwrap_or_else(|failure| panic!("case {identity}: {failure}"));
    let Some(stream) = stream else {
        return;
    };
    let verdict = grade(&stream, rules);
    assert!(
        verdict.passes(),
        "case {identity}: {verdict:?}; expected Ok or KnownProblem\n--- stream ---\n{stream}"
    );
}
