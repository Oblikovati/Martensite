//! VE retains the actual native scopes and ordered postprocessing around worker-local tasks.
use super::task::{VertexEdgeWork, execute_vertex_edge_work};
use super::{BOPAlgoPaveFiller, Pair, PaveBlockIndexLinks, fill_map_one, pave_block_index_links};
use crate::bop_algo::pave_filler::progress_observation::native_user_break;
use crate::bop_algo::{ProgressCancellation, ProgressCompletion};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    ncollection::{DataMap, Map},
    standard::Failure,
};
use martensite_tkg3d::top_abs::ShapeEnum;

impl BOPAlgoPaveFiller {
    /// Runs the original VE candidate traversal with its actual borrowed range.
    /// Example: `filler.perform_ve_with_progress(range, observed)?`.
    pub(in crate::bop_algo::pave_filler) fn perform_ve_with_progress(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        self.fill_shrunk_data(ShapeEnum::Vertex, ShapeEnum::Edge)?;
        let count = self.initialize_progress_pairs(ShapeEnum::Vertex, ShapeEnum::Edge)?;
        let scope = ProgressScope::unnamed(range, 1.0, false);
        if count == 0 {
            return Ok(Vec::new());
        }
        let Some(pairs) = self.prepare_ve_pairs(&scope, observed)? else {
            return Ok(Vec::new());
        };
        self.intersect_ve_with_progress(&pairs, true, &scope.next(1.0), observed)
    }

    /// Preserves native candidate traversal while collecting the actual VE map.
    /// Example: `filler.prepare_ve_pairs(scope, observed)?`.
    pub(super) fn prepare_ve_pairs(
        &mut self,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Option<PaveBlockIndexLinks>, Failure> {
        let mut pairs = pave_block_index_links();
        while self.progress_pair_cursor()?.more() {
            if native_user_break(&self.options, scope, observed) {
                return Ok(None);
            }
            let (vertex, edge) = self.progress_pair_cursor()?.value()?;
            if let Some(block) = self.ve_candidate_block(vertex, edge)? {
                fill_map_one(&mut pairs, block, vertex);
            }
            self.progress_pair_cursor()?.next();
        }
        Ok(Some(pairs))
    }

    /// Preserves native IntersectVE preparation, context execution and ordered split effects.
    /// Example: `filler.intersect_ve_with_progress(pairs, true, range, observed)?`.
    pub(in crate::bop_algo::pave_filler) fn intersect_ve_with_progress(
        &mut self,
        pairs: &PaveBlockIndexLinks,
        add_interfs: bool,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        if pairs.extent() == 0 {
            return Ok(Vec::new());
        }
        let outer = ProgressScope::unnamed(range, 10.0, false);
        let mut same_domain = DataMap::default();
        let Some(mut work) = self.prepare_ve_work(pairs, &mut same_domain, &outer, observed)?
        else {
            return Ok(Vec::new());
        };
        self.run_prepared_ve_work(&mut work, &same_domain, add_interfs, &outer, observed)
    }

    fn run_prepared_ve_work(
        &mut self,
        work: &mut [VertexEdgeWork],
        same_domain: &DataMap<Pair, Vec<i32>>,
        add_interfs: bool,
        outer: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        let tasks = ve_task_scope(outer, work.len());
        execute_vertex_edge_work(work, &self.context, &tasks, self.options.run_parallel())?;
        let completions = collect_ve_completions(work, observed)?;
        if native_user_break(&self.options, outer, observed) {
            return Ok(completions);
        }
        self.store_ve_work(work, same_domain, add_interfs, outer, observed)?;
        Ok(completions)
    }

    fn store_ve_work(
        &mut self,
        work: &[VertexEdgeWork],
        same_domain: &DataMap<Pair, Vec<i32>>,
        add_interfs: bool,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let mut edges: Map<i32> = Map::new(1);
        for item in work {
            if native_user_break(&self.options, scope, observed) {
                return Ok(());
            }
            self.apply_ve_solver(&item.solver, same_domain, add_interfs, &mut edges)?;
        }
        self.split_pave_blocks(&edges, add_interfs)
    }
}

fn collect_ve_completions(
    work: &mut [VertexEdgeWork],
    observed: &ProgressCancellation,
) -> Result<Vec<ProgressCompletion>, Failure> {
    let mut completions = Vec::new();
    for item in work {
        let completion = item.take_completion()?;
        if matches!(completion, ProgressCompletion::Cancelled { .. }) {
            observed.record_cancelled();
        }
        if !matches!(completion, ProgressCompletion::Completed) {
            completions.push(completion);
        }
    }
    Ok(completions)
}

fn ve_task_scope<'a>(outer: &'a ProgressScope<'a>, count: usize) -> ProgressScope<'a> {
    let range = outer.next(9.0);
    ProgressScope::with_literal(
        &range,
        "Performing Vertex-Edge intersection",
        count as f64,
        false,
    )
}
