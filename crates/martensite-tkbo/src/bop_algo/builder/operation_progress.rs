//! One General Fuse build program lends actual work weights through every image/result stage.
use super::{BuilderHooks, perform, ranged_driver::BuilderProgressProfile, work_estimates};
use crate::{
    bop_algo::{
        BOPAlgoPaveFiller, ProgressCancellation, ProgressCompletion, ProgressOperation,
        builder_area::progress::{classify_area_progress, legacy_unbound_area_result},
    },
    bop_ds::DataStructure,
};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::Failure,
};
use martensite_tkg3d::top_abs::ShapeEnum;
#[derive(Clone, Copy)]
pub(crate) enum BuildMode {
    Unbound,
    Ranged,
}
const IMAGE_KINDS: [ShapeEnum; 8] = [
    ShapeEnum::Vertex,
    ShapeEnum::Edge,
    ShapeEnum::Wire,
    ShapeEnum::Face,
    ShapeEnum::Shell,
    ShapeEnum::Solid,
    ShapeEnum::CompSolid,
    ShapeEnum::Compound,
];
/// Preserve actual check/prepare/result hooks; `run_general_fuse(hooks, filler, range, mode)?`.
pub(crate) fn run_general_fuse<H: BuilderHooks>(
    hooks: &mut H,
    filler: &mut BOPAlgoPaveFiller,
    range: &ProgressRange<'_>,
    mode: BuildMode,
) -> Result<ProgressCompletion, Failure> {
    perform::adopt_filler(hooks.general_fuse(), filler);
    let scope = ProgressScope::with_literal(
        range,
        "Building the result of General Fuse operation",
        100.,
        false,
    );
    let observed = ProgressCancellation::default();
    if !prepare_checked_receiver(hooks, filler)? {
        return Ok(finish(hooks, ProgressCompletion::Completed, &observed));
    }
    build_prepared(hooks, filler, &scope, &observed, mode)
}
/// Prepare the actual receiver only after its own data check; `prepare_checked_receiver(h, filler)?`.
pub(crate) fn prepare_checked_receiver<H: BuilderHooks>(
    hooks: &mut H,
    filler: &BOPAlgoPaveFiller,
) -> Result<bool, Failure> {
    hooks.check_data(filler);
    if hooks.general_fuse().options().has_errors() {
        return Ok(false);
    }
    hooks.prepare()?;
    Ok(!hooks.general_fuse().options().has_errors())
}
fn build_prepared<H: BuilderHooks>(
    hooks: &mut H,
    filler: &mut BOPAlgoPaveFiller,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
    mode: BuildMode,
) -> Result<ProgressCompletion, Failure> {
    let structure = filler
        .data_structure_mut()
        .ok_or_else(perform::no_structure)?;
    let steps = work_estimates::builder_work_estimates(
        structure,
        BuilderProgressProfile::GeneralFuse,
        hooks.general_fuse().has_history(),
    )?;
    let completion = run_images(hooks, structure, scope, &steps, observed, mode)?;
    if stopped(hooks, &completion) {
        return Ok(completion);
    }
    finish_build(hooks, structure, scope, &steps, observed, completion, mode)
}
fn finish_build<H: BuilderHooks>(
    hooks: &mut H,
    structure: &DataStructure,
    scope: &ProgressScope<'_>,
    steps: &crate::bop_algo::PISteps,
    observed: &ProgressCancellation,
    completion: ProgressCompletion,
    mode: BuildMode,
) -> Result<ProgressCompletion, Failure> {
    prepare_selected_history(
        hooks.general_fuse(),
        structure,
        &scope.next(steps.get_step(work_estimates::HISTORY_STEP)),
        observed,
        mode,
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
fn prepare_selected_history(
    builder: &mut super::BOPAlgoBuilder,
    structure: &DataStructure,
    range: &ProgressRange<'_>,
    observed: &ProgressCancellation,
    mode: BuildMode,
) -> Result<(), Failure> {
    // Keep legacy exception-side diagnostic ordering selectable through its original entry.
    match mode {
        BuildMode::Unbound => builder.prepare_history(structure),
        BuildMode::Ranged => builder.prepare_history_progress_raw(structure, range, observed),
    }
}
/// Reuse image choreography for existing derived unbound programs.
/// Example: `run_unbound_images(hooks, structure)?` preserves their BuildResult receiver.
pub(crate) fn run_unbound_images<H: BuilderHooks>(
    hooks: &mut H,
    structure: &mut DataStructure,
) -> Result<(), Failure> {
    let range = ProgressRange::default();
    let scope = ProgressScope::unnamed(&range, 100., false);
    let steps = work_estimates::builder_work_estimates(
        structure,
        BuilderProgressProfile::GeneralFuse,
        hooks.general_fuse().has_history(),
    )?;
    let completion = run_images(
        hooks,
        structure,
        &scope,
        &steps,
        &ProgressCancellation::default(),
        BuildMode::Unbound,
    )?;
    legacy_unbound_area_result(completion)
}
/// Run shared image/result choreography with the actual receiver weights; `run_images(h, ds, scope, steps, observed, mode)?`.
pub(crate) fn run_images<H: BuilderHooks>(
    hooks: &mut H,
    structure: &mut DataStructure,
    scope: &ProgressScope<'_>,
    steps: &crate::bop_algo::PISteps,
    observed: &ProgressCancellation,
    mode: BuildMode,
) -> Result<ProgressCompletion, Failure> {
    let mut completion = ProgressCompletion::Completed;
    for (index, kind) in IMAGE_KINDS.into_iter().enumerate() {
        let child = run_image(
            hooks,
            structure,
            kind,
            &scope.next(steps.get_step(index as i32)),
            observed,
            mode,
        )?;
        if matches!(child, ProgressCompletion::Cancelled { .. }) {
            observed.record_cancelled();
        }
        completion = combine(completion, child);
        if stopped(hooks, &completion) {
            return Ok(finish(hooks, completion, observed));
        }
        hooks.build_result(kind, structure)?;
        if hooks.general_fuse().options().has_errors() {
            return Ok(finish(hooks, completion, observed));
        }
    }
    Ok(completion)
}
/// Lend a range to an actual image stage; `run_image(h, ds, kind, range, observed, mode)?`.
pub(crate) fn run_image<H: BuilderHooks>(
    hooks: &mut H,
    structure: &mut DataStructure,
    kind: ShapeEnum,
    range: &ProgressRange<'_>,
    observed: &ProgressCancellation,
    mode: BuildMode,
) -> Result<ProgressCompletion, Failure> {
    if matches!(mode, BuildMode::Unbound) {
        return run_unbound_image(hooks, structure, kind);
    }
    if kind == ShapeEnum::Solid {
        return super::solids::fill_images_solids_with_progress(hooks, structure, range);
    }
    let builder = hooks.general_fuse();
    match kind {
        ShapeEnum::Vertex => builder.fill_vertices_progress_raw(structure, range, observed)?,
        ShapeEnum::Edge => builder.fill_edges_progress_raw(structure, range, observed)?,
        ShapeEnum::Wire | ShapeEnum::Shell | ShapeEnum::CompSolid => {
            builder.fill_containers_progress_raw(kind, structure, range, observed)?
        }
        ShapeEnum::Face => {
            return builder.fill_images_faces_progress_raw(structure, range, observed);
        }
        ShapeEnum::Compound => builder.fill_compounds_progress_raw(structure, range, observed)?,
        actual => {
            return Err(Failure::new(&format!(
                "image kind {actual:?}; expected one General Fuse stage"
            )));
        }
    }
    Ok(classify_area_progress(
        ProgressOperation::Builder,
        builder.options(),
        Ok(()),
        observed,
    ))
}
fn run_unbound_image<H: BuilderHooks>(
    hooks: &mut H,
    structure: &mut DataStructure,
    kind: ShapeEnum,
) -> Result<ProgressCompletion, Failure> {
    if kind == ShapeEnum::Solid {
        super::solids::fill_images_solids_with(hooks, structure)?;
        return Ok(ProgressCompletion::Completed);
    }
    let builder = hooks.general_fuse();
    match kind {
        ShapeEnum::Vertex => builder.fill_images_vertices(structure)?,
        ShapeEnum::Edge => builder.fill_images_edges(structure)?,
        ShapeEnum::Wire | ShapeEnum::Shell | ShapeEnum::CompSolid => {
            builder.fill_images_containers(kind, structure)?
        }
        ShapeEnum::Face => builder.fill_images_faces(structure)?,
        ShapeEnum::Compound => builder.fill_images_compounds(structure)?,
        actual => {
            return Err(Failure::new(&format!(
                "image kind {actual:?}; expected one General Fuse stage"
            )));
        }
    }
    Ok(ProgressCompletion::Completed)
}
/// Honor reports and typed failures at a native stage boundary; `stopped(h, completion)`.
pub(crate) fn stopped<H: BuilderHooks>(hooks: &mut H, completion: &ProgressCompletion) -> bool {
    hooks.general_fuse().options().has_errors()
        || matches!(
            completion,
            ProgressCompletion::Failed { .. } | ProgressCompletion::Incomplete { .. }
        )
}
/// Retain child noncompletion while classifying actual reports; `finish(h, completion, observed)`.
pub(crate) fn finish<H: BuilderHooks>(
    hooks: &mut H,
    completion: ProgressCompletion,
    observed: &ProgressCancellation,
) -> ProgressCompletion {
    if matches!(completion, ProgressCompletion::Completed) {
        return classify_area_progress(
            ProgressOperation::Builder,
            hooks.general_fuse().options(),
            Ok(()),
            observed,
        );
    }
    completion
}
/// Retain earlier cancellation unless a later failure supersedes it; `combine(first, next)`.
pub(crate) fn combine(first: ProgressCompletion, next: ProgressCompletion) -> ProgressCompletion {
    if matches!(first, ProgressCompletion::Completed)
        || matches!(
            next,
            ProgressCompletion::Failed { .. } | ProgressCompletion::Incomplete { .. }
        )
    {
        return next;
    }
    first
}
#[cfg(test)]
#[path = "operation_progress_contract.rs"]
mod tests;

#[cfg(test)]
#[path = "operation_receiver_contract.rs"]
mod receiver_tests;
