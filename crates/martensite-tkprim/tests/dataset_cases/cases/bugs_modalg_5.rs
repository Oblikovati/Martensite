//! `bugs_modalg_5` case bodies.
use crate::support::{
    checkshape::checkshape,
    curves::{explode, shape_set},
    log,
    logged_checks::checknbshapes,
    prim::{pcylinder, wedge_axes, wedge_limits},
    props_view::{Expected, checkprops},
    session::{Session, checkview_display},
    topology::tscale,
};
use martensite_tkg3d::top_abs::ShapeEnum;

fn bug22614(s: &mut dyn Session, face: usize, area: f64, counts: [usize; 9], image: &str) {
    log::puts("========================");
    log::puts("OCC22614");
    log::puts("========================");
    log::puts("");
    let p = shape_set(s, "p", pcylinder(10., 50.));
    let faces = explode(s, "p", &p, Some(ShapeEnum::Face));
    let scaled = tscale(&faces[face - 1], [0., 0., 0.], 0.001, false);
    let name = format!("p_{face}");
    shape_set(s, &name, scaled.clone());
    // renamevar p_N result
    let result = shape_set(s, "result", scaled);
    checkprops(s, "result", &result, Expected::s(area));
    checkshape(&result);
    checknbshapes(&result, counts.map(Some), None);
    checkview_display(s, Some("result"), image);
}

/// Scaling the cylindrical face (linear pcurves) of a cylinder.
pub fn bug22614_1(s: &mut dyn Session) {
    bug22614(s, 1, 0.00314159, [2, 3, 1, 1, 0, 0, 0, 0, 7], "bug22614_1");
}

/// Scaling its top planar face.
pub fn bug22614_2(s: &mut dyn Session) {
    bug22614(s, 2, 0.000314159, [1, 1, 1, 1, 0, 0, 0, 0, 4], "bug22614_2");
}

fn bug24144(s: &mut dyn Session, result: crate::support::restore::Restored, image: &str) {
    let result = shape_set(s, "result", result.shape());
    checkprops(s, "result", &result, Expected::s(2.25939e+06));
    checkshape(&result);
    checknbshapes(&result, [6, 9, 5, 5, 1, 1, 0, 0, 27].map(Some), None);
    checkview_display(s, Some("result"), image);
}

fn banner_24144() {
    log::puts("=========");
    log::puts("OCC24144");
    log::puts("=========");
    log::puts("");
}

/// The wedge on the axes (Z, X = Y) with the limits xmin 0, zmin 1075, xmax 200,
/// zmax 1075.
pub fn bug24144_1(s: &mut dyn Session) {
    banner_24144();
    let result = wedge_axes(
        [0., 0., 0.],
        [0., 0., 1.],
        [0., 1., 0.],
        [200.0, 620.0, 2150.0],
        [0., 1075., 200., 1075.],
    );
    bug24144(s, result.into(), "bug24144_1");
}

/// The same wedge on the default axes.
pub fn bug24144_2(s: &mut dyn Session) {
    banner_24144();
    let result = wedge_limits([200.0, 620.0, 2150.0], [0., 1075., 200., 1075.]);
    bug24144(s, result.into(), "bug24144_2");
}
