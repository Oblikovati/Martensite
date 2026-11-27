use super::super::bvh_cache::RaytraceBvhCache;
use super::*;
use martensite_tkernel::ncollection::Mat4;

fn triangle(x: f32, material: i32) -> RaytraceTriangleSet {
    let point = |x: f32, y: f32| BvhVec3f::new(x, y, 0.);
    RaytraceTriangleSet {
        vertices: vec![point(x, 0.), point(x + 1., 0.), point(x, 1.)],
        normals: vec![[0., 0., 1.]; 3],
        tex_coords: vec![[0.; 2]; 3],
        elements: vec![[0, 1, 2, material]],
        transform: Mat4::identity(),
    }
}

#[test]
fn top_leaves_link_each_set_tree_and_geometry_offsets() {
    let buffers = build(
        vec![
            triangle(0., 0),
            triangle(5., 1),
            RaytraceTriangleSet::default(),
        ],
        &mut RaytraceBvhCache::default(),
    )
    .unwrap();
    // The empty set is dropped: two objects, each one bottom node after the top tree.
    assert_eq!(buffers.transforms.len(), 2);
    assert_eq!((buffers.vertices.len(), buffers.triangles.len()), (6, 2));
    let leaves: Vec<_> = buffers.node_info.iter().filter(|n| n[0] > 0).collect();
    assert_eq!(leaves.len(), 2);
    for leaf in leaves {
        let bottom = buffers.node_info[leaf[1] as usize];
        assert_eq!(bottom[0], -1, "bottom leaves are marked");
        let first = buffers.triangles[leaf[3] as usize];
        let vertex = buffers.vertices[(leaf[2] + first[0]) as usize];
        // Each object's geometry follows its own offsets: material 0 sits at x = 0.
        assert_eq!(vertex[0], if first[3] == 0 { 0. } else { 5. });
    }
    assert_eq!(buffers.bounds.corner_max().x, 6.);
}

#[test]
fn a_scene_without_triangles_packs_nothing() {
    let cache = &mut RaytraceBvhCache::default();
    let buffers = build(vec![RaytraceTriangleSet::default()], cache).unwrap();
    assert!(buffers.transforms.is_empty() && buffers.node_info.is_empty());
}

#[test]
fn unchanged_sets_reuse_their_trees_and_pack_the_same_buffers() {
    let scene = || vec![triangle(0., 0), triangle(5., 1)];
    let mut cache = RaytraceBvhCache::default();
    let fresh = build(scene(), &mut cache).unwrap();
    assert_eq!(cache.built_last, 2);
    let reused = build(scene(), &mut cache).unwrap();
    assert_eq!(cache.built_last, 0, "both trees are reused");
    assert_eq!(reused, fresh);
    // A moved set rebuilds its own tree only (its root box follows the transform).
    let mut moved = scene();
    moved[1]
        .transform
        .translate(martensite_tkernel::ncollection::Vec3::new(0., 2., 0.));
    let rebuilt = build(moved.clone(), &mut cache).unwrap();
    assert_eq!(cache.built_last, 1);
    assert_eq!(
        rebuilt,
        build(moved, &mut RaytraceBvhCache::default()).unwrap()
    );
}
