//! Primitive and swept solids checked, then shown (OCC22588, OCC22798, OCC22818, OCC23170,
//! OCC23282).
use super::prelude::*;
use super::{display_result_view, shaded_fit_screenshot};
use crate::shape_mesh::{MeshRequest, mesh_shape};
use crate::shape_report_checks::{ShapeCounts, check_shape_counts};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomPlane, GeomSphericalSurface, GeomSurface};
use martensite_tkmath::gp::{Axis1, Axis2, Axis3, Direction3, GeneralTransform3, Matrix3, Point3};
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeCone, BRepPrimAPIMakeRevol, BRepPrimAPIMakeSphere,
};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIGTransform, BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace,
    BRepBuilderAPIMakeShape, BRepBuilderAPIMakeVertex, BRepBuilderAPIMakeWire,
    BRepBuilderAPINurbsConvert,
};

/// Counts of a one-face solid: 2 vertices, 3 edges, 1 wire, face, shell and solid.
const ONE_FACE_SOLID: [usize; 9] = [2, 3, 1, 1, 1, 1, 0, 0, 9];

/// A sphere of radius 100 converted to NURBS or scaled by (2, 3, 4).
#[derive(Clone, Copy, Debug)]
pub(crate) enum SphereChange {
    /// `nurbsconvert`.
    Nurbs,
    /// `scalexyz 2 3 4`.
    Scaled,
}

/// `bug22798_1`/`bug22798_2`: a changed sphere keeps one face and its area.
/// Example: `changed_sphere_view(SphereChange::Nurbs, 126261., "bug22798_1.png")?`.
pub(crate) fn changed_sphere_view(change: SphereChange, area: f64, image: &'static str) -> Outcome {
    banner(&["============", "OCC22798", "============", ""]);
    let sphere = BRepPrimAPIMakeSphere::new(100.)?.shape()?.clone();
    let result = match change {
        SphereChange::Nurbs => nurbs(&sphere)?,
        SphereChange::Scaled => scaled(&sphere, [2., 3., 4.])?,
    };
    check_mass(&result, &MassCheck::near(MassKind::Area, area))?;
    check_shape_structure(&result)?;
    check_shape_counts(&result, "result", &ShapeCounts::all(ONE_FACE_SOLID))?;
    let mut run = SessionRun::new()?;
    display_result_view(&mut run, &result, image)?;
    run.finish()
}

fn nurbs(shape: &Shape) -> Result<Shape, Failure> {
    let mut converter = BRepBuilderAPINurbsConvert::new();
    converter.perform(shape, false)?;
    Ok(converter.shape()?.clone())
}

fn scaled(shape: &Shape, [x, y, z]: [f64; 3]) -> Result<Shape, Failure> {
    let mut transform = GeneralTransform3::default();
    transform.set_vectorial_part(Matrix3::new([[x, 0., 0.], [0., y, 0.], [0., 0., z]]));
    let mut maker = BRepBuilderAPIGTransform::new(transform);
    maker.perform(shape, false)?;
    Ok(maker.shape()?.clone())
}

/// `bug23170`: the full parameter range of a sphere of radius 10 makes a valid face.
pub(crate) fn full_sphere_face_view() -> Outcome {
    banner(&["============", "OCC23170", "============", ""]);
    let frame = Axis3::from_axis2(Axis2::new(
        Point3::new(0., 0., 0.),
        Direction3::new(0., 0., 1.)?,
    )?);
    let surface: Handle<dyn GeomSurface> =
        Handle::new(GeomSphericalSurface::new(frame, 10.)?).cast();
    // The regression's literal range, a few ulps from TAU and FRAC_PI_2, is the input.
    #[allow(clippy::approx_constant)]
    let bounds = (0., 6.283185307179588, -1.570796326794897, 1.570796326794897);
    let mut maker =
        BRepBuilderAPIMakeFace::from_surface_bounds(&surface, bounds, precision::confusion())?;
    let result = maker.shape()?.clone();
    check_mass(&result, &MassCheck::near(MassKind::Area, 1256.63))?;
    check_shape_structure(&result)?;
    let mut run = SessionRun::new()?;
    display_result_view(&mut run, &result, "bug23170.png")?;
    run.finish()
}

/// A cone with radii 10 and 0, a height, a sweep angle and an optional 0.1 mesh.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ConeCase {
    /// The banner's bug line.
    pub bug: &'static str,
    /// The cone height.
    pub height: f64,
    /// The sweep angle in degrees.
    pub angle: f64,
    /// Mesh at 0.1 deflection before the checks.
    pub meshed: bool,
    /// The expected area.
    pub area: f64,
    /// The image dumped.
    pub image: &'static str,
}

/// `bug22818_3/4`, `bug23282_1/2`: a partial cone is valid and shows shaded.
/// Example: `partial_cone_view(&case)?`.
pub(crate) fn partial_cone_view(case: &ConeCase) -> Outcome {
    banner(&["================", case.bug, "================", ""]);
    let angle = case.angle * std::f64::consts::PI / 180.;
    let result = BRepPrimAPIMakeCone::with_angle(10., 0., case.height, angle)?
        .shape()?
        .clone();
    if case.meshed {
        mesh_shape(&result, &MeshRequest::new(0.1))?;
    }
    check_shape_structure(&result)?;
    check_mass(&result, &MassCheck::near(MassKind::Area, case.area))?;
    check_shape_counts(&result, "result", &four_faces(4, 7))?;
    shaded_fit_screenshot(&result, case.image)
}

/// Counts of a four-face solid with the given vertices and edges (21 shapes in all).
fn four_faces(vertices: usize, edges: usize) -> ShapeCounts {
    ShapeCounts::all([vertices, edges, 4, 4, 1, 1, 0, 0, 21])
}

/// A triangle in the plane x = 0 revolved half a turn about the Z axis through (0, 10, 0).
#[derive(Clone, Copy, Debug)]
pub(crate) struct RevolvedTriangle {
    /// The third triangle corner (the others are (0,10,0) and (0,10,10)).
    pub corner: [f64; 3],
    /// A point of the triangle's plane.
    pub plane_point: [f64; 3],
    /// The expected area.
    pub area: f64,
    /// The image dumped.
    pub image: &'static str,
}

/// `bug22818_1/2`: a triangle added to an unbounded plane face, revolved by 180 degrees.
pub(crate) fn revolved_triangle_view(case: &RevolvedTriangle) -> Outcome {
    banner(&["================", "OCC22818", "================", ""]);
    let wire = triangle_wire([[0., 10., 0.], [0., 10., 10.], case.corner])?;
    let plane = plane_surface(case.plane_point, [1., 0., 0.], None)?;
    let mut face = BRepBuilderAPIMakeFace::from_surface(&plane, precision::confusion())?
        .shape()?
        .clone();
    BRepBuilder::default().add(&mut face, wire.as_ref())?;
    let result = revolved(&face, [0., 10., 0.], 180.)?;
    check_shape_structure(&result)?;
    check_mass(&result, &MassCheck::near(MassKind::Area, case.area))?;
    check_shape_counts(&result, "result", &four_faces(3, 8))?;
    shaded_fit_screenshot(&result, case.image)
}

/// `bug22588`: a triangle face in the XZ plane revolved by 45 degrees about the Z axis.
pub(crate) fn revolved_wedge_view() -> Outcome {
    banner(&["============", "OCC22588", "============", ""]);
    let wire = triangle_wire([[0., 0., 0.], [0., 0., 160.], [25., 0., 25.]])?;
    let plane = plane_surface([25., 0., 25.], [0., 1., 0.], Some([-25., 0., -25.]))?;
    let face = BRepBuilderAPIMakeFace::from_surface_wire(&plane, &wire, true)?
        .shape()?
        .clone();
    let result = revolved(&face, [0., 0., 0.], 45.)?;
    check_mass(&result, &MassCheck::near(MassKind::Area, 5694.99))?;
    check_shape_structure(&result)?;
    check_shape_counts(&result, "result", &four_faces(3, 8))?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("result", &result, &DisplayRequest::default())?;
    run.session().fit()?;
    run.dump("bug22588.png")?;
    run.finish()
}

/// The closed wire through three new vertices, edge after edge.
fn triangle_wire(corners: [[f64; 3]; 3]) -> Result<topods::Wire, Failure> {
    let vertex =
        |[x, y, z]: [f64; 3]| BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z))?.vertex();
    let vertices = [
        vertex(corners[0])?,
        vertex(corners[1])?,
        vertex(corners[2])?,
    ];
    let mut wire = BRepBuilderAPIMakeWire::new();
    for index in 0..3 {
        let next = &vertices[(index + 1) % 3];
        let edge = BRepBuilderAPIMakeEdge::from_vertices(&vertices[index], next)?.edge()?;
        wire.add_edge(&edge)?;
    }
    wire.wire()
}

/// A plane through `point` with normal `normal` and an optional X direction.
fn plane_surface(
    [px, py, pz]: [f64; 3],
    [nx, ny, nz]: [f64; 3],
    x: Option<[f64; 3]>,
) -> Result<Handle<dyn GeomSurface>, Failure> {
    let origin = Point3::new(px, py, pz);
    let normal = Direction3::new(nx, ny, nz)?;
    let frame = match x {
        Some([xx, xy, xz]) => Axis3::new(origin, normal, Direction3::new(xx, xy, xz)?)?,
        None => Axis3::from_axis2(Axis2::new(origin, normal)?),
    };
    Ok(Handle::new(GeomPlane::new(frame)).cast())
}

/// Revolve `shape` about the Z direction through `origin` by `degrees`.
fn revolved(shape: &Shape, [x, y, z]: [f64; 3], degrees: f64) -> Result<Shape, Failure> {
    let axis = Axis1::new(Point3::new(x, y, z), Direction3::new(0., 0., 1.)?);
    let angle = degrees * (std::f64::consts::PI / 180.0);
    Ok(BRepPrimAPIMakeRevol::new(shape, axis, angle, false)?
        .shape()?
        .clone())
}
