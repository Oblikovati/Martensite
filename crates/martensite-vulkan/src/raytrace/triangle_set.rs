//! GlTriangleSet: one primitive array's vertices, normals, texture
//! coordinates and triangles with material ids, in object space under the structure transform,
//! and its quad BVH (BVHBinnedBuilder collapsed by CollapseToQuadTree).
use crate::{RenderError, error::rejected};
use martensite_tkernel::ncollection::{Mat4, Vec4};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::bvh::{
    BvhBinnedBuilder, BvhBox, BvhBuilder, BvhQuadTree, BvhSet, BvhTransform, BvhTree, BvhVec3f,
    BvhVector, cwise_std_max, cwise_std_min,
};

/// `BVHConstants_NbBinsBest`, `LeafNodeSizeAverage` and `MaxTreeDepth` of GlView's
/// `myRaytraceBVHBuilder`.
const NB_BINS_BEST: usize = 48;
const LEAF_NODE_SIZE_AVERAGE: i32 = 4;
pub(crate) const MAX_TREE_DEPTH: i32 = 32;

/// One ray-traced primitive array.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct RaytraceTriangleSet {
    pub vertices: Vec<BvhVec3f>,
    pub normals: Vec<[f32; 3]>,
    pub tex_coords: Vec<[f32; 2]>,
    /// Vertex indices and the material id of each triangle (`BVHVec4i`).
    pub elements: Vec<[i32; 4]>,
    /// The structure transformation (`BVHTransform`), identity without one.
    pub transform: Mat4<f32>,
}

impl RaytraceTriangleSet {
    fn corner(&self, index: i32, corner: usize) -> Result<BvhVec3f, Failure> {
        let element = usize::try_from(index)
            .ok()
            .and_then(|index| self.elements.get(index))
            .ok_or_else(|| {
                Failure::out_of_range(&format!("triangle {index} of {}", self.elements.len()))
            })?;
        let vertex = usize::try_from(element[corner]).ok();
        vertex
            .and_then(|vertex| self.vertices.get(vertex))
            .copied()
            .ok_or_else(|| {
                Failure::out_of_range(&format!(
                    "vertex {} of {}",
                    element[corner],
                    self.vertices.len()
                ))
            })
    }
    /// The untransformed box of every triangle (`BVHPrimitiveSet::Box`).
    fn primitive_box(&self) -> Result<BvhBox<BvhVec3f>, Failure> {
        martensite_tkmath::bvh::combined_element_boxes(self)
    }
    /// `GlTriangleSet::Box`: the eight corners of the primitive box under the transform.
    /// Example: `set.transformed_box?.corner_max.x`.
    pub fn transformed_box(&self) -> Result<BvhBox<BvhVec3f>, Failure> {
        let source = self.primitive_box()?;
        let (min, max) = (source.corner_min(), source.corner_max());
        let mut result = BvhBox::new();
        for corner in 0..8 {
            let pick =
                |bit: usize, low: f32, high: f32| if corner >> bit & 1 == 0 { low } else { high };
            let point = Vec4::new(
                pick(2, min.x, max.x),
                pick(1, min.y, max.y),
                pick(0, min.z, max.z),
                1.,
            );
            let moved = self.transform * point;
            result.add(&BvhVec3f::new(moved.x, moved.y, moved.z));
        }
        Ok(result)
    }
    /// The inverse transform the shader moves rays with (`BVHTransform::Inversed`).
    pub fn inverse_transform(&self) -> Mat4<f32> {
        let mut transform = BvhTransform::default();
        transform.set_transform(self.transform);
        *transform.inversed()
    }
    /// `QuadBVH()`: the binned SAH tree over the transformed root box, collapsed to four-wide
    /// nodes. Example: `set.quad_bvh()?.length()`.
    pub fn quad_bvh(&mut self) -> Result<BvhTree<BvhVec3f, BvhQuadTree>, RenderError> {
        let failure = |error: Failure| rejected(error.to_string(), "a ray-tracing triangle BVH");
        let builder = BvhBinnedBuilder::<BvhVec3f, NB_BINS_BEST>::new(
            LEAF_NODE_SIZE_AVERAGE,
            MAX_TREE_DEPTH,
            false,
            1,
        );
        let root = self.transformed_box().map_err(failure)?;
        let mut tree = BvhTree::default();
        builder.build(self, &mut tree, &root).map_err(failure)?;
        tree.collapse_to_quad_tree().map_err(failure)
    }
}

impl BvhSet<BvhVec3f> for RaytraceTriangleSet {
    fn size(&self) -> i32 {
        self.elements.len() as i32
    }
    fn element_box(&self, index: i32) -> Result<BvhBox<BvhVec3f>, Failure> {
        let (a, b, c) = (
            self.corner(index, 0)?,
            self.corner(index, 1)?,
            self.corner(index, 2)?,
        );
        let (mut min, mut max) = (a, a);
        cwise_std_min(&mut min, &b);
        cwise_std_min(&mut min, &c);
        cwise_std_max(&mut max, &b);
        cwise_std_max(&mut max, &c);
        Ok(BvhBox::from_corners(min, max))
    }
    /// `GlTriangleSet::Center`: the middle of the triangle's extent along the axis.
    fn center(&self, index: i32, axis: i32) -> Result<f32, Failure> {
        let mut values = [0.; 3];
        for (corner, value) in values.iter_mut().enumerate() {
            *value = self.corner(index, corner)?.component(axis);
        }
        let [a, b, c] = values;
        Ok((a.min(b).min(c) + a.max(b).max(c)) * 0.5)
    }
    fn swap(&mut self, first: i32, second: i32) -> Result<(), Failure> {
        let count = self.elements.len();
        let index = |value: i32| {
            usize::try_from(value)
                .ok()
                .filter(|index| *index < count)
                .ok_or_else(|| Failure::out_of_range(&format!("triangle {value} of {count}")))
        };
        let (first, second) = (index(first)?, index(second)?);
        self.elements.swap(first, second);
        Ok(())
    }
    fn bounding_box(&self) -> Result<BvhBox<BvhVec3f>, Failure> {
        self.transformed_box()
    }
}

#[cfg(test)]
#[path = "triangle_set_tests.rs"]
mod tests;
