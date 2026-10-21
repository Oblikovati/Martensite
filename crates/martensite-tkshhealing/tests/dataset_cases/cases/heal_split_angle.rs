//! tests/heal/split_angle case bodies at 3d097a0328e71b826377d4814ab05ec3c3d23871 whose
//! inputs are in the public dataset: each restores `a`, then the grid end runs
//! `puts [DT_SplitAngle result a]` and `puts [checkshape result]`; tests/heal/end shows
//! `result` with `checkview -display result -2d`.
use crate::support::{
    checkshape::checkshape,
    log,
    restore::restore_as,
    session::{Session, checkview_display, set},
    swdraw::dt_split_angle,
};

/// `restore file a`, then the grid end.
fn split_angle(s: &mut dyn Session, file: &str, image: &str) {
    let a = restore_as(s, file, "a").shape();
    let printed = dt_split_angle(&a, None);
    let result = set(s, "result", printed.shape);
    log::result(&printed.text);
    checkshape(&result);
    checkview_display(s, Some("result"), image);
}

/// tests/heal/split_angle/A1.
pub fn a1(s: &mut dyn Session) {
    split_angle(s, "CCH_coco1.rle", "A1");
}

/// tests/heal/split_angle/A2.
pub fn a2(s: &mut dyn Session) {
    split_angle(s, "CCH_testkhes.rle", "A2");
}

/// tests/heal/split_angle/A6.
pub fn a6(s: &mut dyn Session) {
    split_angle(s, "CFI_5_l12fgq.rle", "A6");
}

/// tests/heal/split_angle/A7.
pub fn a7(s: &mut dyn Session) {
    split_angle(s, "CFI_6_g56fhe.rle", "A7");
}

/// tests/heal/split_angle/A8.
pub fn a8(s: &mut dyn Session) {
    split_angle(s, "CFI_8_i1_fia.rle", "A8");
}

/// tests/heal/split_angle/B2.
pub fn b2(s: &mut dyn Session) {
    split_angle(s, "CFI_g1234fio.rle", "B2");
}

/// tests/heal/split_angle/B3.
pub fn b3(s: &mut dyn Session) {
    split_angle(s, "CFI_g5678fip.rle", "B3");
}

/// tests/heal/split_angle/B4.
pub fn b4(s: &mut dyn Session) {
    split_angle(s, "CFI_n12__fiw.rle", "B4");
}

/// tests/heal/split_angle/B5.
pub fn b5(s: &mut dyn Session) {
    split_angle(s, "CFI_o12__fix.rle", "B5");
}

/// tests/heal/split_angle/B8.
pub fn b8(s: &mut dyn Session) {
    split_angle(s, "CFI_pro14fjq.rle", "B8");
}

/// tests/heal/split_angle/C4.
pub fn c4(s: &mut dyn Session) {
    split_angle(s, "CIN001_a13.rle", "C4");
}

/// tests/heal/split_angle/C5.
pub fn c5(s: &mut dyn Session) {
    split_angle(s, "CIN001_a14.rle", "C5");
}

/// tests/heal/split_angle/C6.
pub fn c6(s: &mut dyn Session) {
    split_angle(s, "CIN001_a49.rle", "C6");
}

/// tests/heal/split_angle/C7.
pub fn c7(s: &mut dyn Session) {
    split_angle(s, "CIN001_a51.rle", "C7");
}

/// tests/heal/split_angle/C8.
pub fn c8(s: &mut dyn Session) {
    split_angle(s, "CIN001_a62.rle", "C8");
}

/// tests/heal/split_angle/C9.
pub fn c9(s: &mut dyn Session) {
    split_angle(s, "CIN001_b13.rle", "C9");
}

/// tests/heal/split_angle/D1.
pub fn d1(s: &mut dyn Session) {
    split_angle(s, "CIN001_b14.rle", "D1");
}

/// tests/heal/split_angle/D2.
pub fn d2(s: &mut dyn Session) {
    split_angle(s, "CIN001_b49.rle", "D2");
}

/// tests/heal/split_angle/D3.
pub fn d3(s: &mut dyn Session) {
    split_angle(s, "CIN001_b50.rle", "D3");
}

/// tests/heal/split_angle/D4.
pub fn d4(s: &mut dyn Session) {
    split_angle(s, "CIN001_b51.rle", "D4");
}

/// tests/heal/split_angle/D5.
pub fn d5(s: &mut dyn Session) {
    split_angle(s, "CIN001_b62.rle", "D5");
}

/// tests/heal/split_angle/D7.
pub fn d7(s: &mut dyn Session) {
    split_angle(s, "CTO900_buc60127-part.rle", "D7");
}

/// tests/heal/split_angle/E1.
pub fn e1(s: &mut dyn Session) {
    split_angle(s, "CTO900_ger60039a.rle", "E1");
}

/// tests/heal/split_angle/E2.
pub fn e2(s: &mut dyn Session) {
    split_angle(s, "CTO900_ger60041a.rle", "E2");
}

/// tests/heal/split_angle/F2.
pub fn f2(s: &mut dyn Session) {
    split_angle(s, "wrong_checkshape_2.brep", "F2");
}
