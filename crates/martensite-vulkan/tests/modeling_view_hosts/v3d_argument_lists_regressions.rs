//! Full argument-list Boolean originals use genuine 121-box/100-cylinder operations.
use super::*;

use v3d_argument_lists_cases as cases;

#[test]
fn bug25614_common() {
    case_bugs_process_case::run_bugs_case(
        "v3d_argument_lists_regressions::bug25614_common",
        || {
            cases::argument_list_view(
                cases::ArgumentListOperation::Common,
                [1720, 4260, 3220, 3220, 760, 760, 0, 1, 13941],
                "bug25614_common.png",
            )
        },
    );
}

#[test]
fn bug25614_cut21() {
    case_bugs_process_case::run_bugs_case("v3d_argument_lists_regressions::bug25614_cut21", || {
        cases::argument_list_view(
            cases::ArgumentListOperation::Cut21,
            [2640, 5960, 3720, 3720, 560, 560, 0, 1, 17161],
            "bug25614_cut21.png",
        )
    });
}

#[test]
fn bug25614_genfuse() {
    case_bugs_process_case::run_bugs_case(
        "v3d_argument_lists_regressions::bug25614_genfuse",
        || {
            cases::argument_list_view(
                cases::ArgumentListOperation::GeneralFuse,
                [2728, 7032, 5746, 5746, 1441, 1441, 0, 1, 24135],
                "bug25614_genfuse.png",
            )
        },
    );
}
