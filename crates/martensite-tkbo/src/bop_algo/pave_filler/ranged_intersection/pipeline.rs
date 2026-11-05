//! One weighted program supplies both base and actual derived initializers; nested callers reuse it.
use super::*;
use crate::bop_algo::pave_filler::weights::intersection_weights;
use martensite_tkernel::message::{ProgressRange, ProgressScope};
/// One stage of the pipeline; it stops at the first stage that reports an error.
type RangedStage = fn(
    &mut BOPAlgoPaveFiller,
    &ProgressRange<'_>,
    &ProgressCancellation,
) -> Result<Vec<ProgressCompletion>, Failure>;
const RANGED_STAGES: [RangedStage; 14] = [
    BOPAlgoPaveFiller::prepare_with_progress,
    perform_vv_slot,
    BOPAlgoPaveFiller::perform_ve_with_progress,
    BOPAlgoPaveFiller::perform_ee_with_progress,
    BOPAlgoPaveFiller::perform_vf_with_progress,
    BOPAlgoPaveFiller::perform_ef_with_progress,
    repeat_slot,
    BOPAlgoPaveFiller::force_interf_ee_with_progress,
    BOPAlgoPaveFiller::force_interf_ef_with_progress,
    BOPAlgoPaveFiller::perform_ff_with_progress,
    BOPAlgoPaveFiller::make_split_edges_with_progress,
    BOPAlgoPaveFiller::make_blocks_with_progress,
    BOPAlgoPaveFiller::make_pcurves_with_progress,
    degenerated_slot,
];
/// Lend the real initialization child and retain every child execution outcome.
/// Example: `run_intersection_program(filler, range, observed, BOPAlgoPaveFiller::initialize_with_progress)?`.
pub(super) fn run_intersection_program(
    filler: &mut BOPAlgoPaveFiller,
    range: &ProgressRange<'_>,
    observed: &ProgressCancellation,
    initialize: impl FnOnce(&mut BOPAlgoPaveFiller, &ProgressRange<'_>) -> Result<(), Failure>,
) -> Result<Vec<ProgressCompletion>, Failure> {
    let scope =
        ProgressScope::with_literal(range, "Performing intersection of shapes", 100., false);
    initialize(filler, &scope.next(5.))?;
    if filler.options().has_errors() {
        return Ok(Vec::new());
    }
    run_initialized_intersections(filler, &scope, observed)
}
fn run_initialized_intersections(
    filler: &mut BOPAlgoPaveFiller,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
) -> Result<Vec<ProgressCompletion>, Failure> {
    let weights = intersection_weights(filler)?;
    let mut outcomes = Vec::new();
    for (slot, stage) in RANGED_STAGES.into_iter().enumerate() {
        outcomes.extend(stage(
            filler,
            &scope.next(weights.get_step(slot as i32)),
            observed,
        )?);
        if filler.options().has_errors() {
            return Ok(outcomes);
        }
        filler.finish_weighted_stage(slot)?;
    }
    Ok(outcomes)
}
fn perform_vv_slot(
    filler: &mut BOPAlgoPaveFiller,
    range: &ProgressRange<'_>,
    observed: &ProgressCancellation,
) -> Result<Vec<ProgressCompletion>, Failure> {
    filler.perform_vv_with_progress(range, observed)?;
    Ok(Vec::new())
}
fn repeat_slot(
    filler: &mut BOPAlgoPaveFiller,
    range: &ProgressRange<'_>,
    observed: &ProgressCancellation,
) -> Result<Vec<ProgressCompletion>, Failure> {
    filler
        .repeat_intersection_with_progress(range, observed)
        .map(|outcome| vec![outcome])
}
fn degenerated_slot(
    filler: &mut BOPAlgoPaveFiller,
    range: &ProgressRange<'_>,
    observed: &ProgressCancellation,
) -> Result<Vec<ProgressCompletion>, Failure> {
    filler.process_de_with_progress(range, observed)?;
    Ok(Vec::new())
}
