//! The variables of one case beyond the contract's shapes, and the steps that only build inputs.
use crate::input_shapes::{
    bezier_curve, box_solid, bspline_curve, cone_solid, conic_frame, cylinder_on_plane,
    cylinder_solid, sphere_solid, torus_solid, wedge_solid,
};
use crate::steps::{Curve, Place, Prim, Profile, Step, Sweep};
use crate::support::session::Session;
use martensite_tkbool::brep_fill::BRepFillPipe;
use martensite_tkbrep::{
    brep_tools::History,
    topexp,
    topods::{self, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::{
    geom::{GeomCircle, GeomCurve, GeomEllipse, GeomLine, GeomParabola},
    top_abs::ShapeEnum,
};
use martensite_tkgeomalgo::geom_fill::GeomFillTrihedron;
use martensite_tkmath::{
    gp::{Axis1, Direction3, Point3, Transform3, TransformForm, Vector3},
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakePrism, BRepPrimAPIMakeRevol};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon,
    BRepBuilderAPIMakeShape, BRepBuilderAPIMakeVertex, BRepBuilderAPIMakeWire,
    BRepBuilderAPITransform,
};
use std::collections::HashMap;
use std::f64::consts::PI;

/// What a command leaves behind when it stops the script: a null shape or a not-done
/// builder makes the command-host command return 1, which ends the native script as well.
pub const COMMAND_RETURNED_ONE: &str = "command returned 1";

/// One running script: the contract's session holds its shapes, this adds what a shape variable
/// is not. A curve hides a shape of the same name, as the one command-host name space does.
pub struct FilletScript<'s> {
    session: &'s mut dyn Session,
    curves: HashMap<String, Handle<dyn GeomCurve>>,
    curves_2d: HashMap<String, Handle<dyn Geom2dCurve>>,
    /// The history of the last Boolean operation, as the command-host session keeps it.
    pub last_history: Option<Handle<History>>,
    /// The history `savehistory` stored; the scripts use one name.
    pub saved_history: Option<Handle<History>>,
}

impl<'s> FilletScript<'s> {
    /// A script over `session`; `FilletScript::new(&mut raster)`.
    pub fn new(session: &'s mut dyn Session) -> Self {
        Self {
            session,
            curves: HashMap::new(),
            curves_2d: HashMap::new(),
            last_history: None,
            saved_history: None,
        }
    }

    /// The contract's session, for its checks and view commands.
    pub fn session(&mut self) -> &mut dyn Session {
        &mut *self.session
    }

    /// The shape in a variable: a null shape when no variable has the name, as `topology-command::Get`.
    pub fn shape(&self, name: &str) -> Shape {
        if self.curves.contains_key(name) || self.curves_2d.contains_key(name) {
            return Shape::default();
        }
        self.session.shape(name).cloned().unwrap_or_default()
    }

    /// The defined shape in a variable, or the failure that stops the script.
    pub fn required(&self, name: &str) -> Result<Shape, Failure> {
        let shape = self.shape(name);
        if shape.is_null() {
            return Err(command_stopped(&format!("{name} is not a defined shape")));
        }
        Ok(shape)
    }

    /// Fill a variable, replacing any curve of that name.
    pub fn set_shape(&mut self, name: &str, shape: Shape) {
        self.curves.remove(name);
        self.curves_2d.remove(name);
        self.session.bind(name, &shape);
    }

    /// Fill a variable with a 3D curve, which hides a shape of that name.
    pub fn set_curve(&mut self, name: &str, curve: Handle<dyn GeomCurve>) {
        self.curves_2d.remove(name);
        self.curves.insert(name.to_owned(), curve);
    }

    /// Fill a variable with a 2D curve.
    pub fn set_curve_2d(&mut self, name: &str, curve: Handle<dyn Geom2dCurve>) {
        self.curves.remove(name);
        self.curves_2d.insert(name.to_owned(), curve);
    }

    /// The 3D curve in a variable, if it holds one.
    pub fn curve(&self, name: &str) -> Option<Handle<dyn GeomCurve>> {
        self.curves.get(name).cloned()
    }

    /// The 2D curve in a variable, or the failure that stops the script.
    pub fn curve_2d(&self, name: &str) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        self.curves_2d
            .get(name)
            .cloned()
            .ok_or_else(|| command_stopped(&format!("{name} is not a 2D curve")))
    }

    /// `vertex name x y z`.
    pub fn apply_vertex(&mut self, name: &str, point: Point3) -> Result<(), Failure> {
        let vertex = BRepBuilderAPIMakeVertex::new(point)?.vertex()?;
        self.set_shape(name, vertex.as_ref().clone());
        Ok(())
    }

    /// The parts of an explosion, as `name_1`, `name_2` and so on.
    pub fn set_parts(&mut self, name: &str, parts: Vec<Shape>) {
        for (index, part) in parts.into_iter().enumerate() {
            self.session.bind(&format!("{name}_{}", index + 1), &part);
        }
    }

    /// Run a step that only builds inputs; `true` when the step belongs to this module.
    pub fn apply_input_step(&mut self, step: &Step) -> Result<bool, Failure> {
        match step {
            Step::Prim(prim) => self.primitive(prim)?,
            Step::Place(place) => self.place(place)?,
            Step::Curve(curve) => self.make_curve(curve)?,
            Step::Profile(profile) => self.profile(profile)?,
            Step::Sweep(sweep) => self.sweep(sweep)?,
            Step::Topo(topo) => self.topology(topo)?,
            Step::View(view) => self.session.view(*view),
            Step::Sample(sample) => crate::sample_steps::apply(self, sample)?,
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn primitive(&mut self, prim: &Prim) -> Result<(), Failure> {
        let (name, solid) = match prim {
            Prim::Box(name, v) => (*name, box_solid(v)?),
            Prim::Wedge(name, v) => (*name, wedge_solid(v)?),
            Prim::Cylinder(name, v) => (*name, cylinder_solid(v)?),
            Prim::CylinderOnPlane(name, plane, v) => (*name, cylinder_on_plane(plane, v)?),
            Prim::Cone(name, v) => (*name, cone_solid(v)?),
            Prim::Sphere(name, v) => (*name, sphere_solid(v)?),
            Prim::Torus(name, v) => (*name, torus_solid(v)?),
        };
        self.set_shape(name, solid.as_ref().clone());
        Ok(())
    }

    fn place(&mut self, place: &Place) -> Result<(), Failure> {
        let (names, trsf) = placement(place)?;
        for name in names {
            let moved = transformed(&self.required(name)?, trsf)?;
            self.set_shape(name, moved);
        }
        Ok(())
    }

    fn make_curve(&mut self, curve: &Curve) -> Result<(), Failure> {
        let (name, built) = match curve {
            Curve::Line(name, v) => (*name, line_curve(v)?),
            Curve::Circle(name, v) => (*name, circle_curve(v)?),
            Curve::Ellipse(name, v) => (*name, ellipse_curve(v)?),
            Curve::Parabola(name, v) => (*name, parabola_curve(v)?),
            Curve::Bezier(name, v) => (*name, bezier_curve(v)?),
            Curve::BSpline(name, v) => (*name, bspline_curve(v)?),
        };
        self.set_curve(name, built);
        Ok(())
    }

    fn profile(&mut self, profile: &Profile) -> Result<(), Failure> {
        match profile {
            Profile::Vertex(name, p) => self.apply_vertex(name, Point3::new(p[0], p[1], p[2])),
            Profile::EdgeOfVertices(name, first, last) => self.edge_of_vertices(name, first, last),
            Profile::Polyline(name, v) => self.polyline(name, v),
            Profile::Edge(name, curve, range) => self.edge_on_curve(name, curve, *range),
            Profile::Wire(name, sources) => self.wire_of(name, sources),
            Profile::Plane(name, wire) => self.plane_face(name, wire),
        }
    }

    fn polyline(&mut self, name: &'static str, values: &[f64]) -> Result<(), Failure> {
        let mut polygon = BRepBuilderAPIMakePolygon::new();
        for p in values.chunks_exact(3) {
            polygon.add_point(Point3::new(p[0], p[1], p[2]))?;
        }
        self.set_shape(name, polygon.wire()?.as_ref().clone());
        Ok(())
    }

    fn edge_of_vertices(
        &mut self,
        name: &'static str,
        first: &str,
        last: &str,
    ) -> Result<(), Failure> {
        let (first, last) = (self.required(first)?, self.required(last)?);
        let maker = BRepBuilderAPIMakeEdge::from_vertices(
            &topods::vertex(&first)?,
            &topods::vertex(&last)?,
        )?;
        self.set_shape(name, maker.edge()?.as_ref().clone());
        Ok(())
    }

    fn edge_on_curve(
        &mut self,
        name: &'static str,
        curve: &str,
        range: Option<[f64; 2]>,
    ) -> Result<(), Failure> {
        let handle = self
            .curves
            .get(curve)
            .ok_or_else(|| command_stopped(&format!("{curve} is not a defined curve")))?;
        let edge = match range {
            Some([first, last]) => {
                BRepBuilderAPIMakeEdge::from_curve_params(handle, first, last)?.edge()?
            }
            None => BRepBuilderAPIMakeEdge::from_curve(handle)?.edge()?,
        };
        self.set_shape(name, edge.as_ref().clone());
        Ok(())
    }

    fn wire_of(&mut self, name: &'static str, sources: &[&str]) -> Result<(), Failure> {
        let mut maker = BRepBuilderAPIMakeWire::new();
        for source in sources {
            add_to_wire(&mut maker, &self.shape(source))?;
        }
        if !maker.is_done() {
            return Err(command_stopped(&format!(
                "wire {name} not done: {sources:?}"
            )));
        }
        self.set_shape(name, maker.shape()?.clone());
        Ok(())
    }

    fn plane_face(&mut self, name: &'static str, wire: &str) -> Result<(), Failure> {
        let source = topods::wire(&self.required(wire)?)?;
        let maker = BRepBuilderAPIMakeFace::from_wire(&source, false)?;
        self.set_shape(name, maker.face()?.as_ref().clone());
        Ok(())
    }

    fn sweep(&mut self, sweep: &Sweep) -> Result<(), Failure> {
        let (name, shape) = match sweep {
            Sweep::Prism(name, base, v) => (*name, self.prism(base, v)?),
            Sweep::Revol(name, base, v) => (*name, self.revolution(base, v)?),
            Sweep::Pipe(name, spine, profile) => (*name, self.pipe(spine, profile)?),
        };
        self.set_shape(name, shape);
        Ok(())
    }

    fn prism(&self, base: &str, v: &[f64; 3]) -> Result<Shape, Failure> {
        let base = self.required(base)?;
        let vector = Vector3::new(v[0], v[1], v[2]);
        Ok(BRepPrimAPIMakePrism::new(&base, vector, false, true)?
            .shape()?
            .clone())
    }

    fn revolution(&self, base: &str, v: &[f64; 7]) -> Result<Shape, Failure> {
        let base = self.required(base)?;
        let mut revol =
            BRepPrimAPIMakeRevol::new(&base, axis(&v[..6])?, v[6] * (PI / 180.0), false)?;
        if !revol.is_done() {
            return Err(command_stopped("revol not done"));
        }
        Ok(revol.shape()?.clone())
    }

    /// `BRepOffsetAPIMakePipe` is only a wrapper of `BRepFillPipe`, and martensite-tkfillet does
    /// not reach martensite-tkoffset, so the sweep is called directly with the same arguments.
    fn pipe(&self, spine: &str, profile: &str) -> Result<Shape, Failure> {
        let spine = topods::wire(&self.required(spine)?)?;
        let profile = self.required(profile)?;
        let trihedron = GeomFillTrihedron::IsCorrectedFrenet;
        let pipe = BRepFillPipe::with_profile(&spine, &profile, trihedron, false, false)?;
        let mut map = IndexedMapOfShape::default();
        topexp::map_shapes(pipe.shape(), &mut map, true, true);
        if map.extent() == 1 {
            return Err(command_stopped(
                "pipe not done: the swept shape has no sub-shape",
            ));
        }
        Ok(pipe.shape().clone())
    }
}

/// The names a `ttranslate`, `trotate` or `tscale` moves, and the transformation.
fn placement(place: &Place) -> Result<(&'static [&'static str], Transform3), Failure> {
    let mut trsf = Transform3::default();
    let names = match place {
        Place::Translate(names, v) => {
            trsf.set_translation(Vector3::new(v[0], v[1], v[2]));
            *names
        }
        Place::Rotate(names, v) => {
            trsf.set_rotation(axis(&v[..6])?, v[6] * (PI / 180.0))?;
            *names
        }
        Place::Scale(names, v) => {
            trsf.set_scale(Point3::new(v[0], v[1], v[2]), v[3])?;
            *names
        }
    };
    Ok((names, trsf))
}

fn line_curve(v: &[f64]) -> Result<Handle<dyn GeomCurve>, Failure> {
    let direction = Direction3::from_vector(Vector3::new(v[3], v[4], v[5]))?;
    let line = GeomLine::from_point(Point3::new(v[0], v[1], v[2]), direction);
    Ok(Handle::new(line).cast())
}

fn circle_curve(v: &[f64]) -> Result<Handle<dyn GeomCurve>, Failure> {
    let (frame, r) = conic_frame(v, 1)?;
    Ok(Handle::new(GeomCircle::new(frame, r[0])?).cast())
}

fn ellipse_curve(v: &[f64]) -> Result<Handle<dyn GeomCurve>, Failure> {
    let (frame, r) = conic_frame(v, 2)?;
    Ok(Handle::new(GeomEllipse::new(frame, r[0], r[1])?).cast())
}

fn parabola_curve(v: &[f64]) -> Result<Handle<dyn GeomCurve>, Failure> {
    let (frame, r) = conic_frame(v, 1)?;
    Ok(Handle::new(GeomParabola::new(frame, r[0])?).cast())
}

/// `wire` takes edges and wires; a name holding anything else adds nothing.
fn add_to_wire(maker: &mut BRepBuilderAPIMakeWire, shape: &Shape) -> Result<(), Failure> {
    if shape.is_null() {
        return Ok(());
    }
    match shape.shape_type()? {
        ShapeEnum::Edge => maker.add_edge(&topods::edge(shape)?)?,
        ShapeEnum::Wire => maker.add_wire(&topods::wire(shape)?)?,
        _ => {}
    }
    Ok(())
}

/// The failure a command-host command returning 1 stands for.
pub fn command_stopped(detail: &str) -> Failure {
    Failure::with_message(COMMAND_RETURNED_ONE, detail)
}

fn axis(v: &[f64]) -> Result<Axis1, Failure> {
    let direction = Direction3::from_vector(Vector3::new(v[3], v[4], v[5]))?;
    Ok(Axis1::new(Point3::new(v[0], v[1], v[2]), direction))
}

/// The `transform` command: `Moved` for an identity, `BRepBuilderAPITransform` otherwise.
fn transformed(shape: &Shape, trsf: Transform3) -> Result<Shape, Failure> {
    if trsf.form() == TransformForm::Identity {
        return shape.moved(&Location::new(trsf), true);
    }
    let mut transformer = BRepBuilderAPITransform::new(trsf);
    transformer.perform(shape, false, false)?;
    if !transformer.is_done() {
        return Err(command_stopped("transform not done"));
    }
    Ok(transformer.shape()?.clone())
}
