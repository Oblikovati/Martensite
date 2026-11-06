//! `TopoDSToStepMakeManifoldSolidBrep`: a shell, or a solid with one closed shell, to a
//! manifold_solid_brep.
use super::tessellated_items::tessellated_solid;
use super::{
    TopoDSToStep, TopoDSToStepBuilder, TopoDSToStepShapeMap, TopoDSToStepTool, add_warning,
    step_model_of, topo_ds_to_step_brep_class,
};
use crate::geom_to_step::empty_step_name;
use crate::step_data::StepDataFactors;
use crate::step_shape::{StepShapeClosedShell, StepShapeManifoldSolidBrep, StepShapeOpenShell};
use crate::step_visual::StepVisualTessellatedItem;
use martensite_tkbrep::topods::{Shell, Solid};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tktopalgo::brep_class3d::BRepClass3d;
use martensite_tkxsbase::transfer::TransferFinderProcess;

topo_ds_to_step_brep_class!(
    /// Make manifold solid brep.
    TopoDSToStepMakeManifoldSolidBrep,
    "TopoDSToStepMakeManifoldSolidBrep",
    StepShapeManifoldSolidBrep
);

/// The brep and the tessellated item of a shell (static `make_manifold_solid_brep`).
fn make_manifold_solid_brep(
    shell: &Shell,
    fp: &Handle<TransferFinderProcess>,
    factors: &StepDataFactors,
    progress: &ProgressRange<'_>,
) -> Result<
    (
        Handle<StepShapeManifoldSolidBrep>,
        Handle<StepVisualTessellatedItem>,
    ),
    Failure,
> {
    let model = step_model_of(fp)?;
    let (mode, tessellated) = {
        let parameters = model.internal_parameters();
        (
            parameters.write_surface_cur_mode,
            parameters.write_tessellated.as_raw(),
        )
    };
    let mut tool = TopoDSToStepTool::with_map(&TopoDSToStepShapeMap::default(), false, mode);
    let builder =
        TopoDSToStepBuilder::with_shape(shell, &mut tool, fp, tessellated, factors, progress)?;
    let nothing = (Handle::default(), Handle::default());
    if progress.user_break() {
        return Ok(nothing);
    }
    TopoDSToStep::add_tool_results(fp, &tool)?;
    if !builder.is_done() {
        return Ok(nothing);
    }
    let item = builder.tessellated_value()?;
    let value = builder.value()?;
    let mut closed: Handle<StepShapeClosedShell> = value.cast();
    // si OPEN on le force a CLOSED mais que c est une honte !
    if closed.is_null() {
        let open: Handle<StepShapeOpenShell> = value.cast();
        let Some(live) = open.get() else {
            return Ok((Handle::default(), item));
        };
        closed = Handle::new(StepShapeClosedShell::new());
        closed.init(&live.name(), &live.cfs_faces()?);
    }
    let brep = Handle::new(StepShapeManifoldSolidBrep::new());
    brep.init(&empty_step_name(), &closed);
    Ok((brep, item))
}

impl TopoDSToStepMakeManifoldSolidBrep {
    /// `TopoDSToStepMakeManifoldSolidBrep(aShell, FP, theLocalFactors, theProgress)`.
    pub fn with_shell(
        shell: &Shell,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<Self, Failure> {
        let mut made = Self::default();
        let (brep, item) = make_manifold_solid_brep(shell, fp, factors, progress)?;
        made.root.done = brep.is_some() || item.is_some();
        if made.root.done {
            made.result = brep;
            made.tessellated = item;
        }
        if !made.root.done && !progress.user_break() {
            add_warning(fp, shell, " Closed Shell not mapped to ManifoldSolidBrep")?;
        }
        Ok(made)
    }
    /// `TopoDSToStepMakeManifoldSolidBrep(aSolid, FP, theLocalFactors, theProgress)`: a solid
    /// containing only one closed shell.
    pub fn with_solid(
        solid: &Solid,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<Self, Failure> {
        let mut made = Self::default();
        let outer = BRepClass3d::outer_shell(solid)?;
        if outer.is_null() {
            add_warning(
                fp,
                &outer,
                " Outer Shell is null; not mapped to ManifoldSolidBrep ",
            )?;
            return Ok(made);
        }
        let (brep, item) = make_manifold_solid_brep(&outer, fp, factors, progress)?;
        made.root.done = brep.is_some() || item.is_some();
        if made.root.done {
            made.result = brep;
            if item.is_some() {
                made.tessellated = tessellated_solid(&[item], &made.result)?;
            }
        }
        if !made.root.done && !progress.user_break() {
            add_warning(
                fp,
                &outer,
                " Outer Shell of Solid not mapped to ManifoldSolidBrep",
            )?;
        }
        Ok(made)
    }
}
