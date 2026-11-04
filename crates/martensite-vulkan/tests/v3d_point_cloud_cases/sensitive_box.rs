//! `sensitivebox`: the bounding-box sensitive of a point cloud, detected through one and then
//! two view clipping planes.
use super::cloud::node_cloud;
use crate::case_view::viewer_session::ViewerSession;
use crate::session_run::{SessionRun, banner, check_point};
use crate::shape_mesh::{MeshRequest, mesh_shape};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::Vec4;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomCartesianPoint, GeomSphericalSurface};
use martensite_tkmath::gp::Axis3;
use martensite_tkservice::graphic3d::{
    GRAPHIC3D_ARRAY_FLAGS_NONE, Graphic3dClipPlane, Graphic3dClipPlaneSource,
};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape as _,
};
use martensite_tkv3d::ais::AisPoint;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;
use std::error::Error;

type Outcome = Result<(), Box<dyn Error>>;

/// The meshed face of a whole sphere of radius 10, deflection 0.01.
pub(super) fn meshed_sphere_face() -> Result<Shape, Box<dyn Error>> {
    let surface = Handle::new(GeomSphericalSurface::new(Axis3::default(), 10.)?).cast();
    let face = BRepBuilderAPIMakeFace::from_surface(&surface, precision::confusion())?
        .shape()?
        .clone();
    mesh_shape(&face, &MeshRequest::new(0.01))?;
    Ok(face)
}

/// A view clipping plane `a x + b y + c z + d >= 0` added to the active view
/// through the plane's equation.
fn view_clip_plane(
    session: &ViewerSession,
    [a, b, c, d]: [f64; 4],
) -> Result<Handle<dyn Graphic3dClipPlaneSource>, Box<dyn Error>> {
    let plane = Graphic3dClipPlane::from_equation(Vec4::new(a, b, c, d))?;
    let plane: Handle<dyn Graphic3dClipPlaneSource> = Handle::new(plane).cast();
    session.view().add_clip_plane(&plane);
    Ok(plane)
}

/// Detect at (x, y), print the case's faulty-selection line, then mark the point and
/// check it against `expected` within 0.0001.
fn detect_and_mark(run: &mut SessionRun, name: &str, faulty: &str, expected: [f64; 3]) -> Outcome {
    run.session().move_cursor(200, 200)?;
    let point = run.session().detected_point()?;
    if point.contains(&f64::MAX) {
        println!("Faulty : {faulty}");
    }
    let [x, y, z] = point;
    let marker = AisPoint::new(&Handle::new(GeomCartesianPoint::new(x, y, z)).cast())?;
    run.session().display_object(name, marker.cast())?;
    check_point(name, point, expected, 0.0001);
    Ok(())
}

/// Dump the axonometric and the top view, then erase the marker and move the cursor away.
fn dump_views(
    run: &mut SessionRun,
    axo: &'static str,
    top: &'static str,
    marker: &str,
    away: (i32, i32),
) -> Outcome {
    run.dump(axo)?;
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    run.dump(top)?;
    run.session().set_axonometric()?;
    run.session().erase(&[marker])?;
    run.session().move_cursor(away.0, away.1)?;
    Ok(())
}

/// The cloud of the meshed sphere's nodes, detectable by its bounding box only.
fn box_sensitive_cloud(run: &mut SessionRun) -> Outcome {
    let face = meshed_sphere_face()?;
    run.session().set_display_mode(1)?;
    let cloud = node_cloud(&face, GRAPHIC3D_ARRAY_FLAGS_NONE)?;
    run.session().display_object("p", cloud.cast())?;
    run.session().set_object_selection_mode("p", 2, true)?;
    run.session().set_axonometric()?;
    run.session().fit()?;
    Ok(())
}

/// `sensitivebox`.
#[allow(clippy::excessive_precision)] // The expected points are the original's, digit for digit.
pub(crate) fn clipped_box_detection() -> Outcome {
    banner(&["========", "Sensitive box selection", "========"]);
    let mut run = SessionRun::new()?;
    box_sensitive_cloud(&mut run)?;
    let first = view_clip_plane(run.session(), [-1., 0., 0., 0.])?;
    let expected = [
        -1.7763568394002505e-15,
        -0.51078486684208357,
        0.59985611160264973,
    ];
    detect_and_mark(&mut run, "pp1", "Selection 1", expected)?;
    dump_views(
        &mut run,
        "sensitivebox_clip1_selection_axo.png",
        "sensitivebox_clip1_selection_top.png",
        "pp1",
        (300, 200),
    )?;
    run.dump("sensitivebox_clip1_no_selection.png")?;
    first.set_equation(Vec4::new(-1., 0., 0., 3.))?;
    let second = view_clip_plane(run.session(), [1., 0., 0., 3.])?;
    let expected = [2.9999999999999991, -3.5107848668420845, 3.5998561116026506];
    detect_and_mark(&mut run, "pp2", "Selection 2", expected)?;
    dump_views(
        &mut run,
        "sensitivebox_clip2_selection_axo.png",
        "sensitivebox_clip2_selection_top.png",
        "pp2",
        (75, 200),
    )?;
    run.dump("sensitivebox_clip2_no_selection.png")?;
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    run.session().move_cursor(250, 200)?;
    run.dump("sensitivebox_clip2_no_selection_top.png")?;
    run.session().set_axonometric()?;
    run.session().fit()?;
    for plane in [first, second] {
        run.session().view().remove_clip_plane(&plane);
    }
    run.session().move_cursor(200, 200)?;
    run.dump("sensitivebox.png")?;
    run.finish()
}
