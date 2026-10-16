//! The `using BvhSet<T, N>::Box;` declarations of the derived BVH sets re-expose the
//! whole-set `Box()` beside each set's own `Box(int)`. In Rust every set reaches it as the
//! `BvhSet::bounding_box` trait method; the expected boxes are the closed-form unions.
use martensite_tkmath::bvh::{
    BvhBox, BvhBoxSet, BvhGeometry, BvhIndexedBoxSet, BvhObject, BvhObjectSet, BvhSet,
    BvhTriangulation, BvhVec3d, BvhVec4i, array,
};

fn unit_box(x: f64) -> BvhBox<BvhVec3d> {
    BvhBox::from_corners(BvhVec3d::new(x, 0.0, 0.0), BvhVec3d::new(x + 1.0, 1.0, 1.0))
}

/// Union of unit boxes at x = 2 and x = 7: [2, 8] x [0, 1] x [0, 1].
fn expected_union() -> BvhBox<BvhVec3d> {
    BvhBox::from_corners(BvhVec3d::new(2.0, 0.0, 0.0), BvhVec3d::new(8.0, 1.0, 1.0))
}

/// A one-triangle mesh spanning [x, x + 1] x [0, 1] x {0}, already marked dirty.
fn triangle(x: f64) -> BvhTriangulation<BvhVec3d> {
    let mut mesh = BvhTriangulation::default();
    array::append(&mut mesh.vertices, BvhVec3d::new(x, 0.0, 0.0));
    array::append(&mut mesh.vertices, BvhVec3d::new(x + 1.0, 0.0, 0.0));
    array::append(&mut mesh.vertices, BvhVec3d::new(x, 1.0, 0.0));
    array::append(&mut mesh.elements, BvhVec4i::new(0, 1, 2, 0));
    mesh.mark_dirty();
    mesh
}

fn flat_union() -> BvhBox<BvhVec3d> {
    BvhBox::from_corners(BvhVec3d::new(2.0, 0.0, 0.0), BvhVec3d::new(8.0, 1.0, 0.0))
}

#[test]
fn box_sets_expose_the_inherited_whole_set_box() {
    let mut boxes = BvhBoxSet::<BvhVec3d, i32>::default();
    boxes.add(0, unit_box(2.0));
    boxes.add(1, unit_box(7.0));
    boxes.mark_dirty();
    assert_eq!(BvhSet::bounding_box(&boxes).unwrap(), expected_union());
    let mut indexed = BvhIndexedBoxSet::<BvhVec3d, &str>::default();
    indexed.add("left", unit_box(2.0));
    indexed.add("right", unit_box(7.0));
    indexed.mark_dirty();
    assert_eq!(BvhSet::bounding_box(&indexed).unwrap(), expected_union());
}

#[test]
fn triangulation_exposes_the_inherited_whole_set_box() {
    let mut mesh = triangle(2.0);
    array::append(&mut mesh.vertices, BvhVec3d::new(7.0, 0.0, 0.0));
    array::append(&mut mesh.vertices, BvhVec3d::new(8.0, 0.0, 0.0));
    array::append(&mut mesh.vertices, BvhVec3d::new(7.0, 1.0, 0.0));
    array::append(&mut mesh.elements, BvhVec4i::new(3, 4, 5, 0));
    mesh.mark_dirty();
    assert_eq!(BvhSet::bounding_box(&mesh).unwrap(), flat_union());
}

#[test]
fn object_sets_expose_the_inherited_whole_set_box() {
    let mut objects = BvhObjectSet::<BvhVec3d>::default();
    objects.objects_mut().append(Some(Box::new(triangle(2.0))));
    objects.objects_mut().append(Some(Box::new(triangle(7.0))));
    assert_eq!(BvhSet::bounding_box(&objects).unwrap(), flat_union());
    let mut geometry = BvhGeometry::<BvhVec3d>::default();
    geometry.objects_mut().append(Some(Box::new(triangle(2.0))));
    geometry.objects_mut().append(Some(Box::new(triangle(7.0))));
    geometry.mark_dirty();
    assert_eq!(BvhSet::bounding_box(&geometry).unwrap(), flat_union());
}
