//! Cuts by a half space and general fuses of boxes, wires and dataset shapes, checked, then
//! shown (OCC25937, OCC25982, OCC26253, OCC27270).
use super::display_result_view;
use super::prelude::*;
use crate::common_operation;
use crate::primitive_shapes::{box_at, box_sized};
use crate::shape_report_checks::{ShapeCounts, check_section_vertices, check_shape_counts};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Compound, Shape as TopoShape};
use martensite_tkbrep::toptools::MapOfShape;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomPlane, GeomSurface};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis2, Axis3, Direction3, Point3, Transform3, Vector3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeHalfSpace;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPICopy, BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape,
    BRepBuilderAPITransform,
};

/// The object cut by the half space x < 10 in the OCC25937 regressions.
#[derive(Clone, Copy, Debug)]
pub(crate) enum HalfSpaceObject {
    /// The closed square polyline of side 20 about the origin, in the XY plane.
    Square,
    /// The face of the XY plane bounded to [-10, 10] in U and V.
    Face,
    /// The 20x20x10 box at (-10, -10, 0).
    Box,
}

/// `bug25937_1/2/3`: an object minus the half space beyond x = 10 is unchanged.
/// Example: `half_space_cut_view(HalfSpaceObject::Box, "bug25937_3.png")?`.
pub(crate) fn half_space_cut_view(object: HalfSpaceObject, image: &'static str) -> Outcome {
    banner(&["============", "OCC25937", "============", ""]);
    let plane = plane_surface([1., 0., 0.])?;
    let wall = BRepBuilderAPIMakeFace::from_surface(&plane, precision::confusion())?
        .shape()?
        .clone();
    let half =
        BRepPrimAPIMakeHalfSpace::with_face(&topods::face(&wall)?, Point3::new(10., 0., 0.))?;
    let half = half.solid()?.as_ref().clone();
    let a = half_space_object(object)?;
    let result = common_operation::prepared(&a, &half, Operation::Cut);
    let (counts, masses) = match object {
        HalfSpaceObject::Square => ([4, 3, 1, 0, 0, 0, 0, 1, 9], [Some(40.), None]),
        HalfSpaceObject::Face => ([4, 4, 1, 1, 0, 0, 0, 1, 11], [Some(60.), Some(200.)]),
        HalfSpaceObject::Box => ([8, 12, 6, 6, 1, 1, 0, 1, 35], [None, Some(1000.)]),
    };
    for (kind, mass) in [MassKind::Length, MassKind::Area].into_iter().zip(masses) {
        if let Some(mass) = mass {
            check_mass(&result, &MassCheck::near(kind, mass))?;
        }
    }
    // The face regression checks its section before its validity, the polyline after it.
    if matches!(object, HalfSpaceObject::Face) {
        check_section_vertices(&result)?;
    }
    check_shape_structure(&result)?;
    if matches!(object, HalfSpaceObject::Square) {
        check_section_vertices(&result)?;
    }
    let expected = ShapeCounts {
        per_location: true,
        subject: "Result obtained by Cut a wire by halfspace",
        ..ShapeCounts::all(counts)
    };
    check_shape_counts(&result, "result", &expected)?;
    shown(&result, image)
}

fn half_space_object(object: HalfSpaceObject) -> Result<Shape, Failure> {
    match object {
        HalfSpaceObject::Square => square_polyline(),
        HalfSpaceObject::Face => {
            let plane = plane_surface([0., 0., 1.])?;
            let bounds = (-10., 10., -10., 10.);
            let mut maker = BRepBuilderAPIMakeFace::from_surface_bounds(
                &plane,
                bounds,
                precision::confusion(),
            )?;
            Ok(maker.shape()?.clone())
        }
        HalfSpaceObject::Box => box_at([-10., -10., 0.], [20., 20., 10.]),
    }
}

/// The polyline through the square's corners back to the first one.
fn square_polyline() -> Result<Shape, Failure> {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    for [x, y] in [
        [-10., -10.],
        [10., -10.],
        [10., 10.],
        [-10., 10.],
        [-10., -10.],
    ] {
        polygon.add_point(Point3::new(x, y, 0.))?;
    }
    Ok(polygon.shape()?.clone())
}

/// A plane through the origin with normal `normal`.
fn plane_surface([x, y, z]: [f64; 3]) -> Result<Handle<dyn GeomSurface>, Failure> {
    let frame = Axis3::from_axis2(Axis2::new(
        Point3::new(0., 0., 0.),
        Direction3::new(x, y, z)?,
    )?);
    Ok(Handle::new(GeomPlane::new(frame)).cast())
}

fn shown(result: &Shape, image: &'static str) -> Outcome {
    let mut run = SessionRun::new()?;
    display_result_view(&mut run, result, image)?;
    run.finish()
}

/// `bug27270`: two compounds of a cube's six wires, one copied and moved, fuse into a
/// container with more than two members.
pub(crate) fn fused_wire_compounds_view() -> Outcome {
    banner(&["============", "OCC27270", "============", ""]);
    let c1 = compound(&distinct(&box_sized([200., 200., 200.])?, ShapeEnum::Wire)?)?;
    let copy = BRepBuilderAPICopy::with_shape(&c1, true, false)?
        .shape()?
        .clone();
    let c2 = moved(&copy, [100., 0., 300.])?;
    let result = common_operation::direct(&c1, &c2, Operation::Fuse);
    if members(&result)? > 2 {
        println!("OK: result is correct");
    } else {
        println!("Error: incorrect assembling of sub-shapes in container shapes");
    }
    check_mass(&result, &MassCheck::near(MassKind::Length, 9600.))?;
    check_shape_structure(&result)?;
    shown(&result, "bug27270.png")
}

/// The distinct sub-shapes of `kind` in exploration order.
fn distinct(shape: &Shape, kind: ShapeEnum) -> Result<Vec<Shape>, Failure> {
    let mut seen = MapOfShape::default();
    let mut found = Vec::new();
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape)?;
    while explorer.more() {
        if seen.add(explorer.current().clone()) {
            found.push(explorer.current().clone());
        }
        explorer.next()?;
    }
    Ok(found)
}

fn compound(parts: &[Shape]) -> Result<Shape, Failure> {
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    let mut shape: TopoShape = compound.as_ref().clone();
    for part in parts {
        builder.add(&mut shape, part)?;
    }
    Ok(shape)
}

/// The number of direct members of a container, as an explode without type lists them.
fn members(shape: &Shape) -> Result<usize, Failure> {
    let mut count = 0;
    let mut iterator = martensite_tkbrep::topods::ShapeIterator::new(shape, true, true);
    while iterator.more() {
        count += 1;
        iterator.next();
    }
    Ok(count)
}

fn moved(shape: &Shape, [x, y, z]: [f64; 3]) -> Result<Shape, Failure> {
    let mut transform = Transform3::default();
    transform.set_translation_part(Vector3::new(x, y, z));
    let mut maker = BRepBuilderAPITransform::new(transform);
    maker.perform(shape, false, false)?;
    Ok(maker.shape()?.clone())
}

/// `bug26253`: a box and a dataset shape split each other by the general fuse.
pub(crate) fn general_fuse_view() -> Outcome {
    banner(&["============", "OCC26253", "============", ""]);
    let b1 = box_at([-805., 585., 143.], [407., 30., 8.])?;
    let b2 = read_dataset_shape("bug26253_qs.brep")?;
    let result = common_operation::general_fuse(&b1, &b2);
    check_mass(&result, &MassCheck::near(MassKind::Area, 64832.))?;
    check_shape_structure(&result)?;
    let expected = ShapeCounts {
        per_location: true,
        subject: "result done by General Fuse algorithm",
        ..ShapeCounts::all([40, 68, 36, 36, 11, 6, 0, 1, 198])
    };
    check_shape_counts(&result, "result", &expected)?;
    shown(&result, "bug26253.png")
}
