//! `BOPAlgoPaveFiller_7`: split edges of pave blocks (`MakeSplitEdges`), p-curves of
//! split and section edges (`MakePCurves`) and planar p-curves of the inputs (`Prepare`).
use super::{BOPAlgoPaveFiller, missing_structure};
use crate::bop_algo::tools::compute_tolerance_of_cb;
use crate::bop_ds::{CommonBlockHandle, CommonBlockHandleIdentity, PaveBlockHandle, ShapeInfo};
use crate::bop_tools::make_split_edge;
use martensite_tkbrep::topods::{Edge, Vertex};
use martensite_tkernel::ncollection::Map;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;

use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;
use std::sync::Arc;

/// `BOPAlgoSplitEdge`: one split of an edge between two paves.
struct SplitEdgeTask {
    edge: Edge,
    vertices: [Vertex; 2],
    parameters: [f64; 2],
    block: PaveBlockHandle,
    common: Option<CommonBlockHandle>,
    split: Edge,
    bounds: BoundingBox3,
    tolerance: f64,
}

impl BOPAlgoPaveFiller {
    /// Makes the split edges of all pave blocks (one per common block), reusing an untouched
    /// edge when possible (`MakeSplitEdges`); `filler.make_split_edges()?`.
    pub fn make_split_edges(&mut self) -> Result<(), Failure> {
        self.make_split_edges_with_progress(
            &martensite_tkernel::message::ProgressRange::default(),
            &crate::bop_algo::ProgressCancellation::default(),
        )
        .map(|_| ())
    }

    fn split_edge_task(
        &mut self,
        block: &PaveBlockHandle,
        list_length: usize,
        fence: &mut Map<CommonBlockHandle, CommonBlockHandleIdentity>,
    ) -> Result<Option<SplitEdgeTask>, Failure> {
        let structure = self.structure_mut()?;
        let original = block.lock().expect("pave-block lock").original_edge();
        if structure.shape_info(original)?.has_flag() {
            // Degenerated edges are split by ProcessDE.
            return Ok(None);
        }
        let common = structure.common_block(block);
        if let Some(common) = &common
            && !fence.add(Arc::clone(common))
        {
            return Ok(None);
        }
        let (first, last) = block.lock().expect("pave-block lock").indices();
        if !structure.is_new_shape(first)
            && !structure.is_new_shape(last)
            && (!self.non_destructive || common.is_none())
            && self.reuse_untouched_edge(block, common.as_ref(), list_length)?
        {
            return Ok(None);
        }
        let (block, edge) = match &common {
            Some(common) => {
                let real = Arc::clone(common.lock().expect("common-block lock").pave_block1()?);
                let edge = real.lock().expect("pave-block lock").original_edge();
                (real, edge)
            }
            None => (Arc::clone(block), original),
        };
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        let ((first, last), range) = {
            let locked = block.lock().expect("pave-block lock");
            (locked.indices(), locked.range())
        };
        Ok(Some(SplitEdgeTask {
            edge: Edge::unchecked(structure.shape(edge)?.oriented(Orientation::Forward)),
            vertices: [
                Vertex::unchecked(structure.shape(first)?.oriented(Orientation::Forward)),
                Vertex::unchecked(structure.shape(last)?.oriented(Orientation::Reversed)),
            ],
            parameters: [range.0, range.1],
            block,
            common,
            // Native SetData seeds diagnostic state before the worker can observe a break.
            split: Edge::unchecked(structure.shape(edge)?.oriented(Orientation::Forward)),
            bounds: BoundingBox3::default(),
            tolerance: 0.,
        }))
    }

    /// No new vertices: a common block reuses a member edge that was not split, a lone block
    /// keeps its own edge. Returns true when no split is made.
    fn reuse_untouched_edge(
        &mut self,
        block: &PaveBlockHandle,
        common: Option<&CommonBlockHandle>,
        list_length: usize,
    ) -> Result<bool, Failure> {
        let Some(common) = common else {
            if list_length == 1 {
                let original = block.lock().expect("pave-block lock").original_edge();
                block.lock().expect("pave-block lock").set_edge(original);
                return Ok(true);
            }
            return Ok(false);
        };
        let members = common
            .lock()
            .expect("common-block lock")
            .pave_blocks()
            .to_vec();
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        let mut found = None;
        for member in members {
            let edge = member.lock().expect("pave-block lock").original_edge();
            if structure.pave_blocks(edge)?.len() == 1 {
                found = Some((member, edge));
                break;
            }
        }
        let Some((member, edge)) = found else {
            return Ok(false);
        };
        {
            let mut locked = common.lock().expect("common-block lock");
            locked.set_real_pave_block(&member);
            locked.set_edge(edge);
        }
        let context = self.context.clone();
        let tolerance = compute_tolerance_of_cb(
            Some(common),
            self.data_structure.as_ref().ok_or_else(missing_structure)?,
            context.get(),
        )?;
        self.update_edge_tolerance(edge, tolerance)?;
        Ok(true)
    }

    fn store_split_edge(&mut self, task: SplitEdgeTask) -> Result<(), Failure> {
        let (first, last) = task.block.lock().expect("pave-block lock").indices();
        let mut info = ShapeInfo::default();
        info.set_shape_type(ShapeEnum::Edge);
        info.set_shape(&task.split.into_shape());
        info.set_box(&task.bounds);
        info.change_sub_shapes().extend([first, last]);
        let index = self.structure_mut()?.append_shape_info(&info);
        match task.common {
            Some(common) => {
                self.update_edge_tolerance(index, task.tolerance)?;
                common.lock().expect("common-block lock").set_edge(index);
            }
            None => task.block.lock().expect("pave-block lock").set_edge(index),
        }
        Ok(())
    }
}

#[path = "split_edges/progress.rs"]
mod progress;
