//! Recorded failures that the harness can still re-grade.
//!
//! A translated regression that is not admitted stays `#[ignore = "recorded failure: ..."]`.
//! The harness re-grades it with `<name> --exact --include-ignored`, but the isolated worker
//! transport starts its child with `--exact <identity>` only, and libtest skips an ignored test
//! there. The ignored test therefore names a companion worker test as the child identity; the
//! companion is an ordinary test that runs the regression only inside that child process.

/// The environment marker the isolated worker transport sets in its child process
/// (`MARTENSITE_EXACT_CASE_CHILD_V1` of the pinned process-policy crate). If the transport
/// renames it, the companion stays a no-op inside the child, the child writes no bound records
/// and the parent refuses the run, so a recorded failure can never pass by accident.
const CASE_CHILD_MARKER: &str = "MARTENSITE_EXACT_CASE_CHILD_V1";

/// Whether this process is an isolated worker child; `inside_case_worker()` is false in a
/// plain `cargo test` run.
pub(crate) fn inside_case_worker() -> bool {
    std::env::var_os(CASE_CHILD_MARKER).is_some()
}

/// A recorded failure and its worker companion. `$run` is the group's runner, called with the
/// worker identity first and then the listed arguments; the ignored test re-grades through the
/// companion. Example:
/// `recorded_case!(bug1, bug1_case_worker, "recorded failure: x", bugs(cases::b));`
macro_rules! recorded_case {
    ($name:ident, $worker:ident, $reason:literal, $run:ident($($arg:expr),* $(,)?)) => {
        #[test]
        #[ignore = $reason]
        fn $name() {
            $run(stringify!($worker) $(, $arg)*);
        }

        /// Worker companion of the recorded failure of the same name; a no-op outside the
        /// isolated worker child its ignored test starts.
        #[test]
        fn $worker() {
            if crate::recorded_case::inside_case_worker() {
                $run(stringify!($worker) $(, $arg)*);
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn plain_test_runs_are_not_case_workers() {
        assert!(
            !super::inside_case_worker(),
            "expected no {} in a plain run",
            super::CASE_CHILD_MARKER
        );
    }
}
