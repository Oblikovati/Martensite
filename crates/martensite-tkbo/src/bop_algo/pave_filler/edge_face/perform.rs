//! Ordered EF preparation, actual context execution and full new-vertex tail.
use super::*;
use super::{
    execution::{edge_face_intersection_scope, execute_edge_face_tasks},
    progress::EdgeFaceBoundary,
};
use crate::bop_algo::{ProgressCancellation, ProgressCompletion};
use martensite_tkernel::message::{ProgressRange, ProgressScope};

impl BOPAlgoPaveFiller {
    /// Intersects real edge pave blocks and faces using the same ranged pipeline.
    /// Example: `filler.perform_ef()?`.
    pub fn perform_ef(&mut self) -> Result<(), Failure> {
        let range = ProgressRange::default();
        let observed = ProgressCancellation::default();
        self.perform_ef_with_progress(&range, &observed).map(|_| ())
    }

    /// Preserves native EF scopes and reports, retaining every worker and tail outcome.
    /// Example: `filler.perform_ef_with_progress(range, observed)?`.
    pub(in crate::bop_algo::pave_filler) fn perform_ef_with_progress(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        self.fill_shrunk_data(ShapeEnum::Edge, ShapeEnum::Face)?;
        let count = self.initialize_progress_pairs(ShapeEnum::Edge, ShapeEnum::Face)?;
        let scope = ProgressScope::unnamed(range, 10.0, false);
        if count == 0 {
            return Ok(Vec::new());
        }
        self.start_ef_stage(&EdgeFaceBoundary {
            scope: &scope,
            observed,
        })
    }

    fn start_ef_stage(
        &mut self,
        boundary: &EdgeFaceBoundary<'_, '_>,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        if self.glue == Glue::Full {
            self.initialize_glued_ef_faces()?;
            return Ok(Vec::new());
        }
        self.run_ef_stage(boundary)
    }

    fn run_ef_stage(
        &mut self,
        boundary: &EdgeFaceBoundary<'_, '_>,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        let Some(tasks) = self.prepare_ef_tasks(boundary)? else {
            return Ok(Vec::new());
        };
        let scope = edge_face_intersection_scope(boundary.scope, tasks.len());
        let context = self.context.clone();
        let outputs = execute_edge_face_tasks(
            tasks,
            &scope,
            &context,
            self.options.run_parallel(),
            "PerformEF",
        )?;
        let (tasks, mut completions) = unpack_edge_face_tasks(outputs, boundary.observed);
        if boundary.cancelled(&self.options) {
            return Ok(completions);
        }
        if let Some(mut stored) = self.store_ef_tasks(&tasks, boundary)? {
            completions.extend(self.finish_ef_store(&mut stored, boundary)?);
        }
        Ok(completions)
    }

    fn store_ef_tasks(
        &mut self,
        tasks: &[EdgeFaceTask],
        boundary: &EdgeFaceBoundary<'_, '_>,
    ) -> Result<Option<EdgeFaceResults>, Failure> {
        let mut results = EdgeFaceResults {
            faces: Map::new(100),
            couples: VertexCouples::with_hasher(ShapeMapHasher, 100),
            blocks_on_faces: pave_block_index_links(),
        };
        for task in tasks {
            if boundary.cancelled(&self.options) {
                return Ok(None);
            }
            if !self.analyze_edge_face_task(task, &mut results, boundary)? {
                return Ok(None);
            }
        }
        Ok(Some(results))
    }

    fn finish_ef_store(
        &mut self,
        results: &mut EdgeFaceResults,
        boundary: &EdgeFaceBoundary<'_, '_>,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        let context = self.context.clone();
        perform_common_blocks_on_faces(
            &results.blocks_on_faces,
            self.structure_mut()?,
            context.get(),
        )?;
        self.update_vertices_of_cb()?;
        let children = self.ef_new_vertex_children(&mut results.couples, boundary)?;
        if !self.options.has_errors() {
            self.structure_mut()?
                .update_face_info_in_map(&results.faces)?;
        }
        Ok(children)
    }

    fn ef_new_vertex_children(
        &mut self,
        couples: &mut VertexCouples,
        boundary: &EdgeFaceBoundary<'_, '_>,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        let range = boundary.scope.next(1.0);
        self.perform_new_vertices_with_progress(couples, false, &range, boundary.observed)
    }
}

/// Retain ordered worker metadata and observed cancellation for ordinary and forced EF.
/// Example: `let (tasks, outcomes) = unpack_edge_face_tasks(outputs, observed);`.
pub(super) fn unpack_edge_face_tasks(
    outputs: Vec<(EdgeFaceTask, ProgressCompletion)>,
    observed: &ProgressCancellation,
) -> (Vec<EdgeFaceTask>, Vec<ProgressCompletion>) {
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
