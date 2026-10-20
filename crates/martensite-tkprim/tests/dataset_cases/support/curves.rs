//! The geometry and topology construction commands the primitive cases call besides those
//! of the topological algorithm harness (`circle`/`ellipse`/`plane`/`sphere`/`trim`,
//! `edge`/`mkedge`/`polyline`/`mkface`, `add`/`explode`), on the Rust API.
use super::restore::Restored;
use super::session::Session;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{self, Shape},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{
    geom::{
        GeomCircle, GeomCurve, GeomEllipse, GeomPlane, GeomRectangularTrimmedSurface,
        GeomSphericalSurface, GeomSurface, GeomTrimmedCurve,
    },
    top_abs::ShapeEnum,
};
use martensite_tkmath::gp::{Axis2, Axis3, Direction3, Point3};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIFaceError, BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace,
    BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape,
};

/// `BRepTest_SurfaceCommands` `mkplane name wire [1]`: `BRepBuilderAPI_MakeFace(W,
/// OnlyPlane)` (`OnlyPlane` for a fourth word `1`); `Err` with the command's line.
pub fn mkplane(wire: &Shape, only_plane: bool) -> Result<Shape, String> {
    let wire = topods::wire(wire).map_err(|_| "mkplane: not a wire".to_string())?;
    let maker = BRepBuilderAPIMakeFace::from_wire(&wire, only_plane).unwrap();
    match maker.error() {
        BRepBuilderAPIFaceError::FaceDone => Ok(maker.face().unwrap().into_shape()),
        error => {
            let text = format!("Error. mkplane has been finished with {error:?} status.\n");
            super::log::result(&text);
            Err(text)
        }
    }
}

/// The `imagedir` of the case: a fresh directory for the files the script writes.
pub fn imagedir() -> std::path::PathBuf {
    let directory = std::env::temp_dir()
        .join("martensite-tkprim-draw-images")
        .join(std::process::id().to_string());
    std::fs::create_dir_all(&directory).unwrap();
    directory
}

/// `vertex name x y z`, the variable set in `session`.
pub fn vertex_set(session: &mut dyn Session, name: &str, at: [f64; 3]) -> Shape {
    let vertex = super::topology::vertex(point(at));
    session.set(name, &Restored::Shape(vertex.clone()));
    vertex
}

/// `BRepTest_BasicCommands` `transform` with `-copy` (`trotate ... -copy`, ...): a
/// non-identity transformation by `BRepBuilderAPI_Transform::Perform(S, Copy = true)`.
pub fn transform_copy(shape: &Shape, trsf: martensite_tkmath::gp::Transform3) -> Shape {
    let mut maker = martensite_tktopalgo::brep_builder_api::BRepBuilderAPITransform::with_shape(
        shape, trsf, true, false,
    )
    .unwrap();
    maker.shape().unwrap().clone()
}

/// `name` set to `shape` in `session` (a command's result variable).
pub fn shape_set(session: &mut dyn Session, name: &str, shape: Shape) -> Shape {
    session.set(name, &Restored::Shape(shape.clone()));
    shape
}

fn point([x, y, z]: [f64; 3]) -> Point3 {
    Point3::new(x, y, z)
}

fn direction([x, y, z]: [f64; 3]) -> Direction3 {
    Direction3::new(x, y, z).expect("a gp_Dir")
}

/// `circle name x y z r` (6 words): `Geom_Circle(gp_Ax2(P, Z), r)`.
pub fn circle(center: [f64; 3], radius: f64) -> Handle<dyn GeomCurve> {
    circle_normal(center, [0., 0., 1.], radius)
}

/// `circle name x y z nx ny nz r` (9 words): `Geom_Circle(gp_Ax2(P, N), r)`.
pub fn circle_normal(center: [f64; 3], normal: [f64; 3], radius: f64) -> Handle<dyn GeomCurve> {
    let frame = Axis2::new(point(center), direction(normal)).unwrap();
    Handle::new(GeomCircle::new(frame, radius).unwrap()).cast()
}

/// `ellipse name x y z major minor` (7 words): `Geom_Ellipse(gp_Ax2(P, Z), major, minor)`.
pub fn ellipse(center: [f64; 3], major: f64, minor: f64) -> Handle<dyn GeomCurve> {
    let frame = Axis2::new(point(center), Direction3::Z).unwrap();
    Handle::new(GeomEllipse::new(frame, major, minor).unwrap()).cast()
}

/// `plane name` (2 words): `Geom_Plane(gp_Ax3(origin, Z, X))`.
pub fn plane_default() -> Handle<dyn GeomSurface> {
    Handle::new(GeomPlane::new(Axis3::default())).cast()
}

/// `plane name x y z nx ny nz` (8 words): `Geom_Plane(gp_Ax3(P, N))`.
pub fn plane_normal(origin: [f64; 3], normal: [f64; 3]) -> Handle<dyn GeomSurface> {
    let frame = Axis3::from_normal(point(origin), direction(normal)).unwrap();
    Handle::new(GeomPlane::new(frame)).cast()
}

/// `sphere name r` (3 words): `Geom_SphericalSurface(gp_Ax3(origin, Z, X), r)`.
pub fn sphere(radius: f64) -> Handle<dyn GeomSurface> {
    Handle::new(GeomSphericalSurface::new(Axis3::default(), radius).unwrap()).cast()
}

/// `trim name curve u1 u2`: `Geom_TrimmedCurve(C, u1, u2, USense = true)`.
pub fn trim_curve(curve: &Handle<dyn GeomCurve>, u1: f64, u2: f64) -> Handle<dyn GeomCurve> {
    Handle::new(GeomTrimmedCurve::new(curve, u1, u2, true, true).unwrap()).cast()
}

/// `trim name surface u1 u2 v1 v2`: `Geom_RectangularTrimmedSurface(S, u1, u2, v1, v2)`.
pub fn trim_surface(
    surface: &Handle<dyn GeomSurface>,
    u: [f64; 2],
    v: [f64; 2],
) -> Handle<dyn GeomSurface> {
    let trimmed =
        GeomRectangularTrimmedSurface::new(surface, (u[0], u[1]), (v[0], v[1]), (true, true));
    Handle::new(trimmed.unwrap()).cast()
}

/// `BRepTest_CurveCommands` `edge name v1 v2`: `BRepBuilderAPI_MakeEdge(V1, V2)`.
pub fn edge(v1: &Shape, v2: &Shape) -> Shape {
    let v1 = topods::vertex(v1).expect("edge: a vertex");
    let v2 = topods::vertex(v2).expect("edge: a vertex");
    BRepBuilderAPIMakeEdge::from_vertices(&v1, &v2)
        .unwrap()
        .edge()
        .unwrap()
        .into_shape()
}

/// `BRepTest_CurveCommands` `mkedge name curve first last`:
/// `BRepBuilderAPI_MakeEdge(C, first, last)`.
pub fn mkedge_range(curve: &Handle<dyn GeomCurve>, first: f64, last: f64) -> Shape {
    BRepBuilderAPIMakeEdge::from_curve_params(curve, first, last)
        .unwrap()
        .edge()
        .unwrap()
        .into_shape()
}

/// `BRepTest_CurveCommands` `polyline name x1 y1 z1 x2 y2 z2 ...`:
/// `BRepBuilderAPI_MakePolygon` of the points, its wire.
pub fn polyline(points: &[[f64; 3]]) -> Shape {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    for p in points {
        polygon.add_point(point(*p)).unwrap();
    }
    polygon.wire().unwrap().into_shape()
}

/// `BRepTest_SurfaceCommands` `mkface name surface wire` (4 words):
/// `BRepBuilderAPI_MakeFace(S, W, Inside = true)`.
pub fn mkface_wire(surface: &Handle<dyn GeomSurface>, wire: &Shape) -> Shape {
    let wire = topods::wire(wire).expect("mkface: a wire");
    let mut maker = BRepBuilderAPIMakeFace::from_surface_wire(surface, &wire, true).unwrap();
    maker.shape().unwrap().clone()
}

/// `add what where`: `BRep_Builder::Add(where, what)`, the variable reset.
pub fn add(what: &Shape, mut into: Shape) -> Shape {
    BRepBuilder::default()
        .add(&mut into, what)
        .unwrap_or_else(|failure| panic!("add: {failure:?}"));
    into
}

/// `explode name [type]`: each part set as `name_1`, `name_2`, ... and the
/// list of their names (the command's script result, logged).
pub fn explode(
    session: &mut dyn Session,
    name: &str,
    shape: &Shape,
    kind: Option<ShapeEnum>,
) -> Vec<Shape> {
    let parts = match kind {
        None => super::commands::explode(shape),
        Some(kind) => super::commands::explode_type(shape, kind),
    };
    let mut names = Vec::new();
    for (index, part) in parts.iter().enumerate() {
        let part_name = format!("{name}_{}", index + 1);
        session.set(&part_name, &Restored::Shape(part.clone()));
        names.push(part_name);
    }
    super::log::result(&names.join(" "));
    parts
}
