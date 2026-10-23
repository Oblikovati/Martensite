//! heal fix gaps regression cases whose
//! inputs are in the public dataset: each restores `a`, then the grid end runs
//! `fixwgaps result a 0.001` and `puts [checkshape result]`; end shows
//! `result` with `checkview -display result -2d`.
use crate::support::{
    checkshape::checkshape,
    log,
    restore::restore_as,
    session::{Session, checkview_display, set},
    swdraw::fixwgaps,
};

/// `restore file a`, then the grid end.
fn fix_gaps(s: &mut dyn Session, file: &str, image: &str) {
    let a = restore_as(s, file, "a").shape();
    let printed = fixwgaps(&a, Some(0.001));
    let result = printed
        .shape
        .map(|shape| set(s, "result", shape))
        .expect("checkshape: result is not defined (fixwgaps fixed no gap)");
    log::result(&printed.text);
    checkshape(&result);
    checkview_display(s, Some("result"), image);
}

/// fix gaps — A5.
pub fn a5(s: &mut dyn Session) {
    fix_gaps(s, "CTO900_ger60039a.rle", "A5");
}

/// fix gaps — A6.
pub fn a6(s: &mut dyn Session) {
    fix_gaps(s, "CTO900_ger60041a.rle", "A6");
}

/// fix gaps — B1.
pub fn b1(s: &mut dyn Session) {
    fix_gaps(s, "RevolutionFG.rle", "B1");
}

/// fix gaps — B2.
pub fn b2(s: &mut dyn Session) {
    fix_gaps(s, "boxFG.rle", "B2");
}

/// fix gaps — B3.
pub fn b3(s: &mut dyn Session) {
    fix_gaps(s, "pcpc2_FS_FG.rle", "B3");
}
