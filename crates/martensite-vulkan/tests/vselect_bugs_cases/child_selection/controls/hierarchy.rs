use super::*;

#[test]
fn five_separate_displayed_children_keep_exact_parent_and_sibling_identities() {
    let session = child_cpu_session();
    let context = session.context();
    let scene = recompute::displayed_sphere_children(context).unwrap();
    assert_eq!(scene.children.len(), 5);
    assert_children_bound(&scene);
    assert!(context.is_displayed(&scene.parent.cast()));
    for child in &scene.children {
        assert!(context.is_displayed(&child.cast()));
        assert!(context.main_selector().contains(&child.cast()));
    }
}

#[test]
fn parent_recomputation_keeps_modes_primitives_membership_and_real_child_pick() {
    let mut session = child_cpu_session();
    let scene = recompute::displayed_sphere_children(session.context()).unwrap();
    let topology = scene_topology(&scene);
    let before = membership::scene_membership(&session, &scene);
    recompute::recompute_parent(session.context(), &scene).unwrap();
    assert_eq!(membership::scene_membership(&session, &scene), before);
    assert_scene_topology(&scene, &topology);
    assert_children_bound(&scene);
    // x=y=5 lies inside child5's disk and outside all the earlier radius1 disks.
    picking::pick_world_xy(&mut session, 5., 5.);
    picking::assert_selected_child(&session, &scene.children[4]);
}
