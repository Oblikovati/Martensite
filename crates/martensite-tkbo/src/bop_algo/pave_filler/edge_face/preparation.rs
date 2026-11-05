//! Candidate guards follow the actual cursor and each real pave-block boundary.
use super::progress::EdgeFaceBoundary;
use super::*;
use martensite_tkmath::bnd::BoundingBox3;

struct EdgeFaceBlockSettings {
    range: (f64, f64),
    quick: bool,
}

struct EdgeFaceCandidate {
    indices: (i32, i32),
    edge: Edge,
    face: Face,
    boxes: [BoundingBox3; 2],
    fuzzy: f64,
}

impl BOPAlgoPaveFiller {
    /// Builds tasks only after each real candidate and block checkpoint.
    /// Example: `let tasks = filler.prepare_ef_tasks(boundary)?;`.
    pub(super) fn prepare_ef_tasks(
        &mut self,
        boundary: &EdgeFaceBoundary<'_, '_>,
    ) -> Result<Option<Vec<EdgeFaceTask>>, Failure> {
        let mut boxes = PaveBlockBoxes::new();
        let mut tasks = Vec::new();
        while self.progress_pair_cursor()?.more() {
            if boundary.cancelled(&self.options) {
                return Ok(None);
            }
            let (edge, face) = self.progress_pair_cursor()?.value()?;
            if !self.append_edge_face_tasks(edge, face, &mut boxes, &mut tasks, boundary)? {
                return Ok(None);
            }
            self.progress_pair_cursor()?.next();
        }
        Ok(Some(tasks))
    }

    /// Initializes the native GlueFull face records without progress polls.
    /// Example: `filler.initialize_glued_ef_faces()?;`.
    pub(super) fn initialize_glued_ef_faces(&mut self) -> Result<(), Failure> {
        while self.progress_pair_cursor()?.more() {
            let (edge, face) = self.progress_pair_cursor()?.value()?;
            let structure = self.structure_mut()?;
            if !structure.shape_info(edge)?.has_flag() {
                structure.change_face_info(face)?;
            }
            self.progress_pair_cursor()?.next();
        }
        Ok(())
    }

    fn edge_face_candidate(
        &mut self,
        edge: i32,
        face: i32,
    ) -> Result<Option<EdgeFaceCandidate>, Failure> {
        let fuzzy = self.options.fuzzy_value();
        let structure = self.structure_mut()?;
        if structure.shape_info(edge)?.has_flag() {
            return Ok(None);
        }
        let candidate = EdgeFaceCandidate::from_structure(structure, (edge, face), fuzzy)?;
        structure.change_face_info(face)?;
        Ok(Some(candidate))
    }

    fn append_edge_face_tasks(
        &mut self,
        edge: i32,
        face: i32,
        boxes: &mut PaveBlockBoxes,
        tasks: &mut Vec<EdgeFaceTask>,
        boundary: &EdgeFaceBoundary<'_, '_>,
    ) -> Result<bool, Failure> {
        let Some(candidate) = self.edge_face_candidate(edge, face)? else {
            return Ok(true);
        };
        let blocks = self.structure_mut()?.change_pave_blocks(edge)?.clone();
        for block in blocks {
            if boundary.cancelled(&self.options) {
                return Ok(false);
            }
            self.append_ef_block_task(&candidate, &block, boxes, tasks)?;
        }
        Ok(true)
    }

    fn edge_face_block_task(
        &mut self,
        candidate: &EdgeFaceCandidate,
        block: &PaveBlockHandle,
        boxes: &mut PaveBlockBoxes,
    ) -> Result<Option<EdgeFaceTask>, Failure> {
        let Some(settings) = self.ef_block_settings(candidate, block, boxes)? else {
            return Ok(None);
        };
        candidate
            .configured_task(block, settings.range, settings.quick)
            .map(Some)
    }

    fn ef_block_settings(
        &mut self,
        candidate: &EdgeFaceCandidate,
        block: &PaveBlockHandle,
        boxes: &mut PaveBlockBoxes,
    ) -> Result<Option<EdgeFaceBlockSettings>, Failure> {
        let Some(real) = self.ef_block_not_on_face(candidate.indices.1, block)? else {
            return Ok(None);
        };
        let Some((range, block_box)) = pave_block_box(&candidate.edge, block, boxes)? else {
            return Ok(None);
        };
        if candidate.boxes[1].is_out_box(&block_box) {
            return Ok(None);
        }
        let quick = self.ef_known_block_ends(candidate.indices.1, &real)?;
        Ok(Some(EdgeFaceBlockSettings { range, quick }))
    }

    fn ef_known_block_ends(&mut self, face: i32, block: &PaveBlockHandle) -> Result<bool, Failure> {
        let info = self.structure_mut()?.face_info(face)?;
        let (first, last) = block.lock().expect("pave-block lock").indices();
        let known = |vertex: i32| {
            info.vertices_in().contains(vertex) || info.vertices_on().contains(vertex)
        };
        Ok(known(first) && known(last))
    }

    fn ef_block_not_on_face(
        &mut self,
        face: i32,
        block: &PaveBlockHandle,
    ) -> Result<Option<PaveBlockHandle>, Failure> {
        let structure = self.structure_mut()?;
        let real = structure.real_pave_block(block)?;
        let info = structure.face_info(face)?;
        if info.pave_blocks_on().contains(&real) {
            return Ok(None);
        }
        Ok(Some(real))
    }

    fn append_ef_block_task(
        &mut self,
        candidate: &EdgeFaceCandidate,
        block: &PaveBlockHandle,
        boxes: &mut PaveBlockBoxes,
        tasks: &mut Vec<EdgeFaceTask>,
    ) -> Result<(), Failure> {
        if let Some(task) = self.edge_face_block_task(candidate, block, boxes)? {
            tasks.push(task);
            self.mark_face_block_done(candidate.indices.1, block);
        }
        Ok(())
    }
}

impl EdgeFaceCandidate {
    fn from_structure(
        structure: &crate::bop_ds::DataStructure,
        indices: (i32, i32),
        fuzzy: f64,
    ) -> Result<Self, Failure> {
        Ok(Self {
            indices,
            edge: Edge::unchecked(structure.shape(indices.0)?.clone()),
            face: Face::unchecked(structure.shape(indices.1)?.clone()),
            boxes: [
                structure.shape_info(indices.0)?.box_bounds().clone(),
                structure.shape_info(indices.1)?.box_bounds().clone(),
            ],
            fuzzy,
        })
    }

    fn configured_task(
        &self,
        block: &PaveBlockHandle,
        range: (f64, f64),
        quick: bool,
    ) -> Result<EdgeFaceTask, Failure> {
        let shrunk = shrunk_or_whole(block, range);
        let mut task = EdgeFaceTask::new(self.indices, block, &self.edge, &self.face);
        let mut new_shrunk = shrunk;
        correct_range_face(&self.edge, &self.face, &shrunk, &mut new_shrunk)?;
        task.new_shrunk_range = new_shrunk;
        let block_range = self.corrected_block_range(range)?;
        task.configure(self.boxes.clone(), self.fuzzy, quick, block_range);
        Ok(task)
    }

    fn corrected_block_range(&self, range: (f64, f64)) -> Result<Range, Failure> {
        let source = Range::new(range.0, range.1);
        let mut corrected = source;
        correct_range_face(&self.edge, &self.face, &source, &mut corrected)?;
        Ok(corrected)
    }
}
