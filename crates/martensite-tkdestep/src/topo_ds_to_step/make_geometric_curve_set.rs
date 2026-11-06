//! `TopoDSToStepMakeGeometricCurveSet`: the edges of a shape to a geometric_curve_set.
use super::{
    TopoDSToStep, TopoDSToStepRoot, TopoDSToStepShapeMap, TopoDSToStepTool,
    TopoDSToStepWireframeBuilder, step_model_of,
};
use crate::geom_to_step::{empty_step_name, shared_array1};
use crate::step_data::{StepDataFactors, StepDataSelectType};
use crate::step_entity::step_live;
use crate::step_shape::{StepShapeGeometricCurveSet, StepShapeGeometricSetSelect};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkxsbase::transfer::TransferFinderProcess;

/// Make geometric curve set.
#[derive(Clone, Default)]
pub struct TopoDSToStepMakeGeometricCurveSet {
    /// Root.
    pub root: TopoDSToStepRoot,
    the_geometric_curve_set: Handle<StepShapeGeometricCurveSet>,
}

impl TopoDSToStepMakeGeometricCurveSet {
    /// `TopoDSToStepMakeGeometricCurveSet(aShape, FP, theLocalFactors)`.
    pub fn new(
        shape: &Shape,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let mut made = Self::default();
        let model = step_model_of(fp)?;
        let mode = model.internal_parameters().write_surface_cur_mode;
        let mut tool = TopoDSToStepTool::with_map(&TopoDSToStepShapeMap::default(), false, mode);
        let wireframe = TopoDSToStepWireframeBuilder::with_shape(shape, &mut tool, factors)?;
        TopoDSToStep::add_tool_results(fp, &tool)?;
        if !wireframe.is_done() {
            return Ok(made);
        }
        let list = wireframe.value()?;
        let items = step_live(&list, "TopoDSToStepMakeGeometricCurveSet")?
            .sequence(|items| items.iter().cloned().collect::<Vec<_>>());
        if items.is_empty() {
            return Ok(made);
        }
        let mut elements = Vec::with_capacity(items.len());
        for item in &items {
            let mut select = StepShapeGeometricSetSelect::default();
            select.set_value(item)?;
            elements.push(select);
        }
        let set = Handle::new(StepShapeGeometricCurveSet::new());
        set.set_name(&empty_step_name());
        set.set_elements(&shared_array1(elements));
        made.the_geometric_curve_set = set;
        made.root.done = true;
        Ok(made)
    }
    /// `is_done`.
    pub fn is_done(&self) -> bool {
        self.root.is_done()
    }
    /// `Value`.
    ///
    /// # Errors
    /// a not done failure when the translation gave no result.
    pub fn value(&self) -> Result<Handle<StepShapeGeometricCurveSet>, Failure> {
        self.root
            .check_done("TopoDSToStepMakeGeometricCurveSet::Value()")?;
        Ok(self.the_geometric_curve_set.clone())
    }
}
