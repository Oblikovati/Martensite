//! heal surface to bspline regression cases
//! whose inputs are in the public dataset: each restores `a`, then the grid end runs
//! `puts [DT_ToBspl result a]` (default options `ero`; it prints nothing) and
//! `puts [checkshape result]`; tests/heal/end shows `result` with
//! `checkview -display result -2d`.
use crate::support::{
    checkshape::checkshape,
    log,
    restore::restore_as,
    session::{Session, checkview_display, set},
    swdraw::dt_to_bspl,
};

/// `restore file a`, then the grid end.
fn surface_to_bspline(s: &mut dyn Session, file: &str, image: &str) {
    let a = restore_as(s, file, "a").shape();
    let result = set(s, "result", dt_to_bspl(&a, "ero"));
    log::puts("");
    checkshape(&result);
    checkview_display(s, Some("result"), image);
}

/// tests/heal/surface_to_bspline/A1.
pub fn a1(s: &mut dyn Session) {
    log::todo("Faulty shapes in variables faulty_1 to faulty_2");
    surface_to_bspline(s, "aaa.rle", "A1");
}

/// tests/heal/surface_to_bspline/A2.
pub fn a2(s: &mut dyn Session) {
    surface_to_bspline(s, "BoxAndConeFS.rle", "A2");
}

/// tests/heal/surface_to_bspline/A3.
pub fn a3(s: &mut dyn Session) {
    surface_to_bspline(s, "boxFG.rle", "A3");
}

/// tests/heal/surface_to_bspline/A4.
pub fn a4(s: &mut dyn Session) {
    surface_to_bspline(s, "BUC50070-#53168-good.rle", "A4");
}

/// tests/heal/surface_to_bspline/A6.
pub fn a6(s: &mut dyn Session) {
    surface_to_bspline(s, "CCH_coco1.rle", "A6");
}

/// tests/heal/surface_to_bspline/A7.
pub fn a7(s: &mut dyn Session) {
    surface_to_bspline(s, "CCH_testkhes.rle", "A7");
}

/// tests/heal/surface_to_bspline/B2.
pub fn b2(s: &mut dyn Session) {
    surface_to_bspline(s, "CFI_5_l12fgq.rle", "B2");
}

/// tests/heal/surface_to_bspline/B3.
pub fn b3(s: &mut dyn Session) {
    surface_to_bspline(s, "CFI_6_g56fhe.rle", "B3");
}

/// tests/heal/surface_to_bspline/B4.
pub fn b4(s: &mut dyn Session) {
    surface_to_bspline(s, "CFI_8_i1_fia.rle", "B4");
}

/// tests/heal/surface_to_bspline/B7.
pub fn b7(s: &mut dyn Session) {
    surface_to_bspline(s, "CFI_g1234fio.rle", "B7");
}

/// tests/heal/surface_to_bspline/B8.
pub fn b8(s: &mut dyn Session) {
    surface_to_bspline(s, "CFI_g5678fip.rle", "B8");
}

/// tests/heal/surface_to_bspline/B9.
pub fn b9(s: &mut dyn Session) {
    surface_to_bspline(s, "CFI_n12__fiw.rle", "B9");
}

/// tests/heal/surface_to_bspline/C1.
pub fn c1(s: &mut dyn Session) {
    surface_to_bspline(s, "CFI_o12__fix.rle", "C1");
}

/// tests/heal/surface_to_bspline/C4.
pub fn c4(s: &mut dyn Session) {
    surface_to_bspline(s, "CFI_pro14fjq.rle", "C4");
}

/// tests/heal/surface_to_bspline/D1.
pub fn d1(s: &mut dyn Session) {
    surface_to_bspline(s, "CTO900_buc60127-part.rle", "D1");
}

/// tests/heal/surface_to_bspline/D4.
pub fn d4(s: &mut dyn Session) {
    surface_to_bspline(s, "CTO900_ger60039a.rle", "D4");
}

/// tests/heal/surface_to_bspline/D5.
pub fn d5(s: &mut dyn Session) {
    surface_to_bspline(s, "CTO900_ger60041a.rle", "D5");
}

/// tests/heal/surface_to_bspline/E2.
pub fn e2(s: &mut dyn Session) {
    log::todo("Faulty shapes in variables faulty_1 to faulty_2");
    surface_to_bspline(s, "DSE3.rle", "E2");
}

/// tests/heal/surface_to_bspline/E3.
pub fn e3(s: &mut dyn Session) {
    surface_to_bspline(s, "DSEwithTol1.rle", "E3");
}

/// tests/heal/surface_to_bspline/E5.
pub fn e5(s: &mut dyn Session) {
    surface_to_bspline(s, "RevolutionFG.rle", "E5");
}

/// tests/heal/surface_to_bspline/E6.
pub fn e6(s: &mut dyn Session) {
    surface_to_bspline(s, "ForDSE.rle", "E6");
}

/// tests/heal/surface_to_bspline/E7.
pub fn e7(s: &mut dyn Session) {
    surface_to_bspline(s, "newFS.rle", "E7");
}

/// tests/heal/surface_to_bspline/E8.
pub fn e8(s: &mut dyn Session) {
    surface_to_bspline(s, "pcpc2_FS_FG.rle", "E8");
}

/// tests/heal/surface_to_bspline/F1.
pub fn f1(s: &mut dyn Session) {
    surface_to_bspline(s, "RevolutionFG.rle", "F1");
}

/// tests/heal/surface_to_bspline/F2.
pub fn f2(s: &mut dyn Session) {
    log::todo("Faulty shapes in variables faulty_1 to faulty_2");
    surface_to_bspline(s, "rrr.rle", "F2");
}

/// tests/heal/surface_to_bspline/F5.
pub fn f5(s: &mut dyn Session) {
    surface_to_bspline(s, "spheraDSE.rle", "F5");
}

/// tests/heal/surface_to_bspline/F6.
pub fn f6(s: &mut dyn Session) {
    surface_to_bspline(s, "1-Ring.brep", "F6");
}

/// tests/heal/surface_to_bspline/F8.
pub fn f8(s: &mut dyn Session) {
    surface_to_bspline(s, "bowl_noremove.brep", "F8");
}
