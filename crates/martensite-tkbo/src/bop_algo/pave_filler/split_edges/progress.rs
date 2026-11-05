//! Pool checks precede task creation; storage checks retain native partial diagnostic state.
use super::super::progress_observation::native_user_break;
use super::*;
use crate::bop_algo::{ProgressCancellation, ProgressCompletion};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
#[path = "workers.rs"]
mod workers;

impl BOPAlgoPaveFiller {
    /// Lend actual worker ranges while retaining cancellation separately from raw stored edges.
    /// Example: `filler.make_split_edges_with_progress(range, observed)?` before section blocks.
    pub(in crate::bop_algo::pave_filler) fn make_split_edges_with_progress(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        let scope = ProgressScope::unnamed(range, 1., false);
        if self
            .data_structure
            .as_ref()
            .ok_or_else(missing_structure)?
            .pave_blocks_pool()
            .is_empty()
        {
            return Ok(Vec::new());
        }
        self.update_common_blocks_with_sd_vertices()?;
        let Some(tasks) = self.collect_split_edge_tasks(&scope, observed)? else {
            return Ok(Vec::new());
        };
        self.run_split_edge_tasks(tasks, &scope, observed)
    }

    fn collect_split_edge_tasks(
        &mut self,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Option<Vec<SplitEdgeTask>>, Failure> {
        let mut tasks = Vec::new();
        let mut fence = Map::with_hasher(CommonBlockHandleIdentity, 100);
        let lists = self
            .data_structure
            .as_ref()
            .ok_or_else(missing_structure)?
            .pave_blocks_pool()
            .to_vec();
        for list in &lists {
            if native_user_break(&self.options, scope, observed) {
                return Ok(None);
            }
            self.append_split_list_tasks(list, &mut fence, &mut tasks)?;
        }
        Ok(Some(tasks))
    }

    fn append_split_list_tasks(
        &mut self,
        list: &[PaveBlockHandle],
        fence: &mut Map<CommonBlockHandle, CommonBlockHandleIdentity>,
        tasks: &mut Vec<SplitEdgeTask>,
    ) -> Result<(), Failure> {
        for block in list {
            if let Some(task) = self.split_edge_task(block, list.len(), fence)? {
                tasks.push(task);
            }
        }
        Ok(())
    }

    fn run_split_edge_tasks(
        &mut self,
        tasks: Vec<SplitEdgeTask>,
        outer: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        let scope = ProgressScope::with_literal(
            &outer.next(1.),
            "Splitting edges",
            tasks.len() as f64,
            false,
        );
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        let outputs = workers::execute_split_edge_tasks(
            tasks,
            &scope,
            structure,
            &self.context,
            self.options.run_parallel(),
        )?;
        let (tasks, outcomes) = unpack_split_edge_tasks(outputs, observed);
        if self.options.has_errors() {
            return Ok(outcomes);
        }
        for task in tasks {
            if native_user_break(&self.options, outer, observed) {
                return Ok(outcomes);
            }
            self.store_split_edge(task)?;
        }
        Ok(outcomes)
    }
}
fn unpack_split_edge_tasks(
    outputs: Vec<(SplitEdgeTask, ProgressCompletion)>,
    observed: &ProgressCancellation,
) -> (Vec<SplitEdgeTask>, Vec<ProgressCompletion>) {
    let mut tasks = Vec::with_capacity(outputs.len());
    let mut outcomes = Vec::with_capacity(outputs.len());
    for (task, outcome) in outputs {
        if matches!(outcome, ProgressCompletion::Cancelled { .. }) {
            observed.record_cancelled();
        }
        tasks.push(task);
        outcomes.push(outcome);
    }
    (tasks, outcomes)
}
#[cfg(test)]
#[path = "contract.rs"]
mod tests;

#[cfg(test)]
#[path = "census.rs"]
mod census;
#[cfg(test)]
#[path = "fixture.rs"]
mod fixture;
