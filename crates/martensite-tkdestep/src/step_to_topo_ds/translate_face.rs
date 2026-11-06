//! `StepToTopoDSTranslateFace`: a face_surface to a face with its wires. The tessellated overloads
//! are in `translate_tessellated`.
use super::{
    StepToTopoDSNMTool, StepToTopoDSRoot, StepToTopoDSTool, StepToTopoDSTranslateEdgeLoop,
    StepToTopoDSTranslateFaceError, StepToTopoDSTranslatePolyLoop, StepToTopoDSTranslateVertexLoop,
};
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_geom::{
    StepGeomBSplineSurface, StepGeomOffsetSurface, StepGeomRectangularTrimmedSurface,
    StepGeomSurface, StepGeomToroidalSurface,
};
use crate::step_shape::{
    StepShapeEdgeLoop, StepShapeFaceBound, StepShapeFaceOuterBound, StepShapeFaceSurface,
    StepShapeLoop, StepShapePolyLoop, StepShapeVertexLoop,
};
use crate::step_to_geom::StepToGeom;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{self, Face, Shape, ShapeIterator};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkg3d::geom::{
    GeomBSplineSurface, GeomPlane, GeomSphericalSurface, GeomSurface, GeomSurfaceOfRevolution,
    GeomToroidalSurface,
};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkshhealing::shape_algo::ShapeAlgo;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
use martensite_tkxsbase::transfer::TransferTransientProcess;

/// `FORWARD` for true, `REVERSED` for false.
fn orientation_of(forward: bool) -> Orientation {
    if forward {
        Orientation::Forward
    } else {
        Orientation::Reversed
    }
}

/// fix for bug 0026376: Solid Works wrote a face on a toroidal surface with a negative major radius;
/// "Solid Works" and "ProE" seem to interpret it as a face of reversed orientation.
fn is_reversed(surface: &Handle<StepGeomSurface>) -> Result<bool, Failure> {
    if let Some(trimmed) = surface.cast::<StepGeomRectangularTrimmedSurface>().get() {
        return is_reversed(&trimmed.basis_surface());
    }
    let torus = surface.cast::<StepGeomToroidalSurface>();
    Ok(torus.get().is_some_and(|live| live.major_radius() < 0.0))
}

/// What the bounds of one face share while they are translated.
struct FaceBuild<'a> {
    entity: &'a Handle<StepShapeFaceSurface>,
    step_surface: &'a Handle<StepGeomSurface>,
    surface: Handle<dyn GeomSurface>,
    same_sense: bool,
    preci: f64,
    max_tol: f64,
    process: &'a TransferTransientProcess,
}

/// Translate face.
#[derive(Default)]
pub struct StepToTopoDSTranslateFace {
    /// Root.
    pub root: StepToTopoDSRoot,
    my_result: Shape,
    my_error: StepToTopoDSTranslateFaceError,
}

impl StepToTopoDSTranslateFace {
    /// `StepToTopoDSTranslateFace()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `StepToTopoDSTranslateFace(FS, T, NMTool, factors)`.
    pub fn with_face_surface(
        face_surface: &Handle<StepShapeFaceSurface>,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let mut made = Self::new();
        made.init(face_surface, tool, nm_tool, factors)?;
        Ok(made)
    }
    fn finish(&mut self, shape: Shape) {
        self.my_result = shape;
        self.my_error = StepToTopoDSTranslateFaceError::Done;
        self.root.done = true;
    }
    fn fail(&mut self) {
        self.my_error = StepToTopoDSTranslateFaceError::Other;
        self.root.done = false;
    }
    /// The surface of the face: pdn to force a B-spline surface to be periodic. Null on failure.
    fn make_face_surface(
        step_surface: &Handle<StepGeomSurface>,
        process: &TransferTransientProcess,
        factors: &StepDataFactors,
    ) -> Result<Handle<dyn GeomSurface>, Failure> {
        let entity: Handle<StandardTransient> = step_surface.cast();
        if step_surface.cast::<StepGeomOffsetSurface>().is_some() {
            //:d4 abv 12 Mar 98
            process.add_warning_text(&entity, " Type OffsetSurface is out of scope of AP 214", "");
        }
        let surface = StepToGeom::make_surface(step_surface, factors)?;
        if surface.is_null() {
            process.add_fail_text(&entity, " Surface has not been created", "");
            return Ok(surface);
        }
        if step_surface.cast::<StepGeomBSplineSurface>().is_null() {
            return Ok(surface);
        }
        let algorithms = ShapeAlgo::algo_container();
        let periodic =
            step_live(&algorithms, "ShapeAlgo::AlgoContainer")?.convert_to_periodic(&surface)?;
        if periodic.is_null() {
            return Ok(surface);
        }
        process.add_warning_text(&entity, "Surface forced to be periodic", "");
        Ok(periodic)
    }
    /// The result of a tessellated overload.
    pub(super) fn finish_tessellated(&mut self, shape: Shape) {
        self.my_result = shape;
        self.my_error = StepToTopoDSTranslateFaceError::Done;
        self.root.done = true;
    }
    /// `init(face_surface, topo_ds_tool, topo_ds_tool_nm, factors)`.
    pub fn init(
        &mut self,
        face_surface: &Handle<StepShapeFaceSurface>,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        self.root.done = true;
        if tool.is_bound(&face_surface.cast()) {
            self.finish(topods::face(tool.find(&face_surface.cast())?)?.into_shape());
            return Ok(());
        }
        // Within this method the process is used for message handling only.
        let process_handle = tool.transient_process();
        let process = step_live(&process_handle, "StepToTopoDSTranslateFace")?;
        let live = step_live(face_surface, "StepToTopoDSTranslateFace")?;
        let step_surface = live.face_geometry();
        // sln 01.10.2001 BUC61003: an entity read with an error may have a null surface.
        if step_surface.is_null() {
            process.add_fail_text(&step_surface.cast(), " Surface has not been created", "");
            self.fail();
            return Ok(());
        }
        // Added to process non-manifold topology (ssv; 14.11.2010)
        if nm_tool.is_active() && nm_tool.is_bound(&step_surface.cast()) {
            // Reverse shape's orientation for the next shell
            self.finish(nm_tool.find(&step_surface.cast())?.reversed());
            return Ok(());
        }
        let surface = Self::make_face_surface(&step_surface, process, factors)?;
        let Some(live_surface) = surface.get() else {
            self.fail();
            return Ok(());
        };
        let same_sense = if is_reversed(&step_surface)? {
            !live.same_sense()
        } else {
            live.same_sense()
        };
        // -- Statistics --
        tool.add_continuity_surface(live_surface);
        let mut face = Face::default();
        BRepBuilder::default().make_face_on_surface(&mut face, &surface, precision::confusion())?;
        let build = FaceBuild {
            entity: face_surface,
            step_surface: &step_surface,
            surface: surface.clone(),
            same_sense,
            preci: self.root.precision(),
            max_tol: self.root.max_tol(),
            process,
        };
        for rank in 1..=live.nb_bounds()? {
            let bound = live.bounds_value(rank)?;
            Self::translate_bound(&build, &bound, &mut face, tool, nm_tool, factors)?;
        }
        face.set_orientation(orientation_of(live.same_sense()));
        let shape = face.into_shape();
        tool.bind(&face_surface.cast(), &shape)?;
        // Register face in NM tool (ssv; 14.11.2010)
        if nm_tool.is_active() {
            nm_tool.bind(&step_surface.cast(), &shape);
        }
        self.finish(shape);
        Ok(())
    }
    /// One face bound: a vertex loop, a poly loop or an edge loop. A bound that fails leaves the
    /// face without that wire: "a Wire is missing, well we continue anyway" (CKY JAN-97).
    fn translate_bound(
        build: &FaceBuild<'_>,
        bound: &Handle<StepShapeFaceBound>,
        face: &mut Face,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let Some(live_bound) = bound.get() else {
            return Ok(());
        };
        let face_loop: Handle<StepShapeLoop> = live_bound.bound();
        if face_loop.is_null() {
            return Ok(());
        }
        let vertex_loop = face_loop.cast::<StepShapeVertexLoop>();
        if vertex_loop.is_some() {
            return Self::translate_vertex_loop(build, &vertex_loop, face, tool, nm_tool, factors);
        }
        let poly_loop = face_loop.cast::<StepShapePolyLoop>();
        if poly_loop.is_some() {
            return Self::translate_poly_loop(
                build,
                &poly_loop,
                live_bound.orientation(),
                face,
                tool,
                factors,
            );
        }
        let entity: Handle<StandardTransient> = face_loop.cast();
        if face_loop.cast::<StepShapeEdgeLoop>().is_null() {
            // Type not yet implemented or non sens
            build
                .process
                .add_fail_text(&entity, " Type of loop not yet implemented", "");
            return Ok(());
        }
        let mut translator = StepToTopoDSTranslateEdgeLoop::new();
        translator.root.set_precision(build.preci); // gka
        translator.root.set_max_tol(build.max_tol);
        translator.init(
            bound,
            face,
            &build.surface,
            build.step_surface,
            build.same_sense,
            tool,
            nm_tool,
            factors,
        )?;
        if !translator.is_done() {
            // There was a problem in the mapping: We lost Face (so to speak...).
            // No way today to recover at least all the geometries (Points, 3D Curves, Surface).
            build
                .process
                .add_fail_text(&entity, " EdgeLoop not mapped to TopoDS", "");
            if bound.cast::<StepShapeFaceOuterBound>().is_some() {
                build.process.add_warning_text(
                    &build.entity.cast(),
                    "No Outer Bound : Face not done",
                    "",
                );
            }
            return Ok(());
        }
        // If the topological orientation is opposite to the geometric orientation of the surface,
        // the underlying topological orientation is not implicitly reversed in STEP: if the
        // face_surface is reversed, the wire orientation has to be explicitly reversed.
        let forward = live_bound.orientation() == build.same_sense;
        let mut wire = topods::wire(translator.value()?)?;
        wire.set_orientation(orientation_of(forward));
        BRepBuilder::default().add(face, &wire)
    }
    /// A vertex loop: PROBLEM if SPHERE or TORE; abv 10.07.00 pr1sy.stp: the loop can be wrong.
    fn translate_vertex_loop(
        build: &FaceBuild<'_>,
        vertex_loop: &Handle<StepShapeVertexLoop>,
        face: &mut Face,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let surface = &build.surface;
        let has_only_this_bound =
            step_live(build.entity, "StepToTopoDSTranslateFace")?.nb_bounds()? == 1;
        let closed_kind = surface.cast::<GeomSphericalSurface>().is_some()
            || surface.cast::<GeomBSplineSurface>().is_some()
            || surface.cast::<GeomSurfaceOfRevolution>().is_some();
        // ISO 10303 part 42: "If the face has only one bound and this is of type vertex_loop, then
        // the interior of the face is the domain of the face_surface.face_geometry. In such a case
        // the underlying surface shall be closed (e.g. a spherical_surface.)"
        if closed_kind && has_only_this_bound {
            return Self::add_natural_bounds(surface, build.preci, face);
        }
        if surface.cast::<GeomToroidalSurface>().is_some() {
            return Ok(());
        }
        let entity: Handle<StandardTransient> = vertex_loop.cast();
        if surface.cast::<GeomPlane>().is_some() {
            build
                .process
                .add_warning_text(&entity, "VertexLoop on plane is ignored", "");
            return Ok(()); // smh: BUC60809
        }
        let mut translator = StepToTopoDSTranslateVertexLoop::new();
        translator.root.set_precision(build.preci); // gka
        translator.root.set_max_tol(build.max_tol);
        translator.init(vertex_loop, tool, nm_tool, factors)?;
        if translator.is_done() {
            return BRepBuilder::default().add(face, translator.value()?);
        }
        build
            .process
            .add_warning_text(&entity, " a VertexLoop not mapped to TopoDS", "");
        Ok(())
    }
    /// The wires of the natural bounds of the surface.
    fn add_natural_bounds(
        surface: &Handle<dyn GeomSurface>,
        preci: f64,
        face: &mut Face,
    ) -> Result<(), Failure> {
        let auxiliary = BRepBuilderAPIMakeFace::from_surface(surface, preci)?;
        let natural = auxiliary.face()?;
        let builder = BRepBuilder::default();
        let mut iterator = ShapeIterator::new(&natural, true, true);
        while iterator.more() {
            builder.add(face, iterator.value()?)?;
            iterator.next();
        }
        Ok(())
    }
    /// A poly loop of a faceted B-rep.
    fn translate_poly_loop(
        build: &FaceBuild<'_>,
        poly_loop: &Handle<StepShapePolyLoop>,
        bound_forward: bool,
        face: &mut Face,
        tool: &mut StepToTopoDSTool,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let forward = step_live(build.entity, "StepToTopoDSTranslateFace")?.same_sense();
        face.set_orientation(orientation_of(forward));
        let mut translator = StepToTopoDSTranslatePolyLoop::new();
        translator.root.set_precision(build.preci); // gka
        translator.root.set_max_tol(build.max_tol);
        translator.init(poly_loop, tool, &build.surface, face, factors)?;
        if !translator.is_done() {
            let entity: Handle<StandardTransient> = poly_loop.cast();
            build
                .process
                .add_warning_text(&entity, " a PolyLoop not mapped to TopoDS", "");
            return Ok(());
        }
        let mut wire = topods::wire(translator.value()?)?;
        wire.set_orientation(orientation_of(bound_forward));
        BRepBuilder::default().add(face, &wire)
    }
    /// `is_done`.
    pub fn is_done(&self) -> bool {
        self.root.is_done()
    }
    /// `Value`.
    ///
    /// # Errors
    /// a not done failure without a result.
    pub fn value(&self) -> Result<&Shape, Failure> {
        self.root.check_done("StepToTopoDSTranslateFace::Value()")?;
        Ok(&self.my_result)
    }
    /// `Error`.
    pub fn error(&self) -> StepToTopoDSTranslateFaceError {
        self.my_error
    }
}
