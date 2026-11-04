use super::*;
use martensite_tkservice::graphic3d::Graphic3dCameraProjection;
use martensite_tkv3d::{
    std_select::{StdSelectBRepOwner, StdSelectBRepOwnerMembers as _},
    v3d::V3dTypeOfOrientation,
};

pub(super) fn pick_world_xy(session: &mut ViewerSession, x: f64, y: f64) {
    session
        .set_projection(Graphic3dCameraProjection::Orthographic)
        .unwrap();
    session
        .set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)
        .unwrap();
    session.fit().unwrap();
    let (xp, yp) = session.view().convert_world_to_pixels(x, y, 0.).unwrap();
    assert!((0..409).contains(&xp) && (0..409).contains(&yp));
    session.select_point(xp, yp).unwrap();
}

pub(super) fn assert_selected_child(session: &ViewerSession, child: &Handle<AisShape>) {
    let context = session.context();
    assert_eq!(context.nb_selected(), 1);
    context.init_selected();
    let selected = context.selected_owner().unwrap();
    assert_eq!(
        selected.selectable().object_address(),
        child.object_address()
    );
    let brep = selected.cast::<StdSelectBRepOwner>();
    assert!(!brep.is_null() && brep.has_shape());
    assert!(brep.shape_ref().clone_shape().is_same(&child.shape()));
}

#[test]
fn parent_only_display_activates_and_really_picks_origin_sphere_child() {
    let mut session = child_cpu_session();
    let scene = parent_display::parent_only_display(session.context()).unwrap();
    assert_eq!(scene.children.len(), 1);
    assert_children_bound(&scene);
    let child = &scene.children[0];
    assert!(session.context().main_selector().contains(&child.cast()));
    let selection = child.selection(0);
    assert!(!selection.is_empty());
    assert_eq!(
        selection.get_selection_state(),
        martensite_tkv3d::select_mgr::SelectMgrStateOfSelection::Activated
    );
    // (-5,-3) is strictly inside radius10's disk and outside box[0,20]^2.
    pick_world_xy(&mut session, -5., -3.);
    assert_selected_child(&session, &scene.children[0]);
}
