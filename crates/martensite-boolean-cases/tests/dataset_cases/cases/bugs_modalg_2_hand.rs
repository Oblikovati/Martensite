//! `tests/bugs/modalg_2`: a section, and Booleans of wires and edges against a shell. All but
//! `bug420` declare a known problem in two TODO lines, and native reports `BAD (known problem)`.
use crate::boolean_direct_section::direct_section;
use crate::{
    common_operation::{performed_direct, report_result_alerts},
    support::{
        checkprops::{Expected, checkprops},
        checkshape::checkshape,
        checkview::{CheckView, checkview},
        named_checks::{Counting, checknbshapes, checksection, distmini},
        named_shapes::{bound, explode_kind, renamevar, restore},
        properties::Prop,
        session::Session,
        two_step::TwoStepBoolean,
    },
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkernel::precision;
use martensite_tkg3d::top_abs::ShapeEnum;

macro_rules! brep {
    ($name:literal) => {
        include_bytes!(concat!(
            "../../../../../external/martensite-test-data/dataset-7.9.0/brep/",
            $name
        ))
    };
}

const SHELL: &[u8] = brep!("OCC240a.brep");
const EDGE2: &[u8] = brep!("case_2_edge2.brep");
const EDGE3: &[u8] = brep!("case_2_edge3.brep");
const WIRE1: &[u8] = brep!("case_2_wire1.brep");
const WIRE_A: &[u8] = brep!("case_9_wire_a.brep");
const WIRE_2: &[u8] = brep!("case_9_wire2.brep");
const NOT_VALID: &str = "TODO OCC12345 ALL: Error : The command is not valid";
const LENGTH: &str = "TODO OCC12345 ALL: Error : The length of result shape is";
const COUNT: &str = "TODO OCC12345 ALL: Error :  is WRONG because number of";

/// The scripts' last lines: the counts, the zero length, the shape, the section and the view.
fn checks(s: &mut dyn Session, vertices: usize, edges: usize) {
    let expected = [("VERTEX", vertices), ("EDGE", edges)];
    checknbshapes(s, "result", Counting::Default, &expected);
    checkprops(s, "result", Prop::Length, Expected::Mass(0.));
    checkshape(&bound(s, "result"));
    checksection(s, "result", None);
    checkview(s, &CheckView::of("result"));
}

/// The TODO lines, `restore`, `puts [bop a b]`, `puts [<op> result]`, then [`checks`].
fn known_problem(
    s: &mut dyn Session,
    todos: [&str; 2],
    inputs: [&[u8]; 2],
    kind: Operation,
    counts: [usize; 2],
) {
    println!("{}\n{}", todos[0], todos[1]);
    restore(s, "a", inputs[0]);
    restore(s, "b", inputs[1]);
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", kind);
    checks(s, counts[0], counts[1]);
}

pub fn bug238_2(s: &mut dyn Session) {
    println!("TODO OCC12345 ALL: Faulty : Result shape is WRONG !!!\n{LENGTH}");
    restore(s, "a", brep!("OCC238a.brep"));
    restore(s, "b", brep!("case_2_edge1.brep"));
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    explode_kind(s, "result", ShapeEnum::Vertex);
    explode_kind(s, "a", ShapeEnum::Edge);
    if distmini(s, "result_2", "a_3") > 1.0e-5 {
        println!("Faulty : Result shape is WRONG !!!");
    } else {
        println!(" Result shape is COMPLETE");
    }
    checks(s, 14, 7);
}

pub fn bug240_1(s: &mut dyn Session) {
    known_problem(
        s,
        [NOT_VALID, COUNT],
        [SHELL, EDGE2],
        Operation::Common,
        [14, 6],
    );
}

pub fn bug240_2(s: &mut dyn Session) {
    known_problem(
        s,
        [COUNT, LENGTH],
        [SHELL, EDGE2],
        Operation::Cut21,
        [14, 7],
    );
}

pub fn bug241_1(s: &mut dyn Session) {
    known_problem(
        s,
        [NOT_VALID, COUNT],
        [SHELL, EDGE3],
        Operation::Common,
        [6, 3],
    );
}

pub fn bug241_2(s: &mut dyn Session) {
    known_problem(s, [LENGTH, COUNT], [SHELL, EDGE3], Operation::Cut21, [6, 3]);
}

pub fn bug242_1(s: &mut dyn Session) {
    known_problem(
        s,
        [NOT_VALID, COUNT],
        [SHELL, WIRE1],
        Operation::Common,
        [18, 9],
    );
}

pub fn bug242_2(s: &mut dyn Session) {
    known_problem(
        s,
        [LENGTH, COUNT],
        [SHELL, WIRE1],
        Operation::Cut21,
        [18, 13],
    );
}

pub fn bug250_2(s: &mut dyn Session) {
    known_problem(
        s,
        [LENGTH, COUNT],
        [WIRE_A, WIRE_2],
        Operation::Cut21,
        [7, 6],
    );
}

pub fn bug250_3(s: &mut dyn Session) {
    known_problem(s, [LENGTH, COUNT], [WIRE_A, WIRE_2], Operation::Cut, [4, 2]);
}

pub fn bug250_4(s: &mut dyn Session) {
    known_problem(
        s,
        [LENGTH, COUNT],
        [WIRE_A, WIRE_2],
        Operation::Fuse,
        [9, 9],
    );
}

/// `bcut result a b`: one command, with its own intersections.
fn direct_cut(s: &mut dyn Session, name: &str, first: &str, second: &str) {
    let (first, second) = (bound(s, first), bound(s, second));
    let operation = performed_direct(&first, &second, Operation::Cut, precision::confusion());
    report_result_alerts(operation.options());
    if !operation.options().has_errors() {
        s.bind(name, operation.shape());
    }
}

pub fn bug420(s: &mut dyn Session) {
    println!("REQUIRED All: Faulty shapes in variables faulty_1 to faulty_2");
    restore(s, "a", brep!("CTO909_objects.brep"));
    checkshape(&bound(s, "a"));
    restore(s, "b", brep!("CTO909_tool_4.brep"));
    checkshape(&bound(s, "b"));
    let section = direct_section(&bound(s, "a"), &bound(s, "b"));
    s.bind("sec", &section);
    // `puts [checksection sec]`: the command's report, then the same text as its value.
    checksection(s, "sec", None);
    checksection(s, "sec", None);
    checkshape(&bound(s, "sec"));
    explode_kind(s, "a", ShapeEnum::Solid);
    renamevar(s, "a_1", "a");
    direct_cut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(135.092));
    checkshape(&bound(s, "result"));
    checkview(s, &CheckView::of("result"));
}
