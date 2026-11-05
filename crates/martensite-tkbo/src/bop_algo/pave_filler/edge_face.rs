//! `BOPAlgoPaveFiller_5`: edge/face intersection (`PerformEF`) and its task class.
use super::edge_edge::{PaveBlockBoxes, VertexCouples, pave_block_box};
use super::{BOPAlgoPaveFiller, EdgeRangeDistance, Glue, missing_structure};
use crate::bop_algo::tools::{
    PaveBlockIndexLinks, fill_map_one, pave_block_index_links, perform_common_blocks_on_faces,
};
use crate::bop_ds::{CoupleOfPaveBlocks, InterfEF, Pair, PaveBlockHandle, VertexIndexSet};
use crate::bop_tools::{compute_vertices, correct_range_face, make_new_vertex_edge_face};
use crate::int_tools::{CommonPrt, Range, Tools};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::brep_adaptor::BRepAdaptorCurve;
use martensite_tkbrep::topods::{Edge, Face, Vertex};
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::ncollection::Map;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::{Adaptor3dCurve, Adaptor3dSurface};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::geom_abs::{CurveType, SurfaceType};
use std::sync::Arc;

#[path = "edge_face/analysis.rs"]
mod analysis;
#[path = "edge_face/execution.rs"]
mod execution;
#[path = "edge_face/forced.rs"]
mod forced;
#[path = "edge_face/perform.rs"]
mod perform;
#[path = "edge_face/preparation.rs"]
mod preparation;
#[path = "edge_face/progress.rs"]
mod progress;
#[path = "edge_face/task.rs"]
mod task;
#[path = "edge_face/task_progress.rs"]
mod task_progress;
pub(super) use task::EdgeFaceTask;

/// The state shared by the analysis of the EF tasks (native locals of PerformEF).
struct EdgeFaceResults {
    faces: Map<i32>,
    couples: VertexCouples,
    blocks_on_faces: PaveBlockIndexLinks,
}

impl BOPAlgoPaveFiller {
    /// Saves the face/pave block pair to avoid its forced intersection later (`myFPBDone`).
    fn mark_face_block_done(&mut self, face: i32, block: &PaveBlockHandle) {
        let done = self.fpb_done.entry(face).or_default();
        if !done.iter().any(|known| Arc::ptr_eq(known, block)) {
            done.push(Arc::clone(block));
        }
    }

    fn analyze_ef_vertex(
        &mut self,
        task: &EdgeFaceTask,
        part: &CommonPrt,
        bounds: &EfBounds,
        results: &mut EdgeFaceResults,
    ) -> Result<(), Failure> {
        let mut parameter = 0.;
        Tools::vertex_parameter(part, &mut parameter);
        let mut vertex = Vertex::default();
        make_new_vertex_edge_face(&task.edge, parameter, &task.face, &mut vertex)?;
        let common = *part.range1();
        let mut on_pave = [
            Tools::is_in_range(&bounds.sides[0], &common, 5.0e-8),
            Tools::is_in_range(&bounds.sides[1], &common, 5.0e-8),
        ];
        if (on_pave[0] && on_pave[1]) || (bounds.line_plane && (on_pave[0] || on_pave[1])) {
            let face = task.face_index;
            if self.face_has_vertex(face, bounds.vertices.0)?
                && self.face_has_vertex(face, bounds.vertices.1)?
            {
                let mut edge_part = part.clone();
                edge_part.set_type(ShapeEnum::Edge);
                self.append_ef_record(task, Some(&edge_part));
                results.faces.add(task.face_index);
                fill_map_one(
                    &mut results.blocks_on_faces,
                    Arc::clone(&task.block),
                    task.face_index,
                );
                return Ok(());
            }
        }
        if !bounds.splittable {
            return Ok(());
        }
        let ends = [bounds.vertices.0, bounds.vertices.1];
        // The face maps are read live: ForceInterfVF may add the first end to the In vertices.
        for (position, flag) in on_pave.iter_mut().enumerate() {
            if *flag && !self.face_has_vertex(task.face_index, ends[position])? {
                *flag = self.force_interf_vf(ends[position], task.face_index)?;
            }
        }
        if on_pave[0] || on_pave[1] {
            return self.extend_existing_ef_vertices(task, &vertex, on_pave, ends);
        }
        let (face_on, _) = self.face_vertex_sets(task.face_index)?;
        if self.vertex_near_face_vertices(&vertex, &face_on)? {
            return Ok(());
        }
        let mut new_tolerance = BRepTool::vertex_tolerance(&vertex)?;
        new_tolerance = new_tolerance.max(bounds.tolerances.0.max(bounds.tolerances.1));
        BRepBuilder::default().update_vertex_tolerance(&vertex, new_tolerance)?;
        if bounds.line_plane {
            // Line/Plane hits get half the common range, applied only at the next intersection.
            new_tolerance = new_tolerance.max((common.last() - common.first()) / 2.);
        }
        let point = BRepTool::pnt(&vertex)?;
        if !self
            .context_ref()?
            .is_point3d_in_face(point, &task.face, new_tolerance)?
        {
            return Ok(());
        }
        results.faces.add(task.face_index);
        let index = self.append_ef_record(task, Some(part));
        let mut couple = CoupleOfPaveBlocks::new();
        couple.set_pave_blocks(Some(Arc::clone(&task.block)), Some(Arc::clone(&task.block)));
        couple.set_interference_index(index);
        couple.set_tolerance(new_tolerance);
        results.couples.add(vertex.into_shape(), couple);
        Ok(())
    }

    /// The new point is close to a block end: grows that vertex when the point really lies on
    /// the face (within 1e4 of the vertex tolerance, at most 0.1 for small tolerances).
    fn extend_existing_ef_vertices(
        &mut self,
        task: &EdgeFaceTask,
        vertex: &Vertex,
        on_pave: [bool; 2],
        ends: [i32; 2],
    ) -> Result<(), Failure> {
        let point = BRepTool::pnt(vertex)?;
        let projector = self.context_ref()?.proj_ps(&task.face)?;
        let distance = {
            let mut projection = projector.write();
            projection.perform(point)?;
            if projection.is_done() && projection.nb_points()? > 0 {
                projection.lower_distance()?
            } else {
                precision::infinite()
            }
        };
        if distance >= precision::intersection() {
            return Ok(());
        }
        for (position, flag) in on_pave.iter().enumerate() {
            if !*flag {
                continue;
            }
            let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
            let existing = Vertex::unchecked(structure.shape(ends[position])?.clone());
            let gap = BRepTool::pnt(&existing)?.distance(&point);
            let tolerance = BRepTool::vertex_tolerance(&existing)?;
            let mut maximum = 1.0e4 * tolerance;
            if tolerance < 0.01 {
                maximum = maximum.min(0.1);
            }
            if gap < maximum {
                self.update_vertex(ends[position], gap)?;
                if !self.vertices_to_avoid_extension.contains(&ends[position]) {
                    self.vertices_to_avoid_extension.push(ends[position]);
                }
            }
        }
        Ok(())
    }

    fn analyze_ef_edge(
        &mut self,
        task: &EdgeFaceTask,
        part: &CommonPrt,
        vertices: (i32, i32),
        results: &mut EdgeFaceResults,
    ) -> Result<(), Failure> {
        results.faces.add(task.face_index);
        let face = task.face_index;
        if !self.face_has_vertex(face, vertices.0)? || !self.face_has_vertex(face, vertices.1)? {
            self.append_ef_record(task, None);
            return Ok(());
        }
        self.append_ef_record(task, Some(part));
        fill_map_one(
            &mut results.blocks_on_faces,
            Arc::clone(&task.block),
            task.face_index,
        );
        Ok(())
    }

    /// Native `CheckFacePaves(aVnew, aMIF)`: true when the new vertex interferes with one of
    /// the face's On vertices.
    fn vertex_near_face_vertices(
        &self,
        vertex: &Vertex,
        face_on: &VertexIndexSet,
    ) -> Result<bool, Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        for index in face_on.iter().copied() {
            let other = Vertex::unchecked(structure.shape(index)?.clone());
            if compute_vertices(vertex, &other, 0.)? == 0 {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Native `CheckFacePaves(nV, aMIFOn, aMIFIn)` on the live face record.
    fn face_has_vertex(&mut self, face: i32, vertex: i32) -> Result<bool, Failure> {
        let info = self.structure_mut()?.change_face_info(face)?;
        Ok(info.vertices_on().contains(vertex) || info.vertices_in().contains(vertex))
    }

    fn face_vertex_sets(&mut self, face: i32) -> Result<(VertexIndexSet, VertexIndexSet), Failure> {
        let info = self.structure_mut()?.change_face_info(face)?;
        let mut on = VertexIndexSet::default();
        let mut inner = VertexIndexSet::default();
        for vertex in info.vertices_on().iter().copied() {
            on.add(vertex);
        }
        for vertex in info.vertices_in().iter().copied() {
            inner.add(vertex);
        }
        Ok((on, inner))
    }

    /// Appends an EF record (with its common part when given) and its interference pair.
    fn append_ef_record(&mut self, task: &EdgeFaceTask, part: Option<&CommonPrt>) -> i32 {
        let structure = self
            .data_structure
            .as_mut()
            .expect("PerformEF runs on an initialized DS");
        let record = structure.interf_ef().append(InterfEF::default());
        record.set_indices(task.edge_index, task.face_index);
        if let Some(part) = part {
            record.set_common_part(part);
        }
        let index = structure.interf_ef().len() as i32 - 1;
        structure.add_interference(task.edge_index, task.face_index);
        index
    }

    fn record_distance(&mut self, key: Pair, distance: EdgeRangeDistance) {
        if let Some(list) = self.distances.change_seek(&key) {
            list.push(distance);
            return;
        }
        self.distances.bind(key, vec![distance]);
    }

    pub(super) fn context_ref(&self) -> Result<&crate::int_tools::Context, Failure> {
        self.context.get().ok_or_else(missing_structure)
    }
}

/// Values of the pave block of an EF task used by every common part.
struct EfBounds {
    vertices: (i32, i32),
    splittable: bool,
    sides: [Range; 2],
    line_plane: bool,
    tolerances: (f64, f64),
}

fn shrunk_or_whole(block: &PaveBlockHandle, range: (f64, f64)) -> Range {
    let locked = block.lock().expect("pave-block lock must not be poisoned");
    if locked.has_shrunk_data() {
        let (first, last, _, _) = locked.shrunk_data();
        return Range::new(first, last);
    }
    Range::new(range.0, range.1)
}

#[cfg(test)]
#[path = "edge_face/progress_contract.rs"]
mod progress_contract;
#[cfg(test)]
#[path = "edge_face/progress_fixture.rs"]
mod progress_fixture;

#[cfg(test)]
#[path = "edge_face/census.rs"]
mod census;
