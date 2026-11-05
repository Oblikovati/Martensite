//! Preparation preserves polls after actual shrunk-data filling and before eligible face selection.
use super::*;
type PreparedForcedTree = (BoxSet<BvhVec3d>, PaveBlockIndexMap);
impl BOPAlgoPaveFiller {
    /// Preserve primary source-edge polling before reading each eligible block list.
    /// Example: `filler.collect_forced_ef_blocks(boundary)?` does not initialize missing lists.
    pub(super) fn collect_forced_ef_blocks(
        &self,
        boundary: &EdgeFaceBoundary<'_, '_>,
    ) -> Result<Option<PaveBlockIndexMap>, Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        let mut blocks = PaveBlockIndexMap::default();
        for index in 0..structure.nb_source_shapes() {
            let info = structure.shape_info(index)?;
            if info.shape_type() != ShapeEnum::Edge || !info.has_reference() || info.has_flag() {
                continue;
            }
            if boundary.cancelled(&self.options) {
                return Ok(None);
            }
            for block in structure.pave_blocks(index)? {
                blocks.add(structure.real_pave_block(block)?);
            }
        }
        Ok(Some(blocks))
    }
    /// A box tree of the blocks with valid shrunk data, keyed by their 1-based index.
    /// Example: `filler.forced_block_tree(blocks, boundary)?` polls after actual preparation.
    pub(super) fn forced_block_tree(
        &mut self,
        blocks: &PaveBlockIndexMap,
        boundary: &EdgeFaceBoundary<'_, '_>,
    ) -> Result<Option<PreparedForcedTree>, Failure> {
        let mut tree = BoxSet::default();
        let mut prepared = PaveBlockIndexMap::default();
        for block in blocks.iter() {
            if !self.ensure_forced_ef_shrunk_data(block)? {
                continue;
            }
            if boundary.cancelled(&self.options) {
                return Ok(None);
            }
            let bounds = block
                .lock()
                .expect("pave-block lock")
                .shrunk_data()
                .2
                .clone();
            let index = prepared.add(Arc::clone(block)) as i32;
            tree.add(index, box3_to_bvh(&bounds)?);
        }
        tree.build()?;
        Ok(Some((tree, prepared)))
    }

    fn ensure_forced_ef_shrunk_data(&mut self, block: &PaveBlockHandle) -> Result<bool, Failure> {
        // Validation reacquires this block; release the probe guard first.
        let has_shrunk = block.lock().expect("pave-block lock").has_shrunk_data();
        let valid = has_shrunk
            && self
                .data_structure
                .as_ref()
                .ok_or_else(missing_structure)?
                .is_valid_shrunk_data(block)?;
        if valid {
            return Ok(true);
        }
        self.fill_shrunk_data_block(block)?;
        Ok(block.lock().expect("pave-block lock").has_shrunk_data())
    }

    /// Keep the face check before selection, then prepare only its eligible source blocks.
    /// Example: `filler.append_forced_ef_tasks(face, tree, blocks, check_mode, tasks, boundary)?`.
    pub(super) fn append_forced_ef_tasks(
        &mut self,
        face: i32,
        tree: &mut BoxSet<BvhVec3d>,
        prepared: &PaveBlockIndexMap,
        check_mode: bool,
        tasks: &mut Vec<EdgeFaceTask>,
        boundary: &EdgeFaceBoundary<'_, '_>,
    ) -> Result<bool, Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        let info = structure.shape_info(face)?;
        if info.shape_type() != ShapeEnum::Face || !info.has_reference() {
            return Ok(true);
        }
        if boundary.cancelled(&self.options) {
            return Ok(false);
        }
        let selected = forced_ef_box_candidates(tree, info.box_bounds())?;
        let face_shape = Face::unchecked(info.shape().clone());
        let face_info = structure.face_info(face)?;
        let face_vertices = face_vertex_union(face_info);
        for index in selected {
            let block = Arc::clone(prepared.find_key(index as usize)?);
            if !forced_ef_block_available(face_info, &face_vertices, &block) {
                continue;
            }
            if let Some(task) = self.forced_ef_task(face, &face_shape, &block, check_mode)? {
                tasks.push(task);
            }
        }
        Ok(true)
    }
}

fn forced_ef_box_candidates(
    tree: &mut BoxSet<BvhVec3d>,
    bounds: &martensite_tkmath::bnd::BoundingBox3,
) -> Result<Vec<i32>, Failure> {
    let mut selector = BoxSelector::default();
    selector.set_box(box3_to_bvh(bounds)?);
    selector.set_bvh_set(Some(tree));
    if selector.select()? == 0 {
        return Ok(Vec::new());
    }
    Ok(selector.indices().iter().copied().collect())
}
fn forced_ef_block_available(
    info: &FaceInfo,
    vertices: &Map<i32>,
    block: &PaveBlockHandle,
) -> bool {
    if in_face_maps(info, block) {
        return false;
    }
    let (first, last) = block.lock().expect("pave-block lock").indices();
    vertices.contains(&first) && vertices.contains(&last)
}
