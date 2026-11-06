//! Prepared VF storage preserve the original coordinator geometry ordering.
use super::{BOPAlgoPaveFiller, VertexFaceSolver};
use crate::bop_ds::{InterfVF, Pair};
use martensite_tkernel::{
    ncollection::{DataMap, Map},
    standard::Failure,
};

impl BOPAlgoPaveFiller {
    /// Merges one task in input order, preserving native warning-only failures.
    /// Example: `filler.apply_vf_solver(solver, grouped)?;`.
    pub(super) fn apply_vf_solver(
        &mut self,
        solver: &VertexFaceSolver,
        grouped: &DataMap<Pair, Map<i32>>,
    ) -> Result<(), Failure> {
        if solver.flag != 0 {
            self.warn_failed_vertex_face(solver)?;
            return Ok(());
        }
        let originals = original_vertex_face_indices(solver, grouped)?;
        let mut updated = solver.vertex;
        for original in originals {
            updated = self.store_original_vertex_face(solver, original)?;
        }
        let info = self.structure_mut()?.change_face_info(solver.face)?;
        info.change_vertices_in().add(updated);
        Ok(())
    }
}

impl BOPAlgoPaveFiller {
    fn warn_failed_vertex_face(&mut self, solver: &VertexFaceSolver) -> Result<(), Failure> {
        if !solver.failed {
            return Ok(());
        }
        let structure = self.structure_mut()?;
        let vertex = structure.shape(solver.vertex)?.clone();
        let face = structure.shape(solver.face)?.clone();
        self.add_intersection_failed_warning(&vertex, &face)
    }

    fn store_original_vertex_face(
        &mut self,
        solver: &VertexFaceSolver,
        original: i32,
    ) -> Result<i32, Failure> {
        let structure = self.structure_mut()?;
        let position = structure.interf_vf_records().len();
        let record = structure.interf_vf().append(InterfVF::default());
        record.set_indices(original, solver.face);
        record.set_uv(solver.uv.0, solver.uv.1);
        structure.add_interference(original, solver.face);
        let updated = self.update_vertex(original, solver.new_tolerance)?;
        let structure = self.structure_mut()?;
        if structure.is_new_shape(updated) {
            structure.interf_vf()[position].set_index_new(updated);
        }
        Ok(updated)
    }
}

fn original_vertex_face_indices(
    solver: &VertexFaceSolver,
    grouped: &DataMap<Pair, Map<i32>>,
) -> Result<Vec<i32>, Failure> {
    let group = grouped.find(&Pair::new(solver.vertex, solver.face))?;
    Ok(group.iter().copied().collect())
}
