//! Modelling regressions of the bugs group shown in a 3D view whose operations take the
//! native release build's time, each in an isolated worker under the bugs policy. The
//! harness builds this host optimized (`"cargo_profile": "release"`); in an unoptimized
//! build the 121 x 100 argument lists take minutes, so the cases exist only in optimized
//! builds.
use super::*;

use modalg_display_release_cases::{ArgumentListOperation as Op, argument_list_view};

fn run(identity: &str, operation: Op, counts: [usize; 9], image: &'static str) {
    case_bugs_process_case::run_bugs_case(identity, || {
        argument_list_view(operation, counts, image)
    });
}

const BUG25614_COMMON: [usize; 9] = [1720, 4260, 3220, 3220, 760, 760, 0, 1, 13941];

#[test]
fn bug25614_common() {
    run(
        "modalg_display_release_regressions::bug25614_common",
        Op::Common,
        BUG25614_COMMON,
        "bug25614_common.png",
    );
}

#[test]
fn bug25614_fuse() {
    let counts = [1808, 3392, 1586, 1586, 1, 1, 0, 1, 8375];
    run(
        "modalg_display_release_regressions::bug25614_fuse",
        Op::Fuse,
        counts,
        "bug25614_fuse.png",
    );
}

#[test]
fn bug25614_cut() {
    let counts = [888, 1412, 726, 726, 121, 121, 0, 1, 3995];
    run(
        "modalg_display_release_regressions::bug25614_cut",
        Op::Cut,
        counts,
        "bug25614_cut.png",
    );
}

const BUG25614_CUT21: [usize; 9] = [2640, 5960, 3720, 3720, 560, 560, 0, 1, 17161];

#[test]
fn bug25614_cut21() {
    run(
        "modalg_display_release_regressions::bug25614_cut21",
        Op::Cut21,
        BUG25614_CUT21,
        "bug25614_cut21.png",
    );
}

#[test]
fn bug25614_section() {
    let counts = [2700, 6000, 0, 0, 0, 0, 0, 1, 8701];
    run(
        "modalg_display_release_regressions::bug25614_section",
        Op::Section,
        counts,
        "bug25614_section.png",
    );
}

const BUG25614_GENFUSE: [usize; 9] = [2728, 7032, 5746, 5746, 1441, 1441, 0, 1, 24135];

#[test]
fn bug25614_genfuse() {
    run(
        "modalg_display_release_regressions::bug25614_genfuse",
        Op::GeneralFuse,
        BUG25614_GENFUSE,
        "bug25614_genfuse.png",
    );
}
