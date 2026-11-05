//! The assigning scope spans joined solid workers and all ordered coordinator updates.
use super::*;
use crate::bop_algo::{
    ProgressCancellation, ProgressCompletion, ProgressOperation,
    builder_area::progress::{classify_area_progress, observe_user_break},
};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
#[path = "workers.rs"]
mod workers;
impl BOPAlgoBuilder {
    /// Run real split-solid tasks with borrowed progress; `builder.build_split_solids_progress_raw(ds, drafts, range, observed)?`.
    /// Mutable raw image maps do not certify publication of a complete operation.
    pub(crate) fn build_split_solids_progress_raw(
        &mut self,
        structure: &DataStructure,
        drafts: &SameDomainShapes,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<ProgressCompletion, Failure> {
        let outer = ProgressScope::unnamed(range, 10., false);
        let Some(mut untouched) = self.untouched_solid_sets(structure, drafts, &outer, observed)?
        else {
            return Ok(classify_area_progress(
                ProgressOperation::Builder,
                self.options(),
                Ok(()),
                observed,
            ));
        };
        let (mut splits, builders) = self.collect_split_builders(structure, drafts)?;
        let range = outer.next(9.);
        let assigning =
            ProgressScope::with_literal(&range, "Splitting solids", builders.len() as f64, false);
        let (builders, completion) = workers::execute(
            builders,
            self.options().run_parallel(),
            &assigning,
            observed,
        )?;
        if observe_user_break(self.options(), &outer, observed) {
            return Ok(retain_split_stop(completion, observed, self));
        }
        for (solid, builder) in builders {
            splits.add(solid.clone(), builder.areas().clone());
            self.merge_solid_builder_report(&solid, &builder)?;
        }
        self.bind_solid_splits(&splits, &mut untouched)?;
        Ok(retain_split_stop(completion, observed, self))
    }
}
fn retain_split_stop(
    completion: ProgressCompletion,
    observed: &ProgressCancellation,
    builder: &BOPAlgoBuilder,
) -> ProgressCompletion {
    if matches!(
        completion,
        ProgressCompletion::Failed { .. } | ProgressCompletion::Incomplete { .. }
    ) {
        return completion;
    }
    if matches!(completion, ProgressCompletion::Cancelled { .. }) {
        return completion;
    }
    classify_area_progress(
        ProgressOperation::Builder,
        builder.options(),
        Ok(()),
        observed,
    )
}
#[cfg(test)]
#[path = "contract.rs"]
mod tests;
