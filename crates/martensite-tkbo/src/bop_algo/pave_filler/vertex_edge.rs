//! `BOPAlgoPaveFiller_2`: vertex/edge intersection and the splitting of pave blocks.
use super::vertex_vertex::pair_compound;
use super::{BOPAlgoPaveFiller, missing_structure};
use crate::bop_algo::AlertIntersectionOfPairOfShapesFailed;
use crate::bop_algo::tools::{PaveBlockIndexLinks, fill_map_one, pave_block_index_links};
use crate::bop_ds::{InterfVE, Pair, Pave, PaveBlockHandle};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    message::AlertHandle,
    message::ProgressRange,
    ncollection::{DataMap, Map},
    standard::Failure,
};
use std::sync::Arc;

#[path = "vertex_edge/driver.rs"]
mod driver;
#[path = "vertex_edge/prepare.rs"]
mod prepare;
#[path = "vertex_edge/split.rs"]
mod split;
#[path = "vertex_edge/task.rs"]
mod task;

/// One prepared `BOPAlgoVertexEdge` solver: indices, its pave block and computed result.
struct VertexEdgeSolver {
    vertex: i32,
    edge: i32,
    flag: i32,
    parameter: f64,
    new_tolerance: f64,
    failed: bool,
}

impl VertexEdgeSolver {
    fn uncomputed(vertex: i32, edge: i32) -> Self {
        Self {
            vertex,
            edge,
            flag: -1,
            parameter: -1.0,
            new_tolerance: -1.0,
            failed: false,
        }
    }
}

impl BOPAlgoPaveFiller {
    /// Intersects the vertices with the first splittable pave block of each edge whose box
    /// they touch (`PerformVE`); `filler.perform_ve()?`.
    pub fn perform_ve(&mut self) -> Result<(), Failure> {
        let range = ProgressRange::default();
        let observed = crate::bop_algo::ProgressCancellation::default();
        self.perform_ve_with_progress(&range, &observed).map(|_| ())
    }

    /// Computes the parameters of the vertices on the pave blocks' edges, adds extra paves
    /// (and VE interferences when `add_interfs`) and splits the touched edges (`IntersectVE`).
    /// `filler.intersect_ve(&pairs, true)?`.
    pub fn intersect_ve(
        &mut self,
        pairs: &PaveBlockIndexLinks,
        add_interfs: bool,
    ) -> Result<(), Failure> {
        let range = ProgressRange::default();
        let observed = crate::bop_algo::ProgressCancellation::default();
        self.intersect_ve_with_progress(pairs, add_interfs, &range, &observed)
            .map(|_| ())
    }

    /// Warns that the intersection of two sub-shapes failed; `filler.add_intersection_failed_warning(&a, &b)?`.
    pub fn add_intersection_failed_warning(
        &mut self,
        first: &Shape,
        second: &Shape,
    ) -> Result<(), Failure> {
        let pair = pair_compound(first, second)?;
        let alert: AlertHandle = Arc::new(AlertIntersectionOfPairOfShapesFailed::new(&pair));
        self.options.add_warning(&alert);
        Ok(())
    }

    fn ve_candidate_block(
        &self,
        vertex: i32,
        edge: i32,
    ) -> Result<Option<PaveBlockHandle>, Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        let info = structure.shape_info(edge)?;
        if info.has_sub_shape(vertex) || info.has_flag() {
            return Ok(None);
        }
        if structure.has_interference_between(vertex, edge)
            || structure.has_interference_shape_subshapes(vertex, edge)?
        {
            return Ok(None);
        }
        let Some(first) = structure.pave_blocks(edge)?.first() else {
            return Ok(None);
        };
        // A non-splittable first block marks a micro edge, which is ignored.
        if !first.lock().expect("pave-block lock").is_splittable() {
            return Ok(None);
        }
        Ok(Some(Arc::clone(first)))
    }

    fn apply_ve_solver(
        &mut self,
        solver: &VertexEdgeSolver,
        same_domain: &DataMap<Pair, Vec<i32>>,
        add_interfs: bool,
        edges: &mut Map<i32>,
    ) -> Result<(), Failure> {
        if solver.flag != 0 {
            if solver.failed {
                let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
                let vertex = structure.shape(solver.vertex)?.clone();
                let edge = structure.shape(solver.edge)?.clone();
                self.add_intersection_failed_warning(&vertex, &edge)?;
            }
            return Ok(());
        }
        let new_vertex = self.update_vertex(solver.vertex, solver.new_tolerance)?;
        let structure = self.data_structure.as_mut().ok_or_else(missing_structure)?;
        let target = structure.pave_blocks(solver.edge)?.iter().find(|block| {
            let (first, last) = block.lock().expect("pave-block lock").range();
            solver.parameter > first && solver.parameter < last
        });
        let Some(target) = target.cloned() else {
            return Ok(());
        };
        let pave = Pave::new(new_vertex, solver.parameter);
        target
            .lock()
            .expect("pave-block lock")
            .append_ext_pave(pave);
        edges.add(solver.edge);
        if !add_interfs {
            return Ok(());
        }
        let originals = same_domain
            .find(&Pair::new(solver.vertex, solver.edge))?
            .clone();
        let is_new = structure.is_new_shape(new_vertex);
        for original in originals {
            let record = structure.interf_ve().append(InterfVE::default());
            record.set_indices(original, solver.edge);
            record.set_parameter(solver.parameter);
            if is_new {
                record.set_index_new(new_vertex);
            }
            structure.add_interference(original, solver.edge);
        }
        Ok(())
    }
}
