//! The walk hands each child test the bounds that the tree stores for that child.
use super::*;
use martensite_tkernel::ncollection::Vec3;

/// A root over two leaves: node 1 spans [0, 1], node 2 spans [2, 3] on every axis.
fn two_leaf_tree() -> BvhTreeHandle<BvhVec3d> {
    let handle = BvhTreeHandle::default();
    {
        let mut tree = handle.write();
        let at = |value: f64| Vec3::new(value, value, value);
        tree.add_inner_node(at(0.0), at(3.0), 1, 2);
        tree.add_leaf_node(at(0.0), at(1.0), 0, 0);
        tree.add_leaf_node(at(2.0), at(3.0), 1, 1);
    }
    handle
}

#[test]
fn each_child_test_sees_the_stored_bounds_of_that_child() {
    let tree = two_leaf_tree();
    let mut seen = Vec::new();
    let mut stack = vec![0];
    let leaf = next_selector_leaf(&tree, &mut stack, |node, bounds| {
        let stored = selector_node_bounds(&tree, node)?;
        assert_eq!(
            bounds.corner_min(),
            stored.corner_min(),
            "node {node} minimum"
        );
        assert_eq!(
            bounds.corner_max(),
            stored.corner_max(),
            "node {node} maximum"
        );
        seen.push(node);
        Ok(node == 2)
    })
    .expect("a valid tree");
    assert_eq!(seen, [1, 2], "left child first");
    assert_eq!(leaf, Some(2), "only the accepted leaf is reached");
}
