//! `bugs_moddata_3` case bodies whose
//! inputs are in the public dataset.
use crate::support::{
    checkshape::checkshape,
    commands::{explode, explode_type},
    distance::distmini,
    geometry::{extrema2d, mk2dcurve_on_face, plane},
    log,
    restore::restore_as,
    session::{Session, checkview_display, set, set_exploded},
    text::llength,
    topology::{SAMEPARAMETER_TOLERANCE, mkface_bounds, sameparameter_in_place},
};
use martensite_tkg3d::top_abs::ShapeEnum;

/// 2D extrema between an arc of circle and an ellipse made p-curves on a plane
/// must not crash.
pub fn bug24627(s: &mut dyn Session) {
    let p = set(s, "p", plane([0., 0., 0.], [0., 0., 1.], [1., 0., 0.]));
    let f = set(s, "f", mkface_bounds(&p, [-1000., 1000., -1000., 1000.]));
    let c = restore_as(s, "bug24627_circle.brep", "c").shape();
    let c_edges = set_exploded(s, "c", explode_type(&c, ShapeEnum::Edge));
    let c2d = set(s, "c2d", mk2dcurve_on_face(&c_edges[0], &f));
    let e = restore_as(s, "bug24627_ellipse.brep", "e").shape();
    let e_edges = set_exploded(s, "e", explode_type(&e, ShapeEnum::Edge));
    let e2d = set(s, "e2d", mk2dcurve_on_face(&e_edges[0], &f));
    extrema2d(&c2d, &e2d);
}

/// Checkshape checks the PolygonOnTriangulation
/// of the edges.
pub fn bug25109(s: &mut dyn Session) {
    log::required("Faulty shapes in variables faulty_1 to faulty_1 ");
    let b = restore_as(s, "bug25109_e1.brep", "b").shape();
    checkshape(&b);
}

/// The copied offset on a C0 surface seems valid.
pub fn bug26522(s: &mut dyn Session) {
    let shape = restore_as(s, "bug26522_s.brep", "s").shape();
    if !checkshape(&shape)
        .text
        .contains("This shape seems to be valid")
    {
        log::puts("Error: copying offset on C0 surface is incorrect");
    }
}

/// `sameparameter result` must not raise.
pub fn bug26837(s: &mut dyn Session) {
    let result = restore_as(s, "bug26837_e.brep", "result").shape();
    let done = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        sameparameter_in_place(&result, SAMEPARAMETER_TOLERANCE, false);
    }));
    if done.is_err() {
        log::puts("Faulty : SameParameter algorithm throws an exception");
    } else {
        log::puts("OK : SameParameter algorithm works properly");
    }
    checkview_display(s, Some("result"), "bug26837");
}

/// `distmini d a_1 a_2 1.0` gives two solutions (9 words in its result).
pub fn bug27565(s: &mut dyn Session) {
    let a = restore_as(s, "bug27565.brep", "a").shape();
    let parts = set_exploded(s, "a", explode(&a));
    let info = distmini(s, "d", &parts[0], &parts[1], Some(1.0))
        .unwrap()
        .text;
    if llength(&info) != 9 {
        log::puts("Error : Extrema is wrong");
    }
}
