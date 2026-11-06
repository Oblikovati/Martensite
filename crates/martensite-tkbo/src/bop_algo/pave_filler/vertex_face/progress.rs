//! Native VF owns ten units; only its scheduled tasks receive the nine-unit child range.
use super::execution::{execute_vertex_face_tasks, vertex_face_intersection_scope};
use super::{BOPAlgoPaveFiller, Glue, VertexFaceSolver};
use crate::bop_algo::pave_filler::progress_observation::native_user_break;
use crate::bop_algo::{ProgressCancellation, ProgressCompletion};
use crate::bop_ds::Pair;
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    ncollection::{DataMap, Map},
    standard::Failure,
};
use martensite_tkg3d::top_abs::ShapeEnum;

type VertexFaceGroups = DataMap<Pair, Map<i32>>;

impl BOPAlgoPaveFiller {
    /// Preserves the actual branch order before allocating VF task ranges.
    /// Example: `filler.run_vertex_face_progress(range, observed)?;`.
    pub(super) fn run_vertex_face_progress(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        let count = self.initialize_progress_pairs(ShapeEnum::Vertex, ShapeEnum::Face)?;
        let outer = ProgressScope::unnamed(range, 10.0, false);
        if self.glue == Glue::Full {
            // No vertex/face intersection in this mode: only the face records are initialized.
            self.initialize_glued_vf_faces()?;
            return Ok(Vec::new());
        }
        if count == 0 {
            self.treat_vertices_ee()?;
            return Ok(Vec::new());
        }
        self.intersect_vertex_face_tasks(&outer, observed)
    }

    fn initialize_glued_vf_faces(&mut self) -> Result<(), Failure> {
        while self.progress_pair_cursor()?.more() {
            let (vertex, face) = self.progress_pair_cursor()?.value()?;
            let structure = self.structure_mut()?;
            if !structure.is_sub_shape(vertex, face)? {
                structure.change_face_info(face)?;
            }
            self.progress_pair_cursor()?.next();
        }
        Ok(())
    }

    fn intersect_vertex_face_tasks(
        &mut self,
        outer: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        let mut grouped = VertexFaceGroups::default();
        let Some(mut solvers) = self.prepare_vertex_face_tasks(outer, observed, &mut grouped)?
        else {
            return Ok(Vec::new());
        };
        // Keep the native named scope alive through post-join polls, merge and EE treatment.
        // Closing it early makes zero-task progress callbacks cancel work retroactively.
        let scope = vertex_face_intersection_scope(outer, solvers.len());
        let completions = execute_vertex_face_tasks(self, &mut solvers, &scope, observed)?;
        if native_user_break(&self.options, outer, observed) {
            return Ok(completions);
        }
        if self.merge_vertex_face_tasks(&solvers, &grouped, outer, observed)? {
            self.treat_vertices_ee()?;
        }
        Ok(completions)
    }

    fn prepare_vertex_face_tasks(
        &mut self,
        outer: &ProgressScope<'_>,
        observed: &ProgressCancellation,
        grouped: &mut VertexFaceGroups,
    ) -> Result<Option<Vec<VertexFaceSolver>>, Failure> {
        let mut solvers = Vec::new();
        while self.progress_pair_cursor()?.more() {
            if native_user_break(&self.options, outer, observed) {
                return Ok(None);
            }
            let (vertex, face) = self.progress_pair_cursor()?.value()?;
            if let Some(solver) = self.vf_candidate(vertex, face, grouped)? {
                solvers.push(solver);
            }
            self.progress_pair_cursor()?.next();
        }
        Ok(Some(solvers))
    }

    fn merge_vertex_face_tasks(
        &mut self,
        solvers: &[VertexFaceSolver],
        grouped: &VertexFaceGroups,
        outer: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<bool, Failure> {
        for solver in solvers {
            if native_user_break(&self.options, outer, observed) {
                return Ok(false);
            }
            self.apply_vf_solver(solver, grouped)?;
        }
        Ok(true)
    }
}
