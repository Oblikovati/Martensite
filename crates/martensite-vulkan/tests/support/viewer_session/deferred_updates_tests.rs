use super::*;
use martensite_tkservice::null_driver::NullGraphicDriver;

fn deferred_session() -> ViewerSession {
    ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 37, 19).unwrap()
}

#[test]
fn no_update_scope_restores_the_prior_flag_and_same_owner_after_success() {
    let mut session = deferred_session();
    let view = session.view().clone();
    view.set_immediate_update(true);
    session
        .without_updates(|current| -> Result<(), Failure> {
            assert_eq!(current.view().object_address(), view.object_address());
            assert!(!current.view().set_immediate_update(false));
            Ok(())
        })
        .unwrap();
    assert!(view.set_immediate_update(false));
}

#[test]
fn no_update_scope_restores_after_returned_error_and_preserves_disabled_state() {
    let mut session = deferred_session();
    let view = session.view().clone();
    view.set_immediate_update(true);
    let error = session
        .without_updates(|_| Err::<(), _>(Failure::new("owned failure")))
        .unwrap_err();
    assert!(error.to_string().contains("owned failure"));
    assert!(view.set_immediate_update(false));
    session.without_updates(|_| Ok::<(), Failure>(())).unwrap();
    assert!(!view.set_immediate_update(false));
}

#[test]
fn no_update_scope_restores_after_an_unwind_without_creating_a_view() {
    let mut session = deferred_session();
    let view = session.view().clone();
    view.set_immediate_update(true);
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = session.without_updates(|_| -> Result<(), Failure> {
            panic!("owned unwind");
        });
    }));
    assert!(outcome.is_err());
    assert_eq!(session.views.len(), 1);
    assert_eq!(session.view().object_address(), view.object_address());
    assert!(view.set_immediate_update(false));
}

#[test]
fn source_window_origin_keeps_the_case_extent_and_view_owner() {
    let session = deferred_session();
    let owner = session.view().object_address();
    session.set_window_origin(32, 32);
    assert_eq!(session.views[0].0.position(), (32, 32, 69, 51));
    assert_eq!(session.source_window_size(), (37, 19));
    assert_eq!(session.view().object_address(), owner);
}
