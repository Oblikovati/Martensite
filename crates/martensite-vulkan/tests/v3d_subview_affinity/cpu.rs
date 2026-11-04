use super::*;
use crate::case_view::{SourceViewOutput, viewer_session::ViewerSession};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::Graphic3dCViewMembers;
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;
fn null_session() -> ViewerSession {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    ViewerSession::with_graphic_driver(&driver, 1024, 512).unwrap()
}
#[test]
fn actual_subview_binding_and_controller_state_without_pixel_claim() {
    let mut session = null_session();
    scene::prepare(&mut session).unwrap();
    let right = session.view().clone();
    let owner = session.context().object_address();
    assert_source_window(&session);
    session.activate_view(1).unwrap();
    assert_eq!(session.context().object_address(), owner);
    assert_source_window(&session);
    session.hide_in_active_view("s").unwrap();
    session.set_camera_rotation_mode(AisRotationMode::PickLast);
    assert_eq!(session.camera_rotation_mode(), AisRotationMode::PickLast);
    let sphere = session.object("s").unwrap();
    let affinity = sphere.view_affinity();
    assert!(!affinity.is_visible(session.view().get().unwrap().view().identification()));
    assert!(affinity.is_visible(right.get().unwrap().view().identification()));
    session.close();
}
fn assert_source_window(session: &ViewerSession) {
    let actual = session.view().window();
    assert_eq!(session.source_window_size(), actual.size());
    assert!(session.view().view_base().is_subview());
}
#[test]
fn missing_parent_refuses_without_rebinding_the_actual_window() {
    let mut session = null_session();
    let before = session.view().window().object_address();
    assert!(
        session
            .bind_subview(&Handle::default(), [0.5, 1.], [0., 0.])
            .is_err()
    );
    assert_eq!(session.view().window().object_address(), before);
    assert!(!session.view().view_base().is_subview());
    session.close();
}
