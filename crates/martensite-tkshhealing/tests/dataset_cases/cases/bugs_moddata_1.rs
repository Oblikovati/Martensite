//! bugs moddata 1 regression cases owned by
//! TKShHealing whose inputs are in the public dataset or built by the script.
use super::bugs_common::banner;
use crate::support::{
    checkshape::checkshape,
    commands::explode_type,
    geom_create::ellipse,
    log, qa_wires,
    restore::{Restored, restore_as},
    session::{Session, checkview_display, set},
};
use martensite_tkg3d::top_abs::ShapeEnum;

/// tests/bugs/moddata_1/buc60868.
/// The free bounds of the shell chain into three wires (`BUC60868`).
pub fn buc60868(s: &mut dyn Session) {
    banner(&["========", "BUC60868", "========", ""]);
    let sh = restore_as(s, "buc60868.brep", "sh").shape();
    checkshape(&sh);
    let (result, text) = qa_wires::buc60868(&sh);
    log::result(&text);
    let result = result
        .map(|shape| set(s, "result", shape))
        .expect("checkshape: result is not a shape (no bound)");
    checkshape(&result);
    let len = explode_type(&result, ShapeEnum::Wire).len();
    if len != 3 {
        log::puts(&format!(
            "Error : the number of curves is {len}, but it should be 3."
        ));
    }
    checkview_display(s, Some("result"), "buc60868");
}

/// tests/bugs/moddata_1/buc60924.
/// The ellipse in the plane z = 1 is planar (`BUC60924 el 0 0 1`).
pub fn buc60924(s: &mut dyn Session) {
    banner(&["========", "BUC60924", "========", ""]);
    let el = ellipse(&[0., 0., 1., 20., 10.]);
    s.set("el", &Restored::Curve(el.clone()));
    let result1 = qa_wires::buc60924(&el, [0., 0., 1.]);
    log::result(&result1);
    if result1 != "The curve is planar !\n" {
        log::puts("BUC60924: Error");
    } else {
        log::puts("BUC60924: OK");
    }
}
