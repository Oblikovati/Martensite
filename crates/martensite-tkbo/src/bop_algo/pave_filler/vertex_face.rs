//! `BOPAlgoPaveFiller_4`: vertex/face intersection and the treatment of EE vertices.
use super::{BOPAlgoPaveFiller, Glue, missing_structure};
use crate::bop_algo::{ProgressCancellation, ProgressCompletion};
use martensite_tkernel::{message::ProgressRange, standard::Failure};

/// One prepared `BOPAlgoVertexFace` solver and its computed result.
struct VertexFaceSolver {
    vertex: i32,
    face: i32,
    flag: i32,
    uv: (f64, f64),
    new_tolerance: f64,
    failed: bool,
    cancelled: bool,
    cause: Option<Failure>,
}

impl BOPAlgoPaveFiller {
    /// Intersects vertices with faces, recording VF interferences and the faces' In vertices,
    /// then treats the vertices created by EE intersections (`PerformVF`); `filler.perform_vf()?`.
    pub fn perform_vf(&mut self) -> Result<(), Failure> {
        let range = ProgressRange::default();
        let observed = ProgressCancellation::default();
        self.perform_vf_with_progress(&range, &observed).map(|_| ())
    }

    /// Retains native VF task outcomes; callers must also inspect the shared cancellation
    /// latch and returned failure before classifying this whole stage as completed.
    /// Example: `let outcomes = filler.perform_vf_with_progress(range, observed)?;`.
    pub(super) fn perform_vf_with_progress(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        self.run_vertex_face_progress(range, observed)
    }
}

#[path = "vertex_face/candidates.rs"]
mod candidates;
#[path = "vertex_face/ee_vertices.rs"]
mod ee_vertices;
#[path = "vertex_face/execution.rs"]
mod execution;
#[path = "vertex_face/progress.rs"]
mod progress;
#[path = "vertex_face/storage.rs"]
mod storage;

#[cfg(test)]
#[path = "vertex_face/contract.rs"]
mod contract;

#[cfg(test)]
#[path = "vertex_face/census.rs"]
mod census;

#[cfg(test)]
#[path = "vertex_face/scope_lifetime_contract.rs"]
mod scope_lifetime_contract;
