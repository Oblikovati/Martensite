//! The variables of one case beyond the contract's shapes, and the steps that only build inputs.
use crate::input_shapes::{
    bezier_curve, box_solid, bspline_curve, cone_solid, conic_frame, cylinder_solid,
    solid_on_plane, sphere_solid, torus_solid, wedge_solid,
};
use crate::steps::{Curve, Place, PlaneKind, Prim, Profile, Step, Sweep};
use crate::support::session::Session;
use martensite_tkbrep::{
    brep_tools::History,
    topods::{self, Shape, Solid},
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dCurve, Geom2dEllipse, Geom2dLine};
use martensite_tkg3d::{
    geom::{GeomCircle, GeomCurve, GeomEllipse, GeomHyperbola, GeomLine, GeomParabola, GeomPlane},
    top_abs::ShapeEnum,
};
use martensite_tkgeomalgo::geom_fill::GeomFillTrihedron;
use martensite_tkmath::{
    gp::{
        Axis1, Axis22d, Direction2, Direction3, Point2, Point3, Transform3, TransformForm, Vector2,
        Vector3, dx2d,
    },
    toploc::Location,
};
use martensite_tkoffset::brep_offset_api::BRepOffsetAPIMakePipe;
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakePrism, BRepPrimAPIMakeRevol};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIFaceError, BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeEdge2d,
    BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape,
    BRepBuilderAPIMakeVertex, BRepBuilderAPIMakeWire, BRepBuilderAPITransform,
};
use std::collections::HashMap;
use std::f64::consts::PI;

/// What a command leaves behind when it stops the script: a null shape or a not-done
/// builder makes the script command return 1, which ends the reference script as well.
pub const COMMAND_RETURNED_ONE: &str = "command returned 1";

/// One running script: the contract's session holds its shapes, this adds what a shape variable
/// is not. A curve hides a shape of the same name, as the one script name space does.
pub struct OffsetScript<'s> {
    session: &'s mut dyn Session,
    curves: HashMap<String, Handle<dyn GeomCurve>>,
    curves_2d: HashMap<String, Handle<dyn Geom2dCurve>>,
    /// The history of the last Boolean operation, as the script session keeps it.
    pub last_history: Option<Handle<History>>,
    /// The history `savehistory` stored; the scripts use one name.
    pub saved_history: Option<Handle<History>>,
    /// The offset and sweep commands' state across steps, as the command file's statics.
    pub commands: crate::command_state::CommandState,
    /// The fillet commands' tolerances and the evolved fillet in progress.
    pub fillets: crate::fillet_steps::FilletCommands,
    /// The form `featlf` defined for `featperform lf`.
    pub linear_form: Option<martensite_tkfeat::brep_feat::BRepFeatMakeLinearForm>,
    /// The arguments and filler of the last `bop a b`.
    pub prepared: Option<(Shape, Shape, martensite_tkbo::bop_algo::BOPAlgoPaveFiller)>,
    /// Named histories, from `savehistory`.
    pub saved_histories: HashMap<&'static str, Handle<History>>,
    /// The general-fuse sequence's arguments and pave filler.
    pub general_objects: Vec<Shape>,
    pub general_tools: Vec<Shape>,
    pub general_filler: Option<martensite_tkbo::bop_algo::BOPAlgoPaveFiller>,
    surfaces: HashMap<&'static str, Handle<dyn martensite_tkg3d::geom::GeomSurface>>,
    /// Variables a command filled with a null shape.
    null_names: std::collections::HashSet<String>,
    /// What the commands of `extra_steps` keep across steps.
    pub extras: crate::extra_steps::ExtraState,
    /// Values a script keeps in a variable to read later.
    pub values: crate::conditions::ScriptValues,
}

impl<'s> OffsetScript<'s> {
    /// A script over `session`; `OffsetScript::new(&mut raster)`.
    pub fn new(session: &'s mut dyn Session) -> Self {
        Self {
            session,
            curves: HashMap::new(),
            curves_2d: HashMap::new(),
            last_history: None,
            saved_history: None,
            commands: Default::default(),
            null_names: Default::default(),
            fillets: Default::default(),
            linear_form: None,
            prepared: None,
            saved_histories: HashMap::new(),
            general_objects: Vec::new(),
            general_tools: Vec::new(),
            general_filler: None,
            surfaces: HashMap::new(),
            extras: Default::default(),
            values: Default::default(),
        }
    }

    /// The contract's session, for its checks and view commands.
    pub fn session(&mut self) -> &mut dyn Session {
        &mut *self.session
    }

    /// The shape in a variable: a null shape when no variable has the name, as `DBRep::get`.
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
        self.set_shape_owned(name, shape);
    }

    /// Fill a variable whose name the script builds at run time.
    pub fn set_shape_owned(&mut self, name: &str, shape: Shape) {
        self.curves.remove(name);
        self.curves_2d.remove(name);
        if shape.is_null() {
            self.session.unbind(name);
            self.null_names.insert(name.to_owned());
            return;
        }
        self.null_names.remove(name);
        self.session.bind(name, &shape);
    }

    /// `[isdraw name]`: a variable a command filled with a null shape is drawn, and holds nothing.
    pub fn is_bound(&self, name: &str) -> bool {
        self.null_names.contains(name) || self.session.shape(name).is_some()
    }

    /// `name` is forgotten, as `unset` does.
    pub fn forget(&mut self, name: &str) {
        self.null_names.remove(name);
        self.session.unbind(name);
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

    /// Fill a variable with a surface.
    pub fn set_surface(
        &mut self,
        name: &'static str,
        surface: Handle<dyn martensite_tkg3d::geom::GeomSurface>,
    ) {
        self.surfaces.insert(name, surface);
    }

    /// The surface in a variable, if it holds one.
    pub fn surface(&self, name: &str) -> Option<Handle<dyn martensite_tkg3d::geom::GeomSurface>> {
        self.surfaces.get(name).cloned()
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
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn primitive(&mut self, prim: &Prim) -> Result<(), Failure> {
        let (name, solid) = match prim {
            Prim::Box(name, v) => (*name, box_solid(v)?),
            Prim::Wedge(name, v) => (*name, wedge_solid(v)?),
            Prim::Cylinder(name, v) => (*name, cylinder_solid(v)?),
            Prim::OnPlane(kind, name, plane, v) => {
                (*name, self.primitive_on_plane(*kind, plane, v)?)
            }
            Prim::Cone(name, v) => (*name, cone_solid(v)?),
            Prim::Sphere(name, v) => (*name, sphere_solid(v)?),
            Prim::Torus(name, v) => (*name, torus_solid(v)?),
        };
        self.set_shape(name, solid.as_ref().clone());
        Ok(())
    }

    fn primitive_on_plane(
        &self,
        kind: PlaneKind,
        plane: &str,
        v: &[f64],
    ) -> Result<Solid, Failure> {
        let frame = self.surface(plane).and_then(|surface| {
            surface
                .cast::<GeomPlane>()
                .get()
                .and_then(|p| p.pln().position().ax2().ok())
        });
        let frame = frame.ok_or_else(|| command_stopped(&format!("{plane} is not a plane")))?;
        solid_on_plane(kind, frame, v)
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
        // A `line x y dx dy`, `circle x y r` or `ellipse x y major minor` is a curve in the plane.
        match curve {
            Curve::Line(name, v) if v.len() == 4 => {
                let direction = Direction2::from_vector(Vector2::new(v[2], v[3]))?;
                let line = Geom2dLine::from_point(Point2::new(v[0], v[1]), direction);
                self.set_curve_2d(name, Handle::new(line).cast());
                return Ok(());
            }
            Curve::Circle(name, v) if v.len() == 3 => {
                let frame = planar_frame(v);
                let circle = Geom2dCircle::new(frame, v[2])?;
                self.set_curve_2d(name, Handle::new(circle).cast());
                return Ok(());
            }
            Curve::Ellipse(name, v) if v.len() == 4 => {
                let frame = planar_frame(v);
                let ellipse = Geom2dEllipse::new(frame, v[2], v[3])?;
                self.set_curve_2d(name, Handle::new(ellipse).cast());
                return Ok(());
            }
            _ => {}
        }
        let (name, built) = match curve {
            Curve::Line(name, v) => (*name, line_curve(v)?),
            Curve::Circle(name, v) => (*name, circle_curve(v)?),
            Curve::Ellipse(name, v) => (*name, ellipse_curve(v)?),
            Curve::Parabola(name, v) => (*name, parabola_curve(v)?),
            Curve::Bezier(name, v) => (*name, bezier_curve(v)?),
            Curve::BSpline(name, v) => (*name, bspline_curve(v, false)?),
            Curve::PeriodicBSpline(name, v) => (*name, bspline_curve(v, true)?),
            Curve::Hyperbola(name, v) => (*name, hyperbola_curve(v)?),
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
            Profile::Plane(name, wire, only_plane) => self.plane_face(name, wire, *only_plane),
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
        if let Some(planar) = self.curves_2d.get(curve) {
            let edge = match range {
                Some([first, last]) => {
                    BRepBuilderAPIMakeEdge2d::from_curve_params(planar, first, last)?.edge()?
                }
                None => BRepBuilderAPIMakeEdge2d::from_curve(planar)?.edge()?,
            };
            self.set_shape(name, edge.as_ref().clone());
            return Ok(());
        }
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

    /// `mkplane`: a builder error is a line of the stream and leaves the name unbound.
    fn plane_face(
        &mut self,
        name: &'static str,
        wire: &str,
        only_plane: bool,
    ) -> Result<(), Failure> {
        let source = topods::wire(&self.required(wire)?)?;
        let maker = BRepBuilderAPIMakeFace::from_wire(&source, only_plane)?;
        match maker.error() {
            BRepBuilderAPIFaceError::FaceDone => {
                self.set_shape(name, maker.face()?.as_ref().clone());
            }
            BRepBuilderAPIFaceError::NoFace => {
                println!("Error. mkplane has been finished with \"No Face\" status.");
            }
            BRepBuilderAPIFaceError::NotPlanar => {
                println!("Error. mkplane has been finished with \"Not Planar\" status.");
            }
            BRepBuilderAPIFaceError::CurveProjectionFailed => {
                println!(
                    "Error. mkplane has been finished with \"Fail in projection curve\" status."
                );
            }
            BRepBuilderAPIFaceError::ParametersOutOfRange => {
                println!(
                    "Error. mkplane has been finished with \"Parameters are out of range\" status."
                );
            }
        }
        Ok(())
    }

    fn sweep(&mut self, sweep: &Sweep) -> Result<(), Failure> {
        let (name, shape) = match sweep {
            Sweep::Prism(name, base, v) => (*name, self.prism(base, v)?),
            Sweep::Revol(name, base, v) => (*name, self.revolution(base, v)?),
            Sweep::Pipe(name, spine, profile, mode, approx) => {
                (*name, self.pipe(spine, profile, *mode, *approx)?)
            }
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

    /// `pipe result spine profile [mode [approx]]`.
    fn pipe(
        &mut self,
        spine: &str,
        profile: &str,
        mode: i32,
        approx: bool,
    ) -> Result<Shape, Failure> {
        let spine = topods::wire(&self.required(spine)?)?;
        let profile = self.required(profile)?;
        let trihedron = match mode {
            1 => GeomFillTrihedron::IsFrenet,
            2 => GeomFillTrihedron::IsDiscreteTrihedron,
            _ => GeomFillTrihedron::IsCorrectedFrenet,
        };
        let mut builder = BRepOffsetAPIMakePipe::with_mode(&spine, &profile, trihedron, approx)?;
        let shape = builder.shape()?.clone();
        let arguments = [profile.clone(), spine.as_ref().clone()];
        let history = History::from_algorithm(
            &arguments,
            &mut crate::more_steps::MakeShapeHistory(&mut builder),
        )?;
        self.last_history = Some(Handle::new(history));
        Ok(shape)
    }
}

/// A conic's frame in the plane, at the words' first point and along X.
fn planar_frame(v: &[f64]) -> Axis22d {
    Axis22d::new(Point2::new(v[0], v[1]), dx2d(), true).expect("a frame from a unit direction")
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

fn hyperbola_curve(v: &[f64]) -> Result<Handle<dyn GeomCurve>, Failure> {
    let (frame, r) = conic_frame(v, 2)?;
    Ok(Handle::new(GeomHyperbola::new(frame, r[0], r[1])?).cast())
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

/// The failure a script command returning 1 stands for.
pub fn command_stopped(detail: &str) -> Failure {
    Failure::with_message(COMMAND_RETURNED_ONE, detail)
}

fn axis(v: &[f64]) -> Result<Axis1, Failure> {
    let direction = Direction3::from_vector(Vector3::new(v[3], v[4], v[5]))?;
    Ok(Axis1::new(Point3::new(v[0], v[1], v[2]), direction))
}

/// The `transform` command: `moved` for an identity, `BRepBuilderAPITransform` otherwise.
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
