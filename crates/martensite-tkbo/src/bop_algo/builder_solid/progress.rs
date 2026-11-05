//! The concrete solid receiver owns preparation; shared choreography has no base fallback.
use super::BuilderSolid;
use crate::bop_algo::builder_area::progress::{
    BuilderAreaHooks, classify_area_progress, perform_area_stages_in_scope,
};
use crate::bop_algo::{BuilderArea, ProgressCancellation, ProgressCompletion, ProgressOperation};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::Failure,
};

/// Run the actual solid receiver and classify its real completion; `perform_solid_with_progress(&mut solid, &range)`.
pub(crate) fn perform_solid_with_progress(
    builder: &mut BuilderSolid,
    range: &ProgressRange<'_>,
) -> ProgressCompletion {
    let observed = ProgressCancellation::default();
    let scope = ProgressScope::unnamed(range, 100., false);
    let result = run_solid_stages(builder, &scope, &observed);
    classify_area_progress(
        ProgressOperation::BuilderSolid,
        builder.options(),
        result,
        &observed,
    )
}

fn run_solid_stages(
    builder: &mut BuilderSolid,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
) -> Result<(), Failure> {
    if !builder.prepare_solid() {
        return Ok(());
    }
    let mut actual = SolidProgressStages { builder, observed };
    let _: &(dyn Send + Sync) = &actual;
    perform_area_stages_in_scope(&mut actual, scope)
}

struct SolidProgressStages<'a> {
    builder: &'a mut BuilderSolid,
    observed: &'a ProgressCancellation,
}

impl BuilderAreaHooks for SolidProgressStages<'_> {
    fn area(&self) -> &BuilderArea {
        // The actual solid keeps its report; no base receiver is substituted.
        &self.builder.area
    }

    fn area_mut(&mut self) -> &mut BuilderArea {
        // Only this lexical operation adapter can mutate the same concrete receiver.
        &mut self.builder.area
    }

    fn perform_shapes_to_avoid(&mut self, range: &ProgressRange<'_>) -> Result<(), Failure> {
        // Dispatch to the actual solid stage, including its own cancellation checkpoints.
        self.builder.perform_shapes_to_avoid(range, self.observed)
    }

    fn perform_loops(&mut self, range: &ProgressRange<'_>) -> Result<(), Failure> {
        // The solid owns shell preparation, actual splitting and remaining-face treatment.
        self.builder.perform_loops(range, self.observed)
    }

    fn perform_areas(&mut self, range: &ProgressRange<'_>) -> Result<(), Failure> {
        // No face-area implementation can be selected through inherited base state.
        self.builder.perform_areas(range, self.observed)
    }

    fn perform_internal_shapes(&mut self, range: &ProgressRange<'_>) -> Result<(), Failure> {
        // The same solid receiver owns real face classification and insertion.
        self.builder.perform_internal_shapes(range, self.observed)
    }
}
