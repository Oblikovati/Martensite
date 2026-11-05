//! Actual candidate and first-block cursors retain their position when cancellation is observed.
use super::progress::EdgeEdgeBoundary;
use super::{BOPAlgoPaveFiller, EdgeEdgeTask, PaveBlockBoxes, RangeAndBox, pave_block_box};
use crate::bop_ds::PaveBlockHandle;
use martensite_tkbrep::topods::Edge;
use martensite_tkernel::standard::Failure;
use std::sync::Arc;

struct EdgeEdgePair {
    edges: [Edge; 2],
    blocks: [Vec<PaveBlockHandle>; 2],
}

struct PreparedEdgeBlock {
    block: PaveBlockHandle,
    box_range: RangeAndBox,
    bounds: (i32, i32),
}

impl BOPAlgoPaveFiller {
    /// Polls the actual pair and first-block cursors before their native work.
    /// Example: `filler.prepare_ee_tasks(boundary)?`.
    pub(super) fn prepare_ee_tasks(
        &mut self,
        boundary: &EdgeEdgeBoundary<'_, '_>,
    ) -> Result<Option<Vec<EdgeEdgeTask>>, Failure> {
        let mut boxes = PaveBlockBoxes::new();
        let mut tasks = Vec::new();
        while self.progress_pair_cursor()?.more() {
            if boundary.cancelled(&self.options) {
                return Ok(None);
            }
            let (first, second) = self.progress_pair_cursor()?.value()?;
            if !self.append_ee_pair((first, second), &mut boxes, &mut tasks, boundary)? {
                return Ok(None);
            }
            self.progress_pair_cursor()?.next();
        }
        Ok(Some(tasks))
    }

    fn append_ee_pair(
        &mut self,
        (first, second): (i32, i32),
        boxes: &mut PaveBlockBoxes,
        tasks: &mut Vec<EdgeEdgeTask>,
        boundary: &EdgeEdgeBoundary<'_, '_>,
    ) -> Result<bool, Failure> {
        let Some(pair) = self.edge_edge_pair(first, second)? else {
            return Ok(true);
        };
        for block in &pair.blocks[0] {
            if boundary.cancelled(&self.options) {
                return Ok(false);
            }
            if let Some(first) = PreparedEdgeBlock::from_block(&pair.edges[0], block, boxes)? {
                append_second_blocks(&pair, &first, boxes, tasks, self.options.fuzzy_value())?;
            }
        }
        Ok(true)
    }

    fn edge_edge_pair(&mut self, first: i32, second: i32) -> Result<Option<EdgeEdgePair>, Failure> {
        let structure = self.structure_mut()?;
        if structure.shape_info(first)?.has_flag() || structure.shape_info(second)?.has_flag() {
            return Ok(None);
        }
        let blocks1 = structure.change_pave_blocks(first)?.clone();
        if blocks1.is_empty() {
            return Ok(None);
        }
        let blocks2 = structure.change_pave_blocks(second)?.clone();
        if blocks2.is_empty() {
            return Ok(None);
        }
        let edges = [
            Edge::unchecked(structure.shape(first)?.clone()),
            Edge::unchecked(structure.shape(second)?.clone()),
        ];
        let blocks = [blocks1, blocks2];
        Ok(Some(EdgeEdgePair { edges, blocks }))
    }
}

impl PreparedEdgeBlock {
    fn from_block(
        edge: &Edge,
        block: &PaveBlockHandle,
        boxes: &mut PaveBlockBoxes,
    ) -> Result<Option<Self>, Failure> {
        let Some(box_range) = pave_block_box(edge, block, boxes)? else {
            return Ok(None);
        };
        Ok(Some(Self::from_box_range(block, box_range)))
    }

    fn from_box_range(block: &PaveBlockHandle, box_range: RangeAndBox) -> Self {
        let bounds = block.lock().expect("pave-block lock").indices();
        Self {
            block: Arc::clone(block),
            box_range,
            bounds,
        }
    }
}

fn append_second_blocks(
    pair: &EdgeEdgePair,
    first: &PreparedEdgeBlock,
    boxes: &mut PaveBlockBoxes,
    tasks: &mut Vec<EdgeEdgeTask>,
    fuzzy: f64,
) -> Result<(), Failure> {
    for block in &pair.blocks[1] {
        let Some(box_range) = pave_block_box(&pair.edges[1], block, boxes)? else {
            continue;
        };
        if first.box_range.1.is_out_box(&box_range.1) {
            continue;
        }
        let second = PreparedEdgeBlock::from_box_range(block, box_range);
        tasks.push(prepared_edge_task(pair, first, second, fuzzy));
    }
    Ok(())
}

fn prepared_edge_task(
    pair: &EdgeEdgePair,
    first: &PreparedEdgeBlock,
    second: PreparedEdgeBlock,
    fuzzy: f64,
) -> EdgeEdgeTask {
    let quick = first.bounds == second.bounds || first.bounds == (second.bounds.1, second.bounds.0);
    let mut task = EdgeEdgeTask::new(
        [Arc::clone(&first.block), second.block],
        pair.edges.clone(),
        [first.box_range.0, second.box_range.0],
    );
    task.configure(
        [first.box_range.1.clone(), second.box_range.1],
        fuzzy,
        quick,
    );
    task
}
