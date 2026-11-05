//! Four operation consumers use the complete canonical modelling tree and ordinary Bugs workers.
use super::*;

use modalg_display_cases as cases;

#[test]
fn bug23378_1() {
    case_bugs_process_case::run_bugs_case("modalg_tool_cuts_regressions::bug23378_1", || {
        cases::blower_cut_view(1, 864274., "bug23378_1.png")
    });
}

#[test]
fn bug29807_b2() {
    case_bugs_process_case::run_bugs_case("modalg_tool_cuts_regressions::bug29807_b2", || {
        cases::tool_cut_view(&cases::tool_cut_spec(cases::SourceCut::Tilted))
    });
}

#[test]
fn bug29807_b4a() {
    case_bugs_process_case::run_bugs_case_expecting(
        "modalg_tool_cuts_regressions::bug29807_b4a",
        cases::TOOL_CUT_TODOS,
        || {
            cases::tool_cut_view_with_todos(
                &cases::tool_cut_spec(cases::SourceCut::RaisedTodo),
                cases::TOOL_CUT_TODOS,
            )
        },
    );
}

#[test]
fn bug29807_b4b() {
    case_bugs_process_case::run_bugs_case("modalg_tool_cuts_regressions::bug29807_b4b", || {
        cases::tool_cut_view(&cases::tool_cut_spec(cases::SourceCut::Tightened))
    });
}
