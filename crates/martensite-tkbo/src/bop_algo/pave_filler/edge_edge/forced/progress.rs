//! Forced coincidence checks keep native preparation polls and scope ownership through storage.
use super::super::{
    execution::execute_edge_tasks, perform::unpack_edge_tasks, progress::EdgeEdgeBoundary,
};
use super::*;
use crate::bop_algo::{
    ProgressCancellation, ProgressCompletion,
    tools::{pave_block_links, perform_common_blocks},
};
use martensite_tkernel::message::{ProgressRange, ProgressScope};

impl BOPAlgoPaveFiller {
    /// Lend actual owned worker ranges while retaining raw completion packets.
    /// Example: `filler.force_interf_ee_with_progress(range, observed)?` before forced EF.
    pub(in crate::bop_algo::pave_filler) fn force_interf_ee_with_progress(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        let scope = ProgressScope::unnamed(range, 10., false);
        let boundary = EdgeEdgeBoundary {
            scope: &scope,
            observed,
        };
        if !self.initialize_forced_ee_blocks(&boundary)? {
            return Ok(Vec::new());
        }
        let Some(groups) = self.real_blocks_by_bounds(&boundary)? else {
            return Ok(Vec::new());
        };
        let Some(tasks) = self.collect_forced_ee_tasks(groups, &boundary)? else {
            return Ok(Vec::new());
        };
        if tasks.is_empty() {
            return Ok(Vec::new());
        }
        self.run_forced_ee_tasks(tasks, &boundary)
    }

    fn initialize_forced_ee_blocks(
        &mut self,
        boundary: &EdgeEdgeBoundary<'_, '_>,
    ) -> Result<bool, Failure> {
        let structure = self.data_structure.as_mut().ok_or_else(missing_structure)?;
        for index in 0..structure.nb_source_shapes() {
            if structure.shape_info(index)?.shape_type() == ShapeEnum::Vertex
                && structure.has_interference_for(index)
            {
                structure.init_pave_blocks_for_vertex(index)?;
            }
            // Native ForceInterfEE checks every source, after its possible initialization.
            if boundary.cancelled(&self.options) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn collect_forced_ee_tasks(
        &self,
        groups: BlocksByBounds,
        boundary: &EdgeEdgeBoundary<'_, '_>,
    ) -> Result<Option<Vec<EdgeEdgeTask>>, Failure> {
        let mut tasks = Vec::new();
        for position in 1..=groups.extent() {
            if boundary.cancelled(&self.options) {
                return Ok(None);
            }
            let blocks = groups.find_from_index(position)?;
            if blocks.len() < 2 {
                continue;
            }
            self.append_forced_ee_tasks(groups.find_key(position)?.indices(), blocks, &mut tasks)?;
        }
        Ok(Some(tasks))
    }

    fn run_forced_ee_tasks(
        &mut self,
        tasks: Vec<EdgeEdgeTask>,
        boundary: &EdgeEdgeBoundary<'_, '_>,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        drop(boundary.scope.next(0.7));
        let scope = ProgressScope::with_literal(
            &boundary.scope.next(9.),
            "Checking for coinciding edges",
            tasks.len() as f64,
            false,
        );
        let outputs =
            execute_edge_tasks(tasks, &scope, self.options.run_parallel(), "ForceInterfEE")?;
        let (tasks, completions) = unpack_edge_tasks(outputs, boundary.observed);
        if boundary.cancelled(&self.options) {
            return Ok(completions);
        }
        self.bind_forced_ee_results(&tasks, boundary)?;
        Ok(completions)
    }

    fn bind_forced_ee_results(
        &mut self,
        tasks: &[EdgeEdgeTask],
        boundary: &EdgeEdgeBoundary<'_, '_>,
    ) -> Result<(), Failure> {
        let mut links = pave_block_links();
        for task in tasks {
            if boundary.cancelled(&self.options) {
                return Ok(());
            }
            self.analyze_forced_ee_task(task, &mut links)?;
        }
        perform_common_blocks(&links, self.structure_mut()?, None)
    }
}

#[cfg(test)]
#[path = "contract.rs"]
mod tests;

#[cfg(test)]
#[path = "census.rs"]
mod census;
