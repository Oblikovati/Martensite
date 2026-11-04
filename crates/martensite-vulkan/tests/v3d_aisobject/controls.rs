use super::*;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::Graphic3dStructureMembers;
use martensite_tkv3d::select3d::Select3DSensitiveEntityMembers;
use martensite_tkv3d::{prs_mgr::*, select_mgr::*};
#[test]
fn tutorial_display_modes_and_forced_owner_are_real_callbacks() {
    let object = object::TutorialObject::new(Handle::default()).unwrap();
    assert!(object.accept_display_mode(0));
    assert!(object.accept_display_mode(1));
    assert!(!object.accept_display_mode(2));
    let owner = owner::TutorialOwner::new(&object);
    assert!(owner.is_forced_hilight());
    assert_eq!(
        owner.selectable(),
        object.cast::<dyn SelectMgrSelectableObject>()
    );
}
#[test]
fn tutorial_arrays_preserve_the_case_capacities_and_indices() {
    let triangles = geometry::triangles().unwrap();
    assert_eq!(triangles.vertex_number(), 26 * 26 + 2 * 26 * 2);
    assert_eq!(triangles.edge_number(), 3 * (25 * 25 * 2 + 2 * 25 * 2));
    let section = geometry::section().unwrap();
    assert_eq!(section.vertex_number(), 4);
    assert_eq!(section.edge_number(), 8);
}

#[test]
fn tutorial_null_driver_runs_actual_compute_and_sensitive_triangle_selection() {
    use crate::case_view::{SourceViewOutput, viewer_session::ViewerSession};
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    let object = object::TutorialObject::new(session.objects_animation()).unwrap();
    session.display_object("p", object.cast()).unwrap();
    session.fit().unwrap();
    let owner = assert_sensitive_owner(&object);
    session.move_cursor(300, 300).unwrap();
    assert_eq!(session.context().detected_owner().clone(), owner);
    let manager = session.context().main_prs_mgr();
    assert_eq!(
        object
            .get_hilight_presentation(&manager)
            .unwrap()
            .groups()
            .len(),
        1
    );
    session.close();
}

fn assert_sensitive_owner(
    object: &Handle<object::TutorialObject>,
) -> Handle<dyn SelectMgrEntityOwnerSource> {
    let selection = object.selection(0);
    assert_eq!(selection.entities().len(), 1);
    let owner = selection.entities()[0].base_sensitive().owner_id();
    assert_eq!(
        owner.selectable(),
        object.cast::<dyn SelectMgrSelectableObject>()
    );
    assert!(owner.is_forced_hilight());
    owner
}
