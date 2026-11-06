//! Prepared VF candidates preserve the original coordinator geometry ordering.
use super::{BOPAlgoPaveFiller, VertexFaceSolver};
use crate::bop_ds::Pair;
use martensite_tkernel::{
    ncollection::{DataMap, Map},
    standard::Failure,
};

impl BOPAlgoPaveFiller {
    /// Deduplicates only after the actual face-info and interference checks.
    /// Example: `let task = filler.vf_candidate(vertex, face, grouped)?;`.
    pub(super) fn vf_candidate(
        &mut self,
        vertex: i32,
        face: i32,
        grouped: &mut DataMap<Pair, Map<i32>>,
    ) -> Result<Option<VertexFaceSolver>, Failure> {
        if self.skip_vertex_face_candidate(vertex, face)? {
            return Ok(None);
        }
        let structure = self.structure_mut()?;
        let mut representative = vertex;
        structure.has_shape_sd_into(vertex, &mut representative);
        let key = Pair::new(representative, face);
        if let Some(group) = grouped.change_seek(&key) {
            group.add(vertex);
            return Ok(None);
        }
        grouped.bind(key, original_vertex_face_group(vertex));
        Ok(Some(VertexFaceSolver::prepared(representative, face)))
    }
}

impl BOPAlgoPaveFiller {
    fn skip_vertex_face_candidate(&mut self, vertex: i32, face: i32) -> Result<bool, Failure> {
        let structure = self.structure_mut()?;
        if structure.is_sub_shape(vertex, face)? || structure.has_interference_between(vertex, face)
        {
            return Ok(true);
        }
        structure.change_face_info(face)?;
        structure.has_interference_shape_subshapes(vertex, face)
    }
}

impl VertexFaceSolver {
    /// Preserves the native untouched outputs until projection completes.
    /// Example: `let solver = VertexFaceSolver::prepared(0, 1);`.
    pub(super) fn prepared(vertex: i32, face: i32) -> Self {
        Self {
            vertex,
            face,
            flag: -1,
            uv: (-1., -1.),
            new_tolerance: -1.,
            failed: false,
            cancelled: false,
            cause: None,
        }
    }
}

fn original_vertex_face_group(vertex: i32) -> Map<i32> {
    let mut group: Map<i32> = Map::new(1);
    group.add(vertex);
    group
}
