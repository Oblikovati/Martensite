//! `bugs_modalg_2` case bodies.
use crate::support::{
    checkshape::checkshape,
    curves::{explode, shape_set},
    log,
    prim::{PrismFlag, box_at, prism},
    props_view::{Expected, checkprops},
    session::{Session, checkview_display},
};
use martensite_tkg3d::top_abs::ShapeEnum;

/// Checkshape of the semi-infinite prism of a box face is valid; its area is
/// the infinite-size 8e+100.
pub fn bug318(s: &mut dyn Session) {
    log::puts("========================");
    log::puts(" OCC318 ");
    log::puts("========================");
    log::puts("");
    let mb = shape_set(s, "mb", box_at([-0.5; 3], [1.; 3]));
    let faces = explode(s, "mb", &mb, Some(ShapeEnum::Face));
    let result = prism(&faces[0], [1., 0., 0.], PrismFlag::SemiInfinite)
        .unwrap()
        .shape;
    let result = shape_set(s, "result", result);
    let che = checkshape(&result);
    if che.text.contains("Faulty") {
        log::puts("Faulty OCC318: Result shape is invalid. It was detected by Checkshape command");
    } else {
        log::puts("OCC318 OK: Result shape is valid");
    }
    checkprops(s, "result", &result, Expected::s(8e+100));
    checkview_display(s, Some("result"), "bug318");
}
