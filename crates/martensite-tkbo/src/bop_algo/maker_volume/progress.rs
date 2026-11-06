//! The concrete volume program retains its conditional images and actual solid-building child.
use super::BOPAlgoMakerVolume;
use crate::bop_algo::{
    BOPAlgoPaveFiller, BuilderHooks, PISteps, ProgressCancellation, ProgressCompletion,
    builder::{
        operation_progress::{BuildMode, combine, finish, run_image, stopped},
        perform::{adopt_filler, no_structure},
        ranged_driver::BuilderProgressProfile,
        work_estimates::{self, HISTORY_STEP, POST_TREATMENT_STEP, VOLUME_SOLIDS_STEP},
    },
};
use crate::bop_ds::DataStructure;
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::Failure,
};
use martensite_tkg3d::top_abs::ShapeEnum;

/// Run the volume receiver's own stages; `run_volume(maker, filler, range)?`.
pub(super) fn run_volume(
    maker: &mut BOPAlgoMakerVolume,
    filler: &mut BOPAlgoPaveFiller,
    range: &ProgressRange<'_>,
) -> Result<ProgressCompletion, Failure> {
    let scope = ProgressScope::with_literal(range, "Building volumes", 100., false);
    adopt_filler(&mut maker.builder, filler);
    let observed = ProgressCancellation::default();
    maker.check_data(filler);
    if maker.builder.options().has_errors() {
        return Ok(finish(maker, ProgressCompletion::Completed, &observed));
    }
    maker.prepare()?;
    let structure = filler.data_structure_mut().ok_or_else(no_structure)?;
    run_prepared_volume(maker, structure, &scope, &observed)
}
fn run_prepared_volume(
    maker: &mut BOPAlgoMakerVolume,
    structure: &mut DataStructure,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
) -> Result<ProgressCompletion, Failure> {
    let steps = volume_work_steps(maker, structure)?;
    let images = run_volume_images(maker, structure, scope, &steps, observed)?;
    if stopped(maker, &images) {
        return Ok(images);
    }
    let child =
        maker.build_volumes_progress(structure, &scope.next(steps.get_step(VOLUME_SOLIDS_STEP)))?;
    let completion = combine(images, child);
    if stopped(maker, &completion) {
        return Ok(completion);
    }
    finish_volume_history(maker, structure, scope, &steps, observed, completion)
}
fn volume_work_steps(
    maker: &BOPAlgoMakerVolume,
    structure: &DataStructure,
) -> Result<PISteps, Failure> {
    work_estimates::builder_work_estimates(
        structure,
        BuilderProgressProfile::MakerVolume {
            intersect: maker.intersect,
        },
        maker.builder.has_history(),
    )
}
const VOLUME_IMAGE_KINDS: [ShapeEnum; 4] = [
    ShapeEnum::Vertex,
    ShapeEnum::Edge,
    ShapeEnum::Wire,
    ShapeEnum::Face,
];
fn run_volume_images(
    maker: &mut BOPAlgoMakerVolume,
    structure: &mut DataStructure,
    scope: &ProgressScope<'_>,
    steps: &PISteps,
    observed: &ProgressCancellation,
) -> Result<ProgressCompletion, Failure> {
    let mut completion = ProgressCompletion::Completed;
    if !maker.intersect {
        return Ok(completion);
    }
    for (index, kind) in VOLUME_IMAGE_KINDS.into_iter().enumerate() {
        let range = scope.next(steps.get_step(index as i32));
        let child = run_image(maker, structure, kind, &range, observed, BuildMode::Ranged)?;
        if matches!(child, ProgressCompletion::Cancelled { .. }) {
            observed.record_cancelled();
        }
        completion = combine(completion, child);
        if stopped(maker, &completion) {
            return Ok(completion);
        }
    }
    Ok(completion)
}
fn finish_volume_history(
    maker: &mut BOPAlgoMakerVolume,
    structure: &DataStructure,
    scope: &ProgressScope<'_>,
    steps: &PISteps,
    observed: &ProgressCancellation,
    completion: ProgressCompletion,
) -> Result<ProgressCompletion, Failure> {
    maker.builder.prepare_history_progress_raw(
        structure,
        &scope.next(steps.get_step(HISTORY_STEP)),
        observed,
    )?;
    if maker.builder.options().has_errors() {
        return Ok(finish(maker, completion, observed));
    }
    maker
        .builder
        .post_treat_progress_raw(structure, &scope.next(steps.get_step(POST_TREATMENT_STEP)))?;
    Ok(finish(maker, completion, observed))
}

#[cfg(test)]
#[path = "progress_contract.rs"]
mod tests;

#[cfg(test)]
#[path = "progress_fixture.rs"]
mod fixture;

#[cfg(test)]
#[path = "progress_census.rs"]
mod census;
