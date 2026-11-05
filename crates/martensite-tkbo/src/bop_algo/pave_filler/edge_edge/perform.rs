//! Ordered edge intersection preparation, execution, and post-treatment share one pipeline.
use super::{BOPAlgoPaveFiller, EdgeEdgeTask, VertexCouples};
use super::{
    execution::{edge_intersection_scope, execute_edge_tasks},
    progress::EdgeEdgeBoundary,
    store::EdgeEdgeStore,
};
use crate::bop_algo::tools::perform_common_blocks;
use crate::bop_algo::{ProgressCancellation, ProgressCompletion};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    ncollection::Map,
    standard::Failure,
};
use martensite_tkg3d::top_abs::ShapeEnum;

impl BOPAlgoPaveFiller {
    /// Intersects the pave blocks of all edge pairs with interfering boxes, making new vertices
    /// at crossing points and common blocks of coinciding parts (`PerformEE`).
    /// `filler.perform_ee()?`.
    pub fn perform_ee(&mut self) -> Result<(), Failure> {
        let range = ProgressRange::default();
        let observed = ProgressCancellation::default();
        self.perform_ee_with_progress(&range, &observed).map(|_| ())
    }

    /// Preserves native scopes and polls without certifying geometry or whole-operation completion.
    /// Example: `filler.perform_ee_with_progress(range, observed)?`.
    pub(in crate::bop_algo::pave_filler) fn perform_ee_with_progress(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        self.fill_shrunk_data(ShapeEnum::Edge, ShapeEnum::Edge)?;
        let count = self.initialize_progress_pairs(ShapeEnum::Edge, ShapeEnum::Edge)?;
        let scope = ProgressScope::unnamed(range, 10.0, false);
        if count == 0 {
            return Ok(Vec::new());
        }
        self.run_ee_stage(&EdgeEdgeBoundary {
            scope: &scope,
            observed,
        })
    }

    fn run_ee_stage(
        &mut self,
        boundary: &EdgeEdgeBoundary<'_, '_>,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        let Some(tasks) = self.prepare_ee_tasks(boundary)? else {
            return Ok(Vec::new());
        };
        let scope = edge_intersection_scope(boundary.scope, tasks.len());
        let outputs = execute_edge_tasks(tasks, &scope, self.options.run_parallel(), "PerformEE")?;
        let (tasks, mut completions) = unpack_edge_tasks(outputs, boundary.observed);
        if boundary.cancelled(&self.options) {
            return Ok(completions);
        }
        let Some(mut stored) = self.store_ee_tasks(&tasks, boundary)? else {
            return Ok(completions);
        };
        completions.extend(self.finish_ee_store(&mut stored, boundary)?);
        Ok(completions)
    }

    fn finish_ee_store(
        &mut self,
        stored: &mut EdgeEdgeStore,
        boundary: &EdgeEdgeBoundary<'_, '_>,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        let context = self.context.clone();
        perform_common_blocks(&stored.links, self.structure_mut()?, context.get())?;
        self.update_vertices_of_cb()?;
        let range = boundary.scope.next(1.0);
        let children = self.perform_new_vertices_with_progress(
            &mut stored.couples,
            true,
            &range,
            boundary.observed,
        )?;
        if !self.options.has_errors() {
            self.finish_ee_edges(&stored.couples, &mut stored.edges)?;
        }
        Ok(children)
    }

    fn finish_ee_edges(
        &mut self,
        couples: &VertexCouples,
        edges: &mut Map<i32>,
    ) -> Result<(), Failure> {
        if edges.extent() == 0 {
            return Ok(());
        }
        for index in 1..=couples.extent() {
            remove_generated_edge_pair(couples.find_from_index(index)?, edges);
        }
        self.split_pave_blocks(edges, false)
    }
}

fn remove_generated_edge_pair(couple: &crate::bop_ds::CoupleOfPaveBlocks, edges: &mut Map<i32>) {
    for block in [couple.pave_block1(), couple.pave_block2()]
        .into_iter()
        .flatten()
    {
        edges.remove(&block.lock().expect("pave-block lock").original_edge());
    }
}

/// Retains ordered actual task completion and latches cancellation without a parent alert.
/// Example: `let (tasks, outcomes) = unpack_edge_tasks(outputs, observed);`.
pub(super) fn unpack_edge_tasks(
    outputs: Vec<(EdgeEdgeTask, ProgressCompletion)>,
    observed: &ProgressCancellation,
) -> (Vec<EdgeEdgeTask>, Vec<ProgressCompletion>) {
    let mut tasks = Vec::with_capacity(outputs.len());
    let mut completions = Vec::with_capacity(outputs.len());
    for (task, completion) in outputs {
        if matches!(completion, ProgressCompletion::Cancelled { .. }) {
            observed.record_cancelled();
        }
        tasks.push(task);
        completions.push(completion);
    }
    (tasks, completions)
}
