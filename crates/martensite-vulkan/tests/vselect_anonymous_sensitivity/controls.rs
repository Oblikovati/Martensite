//! Real null-driver contexts establish command setup and failure guards without GPU evidence.
use super::{require_selected, scene::*};
use crate::case_view::viewer_session::ViewerSession;
use martensite_tkservice::null_driver::NullGraphicDriver;
use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;
use martensite_tkv3d::select_mgr::SelectMgrSelectableObjectMembers as _;

fn cpu_session() -> ViewerSession {
    ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 409, 409)
        .expect("anonymous sensitivity control; expected isolated409x409 viewer")
}

#[test]
fn positioned_boxes_preserve_literal_topology_bounds() {
    for (shape, expected) in source_boxes()
        .unwrap()
        .iter()
        .zip([[10., 10., 0., 20., 20., 10.], [0., 0., 0., 10., 10., 10.]])
    {
        let mut bounds = martensite_tkmath::bnd::BoundingBox3::default();
        BRepBndLib::add(shape, &mut bounds, false).unwrap();
        for (actual, expected) in bounds.get().unwrap().iter().zip(expected) {
            assert!((actual - expected).abs() < 1.0e-6);
        }
    }
}

#[test]
fn anonymous_display_activates_only_edge_mode_and_preserves_width() {
    let session = cpu_session();
    let objects = display_source_boxes(&session).unwrap();
    for object in &objects {
        let mut modes = Default::default();
        session.context().activated_modes(object, &mut modes);
        assert_eq!(modes.iter().copied().collect::<Vec<_>>(), [2]);
        assert_eq!(session.context().width(object).unwrap(), 3.);
        assert!(session.context().is_displayed_mode(object, 0));
    }
    assert!(session.object("b1").is_none());
    assert!(session.object("b2").is_none());
}

#[test]
fn edge_sensitivity_change_is_local_to_first_actual_object() {
    let session = cpu_session();
    let objects = display_source_boxes(&session).unwrap();
    let second_before = objects[1].selection(2).sensitivity();
    session
        .context()
        .set_selection_sensitivity(&objects[0], 2, 5)
        .unwrap();
    assert_eq!(objects[0].selection(2).sensitivity(), 5);
    assert_eq!(objects[1].selection(2).sensitivity(), second_before);
}

#[test]
fn mismatched_count_returns_command_failure_instead_of_continuing_to_dump() {
    let session = cpu_session();
    assert!(require_selected(&session, 0, "ERROR: no boxes must be selected!").is_ok());
    let failure = require_selected(&session, 1, "ERROR: b1 was not selected").unwrap_err();
    assert!(
        failure
            .to_string()
            .contains("selected owner count 0; expected 1")
    );
}

#[test]
fn actual_command_counts_pass_in_isolated_null_view() {
    let session = cpu_session();
    super::selection_command(&session).unwrap();
    assert_eq!(session.context().nb_selected(), 1);
}
