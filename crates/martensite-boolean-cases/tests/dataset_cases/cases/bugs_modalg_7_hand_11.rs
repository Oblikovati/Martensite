//! `bugs_modalg_7` bodies written by hand: cells, API checks and revolutions.
#![allow(unused_imports)]
use crate::support::bugs_commands::progress::tracked;
use crate::support::{
    bop_session::BopOptions,
    bugs_commands::{
        cells::ScriptCells,
        geometry::{
            circle_framed, ellipse_framed, frame_along, frame_at, line, mkedge_range, pcylinder_on,
            plane_at, psphere_on, trim_curve,
        },
        inspect::{
            bopapicheck, bopargcheck, bopcheck_level, bopcheck_text, checkmaxtol_value,
            checksection_count, compare, whatis,
        },
        options_bop::{ScriptBop, SectionFlags, bsection_with, generated, modified},
        pcurves::{checklength, pcurve_of_edge, pcurves_of_face, translate_pcurve},
        repair::{fixshape, settolerance},
        view::{donly, fit, smallview},
    },
    checkprops::{Expected, checkprops, checkprops_equal},
    checkshape::{checkshape, checkshape_named},
    checkview::{CheckView, checkview, screenshot},
    commands::{bound_shape, compound, explode, make_box, psphere, ttranslate},
    construction::{mkplane, revol, vertex, wire},
    nbshapes::{Counting, NbCount, checknbshapes, checknbshapes_like},
    one_step::{bcut, bsection},
    properties::{Prop, props},
    restore::restore,
    section_check::checksection_expecting,
    session::Session,
    surface_commands::{circle_along, mkedge, mkface, mkface_bounded, plane, polyline},
    tolerance_check::{checkmaxtol_ref, tolerance_report},
    two_step::TwoStepBoolean,
};
use martensite_tkbo::{
    bop_algo::{
        BOPAlgoBOP, BOPAlgoBuilder, BOPAlgoPaveFiller, BOPAlgoSection, BOPAlgoSplitter,
        BuilderHooks, Operation, perform_with_filler_progress,
    },
    brep_algo_api::{
        BRepAlgoAPIBuilderAlgo, BRepAlgoAPICheck, BRepAlgoAPICommon, BRepAlgoAPICut,
        BRepAlgoAPIFuse, BRepAlgoAPISection, BRepAlgoAPISplitter,
    },
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::List;
use martensite_tkg3d::top_abs::ShapeEnum;

fn view_of(s: &mut dyn Session, name: &str) {
    checkview(s, &CheckView::of(name));
}

/// The three cylinders of `bug31835`, then its stepwise common under `options`.
fn cylinders_stepwise(s: &mut dyn Session) {
    let first = frame_along(
        [-200., 2.22044604925033e-14, 12.6935294289015],
        [1., -1.11022302462516e-16, 0.],
    );
    pcylinder_on(s, "c1", first, 100., 400.);
    let second = frame_along([0., 0., -187.306470571099], [0., 0., 1.]);
    pcylinder_on(s, "c2", second, 100., 400.);
    let third = frame_along(
        [0., 200., 12.6935294289015],
        [0., -1., 1.11022302462516e-16],
    );
    pcylinder_on(s, "c3", third, 100., 400.);
}

fn common_step(
    s: &mut dyn Session,
    bop: &mut ScriptBop,
    (name, first, second): (&str, &str, &str),
    message: &str,
) {
    bop.direct(s, name, (first, second), Operation::Common);
    checkshape_named(s, name);
    if !bopcheck_text(s, name, &bop.options).contains("OK") {
        println!("{message}");
    }
    checkmaxtol_ref(s, name, "2.e-7");
}

fn cylinder_commons(s: &mut dyn Session, bop: &mut ScriptBop) {
    common_step(
        s,
        bop,
        ("r", "c1", "c2"),
        "Error: r is self-intersecting shape",
    );
    let faces = |count| {
        [
            NbCount::Wires(count),
            NbCount::Faces(count),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ]
    };
    checknbshapes(s, "r", Counting::Located, &faces(5));
    checkprops(s, "r", Prop::Area, Expected::Mass(160000.));
    checkprops(s, "r", Prop::Volume, Expected::Mass(5.33333e+06));
    common_step(
        s,
        bop,
        ("result", "r", "c3"),
        "Error: result is self-intersecting shape",
    );
    checknbshapes(s, "result", Counting::Located, &faces(12));
    checkprops(s, "result", Prop::Area, Expected::Mass(140589.));
    checkprops(s, "result", Prop::Volume, Expected::Mass(4.68629e+06));
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["c1", "c2", "c3"]);
    bop.bfillds();
}

fn cells_common_of_all(s: &mut dyn Session, bop: &mut ScriptBop, reset: bool) {
    let mut cells = ScriptCells::default();
    cells.bcbuild(s, bop, "r");
    cells.bcremoveall();
    cells.bcadd(s, "result1", &[("c1", true), ("c2", true), ("c3", true)], 0);
    if reset {
        bop.reset_options();
    }
    checkprops_equal(s, "result1", "result");
    checknbshapes_like(s, "result1", "result");
    checkmaxtol_ref(s, "result1", "2.e-7");
    view_of(s, "result");
}

pub fn bug31835_1(s: &mut dyn Session) {
    println!(
        "============================================================================================="
    );
    println!(
        "0031835: Modeling Algorithms - step by step Boolean common produces bad shape on given three cylinders"
    );
    println!(
        "============================================================================================="
    );
    println!();
    let mut bop = ScriptBop::default();
    cylinders_stepwise(s);
    cylinder_commons(s, &mut bop);
    cells_common_of_all(s, &mut bop, false);
}

pub fn bug31835_2(s: &mut dyn Session) {
    println!(
        "============================================================================================="
    );
    println!(
        "0031835: Modeling Algorithms - step by step Boolean common produces bad shape on given three cylinders"
    );
    println!(
        "============================================================================================="
    );
    println!();
    let mut bop = ScriptBop::default();
    cylinders_stepwise(s);
    bop.options.fuzzy = 1.0e-6;
    bop.options.non_destructive = true;
    bop.options.run_parallel = true;
    bop.options.fill_history = false;
    cylinder_commons(s, &mut bop);
    cells_common_of_all(s, &mut bop, true);
}

const NOT_ALLOWED: &str =
    "Error: Boolean operation of the given type is not allowed on the given inputs";

pub fn bug30386_5(s: &mut dyn Session) {
    println!(
        "REQUIRED All: Error: Boolean operation of the given type is not allowed on the given inputs"
    );
    println!("=======================================================");
    println!("0030386: Modeling Algorithms - Unable to perform Cut operation");
    println!("=======================================================");
    println!();
    vertex(s, "v", [0., 0., 2.]);
    let l = line([5., 5., 0.], [0., 0., 1.]);
    mkedge_range(s, "e", &l, -10., 10.);
    let p = plane_at([0., 0., 1.]);
    mkface_bounded(s, "f", &p, (-15., 15., -15., 15.));
    make_box(s, "b", [8., 8., 8.], [5., 5., 5.]);
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["v", "e", "f", "b"]);
    bop.bfillds();
    bop.bbuild(s, "s1");
    make_box(s, "b2", [0., 0., 0.], [10., 10., 10.]);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["f"]);
    bop.add_tools(s, &["b2"]);
    bop.bfillds();
    bop.bbuild(s, "s2");
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["s1"]);
    bop.add_tools(s, &["s2"]);
    bop.bfillds();
    if bop
        .bbop_answer(s, "r", Operation::Common)
        .contains(NOT_ALLOWED)
    {
        println!("Error: COMMON Operation must be allowed on any types of arguments");
    }
    for kind in [Operation::Fuse, Operation::Cut, Operation::Cut21] {
        if !bop.bbop_answer(s, "r", kind).contains(NOT_ALLOWED) {
            println!("Error: Operation must not be allowed");
        }
    }
}

pub fn bug28245(s: &mut dyn Session) {
    println!("========");
    println!("OCC28245");
    println!("========");
    println!();
    for (edge, wire_name, face, at) in [
        ("e1", "w1", "f1", [0., 0., 0.]),
        ("e2", "w2", "f2", [10., 0., 0.]),
        ("e3", "w3", "f3", [5., 9., 0.]),
    ] {
        let curve = circle_along(at, [0., 0., 1.], 10.);
        mkedge(s, edge, &curve);
        wire(s, wire_name, &[edge]);
        mkplane(s, face, wire_name);
    }
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["f1", "f2", "f3"]);
    bop.bfillds();
    let mut cells = ScriptCells::default();
    cells.bcbuild(s, &mut bop, "rx");
    cells.bcadd(s, "res", &[("f1", true)], 1);
    cells.bcadd(s, "res", &[("f2", true), ("f3", false)], 1);
    cells.bcadd(s, "res", &[("f3", true), ("f1", false)], 2);
    let options = BopOptions::default();
    bopcheck_text(s, "res", &options);
    if !bopcheck_text(s, "res", &options).contains("This shape seems to be OK") {
        println!("Error: Result of Cells Builder algorithm is invalid");
    }
    view_of(s, "res");
    cells.bcremoveint(s, "res");
    if !bopcheck_text(s, "res", &options).contains("This shape seems to be OK") {
        println!(
            "Error: Result of Cells Builder algorithm becomes invalid after removal of internal boundaries on faces"
        );
    }
    view_of(s, "res");
}

fn revolved_is_valid(s: &mut dyn Session, name: &str, degrees: f64) {
    crate::support::bugs_commands::repair::revol_copy(s, name, "e", degrees);
    if s.shape(name).is_none() {
        println!("Error: algorithm did not create shape");
        return;
    }
    let valid = checkshape(&bound_shape(s, name)).text.contains("valid");
    if valid && bopcheck_level(s, name, &BopOptions::default(), 4).contains("OK") {
        println!("OK: valid shape is created");
    } else {
        println!("Error: algorithm created wrong shape");
    }
}

pub fn bug31460(s: &mut dyn Session) {
    println!("========");
    println!("0031460: Modeling Algorithms - Regression: Revolution not done.");
    println!("========");
    println!();
    let circle = circle_framed([3., 2.9999999999989, 0.], [0., 0., -1.], [-1., 0., 0.], 3.);
    let trimmed = trim_curve(&circle, 4.71238898038469, 6.28318530717959);
    mkedge(s, "e", &trimmed);
    revolved_is_valid(s, "r10", 10.);
    revolved_is_valid(s, "r360", 360.);
}

/// One `if {![regexp text [bopapicheck ...]]} {puts message}`.
fn api_check(
    s: &dyn Session,
    shapes: (&str, Option<&str>),
    kind: Option<Operation>,
    tests: [bool; 2],
    text: &str,
    message: &str,
) {
    if !bopapicheck(s, shapes, kind, tests).contains(text) {
        println!("{message}");
    }
}

pub fn bug29484(s: &mut dyn Session) {
    println!("========");
    println!("OCC29484");
    println!("========");
    println!();
    println!("check simple box");
    let all = [true, true];
    const INVALID: &str = "Error: The check is invalid";
    const TYPES: &str = "Error: The check on types is invalid";
    make_box(s, "b1", [0., 0., 0.], [2., 2., 2.]);
    api_check(
        s,
        ("b1", None),
        None,
        all,
        "The shape seems to be valid",
        "Error: The simple box is considered as invalid by BRepAlgoAPICheck",
    );
    println!("add another box");
    make_box(s, "b2", [1., 1., 1.], [2., 2., 2.]);
    compound(s, &["b1", "b2"], "c");
    println!("check si compound");
    api_check(
        s,
        ("c", None),
        None,
        all,
        "The shape is invalid",
        "Error: Self-intersection is not detected",
    );
    api_check(
        s,
        ("c", None),
        None,
        [true, false],
        "The shape seems to be valid",
        "Error: Check on self-intersection is still enabled",
    );
    println!("increase tolerance of the vertices of the box to test Small edges detection");
    for part in explode(s, "b1", Some(ShapeEnum::Vertex)) {
        settolerance(s, &part, 5.);
    }
    api_check(
        s,
        ("b1", None),
        None,
        [true, false],
        "The shape is invalid",
        "Error: Small edges are not detected",
    );
    api_check(
        s,
        ("c", None),
        None,
        [false, false],
        "The shape seems to be valid",
        "Error: Check on small edges is still enabled",
    );
    println!("b1 - invalid, b2 - ok");
    api_check(
        s,
        ("b1", Some("b2")),
        None,
        all,
        "The first shape is invalid",
        INVALID,
    );
    api_check(
        s,
        ("b2", Some("b1")),
        None,
        all,
        "The second shape is invalid",
        INVALID,
    );
    println!("test types");
    api_check(
        s,
        ("b1", Some("b2")),
        Some(Operation::Fuse),
        [false, false],
        "The shapes seem to be valid",
        "Error: The check on types is invalid",
    );
    println!("check empty shape");
    compound(s, &[], "c");
    api_check(
        s,
        ("c", None),
        None,
        all,
        "The shape is empty",
        "Error: Empty shape is not detected",
    );
    api_check(
        s,
        ("b2", Some("c")),
        None,
        all,
        "The shapes are not valid for Boolean operation",
        "Error: Empty shape is not detected",
    );
    println!("check vertex/solid BOPs");
    vertex(s, "v", [1., 1., 1.]);
    api_check(
        s,
        ("b2", Some("v")),
        Some(Operation::Common),
        all,
        "The shapes seem to be valid",
        TYPES,
    );
    api_check(
        s,
        ("b2", Some("v")),
        Some(Operation::Fuse),
        all,
        "The shapes are not valid for Boolean operation",
        TYPES,
    );
    api_check(
        s,
        ("b2", Some("v")),
        Some(Operation::Cut),
        all,
        "The shapes are not valid for Boolean operation",
        TYPES,
    );
    api_check(
        s,
        ("b2", Some("v")),
        Some(Operation::Cut21),
        all,
        "The shapes seem to be valid",
        TYPES,
    );
}
