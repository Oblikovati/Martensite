//! `TopoDSToStepMakeFacetedBrepAndBrepWithVoids`: a solid with more than one closed shell to the
//! combined entity.
use super::make_brep_with_voids::shells_of;
use super::tessellated_items::tessellated_solid;
use super::{
    TopoDSToStep, TopoDSToStepBuilder, TopoDSToStepShapeMap, TopoDSToStepTool, add_warning,
    step_model_of, topo_ds_to_step_brep_class,
};
use crate::geom_to_step::{empty_step_name, shared_array1};
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_shape::{
    StepShapeClosedShell, StepShapeFacetedBrepAndBrepWithVoids, StepShapeOrientedClosedShell,
};
use crate::step_visual::StepVisualTessellatedItem;
use martensite_tkbrep::topods::{self, Shell, Solid};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tktopalgo::brep_class3d::BRepClass3d;
use martensite_tkxsbase::transfer::TransferFinderProcess;

topo_ds_to_step_brep_class!(
    /// Make faceted brep and brep with voids.
    TopoDSToStepMakeFacetedBrepAndBrepWithVoids,
    "TopoDSToStepMakeFacetedBrepAndBrepWithVoids",
    StepShapeFacetedBrepAndBrepWithVoids
);

/// What the shells of the solid gave.
#[derive(Default)]
struct ShellResults {
    outer: Handle<StepShapeClosedShell>,
    voids: Vec<Handle<StepShapeClosedShell>>,
    tessellated: Vec<Handle<StepVisualTessellatedItem>>,
}

/// Each closed shell through the builder; `None` when the progress stopped first.
fn translate_shells(
    solid: &Solid,
    outer_shell: &Shell,
    fp: &Handle<TransferFinderProcess>,
    factors: &StepDataFactors,
    progress: &ProgressRange<'_>,
) -> Result<Option<ShellResults>, Failure> {
    let model = step_model_of(fp)?;
    let mode = model.internal_parameters().write_surface_cur_mode;
    let map = TopoDSToStepShapeMap::default();
    let mut tool = TopoDSToStepTool::with_model(&model)?;
    let mut builder = TopoDSToStepBuilder::new();
    let mut results = ShellResults::default();
    let shells = shells_of(solid)?;
    let scope = ProgressScope::unnamed(progress, shells.len() as f64, false);
    for child in &shells {
        if !scope.more() {
            break;
        }
        let range = scope.next(1.0);
        let current = topods::shell(child)?;
        if !child.closed()? {
            let text = " Shell from Solid not closed; not mapped to FacetedBrepAndBrepWithVoids";
            add_warning(fp, &current, text)?;
            continue;
        }
        tool.init(&map, false, mode);
        builder.init(&current, &mut tool, fp, 0, factors, &range)?;
        TopoDSToStep::add_tool_results(fp, &tool)?;
        if !builder.is_done() {
            let text = " Shell from Solid not mapped to FacetedBrepAndBrepWithVoids";
            add_warning(fp, &current, text)?;
            continue;
        }
        let closed: Handle<StepShapeClosedShell> = builder.value()?.cast();
        if outer_shell.is_equal(&current) {
            results.outer = closed;
        } else {
            results.voids.push(closed);
        }
        let item = builder.tessellated_value()?;
        if item.is_some() {
            results.tessellated.push(item);
        }
    }
    Ok(scope.more().then_some(results))
}

impl TopoDSToStepMakeFacetedBrepAndBrepWithVoids {
    /// `TopoDSToStepMakeFacetedBrepAndBrepWithVoids(aSolid, FP, theLocalFactors, theProgress)`.
    pub fn new(
        solid: &Solid,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<Self, Failure> {
        let mut made = Self::default();
        let outer_shell = BRepClass3d::outer_shell(solid)?;
        let results = if outer_shell.is_null() {
            ShellResults::default()
        } else {
            let Some(results) = translate_shells(solid, &outer_shell, fp, factors, progress)?
            else {
                return Ok(made);
            };
            results
        };
        if results.voids.is_empty() {
            let text = " Solid contains no Shell to be mapped to FacetedBrepAndBrepWithVoids";
            add_warning(fp, solid, text)?;
            return Ok(made);
        }
        let name = empty_step_name();
        let mut oriented = Vec::with_capacity(results.voids.len());
        for void in &results.voids {
            step_live(void, "TopoDSToStepMakeFacetedBrepAndBrepWithVoids")?;
            let shell = Handle::new(StepShapeOrientedClosedShell::new());
            shell.init(&name, void, true);
            oriented.push(shell);
        }
        let brep = Handle::new(StepShapeFacetedBrepAndBrepWithVoids::new());
        brep.init_fields(&name, &results.outer, &shared_array1(oriented));
        made.result = brep;
        if !results.tessellated.is_empty() {
            made.tessellated = tessellated_solid(&results.tessellated, &made.result.cast())?;
        }
        made.root.done = true;
        Ok(made)
    }
}
