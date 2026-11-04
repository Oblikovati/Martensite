//! Pin the existing OCC74 transition before sharing it with the missing mode cases.
use crate::case_view::viewer_session::{DisplayRequest, ViewerSession};
use crate::primitive_shapes::box_sized;
use martensite_tkernel::{ncollection::List, standard::Handle};
use martensite_tkservice::null_driver::NullGraphicDriver;
use martensite_tkv3d::ais::AisInteractiveObject;

const LEGACY_ACTIVE_MODE_FIXTURE: [(i32, &[i32]); 8] = [
    (0, &[0]),
    (1, &[1]),
    (2, &[2]),
    (3, &[3]),
    (4, &[4]),
    (5, &[5]),
    (6, &[6]),
    (-1, &[]),
];

#[derive(Debug, PartialEq, Eq)]
struct ModeObservation {
    modes: Vec<i32>,
    displayed: bool,
    automatic: bool,
    selected: i32,
}

#[test]
fn existing_legacy_mode_transition_matches_owned_context_fixture() {
    let session = baseline_box_session();
    let object = baseline_object(&session);
    session
        .context()
        .set_selected_object(&object, false)
        .unwrap();
    assert_eq!(session.selected_count(), 1);
    for (requested, expected) in LEGACY_ACTIVE_MODE_FIXTURE {
        super::redisplay_with_one_mode(&session, "b", requested).unwrap();
        assert_mode_observation(&session, &object, requested, expected);
    }
    assert_local_mode_order(&session, &object);
    assert_missing_operand_behavior();
}

fn baseline_box_session() -> ViewerSession {
    let driver = NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409)
        .expect("mode baseline view=409; expected a real null-driver context");
    let shape = box_sized([10.; 3]).unwrap();
    session
        .display("b", &shape, &DisplayRequest::default())
        .unwrap();
    assert!(session.context().get_auto_activate_selection());
    session
}

fn baseline_object(session: &ViewerSession) -> Handle<dyn AisInteractiveObject> {
    session
        .object("b")
        .cloned()
        .expect("mode baseline operand=b; expected a real displayed box")
}

fn observe_mode(
    session: &ViewerSession,
    object: &Handle<dyn AisInteractiveObject>,
) -> ModeObservation {
    let mut modes = List::default();
    session.context().activated_modes(object, &mut modes);
    ModeObservation {
        modes: modes.iter().copied().collect(),
        displayed: session.context().is_displayed(object),
        automatic: session.context().get_auto_activate_selection(),
        selected: session.selected_count(),
    }
}

fn assert_mode_observation(
    session: &ViewerSession,
    object: &Handle<dyn AisInteractiveObject>,
    requested: i32,
    expected: &[i32],
) {
    let actual = observe_mode(session, object);
    record_mode("reset", requested, &actual);
    assert_eq!(actual.modes, expected, "requested mode={requested}");
    assert_eq!(actual.modes.last().copied().unwrap_or(-1), requested);
    assert!(actual.displayed);
    assert!(!actual.automatic);
    assert_eq!(actual.selected, 0);
    assert_eq!(&baseline_object(session), object);
}

fn assert_local_mode_order(session: &ViewerSession, object: &Handle<dyn AisInteractiveObject>) {
    session.context().activate(object, 2, false).unwrap();
    session.context().activate(object, 4, false).unwrap();
    let actual = observe_mode(session, object);
    record_mode("coexistent", 4, &actual);
    assert_eq!(actual.modes, [2, 4]);
    assert_eq!(actual.modes.last(), Some(&4));
    super::redisplay_with_one_mode(session, "b", 3).unwrap();
    assert_mode_observation(session, object, 3, &[3]);
}

fn assert_missing_operand_behavior() {
    let session = baseline_box_session();
    let object = baseline_object(&session);
    let before = observe_mode(&session, &object);
    assert_eq!(before.modes, [0]);
    // Keep this old wrapper's null lookup unchanged; new original bridges must refuse missing names.
    super::redisplay_with_one_mode(&session, "missing_mode_operand", 4).unwrap();
    let actual = observe_mode(&session, &object);
    record_mode("missing", 4, &actual);
    assert_eq!(actual.modes, before.modes);
    assert_eq!(actual.displayed, before.displayed);
    assert_eq!(actual.selected, before.selected);
    assert!(!actual.automatic);
    assert!(session.object("missing_mode_operand").is_none());
    assert_eq!(baseline_object(&session), object);
}

fn record_mode(stage: &str, requested: i32, actual: &ModeObservation) {
    // Process-independent scalar observations can be replayed after the separate structural move.
    eprintln!(
        "{{\"mode_baseline\":\"{stage}\",\"requested\":{requested},\"active\":{:?},\"displayed\":{},\"automatic\":{},\"selected\":{}}}",
        actual.modes, actual.displayed, actual.automatic, actual.selected
    );
}
