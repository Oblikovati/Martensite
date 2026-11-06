//! The commands of the scripts that work on curves and surfaces: the script's variable is a
//! plain value in a case body, and the shape made from it is bound as the script binds it.
use super::session::Session;
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg3d::geom::{
    GeomCircle, GeomConicalSurface, GeomCurve, GeomLine, GeomPlane, GeomRectangularTrimmedSurface,
    GeomSurface, GeomTrimmedCurve,
};
use martensite_tkmath::gp::{Axis2, Axis3, Direction3, Point3, Vector3, dz};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakePrism;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon,
    BRepBuilderAPIMakeShape,
};

fn point(at: [f64; 3]) -> Point3 {
    Point3::new(at[0], at[1], at[2])
}

fn direction(along: [f64; 3]) -> Direction3 {
    Direction3::new(along[0], along[1], along[2])
        .unwrap_or_else(|failure| panic!("direction {along:?}: {failure:?}"))
}

/// `cone c x y z dx dy dz degrees radius`; `cone([0.; 3], [0., -1., 0.], 45., 0.)`.
pub fn cone(at: [f64; 3], normal: [f64; 3], degrees: f64, radius: f64) -> Handle<dyn GeomSurface> {
    let frame = Axis3::from_normal(point(at), direction(normal))
        .unwrap_or_else(|failure| panic!("cone {at:?} {normal:?}: {failure:?}"));
    let angle = degrees * (std::f64::consts::PI / 180.0);
    let surface = GeomConicalSurface::new(frame, angle, radius)
        .unwrap_or_else(|failure| panic!("cone {degrees} {radius}: {failure:?}"));
    Handle::new(surface).cast()
}

/// `plane p x y z dx dy dz`; `plane([0.; 3], [0., 1., 0.])`.
pub fn plane(at: [f64; 3], normal: [f64; 3]) -> Handle<dyn GeomSurface> {
    let frame = Axis3::from_normal(point(at), direction(normal))
        .unwrap_or_else(|failure| panic!("plane {at:?} {normal:?}: {failure:?}"));
    Handle::new(GeomPlane::new(frame)).cast()
}

/// `trimv t surface v1 v2`; `trimv(&c1, 0., 42.4264068711929)`.
pub fn trimv(surface: &Handle<dyn GeomSurface>, v1: f64, v2: f64) -> Handle<dyn GeomSurface> {
    let trimmed = GeomRectangularTrimmedSurface::with_one_direction(surface, v1, v2, false, true)
        .unwrap_or_else(|failure| panic!("trimv {v1} {v2}: {failure:?}"));
    Handle::new(trimmed).cast()
}

/// `mkface name surface`; `mkface(s, "f1", &tc1)`.
pub fn mkface(s: &mut dyn Session, name: &str, surface: &Handle<dyn GeomSurface>) {
    let mut maker = BRepBuilderAPIMakeFace::from_surface(surface, precision::confusion())
        .unwrap_or_else(|failure| panic!("mkface {name}: {failure:?}"));
    s.bind(name, maker.shape().unwrap());
}

/// `mkface name surface u1 u2 v1 v2`; `mkface_bounded(s, "f", &c, (0., 6.28, 0., 1.))`.
pub fn mkface_bounded(
    s: &mut dyn Session,
    name: &str,
    surface: &Handle<dyn GeomSurface>,
    bounds: (f64, f64, f64, f64),
) {
    let mut maker =
        BRepBuilderAPIMakeFace::from_surface_bounds(surface, bounds, precision::confusion())
            .unwrap_or_else(|failure| panic!("mkface {name} {bounds:?}: {failure:?}"));
    s.bind(name, maker.shape().unwrap());
}

/// `circle c x y z r`; `circle([0.; 3], 2.)`.
pub fn circle(at: [f64; 3], radius: f64) -> Handle<dyn GeomCurve> {
    let frame =
        Axis2::new(point(at), dz()).unwrap_or_else(|failure| panic!("circle {at:?}: {failure:?}"));
    circle_on(frame, radius)
}

/// `circle c x y z dx dy dz r`; `circle_along([0.; 3], [0., 0., 1.], 2.)`.
pub fn circle_along(at: [f64; 3], normal: [f64; 3], radius: f64) -> Handle<dyn GeomCurve> {
    let frame = Axis2::new(point(at), direction(normal))
        .unwrap_or_else(|failure| panic!("circle {at:?} {normal:?}: {failure:?}"));
    circle_on(frame, radius)
}

fn circle_on(frame: Axis2, radius: f64) -> Handle<dyn GeomCurve> {
    let curve = GeomCircle::new(frame, radius)
        .unwrap_or_else(|failure| panic!("circle radius {radius}: {failure:?}"));
    Handle::new(curve).cast()
}

/// `mkedge name curve`; `mkedge(s, "c", &circle)`.
pub fn mkedge(s: &mut dyn Session, name: &str, curve: &Handle<dyn GeomCurve>) {
    let maker = BRepBuilderAPIMakeEdge::from_curve(curve)
        .unwrap_or_else(|failure| panic!("mkedge {name}: {failure:?}"));
    s.bind(name, &maker.edge().unwrap().into_shape());
}

/// `polyline name x1 y1 z1 x2 y2 z2 ...`; `polyline(s, "w", &[[0.; 3], [1., 0., 0.]])`.
pub fn polyline(s: &mut dyn Session, name: &str, points: &[[f64; 3]]) {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    for at in points {
        polygon
            .add_point(point(*at))
            .unwrap_or_else(|failure| panic!("polyline {name} {at:?}: {failure:?}"));
    }
    let boundary = polygon
        .wire()
        .unwrap_or_else(|failure| panic!("polyline {name}: {failure:?}"));
    s.bind(name, boundary.as_ref());
}

/// `prism name base dx dy dz inf`: unbounded both ways, along the direction of the vector.
/// Example: `prism_infinite(s, "cyl", "c", [0., 0., 60.])`.
pub fn prism_infinite(s: &mut dyn Session, name: &str, base: &str, along: [f64; 3]) {
    let base = s
        .shape(base)
        .unwrap_or_else(|| panic!("prism {name}: expected the bound base {base}"))
        .clone();
    let vector = Vector3::new(along[0], along[1], along[2]);
    let unit = Direction3::from_vector(vector)
        .unwrap_or_else(|failure| panic!("prism {name} {along:?}: {failure:?}"));
    let mut maker = BRepPrimAPIMakePrism::infinite(&base, unit, true, false, true)
        .unwrap_or_else(|failure| panic!("prism {name} inf: {failure:?}"));
    s.bind(name, maker.shape().unwrap());
}

/// `line l x y z dx dy dz`; `line([0.; 3], [0., 1., 0.])`.
pub fn line(at: [f64; 3], along: [f64; 3]) -> Handle<dyn GeomCurve> {
    Handle::new(GeomLine::from_point(point(at), direction(along))).cast()
}

/// `uiso c surface u`; `uiso(&cylinder, std::f64::consts::PI)`.
pub fn uiso(surface: &Handle<dyn GeomSurface>, parameter: f64) -> Handle<dyn GeomCurve> {
    surface
        .get()
        .unwrap()
        .u_iso(parameter)
        .unwrap_or_else(|failure| panic!("uiso {parameter}: {failure:?}"))
}

/// `viso c surface v`; `viso(&cylinder, 0.)`.
pub fn viso(surface: &Handle<dyn GeomSurface>, parameter: f64) -> Handle<dyn GeomCurve> {
    surface
        .get()
        .unwrap()
        .v_iso(parameter)
        .unwrap_or_else(|failure| panic!("viso {parameter}: {failure:?}"))
}

/// `mkedge name curve first last`; `mkedge_bounded(s, "e", &line, -15., 15.)`.
pub fn mkedge_bounded(
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

/// `circle c x y z dx dy dz ux uy uz r`; `circle_oriented([0.; 3], [0., 0., 1.], [1., 0., 0.], 2.)`.
pub fn circle_oriented(
    at: [f64; 3],
    normal: [f64; 3],
    x_axis: [f64; 3],
    radius: f64,
) -> Handle<dyn GeomCurve> {
    let frame = Axis2::with_x_direction(point(at), direction(normal), direction(x_axis))
        .unwrap_or_else(|failure| panic!("circle {at:?} {normal:?} {x_axis:?}: {failure:?}"));
    circle_on(frame, radius)
}

/// `mksurface name face`: the surface under `face`, where its location puts it.
/// Example: `let c_in = mksurface(s, "part_1");`.
pub fn mksurface(s: &dyn Session, face: &str) -> Handle<dyn GeomSurface> {
    let shape = s
        .shape(face)
        .unwrap_or_else(|| panic!("mksurface {face}: expected a bound face"));
    let (stored, location) =
        BRepTool::surface_with_location(&topods::face(shape).unwrap()).unwrap();
    let placed = stored.get().unwrap().transformed(location.transformation());
    placed.unwrap().cast::<dyn GeomSurface>()
}

/// `trim name name`: with no bounds, a trimmed surface goes back to its basis.
/// Example: `let c_in = untrimmed(&c_in);`.
pub fn untrimmed(surface: &Handle<dyn GeomSurface>) -> Handle<dyn GeomSurface> {
    let trimmed = surface.cast::<GeomRectangularTrimmedSurface>();
    trimmed
        .get()
        .map_or_else(|| surface.clone(), |only| only.basis_surface())
}

/// The curve `mkcurve` takes from an edge, and the bounds of its range.
pub struct EdgeCurve {
    curve: Handle<dyn GeomCurve>,
    /// `bounds curve first last`: the start of the range.
    pub first: f64,
    /// `bounds curve first last`: the end of the range.
    pub last: f64,
}

/// `mkcurve curve edge`; `let curve = mkcurve(s, "e");`.
pub fn mkcurve(s: &dyn Session, edge: &str) -> EdgeCurve {
    let shape = s
        .shape(edge)
        .unwrap_or_else(|| panic!("mkcurve {edge}: expected a bound edge"));
    let stored = BRepTool::curve_with_location(&topods::edge(shape).unwrap()).unwrap();
    let (first, last) = (stored.first, stored.last);
    let trimmed = GeomTrimmedCurve::new(&stored.curve, first, last, true, true).unwrap();
    let trimmed: Handle<dyn GeomCurve> = Handle::new(trimmed).cast();
    let curve = trimmed
        .get()
        .unwrap()
        .transformed(stored.location.transformation())
        .unwrap()
        .cast::<dyn GeomCurve>();
    EdgeCurve { curve, first, last }
}

impl EdgeCurve {
    /// `cvalue curve at x y z`; `let [x, y, z] = curve.value(0.5);`.
    pub fn value(&self, at: f64) -> [f64; 3] {
        let point = self.curve.get().unwrap().value(at).unwrap();
        [point.x(), point.y(), point.z()]
    }
}
