use super::*;
use martensite_tkernel::standard::HandleReference as _;
use martensite_tkservice::null_driver::NullGraphicDriver;
#[path = "failing_dimension_view_driver.rs"]
mod failing_driver;
use failing_driver::FailingDimensionViewDriver;

#[test]
fn viewer_cpu_constructor_keeps_the_declared_extent_and_mono_dump_default() {
    let session =
        ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 37, 19).unwrap();
    assert_eq!(session.source_window_size(), (37, 19));
    assert_eq!(session.stereo_dump_option(), V3dStereoDumpOptions::Mono);
    assert!(session.earlier_viewers.is_empty());
    assert_eq!(session.views.len(), 1);
    assert_eq!(
        session.view().viewer().object_address(),
        session.viewer.object_address()
    );
}

#[test]
fn second_viewer_retains_actual_earlier_view_and_context_owners() {
    let mut session =
        ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 37, 19).unwrap();
    let viewer = session.viewer.object_address();
    let context = session.context.object_address();
    let first_view = session.view().object_address();
    session.open_viewer().unwrap();
    assert_ne!(session.viewer.object_address(), viewer);
    assert_ne!(session.context.object_address(), context);
    assert_eq!(session.earlier_viewers[0].viewer.object_address(), viewer);
    assert_eq!(session.earlier_viewers[0].context.object_address(), context);
    assert_eq!(session.views[0].1.object_address(), first_view);
    assert_eq!(session.views[0].1.viewer().object_address(), viewer);
    assert_eq!(session.source_window_size(), (37, 19));
}

#[test]
fn viewer_switch_uses_the_actual_retained_view_instead_of_reconstructing_it() {
    let mut session =
        ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 37, 19).unwrap();
    let first = owner_addresses(&session);
    session.open_viewer().unwrap();
    let second = owner_addresses(&session);
    session.activate_view(0).unwrap();
    assert_eq!(owner_addresses(&session), first);
    session.activate_view(1).unwrap();
    assert_eq!(owner_addresses(&session), second);
}

#[test]
fn failed_second_view_creation_preserves_existing_owner_identity_and_selection() {
    let driver = FailingDimensionViewDriver::new_handle(2);
    let mut session = ViewerSession::with_graphic_driver(&driver, 37, 19).unwrap();
    let before = owner_addresses(&session);
    session.set_stereo_dump(V3dStereoDumpOptions::RightEye);
    let error = session.open_viewer().expect_err("second request must fail");
    assert!(error.to_string().contains("owned view request 2"));
    let after = owner_addresses(&session);
    assert_eq!(after, before);
    assert_eq!(session.stereo_dump_option(), V3dStereoDumpOptions::RightEye);
    assert!(session.earlier_viewers.is_empty());
    assert_eq!(session.views.len(), 1);
}

#[test]
fn failed_initial_view_creation_returns_its_named_driver_failure() {
    let driver = FailingDimensionViewDriver::new_handle(1);
    let error = ViewerSession::with_graphic_driver(&driver, 37, 19)
        .err()
        .expect("first request must fail");
    assert!(error.to_string().contains("owned view request 1"));
}

#[test]
fn failed_view_activation_preserves_the_complete_previous_owner() {
    let mut session =
        ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 37, 19).unwrap();
    session.open_viewer().unwrap();
    // Keep the orphan alive so refusal tests its missing registry entry, not an expired viewer.
    let _orphaned = session.earlier_viewers.pop().unwrap();
    let before = owner_addresses(&session);
    let active = session.active;
    let error = session
        .activate_view(0)
        .expect_err("unregistered owner must refuse");
    let expected = "expected a registered viewer/context/controller owner";
    assert!(error.to_string().contains(expected));
    assert_eq!(owner_addresses(&session), before);
    assert_eq!(session.active, active);
}

#[test]
fn cross_viewer_switch_restores_pointer_input_and_keeps_window_mutations_local() {
    let mut session =
        ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 37, 19).unwrap();
    session.move_cursor(4, 7).unwrap();
    session.open_viewer().unwrap();
    session.move_cursor(9, 11).unwrap();
    session.views[1].0.set_size(55, 33);
    session.activate_view(0).unwrap();
    assert_eq!(session.source_window_size(), (37, 19));
    assert_eq!(cursor_position(&session), (4, 7));
    session.views[0].0.set_size(71, 29);
    session.activate_view(1).unwrap();
    assert_eq!(session.source_window_size(), (55, 33));
    assert_eq!(cursor_position(&session), (9, 11));
}

fn owner_addresses(session: &ViewerSession) -> [*const (); 3] {
    [
        session.viewer.object_address(),
        session.context.object_address(),
        session.view().object_address(),
    ]
}

fn cursor_position(session: &ViewerSession) -> (i32, i32) {
    let position = session.controller.last_mouse_position();
    (position.x(), position.y())
}

#[test]
fn grouped_session_mutation_restores_the_view_after_a_returned_failure() {
    let mut session =
        ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 37, 19).unwrap();
    let view = session.view().clone();
    view.set_immediate_update(true);
    let error = session
        .update_session(|current| {
            assert!(!current.view().set_immediate_update(false));
            Err(Failure::new("owned grouped command failure"))
        })
        .expect_err("operation must return its failure");
    assert!(error.to_string().contains("owned grouped command failure"));
    assert!(view.set_immediate_update(false));
}

#[test]
fn unnamed_checkview_keeps_context_and_retains_the_earlier_view() {
    let mut session =
        ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 37, 19).unwrap();
    let previous = owner_addresses(&session);
    session.open_view(409, 409).unwrap();
    let current = owner_addresses(&session);
    assert_eq!(session.views.len(), 2);
    assert_eq!(session.views[0].0.size(), (37, 19));
    assert_eq!(session.source_window_size(), (409, 409));
    assert_ne!(
        session.views[0].1.object_address(),
        session.view().object_address()
    );
    assert_eq!(previous[0], current[0]);
    assert_eq!(previous[1], current[1]);
    assert_ne!(previous[2], current[2]);
}
