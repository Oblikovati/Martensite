//! `bnd`: conversion of Bnd boxes to BVH boxes.
use super::{BoundingBox2, BoundingBox3};
use crate::bvh::{BvhBox, BvhVec2d, BvhVec3d};
use martensite_tkernel::standard::Failure;

/// `bnd::box2_to_bvh`: copies the gap-adjusted limits (±1e100 on open
/// sides) into BVH corners; a void box raises `construction failure` like `Get`.
/// Example: `bnd::box2_to_bvh(&bounds)?.corner_min()`.
pub fn box2_to_bvh(bounds: &BoundingBox2) -> Result<BvhBox<BvhVec2d>, Failure> {
    let [x_min, y_min, x_max, y_max] = bounds.get()?;
    Ok(BvhBox::from_corners(
        BvhVec2d::new(x_min, y_min),
        BvhVec2d::new(x_max, y_max),
    ))
}

/// `bnd::box2_to_bvh`: copies the gap-adjusted limits (±1e100 on open
/// sides) into BVH corners; a void box raises `construction failure` like `Get`.
/// Example: `bnd::box3_to_bvh(&bounds)?.corner_max()`.
pub fn box3_to_bvh(bounds: &BoundingBox3) -> Result<BvhBox<BvhVec3d>, Failure> {
    let [x_min, y_min, z_min, x_max, y_max, z_max] = bounds.get()?;
    Ok(BvhBox::from_corners(
        BvhVec3d::new(x_min, y_min, z_min),
        BvhVec3d::new(x_max, y_max, z_max),
    ))
}
