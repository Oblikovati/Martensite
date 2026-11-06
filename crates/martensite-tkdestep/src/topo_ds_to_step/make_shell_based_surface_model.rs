//! `TopoDSToStepMakeShellBasedSurfaceModel`: a face, a shell or a solid to a
//! shell_based_surface_model.
use super::make_brep_with_voids::shells_of;
use super::{
    TopoDSToStep, TopoDSToStepBuilder, TopoDSToStepShapeMap, TopoDSToStepTool, add_warning,
    step_model_of, topo_ds_to_step_brep_class,
};
use crate::de_step::DeStepWriteModeStepSchema;
use crate::geom_to_step::{empty_step_name, shared_array1};
use crate::step_data::{StepDataFactors, StepDataSelectType};
use crate::step_shape::{
    StepShapeClosedShell, StepShapeFace, StepShapeFaceSurface, StepShapeOpenShell, StepShapeShell,
    StepShapeShellBasedSurfaceModel, StepShapeTopologicalRepresentationItem,
};
use martensite_tkbrep::topods::{self, Face, Shape, Shell, Solid};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkxsbase::transfer::TransferFinderProcess;

topo_ds_to_step_brep_class!(
    /// Make shell based surface model.
    TopoDSToStepMakeShellBasedSurfaceModel,
    "TopoDSToStepMakeShellBasedSurfaceModel",
    StepShapeShellBasedSurfaceModel
);

/// The pcurve mode and the tessellation to write: tessellation only goes to AP242.
fn write_modes(
    fp: &Handle<TransferFinderProcess>,
    warned_shape: &Shape,
) -> Result<(bool, i32), Failure> {
    let model = step_model_of(fp)?;
    let parameters = model.internal_parameters();
    let mode = parameters.write_surface_cur_mode;
    let mut tessellated = parameters.write_tessellated.as_raw();
    if parameters.write_schema != DeStepWriteModeStepSchema::AP242_DIS {
        tessellated = 0;
        add_warning(
            fp,
            warned_shape,
            " Tessellation can not be exported into not AP242",
        )?;
    }
    Ok((mode, tessellated))
}

/// The select on an open shell, else on a closed one.
fn shell_select(
    item: &Handle<StepShapeTopologicalRepresentationItem>,
) -> Result<StepShapeShell, Failure> {
    let mut select = StepShapeShell::default();
    let open: Handle<StepShapeOpenShell> = item.cast();
    if open.is_some() {
        select.set_value(&open.cast())?;
    } else {
        let closed: Handle<StepShapeClosedShell> = item.cast();
        select.set_value(&closed.cast())?;
    }
    Ok(select)
}

/// The model over the given shells.
fn surface_model(boundary: Vec<StepShapeShell>) -> Handle<StepShapeShellBasedSurfaceModel> {
    let model = Handle::new(StepShapeShellBasedSurfaceModel::new());
    model.init(&empty_step_name(), &shared_array1(boundary));
    model
}

impl TopoDSToStepMakeShellBasedSurfaceModel {
    /// `TopoDSToStepMakeShellBasedSurfaceModel(aFace, FP, theLocalFactors, theProgress)`.
    pub fn with_face(
        face: &Face,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<Self, Failure> {
        let mut made = Self::default();
        let (mode, tessellated) = write_modes(fp, face)?;
        let mut tool = TopoDSToStepTool::with_map(&TopoDSToStepShapeMap::default(), false, mode);
        let builder =
            TopoDSToStepBuilder::with_shape(face, &mut tool, fp, tessellated, factors, progress)?;
        if progress.user_break() {
            return Ok(made);
        }
        TopoDSToStep::add_tool_results(fp, &tool)?;
        if !builder.is_done() {
            add_warning(
                fp,
                face,
                " Single Face not mapped to ShellBasedSurfaceModel",
            )?;
            return Ok(made);
        }
        let surface: Handle<StepShapeFaceSurface> = builder.value()?.cast();
        if surface.is_some() {
            let open = Handle::new(StepShapeOpenShell::new());
            open.init(
                &empty_step_name(),
                &shared_array1::<Handle<StepShapeFace>>(vec![surface.cast()]),
            );
            let mut select = StepShapeShell::default();
            select.set_value(&open.cast())?;
            made.result = surface_model(vec![select]);
        }
        made.tessellated = builder.tessellated_value()?;
        made.root.done = true;
        Ok(made)
    }
    /// `TopoDSToStepMakeShellBasedSurfaceModel(aShell, FP, theLocalFactors, theProgress)`.
    pub fn with_shell(
        shell: &Shell,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<Self, Failure> {
        let mut made = Self::default();
        let (mode, tessellated) = write_modes(fp, shell)?;
        let mut tool = TopoDSToStepTool::with_map(&TopoDSToStepShapeMap::default(), false, mode);
        let builder =
            TopoDSToStepBuilder::with_shape(shell, &mut tool, fp, tessellated, factors, progress)?;
        if progress.user_break() {
            return Ok(made);
        }
        if builder.is_done() {
            let value = builder.value()?;
            if value.is_some() {
                let mut select = StepShapeShell::default();
                if shell.closed()? {
                    let closed: Handle<StepShapeClosedShell> = value.cast();
                    select.set_value(&closed.cast())?;
                } else {
                    let open: Handle<StepShapeOpenShell> = value.cast();
                    select.set_value(&open.cast())?;
                }
                made.result = surface_model(vec![select]);
                TopoDSToStep::add_result(fp, shell, &made.result.cast())?;
            }
            made.tessellated = builder.tessellated_value()?;
            made.root.done = true;
        } else {
            add_warning(fp, shell, " Shell not mapped to ShellBasedSurfaceModel")?;
        }
        TopoDSToStep::add_tool_results(fp, &tool).map(|()| made)
    }
    /// `TopoDSToStepMakeShellBasedSurfaceModel(aSolid, FP, theLocalFactors, theProgress)`.
    /// The tessellated set of the shells is built and then dropped, so `tessellated_value`
    /// stays null here; the set is not built since nothing can see it.
    pub fn with_solid(
        solid: &Solid,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<Self, Failure> {
        let mut made = Self::default();
        // The warning names the null shell declared above the loop.
        let (mode, tessellated) = write_modes(fp, &Shape::default())?;
        let map = TopoDSToStepShapeMap::default();
        let shells = shells_of(solid)?;
        let scope = ProgressScope::unnamed(progress, shells.len() as f64, false);
        let mut boundary = Vec::new();
        for child in &shells {
            if !scope.more() {
                break;
            }
            let shell = topods::shell(child)?;
            let mut tool = TopoDSToStepTool::with_map(&map, false, mode);
            let builder = TopoDSToStepBuilder::with_shape(
                &shell,
                &mut tool,
                fp,
                tessellated,
                factors,
                &scope.next(1.0),
            )?;
            TopoDSToStep::add_tool_results(fp, &tool)?;
            if !builder.is_done() {
                add_warning(
                    fp,
                    &shell,
                    " Shell from Solid not mapped to ShellBasedSurfaceModel",
                )?;
                continue;
            }
            let value = builder.value()?;
            if value.is_some() {
                boundary.push(shell_select(&value)?);
            }
        }
        if !scope.more() {
            return Ok(made);
        }
        if boundary.is_empty() {
            add_warning(
                fp,
                solid,
                " Solid contains no Shell to be mapped to ShellBasedSurfaceModel",
            )?;
            return Ok(made);
        }
        made.result = surface_model(boundary);
        made.root.done = true;
        Ok(made)
    }
}
