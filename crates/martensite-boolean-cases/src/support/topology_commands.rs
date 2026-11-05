//! The commands that build shapes from the topology up: vertices, edges, wires and faces, the
//! plane and circle variables they use, half-spaces and revolutions.
use super::{commands::bound_shape, session::Session};
use martensite_tkbrep::topods::{self, Edge, Vertex, Wire};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg3d::{
    geom::{GeomCircle, GeomCurve, GeomPlane, GeomSurface},
    top_abs::ShapeEnum,
};
use martensite_tkmath::gp::{Axis1, Axis2, Axis3, Direction3, Point3};
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeHalfSpace, BRepPrimAPIMakeRevol, BRepPrimAPIMakeSphere,
};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape,
    BRepBuilderAPIMakeVertex, BRepBuilderAPIMakeWire,
};

fn point(at: [f64; 3]) -> Point3 {
    Point3::new(at[0], at[1], at[2])
}

fn z_direction() -> Direction3 {
    Direction3::new(0., 0., 1.).unwrap()
}

fn x_direction() -> Direction3 {
    Direction3::new(1., 0., 0.).unwrap()
}

/// A `plane` variable: a plane surface and its frame.
pub struct PlaneVariable {
    frame: Axis3,
    /// The plane as a surface; `&pl1.surface`.
    pub surface: Handle<dyn GeomSurface>,
}

impl PlaneVariable {
    fn on(frame: Axis3) -> Self {
        let surface: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(frame)).cast();
        Self { frame, surface }
    }

    /// The frame primitives are built on; `pl1.axes()`.
    pub fn axes(&self) -> Axis2 {
        self.frame.ax2().unwrap()
    }
}

fn direction_of(along: [f64; 3]) -> Direction3 {
    Direction3::new(along[0], along[1], along[2])
        .unwrap_or_else(|failure| panic!("direction {along:?}: {failure:?}"))
}

/// `plane name x y z dx dy dz`: its X axis chosen from the normal.
/// Example: `let p = plane_along([0.; 3], [0., 0., 1.]);`.
pub fn plane_along(at: [f64; 3], normal: [f64; 3]) -> PlaneVariable {
    let frame = Axis3::from_normal(point(at), direction_of(normal))
        .unwrap_or_else(|failure| panic!("plane {at:?} {normal:?}: {failure:?}"));
    PlaneVariable::on(frame)
}

/// `plane name x y z dx dy dz ux uy uz`.
/// Example: `let p = plane_axes([0.; 3], [0., 0., 1.], [1., 0., 0.]);`.
pub fn plane_axes(at: [f64; 3], normal: [f64; 3], x_axis: [f64; 3]) -> PlaneVariable {
    let frame = Axis3::new(point(at), direction_of(normal), direction_of(x_axis))
        .unwrap_or_else(|failure| panic!("plane {at:?} {normal:?} {x_axis:?}: {failure:?}"));
    PlaneVariable::on(frame)
}

/// `plane name x y z`: a plane through the point, normal to Z, its X axis along X.
/// Example: `let pl1 = plane([0., 0., 0.]);`.
pub fn plane(origin: [f64; 3]) -> PlaneVariable {
    PlaneVariable::on(Axis3::new(point(origin), z_direction(), x_direction()).unwrap())
}

/// `circle name x y z radius`: a circle about Z; `let c1 = circle([0., 0., 0.], 20.);`.
pub fn circle(center: [f64; 3], radius: f64) -> Handle<dyn GeomCurve> {
    let frame = Axis2::new(point(center), z_direction()).unwrap();
    let curve = GeomCircle::new(frame, radius)
        .unwrap_or_else(|failure| panic!("circle {center:?} {radius}: {failure:?}"));
    Handle::new(curve).cast()
}

/// `psphere name plane radius`: a sphere on the frame of the plane.
/// Example: `psphere_on(s, "s1", &pl1, 7.5)`.
pub fn psphere_on(session: &mut dyn Session, name: &str, plane: &PlaneVariable, radius: f64) {
    let mut maker = BRepPrimAPIMakeSphere::with_axis(plane.frame.ax2().unwrap(), radius)
        .unwrap_or_else(|failure| panic!("psphere {name} {radius}: {failure:?}"));
    session.bind(name, maker.shape().unwrap());
}

/// `vertex name x y z`; `vertex(s, "v1", [250., 250., 0.])`.
pub fn vertex(session: &mut dyn Session, name: &str, at: [f64; 3]) {
    let maker = BRepBuilderAPIMakeVertex::new(point(at))
        .unwrap_or_else(|failure| panic!("vertex {name} {at:?}: {failure:?}"));
    session.bind(name, &maker.vertex().unwrap().into_shape());
}

fn bound_vertex(session: &dyn Session, name: &str) -> Vertex {
    topods::vertex(&bound_shape(session, name))
        .unwrap_or_else(|failure| panic!("{name}: expected a vertex: {failure:?}"))
}

fn bound_edge(session: &dyn Session, name: &str) -> Edge {
    topods::edge(&bound_shape(session, name))
        .unwrap_or_else(|failure| panic!("{name}: expected an edge: {failure:?}"))
}

fn bound_wire(session: &dyn Session, name: &str) -> Wire {
    topods::wire(&bound_shape(session, name))
        .unwrap_or_else(|failure| panic!("{name}: expected a wire: {failure:?}"))
}

/// `edge name v1 v2`; `edge(s, "e1", "v1", "v2")`.
pub fn edge(session: &mut dyn Session, name: &str, first: &str, last: &str) {
    let (first, last) = (bound_vertex(session, first), bound_vertex(session, last));
    let maker = BRepBuilderAPIMakeEdge::from_vertices(&first, &last)
        .unwrap_or_else(|failure| panic!("edge {name}: {failure:?}"));
    session.bind(name, &maker.edge().unwrap().into_shape());
}

/// `mkedge name curve`; `mkedge(s, "e1", &c1)`.
pub fn mkedge(session: &mut dyn Session, name: &str, curve: &Handle<dyn GeomCurve>) {
    let maker = BRepBuilderAPIMakeEdge::from_curve(curve)
        .unwrap_or_else(|failure| panic!("mkedge {name}: {failure:?}"));
    session.bind(name, &maker.edge().unwrap().into_shape());
}

/// `wire name e1 e2 ...`, of edges; `wire(s, "w1", &["e1", "e2"])`.
pub fn wire(session: &mut dyn Session, name: &str, edges: &[&str]) {
    let mut maker = BRepBuilderAPIMakeWire::new();
    for part in edges {
        let part = bound_edge(session, part);
        maker
            .add_edge(&part)
            .unwrap_or_else(|failure| panic!("wire {name}: {failure:?}"));
    }
    session.bind(name, &maker.wire().unwrap().into_shape());
}

/// `mkplane name wire [1]`; `only_plane` is the script's `1`. Example: `mkplane(s, "f", "w1", false)`.
pub fn mkplane(session: &mut dyn Session, name: &str, boundary: &str, only_plane: bool) {
    let boundary = bound_wire(session, boundary);
    let maker = BRepBuilderAPIMakeFace::from_wire(&boundary, only_plane)
        .unwrap_or_else(|failure| panic!("mkplane {name}: {failure:?}"));
    session.bind(name, &maker.face().unwrap().into_shape());
}

/// `mkface name plane u1 u2 v1 v2`; `mkface_bounded(s, "f", &pl1, (-1., 1., -1., 1.))`.
pub fn mkface_bounded(
    session: &mut dyn Session,
    name: &str,
    plane: &PlaneVariable,
    bounds: (f64, f64, f64, f64),
) {
    let maker =
        BRepBuilderAPIMakeFace::from_surface_bounds(&plane.surface, bounds, precision::confusion())
            .unwrap_or_else(|failure| panic!("mkface {name} {bounds:?}: {failure:?}"));
    session.bind(name, &maker.face().unwrap().into_shape());
}

/// `mkface name plane wire`, its material inside the wire; `mkface_of_wire(s, "d1", &pl1, "w1")`.
pub fn mkface_of_wire(
    session: &mut dyn Session,
    name: &str,
    plane: &PlaneVariable,
    boundary: &str,
) {
    let boundary = bound_wire(session, boundary);
    let maker = BRepBuilderAPIMakeFace::from_surface_wire(&plane.surface, &boundary, true)
        .unwrap_or_else(|failure| panic!("mkface {name}: {failure:?}"));
    session.bind(name, &maker.face().unwrap().into_shape());
}

/// `halfspace name face-or-shell x y z`; the point marks the side of the material.
/// Example: `halfspace(s, "hs", "f", [0., 0., -100.])`.
pub fn halfspace(session: &mut dyn Session, name: &str, boundary: &str, material_at: [f64; 3]) {
    let boundary = bound_shape(session, boundary);
    let at = point(material_at);
    // The command reads its boundary as a face, and failing that as a shell.
    let maker = match boundary.shape_type().unwrap() {
        ShapeEnum::Face => {
            BRepPrimAPIMakeHalfSpace::with_face(&topods::face(&boundary).unwrap(), at)
        }
        ShapeEnum::Shell => {
            BRepPrimAPIMakeHalfSpace::with_shell(&topods::shell(&boundary).unwrap(), at)
        }
        other => panic!("halfspace {name}: {other:?} is neither a face nor a shell"),
    }
    .unwrap_or_else(|failure| panic!("halfspace {name}: {failure:?}"));
    session.bind(name, &maker.solid().unwrap().clone().into_shape());
}

/// `revol name base x y z dx dy dz degrees`.
/// Example: `revol(s, "bb", "wb", [0., 0., 0.], [0., 0., 1.], 360.)`.
pub fn revol(
    session: &mut dyn Session,
    name: &str,
    base: &str,
    at: [f64; 3],
    axis: [f64; 3],
    degrees: f64,
) {
    let base = bound_shape(session, base);
    let direction = Direction3::new(axis[0], axis[1], axis[2])
        .unwrap_or_else(|failure| panic!("revol {name} axis {axis:?}: {failure:?}"));
    let line = Axis1::new(point(at), direction);
    let angle = degrees * (std::f64::consts::PI / 180.0);
    let mut maker = BRepPrimAPIMakeRevol::new(&base, line, angle, false)
        .unwrap_or_else(|failure| panic!("revol {name} {degrees}: {failure:?}"));
    session.bind(name, maker.shape().unwrap());
}
