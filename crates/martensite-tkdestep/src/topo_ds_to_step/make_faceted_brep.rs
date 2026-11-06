//! `TopoDSToStepMakeFacetedBrep`: a closed shell, or the outer shell of a solid, to a faceted_brep.
use super::tessellated_items::tessellated_solid;
use super::{
    TopoDSToStep, TopoDSToStepBuilder, TopoDSToStepShapeMap, TopoDSToStepTool, add_warning,
    step_model_of, topo_ds_to_step_brep_class,
};
use crate::geom_to_step::empty_step_name;
use crate::step_data::StepDataFactors;
use crate::step_shape::{StepShapeClosedShell, StepShapeFacetedBrep};
use martensite_tkbrep::topods::{Shell, Solid};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tktopalgo::brep_class3d::BRepClass3d;
use martensite_tkxsbase::transfer::TransferFinderProcess;

topo_ds_to_step_brep_class!(
    /// Make faceted brep.
    TopoDSToStepMakeFacetedBrep,
    "TopoDSToStepMakeFacetedBrep",
    StepShapeFacetedBrep
);

/// The builder of a shell in a faceted context; `None` on a user break.
fn faceted_builder(
    shell: &Shell,
    fp: &Handle<TransferFinderProcess>,
    factors: &StepDataFactors,
    progress: &ProgressRange<'_>,
) -> Result<Option<TopoDSToStepBuilder>, Failure> {
    let model = step_model_of(fp)?;
    let (mode, tessellated) = {
        let parameters = model.internal_parameters();
        (
            parameters.write_surface_cur_mode,
            parameters.write_tessellated.as_raw(),
        )
    };
    let mut tool = TopoDSToStepTool::with_map(&TopoDSToStepShapeMap::default(), true, mode);
    let builder =
        TopoDSToStepBuilder::with_shape(shell, &mut tool, fp, tessellated, factors, progress)?;
    if progress.user_break() {
        return Ok(None);
    }
    TopoDSToStep::add_tool_results(fp, &tool)?;
    Ok(Some(builder))
}

/// The faceted_brep on the builder's closed shell.
fn faceted_brep_of(builder: &TopoDSToStepBuilder) -> Result<Handle<StepShapeFacetedBrep>, Failure> {
    let closed: Handle<StepShapeClosedShell> = builder.value()?.cast();
    let brep = Handle::new(StepShapeFacetedBrep::new());
    brep.init(&empty_step_name(), &closed);
    Ok(brep)
}

impl TopoDSToStepMakeFacetedBrep {
    /// `TopoDSToStepMakeFacetedBrep(aShell, FP, theLocalFactors, theProgress)`.
    pub fn with_shell(
        shell: &Shell,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<Self, Failure> {
        let mut made = Self::default();
        if !shell.closed()? {
            add_warning(fp, shell, " Shell not closed; not mapped to FacetedBrep")?;
            return Ok(made);
        }
        let Some(builder) = faceted_builder(shell, fp, factors, progress)? else {
            return Ok(made);
        };
        if !builder.is_done() {
            add_warning(fp, shell, " Closed Shell not mapped to FacetedBrep")?;
            return Ok(made);
        }
        made.result = faceted_brep_of(&builder)?;
        made.tessellated = builder.tessellated_value()?;
        made.root.done = true;
        Ok(made)
    }
    /// `TopoDSToStepMakeFacetedBrep(aSolid, FP, theLocalFactors, theProgress)`: a solid containing
    /// only one closed shell.
    ///
    /// # Errors
    /// The tessellated solid is made when the builder has no tessellated value (`if
    /// (!StepB.TessellatedValue())`), and dereferences it: that null dereference is kept as
    /// a null object failure, so this constructor fails on every solid written without
    /// tessellation.
    pub fn with_solid(
        solid: &Solid,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<Self, Failure> {
        let mut made = Self::default();
        // Looking for the Outer Shell
        let outer = BRepClass3d::outer_shell(solid)?;
        if outer.is_null() {
            add_warning(
                fp,
                &outer,
                " Solid contains no Outer Shell to be mapped to FacetedBrep",
            )?;
            return Ok(made);
        }
        if !outer.closed()? {
            add_warning(fp, &outer, " Shell not closed; not mapped to FacetedBrep")?;
            return Ok(made);
        }
        let Some(builder) = faceted_builder(&outer, fp, factors, progress)? else {
            return Ok(made);
        };
        if !builder.is_done() {
            add_warning(
                fp,
                &outer,
                " Closed Outer Shell from Solid not mapped to FacetedBrep",
            )?;
            return Ok(made);
        }
        made.result = faceted_brep_of(&builder)?;
        made.root.done = true;
        let item = builder.tessellated_value()?;
        if item.is_null() {
            made.tessellated = tessellated_solid(&[item], &made.result.cast())?;
        }
        Ok(made)
    }
}
