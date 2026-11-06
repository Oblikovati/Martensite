//! Section construction, recursive post-treatment and final forced EF retain the native 1/2/1 ranges.
use super::super::progress_observation::native_user_break;
use super::*;
use crate::bop_algo::{ProgressCancellation, ProgressCompletion};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
impl BOPAlgoPaveFiller {
    /// Reuse the complete intersection program for actual nested section work.
    /// Example: `filler.make_blocks_with_progress(range, observed)?` before pcurves.
    pub(in crate::bop_algo::pave_filler) fn make_blocks_with_progress(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        let outer = ProgressScope::unnamed(range, 4., false);
        if self.glue != Glue::Off {
            return Ok(Vec::new());
        }
        let count = self.ff_count()?;
        let scope = ProgressScope::with_literal(
            &outer.next(1.),
            "Building section edges",
            count as f64,
            false,
        );
        if count == 0 {
            return Ok(Vec::new());
        }
        let mut state = MakeBlocksState::new();
        if !self.run_section_pairs(count, &scope, observed, &mut state)? {
            return Ok(Vec::new());
        }
        self.finish_ranged_blocks(state, &outer, observed)
    }
    fn run_section_pairs(
        &mut self,
        previous: usize,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
        state: &mut MakeBlocksState,
    ) -> Result<bool, Failure> {
        let mut to_recheck = Vec::new();
        let mut count = previous;
        let mut index = 0;
        while index < count {
            if native_user_break(&self.options, scope, observed) {
                return Ok(false);
            }
            // Pairs with curves are processed again at the end so that no section edge is
            // missed because of the processing order.
            let current = if index < previous {
                index
            } else {
                to_recheck[index - previous]
            };
            if self.make_pair_blocks(index, current, previous, state)? {
                to_recheck.push(current);
                count += 1;
            }
            index += 1;
            drop(scope.next(1.));
        }
        Ok(true)
    }
    fn finish_ranged_blocks(
        &mut self,
        mut state: MakeBlocksState,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        self.remove_micro_section_edges(&mut state.sections, &mut state.micro_blocks)?;
        self.make_sd_vertices_ff(&state.groups, &mut state.new_sd)?;
        let mut outcomes =
            self.post_treat_ff_with_progress(&mut state, &scope.next(2.), observed)?;
        if self.options.has_errors() {
            return Ok(outcomes);
        }
        self.correct_tolerance_of_se()?;
        self.update_face_info(&mut state.existing_edges, &state.new_sd, &state.block_faces)?;
        self.update_pave_blocks_sd(&state.new_sd)?;
        outcomes.extend(self.put_se_in_other_faces_with_progress(&scope.next(1.), observed)?);
        Ok(outcomes)
    }
}
