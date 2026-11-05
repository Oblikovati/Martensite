//! Preserve virtual classification while sharing the native 4/5/1 solid choreography.
use super::*;
use crate::bop_algo::{
    ProgressCancellation, ProgressCompletion, ProgressOperation,
    builder_area::progress::classify_area_progress,
};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
#[derive(Clone, Copy)]
pub(super) enum SolidDispatchMode {
    Unbound,
    Ranged,
}
fn retain_child(
    completion: ProgressCompletion,
    mode: SolidDispatchMode,
) -> Result<ProgressCompletion, Failure> {
    if matches!(mode, SolidDispatchMode::Unbound) {
        crate::bop_algo::builder_area::progress::legacy_unbound_area_result(completion)?;
        return Ok(ProgressCompletion::Completed);
    }
    Ok(completion)
}
/// Lend each real child its native range; `fill_solids_using(hooks, ds, range, classifier)?`.
pub(super) fn fill_solids_using<H: BuilderHooks>(
    hooks: &mut H,
    structure: &mut DataStructure,
    range: &ProgressRange<'_>,
    mode: SolidDispatchMode,
    classify: impl FnOnce(
        &mut H,
        &mut DataStructure,
        &mut SameDomainShapes,
        &ProgressRange<'_>,
    ) -> Result<ProgressCompletion, Failure>,
) -> Result<ProgressCompletion, Failure> {
    if !has_source_solid(structure)? {
        return Ok(classify_area_progress(
            ProgressOperation::Builder,
            hooks.general_fuse().options(),
            Ok(()),
            &ProgressCancellation::default(),
        ));
    }
    let scope = ProgressScope::with_literal(range, "Building splits of solids", 10., false);
    let observed = ProgressCancellation::default();
    let mut drafts = DataMap::with_hasher(ShapeMapHasher, 1);
    let mut completion = retain_child(
        classify(hooks, structure, &mut drafts, &scope.next(4.))?,
        mode,
    )?;
    if must_stop(hooks, &completion) {
        return Ok(final_outcome(hooks, completion, &observed));
    }
    if matches!(completion, ProgressCompletion::Cancelled { .. }) {
        observed.record_cancelled();
    }
    let split = hooks.general_fuse().build_split_solids_progress_raw(
        structure,
        &drafts,
        &scope.next(5.),
        &observed,
    )?;
    completion = combine(completion, retain_child(split, mode)?);
    if must_stop(hooks, &completion) {
        return Ok(final_outcome(hooks, completion, &observed));
    }
    let internal = hooks.general_fuse().fill_internal_shapes_progress_raw(
        structure,
        &scope.next(1.),
        &observed,
    )?;
    completion = combine(completion, retain_child(internal, mode)?);
    if matches!(completion, ProgressCompletion::Completed) {
        return Ok(classify_area_progress(
            ProgressOperation::Builder,
            hooks.general_fuse().options(),
            Ok(()),
            &observed,
        ));
    }
    Ok(completion)
}
fn has_source_solid(structure: &DataStructure) -> Result<bool, Failure> {
    for index in 0..structure.nb_source_shapes() {
        if structure.shape_info(index)?.shape_type() == ShapeEnum::Solid {
            return Ok(true);
        }
    }
    Ok(false)
}
fn must_stop<H: BuilderHooks>(hooks: &mut H, completion: &ProgressCompletion) -> bool {
    hooks.general_fuse().options().has_errors()
        || matches!(
            completion,
            ProgressCompletion::Failed { .. } | ProgressCompletion::Incomplete { .. }
        )
}
fn combine(first: ProgressCompletion, next: ProgressCompletion) -> ProgressCompletion {
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
#[path = "dispatch_contract.rs"]
mod tests;

fn final_outcome<H: BuilderHooks>(
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
