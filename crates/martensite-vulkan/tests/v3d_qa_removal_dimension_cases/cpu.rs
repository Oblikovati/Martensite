//! These controls use only the Null driver; image/color qualification remains mandatory.
use super::{primitive_removal, shared_arrows};
use crate::{
    case_todo::{TodoOutcome, todo_outcome},
    case_view::{SourceViewOutput, viewer_session::ViewerSession},
};
use martensite_tkv3d::prs_dim::PrsDimDimensionMembers;

fn null_session() -> ViewerSession {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap()
}

#[test]
fn four_anonymous_primitives_are_selected_then_removed_by_their_own_identity() {
    let session = null_session();
    let objects = primitive_removal::primitive_objects().unwrap();
    assert_eq!(objects.len(), 4);
    primitive_removal::display_and_select(session.context(), &objects).unwrap();
    assert_eq!(session.context().nb_selected(), 4);
    let mut displayed = Default::default();
    session.context().displayed_objects(&mut displayed).unwrap();
    assert_eq!(displayed.len(), 4);
    primitive_removal::remove_primitives(session.context(), &objects).unwrap();
    displayed.clear();
    session.context().displayed_objects(&mut displayed).unwrap();
    assert!(displayed.is_empty());
    assert_eq!(session.context().nb_selected(), 0);
    session.close();
}

#[test]
fn false_and_true_updates_both_restart_real_invalidated_selection() {
    use crate::v3d_qa_command_cases::selected_removal::{prepare_selected_boxes, remove_selection};
    for update in [false, true] {
        let session = null_session();
        prepare_selected_boxes(session.context()).unwrap();
        assert_eq!(session.context().nb_selected(), 3);
        remove_selection(session.context(), update).unwrap();
        assert_eq!(session.context().nb_selected(), 0);
        let mut displayed = Default::default();
        session.context().displayed_objects(&mut displayed).unwrap();
        assert!(displayed.is_empty());
        session.close();
    }
}

#[test]
fn all_six_real_dimensions_retain_the_same_mutated_aspect() {
    let session = null_session();
    let scene = shared_arrows::display_dimensions(session.context()).unwrap();
    assert_eq!(scene.vertices.len(), 8);
    assert!(scene.aspect.is_arrows_3d());
    assert_eq!(scene.aspect.arrow_aspect().length(), 40.);
    assert_eq!(
        scene.aspect.arrow_aspect().angle(),
        std::f64::consts::PI / 180. * 10.
    );
    assert_shared_aspects(&scene);
    session.close();
}

fn assert_shared_aspects(scene: &shared_arrows::ArrowDimensions) {
    let expected = scene.aspect.object_address();
    assert!(
        scene
            .lengths
            .iter()
            .all(|d| d.dimension_aspect().object_address() == expected)
    );
    assert!(
        scene
            .radii
            .iter()
            .all(|d| d.dimension_aspect().object_address() == expected)
    );
}

#[test]
fn real_vertex_lengths_and_circle_radii_match_authored_geometry() {
    let session = null_session();
    let scene = shared_arrows::display_dimensions(session.context()).unwrap();
    for (dim, expected) in scene.lengths.iter().zip([500., 90., 150.]) {
        assert!((dim.get_value() - expected).abs() < 1.0e-10);
    }
    for (dim, expected) in scene.radii.iter().zip([88.58, 43.80, 17.86]) {
        assert!((dim.get_value() - expected).abs() < 1.0e-10);
    }
    assert_eq!(scene.lengths[1].get_flyout(), 30.);
    let mut displayed = Default::default();
    session.context().displayed_objects(&mut displayed).unwrap();
    assert_eq!(displayed.len(), 6);
    session.close();
}

#[test]
fn known_failure_gate_requires_actual_color_failure_and_rejects_extras_or_improvements() {
    const COLOR: &str =
        "Error : Colors are not equal in default coordinate and in the near coordinates too";
    let failing =
        |line: &str| !crate::case_v3d_policy::accepted_report(&format!("{line}\nTEST COMPLETED"));
    assert_eq!(todo_outcome(COLOR, &[COLOR], failing), TodoOutcome::Todo);
    assert_eq!(
        todo_outcome("TEST COMPLETED", &[COLOR], failing),
        TodoOutcome::Improvement(COLOR)
    );
    assert!(matches!(
        todo_outcome(
            &format!("{COLOR}\nError: unrelated failure"),
            &[COLOR],
            failing
        ),
        TodoOutcome::Failed(_)
    ));
}
