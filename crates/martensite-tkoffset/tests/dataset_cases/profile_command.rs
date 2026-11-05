//! The profile commands: a planar (or on-surface) wire or face drawn by a moving point
//! and direction.
use crate::script::{OffsetScript, command_stopped};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dCurve, Geom2dLine};
use martensite_tkg3d::geom::{GeomPlane, GeomSurface};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeomalgo::geom_api;
use martensite_tkmath::gp::{
    Axis2, Axis2d, Axis3, Direction2, Direction3, Plane3, Point2, Point3, Vector3, xoy,
};
use martensite_tkmath::toploc::Location;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeWire,
};
use martensite_tktopalgo::brep_lib;

/// A 2D curve of the profile and its parameter length.
type Segment = (Handle<dyn Geom2dCurve>, f64);

/// A move of the profile point.
#[derive(Clone, Copy)]
enum Motion {
    None,
    Line(f64),
    Circle(f64, f64),
}

/// Why a profile instruction stopped the command.
enum Stop {
    /// `goto badargs`: too few values for the instruction.
    BadArgs,
    /// A message, then return 1.
    Error(String),
}

/// The moving point and direction, and the first point.
struct Cursor {
    x0: f64,
    y0: f64,
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    first: bool,
    stay_first: bool,
}

impl Cursor {
    fn new() -> Self {
        Self {
            x0: 0.0,
            y0: 0.0,
            x: 0.0,
            y: 0.0,
            dx: 1.0,
            dy: 0.0,
            first: true,
            stay_first: false,
        }
    }

    /// The second character of the instruction is `upper` or its lower case.
    fn doubled(code: &str, upper: u8) -> bool {
        code.as_bytes()
            .get(1)
            .is_some_and(|c| c.eq_ignore_ascii_case(&upper))
    }

    /// The common instructions of `profile` and `2dprofile`; `ok(None)` for another code.
    /// `i` points at the instruction and is left at its last value.
    fn instruction(
        &mut self,
        args: &ProfileWords<'_>,
        i: &mut usize,
    ) -> Result<Option<Motion>, Stop> {
        let n = args.len();
        let code = args.name(*i);
        let mut take = |count: usize| -> Result<usize, Stop> {
            *i += count;
            if *i >= n { Err(Stop::BadArgs) } else { Ok(*i) }
        };
        let motion = match code.as_bytes().first().map(u8::to_ascii_uppercase) {
            Some(b'F') => {
                let at = take(2)?;
                if !self.first {
                    return Err(Stop::Error(
                        "profile: The F instruction must precede all moves".into(),
                    ));
                }
                (self.x0, self.y0) = (args.real(at - 1), args.real(at));
                (self.x, self.y) = (self.x0, self.y0);
                self.stay_first = true;
                Motion::None
            }
            Some(b'X') => {
                let at = take(1)?;
                let mut length = args.real(at);
                if Self::doubled(code, b'X') {
                    length -= self.x;
                }
                (self.dx, self.dy) = (1.0, 0.0);
                Motion::Line(length)
            }
            Some(b'Y') => {
                let at = take(1)?;
                let mut length = args.real(at);
                if Self::doubled(code, b'Y') {
                    length -= self.y;
                }
                (self.dx, self.dy) = (0.0, 1.0);
                Motion::Line(length)
            }
            Some(b'L') => Motion::Line(args.real(take(1)?)),
            Some(b'T') => {
                let at = take(2)?;
                let (mut vx, mut vy) = (args.real(at - 1), args.real(at));
                if Self::doubled(code, b'T') {
                    vx -= self.x;
                    vy -= self.y;
                }
                let length = (vx * vx + vy * vy).sqrt();
                if length > precision::confusion() {
                    (self.dx, self.dy) = (vx / length, vy / length);
                    Motion::Line(length)
                } else {
                    Motion::None
                }
            }
            Some(b'R') => {
                let angle = args.real(take(1)?) * (std::f64::consts::PI / 180.0);
                if Self::doubled(code, b'R') {
                    (self.dx, self.dy) =
                        (martensite_cmath::cos(angle), martensite_cmath::sin(angle));
                } else {
                    let (c, s) = (martensite_cmath::cos(angle), martensite_cmath::sin(angle));
                    let t = c * self.dx - s * self.dy;
                    self.dy = s * self.dx + c * self.dy;
                    self.dx = t;
                }
                Motion::None
            }
            Some(b'D') => {
                let at = take(2)?;
                let (vx, vy) = (args.real(at - 1), args.real(at));
                let length = (vx * vx + vy * vy).sqrt();
                if length > precision::confusion() {
                    (self.dx, self.dy) = (vx / length, vy / length);
                }
                Motion::None
            }
            Some(b'C') => {
                let at = take(2)?;
                let radius = args.real(at - 1);
                if radius.abs() > precision::confusion() {
                    Motion::Circle(radius, args.real(at) * (std::f64::consts::PI / 180.0))
                } else {
                    Motion::None
                }
            }
            Some(b'I') => self.intersection(args, take(1)?, code)?,
            _ => return Ok(None),
        };
        Ok(Some(motion))
    }

    /// `IX`/`IY`: the line to the intersection with a vertical or horizontal.
    fn intersection(&self, args: &ProfileWords<'_>, at: usize, code: &str) -> Result<Motion, Stop> {
        let value = args.real(at);
        let (delta, origin) = if Self::doubled(code, b'X') {
            (self.dx, self.x)
        } else if Self::doubled(code, b'Y') {
            (self.dy, self.y)
        } else {
            return Ok(Motion::None);
        };
        if delta.abs() < precision::confusion() {
            return Err(Stop::Error(format!(
                "Profile : cannot intersect, arg {}",
                at - 1
            )));
        }
        Ok(Motion::Line((value - origin) / delta))
    }

    /// The 2D curve of a move from the current point (normalizing negative lengths and
    /// radii as the reference does), and the point and direction after it.
    fn curve(&mut self, motion: Motion) -> Result<Option<Segment>, Failure> {
        match motion {
            Motion::None => Ok(None),
            Motion::Line(mut length) => {
                if length < 0.0 {
                    length = -length;
                    (self.dx, self.dy) = (-self.dx, -self.dy);
                }
                let line = Geom2dLine::from_point(
                    Point2::new(self.x, self.y),
                    Direction2::new(self.dx, self.dy)?,
                );
                self.x += length * self.dx;
                self.y += length * self.dy;
                Ok(Some((Handle::new(line).cast(), length)))
            }
            Motion::Circle(mut radius, mut angle) => {
                let mut sense = true;
                if radius < 0.0 {
                    radius = -radius;
                    sense = !sense;
                    (self.dx, self.dy) = (-self.dx, -self.dy);
                }
                let center = Point2::new(self.x - radius * self.dy, self.y + radius * self.dx);
                let axis = Axis2d::new(center, Direction2::new(self.dy, -self.dx)?);
                if angle < 0.0 {
                    angle = -angle;
                    sense = !sense;
                }
                let circle: Handle<dyn Geom2dCurve> =
                    Handle::new(Geom2dCircle::from_axis(axis, radius, sense)?).cast();
                let (p, v) = circle.d1(angle)?;
                (self.x, self.y) = (p.x(), p.y());
                (self.dx, self.dy) = (v.x() / radius, v.y() / radius);
                Ok(Some((circle, angle)))
            }
        }
    }

    /// The closing line back to the first point, when it is not degenerate.
    fn closing(&mut self) -> Motion {
        let (dx, dy) = (self.x0 - self.x, self.y0 - self.y);
        let length = (dx * dx + dy * dy).sqrt();
        if length <= precision::confusion() {
            return Motion::None;
        }
        (self.dx, self.dy) = (dx / length, dy / length);
        Motion::Line(length)
    }
}

/// The plane or surface the `profile` wire lies on.
struct Support {
    plane: Plane3,
    planar: bool,
    surface: Handle<dyn GeomSurface>,
    location: Location,
}

impl Support {
    fn new() -> Result<Self, Failure> {
        Ok(Self {
            plane: Plane3::from_position(Axis3::from_axis2(xoy())),
            planar: true,
            surface: Handle::default(),
            location: Location::default(),
        })
    }

    /// The edge of a 2D curve `[0, length]`: on the plane, or as a p-curve on the surface.
    fn edge(
        &self,
        curve: &Handle<dyn Geom2dCurve>,
        length: f64,
    ) -> Result<martensite_tkbrep::topods::Edge, Failure> {
        if self.planar {
            BRepBuilderAPIMakeEdge::from_curve_params(
                &geom_api::to_3d(curve, &self.plane)?,
                0.0,
                length,
            )?
            .edge()
        } else {
            BRepBuilderAPIMakeEdge::from_pcurve_params(curve, &self.surface, 0.0, length)?.edge()
        }
    }

    /// `S face`: the plane of a planar face, otherwise its surface and location.
    fn set_face(&mut self, script: &OffsetScript<'_>, name: &str) -> Result<bool, Failure> {
        let face = face_named(script, name);
        if face.is_null() {
            return Ok(false);
        }
        let (surface, location) = BRepTool::surface_with_location(&topods::face(&face)?)?;
        self.surface = surface;
        self.location = location;
        match self.surface.cast::<GeomPlane>().get() {
            Some(plane) => self.plane = plane.pln(),
            None => self.planar = false,
        }
        Ok(true)
    }

    /// The face or wire of the profile, moved to the face location, with the 3D curves
    /// of a profile on a surface.
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
            brep_lib::build_curves3d(
                &shape,
                1.0e-5,
                martensite_tkmath::geom_abs::Shape::C1,
                14,
                0,
            )?;
        }
        Ok(shape)
    }
}

/// The `profile` instructions that only it has: `O`, `P`, `S` and `W`.
fn plane_instruction(
    script: &OffsetScript<'_>,
    args: &ProfileWords<'_>,
    i: &mut usize,
    support: &mut Support,
    cursor: &mut Cursor,
) -> Result<Option<Result<(), Stop>>, Failure> {
    let n = args.len();
    let code = args.name(*i);
    match code.as_bytes().first().map(u8::to_ascii_uppercase) {
        Some(b'O') => {
            *i += 3;
            if *i >= n {
                return Ok(Some(Err(Stop::BadArgs)));
            }
            support.plane.set_location(args.point(*i - 2));
        }
        Some(b'P') => {
            *i += 6;
            if *i >= n {
                return Ok(Some(Err(Stop::BadArgs)));
            }
            let (vn, vx) = (args.vector(*i - 5), args.vector(*i - 2));
            if vn.magnitude() <= precision::confusion() || vx.magnitude() <= precision::confusion()
            {
                return Ok(Some(Err(Stop::Error("profile : null direction".into()))));
            }
            let frame = Axis2::with_x_direction(
                support.plane.location(),
                Direction3::from_vector(vn)?,
                Direction3::from_vector(vx)?,
            )?;
            support.plane.set_position(Axis3::from_axis2(frame));
        }
        Some(b'S') => {
            *i += 1;
            if *i >= n {
                return Ok(Some(Err(Stop::BadArgs)));
            }
            if !support.set_face(script, args.name(*i))? {
                return Ok(Some(Err(Stop::Error("profile : no face found".into()))));
            }
        }
        _ => return Ok(None),
    }
    cursor.stay_first = true;
    Ok(Some(Ok(())))
}

/// Reports a stop of a profile command and gives its status.
fn stopped_by(stop: Stop) -> Failure {
    match stop {
        Stop::BadArgs => command_stopped("profile : bad number of arguments"),
        Stop::Error(message) => command_stopped(&message),
    }
}

/// Adds the edge of `motion` to the wire.
fn add_move(
    support: &Support,
    cursor: &mut Cursor,
    wire: &mut BRepBuilderAPIMakeWire,
    motion: Motion,
) -> Result<(), Failure> {
    if let Some((curve, length)) = cursor.curve(motion)? {
        wire.add_edge(&support.edge(&curve, length)?)?;
    }
    Ok(())
}

/// `profile name [code values] ...`; the words are the script's, its numbers already resolved.
/// Example: `profile(script, "p", &["X", "10", "Y", "5"])`.
pub fn profile(
    script: &mut OffsetScript<'_>,
    name: &'static str,
    words: &[&str],
) -> Result<(), Failure> {
    let mut a: Vec<&str> = vec!["profile", name];
    a.extend_from_slice(words);
    let n = a.len();
    let args = ProfileWords { words: &a };
    let (mut cursor, mut support) = (Cursor::new(), Support::new()?);
    let mut wire = BRepBuilderAPIMakeWire::new();
    let (mut face, mut close) = (true, true);
    let mut i = 2;
    while i < n {
        let motion = match plane_instruction(script, &args, &mut i, &mut support, &mut cursor)? {
            Some(Ok(())) => Motion::None,
            Some(Err(stop)) => return Err(stopped_by(stop)),
            None if a[i]
                .as_bytes()
                .first()
                .is_some_and(|c| c.eq_ignore_ascii_case(&b'W')) =>
            {
                face = false;
                if Cursor::doubled(a[i], b'W') {
                    close = false;
                }
                i = n - 1;
                Motion::None
            }
            None => match cursor.instruction(&args, &mut i) {
                Ok(Some(motion)) => motion,
                Ok(None) => {
                    return Err(command_stopped(&format!("profile : unknown code {}", a[i])));
                }
                Err(stop) => return Err(stopped_by(stop)),
            },
        };
        add_move(&support, &mut cursor, &mut wire, motion)?;
        cursor.first = cursor.stay_first;
        cursor.stay_first = false;
        i += 1;
        if i == n && close {
            let closing = cursor.closing();
            if let Motion::Line(_) = closing {
                add_move(&support, &mut cursor, &mut wire, closing)?;
                i += 1;
            }
        }
    }
    let shape = support.result(&wire, face)?;
    script.set_shape(name, shape);
    Ok(())
}

/// The words of a `profile`: the number conversion of a word that is no number reads as 0.
struct ProfileWords<'a> {
    words: &'a [&'a str],
}

impl ProfileWords<'_> {
    fn len(&self) -> usize {
        self.words.len()
    }

    fn name(&self, index: usize) -> &str {
        self.words[index]
    }

    fn real(&self, index: usize) -> f64 {
        self.words
            .get(index)
            .and_then(|word| word.parse().ok())
            .unwrap_or(0.)
    }

    fn point(&self, index: usize) -> Point3 {
        Point3::new(self.real(index), self.real(index + 1), self.real(index + 2))
    }

    fn vector(&self, index: usize) -> Vector3 {
        Vector3::new(self.real(index), self.real(index + 1), self.real(index + 2))
    }
}

fn face_named(script: &OffsetScript<'_>, name: &str) -> Shape {
    let shape = script.shape(name);
    if !shape.is_null() && shape.shape_type().ok() == Some(ShapeEnum::Face) {
        return shape;
    }
    Shape::default()
}
