use super::*;
use martensite_tkv3d::select_mgr::SelectMgrStateOfSelection;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ChildMembership {
    address: *const (),
    modes: Vec<i32>,
    primitives: Vec<(i32, usize)>,
}

fn primitive_membership(object: &Handle<AisShape>) -> Vec<(i32, usize)> {
    let selections = object.selections();
    assert!(!selections.is_empty());
    selections
        .iter()
        .map(|selection| {
            assert!(!selection.is_empty());
            assert_eq!(
                selection.get_selection_state(),
                SelectMgrStateOfSelection::Activated
            );
            (selection.mode(), selection.entities().len())
        })
        .collect()
}

fn object_membership(session: &ViewerSession, object: &Handle<AisShape>) -> ChildMembership {
    let mut modes = List::default();
    session
        .context()
        .activated_modes(&object.cast(), &mut modes);
    let modes: Vec<i32> = modes.iter().copied().collect();
    assert_eq!(modes, vec![0]);
    assert!(session.context().main_selector().contains(&object.cast()));
    ChildMembership {
        address: object.object_address(),
        modes,
        primitives: primitive_membership(object),
    }
}

pub(super) fn scene_membership(
    session: &ViewerSession,
    scene: &ChildSelectionScene,
) -> Vec<ChildMembership> {
    scene_objects(scene)
        .iter()
        .map(|object| object_membership(session, object))
        .collect()
}

fn select_unrelated_box(context: &AisInteractiveContext) -> Handle<AisShape> {
    let shape = crate::primitive_shapes::box_at([50.; 3], [2.; 3]).unwrap();
    let other = AisShape::new(&shape);
    context.display(&other.cast(), false).unwrap();
    context.set_selected_object(&other.cast(), false).unwrap();
    other
}

fn assert_unrelated_selected(context: &AisInteractiveContext, other: &Handle<AisShape>) {
    context.init_selected();
    assert_eq!(context.nb_selected(), 1);
    let selected = context.selected_owner().unwrap();
    assert_eq!(
        selected.selectable().object_address(),
        other.object_address()
    );
}

#[test]
fn repeated_real_recompute_keeps_membership_and_unrelated_selected_identity() {
    let session = child_cpu_session();
    let context = session.context();
    let scene = recompute::displayed_sphere_children(context).unwrap();
    let other = select_unrelated_box(context);
    let topology = other.shape();
    let scene_shapes = scene_topology(&scene);
    let before = scene_membership(&session, &scene);
    for _ in 0..2 {
        recompute::recompute_parent(context, &scene).unwrap();
    }
    assert_eq!(scene_membership(&session, &scene), before);
    assert_scene_topology(&scene, &scene_shapes);
    assert_children_bound(&scene);
    assert_unrelated_selected(context, &other);
    assert!(other.shape().is_same(&topology));
}
