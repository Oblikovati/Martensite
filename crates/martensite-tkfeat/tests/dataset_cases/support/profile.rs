//! `profile name [code values] ...`: a wire or face drawn by a moving point and direction, on the
//! plane of a face or the XY plane.
use super::{
    feat_script::{FeatScript, checked, command_failed},
    session::Session,
};
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, Edge, Shape},
};
use martensite_tkernel::{
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dCurve, Geom2dLine};
use martensite_tkg3d::geom::{GeomPlane, GeomSurface};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeomalgo::geom_api;
use martensite_tkmath::{
    geom_abs::Shape as Continuity,
    gp::{Axis2, Axis2d, Axis3, Direction2, Direction3, Plane3, Point2, Point3, Vector3, xoy},
    toploc::Location,
};
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeWire},
    brep_lib,
};
use std::f64::consts::PI;

/// One instruction of a profile, by its code in the scripts.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Code<'a> {
    /// `S face`: the plane or surface of a face.
    S(&'a str),
    /// `O x y z`: the plane's origin.
    O(f64, f64, f64),
    /// `P nx ny nz ux uy uz`: the plane's normal and X direction.
    P([f64; 3], [f64; 3]),
    /// `F x y`: the first point.
    F(f64, f64),
    /// `X dx`.
    X(f64),
    /// `Y dy`.
    Y(f64),
    /// `TT x y`: set the point.
    TT(f64, f64),
    /// `C radius degrees`: an arc tangent to the direction.
    C(f64, f64),
    /// `W`: a closed wire, no face.
    W,
    /// `WW`: an open wire.
    WW,
}

/// The moving point and direction, the first point and the plane.
struct Pen {
    first_point: [f64; 2],
    at: [f64; 2],
    along: [f64; 2],
    is_first: bool,
    plane: Plane3,
    planar: bool,
    surface: Handle<dyn GeomSurface>,
    location: Location,
}

/// A 2D curve of the profile and its parameter length.
type Segment = (Handle<dyn Geom2dCurve>, f64);

/// One move of the pen; the reference keeps it in `move`, `length`, `radius` and `angle`.
#[derive(Clone, Copy)]
enum Motion {
    None,
    Line(f64),
    Circle(f64, f64),
}

impl Pen {
    fn new() -> Self {
        Self {
            first_point: [0.0; 2],
            at: [0.0; 2],
            along: [1.0, 0.0],
            is_first: true,
            plane: Plane3::from_position(Axis3::from_axis2(xoy())),
            planar: true,
            surface: Handle::default(),
            location: Location::default(),
        }
    }

    fn set_face(&mut self, face: &Shape) {
        let face = checked(topods::face(face));
        let (surface, location) = checked(BRepTool::surface_with_location(&face));
        self.surface = surface;
        self.location = location;
        match self.surface.cast::<GeomPlane>().get() {
            Some(plane) => self.plane = plane.pln(),
            None => self.planar = false,
        }
    }

    fn set_frame(&mut self, normal: [f64; 3], x_axis: [f64; 3]) {
        let n = Vector3::new(normal[0], normal[1], normal[2]);
        let x = Vector3::new(x_axis[0], x_axis[1], x_axis[2]);
        if n.magnitude() <= precision::confusion() || x.magnitude() <= precision::confusion() {
            command_failed("profile : null direction");
        }
        let (n, x) = (
            checked(Direction3::from_vector(n)),
            checked(Direction3::from_vector(x)),
        );
        let frame = checked(Axis2::with_x_direction(self.plane.location(), n, x));
        self.plane.set_position(Axis3::from_axis2(frame));
    }

    /// The 2D curve of a move from the current point (normalizing negative lengths and radii as
    /// the reference does), leaving the pen after it.
    fn curve(&mut self, motion: Motion) -> Result<Option<Segment>, Failure> {
        match motion {
            Motion::None => Ok(None),
            Motion::Line(length) => self.line(length).map(Some),
            Motion::Circle(radius, angle) => self.arc(radius, angle).map(Some),
        }
    }

    fn line(&mut self, mut length: f64) -> Result<Segment, Failure> {
        if length < 0.0 {
            length = -length;
            self.along = [-self.along[0], -self.along[1]];
        }
        let line = Geom2dLine::from_point(
            Point2::new(self.at[0], self.at[1]),
            Direction2::new(self.along[0], self.along[1])?,
        );
        self.at[0] += length * self.along[0];
        self.at[1] += length * self.along[1];
        Ok((Handle::new(line).cast(), length))
    }

    fn arc(&mut self, mut radius: f64, mut angle: f64) -> Result<Segment, Failure> {
        let mut sense = true;
        if radius < 0.0 {
            radius = -radius;
            sense = !sense;
            self.along = [-self.along[0], -self.along[1]];
        }
        let [dx, dy] = self.along;
        let center = Point2::new(self.at[0] - radius * dy, self.at[1] + radius * dx);
        let axis = Axis2d::new(center, Direction2::new(dy, -dx)?);
        if angle < 0.0 {
            angle = -angle;
            sense = !sense;
        }
        let circle: Handle<dyn Geom2dCurve> =
            Handle::new(Geom2dCircle::from_axis(axis, radius, sense)?).cast();
        let (point, tangent) = circle.d1(angle)?;
        self.at = [point.x(), point.y()];
        self.along = [tangent.x() / radius, tangent.y() / radius];
        Ok((circle, angle))
    }

    /// The closing line back to the first point, when it is not degenerate.
    fn closing(&mut self) -> Motion {
        let dx = self.first_point[0] - self.at[0];
        let dy = self.first_point[1] - self.at[1];
        let length = (dx * dx + dy * dy).sqrt();
        if length <= precision::confusion() {
            return Motion::None;
        }
        self.along = [dx / length, dy / length];
        Motion::Line(length)
    }

    fn edge(&self, curve: &Handle<dyn Geom2dCurve>, length: f64) -> Result<Edge, Failure> {
        if self.planar {
            let curve3d = geom_api::to_3d(curve, &self.plane)?;
            return BRepBuilderAPIMakeEdge::from_curve_params(&curve3d, 0.0, length)?.edge();
        }
        BRepBuilderAPIMakeEdge::from_pcurve_params(curve, &self.surface, 0.0, length)?.edge()
    }

    fn add(&mut self, wire: &mut BRepBuilderAPIMakeWire, motion: Motion) -> Result<(), Failure> {
        if let Some((curve, length)) = self.curve(motion)? {
            wire.add_edge(&self.edge(&curve, length)?)?;
        }
        Ok(())
    }

    /// The face or wire of the profile, moved to the face's location, with the 3D curves of a
    /// profile on a surface.
    fn result(&self, wire: &BRepBuilderAPIMakeWire, face: bool) -> Result<Shape, Failure> {
        let mut shape = if !face {
            wire.wire()?.into_shape()
        } else if self.planar {
            BRepBuilderAPIMakeFace::from_plane_wire(self.plane, &wire.wire()?, true)?
                .face()?
                .into_shape()
        } else {
            let mut maker = BRepBuilderAPIMakeFace::new();
            maker.init_surface(&self.surface, false, precision::confusion())?;
            maker.add(&wire.wire()?)?;
            maker.face()?.into_shape()
        };
        if !self.location.is_identity() {
            shape.move_by(&self.location, false)?;
        }
        if !self.planar {
            brep_lib::build_curves3d(&shape, 1.0e-5, Continuity::C1, 14, 0)?;
        }
        Ok(shape)
    }
}

fn face_of(s: &FeatScript<'_>, name: &str) -> Shape {
    let shape = s.shape_or_null(name);
    if shape.is_null() || !matches!(shape.shape_type(), Ok(ShapeEnum::Face)) {
        command_failed("profile : no face found");
    }
    shape
}

/// What one instruction does to the pen, and the move it asks for.
fn apply_code(s: &FeatScript<'_>, pen: &mut Pen, code: Code<'_>, stay_first: &mut bool) -> Motion {
    match code {
        Code::S(name) => {
            pen.set_face(&face_of(s, name));
            *stay_first = true;
            Motion::None
        }
        Code::O(x, y, z) => {
            pen.plane.set_location(Point3::new(x, y, z));
            *stay_first = true;
            Motion::None
        }
        Code::P(normal, x_axis) => {
            pen.set_frame(normal, x_axis);
            *stay_first = true;
            Motion::None
        }
        Code::F(x, y) => {
            if !pen.is_first {
                command_failed("profile: The F instruction must precede all moves");
            }
            pen.first_point = [x, y];
            pen.at = [x, y];
            *stay_first = true;
            Motion::None
        }
        Code::X(length) => {
            pen.along = [1.0, 0.0];
            Motion::Line(length)
        }
        Code::Y(length) => {
            pen.along = [0.0, 1.0];
            Motion::Line(length)
        }
        Code::TT(x, y) => to_point(pen, x, y),
        Code::C(radius, degrees) if radius.abs() > precision::confusion() => {
            Motion::Circle(radius, degrees * (PI / 180.0))
        }
        Code::C(..) | Code::W | Code::WW => Motion::None,
    }
}

fn to_point(pen: &mut Pen, x: f64, y: f64) -> Motion {
    let (vx, vy) = (x - pen.at[0], y - pen.at[1]);
    let length = (vx * vx + vy * vy).sqrt();
    if length <= precision::confusion() {
        return Motion::None;
    }
    pen.along = [vx / length, vy / length];
    Motion::Line(length)
}

/// `profile name codes...`, stopping where the command returns 1; `profile(s, "p", &[Code::X(1.)])`.
pub fn profile(s: &mut FeatScript<'_>, name: &str, codes: &[Code<'_>]) {
    let mut pen = Pen::new();
    let mut wire = BRepBuilderAPIMakeWire::new();
    let (mut face, mut close) = (true, true);
    for (index, code) in codes.iter().enumerate() {
        if matches!(code, Code::W | Code::WW) {
            face = false;
            close = *code == Code::W && close;
        }
        let mut stay_first = false;
        let motion = apply_code(s, &mut pen, *code, &mut stay_first);
        checked(pen.add(&mut wire, motion));
        pen.is_first = stay_first;
        if index + 1 == codes.len() && close {
            let closing = pen.closing();
            checked(pen.add(&mut wire, closing));
        }
    }
    let shape = checked(pen.result(&wire, face));
    s.bind(name, &shape);
}
