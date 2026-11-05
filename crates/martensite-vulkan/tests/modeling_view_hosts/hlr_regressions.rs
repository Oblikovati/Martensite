//! Hidden-line regressions of the exact and polygonal grids, each in an isolated worker.
use super::*;

use hidden_lines::HiddenLineAlgorithm::{Exact, Polygonal};
use hlr_cases::{HlrCase, HlrInput::*, hidden_line_view};
use martensite_tkv3d::v3d::V3dTypeOfOrientation as O;

/// The hidden-line group runs every regression under a 1000 s CPU limit.
fn run(identity: &str, case: HlrCase) {
    case_v3d_process_case::run_case_within(identity, 1000, move || hidden_line_view(&case));
}

/// A regression whose command-host script declares `TODO` lines: it must fail exactly as expected.
fn run_todo(identity: &str, todos: &[&'static str], case: HlrCase) {
    case_todo::run_case_expecting(identity, 1000, todos, move || hidden_line_view(&case));
}

#[test]
fn exact_d1() {
    let case = HlrCase {
        heading: &["OCC25908"],
        input: Dataset("bug25908_hlr-bspline-clone2-tcl-h1.brep"),
        orientation: None,
        algorithm: Exact,
        length: 1051.33,
        image: "D1.png",
    };
    run("hlr_regressions::exact_d1", case);
}

#[test]
fn exact_d2() {
    let case = HlrCase {
        heading: &["OCC25908"],
        input: Dataset("bug25908_hlr-bspline-clone2-tcl-h1.brep"),
        orientation: Some(O::ZUP_TOP),
        algorithm: Exact,
        length: 385.563,
        image: "D2.png",
    };
    run("hlr_regressions::exact_d2", case);
}

#[test]
fn exact_d3() {
    let case = HlrCase {
        heading: &["OCC25908"],
        input: Dataset("bug25908_hlr-bspline-clone2-tcl-h1.brep"),
        orientation: Some(O::ZUP_BOTTOM),
        algorithm: Exact,
        length: 193.345,
        image: "D3.png",
    };
    run("hlr_regressions::exact_d3", case);
}

#[test]
fn exact_d4() {
    let case = HlrCase {
        heading: &["OCC25908"],
        input: Dataset("bug25908_hlr-bspline-clone2-tcl-h1.brep"),
        orientation: Some(O::ZUP_FRONT),
        algorithm: Exact,
        length: 484.427,
        image: "D4.png",
    };
    run("hlr_regressions::exact_d4", case);
}

#[test]
fn exact_d5() {
    let case = HlrCase {
        heading: &["OCC25908"],
        input: Dataset("bug25908_hlr-bspline-clone2-tcl-h1.brep"),
        orientation: Some(O::ZUP_BACK),
        algorithm: Exact,
        length: 544.507,
        image: "D5.png",
    };
    run("hlr_regressions::exact_d5", case);
}

#[test]
fn exact_d6() {
    let case = HlrCase {
        heading: &["OCC25908"],
        input: Dataset("bug25908_hlr-bspline-clone2-tcl-h1.brep"),
        orientation: Some(O::ZUP_LEFT),
        algorithm: Exact,
        length: 577.935,
        image: "D6.png",
    };
    run("hlr_regressions::exact_d6", case);
}

#[test]
fn exact_d7() {
    let case = HlrCase {
        heading: &["OCC25908"],
        input: Dataset("bug25908_hlr-bspline-clone2-tcl-h1.brep"),
        orientation: Some(O::ZUP_RIGHT),
        algorithm: Exact,
        length: 554.265,
        image: "D7.png",
    };
    run("hlr_regressions::exact_d7", case);
}

#[test]
fn poly_d1() {
    let case = HlrCase {
        heading: &["OCC25908"],
        input: Dataset("bug25908_hlr-bspline-clone2-tcl-h1.brep"),
        orientation: None,
        algorithm: Polygonal,
        length: 1032.95,
        image: "D1.png",
    };
    run("hlr_regressions::poly_d1", case);
}

#[test]
fn poly_d2() {
    let case = HlrCase {
        heading: &["OCC25908"],
        input: Dataset("bug25908_hlr-bspline-clone2-tcl-h1.brep"),
        orientation: Some(O::ZUP_TOP),
        algorithm: Polygonal,
        length: 402.413,
        image: "D2.png",
    };
    run("hlr_regressions::poly_d2", case);
}

#[test]
fn poly_d3() {
    let case = HlrCase {
        heading: &["OCC25908"],
        input: Dataset("bug25908_hlr-bspline-clone2-tcl-h1.brep"),
        orientation: Some(O::ZUP_BOTTOM),
        algorithm: Polygonal,
        length: 210.378,
        image: "D3.png",
    };
    run("hlr_regressions::poly_d3", case);
}

#[test]
fn poly_d4() {
    let case = HlrCase {
        heading: &["OCC25908"],
        input: Dataset("bug25908_hlr-bspline-clone2-tcl-h1.brep"),
        orientation: Some(O::ZUP_FRONT),
        algorithm: Polygonal,
        length: 453.641,
        image: "D4.png",
    };
    run("hlr_regressions::poly_d4", case);
}

#[test]
fn poly_d5() {
    let case = HlrCase {
        heading: &["OCC25908"],
        input: Dataset("bug25908_hlr-bspline-clone2-tcl-h1.brep"),
        orientation: Some(O::ZUP_BACK),
        algorithm: Polygonal,
        length: 475.129,
        image: "D5.png",
    };
    run("hlr_regressions::poly_d5", case);
}

#[test]
fn poly_d6() {
    let case = HlrCase {
        heading: &["OCC25908"],
        input: Dataset("bug25908_hlr-bspline-clone2-tcl-h1.brep"),
        orientation: Some(O::ZUP_LEFT),
        algorithm: Polygonal,
        length: 548.613,
        image: "D6.png",
    };
    run("hlr_regressions::poly_d6", case);
}

#[test]
fn poly_d7() {
    let case = HlrCase {
        heading: &["OCC25908"],
        input: Dataset("bug25908_hlr-bspline-clone2-tcl-h1.brep"),
        orientation: Some(O::ZUP_RIGHT),
        algorithm: Polygonal,
        length: 468.174,
        image: "D7.png",
    };
    run("hlr_regressions::poly_d7", case);
}

#[test]
fn exact_bug25813_1() {
    let case = HlrCase {
        heading: &["OCC25813: regression in Hidden Line Removal"],
        input: FusedCylinders,
        orientation: None,
        algorithm: Exact,
        length: 204.19,
        image: "bug25813_1.png",
    };
    run("hlr_regressions::exact_bug25813_1", case);
}

#[test]
fn exact_bug25813_2() {
    let case = HlrCase {
        heading: &["OCC25813: regression in Hidden Line Removal"],
        input: Dataset("bug25813_hlr-bus1-draw-Fusion001.brep"),
        orientation: None,
        algorithm: Exact,
        length: 3053.49,
        image: "bug25813_2.png",
    };
    run("hlr_regressions::exact_bug25813_2", case);
}

#[test]
fn exact_bug25813_3() {
    let case = HlrCase {
        heading: &["OCC25813: regression in Hidden Line Removal"],
        input: Torus,
        orientation: None,
        algorithm: Exact,
        length: 302.685,
        image: "bug25813_3.png",
    };
    run("hlr_regressions::exact_bug25813_3", case);
}

#[test]
fn exact_bug25813_4() {
    let case = HlrCase {
        heading: &["OCC25813: regression in Hidden Line Removal"],
        input: Dataset("bug25813_hlr-test_normandc1m1-draw-Cut001.brep"),
        orientation: None,
        algorithm: Exact,
        length: 1197.37,
        image: "bug25813_4.png",
    };
    run("hlr_regressions::exact_bug25813_4", case);
}

#[test]
fn exact_bug25813_5() {
    let case = HlrCase {
        heading: &["OCC25813: regression in Hidden Line Removal"],
        input: Dataset("bug25813_hlr-fillet2-tcl-f.brep"),
        orientation: None,
        algorithm: Exact,
        length: 1284.73,
        image: "bug25813_5.png",
    };
    run("hlr_regressions::exact_bug25813_5", case);
}

#[test]
fn poly_bug25813_1() {
    let case = HlrCase {
        heading: &["OCC25813: regression in Hidden Line Removal"],
        input: FusedCylinders,
        orientation: None,
        algorithm: Polygonal,
        length: 204.02,
        image: "bug25813_1.png",
    };
    run("hlr_regressions::poly_bug25813_1", case);
}

#[test]
fn poly_bug25813_2() {
    let case = HlrCase {
        heading: &["OCC25813: regression in Hidden Line Removal"],
        input: Dataset("bug25813_hlr-bus1-draw-Fusion001.brep"),
        orientation: None,
        algorithm: Polygonal,
        length: 3025.49,
        image: "bug25813_2.png",
    };
    // TODO OCC30286 Linux
    let todos = ["Error : The length of result shape is 3025.4"];
    run_todo("hlr_regressions::poly_bug25813_2", &todos, case);
}

#[test]
fn poly_bug25813_3() {
    let case = HlrCase {
        heading: &["OCC25813: regression in Hidden Line Removal"],
        input: Torus,
        orientation: None,
        algorithm: Polygonal,
        length: 301.999,
        image: "bug25813_3.png",
    };
    // TODO OCC30286 Linux
    let todos = ["Error : The length of result shape is 302"];
    run_todo("hlr_regressions::poly_bug25813_3", &todos, case);
}

#[test]
fn poly_bug25813_4() {
    let case = HlrCase {
        heading: &["OCC25813: regression in Hidden Line Removal"],
        input: Dataset("bug25813_hlr-test_normandc1m1-draw-Cut001.brep"),
        orientation: None,
        algorithm: Polygonal,
        length: 1196.29,
        image: "bug25813_4.png",
    };
    run("hlr_regressions::poly_bug25813_4", case);
}

#[test]
fn poly_bug25813_5() {
    let case = HlrCase {
        heading: &["OCC25813: regression in Hidden Line Removal"],
        input: Dataset("bug25813_hlr-fillet2-tcl-f.brep"),
        orientation: None,
        algorithm: Polygonal,
        length: 1271.98,
        image: "bug25813_5.png",
    };
    run("hlr_regressions::poly_bug25813_5", case);
}

#[test]
fn exact_bug32747() {
    let case = HlrCase {
        heading: &["OCC32747: Exact HLR algorithm gives wrong result"],
        input: Dataset("bug32747.brep"),
        orientation: Some(O::ZUP_FRONT),
        algorithm: Exact,
        length: 34.3597,
        image: "bug32747.png",
    };
    run("hlr_regressions::exact_bug32747", case);
}

#[test]
fn poly_bug28242() {
    let case = HlrCase {
        heading: &["0028242: [Regression] HLR Poly Algo has broken edges"],
        input: Dataset("bug28242.brep"),
        orientation: None,
        algorithm: Polygonal,
        length: 167.756,
        image: "bug28242.png",
    };
    run("hlr_regressions::poly_bug28242", case);
}
