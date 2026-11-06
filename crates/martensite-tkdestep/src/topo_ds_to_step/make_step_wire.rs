//! `TopoDSToStepMakeStepWire`: a wire to a poly_loop in a faceted context, else to an edge_loop
//! or a vertex_loop.
use super::{
    TopoDSToStepMakeStepEdge, TopoDSToStepMakeStepVertex, TopoDSToStepMakeWireError,
    TopoDSToStepRoot, TopoDSToStepTool, add_warning,
};
use crate::geom_to_step::{empty_step_name, shared_array1};
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_geom::StepGeomCartesianPoint;
use crate::step_shape::{
    StepShapeEdge, StepShapeEdgeLoop, StepShapeOrientedEdge, StepShapePolyLoop,
    StepShapeTopologicalRepresentationItem, StepShapeVertex, StepShapeVertexLoop,
    StepShapeVertexPoint,
};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_tools::WireExplorer;
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::{self, Wire};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkshhealing::shape_analysis::{ShapeAnalysisEdge, ShapeAnalysisWire};
use martensite_tkshhealing::shape_extend::ShapeExtendWireData;
use martensite_tkshhealing::shape_fix::ShapeFixWire;
use martensite_tkxsbase::transfer::TransferFinderProcess;

/// Make step wire.
#[derive(Clone, Default)]
pub struct TopoDSToStepMakeStepWire {
    /// Root.
    pub root: TopoDSToStepRoot,
    my_result: Handle<StepShapeTopologicalRepresentationItem>,
    my_error: TopoDSToStepMakeWireError,
}

/// `new ShapeFixWire(wire, face, Precision::confusion())`, reordered: for toroidal like surfaces
/// we need to use both (3d and 2d) mode to correctly reorder the edges.
fn reordered_wire_data(
    forward_wire: &Wire,
    tool: &TopoDSToStepTool,
) -> Result<Handle<ShapeExtendWireData>, Failure> {
    let analyzer =
        ShapeAnalysisWire::from_wire(forward_wire, tool.current_face(), precision::confusion())?;
    let fixer = ShapeFixWire::new();
    fixer.init(&Handle::new(analyzer));
    fixer.fix_reorder(true)?;
    fixer.wire_data()
}

/// `pdn remove degenerated pcurves`: the wire data of the edges that are not degenerated.
fn without_degenerated(data: &ShapeExtendWireData) -> Result<Handle<ShapeExtendWireData>, Failure> {
    let kept = Handle::new(ShapeExtendWireData::new());
    let live = step_live(&kept, "TopoDSToStepMakeStepWire")?;
    for index in 1..=data.nb_edges() {
        let edge = data.edge(index)?;
        if !BRepTool::degenerated(&edge)? {
            live.add_edge(&edge, 0)?;
        }
    }
    Ok(kept)
}

/// abv 04.05.00: CAX-IF TRJ4: writing complete sphere with single vertex_loop: true when the whole
/// wire is one seam (perhaps made of several seam edges). Moves the pair's first edge last.
fn is_whole_seam(data: &ShapeExtendWireData) -> Result<bool, Failure> {
    let nb = data.nb_edges();
    if nb % 2 != 0 {
        return Ok(false);
    }
    // check if two adjacent edges are the same
    let mut pair = 1;
    while pair < nb {
        let next = data.edge(pair + 1)?;
        if data.edge(pair)?.is_same(&next) {
            break;
        }
        pair += 1;
    }
    if pair >= nb {
        return Ok(false);
    }
    // make the first edge from pair last
    data.set_last(pair)?;
    let mut rank = nb / 2 + 1;
    while rank <= nb {
        let mirror = data.edge(nb - rank + 1)?;
        if !data.edge(rank)?.is_same(&mirror) {
            return Ok(false);
        }
        rank += 1;
    }
    Ok(true)
}

impl TopoDSToStepMakeStepWire {
    /// `TopoDSToStepMakeStepWire()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `TopoDSToStepMakeStepWire(W, T, FP, theLocalFactors)`.
    pub fn with_wire(
        wire: &Wire,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let mut made = Self::new();
        made.init(wire, tool, fp, factors)?;
        Ok(made)
    }
    fn finish(&mut self, result: Handle<StepShapeTopologicalRepresentationItem>) {
        self.my_error = TopoDSToStepMakeWireError::WireDone;
        self.my_result = result;
        self.root.done = true;
    }
    fn fail(
        &mut self,
        error: TopoDSToStepMakeWireError,
        wire: &Wire,
        fp: &Handle<TransferFinderProcess>,
        message: &str,
    ) -> Result<(), Failure> {
        add_warning(fp, wire, message)?;
        self.my_error = error;
        self.root.done = false;
        Ok(())
    }
    /// `init(wire, tool, FP, local_factors)`: the wire is given in its relative orientation
    /// (i.e. in the face).
    pub fn init(
        &mut self,
        wire: &Wire,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        tool.set_current_wire(wire);
        if tool.is_bound(wire) {
            self.finish(tool.find(wire)?);
            return Ok(());
        }
        if matches!(
            wire.orientation(),
            Orientation::Internal | Orientation::External
        ) {
            let text = " Wire(internal/external) from Non Manifold Topology";
            return self.fail(TopoDSToStepMakeWireError::NonManifoldWire, wire, fp, text);
        }
        if tool.faceted() {
            return self.init_poly_loop(wire, tool, fp, factors);
        }
        self.init_edge_loop(wire, tool, fp, factors)
    }
    fn init_poly_loop(
        &mut self,
        wire: &Wire,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let mut points: Vec<Handle<StepGeomCartesianPoint>> = Vec::new();
        let mut maker = TopoDSToStepMakeStepVertex::new();
        let mut explorer = WireExplorer::with_wire_on_face(wire, tool.current_face())?;
        while explorer.more() {
            let edge = explorer.current().clone();
            let (first, last) = topexp::vertices(&edge, false);
            let start = if edge.orientation() == Orientation::Forward {
                first
            } else {
                last
            };
            maker.init(&start, tool, fp, factors)?;
            if !maker.is_done() {
                let text = " a Vertex Point not mapped";
                return self.fail(TopoDSToStepMakeWireError::WireOther, wire, fp, text);
            }
            let vertex_point: Handle<StepShapeVertexPoint> = maker.value()?.cast();
            let point = step_live(&vertex_point, "TopoDSToStepMakeStepWire")?.vertex_geometry();
            points.push(point.cast());
            explorer.next()?;
        }
        if points.len() < 3 {
            let text = " PolyLoop: Wire has less than 3 points";
            return self.fail(TopoDSToStepMakeWireError::WireOther, wire, fp, text);
        }
        let poly_loop = Handle::new(StepShapePolyLoop::new());
        poly_loop.init(&empty_step_name(), &shared_array1(points));
        tool.bind(wire, &poly_loop.cast());
        self.finish(poly_loop.cast());
        Ok(())
    }
    fn init_edge_loop(
        &mut self,
        wire: &Wire,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let forward_wire = topods::wire(&wire.oriented(Orientation::Forward))?;
        let data = reordered_wire_data(&forward_wire, tool)?;
        let live = step_live(&data, "TopoDSToStepMakeStepWire")?;
        let seam_data = without_degenerated(live)?;
        let seam = step_live(&seam_data, "TopoDSToStepMakeStepWire")?;
        if is_whole_seam(seam)? {
            // make vertex_loop
            let vertex = ShapeAnalysisEdge::new().first_vertex(&seam.edge(1)?);
            let maker = TopoDSToStepMakeStepVertex::with_vertex(&vertex, tool, fp, factors)?;
            let loop_vertex: Handle<StepShapeVertex> = maker.value()?.cast();
            let vertex_loop = Handle::new(StepShapeVertexLoop::new());
            vertex_loop.init(&empty_step_name(), &loop_vertex);
            tool.bind(wire, &vertex_loop.cast());
            self.finish(vertex_loop.cast());
            return Ok(());
        }
        let Some(edges) = self.oriented_edges(wire, live, tool, fp, factors)? else {
            return Ok(());
        };
        if edges.is_empty() {
            let text = " No Edges of this Wire were mapped";
            return self.fail(TopoDSToStepMakeWireError::WireOther, wire, fp, text);
        }
        let edge_loop = Handle::new(StepShapeEdgeLoop::new());
        edge_loop.init(&empty_step_name(), &shared_array1(edges));
        tool.bind(wire, &edge_loop.cast());
        // Leaves the error code where it was on this path.
        self.my_result = edge_loop.cast();
        self.root.done = true;
        Ok(())
    }
    /// The oriented_edges of the wire data; `None` when an edge could not be mapped.
    fn oriented_edges(
        &mut self,
        wire: &Wire,
        data: &ShapeExtendWireData,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
    ) -> Result<Option<Vec<Handle<StepShapeOrientedEdge>>>, Failure> {
        let mut edges = Vec::new();
        let mut maker = TopoDSToStepMakeStepEdge::new();
        for index in 1..=data.nb_edges() {
            let edge = data.edge(index)?;
            if BRepTool::degenerated(&edge)? {
                add_warning(fp, wire, " EdgeLoop: Degenerated Pcurve not mapped")?;
                continue;
            }
            maker.init(&edge, tool, fp, factors)?;
            if !maker.is_done() {
                let text = " EdgeLoop: an Edge not mapped";
                self.fail(TopoDSToStepMakeWireError::WireOther, wire, fp, text)?;
                return Ok(None);
            }
            let element: Handle<StepShapeEdge> = maker.value()?.cast();
            let oriented = Handle::new(StepShapeOrientedEdge::new());
            oriented.init(
                &empty_step_name(),
                &element,
                edge.orientation() == Orientation::Forward,
            );
            edges.push(oriented);
        }
        Ok(Some(edges))
    }
    /// `is_done`.
    pub fn is_done(&self) -> bool {
        self.root.is_done()
    }
    /// `Value`.
    ///
    /// # Errors
    /// a not done failure when the translation gave no result.
    pub fn value(&self) -> Result<Handle<StepShapeTopologicalRepresentationItem>, Failure> {
        self.root.check_done("TopoDSToStepMakeStepWire::Value()")?;
        Ok(self.my_result.clone())
    }
    /// `Error`.
    pub fn error(&self) -> TopoDSToStepMakeWireError {
        self.my_error
    }
}
