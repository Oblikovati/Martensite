//! The perf group precedes the common source rules for completed command-report streams.
use crate::case_bop_fatal_rules::has_faulty_word;

/// Grade completed source reports with their exact line priority; `accepted_perf_bop_report("TEST COMPLETED\n")`.
pub(super) fn accepted_perf_bop_report(reports: &str) -> bool {
    // The caller owns CPU and completion checks; these rules never grant an exclusion.
    reports.lines().all(accepted_perf_line)
}

fn accepted_perf_line(line: &str) -> bool {
    if has_faulty_word(line) {
        return false;
    }
    if line.contains("Error: performance") {
        return true;
    }
    crate::case_common_bop_rules::accepted_common_bop_line(line)
}
