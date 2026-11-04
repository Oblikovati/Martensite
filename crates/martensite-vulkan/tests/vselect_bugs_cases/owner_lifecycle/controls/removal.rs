use super::*;

#[test]
fn remove_then_drop_releases_actual_ais_owner_before_legacy_iteration() {
    let session = cpu_session();
    let (_shape, object) = objects::anonymous_box(session.context()).unwrap();
    let weak = object.downgrade();
    removed_detection::detect_and_select(&session).unwrap();
    assert!(session.context().has_detected());
    assert_eq!(session.context().nb_selected(), 1);
    session.context().remove(&object, true).unwrap();
    drop(object);
    assert!(weak.upgrade().is_null());
    removed_detection::walk_legacy_detection(session.context());
    assert!(!session.context().more_detected());
    assert!(session.context().detected_interactive().is_null());
    assert_eq!(session.context().nb_selected(), 0);
}

#[test]
fn removing_target_preserves_unrelated_displayed_object_and_selection() {
    let session = cpu_session();
    let context = session.context();
    let (_target_shape, target) = objects::anonymous_box(context).unwrap();
    let (_other_shape, other) = objects::anonymous_box(context).unwrap();
    context.set_selected_object(&other, false).unwrap();
    context.remove(&target, true).unwrap();
    drop(target);
    removed_detection::walk_legacy_detection(context);
    assert!(context.is_displayed(&other));
    assert_eq!(context.nb_selected(), 1);
    context.init_selected();
    let selected = context.selected_owner().unwrap();
    assert_eq!(
        selected.selectable().object_address(),
        other.object_address()
    );
}
