//! Shape scenes: persistence, zoom and fitting, shaded tori, styled and transparent shapes.
use super::prelude::*;
use crate::case_view::viewer_session::{PersistenceAnchor, SurfaceAspects};
use crate::shape_checks::{MassCheck, MassKind, check_mass, check_shape_valid};
use crate::shape_mesh::{MeshRequest, mesh_shape};
use martensite_tkernel::quantity::ColorRgba;
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeTorus;
use martensite_tkservice::aspect::{AspectInteriorStyle, AspectTypeOfLine};
use martensite_tkservice::graphic3d::Graphic3dTransModeFlags;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;

const OCC21757: &str =
    "OCC21757: Funny (giraffe-like) shading of torus meshed with specific parameters (WNT)";
/// The expected torus colour at the view centre.
const TORUS_COLOR: [f64; 3] = [0.89, 0.63, 0.109];

/// The banner of `persistent_box_highlight`.
const PERSISTENT_BOX_HIGHLIGHT_BANNER: &[&str] = &[
    "========",
    "OCC28361 Visualization, TKV3d - buggy behavior of Transformation Persistence compiled on several Linux platforms in optimized mode",
    "========",
];

/// `bug28361`: a zoom-rotate persistent box is highlighted under the cursor.
pub(crate) fn persistent_box_highlight() -> Result<(), Box<dyn Error>> {
    banner(PERSISTENT_BOX_HIGHLIGHT_BANNER);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let b = box_sized([10., 10., 10.])?;
    let anchor = PersistenceAnchor::Point([90., 90., 90.]);
    let request = DisplayRequest {
        persistence: Some((Graphic3dTransModeFlags::ZOOM_ROTATE_PERS, anchor)),
        ..DisplayRequest::default()
    };
    run.session().display("b", &b, &request)?;
    run.session().move_cursor(266, 164)?;
    if run.session().pixel_color_name(256, 174)? != "CYAN" {
        println!("Error: the box is not highlighted");
    }
    run.dump("bug28361.png")?;
    run.finish()
}

/// The banner of `zoomed_view_not_clipped`.
const ZOOMED_VIEW_NOT_CLIPPED_BANNER: &[&str] = &[
    "================================================================",
    "0024996: Visualization - newly displayed objects are clipped until first camera movement",
    "================================================================",
    "",
];

/// `bug24996`: objects displayed after a zoom out are drawn unclipped.
pub(crate) fn zoomed_view_not_clipped() -> Result<(), Box<dyn Error>> {
    banner(ZOOMED_VIEW_NOT_CLIPPED_BANNER);
    let wireframe = DisplayRequest::default().with_display_mode(0);
    let boxes = [
        ("b1", [0., 0., 0.]),
        ("b2", [3., 2., 1.]),
        ("b3", [5., -4., 0.]),
    ];
    let shapes = boxes.map(|(name, corner)| (name, box_at(corner, [1., 2., 3.])));
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    for (index, (name, shape)) in shapes.into_iter().enumerate() {
        run.session().display(name, &shape?, &wireframe)?;
        if index == 0 {
            run.session().fit()?;
            run.session().zoom(0.25)?;
        }
    }
    run.dump("bug24996.png")?;
    run.finish()
}

/// The 45 degree torus segment of radii 200 and 50, at `origin` along +Z.
fn torus_segment(origin: [f64; 3]) -> Result<Shape, Box<dyn Error>> {
    let [x, y, z] = origin;
    let axes = Axis2::new(Point3::new(x, y, z), Direction3::new(0., 0., 1.)?)?;
    let angle = 45. * (std::f64::consts::PI / 180.0);
    let mut maker = BRepPrimAPIMakeTorus::with_axes_angle(axes, 200., 50., angle)?;
    Ok(maker.shape()?.clone())
}

/// A shaded view of the torus segment meshed at 0.4, fitted.
fn meshed_torus_view() -> Result<(SessionRun, Shape), Box<dyn Error>> {
    banner(&["================", OCC21757, "================", ""]);
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    run.session().clear()?;
    let result = torus_segment([0., 0., 0.])?;
    mesh_shape(&result, &MeshRequest::new(0.4))?;
    show(&mut run, "result", &result)?;
    run.session().fit()?;
    Ok((run, result))
}

/// `bug21757_1`: the meshed torus segment is shaded with its material colour at the centre.
pub(crate) fn meshed_torus_color() -> Result<(), Box<dyn Error>> {
    let (mut run, _) = meshed_torus_view()?;
    run.session().check_color(200, 200, TORUS_COLOR)?;
    run.dump("bug21757_1.png")?;
    run.finish()
}

/// `bug21757`: the meshed torus segment keeps its area and validity.
pub(crate) fn meshed_torus_area() -> Result<(), Box<dyn Error>> {
    let (mut run, result) = meshed_torus_view()?;
    check_mass(&result, &MassCheck::near(MassKind::Area, 65069.2))?;
    check_shape_valid(&result)?;
    run.dump("bug21757.png")?;
    run.finish()
}

/// `bug21757_2`: the torus segment on a raised plane, shaded at quality 0.0004.
pub(crate) fn planar_torus_color() -> Result<(), Box<dyn Error>> {
    banner(&["================", OCC21757, "================", ""]);
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    run.session().clear()?;
    let result = torus_segment([0., 0., 200.])?;
    show(&mut run, "result", &result)?;
    run.session().set_shading_quality("result", 0.0004)?;
    run.session().fit()?;
    run.session().check_color(200, 200, TORUS_COLOR)?;
    run.dump("bug21757_2.png")?;
    run.finish()
}

/// The banner of `dashed_hollow_sphere`.
const DASHED_HOLLOW_SPHERE_BANNER: &[&str] =
    &["================", "BUC60738", "================", ""];

/// `buc60738`: a shaded sphere turned hollow with dashed green edges.
pub(crate) fn dashed_hollow_sphere() -> Result<(), Box<dyn Error>> {
    banner(DASHED_HOLLOW_SPHERE_BANNER);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    let s = translated(&sphere(20.)?, [-40., 0., 0.])?;
    let shaded = DisplayRequest::default().with_display_mode(1);
    run.session().display("s", &s, &shaded)?;
    run.session().fit()?;
    let aspects = dashed_green_edges()?;
    run.session().set_surface_aspects("s", &aspects)?;
    if !run.session().check_color(261, 314, [0., 1., 0.])? {
        println!("Error : The style of edge is NOT dash");
    }
    run.dump_with_shift_controls("buc60738.png")?;
    run.finish()
}

/// A hollow interior with dashed green edges.
fn dashed_green_edges() -> Result<SurfaceAspects, Box<dyn Error>> {
    Ok(SurfaceAspects {
        interior_style: Some(AspectInteriorStyle::Empty),
        draw_edges: Some(true),
        edge_type: Some(AspectTypeOfLine::Dash),
        edge_color: Some(ColorRgba::new(0., 1., 0., 1.)?),
        ..SurfaceAspects::default()
    })
}

/// `buc60740`: a half-transparent shaded cylinder over a shaded box.
pub(crate) fn transparent_cylinder() -> Result<(), Box<dyn Error>> {
    banner(&["==========", "BUC60740", "=========="]);
    let mut run = SessionRun::new()?;
    let b = box_sized([10., 10., 10.])?;
    let c = cylinder(10., 10.)?;
    for (name, shape) in [("b", &b), ("c", &c)] {
        show(&mut run, name, shape)?;
        run.session().set_object_display_mode(name, 1)?;
    }
    run.session().fit()?;
    run.session().set_transparency("c", 0.5)?;
    if !run.session().check_color(154, 257, [0.6, 0.4, 0.07])? {
        println!("Error :function vsettransparency works WRONGLY");
    }
    run.dump("buc60740.png")?;
    run.finish()
}

/// The banner of `erased_object_not_fitted`.
const ERASED_OBJECT_NOT_FITTED_BANNER: &[&str] = &[
    "============",
    "OCC24610: Erased AIS object should not be taken into account by FitAll operation",
    "============",
    "",
];

/// `bug24610`: fitting ignores an erased object, so the view changes after the erase.
pub(crate) fn erased_object_not_fitted() -> Result<(), Box<dyn Error>> {
    banner(ERASED_OBJECT_NOT_FITTED_BANNER);
    let mut run = SessionRun::new()?;
    show(&mut run, "b1", &box_sized([200., 200., 200.])?)?;
    run.session().fit()?;
    show(&mut run, "b2", &box_sized([2., 2., 2.])?)?;
    let before = run.session().pixel_rgba(32, 305)?;
    run.session().erase(&["b1"])?;
    run.session().fit()?;
    if before == run.session().pixel_rgba(32, 305)? {
        println!("ERROR: OCC24610 is reproduced");
    }
    run.dump("bug24610_2.png")?;
    run.finish()
}
