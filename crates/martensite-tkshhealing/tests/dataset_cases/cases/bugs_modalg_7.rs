//! tests/bugs/modalg_7 case bodies at 3d097a0328e71b826377d4814ab05ec3c3d23871 owned by
//! TKShHealing. Each body follows its script line by line; the bugs group end only prints
//! `TEST COMPLETED`.
use super::bugs_common::banner;
use crate::support::{
    checkshape::checkshape,
    commands::{explode, explode_type},
    log,
    logged_checks::checknbshapes_some,
    restore::restore_as,
    session::{Session, set, set_exploded},
    sewing::{checkmaxtol_min, sewing, tolerance_text},
    swdraw::fixshape,
    text::numbers_after,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;

/// OCC23171: sewing at 1e-6 keeps the stitched shape within the 0.003 tolerance.
pub fn bug23171_1(s: &mut dyn Session) {
    banner(&["========", "OCC23171", "========", ""]);
    let a = restore_as(s, "bug23171_stitch.brep", "a").shape();
    let r = sewing(0.000001, &[&a])
        .map(|shape| set(s, "r", shape))
        .expect("checkshape: r is not a shape");
    checkshape(&r);
    let mut counts = [None; 9];
    counts[0] = Some(6);
    counts[1] = Some(8);
    checknbshapes_some(&r, counts);
    checkmaxtol_min(&r, "r", 0.003);
}

/// The MAX and MIN of one `KIND +: +MAX=... +AVG=... +MIN=...` row of `tolerance`.
fn max_min(text: &str, kind: &str) -> (f64, f64) {
    let line = text
        .lines()
        .find(|line| line.contains(&format!("{kind} ")) && line.contains("MAX="))
        .unwrap_or_else(|| panic!("tolerance prints no {kind} row"));
    let max = numbers_after(line, "MAX=", 1, "").unwrap()[0];
    let min = numbers_after(line, "MIN=", 1, "").unwrap()[0];
    (max, min)
}

/// One `foreach` body: when the sub-shape is valid, its tolerance must not exceed the
/// minimal tolerance of its edges (faces) and vertices.
fn check_sub_shape(name: &str, part: &Shape, kind: &str) {
    if !checkshape(part).text.eq("This shape seems to be valid") {
        return;
    }
    let text = tolerance_text(part);
    log::result(&text);
    let (_, min_vertex) = max_min(&text, "VERTEX");
    if kind == "FACE" {
        let (max_face, _) = max_min(&text, "FACE");
        let (_, min_edge) = max_min(&text, "EDGE");
        if max_face > min_edge || max_face > min_vertex {
            log::puts(&format!(
                "Error: the tolerance value of the FACE {name} should be less then or equal to the minimal tolerance value of its sub-shapes(EDGES and VERTICES)"
            ));
        }
    } else {
        let (max_edge, _) = max_min(&text, "EDGE");
        if max_edge > min_vertex {
            log::puts(&format!(
                "Error: the tolerance value of the EDGE {name} should be less then or equal to the minimal tolerance value of its sub-shapes(VERTICES)"
            ));
        }
    }
}

/// OCC23610 (`TODO OCC23610 ALL: should be less then or equal to the minimal tolerance
/// value of its sub-shapes`).
pub fn bug23610(s: &mut dyn Session) {
    log::todo("should be less then or equal to the minimal tolerance value of its sub-shapes");
    banner(&["========", "OCC23610", "========", ""]);
    let b = restore_as(s, "buc60462b.brep", "b").shape();
    let faces = set_exploded(s, "b", explode_type(&b, ShapeEnum::Face));
    for (index, face) in faces.iter().enumerate() {
        check_sub_shape(&format!("b_{}", index + 1), face, "FACE");
    }
    let edges = set_exploded(s, "b", explode_type(&b, ShapeEnum::Edge));
    for (index, edge) in edges.iter().enumerate() {
        check_sub_shape(&format!("b_{}", index + 1), edge, "EDGE");
    }
}

/// OCC27614 (`TODO OCC27614 ALL: ERROR: OCC27614 is reproduced.`): the sewn tolerance
/// stays below the sewing tolerance.
pub fn bug27614(s: &mut dyn Session) {
    log::todo("ERROR: OCC27614 is reproduced.");
    banner(&["========", "OCC27614", "========", ""]);
    let sew_tol = 0.2;
    let c = restore_as(s, "bug27614_CC.brep", "c").shape();
    let parts = set_exploded(s, "c", explode(&c));
    let result = sewing(sew_tol, &[&parts[0], &parts[1]])
        .map(|shape| set(s, "result", shape))
        .expect("checkshape: result is not a shape");
    checkshape(&result);
    let info = tolerance_text(&result);
    log::result(&info);
    let word = info.split_whitespace().nth(1).expect("lindex $bug_info 1");
    let max_tol: f64 = word[word.find('=').unwrap() + 1..].trim().parse().unwrap();
    if max_tol > sew_tol {
        log::puts(&format!(
            "ERROR: OCC27614 is reproduced. Tolerance of shape is too high: {max_tol} (expected: {sew_tol})."
        ));
    }
}

/// 0030396: ShapeFix of a boolean result must not recurse endlessly.
pub fn bug30396_2(s: &mut dyn Session) {
    banner(&[
        "========",
        "0030396: Infinite recursion during ShapeFix after BRepAlgoAPI_Cut",
        "========",
        "",
    ]);
    let a = restore_as(s, "bug30396_bad_result.brep", "a").shape();
    set(s, "r", fixshape(&a, &[]));
}

/// tests/bugs/modalg_7/bug31430.
/// 0031430: offset surfaces on C1 surfaces with full multiplicity must not throw; the
/// missing seam of face 4 is added (ShapeAnalysis_Surface closure), giving 4 faces.
pub fn bug31430(s: &mut dyn Session) {
    banner(&[
        "========",
        "0031430: Modeling Data - Offset surfaces on C1 surfaces (with multiplicity equal to degree) may still throw ex-ception",
        "========",
        "",
    ]);
    let shape = restore_as(s, "bug31430.brep", "s").shape();
    let r = set(s, "r", fixshape(&shape, &[]));
    checkshape(&r);
    let mut counts = [None; 9];
    counts[..6].copy_from_slice(&[Some(6), Some(8), Some(6), Some(4), Some(1), Some(1)]);
    checknbshapes_some(&r, counts);
}
