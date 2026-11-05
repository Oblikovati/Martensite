//! The actual Section receiver retains both image stages, report clearing and all three polling loops.
use super::*;
use crate::bop_algo::{
    ProgressCancellation, ProgressCompletion,
    builder::{
        operation_progress::{
            BuildMode, combine, finish, prepare_checked_receiver, run_image, stopped,
        },
        ranged_driver::BuilderProgressProfile,
        work_estimates,
    },
    builder_area::progress::observe_user_break,
};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
/// Run the actual Section build program; `run_section_progress(section, filler, range)?`.
pub(super) fn run_section_progress(
    section: &mut BOPAlgoSection,
    filler: &mut BOPAlgoPaveFiller,
    range: &ProgressRange<'_>,
) -> Result<ProgressCompletion, Failure> {
    let scope =
        ProgressScope::with_literal(range, "Building result of SECTION operation", 100., false);
    section.builder.context = filler.context_handle();
    let observed = ProgressCancellation::default();
    if !prepare_checked_receiver(section, filler)? {
        return Ok(finish(section, ProgressCompletion::Completed, &observed));
    }
    let structure = filler.data_structure_mut().ok_or_else(no_structure)?;
    run_prepared_section(section, structure, &scope, &observed)
}
fn run_prepared_section(
    section: &mut BOPAlgoSection,
    structure: &mut DataStructure,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
) -> Result<ProgressCompletion, Failure> {
    let steps = work_estimates::builder_work_estimates(
        structure,
        BuilderProgressProfile::Section,
        section.builder.has_history(),
    )?;
    let completion = run_section_images(section, structure, scope, &steps, observed)?;
    if stopped(section, &completion) {
        return Ok(completion);
    }
    build_section_progress(
        section,
        structure,
        &scope.next(steps.get_step(work_estimates::SECTION_RESULT_STEP)),
        observed,
    )?;
    if section.builder.options().has_errors() {
        return Ok(finish(section, completion, observed));
    }
    finish_section_history(section, structure, scope, &steps, observed, completion)
}
fn run_section_images(
    section: &mut BOPAlgoSection,
    structure: &mut DataStructure,
    scope: &ProgressScope<'_>,
    steps: &crate::bop_algo::PISteps,
    observed: &ProgressCancellation,
) -> Result<ProgressCompletion, Failure> {
    let mut completion = ProgressCompletion::Completed;
    for (index, kind) in [ShapeEnum::Vertex, ShapeEnum::Edge].into_iter().enumerate() {
        let child = run_image(
            section,
            structure,
            kind,
            &scope.next(steps.get_step(index as i32)),
            observed,
            BuildMode::Ranged,
        )?;
        if matches!(child, ProgressCompletion::Cancelled { .. }) {
            observed.record_cancelled();
        }
        completion = combine(completion, child);
        if stopped(section, &completion) {
            return Ok(finish(section, completion, observed));
        }
        section.build_result(kind, structure)?;
        if section.builder.options().has_errors() {
            return Ok(finish(section, completion, observed));
        }
    }
    Ok(completion)
}
fn build_section_progress(
    section: &mut BOPAlgoSection,
    structure: &DataStructure,
    range: &ProgressRange<'_>,
    observed: &ProgressCancellation,
) -> Result<(), Failure> {
    let scope =
        ProgressScope::with_literal(range, "Building the result of Section operation", 1., false);
    section.build_section_checked(structure, |actual| {
        observe_user_break(actual.builder.options(), &scope, observed)
    })
}
fn finish_section_history(
    section: &mut BOPAlgoSection,
    structure: &DataStructure,
    scope: &ProgressScope<'_>,
    steps: &crate::bop_algo::PISteps,
    observed: &ProgressCancellation,
    completion: ProgressCompletion,
) -> Result<ProgressCompletion, Failure> {
    section.builder.prepare_history_progress_raw(
        structure,
        &scope.next(steps.get_step(work_estimates::SECTION_HISTORY_STEP)),
        observed,
    )?;
    if section.builder.options().has_errors() {
        return Ok(finish(section, completion, observed));
    }
    section.builder.post_treat_progress_raw(
        structure,
        &scope.next(steps.get_step(work_estimates::SECTION_POST_TREATMENT_STEP)),
    )?;
    Ok(finish(section, completion, observed))
}

#[cfg(test)]
#[path = "progress_fixture.rs"]
mod fixture;
#[cfg(test)]
#[path = "progress_contract.rs"]
mod tests;

#[cfg(test)]
#[path = "progress_census.rs"]
mod census;
