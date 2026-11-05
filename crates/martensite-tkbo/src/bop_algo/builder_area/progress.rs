//! Actual area receivers own each geometry stage; the base exposes no inherited Perform entry.
use super::BuilderArea;
use crate::bop_algo::{
    Options, ProgressCancellation, ProgressCompletion, ProgressFailureClass, ProgressOperation,
    ProgressStage,
};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::Failure,
};

/// Mandatory actual-H stages, with no base geometry fallback; `perform_area_stages(receiver, range)`.
/// This raw internal choreography does not classify operation completion or publish geometry.
pub(crate) trait BuilderAreaHooks {
    /// Read the actual receiver's report state; `receiver.area().options().has_errors()`.
    fn area(&self) -> &BuilderArea;

    /// Prepare the actual receiver's base state; `receiver.area_mut().options_mut().clear()`.
    fn area_mut(&mut self) -> &mut BuilderArea;

    /// Remove the actual receiver's free input entities; `receiver.perform_shapes_to_avoid(range)`.
    fn perform_shapes_to_avoid(&mut self, range: &ProgressRange<'_>) -> Result<(), Failure>;

    /// Build the actual receiver's boundary loops; `receiver.perform_loops(range)`.
    fn perform_loops(&mut self, range: &ProgressRange<'_>) -> Result<(), Failure>;

    /// Build the actual receiver's areas from its loops; `receiver.perform_areas(range)`.
    fn perform_areas(&mut self, range: &ProgressRange<'_>) -> Result<(), Failure>;

    /// Place the actual receiver's internal entities; `receiver.perform_internal_shapes(range)`.
    fn perform_internal_shapes(&mut self, range: &ProgressRange<'_>) -> Result<(), Failure>;
}

type AreaStage<H> = fn(&mut H, &ProgressRange<'_>) -> Result<(), Failure>;

/// Protocol fixture wrapper for the actual-H driver; `perform_area_stages(receiver, range)`.
/// Preparation and explicit ranged completion classification belong to the concrete operation.
#[cfg(test)]
pub(crate) fn perform_area_stages<H: BuilderAreaHooks>(
    receiver: &mut H,
    range: &ProgressRange<'_>,
) -> Result<(), Failure> {
    let scope = ProgressScope::unnamed(range, 100., false);
    perform_area_stages_in_scope(receiver, &scope)
}

/// Use the receiver's already-open native scope; `perform_area_stages_in_scope(receiver, scope)`.
/// Concrete preparation and early returns share this same hundred-unit scope.
pub(crate) fn perform_area_stages_in_scope<H: BuilderAreaHooks>(
    receiver: &mut H,
    scope: &ProgressScope<'_>,
) -> Result<(), Failure> {
    for (weight, stage) in area_stages::<H>() {
        stage(receiver, &scope.next(weight))?;
        if receiver.area().options().has_errors() {
            return Ok(());
        }
    }
    Ok(())
}

fn area_stages<H: BuilderAreaHooks>() -> [(f64, AreaStage<H>); 4] {
    [
        (1., H::perform_shapes_to_avoid),
        (10., H::perform_loops),
        (80., H::perform_areas),
        (9., H::perform_internal_shapes),
    ]
}

/// Retain a confirmed native poll without inventing reports; `observe_user_break(options, scope, observed)`.
pub(crate) fn observe_user_break(
    options: &Options,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
) -> bool {
    let cancelled = options.user_break(scope);
    if cancelled {
        observed.record_cancelled();
    }
    cancelled
}

/// Classify real raw completion without polling; `classify_area_progress(operation, options, result, observed)`.
/// Completion is the classification boundary, not inferred cancellation-location provenance.
pub(crate) fn classify_area_progress(
    operation: ProgressOperation,
    options: &Options,
    result: Result<(), Failure>,
    observed: &ProgressCancellation,
) -> ProgressCompletion {
    match (result, observed.is_cancelled(), options.has_errors()) {
        (Err(cause), _, _) => {
            failed_completion(operation, ProgressFailureClass::KernelFailure, Some(cause))
        }
        (Ok(()), true, _) => cancelled_completion(operation),
        (Ok(()), false, true) => {
            failed_completion(operation, ProgressFailureClass::AlgorithmReport, None)
        }
        (Ok(()), false, false) => ProgressCompletion::Completed,
    }
}

fn failed_completion(
    operation: ProgressOperation,
    class: ProgressFailureClass,
    cause: Option<Failure>,
) -> ProgressCompletion {
    ProgressCompletion::Failed {
        operation,
        stage: ProgressStage::Completion,
        class,
        cause,
        context: None,
    }
}

fn cancelled_completion(operation: ProgressOperation) -> ProgressCompletion {
    ProgressCompletion::Cancelled {
        operation,
        stage: ProgressStage::Completion,
        context: None,
    }
}

/// Preserve only the legacy unbound Result/report contract; `legacy_unbound_area_result(outcome)`.
/// This adapter is never permission to publish a ranged Cancelled or Incomplete result.
pub(crate) fn legacy_unbound_area_result(outcome: ProgressCompletion) -> Result<(), Failure> {
    match outcome {
        ProgressCompletion::Failed {
            cause: Some(cause), ..
        } => Err(cause),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests;
