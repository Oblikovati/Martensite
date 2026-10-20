//! `bugs_modalg_4` case bodies.
use crate::support::{
    checkshape::checkshape,
    curves::shape_set,
    log,
    prim::psphere_latitudes,
    props_view::{Expected, checkprops},
    session::{Session, checkview_display},
    topology::checksection,
};

/// `psphere result 10 10 10` builds the
/// degenerate band between the latitudes 10 and 10 degrees; checkshape names faulty
/// shapes and its edges have a length where the script expects 0.
pub fn bug957(s: &mut dyn Session) {
    log::todo("Faulty shapes in variables faulty_1 to faulty_");
    log::todo("Error : The length of result shape is");
    log::puts("========");
    log::puts("OCC957");
    log::puts("========");
    log::puts("");
    let result = shape_set(s, "result", psphere_latitudes(10., 10., 10.));
    checkprops(s, "result", &result, Expected::l(0.));
    checkshape(&result);
    let alone = checksection(&result);
    log::result(&format!(" nb alone Vertices : {alone}\n"));
    checkview_display(s, Some("result"), "bug957");
}
