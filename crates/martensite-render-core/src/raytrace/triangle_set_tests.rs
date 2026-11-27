use super::*;
use martensite_tkernel::ncollection::Mat4;

/// Two triangles: one at x 0..1, one at x 4..6, both at z = 0.
fn pair() -> RaytraceTriangleSet {
    let point = |x: f32, y: f32| BvhVec3f::new(x, y, 0.);
    RaytraceTriangleSet {
        vertices: vec![
            point(0., 0.),
            point(1., 0.),
            point(0., 1.),
            point(4., 0.),
            point(6., 0.),
            point(4., 2.),
        ],
        normals: vec![[0., 0., 1.]; 6],
        tex_coords: vec![[0.; 2]; 6],
        elements: vec![[0, 1, 2, 0], [3, 4, 5, 1]],
        transform: Mat4::identity(),
    }
}

#[test]
fn centres_are_the_middle_of_each_triangle_extent() {
    let set = pair();
    assert_eq!(set.center(1, 0).unwrap(), 5.);
    assert_eq!(set.center(1, 1).unwrap(), 1.);
    assert!(set.center(2, 0).is_err());
}

#[test]
fn the_box_follows_the_transform_and_the_inverse_undoes_it() {
    let mut set = pair();
    let mut moved = Mat4::<f32>::identity();
    moved.set_value(0, 3, 10.);
    set.transform = moved;
    let bounds = set.transformed_box().unwrap();
    assert_eq!((bounds.corner_min().x, bounds.corner_max().x), (10., 16.));
    assert_eq!(set.inverse_transform().get_value(0, 3), -10.);
}

#[test]
fn quad_bvh_leaves_hold_every_triangle_once() {
    let mut set = pair();
    let tree = set.quad_bvh().unwrap();
    let leaves: i32 = (0..tree.length())
        .filter(|&node| tree.is_outer(node).unwrap())
        .map(|node| tree.end_primitive(node).unwrap() - tree.beg_primitive(node).unwrap() + 1)
        .sum();
    assert_eq!(leaves, 2);
}
