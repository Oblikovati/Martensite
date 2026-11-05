//! Only the ordered receiver mutates the interference graph after workers have joined.
use super::progress::EdgeEdgeBoundary;
use super::{BOPAlgoPaveFiller, EdgeEdgeTask, EdgePairBlocks, VertexCouples, original_edge};
use crate::bop_algo::tools::{PaveBlockLinks, fill_map, pave_block_links};
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::{ncollection::Map, standard::Failure};
use martensite_tkg3d::top_abs::ShapeEnum;
use std::sync::Arc;

pub(super) struct EdgeEdgeStore {
    pub(super) links: PaveBlockLinks,
    pub(super) couples: VertexCouples,
    pub(super) edges: Map<i32>,
}

impl EdgeEdgeStore {
    fn new() -> Self {
        Self {
            links: pave_block_links(),
            couples: VertexCouples::with_hasher(ShapeMapHasher, 100),
            edges: Map::new(1),
        }
    }
}

impl BOPAlgoPaveFiller {
    /// Stores joined task outputs in input order at native result/part checkpoints.
    /// Example: `filler.store_ee_tasks(tasks, boundary)?`.
    pub(super) fn store_ee_tasks(
        &mut self,
        tasks: &[EdgeEdgeTask],
        boundary: &EdgeEdgeBoundary<'_, '_>,
    ) -> Result<Option<EdgeEdgeStore>, Failure> {
        let mut stored = EdgeEdgeStore::new();
        for task in tasks {
            if boundary.cancelled(&self.options) {
                return Ok(None);
            }
            if !self.analyze_edge_edge_task(task, &mut stored, boundary)? {
                return Ok(None);
            }
        }
        Ok(Some(stored))
    }

    fn analyze_edge_edge_task(
        &mut self,
        task: &EdgeEdgeTask,
        stored: &mut EdgeEdgeStore,
        boundary: &EdgeEdgeBoundary<'_, '_>,
    ) -> Result<bool, Failure> {
        if !self.edge_task_has_parts(task)? {
            return Ok(true);
        }
        let blocks = EdgePairBlocks::new(&task.block1, &task.block2);
        let analytical = self.is_line_circle_pair(&blocks)?;
        for part in &task.parts {
            if boundary.cancelled(&self.options) {
                return Ok(false);
            }
            self.store_ee_part(task, part, &blocks, analytical, stored)?;
        }
        Ok(true)
    }

    fn edge_task_has_parts(&mut self, task: &EdgeEdgeTask) -> Result<bool, Failure> {
        if task.failed() {
            self.warn_failed_edge_task(task)?;
            return Ok(false);
        }
        Ok(!task.parts.is_empty())
    }

    fn store_ee_vertex(
        &mut self,
        part: &crate::int_tools::CommonPrt,
        blocks: &EdgePairBlocks,
        analytical: bool,
        stored: &mut EdgeEdgeStore,
    ) -> Result<(), Failure> {
        self.analyze_ee_vertex(
            part,
            blocks,
            analytical,
            &mut stored.couples,
            &mut stored.edges,
        )
    }

    fn warn_failed_edge_task(&mut self, task: &EdgeEdgeTask) -> Result<(), Failure> {
        let structure = self.structure_mut()?;
        let first = structure.shape(original_edge(&task.block1))?.clone();
        let second = structure.shape(original_edge(&task.block2))?.clone();
        self.add_intersection_failed_warning(&first, &second)
    }

    fn store_ee_part(
        &mut self,
        task: &EdgeEdgeTask,
        part: &crate::int_tools::CommonPrt,
        blocks: &EdgePairBlocks,
        analytical: bool,
        stored: &mut EdgeEdgeStore,
    ) -> Result<(), Failure> {
        match part.shape_type() {
            ShapeEnum::Vertex => self.store_ee_vertex(part, blocks, analytical, stored)?,
            ShapeEnum::Edge => self.store_common_edge_part(task, part, blocks, &mut stored.links),
            _ => {}
        }
        Ok(())
    }

    fn store_common_edge_part(
        &mut self,
        task: &EdgeEdgeTask,
        part: &crate::int_tools::CommonPrt,
        blocks: &EdgePairBlocks,
        links: &mut PaveBlockLinks,
    ) {
        // The native `break` leaves the switch only: the next part is still read.
        if task.parts.len() > 1 {
            return;
        }
        if !same_edge_block_bounds(task) {
            return;
        }
        self.append_ee_record(blocks.edges, part);
        fill_map(links, Arc::clone(&task.block1), Arc::clone(&task.block2));
    }
}

fn same_edge_block_bounds(task: &EdgeEdgeTask) -> bool {
    let first = task.block1.lock().expect("pave-block lock");
    first.has_same_bounds(&task.block2.lock().expect("pave-block lock"))
}
