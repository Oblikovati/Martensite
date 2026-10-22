//! tests/heal/fix_shape case bodies at 3d097a0328e71b826377d4814ab05ec3c3d23871 whose
//! inputs are in the public dataset: each restores `a`, then the grid end runs
//! `puts [fixshape result a 0.001 0.005]` and `puts [checkshape result]`; tests/heal/end
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

/// tests/heal/fix_shape/A1.
pub fn a1(s: &mut dyn Session) {
    log::todo("Faulty shapes in variables faulty_1 to faulty_1");
    fix_shape(s, "aaa.rle", "A1");
}

/// tests/heal/fix_shape/A2.
pub fn a2(s: &mut dyn Session) {
    fix_shape(s, "CCH_coco1.rle", "A2");
}

/// tests/heal/fix_shape/A3.
pub fn a3(s: &mut dyn Session) {
    fix_shape(s, "CCH_testkhes.rle", "A3");
}

/// tests/heal/fix_shape/A7.
pub fn a7(s: &mut dyn Session) {
    fix_shape(s, "CFE903_ger60ggw.rle", "A7");
}

/// tests/heal/fix_shape/A8.
pub fn a8(s: &mut dyn Session) {
    fix_shape(s, "CFI_5_l12fgq.rle", "A8");
}

/// tests/heal/fix_shape/A9.
pub fn a9(s: &mut dyn Session) {
    fix_shape(s, "CFI_6_g56fhe.rle", "A9");
}

/// tests/heal/fix_shape/B1.
pub fn b1(s: &mut dyn Session) {
    fix_shape(s, "CFI_8_i1_fia.rle", "B1");
}

/// tests/heal/fix_shape/B4.
pub fn b4(s: &mut dyn Session) {
    fix_shape(s, "CFI_g1234fio.rle", "B4");
}

/// tests/heal/fix_shape/B5.
pub fn b5(s: &mut dyn Session) {
    fix_shape(s, "CFI_g5678fip.rle", "B5");
}

/// tests/heal/fix_shape/B6.
pub fn b6(s: &mut dyn Session) {
    fix_shape(s, "CFI_n12__fiw.rle", "B6");
}

/// tests/heal/fix_shape/B7.
pub fn b7(s: &mut dyn Session) {
    fix_shape(s, "CFI_o12__fix.rle", "B7");
}

/// tests/heal/fix_shape/C1.
pub fn c1(s: &mut dyn Session) {
    fix_shape(s, "CFI_pro14fjq.rle", "C1");
}

/// tests/heal/fix_shape/C6.
pub fn c6(s: &mut dyn Session) {
    fix_shape(s, "CIN001_a13.rle", "C6");
}

/// tests/heal/fix_shape/C7.
pub fn c7(s: &mut dyn Session) {
    fix_shape(s, "CIN001_a14.rle", "C7");
}

/// tests/heal/fix_shape/C8.
pub fn c8(s: &mut dyn Session) {
    fix_shape(s, "CIN001_a49.rle", "C8");
}

/// tests/heal/fix_shape/C9.
pub fn c9(s: &mut dyn Session) {
    fix_shape(s, "CIN001_a51.rle", "C9");
}

/// tests/heal/fix_shape/D1.
pub fn d1(s: &mut dyn Session) {
    fix_shape(s, "CIN001_a62.rle", "D1");
}

/// tests/heal/fix_shape/D2.
pub fn d2(s: &mut dyn Session) {
    fix_shape(s, "CIN001_a64.rle", "D2");
}

/// tests/heal/fix_shape/D4.
pub fn d4(s: &mut dyn Session) {
    fix_shape(s, "CIN001_b13.rle", "D4");
}

/// tests/heal/fix_shape/D5.
pub fn d5(s: &mut dyn Session) {
    fix_shape(s, "CIN001_b14.rle", "D5");
}

/// tests/heal/fix_shape/D6.
pub fn d6(s: &mut dyn Session) {
    fix_shape(s, "CIN001_b49.rle", "D6");
}

/// tests/heal/fix_shape/D7.
pub fn d7(s: &mut dyn Session) {
    fix_shape(s, "CIN001_b50.rle", "D7");
}

/// tests/heal/fix_shape/D8.
pub fn d8(s: &mut dyn Session) {
    fix_shape(s, "CIN001_b51.rle", "D8");
}

/// tests/heal/fix_shape/D9.
pub fn d9(s: &mut dyn Session) {
    fix_shape(s, "CIN001_b62.rle", "D9");
}

/// tests/heal/fix_shape/E1.
pub fn e1(s: &mut dyn Session) {
    fix_shape(s, "CNP001_projoien.rle", "E1");
}

/// tests/heal/fix_shape/E3.
pub fn e3(s: &mut dyn Session) {
    fix_shape(s, "CTO900_buc60127-part.rle", "E3");
}

/// tests/heal/fix_shape/E6.
pub fn e6(s: &mut dyn Session) {
    fix_shape(s, "CTO900_ger60039a.rle", "E6");
}

/// tests/heal/fix_shape/E7.
pub fn e7(s: &mut dyn Session) {
    fix_shape(s, "CTO900_ger60041a.rle", "E7");
}

/// tests/heal/fix_shape/E9.
pub fn e9(s: &mut dyn Session) {
    fix_shape(s, "CTO900_pro5363c.rle", "E9");
}

/// tests/heal/fix_shape/F4.
pub fn f4(s: &mut dyn Session) {
    fix_shape(s, "err_f_2.rle", "F4");
}

/// tests/heal/fix_shape/F7.
pub fn f7(s: &mut dyn Session) {
    log::todo("Faulty shapes in variables faulty_1 to faulty_1");
    fix_shape(s, "rrr.rle", "F7");
}

/// tests/heal/fix_shape/G2.
pub fn g2(s: &mut dyn Session) {
    fix_shape(s, "BoxAndConeFS.rle", "G2");
}

/// tests/heal/fix_shape/G3.
pub fn g3(s: &mut dyn Session) {
    fix_shape(s, "newFS.rle", "G3");
}

/// tests/heal/fix_shape/G4.
pub fn g4(s: &mut dyn Session) {
    fix_shape(s, "pcpc2_FS_FG.rle", "G4");
}

/// tests/heal/fix_shape/G5.
pub fn g5(s: &mut dyn Session) {
    fix_shape(s, "csh001i.brep", "G5");
}
