//! Edge regularity after `encoderegularity` (OCC27383): fused boxes, split and sewn
//! cylinders and spheres, and a fused solid, each checked edge by edge, then shown with hidden
//! lines removed; and a perspective box with exact hidden-line removal (OCC28780).
use super::prelude::*;
use super::sewing::{SewingOptions, sew};
use crate::boolean_typed_explode::all_distinct;
use crate::common_operation::direct;
use crate::primitive_shapes::{
    box_at, box_sized, cylinder, cylinder_sector, mirrored, rotated_about, sphere, sphere_sector,
    translated,
};
use martensite_tkbo::bop_algo::{BOPAlgoBOP, BOPAlgoPaveFiller, Glue, Operation};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::GeomPlane;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkservice::graphic3d::Graphic3dCameraProjection;
use martensite_tkshhealing::shape_fix::ShapeFix;
use martensite_tkshhealing::shape_upgrade::{
    ShapeUpgradeShapeDivideAngle, ShapeUpgradeShapeDivideClosed,
};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPINurbsConvert};
use martensite_tktopalgo::brep_gprop::BRepGProp;
use martensite_tktopalgo::brep_lib::{BRepLibMakeFace, encode_regularity};
use martensite_tkv3d::prs3d::Prs3dTypeOfHlr;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;

/// The `getedgeregularity` name of a continuity.
fn regularity_name(continuity: Continuity) -> &'static str {
    match continuity {
        Continuity::G1 => "G1",
        Continuity::C1 => "C1",
        Continuity::G2 => "G2",
        Continuity::C2 => "C2",
        Continuity::C3 => "C3",
        Continuity::CN => "CN",
        _ => "C0",
    }
}

/// `getedgeregularity edge f1 f2` names `expected`.
fn has_regularity(edge: &Shape, faces: [&Shape; 2], expected: &str) -> Result<bool, Failure> {
    let [f1, f2] = faces;
    let continuity = BRepTool::continuity_on_faces(
        &topods::edge(edge)?,
        &topods::face(f1)?,
        &topods::face(f2)?,
    )?;
    Ok(regularity_name(continuity) == expected)
}

/// The script's check of one edge between two faces.
fn check_regularity(edge: &Shape, faces: [&Shape; 2], expected: &str) -> Outcome {
    if !has_regularity(edge, faces, expected)? {
        println!("Error: Invalid regularity of the edge, expected {expected}");
    }
    Ok(())
}

/// `encoderegularity r` with the default angular tolerance.
fn encoded(shape: &Shape) -> Outcome {
    encode_regularity(shape, 1.0e-10)?;
    Ok(())
}

/// `vdisplay r`, `vfit`, `vhlr on`, `vdump`.
fn hidden_line_view(r: &Shape, image: &'static str) -> Outcome {
    let mut run = SessionRun::new()?;
    run.session().display("r", r, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().set_hidden_line_removal(true, None)?;
    run.dump(image)?;
    run.finish()
}

/// `nurbsconvert`.
fn nurbs(shape: &Shape) -> Result<Shape, Failure> {
    let mut converter = BRepBuilderAPINurbsConvert::new();
    converter.perform(shape, false)?;
    Ok(converter.shape()?.clone())
}

/// The command-host shape-division commands end with `ShapeFix::SameParameter` of the result.
fn same_parameter(result: Shape) -> Result<Shape, Failure> {
    ShapeFix::same_parameter(
        &result,
        false,
        0.0,
        &ProgressRange::default(),
        &Handle::default(),
    )?;
    Ok(result)
}

/// `faces[i]`, `edges[j]` as `explode` numbers them from 1.
fn nth(shape: &Shape, kind: ShapeEnum, number: usize) -> Shape {
    all_distinct(shape, kind).swap_remove(number - 1)
}

/// `bug27383_1`: the edge shared by two coplanar faces of fused boxes is CN.
pub(crate) fn fused_boxes_regularity() -> Outcome {
    let fused = direct(
        &box_sized([10.; 3])?,
        &box_at([5., 5., 0.], [10.; 3])?,
        Operation::Fuse,
    );
    let (r3, r7) = (
        nth(&fused, ShapeEnum::Face, 3),
        nth(&fused, ShapeEnum::Face, 7),
    );
    encoded(&fused)?;
    check_regularity(&nth(&r7, ShapeEnum::Edge, 4), [&r3, &r7], "CN")?;
    hidden_line_view(&fused, "bug27383_1.png")
}

/// `bug27383_2`: the seam between the halves of a cylinder split at 95 degrees is CN.
pub(crate) fn split_angle_regularity() -> Outcome {
    let tool = ShapeUpgradeShapeDivideAngle::with_shape(95.0_f64.to_radians(), cylinder(2., 10.)?);
    tool.perform(true)?;
    let r = same_parameter(tool.result())?;
    encoded(&r)?;
    let (r1, r2) = (nth(&r, ShapeEnum::Face, 1), nth(&r, ShapeEnum::Face, 2));
    check_regularity(&nth(&r1, ShapeEnum::Edge, 2), [&r1, &r2], "CN")?;
    hidden_line_view(&r, "bug27383_2.png")
}

/// `bug27383_3`: two half spheres sewn: every non-degenerated edge is G2, a degenerated one C0.
pub(crate) fn sewn_spheres_regularity() -> Outcome {
    let s1 = nth(&sphere_sector(10., 180.)?, ShapeEnum::Face, 1);
    let s2 = nth(&sphere_sector(10., 180.)?, ShapeEnum::Face, 1);
    let s2 = mirrored(&s2, [0.; 3], [0., 1., 0.])?;
    let s2 = rotated_about(&s2, [0.; 3], [0., 1., 0.], 45.)?;
    let r = sew(&[s1, s2], &SewingOptions::DEFAULT)?;
    encoded(&r)?;
    let (r1, r2) = (nth(&r, ShapeEnum::Face, 1), nth(&r, ShapeEnum::Face, 2));
    for edge in all_distinct(&r1, ShapeEnum::Edge) {
        let mut props = GProps::default();
        BRepGProp::linear_properties(&edge, &mut props, false, false)?;
        let expected = if props.mass().abs() > 1.0e-7 {
            "G2"
        } else {
            "C0"
        };
        check_regularity(&edge, [&r1, &r2], expected)?;
    }
    hidden_line_view(&r, "bug27383_3.png")
}

/// `bug27383_4`: a NURBS cylinder face split where closed: both split edges are G2.
pub(crate) fn closed_split_regularity() -> Outcome {
    let face = nth(&cylinder(2., 10.)?, ShapeEnum::Face, 1);
    let tool = ShapeUpgradeShapeDivideClosed::new(nurbs(&face)?);
    tool.perform(true)?;
    let r = same_parameter(tool.result())?;
    encoded(&r)?;
    let (r1, r2) = (nth(&r, ShapeEnum::Face, 1), nth(&r, ShapeEnum::Face, 2));
    for number in [2, 4] {
        check_regularity(&nth(&r1, ShapeEnum::Edge, number), [&r1, &r2], "G2")?;
    }
    hidden_line_view(&r, "bug27383_4.png")
}

/// `bug27383_5`: a quarter cylinder face sewn to its copy turned half way about a parallel
/// axis: the shared edge is C1.
pub(crate) fn turned_copy_regularity() -> Outcome {
    let quarter = nth(&cylinder_sector(2., 10., 90.)?, ShapeEnum::Face, 1);
    let turned = rotated_about(&quarter, [0., 2., 0.], [0., 0., 1.], 180.)?;
    let r = sew(&[quarter, turned], &SewingOptions::DEFAULT)?;
    encoded(&r)?;
    let (r1, r2) = (nth(&r, ShapeEnum::Face, 1), nth(&r, ShapeEnum::Face, 2));
    check_regularity(&nth(&r1, ShapeEnum::Edge, 2), [&r1, &r2], "C1")?;
    hidden_line_view(&r, "bug27383_5.png")
}

/// `mkface pln p 0 10 -2 0` on the plane through (0, 2, 0) normal to Y.
fn tangent_plane_face() -> Result<Shape, Failure> {
    let plane = GeomPlane::from_point(Point3::new(0., 2., 0.), Direction3::new(0., 1., 0.)?)?;
    let bounds = (0., 10., -2., 0.);
    let maker = BRepLibMakeFace::from_surface_bounds(
        &Handle::new(plane).cast(),
        bounds,
        precision::confusion(),
    )?;
    Ok(maker.face()?.into_shape())
}

/// Sew `face` to the tangent plane face, encode, and check the shared edge is G1.
fn sewn_to_plane(face: &Shape, plane: &Shape) -> Result<Shape, Box<dyn Error>> {
    let r = sew(&[face.clone(), plane.clone()], &SewingOptions::DEFAULT)?;
    encoded(&r)?;
    let (r1, r2) = (nth(&r, ShapeEnum::Face, 1), nth(&r, ShapeEnum::Face, 2));
    check_regularity(&nth(&r1, ShapeEnum::Edge, 2), [&r1, &r2], "G1")?;
    Ok(r)
}

/// `bug27383_6`: a quarter cylinder face, then its NURBS form, sewn to a tangent plane face.
pub(crate) fn tangent_plane_regularity() -> Outcome {
    let pln = tangent_plane_face()?;
    let quarter = nth(&cylinder_sector(2., 10., 90.)?, ShapeEnum::Face, 1);
    sewn_to_plane(&quarter, &pln)?;
    let r = sewn_to_plane(&nurbs(&quarter)?, &pln)?;
    hidden_line_view(&r, "bug27383_6.png")
}

/// `baddobjects`, `baddtools`, `bfillds`, `bbop r 1` with the original command defaults:
/// one pave filler over every argument, then the fuse over it.
fn fused_arguments(objects: &[Shape], tools: &[Shape]) -> Shape {
    let arguments = objects.iter().chain(tools).cloned().collect::<List<_>>();
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(&arguments);
    filler.options_mut().set_run_parallel(false);
    filler.options_mut().set_fuzzy_value(precision::confusion());
    filler.options_mut().set_use_obb(false);
    filler.set_non_destructive(false);
    filler.set_glue(Glue::Off);
    filler.perform();
    assert!(
        !filler.options().has_errors(),
        "bfillds: intersection failed"
    );
    let mut operation = BOPAlgoBOP::new();
    objects
        .iter()
        .for_each(|object| operation.add_argument(object));
    tools.iter().for_each(|tool| operation.add_tool(tool));
    operation.set_operation(Operation::Fuse);
    operation.options_mut().set_run_parallel(false);
    operation.set_check_inverted(true);
    operation.set_to_fill_history(true);
    operation.perform_with_filler(&mut filler);
    assert!(!operation.options().has_errors(), "bbop r 1: fuse failed");
    operation.shape().clone()
}

/// The fused half cylinders and capping spheres of `bug27383_7`; the script's unused
/// `nurbsconvert q p2` is left out, it changes no argument.
fn fused_capsule() -> Result<Shape, Box<dyn Error>> {
    let p1 = cylinder_sector(2., 10., 180.)?;
    let p2 = mirrored(&cylinder_sector(2., 10., 180.)?, [0.; 3], [0., 1., 0.])?;
    let s1 = translated(&sphere(2.)?, [0., 0., 10.])?;
    let s2 = rotated_about(&sphere(2.)?, [0.; 3], [0., 1., 0.], -90.)?;
    Ok(fused_arguments(&[p1, p2], &[s1, s2]))
}

/// The number of edges shared by two faces of `r` (`compare` finds them the same), reporting
/// each that is C0.
fn shared_edges_checked(r: &Shape) -> Result<usize, Failure> {
    let faces = all_distinct(r, ShapeEnum::Face);
    let mut shared = 0;
    for (i, f1) in faces.iter().enumerate() {
        for f2 in &faces[i + 1..] {
            let edges2 = all_distinct(f2, ShapeEnum::Edge);
            for e1 in all_distinct(f1, ShapeEnum::Edge) {
                for _ in edges2.iter().filter(|e2| e1.is_same(e2)) {
                    shared += 1;
                    if has_regularity(&e1, [f1, f2], "C0")? {
                        println!("Error: Invalid regularity of the edge, expected at least G1");
                    }
                }
            }
        }
    }
    Ok(shared)
}

/// `bug27383_7`: every edge shared by two faces of a fused capsule is at least G1.
pub(crate) fn fused_capsule_regularity() -> Outcome {
    let r = fused_capsule()?;
    encoded(&r)?;
    let shared = shared_edges_checked(&r)?;
    if shared == 6 {
        println!("OK: Number of processed edges: {shared}");
    } else {
        println!("Error: incorrect number of processed edges ({shared} instead of 6)");
    }
    hidden_line_view(&r, "bug27383_7.png")
}

/// `bug28780`: a box seen from the top in perspective with the exact hidden-line algorithm.
pub(crate) fn perspective_hidden_lines_view() -> Outcome {
    banner(&["=======", "0028780", "=======", ""]);
    let a = box_sized([10., 20., 50.])?;
    let mut run = SessionRun::new()?;
    run.session().display("a", &a, &DisplayRequest::default())?;
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    run.session().fit()?;
    run.session()
        .set_projection(Graphic3dCameraProjection::Perspective)?;
    // `vhlrtype algo` with no names only changes the default drawer while the view does not
    // remove hidden lines yet; `vhlr on` then turns it on: one call with the algorithm.
    run.session()
        .set_hidden_line_removal(true, Some(Prs3dTypeOfHlr::Algo))?;
    run.dump("bug28780.png")?;
    run.finish()
}
