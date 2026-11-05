//! Preserve native VE edge/vertex filtering and same-domain deduplication before workers.
use super::task::VertexEdgeWork;
use super::{BOPAlgoPaveFiller, Pair, PaveBlockIndexLinks, missing_structure};
use crate::bop_algo::ProgressCancellation;
use crate::bop_algo::pave_filler::progress_observation::native_user_break;
use martensite_tkbrep::topods::{Edge, Vertex};
use martensite_tkernel::{
    message::ProgressScope,
    ncollection::{DataMap, Map},
    standard::Failure,
};

impl BOPAlgoPaveFiller {
    /// Prepares the original ordered solvers; None records the native edge-loop stop.
    /// Example: `filler.prepare_ve_work(pairs, same_domain, scope, observed)?`.
    pub(super) fn prepare_ve_work(
        &self,
        pairs: &PaveBlockIndexLinks,
        same_domain: &mut DataMap<Pair, Vec<i32>>,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Option<Vec<VertexEdgeWork>>, Failure> {
        let mut work = Vec::new();
        for index in 1..=pairs.extent() {
            if native_user_break(&self.options, scope, observed) {
                return Ok(None);
            }
            self.append_ve_edge_work(pairs, index, same_domain, &mut work)?;
        }
        Ok(Some(work))
    }

    fn append_ve_edge_work(
        &self,
        pairs: &PaveBlockIndexLinks,
        index: usize,
        same_domain: &mut DataMap<Pair, Vec<i32>>,
        work: &mut Vec<VertexEdgeWork>,
    ) -> Result<(), Failure> {
        let edge = ve_edge_index(pairs, index)?;
        let bounding = self.ve_bounding_vertices(edge)?;
        for vertex in pairs.find_from_index(index)?.iter().copied() {
            if let Some(candidate) =
                self.prepare_ve_candidate(vertex, edge, &bounding, same_domain)?
            {
                work.push(candidate);
            }
        }
        Ok(())
    }

    fn ve_bounding_vertices(&self, edge: i32) -> Result<Map<i32>, Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        let mut bounding: Map<i32> = Map::new(1);
        for block in structure.pave_blocks(edge)? {
            let (first, last) = block.lock().expect("pave-block lock").indices();
            bounding.add(first);
            bounding.add(last);
        }
        Ok(bounding)
    }

    fn prepare_ve_candidate(
        &self,
        vertex: i32,
        edge: i32,
        bounding: &Map<i32>,
        same_domain: &mut DataMap<Pair, Vec<i32>>,
    ) -> Result<Option<VertexEdgeWork>, Failure> {
        let representative = self.ve_representative(vertex)?;
        if bounding.contains(&representative) {
            return Ok(None);
        }
        let key = Pair::new(representative, edge);
        if let Some(list) = same_domain.change_seek(&key) {
            list.push(vertex);
            return Ok(None);
        }
        same_domain.bind(key, vec![vertex]);
        self.new_ve_work(representative, edge).map(Some)
    }

    fn ve_representative(&self, vertex: i32) -> Result<i32, Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        let mut representative = vertex;
        structure.has_shape_sd_into(vertex, &mut representative);
        Ok(representative)
    }

    fn new_ve_work(&self, vertex: i32, edge: i32) -> Result<VertexEdgeWork, Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        let shapes = (
            Vertex::unchecked(structure.shape(vertex)?.clone()),
            Edge::unchecked(structure.shape(edge)?.clone()),
        );
        Ok(VertexEdgeWork::new(
            vertex,
            edge,
            shapes,
            self.options.fuzzy_value(),
        ))
    }
}

fn ve_edge_index(pairs: &PaveBlockIndexLinks, index: usize) -> Result<i32, Failure> {
    let block = pairs.find_key(index)?;
    let edge = block.lock().expect("pave-block lock").original_edge();
    Ok(edge)
}
