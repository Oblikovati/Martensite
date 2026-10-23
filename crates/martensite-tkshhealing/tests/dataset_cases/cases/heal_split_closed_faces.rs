//! heal split closed faces regression cases
//! whose inputs are in the public dataset: each restores `a`, then the grid end runs
//! `puts [DT_ClosedSplit result a]` (which prints nothing) and `puts [checkshape result]`;
//! tests/heal/end shows `result` with `checkview -display result -2d`.
use crate::support::{
    checkshape::checkshape,
    log,
    restore::restore_as,
    session::{Session, checkview_display, set},
    swdraw::dt_closed_split,
};

/// `restore file a`, then the grid end.
fn split_closed_faces(s: &mut dyn Session, file: &str, image: &str) {
    let a = restore_as(s, file, "a").shape();
    let result = set(s, "result", dt_closed_split(&a));
    log::puts("");
    checkshape(&result);
    checkview_display(s, Some("result"), image);
}

/// tests/heal/split_closed_faces/A1.
pub fn a1(s: &mut dyn Session) {
    split_closed_faces(s, "aaa.rle", "A1");
}

/// tests/heal/split_closed_faces/A2.
pub fn a2(s: &mut dyn Session) {
    split_closed_faces(s, "BoxAndConeFS.rle", "A2");
}

/// tests/heal/split_closed_faces/A3.
pub fn a3(s: &mut dyn Session) {
    split_closed_faces(s, "boxFG.rle", "A3");
}

/// tests/heal/split_closed_faces/A4.
pub fn a4(s: &mut dyn Session) {
    split_closed_faces(s, "BUC50070-#53168-good.rle", "A4");
}

/// tests/heal/split_closed_faces/A5.
pub fn a5(s: &mut dyn Session) {
    split_closed_faces(s, "CCH_coco1.rle", "A5");
}

/// tests/heal/split_closed_faces/A6.
pub fn a6(s: &mut dyn Session) {
    split_closed_faces(s, "CCH_testkhes.rle", "A6");
}

/// tests/heal/split_closed_faces/B1.
pub fn b1(s: &mut dyn Session) {
    split_closed_faces(s, "CFI_5_l12fgq.rle", "B1");
}

/// tests/heal/split_closed_faces/B2.
pub fn b2(s: &mut dyn Session) {
    split_closed_faces(s, "CFI_6_g56fhe.rle", "B2");
}

/// tests/heal/split_closed_faces/B3.
pub fn b3(s: &mut dyn Session) {
    split_closed_faces(s, "CFI_8_i1_fia.rle", "B3");
}

/// tests/heal/split_closed_faces/B6.
pub fn b6(s: &mut dyn Session) {
    split_closed_faces(s, "CFI_g1234fio.rle", "B6");
}

/// tests/heal/split_closed_faces/B7.
pub fn b7(s: &mut dyn Session) {
    split_closed_faces(s, "CFI_g5678fip.rle", "B7");
}

/// tests/heal/split_closed_faces/B8.
pub fn b8(s: &mut dyn Session) {
    split_closed_faces(s, "CFI_n12__fiw.rle", "B8");
}

/// tests/heal/split_closed_faces/B9.
pub fn b9(s: &mut dyn Session) {
    split_closed_faces(s, "CFI_o12__fix.rle", "B9");
}

/// tests/heal/split_closed_faces/C3.
pub fn c3(s: &mut dyn Session) {
    split_closed_faces(s, "CFI_pro14fjq.rle", "C3");
}

/// tests/heal/split_closed_faces/C9.
pub fn c9(s: &mut dyn Session) {
    split_closed_faces(s, "CTO900_buc60127-part.rle", "C9");
}

/// tests/heal/split_closed_faces/D3.
pub fn d3(s: &mut dyn Session) {
    split_closed_faces(s, "CTO900_ger60039a.rle", "D3");
}

/// tests/heal/split_closed_faces/D4.
pub fn d4(s: &mut dyn Session) {
    split_closed_faces(s, "CTO900_ger60041a.rle", "D4");
}

/// tests/heal/split_closed_faces/E1.
pub fn e1(s: &mut dyn Session) {
    log::todo("Faulty shapes in variables faulty_1 to faulty_2");
    split_closed_faces(s, "DSE3.rle", "E1");
}

/// tests/heal/split_closed_faces/E2.
pub fn e2(s: &mut dyn Session) {
    split_closed_faces(s, "DSEwithTol1.rle", "E2");
}

/// tests/heal/split_closed_faces/E4.
pub fn e4(s: &mut dyn Session) {
    split_closed_faces(s, "RevolutionFG.rle", "E4");
}

/// tests/heal/split_closed_faces/E5.
pub fn e5(s: &mut dyn Session) {
    split_closed_faces(s, "ForDSE.rle", "E5");
}

/// tests/heal/split_closed_faces/E6.
pub fn e6(s: &mut dyn Session) {
    split_closed_faces(s, "newFS.rle", "E6");
}

/// tests/heal/split_closed_faces/E7.
pub fn e7(s: &mut dyn Session) {
    split_closed_faces(s, "pcpc2_FS_FG.rle", "E7");
}

/// tests/heal/split_closed_faces/E9.
pub fn e9(s: &mut dyn Session) {
    split_closed_faces(s, "RevolutionFG.rle", "E9");
}

/// tests/heal/split_closed_faces/F3.
pub fn f3(s: &mut dyn Session) {
    split_closed_faces(s, "spheraDSE.rle", "F3");
}

/// tests/heal/split_closed_faces/F4.
pub fn f4(s: &mut dyn Session) {
    split_closed_faces(s, "1-Ring.brep", "F4");
}

/// tests/heal/split_closed_faces/F6.
pub fn f6(s: &mut dyn Session) {
    split_closed_faces(s, "bowl_noremove.brep", "F6");
}

/// tests/heal/split_closed_faces/G5.
pub fn g5(s: &mut dyn Session) {
    split_closed_faces(s, "wrong_checkshape_2.brep", "G5");
}
