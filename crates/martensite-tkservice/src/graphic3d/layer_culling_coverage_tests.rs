use super::*;
use crate::graphic3d::GRAPHIC3D_ZLAYER_ID_DEFAULT;

#[test]
fn contained_root_covers_all_children_and_preserves_queue_order() {
    let layer = Graphic3dLayer::new(GRAPHIC3D_ZLAYER_ID_DEFAULT, None);
    let mut tree = BvhTree::default();
    let low = Vec3::new(-1., -1., -1.);
    let high = Vec3::new(1., 1., 1.);
    tree.add_inner_node(low, high, 1, 2);
    tree.add_leaf_node(low, Vec3::new(0., 0., 0.), 0, 0);
    tree.add_leaf_node(high, high, 1, 1);

    let selector = Graphic3dCullingTool {
        min_ortho_projection_pts: [-2.; 3],
        max_ortho_projection_pts: [2.; 3],
        min_clip_projection_pts: [-2.; 6],
        max_clip_projection_pts: [2.; 6],
        ..Default::default()
    };
    let mut pass = CullingPass {
        selector: &selector,
        ctx: Default::default(),
    };
    assert_eq!(layer.reached_flags(&pass, &tree, 2).unwrap(), [true, true]);
    assert!(!layer.state.read().bvh_is_left_child_queued_first);

    pass.ctx.size_cull2 = 1.;
    assert_eq!(layer.reached_flags(&pass, &tree, 2).unwrap(), [true, false]);
    assert!(!layer.state.read().bvh_is_left_child_queued_first);
}
