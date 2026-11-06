//! `TopoDSToStepMakeStepVertex`: a vertex to a vertex_point.
use super::{TopoDSToStepMakeVertexError, TopoDSToStepRoot, TopoDSToStepTool, step_model_of};
use crate::geom_to_step::{GeomToStepMakeCartesianPoint, empty_step_name};
use crate::step_data::StepDataFactors;
use crate::step_shape::{StepShapeTopologicalRepresentationItem, StepShapeVertexPoint};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::Vertex;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkxsbase::transfer::TransferFinderProcess;
use martensite_tkxsbase::transfer_brep::TransferBRep;

/// Make step vertex.
#[derive(Clone, Default)]
pub struct TopoDSToStepMakeStepVertex {
    /// Root.
    pub root: TopoDSToStepRoot,
    my_result: Handle<StepShapeTopologicalRepresentationItem>,
    my_error: TopoDSToStepMakeVertexError,
}

impl TopoDSToStepMakeStepVertex {
    /// `TopoDSToStepMakeStepVertex()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `TopoDSToStepMakeStepVertex(V, T, FP, theLocalFactors)`.
    pub fn with_vertex(
        vertex: &Vertex,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let mut made = Self::new();
        made.init(vertex, tool, fp, factors)?;
        Ok(made)
    }
    fn finish(
        &mut self,
        error: TopoDSToStepMakeVertexError,
        result: Handle<StepShapeTopologicalRepresentationItem>,
    ) {
        self.my_error = error;
        self.my_result = result;
        self.root.done = true;
    }
    /// `init(vertex, tool, FP, local_factors)`.
    pub fn init(
        &mut self,
        vertex: &Vertex,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        tool.set_current_vertex(vertex)?;
        // [BEGIN] Processing non-manifold topology (ssv; 11.11.2010)
        let model = step_model_of(fp)?;
        if model.internal_parameters().write_nonmanifold {
            let mapper = TransferBRep::shape_mapper(fp, vertex)?;
            let mut known = Handle::<StepShapeVertexPoint>::default();
            let atype = StepShapeVertexPoint::static_type();
            if fp.find_typed_transient_as(&mapper.cast(), atype, &mut known) {
                // Non-manifold topology detected
                self.finish(TopoDSToStepMakeVertexError::VertexOther, known.cast());
                return Ok(());
            }
        }
        // [END] Processing non-manifold topology (ssv; 11.11.2010)
        if tool.is_bound(vertex) {
            self.finish(TopoDSToStepMakeVertexError::VertexOther, tool.find(vertex)?);
            return Ok(());
        }
        let point = BRepTool::pnt(vertex)?;
        let cartesian =
            GeomToStepMakeCartesianPoint::from_pnt(&point, factors.length_factor()).value()?;
        let vertex_point = Handle::new(StepShapeVertexPoint::new());
        vertex_point.init(&empty_step_name(), &cartesian.cast());
        tool.bind(vertex, &vertex_point.cast());
        self.finish(TopoDSToStepMakeVertexError::VertexDone, vertex_point.cast());
        Ok(())
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
        self.root
            .check_done("TopoDSToStepMakeStepVertex::Value()")?;
        Ok(self.my_result.clone())
    }
    /// `Error`.
    pub fn error(&self) -> TopoDSToStepMakeVertexError {
        self.my_error
    }
}
