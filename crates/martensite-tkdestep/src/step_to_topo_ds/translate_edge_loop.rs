//! `StepToTopoDSTranslateEdgeLoop`: an edge_loop of a face bound to a wire with its pcurves.
//!
//! The preparation passes are in `edge_loop_prepare`, the check of the pcurves in `edge_loop_check`.
use super::edge_loop_check::{check_pcurves, remove_single_pcurve};
use super::edge_loop_prepare::{connect_adjacent_edges, prepare_vertices_and_curves};
use super::{
    StepToTopoDSNMTool, StepToTopoDSRoot, StepToTopoDSTool, StepToTopoDSTranslateEdge,
    StepToTopoDSTranslateEdgeLoopError,
};
use crate::de_step::DeStepReadModeSurfaceCurve;
use crate::step_data::{StepDataFactors, StepDataStepModel};
use crate::step_entity::step_live;
use crate::step_geom::StepGeomSurface;
use crate::step_shape::{
    StepShapeEdgeCurve, StepShapeEdgeLoop, StepShapeFaceBound, StepShapeOrientedEdge,
};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topods::{self, Edge, Face, Shape, ShapeIterator, Wire};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::{GeomPlane, GeomRectangularTrimmedSurface, GeomSurface};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkshhealing::shape_algo::ShapeAlgo;
use martensite_tkxsbase::interface::InterfaceInterfaceModel;
use martensite_tkxsbase::transfer::TransferTransientProcess;

/// The surfaces and the senses of one translation, read once.
pub(super) struct FaceContext<'a> {
    pub(super) face: &'a Face,
    pub(super) step_surface: &'a Handle<StepGeomSurface>,
    pub(super) converted_surface: Handle<dyn GeomSurface>,
    pub(super) edge_loop: &'a Handle<StepShapeEdgeLoop>,
    pub(super) same_sense: bool,
    pub(super) forward_wire: bool,
    pub(super) is_plane: bool,
    pub(super) is_c3d_only: bool,
    pub(super) preci: f64,
    pub(super) max_tol: f64,
}

/// The locals of the loop over the edges: they outlive one edge on purpose. A pcurve of an
/// earlier edge stays until it is replaced, and an edge whose curve has no case keeps `has_pcurve`.
pub(super) struct EdgeLoopLocals {
    pub(super) c2d: Handle<dyn Geom2dCurve>,
    pub(super) c2d1: Handle<dyn Geom2dCurve>,
    pub(super) c2d2: Handle<dyn Geom2dCurve>,
    pub(super) has_pcurve: bool,
}

/// Translate edge loop.
#[derive(Default)]
pub struct StepToTopoDSTranslateEdgeLoop {
    /// Root.
    pub root: StepToTopoDSRoot,
    pub(super) my_result: Shape,
    pub(super) my_error: StepToTopoDSTranslateEdgeLoopError,
}

impl StepToTopoDSTranslateEdgeLoop {
    /// `StepToTopoDSTranslateEdgeLoop()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `StepToTopoDSTranslateEdgeLoop(FB, Face, GeomSurf, StepSurf, sameSense, T, NMTool, factors)`.
    #[allow(clippy::too_many_arguments, reason = "the constructor")]
    pub fn with_bound(
        face_bound: &Handle<StepShapeFaceBound>,
        face: &Face,
        surface: &Handle<dyn GeomSurface>,
        step_surface: &Handle<StepGeomSurface>,
        same_sense: bool,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let mut made = Self::new();
        made.init(
            face_bound,
            face,
            surface,
            step_surface,
            same_sense,
            tool,
            nm_tool,
            factors,
        )?;
        Ok(made)
    }
    pub(super) fn fail_edge(&mut self) {
        self.root.done = false;
        self.my_error = StepToTopoDSTranslateEdgeLoopError::Other;
    }
    /// True when the file asks for the 3D curves only (`ReadSurfaceCurveMode` 3).
    fn reads_c3d_only(process: &TransferTransientProcess) -> Result<bool, Failure> {
        let model: Handle<dyn InterfaceInterfaceModel> = process.model();
        let step_model = model.cast::<StepDataStepModel>();
        let live = step_live(&step_model, "StepToTopoDSTranslateEdgeLoop")?;
        let mode = live.internal_parameters().read_surface_curve_mode;
        // 0,1: suivre le code, 2: ne prendre que pcurve, 3: ne prendre que C3D
        Ok(mode == DeStepReadModeSurfaceCurve::USE_3D_PREFERRED)
    }
    /// `init(FaceBound, Face, GeomSurf, StepSurf, sameSense, tool, NMTool, factors)`.
    #[allow(clippy::too_many_arguments, reason = "the member")]
    pub fn init(
        &mut self,
        face_bound: &Handle<StepShapeFaceBound>,
        face: &Face,
        surface: &Handle<dyn GeomSurface>,
        step_surface: &Handle<StepGeomSurface>,
        same_sense: bool,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        self.root.done = true;
        let bound = step_live(face_bound, "StepToTopoDSTranslateEdgeLoop")?;
        let edge_loop = bound.bound().cast::<StepShapeEdgeLoop>();
        if tool.is_bound(&edge_loop.cast()) {
            self.my_result = topods::wire(tool.find(&edge_loop.cast())?)?.into_shape();
            self.my_error = StepToTopoDSTranslateEdgeLoopError::Done;
            return Ok(());
        }
        let process_handle = tool.transient_process();
        let process = step_live(&process_handle, "StepToTopoDSTranslateEdgeLoop")?;
        let is_c3d_only = Self::reads_c3d_only(process)?;
        let live_loop = step_live(&edge_loop, "StepToTopoDSTranslateEdgeLoop")?;
        let loop_entity: Handle<StandardTransient> = edge_loop.cast();
        if live_loop.nb_edge_list() == 0 {
            process.add_warning_text(
                &loop_entity,
                "Wire not done. EdgeLoop does not contain edges.",
                "",
            );
            self.root.done = false;
            return Ok(());
        }
        // A rectangular trimmed surface sends its basis surface to the pcurves.
        let trimmed = surface.cast::<GeomRectangularTrimmedSurface>();
        let converted_surface = match trimmed.get() {
            Some(live) => live.basis_surface(),
            None => surface.clone(),
        };
        tool.set_compute_pcurve(false);
        let context = FaceContext {
            face,
            step_surface,
            converted_surface,
            edge_loop: &edge_loop,
            same_sense,
            forward_wire: bound.orientation(),
            is_plane: surface.cast::<GeomPlane>().is_some(),
            is_c3d_only,
            preci: self.root.precision(),
            max_tol: self.root.max_tol(),
        };
        let builder = BRepBuilder::default();
        let mut wire = Wire::default();
        builder.make_wire(&mut wire);
        prepare_vertices_and_curves(live_loop, tool, nm_tool, process, factors)?;
        connect_adjacent_edges(
            live_loop,
            &loop_entity,
            context.preci,
            tool,
            nm_tool,
            process,
            factors,
        )?;
        // PTV 16.09.2000: default value set as true (if not correct see logic of algorithm).
        let mut locals = EdgeLoopLocals {
            c2d: Handle::default(),
            c2d1: Handle::default(),
            c2d2: Handle::default(),
            has_pcurve: true,
        };
        for rank in 1..=live_loop.nb_edge_list() {
            let oriented = live_loop.edge_list_value(rank)?;
            self.translate_oriented_edge(
                &oriented,
                &context,
                &mut locals,
                &mut wire,
                tool,
                nm_tool,
                factors,
            )?;
        }
        // The EdgeLoop is binded in the Wire
        if !self.root.done {
            process.add_fail_text(&loop_entity, "At least one edge failed : wire not done", "");
            return Ok(());
        }
        wire.set_closed(BRepTool::is_closed(&wire)?)?;
        tool.bind(&edge_loop.cast(), &wire)?;
        // pdn compute parameter of Vertices using projecting
        if !tool.compute_pcurve() {
            Self::compute_vertex_parameters(&wire, face, context.preci)?;
        }
        self.my_result = wire.clone().into_shape();
        self.my_error = StepToTopoDSTranslateEdgeLoopError::Done;
        self.root.done = true;
        // Check des PCurves SYSTEMATIQUE, s il n y en a que quelques unes
        check_pcurves(&wire, face, context.is_plane, context.preci)
    }
    /// The 2D parameters of the vertices on the edges of the wire, by projection.
    fn compute_vertex_parameters(wire: &Wire, face: &Face, preci: f64) -> Result<(), Failure> {
        let builder = BRepBuilder::default();
        let mut iterator = ShapeIterator::new(wire, true, true);
        while iterator.more() {
            let edge = topods::edge(iterator.value()?)?;
            let projector = step_algo_edge_proj_aux()?;
            let live = step_live(&projector, "StepToTopoDSTranslateEdgeLoop")?;
            live.init(face, &edge);
            live.compute(preci)?;
            if !(live.is_first_done() && live.is_last_done()) {
                remove_single_pcurve(&edge, face)?;
            } else if (live.first_param() - live.last_param()).abs() >= precision::p_confusion() {
                builder.range_on_face(&edge, face, live.first_param(), live.last_param())?;
            }
            iterator.next();
        }
        Ok(())
    }
    /// The edge of a rank: oriented and with its pcurves, added to the wire when nothing failed.
    /// A return before the addition is a `continue`.
    #[allow(clippy::too_many_arguments, reason = "the locals of the loop body")]
    fn translate_oriented_edge(
        &mut self,
        oriented: &Handle<StepShapeOrientedEdge>,
        context: &FaceContext<'_>,
        locals: &mut EdgeLoopLocals,
        wire: &mut Wire,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let Some(mut live_oriented) = oriented.get() else {
            return Ok(());
        };
        let element = live_oriented.edge_element();
        if element.is_null() {
            return Ok(());
        }
        // see bug #29979: oriented edge contains another oriented edge
        let nested = element.cast::<StepShapeOrientedEdge>();
        let working = if nested.is_some() {
            nested
        } else {
            oriented.clone()
        };
        live_oriented = step_live(&working, "StepToTopoDSTranslateEdgeLoop")?;
        let edge_curve = live_oriented.edge_element().cast::<StepShapeEdgeCurve>();
        let Some(live_curve) = edge_curve.get() else {
            return Ok(());
        };
        let process_handle = tool.transient_process();
        let process = step_live(&process_handle, "StepToTopoDSTranslateEdgeLoop")?;
        let curve_entity: Handle<StandardTransient> = edge_curve.cast();
        // Map the StepEdge
        let mut translator = StepToTopoDSTranslateEdge::new();
        translator.root.set_precision(context.preci);
        translator.root.set_max_tol(context.max_tol);
        translator.init(&working.cast(), tool, nm_tool, factors)?;
        let mut edge = Edge::default();
        if translator.is_done() {
            edge = topods::edge(translator.value()?)?;
            if edge.is_null() {
                return Ok(()); // NULL, on saute
            }
            let forward = live_oriented.orientation() == live_curve.same_sense();
            edge.set_orientation(if forward {
                Orientation::Forward
            } else {
                Orientation::Reversed
            });
            if !self.update_edge_pcurves(
                &translator,
                &edge,
                &edge_curve,
                context,
                locals,
                tool,
                factors,
            )? {
                return Ok(());
            }
            if edge.is_null() {
                process.add_fail_text(&curve_entity, " an Edge not mapped", "");
                self.fail_edge();
            }
        } else {
            // The Edge is Not mapped => switch to next wire ?
            process.add_fail_text(&curve_entity, " an Edge not mapped", "");
            self.fail_edge();
        }
        if self.root.done {
            BRepBuilder::default().add(wire, &edge)?; // on le fait ici. Sauf si erreur rencontree ... !
            return Ok(());
        }
        let (start, end) = (live_curve.edge_start(), live_curve.edge_end());
        if start.is_some() && end.is_some() && start == end {
            self.root.done = true;
            process.add_fail_text(
                &context.edge_loop.cast(),
                " Edge with equal vertices failed, scipped",
                "",
            );
        }
        Ok(())
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
        self.root
            .check_done("StepToTopoDSTranslateEdgeLoop::Value()")?;
        Ok(&self.my_result)
    }
    /// `Error`.
    pub fn error(&self) -> StepToTopoDSTranslateEdgeLoopError {
        self.my_error
    }
}

/// `ShapeAlgo::algo_container()->ToolContainer()->EdgeProjAux()`.
fn step_algo_edge_proj_aux()
-> Result<Handle<martensite_tkshhealing::shape_fix::ShapeFixEdgeProjAux>, Failure> {
    let algorithms = ShapeAlgo::algo_container();
    let live = step_live(&algorithms, "ShapeAlgo::AlgoContainer")?;
    let tools = live.tool_container();
    Ok(step_live(&tools, "ShapeAlgoAlgoContainer::ToolContainer")?.edge_proj_aux())
}
