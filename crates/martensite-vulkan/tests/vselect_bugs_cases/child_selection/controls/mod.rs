//! Real-context supporting observations; none strengthens the original predicates.
use super::*;
use martensite_tkernel::ncollection::List;
use martensite_tkservice::null_driver::NullGraphicDriver;
use martensite_tkv3d::{
    ais::AisShapeParameters as _,
    prs_mgr::PrsMgrPresentableObjectMembers as _,
    select_mgr::{
        SelectMgrSelectableObjectMembers as _, SelectMgrViewerSelectorObjectMembers as _,
    },
};
mod geometry;
mod hierarchy;
mod membership;
mod picking;

fn child_cpu_session() -> ViewerSession {
    let driver = NullGraphicDriver::new_handle();
    ViewerSession::with_graphic_driver(&driver, 409, 409)
        .expect("child selection CPU scene; expected a real409x409 null-driver context")
}

fn scene_objects(scene: &ChildSelectionScene) -> Vec<Handle<AisShape>> {
    let mut objects = vec![scene.parent.clone()];
    objects.extend(scene.children.iter().cloned());
    objects
}

fn assert_children_bound(scene: &ChildSelectionScene) {
    let mut actual: Vec<*const ()> = scene
        .parent
        .children()
        .iter()
        .map(Handle::object_address)
        .collect();
    let mut expected: Vec<*const ()> = scene.children.iter().map(Handle::object_address).collect();
    actual.sort();
    expected.sort();
    assert_eq!(actual, expected);
    for child in &scene.children {
        assert_eq!(
            child.parent().object_address(),
            scene.parent.object_address()
        );
        assert!(child.children().is_empty());
    }
}

fn scene_topology(scene: &ChildSelectionScene) -> Vec<Shape> {
    let objects = scene_objects(scene);
    objects.iter().map(|object| object.shape()).collect()
}

fn assert_scene_topology(scene: &ChildSelectionScene, before: &[Shape]) {
    let after = scene_topology(scene);
    assert_eq!(after.len(), before.len());
    for (old, current) in before.iter().zip(&after) {
        assert!(old.is_same(current));
    }
}
