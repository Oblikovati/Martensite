//! Actual cursor picks exercise persistent selection ranking, not only comparator structs.
use super::*;

#[test]
fn actual_persistent_box_picks_observe_independent_display_and_owner_priorities() {
    let mut session = cpu_session();
    boxes::box_scene(&mut session).unwrap();
    commands::display_priority(&session, "box_1", 5).unwrap();
    commands::display_priority(&session, "box_2", 4).unwrap();
    assert_named_detection(&mut session, (205, 170), "box_1");
    commands::owner_priority(&session, "box_1", 4).unwrap();
    commands::owner_priority(&session, "box_2", 5).unwrap();
    assert_named_detection(&mut session, (205, 170), "box_2");
}

#[test]
fn actual_quad_corner_is_detected_while_its_far_interior_is_not() {
    let mut session = cpu_session();
    objects::display_array(&mut session, "quad1", objects::quad_array().unwrap()).unwrap();
    commands::persistent_existing(&session, "quad1", Some(1)).unwrap();
    assert_named_detection(&mut session, (376, 100), "quad1");
    session.move_cursor(300, 200).unwrap();
    assert!(session.context().detected_owner().is_null());
}
