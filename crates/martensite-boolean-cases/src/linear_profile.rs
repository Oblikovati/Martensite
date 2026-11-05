//! Exact planar profile moves, frames and closure; no script parser or geometric shortcuts.
#[path = "circular_profile.rs"]
mod circular_profile;
use martensite_tkbrep::topods::{Edge, Shape, Wire};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkgeomalgo::geom_api;
use martensite_tkmath::gp::{Axis2, Direction2, Direction3, Plane3, Point2, Point3};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape, BRepBuilderAPIMakeWire,
};
#[allow(dead_code)] // Each original cohort selects its own profile instructions.
#[derive(Clone, Copy)]
pub enum Command {
    Plane([f64; 3], [f64; 3]),
    Origin([f64; 3]),
    First(f64, f64),
    X(f64),
    Y(f64),
    Direction(f64, f64),
    Arc(f64, f64),
}
struct LinearProfile {
    plane: Plane3,
    first: Point2,
    current: Point2,
    wire: BRepBuilderAPIMakeWire,
    tangent: [f64; 2],
}
fn point(values: [f64; 3]) -> Point3 {
    Point3::new(values[0], values[1], values[2])
}
fn direction(values: [f64; 3]) -> Direction3 {
    Direction3::new(values[0], values[1], values[2]).unwrap()
}
fn line_edge(current: Point2, plane: &Plane3, dx: f64, dy: f64, length: f64) -> Edge {
    let curve: Handle<dyn Geom2dCurve> = Handle::new(Geom2dLine::from_point(
        current,
        Direction2::new(dx, dy).unwrap(),
    ))
    .into_cast();
    let spatial = geom_api::to_3d(&curve, plane).unwrap();
    BRepBuilderAPIMakeEdge::from_curve_params(&spatial, 0., length)
        .unwrap()
        .edge()
        .unwrap()
}
impl LinearProfile {
    fn apply(&mut self, command: Command) {
        match command {
            Command::Plane(n, x) => self.plane.set_position(
                Axis2::with_x_direction(self.plane.location(), direction(n), direction(x))
                    .unwrap()
                    .axis3(),
            ),
            Command::Origin(o) => self.plane.set_location(point(o)),
            Command::First(x, y) => {
                self.first = Point2::new(x, y);
                self.current = self.first;
            }
            Command::X(length) => self.line(length, 1., 0.),
            Command::Y(length) => self.line(length, 0., 1.),
            Command::Direction(dx, dy) => self.set_direction(dx, dy),
            Command::Arc(radius, angle) => self.arc(radius, angle),
        }
    }
    fn set_direction(&mut self, dx: f64, dy: f64) {
        let length = (dx * dx + dy * dy).sqrt();
        if length > precision::confusion() {
            self.tangent = [dx / length, dy / length];
        }
    }
    fn arc(&mut self, radius: f64, degrees: f64) {
        if radius.abs() <= precision::confusion() {
            return;
        }
        let (curve, angle) = circular_profile::tangent_arc(
            [self.current.x(), self.current.y()],
            self.tangent,
            radius,
            degrees,
        );
        let spatial = geom_api::to_3d(&curve, &self.plane).unwrap();
        let edge = BRepBuilderAPIMakeEdge::from_curve_params(&spatial, 0., angle)
            .unwrap()
            .edge()
            .unwrap();
        self.wire.add_edge(&edge).unwrap();
        let (point, derivative) = curve.get().unwrap().d1(angle).unwrap();
        self.current = point;
        self.tangent = [derivative.x() / radius.abs(), derivative.y() / radius.abs()];
    }
    fn line(&mut self, mut length: f64, mut dx: f64, mut dy: f64) {
        if length < 0. {
            length = -length;
            dx = -dx;
            dy = -dy;
        }
        let edge = line_edge(self.current, &self.plane, dx, dy, length);
        self.wire.add_edge(&edge).unwrap();
        self.tangent = [dx, dy];
        self.current = Point2::new(
            self.current.x() + length * dx,
            self.current.y() + length * dy,
        );
    }
    fn close(&mut self) {
        let dx = self.first.x() - self.current.x();
        let dy = self.first.y() - self.current.y();
        let length = (dx * dx + dy * dy).sqrt();
        if length > precision::confusion() {
            self.line(length, dx / length, dy / length);
        }
    }
}
/// Preserve the complete linear profile sequence and explicit plane; `face(&[Command::X(2.),Command::Y(1.),Command::X(-2.)])`.
fn profile(commands: &[Command]) -> LinearProfile {
    let mut state = LinearProfile {
        plane: Plane3::default(),
        first: Point2::default(),
        current: Point2::default(),
        wire: BRepBuilderAPIMakeWire::new(),
        tangent: [1., 0.],
    };
    for command in commands {
        state.apply(*command);
    }
    state.close();
    state
}
/// Preserve W as a closed wire before a separate mkplane; `closed_wire(&[Command::X(2.),Command::Y(1.)])`.
#[allow(dead_code)] // Face-only cohorts retain the original face completion path.
pub fn closed_wire(commands: &[Command]) -> Wire {
    profile(commands).wire.wire().unwrap()
}
/// Preserve explicit-plane face completion; `face(&[Command::X(2.),Command::Y(1.),Command::X(-2.)])`.
#[allow(dead_code)] // Wire-only cohorts use a later, separate mkplane command.
pub fn face(commands: &[Command]) -> Shape {
    let state = profile(commands);
    BRepBuilderAPIMakeFace::from_plane_wire(state.plane, &state.wire.wire().unwrap(), true)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}
