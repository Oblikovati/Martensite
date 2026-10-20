//! Run a primitive case alone in a child process under the bugs group budget.
#[path = "common/isolated_case.rs"]
mod isolated_case;

/// Execute a deferred primitive body; `run_primitive_case("cases::box", || assert_eq!(2, 2))`.
pub fn run_primitive_case(identity: &str, body: fn()) {
    isolated_case::run_isolated(identity, 1000, body);
}
