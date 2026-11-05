//! The curve and surface commands of `tests/bugs` scripts, beside those of `surface_commands`.
use crate::support::{commands::bound_shape, session::Session};
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{
    GeomCircle, GeomCurve, GeomCylindricalSurface, GeomEllipse, GeomGeometry, GeomLine,
    GeomOffsetCurve, GeomPlane, GeomRectangularTrimmedSurface, GeomSphericalSurface, GeomSurface,
    GeomToroidalSurface, GeomTrimmedCurve,
};
use martensite_tkmath::gp::{Axis1, Axis2, Axis3, Direction3, Point3, Vector3};
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeCylinder, BRepPrimAPIMakePrism, BRepPrimAPIMakeSphere, BRepPrimAPIMakeTorus,
};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPICopy, BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape,
};

fn point(at: [f64; 3]) -> Point3 {
    Point3::new(at[0], at[1], at[2])
}

fn direction(along: [f64; 3]) -> Direction3 {
    Direction3::new(along[0], along[1], along[2])
        .unwrap_or_else(|failure| panic!("direction {along:?}: {failure:?}"))
}

fn framed(at: [f64; 3], normal: [f64; 3], x: [f64; 3]) -> Axis3 {
    Axis3::new(point(at), direction(normal), direction(x))
        .unwrap_or_else(|failure| panic!("frame {at:?} {normal:?} {x:?}: {failure:?}"))
}

/// A `plane` variable's frame: `plane p x y z`, its normal along Z and its X axis along X.
/// Example: `frame_at([0., 0., 5.])`.
pub fn frame_at(at: [f64; 3]) -> Axis3 {
    framed(at, [0., 0., 1.], [1., 0., 0.])
}

/// A `plane p x y z dx dy dz` variable's frame; `frame_along([0.; 3], [0., 1., 0.])`.
pub fn frame_along(at: [f64; 3], normal: [f64; 3]) -> Axis3 {
    Axis3::from_normal(point(at), direction(normal))
        .unwrap_or_else(|failure| panic!("plane {at:?} {normal:?}: {failure:?}"))
}

/// A `plane p x y z dx dy dz ux uy uz` variable's frame.
/// Example: `frame_framed([0.; 3], [0., 0., 1.], [0., -1., 0.])`.
pub fn frame_framed(at: [f64; 3], normal: [f64; 3], x: [f64; 3]) -> Axis3 {
    framed(at, normal, x)
}

/// `plane p x y z`; `plane_at([0., 0., 5.])`.
pub fn plane_at(at: [f64; 3]) -> Handle<dyn GeomSurface> {
    Handle::new(GeomPlane::new(frame_at(at))).cast()
}

/// `restore [locate_data_file file] name` of a saved surface; a missing file is the script error.
/// Example: `restore_surface("bug25111_s1.draw")`.
pub fn restore_surface(file: &str) -> Handle<dyn GeomSurface> {
    use crate::support::script_error::script_error;
    use martensite_tkernel::storage::IoStream;
    use martensite_tkgeombase::geom_tools;
    let root = std::env::var_os("MARTENSITE_TEST_DATA_DIR").map_or_else(
        || {
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../external/martensite-test-data")
        },
        std::path::PathBuf::from,
    );
    let found = ["brep", "geom", "others"]
        .iter()
        .map(|folder| root.join("dataset-7.9.0").join(folder).join(file))
        .find(|path| path.is_file());
    let Some(path) = found else {
        script_error(&format!("File {file} could not be found"));
    };
    let bytes = std::fs::read(&path).unwrap_or_else(|cause| panic!("{path:?}: {cause}"));
    let end = bytes
        .iter()
        .position(u8::is_ascii_whitespace)
        .unwrap_or(bytes.len());
    assert!(
        bytes.starts_with(b"DrawTrSurf_"),
        "restore {file}: expected a surface file"
    );
    let surface = geom_tools::read_surface(&mut IoStream::from_bytes(bytes[end..].to_vec()));
    assert!(!surface.is_null(), "restore {file}: expected a surface");
    surface
}

/// `plane p x y z dx dy dz ux uy uz`; `plane_framed([183.6, 0., 0.], [1., 0., 0.], [0., 1., 0.])`.
pub fn plane_framed(at: [f64; 3], normal: [f64; 3], x: [f64; 3]) -> Handle<dyn GeomSurface> {
    Handle::new(GeomPlane::new(framed(at, normal, x))).cast()
}

/// `cylinder c radius`: about Z at the origin; `cylinder(20.)`.
pub fn cylinder(radius: f64) -> Handle<dyn GeomSurface> {
    let surface = GeomCylindricalSurface::new(frame_at([0., 0., 0.]), radius)
        .unwrap_or_else(|failure| panic!("cylinder {radius}: {failure:?}"));
    Handle::new(surface).cast()
}

/// `sphere s x y z radius`; `sphere([0., 0., 0.], 10.)`.
pub fn sphere(at: [f64; 3], radius: f64) -> Handle<dyn GeomSurface> {
    let surface = GeomSphericalSurface::new(frame_at(at), radius)
        .unwrap_or_else(|failure| panic!("sphere {radius}: {failure:?}"));
    Handle::new(surface).cast()
}

/// `trim t surface u1 u2 v1 v2`; `trim_surface(&s, -5., 5., 0., 20.)`.
pub fn trim_surface(
    surface: &Handle<dyn GeomSurface>,
    (u1, u2, v1, v2): (f64, f64, f64, f64),
) -> Handle<dyn GeomSurface> {
    let trimmed = GeomRectangularTrimmedSurface::new(surface, (u1, u2), (v1, v2), (true, true))
        .unwrap_or_else(|failure| panic!("trim {u1} {u2} {v1} {v2}: {failure:?}"));
    Handle::new(trimmed).cast()
}

/// `line l x y z dx dy dz`; `line([0., 0., 0.], [0., 0., 1.])`.
pub fn line(at: [f64; 3], along: [f64; 3]) -> Handle<dyn GeomCurve> {
    let axis = Axis1::new(point(at), direction(along));
    Handle::new(GeomLine::new(axis)).cast()
}

/// `circle c x y z dx dy dz ux uy uz radius`; `circle_framed([0.; 3], [0., 0., 1.], [1., 0., 0.], 5.)`.
pub fn circle_framed(
    at: [f64; 3],
    normal: [f64; 3],
    x: [f64; 3],
    radius: f64,
) -> Handle<dyn GeomCurve> {
    let frame = Axis2::with_x_direction(point(at), direction(normal), direction(x))
        .unwrap_or_else(|failure| panic!("circle {at:?}: {failure:?}"));
    let curve = GeomCircle::new(frame, radius)
        .unwrap_or_else(|failure| panic!("circle radius {radius}: {failure:?}"));
    Handle::new(curve).cast()
}

/// `ellipse e x y z dx dy dz ux uy uz major minor`.
/// Example: `ellipse_framed([0.01, 0.01, 0.], [0., 0., 1.], [1., 0., 0.], 0.01, 0.005)`.
pub fn ellipse_framed(
    at: [f64; 3],
    normal: [f64; 3],
    x: [f64; 3],
    major: f64,
    minor: f64,
) -> Handle<dyn GeomCurve> {
    let frame = Axis2::with_x_direction(point(at), direction(normal), direction(x))
        .unwrap_or_else(|failure| panic!("ellipse {at:?}: {failure:?}"));
    let curve = GeomEllipse::new(frame, major, minor)
        .unwrap_or_else(|failure| panic!("ellipse {major} {minor}: {failure:?}"));
    Handle::new(curve).cast()
}

/// `ellipse e x y z dx dy dz major minor`: the frame from the normal alone.
/// Example: `ellipse_along([0., 0., 300.], [1., 0., 0.], 150., 100.)`.
pub fn ellipse_along(
    at: [f64; 3],
    normal: [f64; 3],
    major: f64,
    minor: f64,
) -> Handle<dyn GeomCurve> {
    let frame = Axis2::new(point(at), direction(normal))
        .unwrap_or_else(|failure| panic!("ellipse {at:?}: {failure:?}"));
    let curve = GeomEllipse::new(frame, major, minor)
        .unwrap_or_else(|failure| panic!("ellipse {major} {minor}: {failure:?}"));
    Handle::new(curve).cast()
}

/// `offset o curve distance dx dy dz`; `offset_curve(&c, 10., [0., 0., 1.])`.
pub fn offset_curve(
    curve: &Handle<dyn GeomCurve>,
    distance: f64,
    along: [f64; 3],
) -> Handle<dyn GeomCurve> {
    let made = GeomOffsetCurve::new(curve, distance, direction(along), false)
        .unwrap_or_else(|failure| panic!("offset {distance}: {failure:?}"));
    Handle::new(made).cast()
}

/// `torus t r1 r2`; `torus(20., 5.)`.
pub fn torus(major: f64, minor: f64) -> Handle<dyn GeomSurface> {
    let made = GeomToroidalSurface::new(frame_at([0., 0., 0.]), major, minor)
        .unwrap_or_else(|failure| panic!("torus {major} {minor}: {failure:?}"));
    Handle::new(made).cast()
}

/// `trimu t surface u1 u2`; `trim_u(&s, 3.14, 9.42)`.
pub fn trim_u(surface: &Handle<dyn GeomSurface>, u1: f64, u2: f64) -> Handle<dyn GeomSurface> {
    let trimmed = GeomRectangularTrimmedSurface::with_one_direction(surface, u1, u2, true, true)
        .unwrap_or_else(|failure| panic!("trimu {u1} {u2}: {failure:?}"));
    Handle::new(trimmed).cast()
}

/// `trim t curve u1 u2`; `trim_curve(&c, 4.7, 6.28)`.
pub fn trim_curve(curve: &Handle<dyn GeomCurve>, first: f64, last: f64) -> Handle<dyn GeomCurve> {
    let trimmed = GeomTrimmedCurve::new(curve, first, last, true, true)
        .unwrap_or_else(|failure| panic!("trim {first} {last}: {failure:?}"));
    Handle::new(trimmed).cast()
}

/// `mkedge name curve first last`; `mkedge_range(s, "e1", &l, 0., 10.)`.
pub fn mkedge_range(
    s: &mut dyn Session,
    name: &str,
    curve: &Handle<dyn GeomCurve>,
    first: f64,
    last: f64,
) {
    let maker = BRepBuilderAPIMakeEdge::from_curve_params(curve, first, last)
        .unwrap_or_else(|failure| panic!("mkedge {name} {first} {last}: {failure:?}"));
    s.bind(name, &maker.edge().unwrap().into_shape());
}

/// `prism name base dx dy dz SemiInf`: unbounded one way, along the vector.
/// Example: `prism_semi_infinite(s, "pryz", "mb_1", [1., 0., 0.])`.
pub fn prism_semi_infinite(s: &mut dyn Session, name: &str, base: &str, along: [f64; 3]) {
    let base = bound_shape(s, base);
    let vector = Vector3::new(along[0], along[1], along[2]);
    let unit = Direction3::from_vector(vector)
        .unwrap_or_else(|failure| panic!("prism {name} {along:?}: {failure:?}"));
    let mut maker = BRepPrimAPIMakePrism::infinite(&base, unit, false, false, true)
        .unwrap_or_else(|failure| panic!("prism {name} SemiInf: {failure:?}"));
    s.bind(name, maker.shape().unwrap());
}

/// `mksurface name face`: the surface of the face with its location; `mksurface(s, "b1_6")`.
pub fn mksurface(s: &dyn Session, face: &str) -> Handle<dyn GeomSurface> {
    let shape = topods::face(&bound_shape(s, face))
        .unwrap_or_else(|failure| panic!("mksurface {face}: {failure:?}"));
    BRepTool::surface(&shape).unwrap_or_else(|failure| panic!("mksurface {face}: {failure:?}"))
}

/// `pcylinder name plane radius height`; `pcylinder_on(s, "c1", frame, 100., 400.)`.
pub fn pcylinder_on(s: &mut dyn Session, name: &str, frame: Axis3, radius: f64, height: f64) {
    let mut maker = BRepPrimAPIMakeCylinder::with_axes(frame.ax2().unwrap(), radius, height);
    s.bind(name, maker.shape().unwrap());
}

/// `ptorus name plane r1 r2`; `ptorus_on(s, "t", frame, 449.3, 50.6)`.
pub fn ptorus_on(s: &mut dyn Session, name: &str, frame: Axis3, r1: f64, r2: f64) {
    let mut maker = BRepPrimAPIMakeTorus::with_axes(frame.ax2().unwrap(), r1, r2)
        .unwrap_or_else(|failure| panic!("ptorus {name} {r1} {r2}: {failure:?}"));
    s.bind(name, maker.shape().unwrap());
}

/// `psphere name plane radius [angle]`; `psphere_on(s, "b2", frame, 8.66, None)`.
pub fn psphere_on(
    s: &mut dyn Session,
    name: &str,
    frame: Axis3,
    radius: f64,
    degrees: Option<f64>,
) {
    let axis = frame.ax2().unwrap();
    let mut maker = match degrees {
        None => BRepPrimAPIMakeSphere::with_axis(axis, radius),
        Some(angle) => BRepPrimAPIMakeSphere::with_axis_angle(
            axis,
            radius,
            angle * (std::f64::consts::PI / 180.),
        ),
    }
    .unwrap_or_else(|failure| panic!("psphere {name} {radius}: {failure:?}"));
    s.bind(name, maker.shape().unwrap());
}

/// `mkcurve name edge`: the located trimmed curve of the edge. Beyond the script: an earlier test
/// asserted it is a trimmed curve after the location.
/// Example: `mkcurve(s, "cut1_1")`.
pub fn mkcurve(s: &dyn Session, edge: &str) -> Handle<dyn GeomGeometry> {
    let shape = topods::edge(&bound_shape(s, edge))
        .unwrap_or_else(|failure| panic!("mkcurve {edge}: {failure:?}"));
    let found = BRepTool::curve_with_location(&shape).unwrap();
    let trimmed = GeomTrimmedCurve::new(&found.curve, found.first, found.last, true, true).unwrap();
    let located = trimmed
        .transformed(found.location.transformation())
        .unwrap();
    assert!(
        located.cast::<GeomTrimmedCurve>().is_some(),
        "mkcurve {edge}: expected a located trimmed curve"
    );
    located
}

/// `tcopy source copy`. Beyond the script: an earlier test asserted a new topology with its own
/// surfaces and curves and no triangulation. Example: `tcopy_checked(s, "cut1", "result")`.
pub fn tcopy_checked(s: &mut dyn Session, source: &str, copy: &str) {
    let original = bound_shape(s, source);
    let mut copier = BRepBuilderAPICopy::new();
    copier.perform(&original, true, false).unwrap();
    let shape = copier.shape().unwrap().clone();
    assert!(
        !shape.is_same(&original),
        "tcopy {source}: expected a new topology"
    );
    check_copied_geometry(&copier, &original);
    s.bind(copy, &shape);
}

fn check_copied_geometry(copier: &BRepBuilderAPICopy, original: &martensite_tkbrep::topods::Shape) {
    use crate::support::commands::distinct_parts;
    use martensite_tkg3d::top_abs::ShapeEnum;
    for source in distinct_parts(original, ShapeEnum::Face) {
        let mapped = copier.modified_shape(&source).unwrap();
        let (from, to) = (
            topods::face(&source).unwrap(),
            topods::face(&mapped).unwrap(),
        );
        let (first, second) = (
            BRepTool::surface_with_location(&from).unwrap().0,
            BRepTool::surface_with_location(&to).unwrap().0,
        );
        assert!(
            !std::ptr::addr_eq(first.get().unwrap(), second.get().unwrap()),
            "tcopy: expected its own surface"
        );
        assert!(
            BRepTool::triangulations(&to).unwrap().0.is_empty(),
            "tcopy: expected no triangulation"
        );
    }
    for source in distinct_parts(original, ShapeEnum::Edge) {
        let mapped = copier.modified_shape(&source).unwrap();
        let from = BRepTool::curve_with_location(&topods::edge(&source).unwrap())
            .unwrap()
            .curve;
        let to = BRepTool::curve_with_location(&topods::edge(&mapped).unwrap())
            .unwrap()
            .curve;
        assert_eq!(
            from.is_null(),
            to.is_null(),
            "tcopy: a curve copies to a curve"
        );
        if !from.is_null() {
            assert!(
                !std::ptr::addr_eq(from.get().unwrap(), to.get().unwrap()),
                "tcopy: expected its own curve"
            );
        }
    }
}
