//! heal direct faces regression cases whose
//! inputs are in the public dataset: each restores `a`, then the grid end runs
//! `puts [directfaces result a]`, `set s [statshape result particul]`, `puts $s` and
//! reports an `Error` when an `indsur : Indirect Surface` row remains; end
//! shows `result` with `checkview -display result -2d`.
use crate::support::{
    log,
    restore::restore_as,
    session::{Session, checkview_display, set},
    swdraw::{directfaces, statshape, statshape_count},
};

/// `restore file a`, then the grid end.
fn direct_faces(s: &mut dyn Session, file: &str, image: &str) {
    let a = restore_as(s, file, "a").shape();
    let printed = directfaces(&a, "result");
    let result = printed
        .shape
        .map(|shape| set(s, "result", shape))
        .expect("statshape: Shape unknown : result (directfaces printed No modif)");
    log::result(&printed.text);
    let text = statshape(&result);
    log::result(&text);
    log::puts(&text);
    let indsur = statshape_count(&text, "indsur", "Indirect Surface");
    if indsur != 0 {
        log::puts(&format!(
            "Error: {indsur} Indirect Surface after directfaces operation."
        ));
    }
    checkview_display(s, Some("result"), image);
}

/// direct faces — A2.
pub fn a2(s: &mut dyn Session) {
    direct_faces(s, "CCV_2_e1_gsx.rle", "A2");
}

/// direct faces — A6.
pub fn a6(s: &mut dyn Session) {
    direct_faces(s, "CIN001_b50.rle", "A6");
}

/// direct faces — A7.
pub fn a7(s: &mut dyn Session) {
    direct_faces(s, "CIN001_b51.rle", "A7");
}

/// direct faces — B1.
pub fn b1(s: &mut dyn Session) {
    direct_faces(s, "DF1.rle", "B1");
}

/// direct faces — B7.
pub fn b7(s: &mut dyn Session) {
    direct_faces(s, "revface.rle", "B7");
}

/// direct faces — B8.
pub fn b8(s: &mut dyn Session) {
    direct_faces(s, "shljapa.rle", "B8");
}
