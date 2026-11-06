//! `StepToTopoDSTranslateVertexLoop`: a vertex_loop to a wire on a degenerated edge.
use super::{
    StepToTopoDSNMTool, StepToTopoDSRoot, StepToTopoDSTool, StepToTopoDSTranslateVertex,
    StepToTopoDSTranslateVertexLoopError,
};
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_shape::StepShapeVertexLoop;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{self, Edge, Shape, Wire};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkg3d::top_abs::Orientation;

/// Translate vertex loop.
#[derive(Default)]
pub struct StepToTopoDSTranslateVertexLoop {
    /// Root.
    pub root: StepToTopoDSRoot,
    my_result: Shape,
    my_error: StepToTopoDSTranslateVertexLoopError,
}

impl StepToTopoDSTranslateVertexLoop {
    /// `StepToTopoDSTranslateVertexLoop()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `StepToTopoDSTranslateVertexLoop(VL, T, NMTool, factors)`.
    pub fn with_loop(
        vertex_loop: &Handle<StepShapeVertexLoop>,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let mut made = Self::new();
        made.init(vertex_loop, tool, nm_tool, factors)?;
        Ok(made)
    }
    fn finish(&mut self, shape: Shape) {
        self.my_result = shape;
        self.my_error = StepToTopoDSTranslateVertexLoopError::Done;
        self.root.done = true;
    }
    /// The wire of one degenerated edge on the vertex of the loop.
    fn degenerated_wire(vertex: &Shape) -> Result<Wire, Failure> {
        let builder = BRepBuilder::default();
        let mut edge = Edge::default();
        builder.make_edge(&mut edge)?;
        builder.add(&mut edge, &vertex.oriented(Orientation::Forward))?;
        builder.add(&mut edge, &vertex.oriented(Orientation::Reversed))?;
        builder.degenerated(&edge, true)?;
        let mut wire = Wire::default();
        builder.make_wire(&mut wire);
        wire.set_closed(true)?;
        builder.add(&mut wire, &edge)?;
        Ok(wire)
    }
    /// `init(VL, tool, NMTool, factors)`: a vertex loop is mapped onto a vertex, an edge and a wire.
    pub fn init(
        &mut self,
        vertex_loop: &Handle<StepShapeVertexLoop>,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        if tool.is_bound(&vertex_loop.cast()) {
            let found = topods::wire(tool.find(&vertex_loop.cast())?)?;
            self.finish(found.into_shape());
            return Ok(());
        }
        let live = step_live(vertex_loop, "StepToTopoDSTranslateVertexLoop")?;
        let translated =
            StepToTopoDSTranslateVertex::with_vertex(&live.loop_vertex(), tool, nm_tool, factors)?;
        if !translated.is_done() {
            let process = tool.transient_process();
            let entity: Handle<StandardTransient> = vertex_loop.cast();
            step_live(&process, "StepToTopoDSTranslateVertexLoop")?.add_warning_text(
                &entity,
                "VertexLoop not mapped to TopoDS ",
                "",
            );
            self.my_error = StepToTopoDSTranslateVertexLoopError::Other;
            self.root.done = false;
            return Ok(());
        }
        let vertex = topods::vertex(translated.value()?)?;
        let wire = Self::degenerated_wire(&vertex)?.into_shape();
        tool.bind(&vertex_loop.cast(), &wire)?;
        self.finish(wire);
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
            .check_done("StepToTopoDSTranslateVertexLoop::Value()")?;
        Ok(&self.my_result)
    }
    /// `Error`.
    pub fn error(&self) -> StepToTopoDSTranslateVertexLoopError {
        self.my_error
    }
}
