//! `BOPAlgoPaveFiller`: the intersection pipeline (`Perform`, `PerformInternal`) and
//! `RepeatIntersection`.
use super::BOPAlgoPaveFiller;
use crate::bop_algo::AlertIntersectionFailed;
use martensite_tkernel::message::{AlertHandle, ProgressRange};
use martensite_tkernel::standard::Failure;
use std::sync::Arc;

use crate::bop_algo::{ProgressCancellation, ProgressCompletion};
#[path = "ranged_intersection/completion.rs"]
mod completion;
#[path = "ranged_intersection/pipeline.rs"]
mod pipeline;

impl BOPAlgoPaveFiller {
    /// Intersect arguments in the caller's range and retain execution diagnostics.
    /// Example: `let status = filler.perform_with_progress(&range);` still requires geometry validation.
    pub fn perform_with_progress(&mut self, range: &ProgressRange<'_>) -> ProgressCompletion {
        self.perform_on_progress(range)
    }

    /// Runs the whole intersection; any failure becomes an `AlertIntersectionFailed` error, as
    /// the native catch of StandardFailure does (`Perform`); `filler.perform();`.
    pub fn perform(&mut self) {
        self.perform_on_progress(&ProgressRange::default());
    }

    /// Initialization and all interference stages in native order (`PerformInternal`).
    /// `filler.perform_internal()?`.
    pub fn perform_internal(&mut self) -> Result<(), Failure> {
        self.perform_internal_with(Self::initialize)
    }

    fn perform_internal_with(
        &mut self,
        initialize: impl FnOnce(&mut Self) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        let range = ProgressRange::default();
        perform_pipeline_with(self, &range, |filler, _range| initialize(filler))
    }

    fn finish_weighted_stage(&mut self, slot: usize) -> Result<(), Failure> {
        match slot {
            2 | 3 | 4 | 10 => self.update_pave_blocks_with_sd_vertices(),
            5 => self.update_after_ef(),
            9 => self.update_after_ff(),
            11 => self.finish_blocks(),
            _ => Ok(()),
        }
    }

    fn update_after_ef(&mut self) -> Result<(), Failure> {
        self.update_pave_blocks_with_sd_vertices()?;
        self.update_interfs_with_sd_vertices()
    }

    fn update_after_ff(&mut self) -> Result<(), Failure> {
        self.update_blocks_with_shared_vertices()?;
        self.structure_mut()?.refine_face_info_in()
    }

    /// The native steps between MakeBlocks and MakePCurves, which cannot fail by error.
    fn finish_blocks(&mut self) -> Result<(), Failure> {
        self.check_self_interference()?;
        self.update_interfs_with_sd_vertices()?;
        let structure = self.structure_mut()?;
        structure.release_pave_blocks()?;
        structure.refine_face_info_on()?;
        self.remove_micro_edges()
    }
}

/// Retains the actual initializer receiver while lending progress only on this stack.
/// Example: `perform_pipeline_with(filler, range, |actual, _| actual.initialize())?`.
fn perform_pipeline_with(
    filler: &mut BOPAlgoPaveFiller,
    range: &ProgressRange<'_>,
    initialize: impl FnOnce(&mut BOPAlgoPaveFiller, &ProgressRange<'_>) -> Result<(), Failure>,
) -> Result<(), Failure> {
    let observed = ProgressCancellation::default();
    pipeline::run_intersection_program(filler, range, &observed, initialize).map(|_| ())
}

#[cfg(test)]
#[path = "initializer_contract.rs"]
mod initializer_contract;

#[cfg(test)]
#[path = "ranged_intersection/census.rs"]
mod census;
#[cfg(test)]
#[path = "ranged_intersection/fixture.rs"]
mod fixture;
#[cfg(test)]
#[path = "ranged_intersection/snapshot.rs"]
mod snapshot;

#[cfg(test)]
#[path = "ranged_intersection/contract.rs"]
mod ranged_contract;
