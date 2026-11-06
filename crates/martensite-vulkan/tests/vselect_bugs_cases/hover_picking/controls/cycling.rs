use super::*;

#[test]
fn actual_three_box_detection_cycles_and_wraps_in_both_directions() {
    let session = cycle_session();
    let detected = detected_objects(&session);
    assert_eq!(
        detected.len(),
        3,
        "source overlap must have three actual owners"
    );
    for name in ["b0", "b1", "b2"] {
        assert!(detected.contains(&hover_object(&session, name).unwrap()));
    }
    assert_cycle_owner(&session, "b0");
    for (stage, name) in expected_steps() {
        super::super::cycle::advance(&session, stage).unwrap();
        assert_cycle_owner(&session, name);
    }
    assert_eq!(session.context().nb_selected(), 0);
}

fn assert_cycle_owner(session: &ViewerSession, name: &str) {
    let actual = highlighted(session);
    assert_eq!(actual, hover_object(session, name).unwrap());
    observe(session, name);
}

#[test]
fn actual_empty_detection_cycle_preserves_the_genuinely_picked_selected_owner() {
    let mut session = cycle_session();
    session.select_point(210, 280).unwrap();
    let prior = selected_owner(&session);
    session.move_cursor(0, 0).unwrap();
    assert!(!session.context().has_detected());
    session.highlight_next_detected().unwrap();
    session.highlight_previous_detected().unwrap();
    assert!(!session.context().has_detected());
    assert!(detected_objects(&session).is_empty());
    assert_eq!(selected_owner(&session), prior);
    observe(&session, "empty_cycle");
}

fn selected_owner(
    session: &ViewerSession,
) -> Handle<dyn martensite_tkv3d::select_mgr::SelectMgrEntityOwnerSource> {
    let context = session.context();
    assert_eq!(context.nb_selected(), 1);
    context.init_selected();
    context
        .selected_owner()
        .expect("actual click; expected one real selected owner")
}

fn expected_steps() -> [(usize, &'static str); 6] {
    [
        (1, "b1"),
        (2, "b2"),
        (3, "b0"),
        (4, "b2"),
        (5, "b1"),
        (6, "b0"),
    ]
}
