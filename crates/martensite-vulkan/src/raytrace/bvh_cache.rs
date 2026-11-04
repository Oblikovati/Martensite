//! Bottom-level BVH reuse across frames (GlView::updateRaytraceGeometry keeps the trees of
//! unchanged structures). A triangle set's tree depends only on its vertices, triangles and
//! transform (the builder starts from the transformed box), so a tree is reused when a set of
//! the next frame equals the input it was built from; the packed buffers are then identical to
//! a fresh build. Entries a build does not use are dropped.
use super::triangle_set::RaytraceTriangleSet;
use crate::RenderError;
use martensite_tkmath::bvh::{BvhQuadTree, BvhTree, BvhVec3f};

/// A triangle set's quad BVH.
type QuadTree = BvhTree<BvhVec3f, BvhQuadTree>;

/// A built bottom-level tree: the set as given, the set after the build reordered its
/// triangles, and the tree with its leaves marked.
#[derive(Clone)]
struct BottomTree {
    input: RaytraceTriangleSet,
    built: RaytraceTriangleSet,
    tree: QuadTree,
}

/// The bottom-level trees of the last build; `RaytraceBvhCache::default()`.
#[derive(Clone, Default)]
pub(crate) struct RaytraceBvhCache {
    trees: Vec<BottomTree>,
    /// Trees the last build computed rather than reused.
    pub built_last: usize,
    /// The structure states of the view's last rebuild (the per-frame CHECK of the counters).
    pub structure_check: super::structure_check::RaytraceStructureCheck,
}

impl std::fmt::Debug for RaytraceBvhCache {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RaytraceBvhCache")
            .field("trees", &self.trees.len())
            .field("built_last", &self.built_last)
            .finish()
    }
}

impl RaytraceBvhCache {
    /// The built set and tree of every set, reusing equal inputs of the previous build;
    /// `cache.bottom_trees(sets, build)?`.
    pub fn bottom_trees(
        &mut self,
        sets: Vec<RaytraceTriangleSet>,
        build: impl Fn(&mut RaytraceTriangleSet) -> Result<QuadTree, RenderError>,
    ) -> Result<Vec<(RaytraceTriangleSet, QuadTree)>, RenderError> {
        let previous = std::mem::take(&mut self.trees);
        self.built_last = 0;
        let mut results = Vec::with_capacity(sets.len());
        for set in sets {
            let entry = match previous.iter().find(|entry| entry.input == set) {
                Some(entry) => entry.clone(),
                None => {
                    self.built_last += 1;
                    let mut built = set.clone();
                    let tree = build(&mut built)?;
                    BottomTree {
                        input: set,
                        built,
                        tree,
                    }
                }
            };
            results.push((entry.built.clone(), entry.tree.clone()));
            self.trees.push(entry);
        }
        Ok(results)
    }
}
