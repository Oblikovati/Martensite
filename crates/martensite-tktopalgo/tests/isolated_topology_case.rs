//! Run a topology case alone in a child process under the bugs group budget.
#[path = "common/isolated_case.rs"]
mod isolated_case;
use std::error::Error;

/// Run an unchanged case body, e.g. `run_isolated_case("cases::point", || Ok(()))`.
pub fn run_isolated_case(identity: &str, body: impl FnOnce() -> Result<(), Box<dyn Error>>) {
    isolated_case::run_isolated(identity, 1000, || {
        body().unwrap_or_else(|error| panic!("topology case {identity}: {error}"));
    });
}
