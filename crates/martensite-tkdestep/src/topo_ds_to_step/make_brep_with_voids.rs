//! `TopoDSToStepMakeBrepWithVoids`: a solid with more than one closed shell to a brep_with_voids.
use super::tessellated_items::tessellated_solid;
use super::{
    TopoDSToStep, TopoDSToStepBuilder, TopoDSToStepShapeMap, TopoDSToStepTool, step_model_of,
    topo_ds_to_step_brep_class,
};
use crate::geom_to_step::{empty_step_name, shared_array1};
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_shape::{
    StepShapeBrepWithVoids, StepShapeClosedShell, StepShapeOpenShell, StepShapeOrientedClosedShell,
};
use crate::step_visual::StepVisualTessellatedItem;
use martensite_tkbrep::topods::{self, Shape, ShapeIterator, Shell, Solid};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_class3d::BRepClass3d;
use martensite_tkxsbase::transfer::TransferFinderProcess;

topo_ds_to_step_brep_class!(
    /// Make brep with voids.
    TopoDSToStepMakeBrepWithVoids,
    "TopoDSToStepMakeBrepWithVoids",
    StepShapeBrepWithVoids
);

/// The shells among the direct children of a solid.
pub(super) fn shells_of(solid: &Solid) -> Result<Vec<Shape>, Failure> {
    let mut shells = Vec::new();
    let mut children = ShapeIterator::new(solid, true, true);
    while children.more() {
        let child = children.value()?;
        if child.shape_type()? == ShapeEnum::Shell {
            shells.push(child.clone());
        }
        children.next();
    }
    Ok(shells)
}

/// The closed shell of a builder's result: si OPEN on le force a CLOSED mais que c est une honte !
fn closed_shell_of(builder: &TopoDSToStepBuilder) -> Result<Handle<StepShapeClosedShell>, Failure> {
    let value = builder.value()?;
    let closed: Handle<StepShapeClosedShell> = value.cast();
    if closed.is_some() {
        return Ok(closed);
    }
    let open: Handle<StepShapeOpenShell> = value.cast();
    let Some(live) = open.get() else {
        return Ok(Handle::default());
    };
    let forced = Handle::new(StepShapeClosedShell::new());
    forced.init(&live.name(), &live.cfs_faces()?);
    Ok(forced)
}

/// What the shells of the solid gave.
#[derive(Default)]
struct ShellResults {
    outer: Handle<StepShapeClosedShell>,
    voids: Vec<Handle<StepShapeClosedShell>>,
    tessellated: Vec<Handle<StepVisualTessellatedItem>>,
}

/// BRepClass3d::outer_shell requires geometry when the solid has voids: a null shell when it
/// raises.
fn outer_shell_or_null(solid: &Solid) -> Shell {
    BRepClass3d::outer_shell(solid).unwrap_or_default()
}

impl TopoDSToStepMakeBrepWithVoids {
    /// `TopoDSToStepMakeBrepWithVoids(aSolid, FP, theLocalFactors, theProgress)`.
    pub fn new(
        solid: &Solid,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<Self, Failure> {
        let mut made = Self::default();
        let outer_shell = outer_shell_or_null(solid);
        let shells = shells_of(solid)?;
        let scope = ProgressScope::unnamed(progress, shells.len() as f64, false);
        let results = translate_shells(solid, &shells, &outer_shell, fp, factors, &scope)?;
        let Some(results) = results else {
            return Ok(made);
        };
        if !results.voids.is_empty() {
            made.result = brep_with_voids(&results.outer, &results.voids)?;
            made.root.done = true;
        }
        if !results.tessellated.is_empty() {
            made.tessellated = tessellated_solid(&results.tessellated, &made.result.cast())?;
            made.root.done = true;
        }
        Ok(made)
    }
}

/// Each shell through the builder; `None` when the progress stopped first.
fn translate_shells(
    solid: &Solid,
    shells: &[Shape],
    outer_shell: &Shell,
    fp: &Handle<TransferFinderProcess>,
    factors: &StepDataFactors,
    scope: &ProgressScope<'_>,
) -> Result<Option<ShellResults>, Failure> {
    let model = step_model_of(fp)?;
    let (mode, tessellated) = {
        let parameters = model.internal_parameters();
        (
            parameters.write_surface_cur_mode,
            parameters.write_tessellated.as_raw(),
        )
    };
    let _ = solid;
    let map = TopoDSToStepShapeMap::default();
    let mut tool = TopoDSToStepTool::with_model(&model)?;
    let mut builder = TopoDSToStepBuilder::new();
    let mut results = ShellResults::default();
    for child in shells {
        if !scope.more() {
            break;
        }
        let mut current = topods::shell(child)?;
        // :e0 abv 25 Mar 98: voids should be reversed according to EXPRESS for ABSR
        let is_outer = outer_shell.is_equal(&current);
        if !outer_shell.is_null() && !is_outer {
            current.reverse();
        }
        // :d7 abv 16 Mar 98: try to treat 'open' shells as closed since flag
        // IsClosed() is often incorrect (taken from MakeManifoldSolid(Solid))
        tool.init(&map, false, mode);
        builder.init(
            &current,
            &mut tool,
            fp,
            tessellated,
            factors,
            &scope.next(1.0),
        )?;
        TopoDSToStep::add_tool_results(fp, &tool)?;
        if !builder.is_done() {
            continue;
        }
        let closed = closed_shell_of(&builder)?;
        let item = builder.tessellated_value()?;
        if item.is_some() {
            results.tessellated.push(item);
        }
        if !outer_shell.is_null() && closed.is_some() {
            if is_outer {
                results.outer = closed;
            } else {
                results.voids.push(closed);
            }
        }
    }
    Ok(scope.more().then_some(results))
}

/// The brep over the outer shell and the voids, oriented false (:e0).
fn brep_with_voids(
    outer: &Handle<StepShapeClosedShell>,
    voids: &[Handle<StepShapeClosedShell>],
) -> Result<Handle<StepShapeBrepWithVoids>, Failure> {
    let name = empty_step_name();
    let mut oriented = Vec::with_capacity(voids.len());
    for void in voids {
        step_live(void, "TopoDSToStepMakeBrepWithVoids")?;
        let shell = Handle::new(StepShapeOrientedClosedShell::new());
        // Warning: the Oriented Shell Orientation is not always TRUE.
        //           Shall check the Shell orientation.
        // => if the Shell is reversed, shall create an OrientedShell.
        shell.init(&name, void, false);
        oriented.push(shell);
    }
    let brep = Handle::new(StepShapeBrepWithVoids::new());
    brep.init(&name, outer, &shared_array1(oriented));
    Ok(brep)
}
