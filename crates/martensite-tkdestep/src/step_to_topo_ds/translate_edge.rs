//! `StepToTopoDSTranslateEdge`: an edge_curve to an edge with its two vertices and parameters.
use super::{StepToTopoDSNMTool, StepToTopoDSRoot, StepToTopoDSTool};
use super::{StepToTopoDSTranslateEdgeError, StepToTopoDSTranslateVertex};
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_geom::{
    StepGeomCartesianPoint, StepGeomCurve, StepGeomPcurve, StepGeomSurfaceCurve,
};
use crate::step_shape::{
    StepShapeEdge, StepShapeEdgeCurve, StepShapeOrientedEdge, StepShapeVertex, StepShapeVertexPoint,
};
use crate::step_to_geom::StepToGeom;
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topods::{self, Edge, Shape, Vertex};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::{GeomCurve, GeomLine, GeomPoint, GeomSurface};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkgeombase::geom_convert::Units as GeomConvertUnits;
use martensite_tkmath::gp::{Line3, Point3};
use martensite_tkshhealing::shape_construct::ShapeConstructCurve;
use martensite_tktopalgo::brep_lib::{BRepLibEdgeError, BRepLibMakeEdge};
use martensite_tkxsbase::transfer::TransferTransientProcess;

/// The warning or the failure of an edge the maker refused; adjusts the curve when it can.
pub(super) fn decode_make_edge_error(
    maker: &BRepLibMakeEdge,
    orig: &Handle<StandardTransient>,
    curve: &Handle<dyn GeomCurve>,
    v1: &Vertex,
    v2: &Vertex,
    process: &TransferTransientProcess,
) -> Result<(), Failure> {
    let text = match maker.error() {
        BRepLibEdgeError::EdgeDone => return Ok(()),
        BRepLibEdgeError::PointProjectionFailed => " Point Projection failed",
        BRepLibEdgeError::ParameterOutOfRange => " Parameter Out Of Range",
        BRepLibEdgeError::DifferentPointsOnClosedCurve => " Different Points on Closed Curve",
        BRepLibEdgeError::PointWithInfiniteParameter => " Point with infinite Parameter",
        BRepLibEdgeError::LineThroughIdenticPoints => " Line through identic Points",
        BRepLibEdgeError::DifferentsPointAndParameter => {
            let (p1, p2) = (BRepTool::pnt(v1)?, BRepTool::pnt(v2)?);
            if ShapeConstructCurve.adjust_curve(curve, p1, p2, true, true)? {
                process.add_warning_text(orig, "Different Points and Parameters, adjusted", "");
                return Ok(());
            }
            " Different Points and Parameters"
        }
    };
    process.add_fail_text(orig, text, "");
    Ok(())
}

/// The curve of an entity, once for each entity of the transfer.
pub(super) fn make_curve(
    entity: &Handle<StepGeomCurve>,
    process: &TransferTransientProcess,
    factors: &StepDataFactors,
) -> Result<Handle<dyn GeomCurve>, Failure> {
    let start: Handle<StandardTransient> = entity.cast();
    let known: Handle<dyn GeomCurve> = process.find_transient(&start).cast();
    if known.is_some() {
        return Ok(known);
    }
    let made = StepToGeom::make_curve(entity, factors)?;
    if made.is_some() {
        process.bind_transient(&start, &made.cast())?;
    }
    Ok(made)
}

/// An edge on a curve without check, for what the maker refuses.
pub(super) fn make_edge(
    curve: &Handle<dyn GeomCurve>,
    v1: &Vertex,
    v2: &Vertex,
    u1: f64,
    u2: f64,
) -> Result<Edge, Failure> {
    let builder = BRepBuilder::default();
    let mut edge = Edge::default();
    builder.make_edge_with_curve(&mut edge, curve, precision::confusion())?;
    builder.add(&mut edge, v1)?;
    builder.add(&mut edge, v2)?;
    builder.update_vertex_on_edge(v1, u1, &edge, 0.0)?;
    builder.update_vertex_on_edge(v2, u2, &edge, 0.0)?;
    Ok(edge)
}

/// :e6 abv 16 Apr 98: ProSTEP TR8, r0601_sy.stp, #14907: the points of a closed edge's vertices.
pub(super) fn cartesian_points(
    edge_curve: &StepShapeEdgeCurve,
    points: &mut [Point3; 2],
    factors: &StepDataFactors,
) -> Result<(), Failure> {
    for (index, slot) in points.iter_mut().enumerate() {
        let vertex = if (index == 0) == edge_curve.same_sense() {
            edge_curve.edge_start()
        } else {
            edge_curve.edge_end()
        };
        let vertex_point = vertex.cast::<StepShapeVertexPoint>();
        let Some(live) = vertex_point.get() else {
            continue;
        };
        let point: Handle<StepGeomCartesianPoint> = live.vertex_geometry().cast();
        let made = StepToGeom::make_cartesian_point(&point, factors)?;
        *slot = step_live(&made, "StepToTopoDSTranslateEdge")?.pnt();
    }
    Ok(())
}

/// #25415: an edge curve that is a line displaced from its true position with a correct direction
/// (some files of FPX Expert 2013, a PCB design system) is shifted to pass through the vertices.
pub(super) fn shift_displaced_line(
    curve: &Handle<dyn GeomCurve>,
    start: Point3,
    u1: f64,
) -> Option<Handle<dyn GeomCurve>> {
    let line = curve.cast::<GeomLine>();
    let live = line.get()?;
    let mut shifted: Line3 = live.lin();
    let origin = start.xyz() - shifted.position().direction().xyz() * u1;
    shifted.set_location(Point3::new(origin.x(), origin.y(), origin.z()));
    Some(Handle::new(GeomLine::from_lin(shifted)).cast())
}

/// What one translation reads or changes while it makes the 3D edge.
pub(super) struct EdgeInProgress<'a> {
    pub(super) entity: &'a Handle<StepShapeEdgeCurve>,
    pub(super) v_end: &'a Handle<StepShapeVertex>,
    pub(super) preci: f64,
    pub(super) v1: Vertex,
    pub(super) v2: Vertex,
    pub(super) edge: Edge,
}

/// Translate edge.
#[derive(Default)]
pub struct StepToTopoDSTranslateEdge {
    /// Root.
    pub root: StepToTopoDSRoot,
    pub(super) my_result: Shape,
    pub(super) my_error: StepToTopoDSTranslateEdgeError,
}

impl StepToTopoDSTranslateEdge {
    /// `StepToTopoDSTranslateEdge()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `StepToTopoDSTranslateEdge(E, T, NMTool, factors)`.
    pub fn with_edge(
        edge: &Handle<StepShapeEdge>,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let mut made = Self::new();
        made.init(edge, tool, nm_tool, factors)?;
        Ok(made)
    }
    pub(super) fn finish(&mut self, shape: Shape) {
        self.my_result = shape;
        self.my_error = StepToTopoDSTranslateEdgeError::Done;
        self.root.done = true;
    }
    /// The shape a tool holds, reversed as the oriented edge asks.
    fn oriented_shared_edge(
        shape: &Shape,
        oriented: &Handle<StepShapeOrientedEdge>,
    ) -> Result<Shape, Failure> {
        let live = step_live(oriented, "StepToTopoDSTranslateEdge")?;
        Ok(if live.orientation() {
            shape.clone()
        } else {
            shape.reversed()
        })
    }
    /// A result already there: from the tool, unless degenerated, or from the non-manifold tool
    /// (ssv; 12.11.2010), or by name in an I-DEAS-like file (ssv; 15.11.2010).
    fn reuse_known_edge(
        &mut self,
        edge_curve: &Handle<StepShapeEdgeCurve>,
        oriented: &Handle<StepShapeOrientedEdge>,
        name: Option<&AsciiString>,
        tool: &StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
    ) -> Result<bool, Failure> {
        if tool.is_bound(&edge_curve.cast()) {
            let found = tool.find(&edge_curve.cast())?.clone();
            self.my_result = found.clone();
            if !BRepTool::degenerated(&topods::edge(&found)?)? {
                self.finish(found);
                return Ok(true);
            }
            let process = tool.transient_process();
            step_live(&process, "StepToTopoDSTranslateEdge")?.add_warning_text(
                &edge_curve.cast(),
                "Degenerated Edge in several faces : transferred for each face",
                "",
            );
        }
        if nm_tool.is_active() && nm_tool.is_bound(&edge_curve.cast()) {
            let existing = nm_tool.find(&edge_curve.cast())?.clone();
            self.finish(Self::oriented_shared_edge(&existing, oriented)?);
            return Ok(true);
        }
        let Some(name) = name.filter(|_| nm_tool.is_ideas_case()) else {
            return Ok(false);
        };
        if !nm_tool.is_bound_name(name) {
            return Ok(false);
        }
        let existing = Self::oriented_shared_edge(nm_tool.find_name(name)?, oriented)?;
        // Register Edge for final processing (I-DEAS case)
        nm_tool.register_nm_edge(&existing);
        self.finish(existing);
        Ok(true)
    }
    /// The vertices in the geometrical sense: the first forward, the last reversed.
    fn translate_vertices(
        v_start: &Handle<StepShapeVertex>,
        v_end: &Handle<StepShapeVertex>,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        factors: &StepDataFactors,
    ) -> Result<(Vertex, Vertex), Failure> {
        let first = StepToTopoDSTranslateVertex::with_vertex(v_start, tool, nm_tool, factors)?;
        let last = StepToTopoDSTranslateVertex::with_vertex(v_end, tool, nm_tool, factors)?;
        let mut v1 = Vertex::default();
        let mut v2 = Vertex::default();
        if first.is_done() {
            v1 = topods::vertex(first.value()?)?;
            v1.set_orientation(Orientation::Forward);
        }
        if v_end == v_start {
            v2 = v1.clone();
            v2.set_orientation(Orientation::Reversed);
        } else if last.is_done() {
            v2 = topods::vertex(last.value()?)?;
            v2.set_orientation(Orientation::Reversed);
        }
        Ok((v1, v2))
    }
    /// `init(edge, tool, NMTool, factors)`: an edge with two vertices and two parameters, always
    /// built FORWARD.
    pub fn init(
        &mut self,
        edge: &Handle<StepShapeEdge>,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let oriented = edge.cast::<StepShapeOrientedEdge>();
        let working = match oriented.get() {
            Some(live) => live.edge_element(),
            None => edge.clone(),
        };
        let edge_curve = working.cast::<StepShapeEdgeCurve>();
        let name_handle = edge_curve.get().map(|live| live.name()).unwrap_or_default();
        let name = name_handle
            .get()
            .filter(|live| !live.is_empty())
            .map(|live| live.string().clone());
        if self.reuse_known_edge(&edge_curve, &oriented, name.as_ref(), tool, nm_tool)? {
            return Ok(());
        }
        let live = step_live(&edge_curve, "StepToTopoDSTranslateEdge")?;
        let process_handle = tool.transient_process();
        let process = step_live(&process_handle, "StepToTopoDSTranslateEdge")?;
        let curve = live.edge_geometry();
        if curve.is_null() {
            process.add_fail_text(
                &edge_curve.cast(),
                " Geom Curve in EdgeCurve is equal to 0",
                "",
            );
            self.my_error = StepToTopoDSTranslateEdgeError::Other;
            self.root.done = false;
            return Ok(());
        }
        // The start and end vertices corresponding to FORWARD, following the geometrical sense
        let (v_start, v_end) = if live.same_sense() {
            (live.edge_start(), live.edge_end())
        } else {
            (live.edge_end(), live.edge_start())
        };
        let (v1, v2) = Self::translate_vertices(&v_start, &v_end, tool, nm_tool, factors)?;
        self.root.done = true;
        let mut state = EdgeInProgress {
            entity: &edge_curve,
            v_end: &v_end,
            preci: self.root.precision(),
            v1,
            v2,
            edge: Edge::default(),
        };
        // A pcurve gives no 3D curve; a surface curve gives its 3D curve; else it is a 3D curve.
        if curve.cast::<StepGeomPcurve>().is_some() {
            let builder = BRepBuilder::default();
            builder.make_edge(&mut state.edge)?;
            builder.add(&mut state.edge, &state.v1)?;
            builder.add(&mut state.edge, &state.v2)?;
        } else if let Some(surface_curve) = curve.cast::<StepGeomSurfaceCurve>().get() {
            self.make_from_curve_3d_in(&surface_curve.curve3d(), &mut state, tool, factors)?;
        } else {
            self.make_from_curve_3d_in(&curve, &mut state, tool, factors)?;
        }
        if !self.root.done {
            return Ok(());
        }
        let shape = state.edge.into_shape();
        tool.bind(&edge_curve.cast(), &shape)?;
        // Bind Edge in NM tool (ssv; 15.11.2010)
        if nm_tool.is_active() {
            nm_tool.bind(&edge_curve.cast(), &shape);
            if let Some(name) = name.filter(|_| nm_tool.is_ideas_case()) {
                nm_tool.bind_name(&name, &shape);
            }
        }
        self.my_result = shape;
        self.my_error = StepToTopoDSTranslateEdgeError::Done;
        Ok(())
    }
    /// `MakePCurve(PCU, ConvSurf, factors)`: an individual pcurve; a failure gives what was made.
    pub fn make_pcurve(
        &self,
        pcurve: &Handle<StepGeomPcurve>,
        surface: &Handle<dyn GeomSurface>,
        factors: &StepDataFactors,
    ) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        let live = step_live(pcurve, "StepToTopoDSTranslateEdge::MakePCurve")?;
        let representation = live.reference_to_curve();
        let Some(representation) = representation.get() else {
            return Ok(Handle::default());
        };
        let curve: Handle<StepGeomCurve> = representation.items_value(1)?.cast();
        let Ok(made) = StepToGeom::make_curve_2d(&curve, factors) else {
            return Ok(Handle::default());
        };
        if made.is_null() {
            return Ok(made);
        }
        // if the surface is a RectangularTrimmedSurface, send the BasisSurface.
        let converted = GeomConvertUnits::degree_to_radian(
            &made,
            surface,
            factors.length_factor(),
            factors.factor_degree_radian(),
        );
        Ok(converted.unwrap_or(made))
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
        self.root.check_done("StepToTopoDSTranslateEdge::Value()")?;
        Ok(&self.my_result)
    }
    /// `Error`.
    pub fn error(&self) -> StepToTopoDSTranslateEdgeError {
        self.my_error
    }
}
