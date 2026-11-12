//! Fitting the view to sub-shapes and to a redisplayed, smaller bounding box.
use super::prelude::*;
use crate::boolean_typed_explode::all_distinct;
use martensite_tkg3d::top_abs::ShapeEnum;

const FIT_HEADING: [&str; 4] = [
    "============",
    "Box edge and vertex fitting",
    "============",
    "",
];

/// Display sub-shapes `first` and `second` (1-based) of a 100 mm box, fit, and dump.
fn fit_sub_shapes(
    kind: ShapeEnum,
    (first, second): (usize, usize),
    resume: &str,
    image: &'static str,
) -> Result<(), Box<dyn Error>> {
    banner(&FIT_HEADING);
    let result = box_sized([100., 100., 100.])?;
    let parts = all_distinct(&result, kind);
    let mut run = SessionRun::new()?;
    let plain = DisplayRequest::default();
    run.session()
        .display(&format!("result_{first}"), &parts[first - 1], &plain)?;
    run.session()
        .display(&format!("result_{second}"), &parts[second - 1], &plain)?;
    run.session().fit()?;
    banner(&["", resume]);
    run.dump(image)?;
    run.finish()
}

/// Two edges of a box fit the view; `box_edges_fit()?`.
pub(crate) fn box_edges_fit() -> Result<(), Box<dyn Error>> {
    fit_sub_shapes(ShapeEnum::Edge, (1, 2), "2 edges are fit", "bug21171_2.png")
}

/// Two vertices of a box fit the view; `box_vertices_fit()?`.
pub(crate) fn box_vertices_fit() -> Result<(), Box<dyn Error>> {
    fit_sub_shapes(
        ShapeEnum::Vertex,
        (8, 1),
        "2 vertexes are fit",
        "bug21171_4.png",
    )
}

/// The bounding-box mode follows a smaller replacement shape before the fit.
pub(crate) fn shrunk_box_bounds_redisplay() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "Visualization - Bounding box is not decreasing for mode 2 #421",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    let bounds = DisplayRequest::default().with_display_mode(2);
    run.session()
        .display("b", &box_sized([100., 200., 300.])?, &bounds)?;
    run.session()
        .display("b", &box_sized([10., 200., 300.])?, &bounds)?;
    run.session().fit()?;
    run.session().check_color(243, 2, [0., 0., 0.])?;
    run.dump("bug_gh421.png")?;
    run.finish()
}
