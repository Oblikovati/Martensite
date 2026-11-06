//! Both native overloads share actual context workers and preserve the add-interference policy.
use super::super::{
    execution::execute_edge_face_tasks, perform::unpack_edge_face_tasks, progress::EdgeFaceBoundary,
};
use super::*;
use crate::bop_algo::{ProgressCancellation, ProgressCompletion};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
#[path = "preparation.rs"]
mod preparation;

impl BOPAlgoPaveFiller {
    /// Collect real blocks only for a primary operation, at native source-edge polls.
    /// Example: `filler.force_interf_ef_with_progress(range, observed)?` before face intersections.
    pub(in crate::bop_algo::pave_filler) fn force_interf_ef_with_progress(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        let scope = ProgressScope::unnamed(range, 1., false);
        if !self.is_primary {
            return Ok(Vec::new());
        }
        let boundary = EdgeFaceBoundary {
            scope: &scope,
            observed,
        };
        let Some(blocks) = self.collect_forced_ef_blocks(&boundary)? else {
            return Ok(Vec::new());
        };
        self.force_interf_ef_blocks_with_progress(&blocks, true, &scope.next(1.), observed)
    }

    /// Preserve section callers that request IN blocks without new EF records/common blocks.
    /// Example: `filler.force_interf_ef_blocks_with_progress(blocks, false, range, observed)?`.
    pub(in crate::bop_algo::pave_filler) fn force_interf_ef_blocks_with_progress(
        &mut self,
        blocks: &PaveBlockIndexMap,
        add_interf: bool,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        let scope = ProgressScope::unnamed(range, 10., false);
        if blocks.extent() == 0 {
            return Ok(Vec::new());
        }
        let boundary = EdgeFaceBoundary {
            scope: &scope,
            observed,
        };
        let Some((tree, prepared)) = self.forced_block_tree(blocks, &boundary)? else {
            return Ok(Vec::new());
        };
        let Some(tasks) = self.collect_forced_ef_tasks(tree, prepared, &boundary)? else {
            return Ok(Vec::new());
        };
        if tasks.is_empty() {
            return Ok(Vec::new());
        }
        self.run_forced_ef_tasks(tasks, add_interf, &boundary)
    }

    fn collect_forced_ef_tasks(
        &mut self,
        mut tree: BoxSet<BvhVec3d>,
        prepared: PaveBlockIndexMap,
        boundary: &EdgeFaceBoundary<'_, '_>,
    ) -> Result<Option<Vec<EdgeFaceTask>>, Failure> {
        let check_mode = self.arguments.extent() == 1;
        let mut tasks = Vec::new();
        let face_count = self
            .data_structure
            .as_ref()
            .ok_or_else(missing_structure)?
            .nb_source_shapes();
        for face in 0..face_count {
            if !self.append_forced_ef_tasks(
                face, &mut tree, &prepared, check_mode, &mut tasks, boundary,
            )? {
                return Ok(None);
            }
        }
        Ok(Some(tasks))
    }

    fn run_forced_ef_tasks(
        &mut self,
        tasks: Vec<EdgeFaceTask>,
        add_interf: bool,
        boundary: &EdgeFaceBoundary<'_, '_>,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        drop(boundary.scope.next(0.7));
        let scope = ProgressScope::with_literal(
            &boundary.scope.next(9.),
            "Checking for edges coinciding with faces",
            tasks.len() as f64,
            false,
        );
        let outputs = execute_edge_face_tasks(
            tasks,
            &scope,
            &self.context,
            self.options.run_parallel(),
            "ForceInterfEF",
        )?;
        let (tasks, outcomes) = unpack_edge_face_tasks(outputs, boundary.observed);
        if boundary.cancelled(&self.options) {
            return Ok(outcomes);
        }
        self.bind_forced_ef_results(&tasks, add_interf, boundary)?;
        Ok(outcomes)
    }

    fn bind_forced_ef_results(
        &mut self,
        tasks: &[EdgeFaceTask],
        add_interf: bool,
        boundary: &EdgeFaceBoundary<'_, '_>,
    ) -> Result<(), Failure> {
        let mut links = pave_block_index_links();
        for task in tasks {
            if boundary.cancelled(&self.options) {
                return Ok(());
            }
            self.bind_forced_ef_task(task, add_interf, &mut links)?;
        }
        if links.extent() > 0 {
            perform_common_blocks_on_faces(&links, self.structure_mut()?, None)?;
        }
        Ok(())
    }

    fn bind_forced_ef_task(
        &mut self,
        task: &EdgeFaceTask,
        add_interf: bool,
        links: &mut crate::bop_algo::tools::PaveBlockIndexLinks,
    ) -> Result<(), Failure> {
        if task.failed() {
            return self.add_intersection_failed_warning(
                &task.edge.clone().into_shape(),
                &task.face.clone().into_shape(),
            );
        }
        let [part] = task.parts.as_slice() else {
            return Ok(());
        };
        if part.shape_type() != ShapeEnum::Edge {
            return Ok(());
        }
        let structure = self.structure_mut()?;
        if add_interf {
            let record = structure.interf_ef().append(InterfEF::default());
            record.set_indices(task.edge_index, task.face_index);
            record.set_common_part(part);
            structure.add_interference(task.edge_index, task.face_index);
        }
        structure
            .change_face_info(task.face_index)?
            .change_pave_blocks_in()
            .add(Arc::clone(&task.block));
        if add_interf {
            fill_map_one(links, Arc::clone(&task.block), task.face_index);
        }
        Ok(())
    }
}
#[cfg(test)]
#[path = "contract.rs"]
mod tests;

#[cfg(test)]
#[path = "census.rs"]
mod census;
