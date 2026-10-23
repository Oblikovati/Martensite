//! heal surface to bspline regression cases
//! whose inputs are in the public dataset: each restores `a`, then the grid end runs
//! `puts [DT_ToBspl result a]` (default options `ero`; it prints nothing) and
//! `puts [checkshape result]`; end shows `result` with
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

/// surface to bspline — A1.
pub fn a1(s: &mut dyn Session) {
    log::todo("Faulty shapes in variables faulty_1 to faulty_2");
    surface_to_bspline(s, "aaa.rle", "A1");
}

/// surface to bspline — A2.
pub fn a2(s: &mut dyn Session) {
    surface_to_bspline(s, "BoxAndConeFS.rle", "A2");
}

/// surface to bspline — A3.
pub fn a3(s: &mut dyn Session) {
    surface_to_bspline(s, "boxFG.rle", "A3");
}

/// surface to bspline — A4.
pub fn a4(s: &mut dyn Session) {
    surface_to_bspline(s, "BUC50070-#53168-good.rle", "A4");
}

/// surface to bspline — A6.
pub fn a6(s: &mut dyn Session) {
    surface_to_bspline(s, "CCH_coco1.rle", "A6");
}

/// surface to bspline — A7.
pub fn a7(s: &mut dyn Session) {
    surface_to_bspline(s, "CCH_testkhes.rle", "A7");
}

/// surface to bspline — B2.
pub fn b2(s: &mut dyn Session) {
    surface_to_bspline(s, "CFI_5_l12fgq.rle", "B2");
}

/// surface to bspline — B3.
pub fn b3(s: &mut dyn Session) {
    surface_to_bspline(s, "CFI_6_g56fhe.rle", "B3");
}

/// surface to bspline — B4.
pub fn b4(s: &mut dyn Session) {
    surface_to_bspline(s, "CFI_8_i1_fia.rle", "B4");
}

/// surface to bspline — B7.
pub fn b7(s: &mut dyn Session) {
    surface_to_bspline(s, "CFI_g1234fio.rle", "B7");
}

/// surface to bspline — B8.
pub fn b8(s: &mut dyn Session) {
    surface_to_bspline(s, "CFI_g5678fip.rle", "B8");
}

/// surface to bspline — B9.
pub fn b9(s: &mut dyn Session) {
    surface_to_bspline(s, "CFI_n12__fiw.rle", "B9");
}

/// surface to bspline — C1.
pub fn c1(s: &mut dyn Session) {
    surface_to_bspline(s, "CFI_o12__fix.rle", "C1");
}

/// surface to bspline — C4.
pub fn c4(s: &mut dyn Session) {
    surface_to_bspline(s, "CFI_pro14fjq.rle", "C4");
}

/// surface to bspline — D1.
pub fn d1(s: &mut dyn Session) {
    surface_to_bspline(s, "CTO900_buc60127-part.rle", "D1");
}

/// surface to bspline — D4.
pub fn d4(s: &mut dyn Session) {
    surface_to_bspline(s, "CTO900_ger60039a.rle", "D4");
}

/// surface to bspline — D5.
pub fn d5(s: &mut dyn Session) {
    surface_to_bspline(s, "CTO900_ger60041a.rle", "D5");
}

/// surface to bspline — E2.
pub fn e2(s: &mut dyn Session) {
    log::todo("Faulty shapes in variables faulty_1 to faulty_2");
    surface_to_bspline(s, "DSE3.rle", "E2");
}

/// surface to bspline — E3.
pub fn e3(s: &mut dyn Session) {
    surface_to_bspline(s, "DSEwithTol1.rle", "E3");
}

/// surface to bspline — E5.
pub fn e5(s: &mut dyn Session) {
    surface_to_bspline(s, "RevolutionFG.rle", "E5");
}

/// surface to bspline — E6.
pub fn e6(s: &mut dyn Session) {
    surface_to_bspline(s, "ForDSE.rle", "E6");
}

/// surface to bspline — E7.
pub fn e7(s: &mut dyn Session) {
    surface_to_bspline(s, "newFS.rle", "E7");
}

/// surface to bspline — E8.
pub fn e8(s: &mut dyn Session) {
    surface_to_bspline(s, "pcpc2_FS_FG.rle", "E8");
}

/// surface to bspline — F1.
pub fn f1(s: &mut dyn Session) {
    surface_to_bspline(s, "RevolutionFG.rle", "F1");
}

/// surface to bspline — F2.
pub fn f2(s: &mut dyn Session) {
    log::todo("Faulty shapes in variables faulty_1 to faulty_2");
    surface_to_bspline(s, "rrr.rle", "F2");
}

/// surface to bspline — F5.
pub fn f5(s: &mut dyn Session) {
    surface_to_bspline(s, "spheraDSE.rle", "F5");
}

/// surface to bspline — F6.
pub fn f6(s: &mut dyn Session) {
    surface_to_bspline(s, "1-Ring.brep", "F6");
}

/// surface to bspline — F8.
pub fn f8(s: &mut dyn Session) {
    surface_to_bspline(s, "bowl_noremove.brep", "F8");
}
