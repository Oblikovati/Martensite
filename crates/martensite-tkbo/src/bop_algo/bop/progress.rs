//! The actual Boolean receiver owns empty-shape shortcuts, image results, shape building and history.
use super::*;
use crate::bop_algo::{
    ProgressCancellation, ProgressCompletion,
    builder::{
        operation_progress::{BuildMode, finish, run_images, stopped},
        ranged_driver::BuilderProgressProfile,
        work_estimates,
    },
};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
/// Run the concrete Boolean build program; `run_boolean_progress(hooks, filler, range)?`.
pub(crate) fn run_boolean_progress<H: BopHooks>(
    hooks: &mut H,
    filler: &mut BOPAlgoPaveFiller,
    range: &ProgressRange<'_>,
) -> Result<ProgressCompletion, Failure> {
    let observed = ProgressCancellation::default();
    adopt_filler(hooks.general_fuse(), filler);
    hooks.boolean().check_operation_data(filler)?;
    if hooks.general_fuse().options().has_errors() {
        return Ok(finish(hooks, ProgressCompletion::Completed, &observed));
    }
    hooks.prepare()?;
    if hooks.general_fuse().options().has_errors() {
        return Ok(finish(hooks, ProgressCompletion::Completed, &observed));
    }
    let structure = filler.data_structure_mut().ok_or_else(no_structure)?;
    if complete_empty_boolean(hooks, structure, range, &observed)? {
        return Ok(finish(hooks, ProgressCompletion::Completed, &observed));
    }
    run_prepared_boolean(hooks, structure, range, &observed)
}
fn complete_empty_boolean<H: BopHooks>(
    hooks: &mut H,
    structure: &DataStructure,
    range: &ProgressRange<'_>,
    observed: &ProgressCancellation,
) -> Result<bool, Failure> {
    if !(hooks.boolean().has_empty_shape_warning() && hooks.boolean().treat_empty_shape()?) {
        return Ok(false);
    }
    hooks
        .general_fuse()
        .prepare_history_progress_raw(structure, range, observed)?;
    Ok(true)
}
fn run_prepared_boolean<H: BopHooks>(
    hooks: &mut H,
    structure: &mut DataStructure,
    range: &ProgressRange<'_>,
    observed: &ProgressCancellation,
) -> Result<ProgressCompletion, Failure> {
    let scope = ProgressScope::with_literal(
        range,
        "Building the result of Boolean operation",
        100.,
        false,
    );
    let profile = BuilderProgressProfile::Boolean {
        operation: hooks.boolean().operation,
    };
    let steps = work_estimates::builder_work_estimates(
        structure,
        profile,
        hooks.general_fuse().has_history(),
    )?;
    let completion = run_images(
        hooks,
        structure,
        &scope,
        &steps,
        observed,
        BuildMode::Ranged,
    )?;
    if stopped(hooks, &completion) {
        return Ok(completion);
    }
    super::shape_progress::build_boolean_shape_progress(
        hooks,
        structure,
        &scope.next(steps.get_step(work_estimates::BOOLEAN_RESULT_STEP)),
        observed,
    )?;
    if hooks.general_fuse().options().has_errors() {
        return Ok(finish(hooks, completion, observed));
    }
    finish_boolean_history(hooks, structure, &scope, &steps, observed, completion)
}
fn finish_boolean_history<H: BopHooks>(
    hooks: &mut H,
    structure: &DataStructure,
    scope: &ProgressScope<'_>,
    steps: &crate::bop_algo::PISteps,
    observed: &ProgressCancellation,
    completion: ProgressCompletion,
) -> Result<ProgressCompletion, Failure> {
    hooks.general_fuse().prepare_history_progress_raw(
        structure,
        &scope.next(steps.get_step(work_estimates::HISTORY_STEP)),
        observed,
    )?;
    if hooks.general_fuse().options().has_errors() {
        return Ok(finish(hooks, completion, observed));
    }
    hooks.general_fuse().post_treat_progress_raw(
        structure,
        &scope.next(steps.get_step(work_estimates::POST_TREATMENT_STEP)),
    )?;
    Ok(finish(hooks, completion, observed))
}

#[cfg(test)]
#[path = "progress_contract.rs"]
mod tests;

#[cfg(test)]
#[path = "progress_census.rs"]
mod census;
#[cfg(test)]
#[path = "progress_fixture.rs"]
mod fixture;
