//! tests/bugs/heal case bodies at 3d097a0328e71b826377d4814ab05ec3c3d23871 whose inputs
//! are in the public dataset. Each body follows its script line by line; the bugs group
//! end only prints `TEST COMPLETED` (none of these cases sets `rel_tol`).
pub(crate) use super::bugs_common::{banner, tolerance_max};
use crate::support::{
    build::{brestore, checknbshapes_t, maxtolerance_printed},
    checkshape::{CheckReport, checkshape},
    log,
    logged_checks::{checknbshapes_some, checkreal},
    restore::restore_as,
    session::{Session, checkview_display, set},
    sewing::tolerance_text,
    swdraw::{fixshape, fixsmall},
    text::numbers_after,
};
use martensite_tkbrep::topods::Shape;

mod qa_cases;
mod surfaces;
pub use surfaces::{bug23944, bug25967, bug28523, bug28768};
mod unify;
pub use qa_cases::{
    bug22, bug24, bug7570, bug22919, bug24008_1, bug24008_2, bug24370, bug26930_1, bug26930_2,
};
pub use unify::{
    bug28529, bug29504_1, bug29504_2, bug29544_1, bug30158_1, bug30158_2, bug30174, bug32332_1,
    bug32332_2,
};

/// `checknbshapes result -face n` and the other single-entity forms, in VERTEX..SHAPE
/// order of [`crate::support::checks::NBSHAPES_ENTITIES`].
fn checknbshapes_one(shape: &Shape, index: usize, count: usize) {
    let mut expected = [None; 9];
    expected[index] = Some(count);
    checknbshapes_some(shape, expected);
}

const EDGE: usize = 1;
const FACE: usize = 3;

/// OCC210 (case 2): FixShape corrects touching wires.
pub fn bug210_2(s: &mut dyn Session) {
    banner(&["========", "OCC210", "(case 2)", "========"]);
    let a = restore_as(s, "OCC210b.brep", "a").shape();
    let result = set(s, "result", fixshape(&a, &["0.01", "0.01"]));
    checkshape(&result);
    checkview_display(s, Some("result"), "bug210_2");
}

/// OCC21317: a face Shape Healing could not fix; the healed face must be valid.
pub fn bug21317(s: &mut dyn Session) {
    banner(&["================", "OCC21317", "================", ""]);
    let a = restore_as(s, "bug21317_faulty.brep", "a").shape();
    let result = set(s, "result", fixshape(&a, &[]));
    let info: CheckReport = checkshape(&result);
    log::puts("");
    if info.text.contains("This shape seems to be valid") {
        log::puts("OK OCC21317");
    } else {
        log::puts("Faulty OCC21317");
    }
    checkview_display(s, Some("result"), "bug21317");
}

/// OCC22602: notched healing (`TODO OCC22602 ALL: Error : notched simple file is WRONG`).
pub fn bug22602(s: &mut dyn Session) {
    log::todo("Error : notched simple file is WRONG");
    banner(&["========", "OCC22602", "========", ""]);
    let a = restore_as(s, "bug22602_notched_simple.brep", "a").shape();
    checkview_display(s, Some("a"), "bug22602_1");
    let switches = ["-l", "-o", "-h", "-m", "-d", "-s", "-i", "+n"];
    let result = set(s, "result", fixshape(&a, &switches));
    checkshape(&result);
    let expected = "\nNumber of shapes in shape\n VERTEX    : 6\n EDGE      : 7\n";
    checknbshapes_t(
        &result,
        "result",
        &[],
        Some(expected),
        "notched simple file",
    );
    checkview_display(s, Some("result"), "bug22602_2");
}

/// OCC22642 (`TODO OCC22642 ALL: Faulty shapes in variables faulty_1 to faulty_`).
pub fn bug22642(s: &mut dyn Session) {
    log::todo("Faulty shapes in variables faulty_1 to faulty_");
    banner(&["========", "OCC22642", "========", ""]);
    let f = restore_as(s, "bug22642_ss_47.brep", "f").shape();
    let switches = ["-l", "-o", "-h", "-m", "-d", "-s", "+i", "-n"];
    let result = set(s, "result", fixshape(&f, &switches));
    checkshape(&result);
}

/// OCC24218: FixFaceOrientation twice (`fixshape r f +o`, `fixshape rr r +o`).
pub fn bug24218(s: &mut dyn Session) {
    banner(&["============", "OCC24218", "============", ""]);
    let f = restore_as(s, "bug24218_f.brep", "f").shape();
    let r = set(s, "r", fixshape(&f, &["+o"]));
    checkshape(&r);
    let rr = set(s, "rr", fixshape(&r, &["+o"]));
    checkshape(&rr);
}

/// OCC24658: FixShape must not change the tolerance of its input.
pub fn bug24658(s: &mut dyn Session) {
    banner(&["============", "OCC24658", "============", ""]);
    let a = restore_as(s, "bug24658.brep", "a").shape();
    log::puts("\nBefore shape healing");
    let expected = tolerance_max(&a);
    set(s, "af", fixshape(&a, &[]));
    log::puts("\nAfter shape healing");
    let max_tol = tolerance_max(&a);
    checkreal("MaxTolerance", max_tol, expected, 1.0e-06, 1.0e-06);
}

/// OCC24934: fixshape must not raise an exception.
pub fn bug24934(s: &mut dyn Session) {
    banner(&["============", "OCC24934", "============", ""]);
    let f1 = restore_as(s, "bug24934_f1.brep", "f1").shape();
    set(s, "ff1", fixshape(&f1, &[]));
    log::puts("OK: Command \"fixshape\" work properly");
}

/// Bug #25553 and #26708: a face with shifted pcurves healed at 1e-7 / 1.
fn healed_single_face(s: &mut dyn Session, title: &str, file: &str, image: &str) {
    banner(&[
        "========================",
        title,
        "========================",
    ]);
    let face = set(s, "face", brestore(file));
    let result = set(s, "result", fixshape(&face, &["1e-7", "1"]));
    checkshape(&result);
    checknbshapes_one(&result, FACE, 1);
    checkview_display(s, Some("result"), image);
}

pub fn bug25553_1(s: &mut dyn Session) {
    healed_single_face(s, "Bug #25553", "bug25553_ii_73.brep", "bug25553_1");
}

pub fn bug25553_2(s: &mut dyn Session) {
    let file = "bug25553_f-cyl-no-seam-shifted-pcurves.brep";
    healed_single_face(s, "Bug #25553", file, "bug25553_2");
}

pub fn bug25553_3(s: &mut dyn Session) {
    let file = "bug25553_f-cyl-no-seam-shifted-pcurves-2.brep";
    healed_single_face(s, "Bug #25553", file, "bug25553_3");
}

pub fn bug26708(s: &mut dyn Session) {
    healed_single_face(s, "Bug #26708", "bug26708_a_205.brep", "bug26708");
}

/// OCC27246 (`TODO OCC27246 ALL: Faulty shapes in variables faulty_1 to`).
pub fn bug27246(s: &mut dyn Session) {
    log::todo("Faulty shapes in variables faulty_1 to");
    banner(&["============", "OCC27246", "============", ""]);
    let tmp = restore_as(s, "bug27246_tmp.brep", "tmp").shape();
    checkshape(&tmp);
    let result = set(s, "result", fixshape(&tmp, &[]));
    checkshape(&result);
    checkview_display(s, Some("result"), "bug27246");
}

/// bug31202: `fixshape result shape 0.000001137 0.0001137`.
pub fn bug31202(s: &mut dyn Session) {
    let shape = restore_as(s, "bug31202.brep", "shape").shape();
    let result = set(s, "result", fixshape(&shape, &["0.000001137", "0.0001137"]));
    checkshape(&result);
    checknbshapes_one(&result, EDGE, 4);
}

/// OCC329: FixShape must not increase any maximum tolerance.
pub fn bug329(s: &mut dyn Session) {
    banner(&[
        "========================",
        " OCC329 ",
        "========================",
        "",
    ]);
    let a = restore_as(s, "OCC329.brep", "a").shape();
    checkshape(&a);
    let [face1, edge1, vert1] = maxtolerance_printed("a", &a);
    let result = set(s, "result", fixshape(&a, &["1.e-7", "0.1"]));
    checkshape(&result);
    let [face2, edge2, vert2] = maxtolerance_printed("result", &result);
    if face2 > face1 || edge2 > edge1 || vert2 > vert1 {
        log::puts("Error : Tolerance invalid. Function FixShape works WRONGLY");
    } else {
        log::puts("Tolerance valed. Function FixShape works CORRECTLY");
    }
    checkview_display(s, Some("result"), "bug329");
}

/// OCC482: `fixsmall result a 0.05`.
pub fn bug482(s: &mut dyn Session) {
    banner(&[
        "========================",
        " OCC482 ",
        "========================",
        "",
    ]);
    let a = restore_as(s, "OCC482.brep", "a").shape();
    let fixed = fixsmall(&a, Some(0.05));
    let result = fixed
        .shape
        .map(|shape| set(s, "result", shape))
        .expect("checkshape: result is not defined (fixsmall fixed nothing)");
    log::result(&fixed.text);
    checkshape(&result);
    checkview_display(s, Some("result"), "bug482");
}
