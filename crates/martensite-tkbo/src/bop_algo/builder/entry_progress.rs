//! Whole-operation entry keeps the real receiver while retaining lower execution diagnostics.
#[cfg(test)]
use super::BOPAlgoBuilder;
use super::{
    BuilderHooks,
    ranged_driver::{BuilderProgressProfile, perform_on_progress},
};
use crate::bop_algo::{
    BOPAlgoPaveFiller, ProgressCompletion, progress_result::retain_intersection_completion,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{message::ProgressRange, ncollection::List, standard::Failure};
/// Run the actual General Fuse receiver; `perform_general_fuse_on_progress(builder, shapes, range)?`.
pub(crate) fn perform_general_fuse_on_progress<H: BuilderHooks>(
    hooks: &mut H,
    shapes: List<Shape>,
    range: &ProgressRange<'_>,
) -> Result<ProgressCompletion, Failure> {
    perform_profile_on_progress(hooks, shapes, range, BuilderProgressProfile::GeneralFuse)
}
/// Retain Splitter's own outer scope and input assembly; `perform_splitter_on_progress(splitter, shapes, range)?`.
pub(crate) fn perform_splitter_on_progress<H: BuilderHooks>(
    hooks: &mut H,
    shapes: List<Shape>,
    range: &ProgressRange<'_>,
) -> Result<ProgressCompletion, Failure> {
    perform_profile_on_progress(hooks, shapes, range, BuilderProgressProfile::Splitter)
}
/// Run a qualified receiver with its fixed native profile; `perform_profile_on_progress(h, shapes, range, profile)?`.
pub(crate) fn perform_profile_on_progress<H: BuilderHooks>(
    hooks: &mut H,
    shapes: List<Shape>,
    range: &ProgressRange<'_>,
    profile: BuilderProgressProfile,
) -> Result<ProgressCompletion, Failure> {
    perform_on_progress(
        hooks,
        shapes,
        range,
        profile,
        BOPAlgoPaveFiller::perform_on_progress,
        build_after_intersection::<H>,
    )
}
fn build_after_intersection<H: BuilderHooks>(
    hooks: &mut H,
    filler: &mut BOPAlgoPaveFiller,
    intersection: ProgressCompletion,
    range: &ProgressRange<'_>,
) -> Result<ProgressCompletion, Failure> {
    // Native PerformInternal always runs: even a failed filler must reach the actual CheckData.
    let built = hooks.perform_internal1_with_progress(filler, range)?;
    Ok(retain_intersection_completion(intersection, built))
}
#[cfg(test)]
#[path = "entry_progress/contract.rs"]
mod contract;

#[cfg(test)]
#[path = "entry_progress/fixture.rs"]
mod fixture;
#[cfg(test)]
#[path = "entry_progress/snapshot.rs"]
pub(crate) mod snapshot;

#[cfg(test)]
#[path = "entry_progress/census.rs"]
mod census;
