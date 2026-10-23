//! heal surface to bezier regression cases
//! whose inputs are in the public dataset: each restores `a`, then the grid end runs
//! `puts [DT_ShapeConvert result a 1 1]`, `set d [statshape result]`, `puts $d` and
//! reports an `Error` when a `bspsur : BSplineSurface` row remains; end shows
//! `result` with `checkview -display result -2d`.
use crate::support::{
    log,
    restore::restore_as,
    session::{Session, checkview_display, set},
    swdraw::{dt_shape_convert, statshape, statshape_count},
};

/// `restore file a`, then the grid end.
fn surface_to_bezier(s: &mut dyn Session, file: &str, image: &str) {
    let a = restore_as(s, file, "a").shape();
    let printed = dt_shape_convert(&a, true, true);
    let result = set(s, "result", printed.shape);
    log::result(&printed.text);
    let d = statshape(&result);
    log::result(&d);
    log::puts(&d);
    let bspsur = statshape_count(&d, "bspsur", "BSplineSurface");
    if bspsur != 0 {
        log::puts(&format!(
            "Error: {bspsur} BSplineSurface after DT_ShapeConvert operation."
        ));
    }
    checkview_display(s, Some("result"), image);
}

/// surface to bezier — A3.
pub fn a3(s: &mut dyn Session) {
    surface_to_bezier(s, "CCH_conge1.rle", "A3");
}

/// surface to bezier — A4.
pub fn a4(s: &mut dyn Session) {
    surface_to_bezier(s, "CCH_propag.rle", "A4");
}

/// surface to bezier — A5.
pub fn a5(s: &mut dyn Session) {
    surface_to_bezier(s, "CCH_r2.rle", "A5");
}

/// surface to bezier — B1.
pub fn b1(s: &mut dyn Session) {
    surface_to_bezier(s, "CFE900_cts20gcu.rle", "B1");
}

/// surface to bezier — C3.
pub fn c3(s: &mut dyn Session) {
    surface_to_bezier(s, "CFI_5_d12fgi.rle", "C3");
}

/// surface to bezier — C4.
pub fn c4(s: &mut dyn Session) {
    surface_to_bezier(s, "CFI_5_l12fgq.rle", "C4");
}

/// surface to bezier — C5.
pub fn c5(s: &mut dyn Session) {
    surface_to_bezier(s, "CFI_6_g12fhd.rle", "C5");
}

/// surface to bezier — C6.
pub fn c6(s: &mut dyn Session) {
    surface_to_bezier(s, "CFI_6_g56fhe.rle", "C6");
}

/// surface to bezier — C9.
pub fn c9(s: &mut dyn Session) {
    surface_to_bezier(s, "CFI_id260039.rle", "C9");
}

/// surface to bezier — D8.
pub fn d8(s: &mut dyn Session) {
    surface_to_bezier(s, "CCH_testkhes.rle", "D8");
}

/// surface to bezier — E1.
pub fn e1(s: &mut dyn Session) {
    surface_to_bezier(s, "CIN900_buc60204.rle", "E1");
}

/// surface to bezier — E2.
pub fn e2(s: &mut dyn Session) {
    surface_to_bezier(s, "CTO900_cts20124a.rle", "E2");
}
