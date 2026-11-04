//! Whole z-layer depth-clear selection regression with the actual viewer and isolated CPU300 worker.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "support/selection_occurrence_rank.rs"]
mod selection_occurrence_rank;
mod vselect_clipping_cases {
    pub(crate) use crate::selection_occurrence_rank::occurrence_rank;
}
#[allow(dead_code, unused_imports)]
// The exact shared query tree includes sibling case helpers; no sibling original wrapper is registered.
#[path = "vselect_bugs_cases/selection_queries/mod.rs"]
mod selection_queries;
mod vselect_zlayer;

#[test]
fn bug27797() {
    case_v3d_process_case::run_case_within("bug27797", 300, vselect_zlayer::complete_zlayer);
}
