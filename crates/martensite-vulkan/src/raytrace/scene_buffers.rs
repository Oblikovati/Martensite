//! GlRaytraceGeometry::ProcessAcceleration and GlView::uploadRaytraceData: the two-level quad BVH over the triangle sets and the texel-buffer contents the
//! ray-tracing shaders read (uScene*Texture, uGeometry*Texture).
use super::triangle_set::{MAX_TREE_DEPTH, RaytraceTriangleSet};
use crate::{RenderError, error::rejected};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::bvh::{
    BvhBinnedBuilder, BvhBox, BvhBuilder, BvhQuadTree, BvhSet, BvhTree, BvhVec3f,
};

/// `BVHGeometry`'s default builder: 32 bins, one object per leaf.
const NB_BINS_OPTIMAL: usize = 32;
const LEAF_NODE_SIZE_SINGLE: i32 = 1;

/// The packed scene: node records, node boxes and inverse transforms of the scene BVH, then the
/// concatenated vertices, normals, texture coordinates and triangles.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct RaytraceSceneBuffers {
    pub node_info: Vec<[i32; 4]>,
    pub min_points: Vec<[f32; 3]>,
    pub max_points: Vec<[f32; 3]>,
    /// Column-major inverse transforms, one per triangle set in BVH object order.
    pub transforms: Vec<[f32; 16]>,
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub tex_coords: Vec<[f32; 2]>,
    pub triangles: Vec<[i32; 4]>,
    /// `TopLevelTreeDepth() + BotLevelTreeDepth()`, the traversal stack the shader needs.
    pub tree_depth: i32,
    /// The world bounds of every set (`myRaytraceGeometry.Box()`).
    pub bounds: BvhBox<BvhVec3f>,
}

fn bvh_failure(error: Failure) -> RenderError {
    rejected(error.to_string(), "a ray-tracing scene BVH")
}

/// Build both tree levels and pack the buffers; empty sets are dropped first, as GlView
/// keeps only sets with triangles. Example: `RaytraceSceneBuffers::build(sets)?.node_info.len`.
pub(crate) fn build(
    sets: Vec<RaytraceTriangleSet>,
    cache: &mut super::bvh_cache::RaytraceBvhCache,
) -> Result<RaytraceSceneBuffers, RenderError> {
    let sets: Vec<_> = sets
        .into_iter()
        .filter(|set| !set.elements.is_empty())
        .collect();
    let built = cache.bottom_trees(sets, |set| {
        let mut tree = set.quad_bvh()?;
        mark_bottom_leaves(&mut tree)?;
        Ok(tree)
    })?;
    let (sets, bottom): (Vec<_>, Vec<_>) = built.into_iter().unzip();
    let bottom_depth = bottom.iter().map(|tree| tree.depth()).fold(1, i32::max);
    let boxes = sets
        .iter()
        .map(|set| set.transformed_box())
        .collect::<Result<Vec<_>, _>>();
    let mut objects = TaggedObjects {
        tags: (0..sets.len()).collect(),
        boxes: boxes.map_err(bvh_failure)?,
    };
    let bounds = objects.bounding_box().map_err(bvh_failure)?;
    let top = top_level_tree(&mut objects, &bounds)?;
    // The build's swaps reorder the objects (BVHObjectSet::Swap); each set keeps its tree.
    let sets = permuted(sets, &objects.tags);
    let bottom = permuted(bottom, &objects.tags);
    let mut buffers = RaytraceSceneBuffers {
        tree_depth: top.depth() + bottom_depth,
        bounds,
        ..Default::default()
    };
    buffers.append_tree(&top);
    buffers.link_leaves(&sets, &bottom, top)?;
    Ok(buffers)
}

/// The bottom trees mark their leaves with x = -1 (ProcessAcceleration).
fn mark_bottom_leaves(tree: &mut BvhTree<BvhVec3f, BvhQuadTree>) -> Result<(), RenderError> {
    for node in 0..tree.length() {
        if tree.is_outer(node).map_err(bvh_failure)? {
            tree.node_info_buffer_mut()[node as usize].x = -1;
        }
    }
    Ok(())
}

/// `myRaytraceGeometry.QuadBVH()`: the binned top-level tree over the object boxes.
fn top_level_tree(
    objects: &mut TaggedObjects,
    bounds: &BvhBox<BvhVec3f>,
) -> Result<BvhTree<BvhVec3f, BvhQuadTree>, RenderError> {
    let builder = BvhBinnedBuilder::<BvhVec3f, NB_BINS_OPTIMAL>::new(
        LEAF_NODE_SIZE_SINGLE,
        MAX_TREE_DEPTH,
        false,
        1,
    );
    let mut tree = BvhTree::default();
    builder
        .build(objects, &mut tree, bounds)
        .map_err(bvh_failure)?;
    tree.collapse_to_quad_tree().map_err(bvh_failure)
}

/// `items` in the order of `tags`, the original index of each slot.
fn permuted<T>(items: Vec<T>, tags: &[usize]) -> Vec<T> {
    let mut slots: Vec<Option<T>> = items.into_iter().map(Some).collect();
    tags.iter()
        .map(|&tag| slots[tag].take().expect("a permutation of the objects"))
        .collect()
}

/// Object boxes with the original index of each slot.
struct TaggedObjects {
    boxes: Vec<BvhBox<BvhVec3f>>,
    tags: Vec<usize>,
}

impl BvhSet<BvhVec3f> for TaggedObjects {
    fn size(&self) -> i32 {
        self.boxes.len() as i32
    }
    fn element_box(&self, index: i32) -> Result<BvhBox<BvhVec3f>, Failure> {
        usize::try_from(index)
            .ok()
            .and_then(|index| self.boxes.get(index))
            .copied()
            .ok_or_else(|| {
                Failure::out_of_range(&format!("object {index} of {}", self.boxes.len()))
            })
    }
    fn center(&self, index: i32, axis: i32) -> Result<f32, Failure> {
        Ok(self.element_box(index)?.center_axis(axis))
    }
    fn swap(&mut self, first: i32, second: i32) -> Result<(), Failure> {
        self.element_box(first)?;
        self.element_box(second)?;
        self.boxes.swap(first as usize, second as usize);
        self.tags.swap(first as usize, second as usize);
        Ok(())
    }
}

impl RaytraceSceneBuffers {
    fn append_tree(&mut self, tree: &BvhTree<BvhVec3f, BvhQuadTree>) {
        let info = tree.node_info_buffer().iter();
        self.node_info.extend(info.map(|n| [n.x, n.y, n.z, n.w]));
        let (min, max) = (
            tree.min_point_buffer().iter(),
            tree.max_point_buffer().iter(),
        );
        self.min_points.extend(min.map(|p| [p.x, p.y, p.z]));
        self.max_points.extend(max.map(|p| [p.x, p.y, p.z]));
    }
    /// Rewrite each top-level leaf to `(object + 1, BVH, vertex, element offsets)` and append
    /// that object's tree and geometry in leaf order.
    fn link_leaves(
        &mut self,
        sets: &[RaytraceTriangleSet],
        trees: &[BvhTree<BvhVec3f, BvhQuadTree>],
        top: BvhTree<BvhVec3f, BvhQuadTree>,
    ) -> Result<(), RenderError> {
        self.transforms = sets
            .iter()
            .map(|set| set.inverse_transform().data())
            .collect();
        for node in 0..top.length() {
            if !top.is_outer(node).map_err(bvh_failure)? {
                continue;
            }
            let object = top.beg_primitive(node).map_err(bvh_failure)?;
            let (set, tree) = (&sets[object as usize], &trees[object as usize]);
            self.node_info[node as usize] = [
                object + 1,
                self.node_info.len() as i32,
                self.vertices.len() as i32,
                self.triangles.len() as i32,
            ];
            self.append_tree(tree);
            self.append_geometry(set);
        }
        Ok(())
    }
    fn append_geometry(&mut self, set: &RaytraceTriangleSet) {
        self.vertices
            .extend(set.vertices.iter().map(|v| [v.x, v.y, v.z]));
        self.normals.extend(&set.normals);
        self.tex_coords.extend(&set.tex_coords);
        self.triangles.extend(&set.elements);
    }
}

#[cfg(test)]
#[path = "scene_buffers_tests.rs"]
mod tests;
