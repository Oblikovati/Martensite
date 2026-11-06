//! Display and selection modes changed on displayed boxes, and highlighted isolines built on
//! the triangulation.
use super::prelude::*;
use crate::case_view::viewer_session::ViewParameters;
use martensite_tkv3d::ais::AisShape;
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;

/// The banner of `mode_change_with_selection`.
const MODE_CHANGE_WITH_SELECTION_BANNER: &[&str] = &[
    "================================================================",
    "CR25466: Visualization - Impossible to change the display mode when a local context is opened",
    "================================================================",
    "",
];

/// `bug25466`: the display mode of a box changes while its face selection mode is active.
pub(crate) fn mode_change_with_selection() -> Result<(), Box<dyn Error>> {
    banner(MODE_CHANGE_WITH_SELECTION_BANNER);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session().set_display_mode(0)?;
    show(&mut run, "b1", &box_at([0., 0., 0.], [1., 2., 3.])?)?;
    show(&mut run, "b2", &box_at([3., 0., 0.], [2., 3., 1.])?)?;
    run.session().fit()?;
    run.session().set_object_selection_mode("b1", 4, true)?;
    run.session().set_object_display_mode("b1", 1)?;
    run.session().check_color(100, 100, [0.78, 0.55, 0.09])?;
    run.dump("bug25466.png")?;
    run.finish()
}

/// The banner of `detected_edge_dumped`.
const DETECTED_EDGE_DUMPED_BANNER: &[&str] = &[
    "================================================================",
    "CR23525: Disappearing of highlight in screenshot",
    "================================================================",
    "",
];

/// `bug23525`: an edge detected under the cursor stays highlighted in the dump.
pub(crate) fn detected_edge_dumped() -> Result<(), Box<dyn Error>> {
    banner(DETECTED_EDGE_DUMPED_BANNER);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    show(&mut run, "t_box", &box_sized([100., 100., 100.])?)?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().set_selection_mode(2, true)?;
    run.session().move_cursor(110, 352)?;
    run.session().check_color(110, 352, [0., 1., 1.])?;
    run.dump("bug23525.png")?;
    run.finish()
}

/// The banner of `vertex_after_shaded_mode`.
const VERTEX_AFTER_SHADED_MODE_BANNER: &[&str] = &[
    "================================================================",
    "OCC25251: Visualization - Vertex disappearing from the view after switching display mode",
    "================================================================",
    "",
];

/// `bug25251`: a vertex stays drawn and highlightable after the shaded mode is set.
pub(crate) fn vertex_after_shaded_mode() -> Result<(), Box<dyn Error>> {
    banner(VERTEX_AFTER_SHADED_MODE_BANNER);
    let (check, away) = ((204, 204), (100, 100));
    let mut run = SessionRun::new()?;
    show(&mut run, "v", &vertex_at([0., 0., 0.])?)?;
    run.session().check_color(check.0, check.1, [1., 1., 0.])?;
    run.session().move_cursor(check.0, check.1)?;
    run.session().check_color(check.0, check.1, [0., 1., 1.])?;
    run.session().set_display_mode(1)?;
    run.session().move_cursor(check.0, check.1)?;
    run.session().check_color(check.0, check.1, [1., 1., 0.])?;
    run.session().move_cursor(away.0, away.1)?;
    run.session().check_color(check.0, check.1, [1., 1., 0.])?;
    run.dump("bug25251.png")?;
    run.finish()
}

fn vertex_at([x, y, z]: [f64; 3]) -> Result<Shape, Box<dyn Error>> {
    use martensite_tkmath::gp::Point3;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;
    Ok(BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z))?
        .vertex()?
        .into_shape())
}

/// The banner of `untriangulated_shaded_cone`.
const CR23200_BANNER: &[&str] = &[
    "================================================================",
    "CR23200",
    "Check that the shape doesn't have a shading presentation",
    "due to its triangulation isn't computed in case of disabled auto triangulation feature.",
    "But in this case the shape should have special wireframe presentation.",
    "================================================================",
    "",
];

/// A shape object in its own shaded mode that never triangulates its shape.
fn untriangulated_shaded(shape: &Shape) -> martensite_tkernel::standard::Handle<AisShape> {
    let object = AisShape::new(shape);
    object.attributes().set_auto_triangulation(false);
    object.set_display_mode(1);
    object
}

/// `bug23200_1`: a shaded cone without automatic triangulation shows its special wireframe.
pub(crate) fn untriangulated_shaded_cone() -> Result<(), Box<dyn Error>> {
    banner(CR23200_BANNER);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session()
        .display_object("c", untriangulated_shaded(&cone(0., 5., 10.)?).cast())?;
    run.session().fit()?;
    let color = run.session().pixel_color_name(200, 77)?;
    run.dump("bug23200_1_shape_pres.png")?;
    if color != "GRAY75" {
        println!("Error: shading presentation of shape is incorrect");
    }
    run.finish()
}

/// The camera of `bug25300_2`, set in three steps in order.
fn bug25300_cameras() -> [ViewParameters; 3] {
    [
        ViewParameters {
            scale: Some(122.2874),
            proj: Some([0.0353, -0.9281, 0.3706]),
            ..ViewParameters::default()
        },
        ViewParameters {
            up: Some([-0.2791, 0.3469, 0.8954]),
            at: Some([-5.5784, 62.4399, 122.7242]),
            ..ViewParameters::default()
        },
        ViewParameters {
            eye: Some([12.0732, -401.6167, 308.0344]),
            ..ViewParameters::default()
        },
    ]
}

/// `bug25300_2`: the highlighted wireframe of a shaded cone shows a cyan isoline at (347, 179)
/// that, built on the triangulation, moves off that pixel.
pub(crate) fn isolines_on_triangulation() -> Result<(), Box<dyn Error>> {
    banner(&["============", "CR25300", "============", ""]);
    banner(&[
        "Visualization - Build wireframe representation consistent with the shape's triangulation",
    ]);
    let a_cone = cone(90., 0., 150.)?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    show(&mut run, "aCone", &a_cone)?;
    for camera in &bug25300_cameras() {
        run.session().set_view_parameters(camera)?;
    }
    run.session().set_display_mode(1)?;
    run.session().move_cursor(300, 300)?;
    run.dump("bug25300_2_iso.png")?;
    run.session().check_color(347, 179, [0., 1., 1.])?;
    run.session().move_cursor(0, 0)?;
    run.session().set_iso_on_triangulation("aCone", true)?;
    run.session().move_cursor(300, 300)?;
    run.dump("bug25300_2_iso_on_triang.png")?;
    run.session().check_color(347, 179, [0., 0., 0.])?;
    run.finish()
}
