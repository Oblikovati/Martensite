//! Three complete selection-query regressions reuse their bodies under the CPU300 lifecycle.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "support/selection_occurrence_rank.rs"]
mod selection_occurrence_rank;
mod vselect_clipping_cases {
    pub(crate) use crate::selection_occurrence_rank::occurrence_rank;
}
#[path = "vselect_bugs_cases/selection_queries/mod.rs"]
mod selection_queries;

#[test]
fn bug25627() {
    case_v3d_process_case::run_case_within(
        "bug25627",
        300,
        selection_queries::selected_topology_roundtrip,
    );
}

#[test]
fn bug26779() {
    case_v3d_process_case::run_case_within(
        "bug26779",
        300,
        selection_queries::local_and_neutral_rows,
    );
}

#[test]
fn bug27848() {
    case_v3d_process_case::run_case_within(
        "bug27848",
        300,
        selection_queries::line_shape_discrimination,
    );
}
