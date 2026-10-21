//! The tests/bugs/heal cases of ShapeUpgrade_UnifySameDomain (`unifysamedom`).
use super::banner;
use crate::support::{
    checkshape::checkshape,
    log,
    logged_checks::checknbshapes_some,
    props::{Prop, checkprops, checkprops_equal},
    restore::restore_as,
    session::{Session, set},
    sewing::checkmaxtol_value,
    swdraw::unifysamedom,
};
use martensite_tkbrep::topods::Shape;

/// `set tolres [checkmaxtol result]` then `Error: bad tolerance of result` above `limit`.
fn check_tolerance_below(result: &Shape, limit: f64) {
    if checkmaxtol_value(result, "result") > limit {
        log::puts("Error: bad tolerance of result");
    }
}

/// `checknbshapes result` with the counts the script gives, in VERTEX..SHAPE order
/// (vertex, edge, wire, face, shell, solid).
fn counts(
    vertex: usize,
    edge: usize,
    wire: Option<usize>,
    face: Option<usize>,
) -> [Option<usize>; 9] {
    let mut expected = [None; 9];
    expected[0] = Some(vertex);
    expected[1] = Some(edge);
    expected[2] = wire;
    expected[3] = face;
    expected
}

/// OCC28529: `unifysamedom result a +b` keeps 143 edges and vertices.
pub fn bug28529(s: &mut dyn Session) {
    banner(&["==========", "OCC28529", "==========", ""]);
    let a = restore_as(s, "bug28529_face_to_unify.brep", "a").shape();
    let result = set(s, "result", unifysamedom(&a, &[], &["+b"]));
    let mut expected = [None; 9];
    expected[0] = Some(143);
    expected[1] = Some(143);
    crate::support::logged_checks::checknbshapes(&result, expected, None);
}

/// OCC29504: unification of a cylinder or sphere split into faces.
fn bug29504(s: &mut dyn Session, file: &str, area: f64) {
    let rule = "====================================================================================================";
    banner(&[
        rule,
        "OCC29504: ShapeUpgrade_UnifySameDomain produces invalid shape and ShapeFix_Shape is unable to fix it",
        rule,
        "",
    ]);
    let shape = restore_as(s, file, "s").shape();
    let result = set(s, "result", unifysamedom(&shape, &[], &[]));
    checkshape(&result);
    checkshape(&shape);
    checknbshapes_some(&result, counts(4, 4, Some(1), Some(1)));
    check_tolerance_below(&result, 1.000008e-07);
    checkprops(&result, Prop::Area, area);
}

pub fn bug29504_1(s: &mut dyn Session) {
    bug29504(s, "bug29504_cyl.brep", 104.72);
}

pub fn bug29504_2(s: &mut dyn Session) {
    bug29504(s, "bug29504_sph.brep", 55.1286);
}

/// OCC29544: unifying edges keeps the length (`checkprops result -equal ce`).
pub fn bug29544_1(s: &mut dyn Session) {
    banner(&["========", "OCC29544", "========", ""]);
    let ce = restore_as(s, "bug29544_edges.brep", "ce").shape();
    let result = set(s, "result", unifysamedom(&ce, &[], &[]));
    checkshape(&result);
    checkprops_equal(&result, &ce);
    let mut expected = [None; 9];
    expected[0] = Some(4);
    expected[1] = Some(2);
    checknbshapes_some(&result, expected);
}

/// OCC30158: the unified box keeps its 6 faces and its volume.
fn bug30158(s: &mut dyn Session, file: &str) {
    let rule =
        "===================================================================================";
    banner(&[
        rule,
        "OCC30158: Shape Healing - Incorrect result produced by ShapeUpgrade_UnifySameDomain",
        rule,
        "",
    ]);
    let shape = restore_as(s, file, "s").shape();
    let result = set(s, "result", unifysamedom(&shape, &[], &[]));
    checkshape(&result);
    checkshape(&shape);
    let mut expected = counts(8, 12, Some(6), Some(6));
    expected[4] = Some(1);
    expected[5] = Some(1);
    checknbshapes_some(&result, expected);
    check_tolerance_below(&result, 1.51e-07);
    checkprops(&result, Prop::Volume, 48.3068);
}

pub fn bug30158_1(s: &mut dyn Session) {
    bug30158(s, "bug30158_1.brep");
}

pub fn bug30158_2(s: &mut dyn Session) {
    bug30158(s, "bug30158_2.brep");
}

/// OCC30174: the unified cylinder.
pub fn bug30174(s: &mut dyn Session) {
    banner(&["============", "OCC30174", "============", ""]);
    let a = restore_as(s, "bug30174_cylinder.brep", "a").shape();
    let result = set(s, "result", unifysamedom(&a, &[], &[]));
    checkshape(&result);
    let mut expected = counts(2, 3, Some(3), Some(3));
    expected[4] = Some(1);
    expected[5] = Some(1);
    checknbshapes_some(&result, expected);
    check_tolerance_below(&result, 2.0e-5);
}

/// OCC32332: unified faces with internal edges.
fn bug32332(s: &mut dyn Session, file: &str, edges: usize, vertices: usize) {
    let rule = "==========================================================";
    banner(&[
        rule,
        "OCC32332: Incorrect result of ShapeUpgrade_UnifySameDomain",
        rule,
        "",
    ]);
    let a = restore_as(s, file, "a").shape();
    let result = set(s, "result", unifysamedom(&a, &[], &[]));
    checkshape(&result);
    checknbshapes_some(&result, counts(vertices, edges, Some(4), None));
    check_tolerance_below(&result, 2.0e-7);
}

pub fn bug32332_1(s: &mut dyn Session) {
    bug32332(s, "bug32332_1.brep", 9, 6);
}

pub fn bug32332_2(s: &mut dyn Session) {
    bug32332(s, "bug32332_2.brep", 12, 8);
}
