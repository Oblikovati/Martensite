//! heal drop small edges regression cases
//! whose inputs are in the public dataset: each restores `a`, then the grid end runs
//! `puts [fixsmall result a 1]` and, when `result` is drawn, `puts [checkshape result]`;
//! tests/heal/end shows `result` with `checkview -display result -2d`, or prints
//! `Error : the resulting shape is not done.` when fixsmall set nothing.
use crate::support::{
    checkshape::checkshape,
    log,
    restore::restore_as,
    session::{Session, checkview_display, set},
    swdraw::fixsmall,
};

/// `restore file a`, then the grid end.
fn drop_small_edges(s: &mut dyn Session, file: &str, image: &str) {
    let a = restore_as(s, file, "a").shape();
    let printed = fixsmall(&a, Some(1.));
    let result = printed.shape.map(|shape| set(s, "result", shape));
    log::puts(&printed.text);
    if let Some(result) = &result {
        checkshape(result);
        checkview_display(s, Some("result"), image);
    } else {
        log::puts("Error : the resulting shape is not done.");
    }
}

/// tests/heal/drop_small_edges/A4.
pub fn a4(s: &mut dyn Session) {
    log::todo("Error : the resulting shape is not done.");
    drop_small_edges(s, "DSE3.rle", "A4");
}

/// tests/heal/drop_small_edges/A5.
pub fn a5(s: &mut dyn Session) {
    drop_small_edges(s, "ForDSE.rle", "A5");
}
