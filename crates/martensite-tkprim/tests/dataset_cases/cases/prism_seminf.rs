//! tests/prism/seminf case bodies, with the group's `end` script (checkshape and the
//! checkview of `result`).
#![allow(non_snake_case)] // The grid's case names.
use crate::support::{
    checkshape::checkshape,
    curves::{explode, shape_set},
    log,
    prim::{PrismFlag, box_at, prism},
    session::{Session, checkview_display},
};
use martensite_tkg3d::top_abs::ShapeEnum;

/// buc40188: the semi-infinite prism of a box face.
pub fn A1(s: &mut dyn Session) {
    let mb = shape_set(s, "mb", box_at([-0.5; 3], [1.; 3]));
    let faces = explode(s, "mb", &mb, Some(ShapeEnum::Face));
    let result = prism(&faces[0], [1., 0., 0.], PrismFlag::SemiInfinite)
        .unwrap()
        .shape;
    let result = shape_set(s, "result", result);
    // tests/prism/end
    let report = checkshape(&result);
    log::puts(&report.text);
    checkview_display(s, Some("result"), "A1");
}
