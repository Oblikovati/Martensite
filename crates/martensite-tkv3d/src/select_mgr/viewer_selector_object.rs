//! The second BVH level: active sensitive entities and per-owner containment filtering.
use super::*;
use super::{
    frustum_box_probe::SelectMgrBoxProbe, viewer_selector_clipping::*,
    viewer_selector_object_transform::*, viewer_selector_walk::*,
};
use crate::select_basics::SelectBasicsSelectingVolumeManager;
use crate::select3d::Select3DSensitiveEntityMembers;
use martensite_tkernel::{
    ncollection::DataMap,
    standard::{Failure, Handle},
};
use martensite_tkmath::{
    bvh::{BvhTreeHandle, BvhVec3d},
    gp::{GeneralTransform3, TransformForm},
};
struct ObjectTraversal<'a> {
    base: &'a SelectMgrViewerSelectorBase,
    probe: Option<&'a SelectMgrBoxProbe>,
    object: &'a Handle<dyn SelectMgrSelectableObject>,
    global: &'a SelectMgrSelectingVolumeManager,
    view: &'a SelectMgrTraversalView,
    set: Handle<SelectMgrSensitiveEntitySet>,
    tree: BvhTreeHandle<BvhVec3d>,
    inverse: GeneralTransform3,
    local: SelectMgrSelectingVolumeManager,
    bypass_bounds: bool,
}
/// The scratch state that the objects of one traversal share: the volumes of `global`
/// scaled without a transformation, by scale, and the volume of the entity under test.
/// Native traverseObject builds both for each object; an untransformed scaled volume depends
/// only on `global` and the scale, so one traversal builds it once (#594).
#[derive(Default)]
pub(super) struct ObjectScratch {
    scaled: DataMap<i32, SelectMgrSelectingVolumeManager>,
    entity_volume: SelectMgrSelectingVolumeManager,
    /// The node stack of the entity tree walk, kept for its allocation.
    stack: Vec<i32>,
}

/// Native private traverseObject body over the actual entity BVH.
/// Example: `select_mgr_traverse_object(&base,&object,&volume,&view)?`.
pub fn select_mgr_traverse_object(
    base: &SelectMgrViewerSelectorBase,
    object: &Handle<dyn SelectMgrSelectableObject>,
    global: &SelectMgrSelectingVolumeManager,
    view: &SelectMgrTraversalView,
) -> Result<(), Failure> {
    traverse_object_with(
        base,
        object,
        (global, view),
        None,
        &mut ObjectScratch::default(),
    )
}

/// `select_mgr_traverse_object` with the scratch state of the traversal that visits `object`;
/// every object of one traversal shares `global` and `probe`, the box probe read from it.
pub(super) fn traverse_object_with(
    base: &SelectMgrViewerSelectorBase,
    object: &Handle<dyn SelectMgrSelectableObject>,
    (global, view): (&SelectMgrSelectingVolumeManager, &SelectMgrTraversalView),
    probe: Option<&SelectMgrBoxProbe>,
    scratch: &mut ObjectScratch,
) -> Result<(), Failure> {
    let set = base.object_sensitives.read().find(object)?.clone();
    if set.size() == 0 {
        return Ok(());
    }
    let bypass_bounds = (set.has_entity_with_persistence() && !view.camera().is_null())
        || set.has_entity_with_flipping();
    let tree = set.bvh()?;
    let Some(inverse) = selector_object_inverse(object, view)? else {
        return Ok(());
    };
    let local = if inverse.form() != TransformForm::Identity {
        global.scale_and_transform(1, &inverse, &Handle::default())?
    } else {
        global.clone()
    };
    let mut traversal = ObjectTraversal {
        base,
        probe,
        object,
        global,
        view,
        set,
        tree,
        inverse,
        local,
        bypass_bounds,
    };
    if !traversal.prepare_clipping()? {
        return Ok(());
    }
    let first_stored = base.fields.read().results.stored.extent() + 1;
    traversal.walk_entities(scratch)?;
    traversal.filter_incomplete_owners(first_stored)
}
impl ObjectTraversal<'_> {
    /// The box test of the object's volume, which shares the traversal's intersector unless
    /// the object has a transformation.
    fn boxes(&self) -> SelectorBoxTest<'_> {
        SelectorBoxTest::new(&self.local, self.probe)
    }
    fn prepare_clipping(&mut self) -> Result<bool, Failure> {
        if !self.bypass_bounds && !self.boxes().node_overlaps(&self.tree, 0)? {
            return Ok(false);
        }
        if !configure_object_clipping(self.object, self.global, &mut self.local)? {
            return Ok(false);
        }
        Ok(!selector_box_clipped(
            self.global,
            &selector_node_bounds(&self.tree, 0)?,
            false,
        ))
    }
    fn walk_entities(&self, scratch: &mut ObjectScratch) -> Result<(), Failure> {
        let mut stack = std::mem::take(&mut scratch.stack);
        stack.clear();
        stack.push(0);
        let walked = self.walk_entity_nodes(&mut stack, scratch);
        scratch.stack = stack;
        walked
    }
    fn walk_entity_nodes(
        &self,
        stack: &mut Vec<i32>,
        scratch: &mut ObjectScratch,
    ) -> Result<(), Failure> {
        let boxes = self.boxes();
        while let Some(node) = next_selector_leaf(&self.tree, stack, |_, bounds| {
            if self.bypass_bounds {
                Ok(true)
            } else {
                boxes.overlaps(bounds)
            }
        })? {
            let reject_partial = !self.base.selecting_volume.read().is_overlap_allowed();
            if selector_box_clipped(
                self.global,
                &selector_node_bounds(&self.tree, node)?,
                reject_partial,
            ) {
                continue;
            }
            let tree = self.tree.read();
            let range = tree.beg_primitive(node)?..=tree.end_primitive(node)?;
            drop(tree);
            for index in range {
                self.visit_entity(index, scratch)?;
            }
        }
        Ok(())
    }
    fn visit_entity(&self, index: i32, scratch: &mut ObjectScratch) -> Result<(), Failure> {
        let wrapper = self.set.get_sensitive_by_id(index)?;
        if !wrapper.is_active_for_selection() {
            return Ok(());
        }
        let entity = wrapper.base_sensitive();
        let Some(inverse) =
            selector_entity_inverse(&entity, self.object, &self.inverse, self.view)?
        else {
            return Ok(());
        };
        select_mgr_compute_frustum(
            self.base,
            &entity,
            self.global,
            &self.local,
            &inverse,
            &mut scratch.scaled,
            &mut scratch.entity_volume,
        )?;
        select_mgr_check_overlap(self.base, &entity, &inverse, &scratch.entity_volume)
    }
    fn filter_incomplete_owners(&self, first: usize) -> Result<(), Failure> {
        if self.base.selecting_volume.read().is_overlap_allowed()
            || !matches!(
                self.global.get_active_selection_type(),
                SelectMgrSelectionType::Box | SelectMgrSelectionType::Polyline
            )
        {
            return Ok(());
        }
        let last = self.base.fields.read().results.stored.extent();
        for index in (first..=last).rev() {
            let criterion = self
                .base
                .fields
                .read()
                .results
                .stored
                .find_from_index(index)?
                .clone();
            let owner = criterion.entity.owner_id();
            let count = self.set.owners().seek(&owner).copied().unwrap_or(0);
            if count > criterion.nb_owner_matches {
                self.base
                    .fields
                    .write()
                    .results
                    .stored
                    .remove_from_index(index)?;
            }
        }
        Ok(())
    }
}
