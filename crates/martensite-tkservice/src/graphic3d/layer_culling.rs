//! The frustum culling of `Graphic3d_Layer`: `updateBVH` and `UpdateCulling`.
use super::bvh_cstructure_set::Graphic3dTrsfPersView;
use super::layer::Graphic3dLayer;
use super::{
    Graphic3dCStructure, Graphic3dCStructureMembers as _, Graphic3dCullingContext,
    Graphic3dCullingTool, Graphic3dRenderingParamsFrustumCulling as Culling,
};
use martensite_tkernel::ncollection::Vec3;
use martensite_tkernel::standard::{Failure, Handle, WeakHandle};
use martensite_tkmath::bvh::{BvhPrimitiveSetAccess as _, BvhTree, BvhVec3d};

/// `NodeInStack`: a node and whether it is completely inside.
#[derive(Clone, Copy, Default)]
struct NodeInStack {
    id: i32,
    is_full_inside: bool,
}

/// The culling inputs of one traversal.
struct CullingPass<'a> {
    selector: &'a Graphic3dCullingTool,
    ctx: Graphic3dCullingContext,
}

impl CullingPass<'_> {
    /// `IsCulled` of a node box, clearing `inside` when the node is not fully inside.
    fn culled(&self, tree: &BvhTree<BvhVec3d>, node: &mut NodeInStack) -> Result<bool, Failure> {
        let (min, max) = (*tree.min_point(node.id)?, *tree.max_point(node.id)?);
        Ok(self
            .selector
            .is_culled(&self.ctx, &min, &max, Some(&mut node.is_full_inside)))
    }
    /// `IsTooSmall` of a node box.
    fn too_small(&self, tree: &BvhTree<BvhVec3d>, node: &NodeInStack) -> Result<bool, Failure> {
        let (min, max): (Vec3<f64>, Vec3<f64>) =
            (*tree.min_point(node.id)?, *tree.max_point(node.id)?);
        Ok(self.selector.is_too_small(&self.ctx, &min, &max))
    }
    /// Whether the two children are in, updating their "fully inside" flags.
    fn children_in(
        &self,
        tree: &BvhTree<BvhVec3d>,
        node: &NodeInStack,
        left: &mut NodeInStack,
        right: &mut NodeInStack,
    ) -> Result<(bool, bool), Failure> {
        if node.is_full_inside {
            return Ok((!self.too_small(tree, left)?, !self.too_small(tree, right)?));
        }
        let left_in = !self.culled(tree, left)?;
        left.is_full_inside &= left_in;
        let right_in = !self.culled(tree, right)?;
        right.is_full_inside &= right_in;
        Ok((left_in, right_in))
    }
}

impl Graphic3dLayer {
    /// `updateBVH`: after an invalidation, files every structure again as always rendered or
    /// in a BVH set; `layer.update_bvh()`.
    pub fn update_bvh(&self) {
        let structures = {
            let mut state = self.state.write();
            if !state.is_bvh_primitives_needs_reset {
                return;
            }
            state.always_rendered.clear(true);
            state.is_bvh_primitives_needs_reset = false;
            state
                .array
                .iter()
                .flat_map(|map| map.iter().cloned().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        };
        let mut sets = self.bvh_sets();
        sets.primitives.clear();
        sets.trsf_pers.clear();
        for structure in structures.iter().map(WeakHandle::upgrade) {
            let Some(value) = structure.get() else {
                continue;
            };
            if value.is_always_rendered() {
                value.mark_as_not_culled();
                self.state
                    .write()
                    .always_rendered
                    .add(structure.downgrade());
            } else if value.transform_persistence().is_null() {
                sets.primitives.add(&structure);
            } else {
                sets.trsf_pers.add(&structure);
            }
        }
    }
    /// `UpdateCulling`: marks the structures of the BVH sets culled or not for a view.
    /// Example: `layer.update_culling(0, &selector, Graphic3dRenderingParamsFrustumCulling::On)?`.
    pub fn update_culling(
        &self,
        view_id: i32,
        selector: &Graphic3dCullingTool,
        state: Culling,
    ) -> Result<(), Failure> {
        self.update_bvh();
        let nb_structures = self.state.read().nb_structures;
        self.state.write().nb_structures_not_culled = nb_structures;
        let empty = {
            let sets = self.bvh_sets();
            sets.primitives.structures().extent() == 0 && sets.trsf_pers.structures().extent() == 0
        };
        if state != Culling::On || empty {
            if state != Culling::NoUpdate {
                self.mark_all_culled(state == Culling::On);
            }
            return Ok(());
        }
        let always = self.state.read().always_rendered.extent() as i32;
        let settings = self.layer_settings();
        let mut ctx = Graphic3dCullingContext::default();
        selector.set_culling_distance(&mut ctx, settings.culling_distance());
        selector.set_culling_size(&mut ctx, settings.culling_size());
        let pass = CullingPass { selector, ctx };
        let reached = [
            self.reached_leaves(&pass, false)?,
            self.reached_leaves(&pass, true)?,
        ];
        let visible = self.mark_reached(&reached, view_id);
        self.state.write().nb_structures_not_culled = always + visible;
        Ok(())
    }
    /// Sets the culled flag of every structure of both BVH sets.
    fn mark_all_culled(&self, culled: bool) {
        let sets = self.bvh_sets();
        let all = sets
            .primitives
            .structures()
            .iter()
            .chain(sets.trsf_pers.structures().iter());
        for structure in all.map(WeakHandle::upgrade) {
            if let Some(structure) = structure.get() {
                structure.set_culled(culled);
            }
        }
    }
    /// Which leaves of one BVH set the traversal reaches, by structure id.
    fn reached_leaves(
        &self,
        pass: &CullingPass<'_>,
        is_trsf_pers: bool,
    ) -> Result<Vec<bool>, Failure> {
        let mut sets = self.bvh_sets();
        let count = if is_trsf_pers {
            sets.trsf_pers.structures().extent()
        } else {
            sets.primitives.structures().extent()
        };
        let mut reached = vec![false; count];
        if count == 0 {
            return Ok(reached);
        }
        let tree = if is_trsf_pers {
            let camera = pass.selector.camera().clone();
            let view = Graphic3dTrsfPersView {
                camera: &camera,
                projection: pass.selector.projection_matrix(),
                world_view: pass.selector.world_view_matrix(),
                viewport: [
                    pass.selector.viewport_width(),
                    pass.selector.viewport_height(),
                ],
                state: pass.selector.world_view_proj_state(),
            };
            sets.trsf_pers.bvh(&view)?.clone()
        } else {
            sets.primitives.bvh()?.clone()
        };
        drop(sets);
        for id in self.traverse(pass, &tree)? {
            if let Some(leaf) = usize::try_from(id).ok().and_then(|id| reached.get_mut(id)) {
                *leaf = true;
            }
        }
        Ok(reached)
    }
    /// Marks each structure of both sets culled unless the traversal reached it and it is
    /// visible in the view, writing only the flags that change: one write per structure and
    /// frame took 4% of a 1,000-structure Vulkan frame (#594). Returns how many it marked
    /// visible.
    fn mark_reached(&self, reached: &[Vec<bool>; 2], view_id: i32) -> i32 {
        let sets = self.bvh_sets();
        let primitive = |id| sets.primitives.get_structure_by_id(id);
        let persistent = |id| sets.trsf_pers.get_structure_by_id(id);
        mark_set(&reached[0], primitive, view_id) + mark_set(&reached[1], persistent, view_id)
    }
    /// The native stack traversal: the primitive indices of the reached leaves, in order;
    /// the queue order alternates between the left and the right child.
    fn traverse(
        &self,
        pass: &CullingPass<'_>,
        tree: &BvhTree<BvhVec3d>,
    ) -> Result<Vec<i32>, Failure> {
        let mut node = NodeInStack {
            id: 0,
            is_full_inside: true,
        };
        let mut leaves = Vec::new();
        if pass.culled(tree, &mut node)? {
            return Ok(leaves);
        }
        let mut stack: Vec<NodeInStack> = Vec::new();
        loop {
            if tree.is_outer(node.id)? {
                leaves.extend(tree.beg_primitive(node.id)?..=tree.end_primitive(node.id)?);
            } else if let Some(next) = self.descend(pass, tree, &node, &mut stack)? {
                node = next;
                continue;
            }
            match stack.pop() {
                Some(next) => node = next,
                None => break,
            }
        }
        Ok(leaves)
    }
    /// The next node below an inner node (`None` when both children are culled), pushing the
    /// second child when both are in.
    fn descend(
        &self,
        pass: &CullingPass<'_>,
        tree: &BvhTree<BvhVec3d>,
        node: &NodeInStack,
        stack: &mut Vec<NodeInStack>,
    ) -> Result<Option<NodeInStack>, Failure> {
        let mut left = NodeInStack {
            id: tree.child::<0>(node.id)?,
            is_full_inside: true,
        };
        let mut right = NodeInStack {
            id: tree.child::<1>(node.id)?,
            is_full_inside: true,
        };
        let (left_in, right_in) = pass.children_in(tree, node, &mut left, &mut right)?;
        if left_in && right_in {
            let mut state = self.state.write();
            let left_first = state.bvh_is_left_child_queued_first;
            stack.push(if left_first { right } else { left });
            state.bvh_is_left_child_queued_first = !left_first;
            return Ok(Some(if left_first { left } else { right }));
        }
        if left_in || right_in {
            return Ok(Some(if left_in { left } else { right }));
        }
        Ok(None)
    }
}

/// A visible ordinary structure and its sole group after the culling guards are released.
pub struct Graphic3dBatchEntry {
    /// The structure's display identifier.
    pub id: i32,
    /// The actual culled structure if requested, otherwise a null handle.
    pub structure: Handle<dyn Graphic3dCStructure>,
    /// The sole group captured from the same structure lock.
    pub group: Handle<dyn super::Graphic3dGroup>,
}

impl Graphic3dLayer {
    /// Collect eligible visible structures along with their groups.
    /// Example: `layer.collect_batch_groups(view_id, &selector, true, &mut groups)?`.
    pub fn collect_batch_groups(
        &self,
        view_id: i32,
        selector: &Graphic3dCullingTool,
        include_structure: bool,
        groups: &mut Vec<Graphic3dBatchEntry>,
    ) -> Result<Option<usize>, Failure> {
        self.update_bvh();
        if !self.state.read().always_rendered.is_empty() {
            return Ok(None);
        }
        let settings = self.layer_settings();
        let mut ctx = Graphic3dCullingContext::default();
        selector.set_culling_distance(&mut ctx, settings.culling_distance());
        selector.set_culling_size(&mut ctx, settings.culling_size());
        let pass = CullingPass { selector, ctx };
        let mut sets = self.bvh_sets();
        if sets.trsf_pers.structures().extent() != 0 {
            return Ok(None);
        }
        let count = sets.primitives.structures().extent();
        if count == 0 {
            return Ok(Some(0));
        }
        let tree = sets.primitives.bvh()?;
        let leaf_ids = self.traverse(&pass, &tree)?;
        drop(tree);
        let mut reached = vec![false; count];
        for id in leaf_ids {
            let Some(reached) = usize::try_from(id).ok().and_then(|id| reached.get_mut(id)) else {
                return Ok(None);
            };
            *reached = true;
        }
        let mut visible = 0;
        let mut compatible = true;
        for (weak, reached) in sets.primitives.structures().iter().zip(reached) {
            let lease = weak.upgrade();
            let Some(structure) = lease.get() else {
                continue;
            };
            let mut fields = structure.cstructure_base().fields.write();
            let displayed = reached
                && fields.visible
                && fields
                    .view_affinity
                    .get()
                    .is_none_or(|a| a.is_visible(view_id));
            fields.is_culled = !displayed;
            if displayed {
                visible += 1;
                if let Some(group) = ordinary_probe_group(&fields) {
                    let (id, group) = (fields.id, group.clone());
                    drop(fields);
                    let structure = if include_structure {
                        lease
                    } else {
                        Handle::default()
                    };
                    groups.push(Graphic3dBatchEntry {
                        id,
                        structure,
                        group,
                    });
                } else {
                    compatible = false;
                }
            }
        }
        drop(sets);
        let mut state = self.state.write();
        state.nb_structures_not_culled = visible as i32;
        Ok(compatible.then_some(visible))
    }
}

/// Marks the structures of one BVH set as `Layer::mark_reached` describes; the number shown.
fn mark_set(
    reached: &[bool],
    structure_at: impl Fn(i32) -> Handle<dyn Graphic3dCStructure>,
    view_id: i32,
) -> i32 {
    let mut visible = 0;
    for (id, reached) in reached.iter().enumerate() {
        let structure = structure_at(id as i32);
        let Some(structure) = structure.get() else {
            continue;
        };
        let shown = *reached && structure.is_visible_in_view(view_id);
        if structure.is_culled() == shown {
            structure.set_culled(!shown);
        }
        visible += i32::from(shown);
    }
    visible
}

type ProbeStructure = WeakHandle<dyn Graphic3dCStructure>;

impl Graphic3dLayer {
    /// Capture structure identities in draw priority order; `layer.batch_structure_order()`.
    pub fn batch_structure_order(&self) -> Vec<(usize, ProbeStructure)> {
        probe_order(&self.state.read())
            .map(|(priority, source)| (priority, source.clone()))
            .collect()
    }

    /// Compare current submission order with a previous capture; `layer.matches_batch_structure_order(&order)`.
    pub fn matches_batch_structure_order(&self, order: &[(usize, ProbeStructure)]) -> bool {
        probe_order(&self.state.read())
            .eq(order.iter().map(|(priority, source)| (*priority, source)))
    }
}

fn probe_order(state: &super::layer::LayerState) -> impl Iterator<Item = (usize, &ProbeStructure)> {
    state
        .array
        .iter()
        .enumerate()
        .flat_map(|(priority, sources)| sources.iter().map(move |source| (priority, source)))
}

fn ordinary_probe_group(
    fields: &super::Graphic3dCStructureFields,
) -> Option<&Handle<dyn super::Graphic3dGroup>> {
    if fields.highlight
        || fields.has_group_flipping
        || fields.has_group_trsf
        || !fields.trsf_pers.is_null()
        || !fields.clip_planes.is_null()
        || fields
            .trsf
            .get()
            .is_some_and(|datum| !datum.transformation().is_identity())
        || fields.groups.len() != 1
    {
        return None;
    }
    fields.groups.first().map(|slot| &**slot)
}
