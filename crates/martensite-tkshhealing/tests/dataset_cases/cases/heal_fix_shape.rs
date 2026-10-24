//! heal fix shape regression cases whose
//! inputs are in the public dataset: each restores `a`, then the grid end runs
//! `puts [fixshape result a 0.001 0.005]` and `puts [checkshape result]`; end
//! shows `result` with `checkview -display result -2d`.
use crate::support::{
    checkshape::checkshape,
    log,
    restore::restore_as,
    session::{Session, checkview_display, set},
    swdraw::fixshape,
};

/// `restore file a`, then the grid end.
fn fix_shape(s: &mut dyn Session, file: &str, image: &str) {
    let a = restore_as(s, file, "a").shape();
    let result = set(s, "result", fixshape(&a, &["0.001", "0.005"]));
    checkshape(&result);
    checkview_display(s, Some("result"), image);
}

/// fix shape — A1.
pub fn a1(s: &mut dyn Session) {
    log::todo("Faulty shapes in variables faulty_1 to faulty_1");
    fix_shape(s, "aaa.rle", "A1");
}

/// fix shape — A2.
pub fn a2(s: &mut dyn Session) {
    fix_shape(s, "CCH_coco1.rle", "A2");
}

/// fix shape — A3.
pub fn a3(s: &mut dyn Session) {
    fix_shape(s, "CCH_testkhes.rle", "A3");
}

/// fix shape — A7.
pub fn a7(s: &mut dyn Session) {
    fix_shape(s, "CFE903_ger60ggw.rle", "A7");
}

/// fix shape — A8.
pub fn a8(s: &mut dyn Session) {
    fix_shape(s, "CFI_5_l12fgq.rle", "A8");
}

/// fix shape — A9.
pub fn a9(s: &mut dyn Session) {
    fix_shape(s, "CFI_6_g56fhe.rle", "A9");
}

/// fix shape — B1.
pub fn b1(s: &mut dyn Session) {
    fix_shape(s, "CFI_8_i1_fia.rle", "B1");
}

/// fix shape — B4.
pub fn b4(s: &mut dyn Session) {
    fix_shape(s, "CFI_g1234fio.rle", "B4");
}

/// fix shape — B5.
pub fn b5(s: &mut dyn Session) {
    fix_shape(s, "CFI_g5678fip.rle", "B5");
}

/// fix shape — B6.
pub fn b6(s: &mut dyn Session) {
    fix_shape(s, "CFI_n12__fiw.rle", "B6");
}

/// fix shape — B7.
pub fn b7(s: &mut dyn Session) {
    fix_shape(s, "CFI_o12__fix.rle", "B7");
}

/// fix shape — C1.
pub fn c1(s: &mut dyn Session) {
    fix_shape(s, "CFI_pro14fjq.rle", "C1");
}

/// fix shape — C6.
pub fn c6(s: &mut dyn Session) {
    fix_shape(s, "CIN001_a13.rle", "C6");
}

/// fix shape — C7.
pub fn c7(s: &mut dyn Session) {
    fix_shape(s, "CIN001_a14.rle", "C7");
}

/// fix shape — C8.
pub fn c8(s: &mut dyn Session) {
    fix_shape(s, "CIN001_a49.rle", "C8");
}

/// fix shape — C9.
pub fn c9(s: &mut dyn Session) {
    fix_shape(s, "CIN001_a51.rle", "C9");
}

/// fix shape — D1.
pub fn d1(s: &mut dyn Session) {
    fix_shape(s, "CIN001_a62.rle", "D1");
}

/// fix shape — D2.
pub fn d2(s: &mut dyn Session) {
    fix_shape(s, "CIN001_a64.rle", "D2");
}

/// fix shape — D4.
pub fn d4(s: &mut dyn Session) {
    fix_shape(s, "CIN001_b13.rle", "D4");
}

/// fix shape — D5.
pub fn d5(s: &mut dyn Session) {
    fix_shape(s, "CIN001_b14.rle", "D5");
}

/// fix shape — D6.
pub fn d6(s: &mut dyn Session) {
    fix_shape(s, "CIN001_b49.rle", "D6");
}

/// fix shape — D7.
pub fn d7(s: &mut dyn Session) {
    fix_shape(s, "CIN001_b50.rle", "D7");
}

/// fix shape — D8.
pub fn d8(s: &mut dyn Session) {
    fix_shape(s, "CIN001_b51.rle", "D8");
}

/// fix shape — D9.
pub fn d9(s: &mut dyn Session) {
    fix_shape(s, "CIN001_b62.rle", "D9");
}

/// fix shape — E1.
pub fn e1(s: &mut dyn Session) {
    fix_shape(s, "CNP001_projoien.rle", "E1");
}

/// fix shape — E3.
pub fn e3(s: &mut dyn Session) {
    fix_shape(s, "CTO900_buc60127-part.rle", "E3");
}

/// fix shape — E6.
pub fn e6(s: &mut dyn Session) {
    fix_shape(s, "CTO900_ger60039a.rle", "E6");
}

/// fix shape — E7.
pub fn e7(s: &mut dyn Session) {
    fix_shape(s, "CTO900_ger60041a.rle", "E7");
}

/// fix shape — E9.
pub fn e9(s: &mut dyn Session) {
    fix_shape(s, "CTO900_pro5363c.rle", "E9");
}

/// fix shape — F4.
pub fn f4(s: &mut dyn Session) {
    fix_shape(s, "err_f_2.rle", "F4");
}

/// fix shape — F7.
pub fn f7(s: &mut dyn Session) {
    log::todo("Faulty shapes in variables faulty_1 to faulty_1");
    fix_shape(s, "rrr.rle", "F7");
}

/// fix shape — G2.
pub fn g2(s: &mut dyn Session) {
    fix_shape(s, "BoxAndConeFS.rle", "G2");
}

/// fix shape — G3.
pub fn g3(s: &mut dyn Session) {
    fix_shape(s, "newFS.rle", "G3");
}

/// fix shape — G4.
pub fn g4(s: &mut dyn Session) {
    fix_shape(s, "pcpc2_FS_FG.rle", "G4");
}

/// fix shape — G5.
pub fn g5(s: &mut dyn Session) {
    fix_shape(s, "csh001i.brep", "G5");
}
