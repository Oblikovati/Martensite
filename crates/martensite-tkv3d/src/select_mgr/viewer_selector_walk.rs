//! Left-first native BVH traversal, releasing tree guards before client callbacks.
use super::{SelectMgrSelectingVolumeManager, frustum_box_probe::SelectMgrBoxProbe};
use crate::select_basics::SelectBasicsSelectingVolumeManager;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::bvh::{BvhTree, BvhTreeHandle, BvhVec3d};
use martensite_tkservice::graphic3d::Graphic3dBndBox3d;
pub(super) fn selector_node_bounds(
    tree: &BvhTreeHandle<BvhVec3d>,
    node: i32,
) -> Result<Graphic3dBndBox3d, Failure> {
    node_bounds_in(&tree.read(), node)
}
/// The bounds of `node` in a tree the caller already holds.
fn node_bounds_in(tree: &BvhTree<BvhVec3d>, node: i32) -> Result<Graphic3dBndBox3d, Failure> {
    Ok(Graphic3dBndBox3d::from_corners(
        *tree.min_point(node)?,
        *tree.max_point(node)?,
    ))
}
/// The box test of one walk: the probe of the traversal's volume when `volume` reads it,
/// otherwise the live test of `volume` (#594).
pub(super) struct SelectorBoxTest<'a> {
    volume: &'a SelectMgrSelectingVolumeManager,
    probe: Option<&'a SelectMgrBoxProbe>,
}
impl<'a> SelectorBoxTest<'a> {
    /// Keep `probe` only for the volume it was read from; `SelectorBoxTest::new(&local, probe)`.
    pub(super) fn new(
        volume: &'a SelectMgrSelectingVolumeManager,
        probe: Option<&'a SelectMgrBoxProbe>,
    ) -> Self {
        let probe = probe.filter(|probe| probe.reads(volume));
        Self { volume, probe }
    }
    /// Whether the volume overlaps `bounds`; `test.overlaps(&bounds)?`.
    pub(super) fn overlaps(&self, bounds: &Graphic3dBndBox3d) -> Result<bool, Failure> {
        match self.probe {
            Some(probe) => probe.overlaps(bounds),
            None => selector_bounds_overlap(bounds, self.volume),
        }
    }
    /// Whether the volume overlaps the bounds of `node`; `test.node_overlaps(&tree, 0)?`.
    pub(super) fn node_overlaps(
        &self,
        tree: &BvhTreeHandle<BvhVec3d>,
        node: i32,
    ) -> Result<bool, Failure> {
        self.overlaps(&selector_node_bounds(tree, node)?)
    }
}
/// Whether `volume` overlaps the node bounds that `next_selector_leaf` passes to its test.
pub(super) fn selector_bounds_overlap(
    bounds: &Graphic3dBndBox3d,
    volume: &SelectMgrSelectingVolumeManager,
) -> Result<bool, Failure> {
    volume.overlaps_box_inside(bounds.corner_min(), bounds.corner_max(), None)
}
/// Pops nodes until a leaf, pushing the children that `intersects` accepts, left on top.
/// The children's bounds are read under the guard that reads the node, so a test does not
/// lock the tree again (#594); `intersects` runs after the guard is released.
pub(super) fn next_selector_leaf(
    tree: &BvhTreeHandle<BvhVec3d>,
    stack: &mut Vec<i32>,
    mut intersects: impl FnMut(i32, &Graphic3dBndBox3d) -> Result<bool, Failure>,
) -> Result<Option<i32>, Failure> {
    while let Some(node) = stack.pop() {
        let current = tree.read();
        if current.is_outer(node)? {
            return Ok(Some(node));
        }
        let (left, right) = (current.child::<0>(node)?, current.child::<1>(node)?);
        let bounds = (
            node_bounds_in(&current, left)?,
            node_bounds_in(&current, right)?,
        );
        drop(current);
        let left_inside = intersects(left, &bounds.0)?;
        let right_inside = intersects(right, &bounds.1)?;
        if right_inside {
            stack.push(right);
        }
        if left_inside {
            stack.push(left);
        }
    }
    Ok(None)
}

#[cfg(test)]
#[path = "viewer_selector_walk_tests.rs"]
mod tests;
