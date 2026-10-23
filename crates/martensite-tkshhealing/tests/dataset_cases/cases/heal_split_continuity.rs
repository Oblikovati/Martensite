//! heal split continuity regression cases
//! whose inputs are in the public dataset: each restores `a`, then the grid end runs
//! `puts [DT_ShapeDivide result a 0.01]`, `set d [statshape result]`, `puts $d` and
//! reports an `Error` when a `c0curv : Curve Only C0` row remains; tests/heal/end shows
//! `result` with `checkview -display result -2d`.
use crate::support::{
    log,
    restore::restore_as,
    session::{Session, checkview_display, set},
    swdraw::{dt_shape_divide, statshape, statshape_count},
};

/// `restore file a`, then the grid end.
fn split_continuity(s: &mut dyn Session, file: &str, image: &str) {
    let a = restore_as(s, file, "a").shape();
    let printed = dt_shape_divide(&a, Some(0.01));
    let result = set(s, "result", printed.shape);
    log::result(&printed.text);
    let d = statshape(&result);
    log::result(&d);
    log::puts(&d);
    let c0curv = statshape_count(&d, "c0curv", "Curve Only C0");
    if c0curv != 0 {
        log::puts(&format!(
            "Error: {c0curv} Curve Only C0 after DT_ShapeDivide operation."
        ));
    }
    checkview_display(s, Some("result"), image);
}

/// tests/heal/split_continuity/A1.
pub fn a1(s: &mut dyn Session) {
    split_continuity(s, "CCH_conge1.rle", "A1");
}

/// tests/heal/split_continuity/A6.
pub fn a6(s: &mut dyn Session) {
    split_continuity(s, "CFI_6_i12fhh.rle", "A6");
}

/// tests/heal/split_continuity/A7.
pub fn a7(s: &mut dyn Session) {
    split_continuity(s, "CFI_6_i56fhi.rle", "A7");
}

/// tests/heal/split_continuity/A9.
pub fn a9(s: &mut dyn Session) {
    split_continuity(s, "CFI_m12__fiv.rle", "A9");
}

/// tests/heal/split_continuity/B3.
pub fn b3(s: &mut dyn Session) {
    split_continuity(s, "CFI_pro6309c.rle", "B3");
}

/// tests/heal/split_continuity/B4.
pub fn b4(s: &mut dyn Session) {
    split_continuity(s, "CFI_pro6309d.rle", "B4");
}
