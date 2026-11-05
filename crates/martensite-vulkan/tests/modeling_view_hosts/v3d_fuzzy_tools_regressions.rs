//! Actual fuzzy prepared fuse of all five authored tool sectors.
use super::*;

#[test]
fn bug25880() {
    case_bugs_process_case::run_bugs_case(
        "v3d_fuzzy_tools_regressions::bug25880",
        v3d_fuzzy_tools::fuzzy_multi_tool_view,
    );
}
