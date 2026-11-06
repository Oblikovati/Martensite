//! Independent line/set laws and actual polls pin the concrete Section receiver.
use super::fixture::section_case;
use super::*;
use crate::bop_algo::{
    ProgressFailureClass,
    pave_filler::owned_progress_fixture::section_progress_attribute_mask,
    progress_test_support::{indicator, pulse_indicator},
};
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{ShapeIterator, Vertex},
};
use std::sync::atomic::Ordering;
#[test]
fn crossing_straight_segments_have_one_section_vertex_at_the_exact_origin() {
    let mut case = section_case(3, false);
    assert_eq!(
        case.perform(&ProgressRange::default()).unwrap(),
        ProgressCompletion::Completed
    );
    let parts: Vec<_> = ShapeIterator::new(case.section.shape(), true, true).collect();
    assert_eq!(parts.len(), 1);
    let point = BRepTool::pnt(&Vertex::unchecked(parts[0].clone())).unwrap();
    assert_eq!([point.x(), point.y(), point.z()], [0., 0., 0.]);
}
#[test]
fn shared_topology_keeps_the_exact_edge_and_alone_vertex_handles() {
    for kind in [16, 17] {
        let mut case = section_case(kind, false);
        assert_eq!(
            case.perform(&ProgressRange::default()).unwrap(),
            ProgressCompletion::Completed
        );
        let parts: Vec<_> = ShapeIterator::new(case.section.shape(), true, true).collect();
        assert_eq!(parts.len(), 1);
        assert!(parts[0].is_same(&case.sources[1]));
    }
}
#[test]
fn supplied_fillers_keep_all_eight_actual_attribute_masks_and_caller_ownership() {
    for kind in 20..28 {
        let mut case = section_case(kind, false);
        assert_eq!(
            case.perform(&ProgressRange::default()).unwrap(),
            ProgressCompletion::Completed
        );
        assert!(case.section.pave_filler().is_none());
        let filler = case.supplied_filler.as_ref().unwrap();
        assert!(filler.data_structure().is_some());
        assert_eq!(section_progress_attribute_mask(filler), (kind - 20) as i32);
        assert!(case.section.has_filled_history());
        assert!(!case.section.options().has_errors());
    }
}
#[test]
fn actual_face_argument_and_vertex_polls_stop_before_history_publication() {
    for target in [5, 7, 9] {
        let mut case = section_case(20, false);
        let (progress, trace) = pulse_indicator(target);
        let completion = case.perform(&progress.start()).unwrap();
        assert!(!matches!(completion, ProgressCompletion::Completed));
        assert_eq!(trace.queries.load(Ordering::SeqCst), target);
        assert!(!case.section.has_filled_history());
        assert!(case.section.options().has_errors());
    }
}
#[test]
fn report_clear_does_not_erase_a_real_previously_observed_cancellation() {
    let mut case = section_case(20, false);
    case.perform(&ProgressRange::default()).unwrap();
    let (progress, trace) = pulse_indicator(1);
    let range = progress.start();
    let scope = ProgressScope::unnamed(&range, 2., false);
    let observed = ProgressCancellation::default();
    assert!(observe_user_break(
        case.section.options(),
        &scope,
        &observed
    ));
    let structure = case
        .supplied_filler
        .as_ref()
        .unwrap()
        .data_structure()
        .unwrap();
    build_section_progress(&mut case.section, structure, &scope.next(1.), &observed).unwrap();
    assert!(!case.section.options().has_errors());
    assert!(trace.queries.load(Ordering::SeqCst) > 1);
    assert!(matches!(
        finish(&mut case.section, ProgressCompletion::Completed, &observed),
        ProgressCompletion::Cancelled { .. }
    ));
}
#[test]
fn an_empty_request_fails_and_a_single_vertex_completes_an_empty_section() {
    let mut empty = section_case(0, false);
    assert!(matches!(
        empty.perform(&ProgressRange::default()).unwrap(),
        ProgressCompletion::Failed {
            class: ProgressFailureClass::AlgorithmReport,
            ..
        }
    ));
    let mut single = section_case(11, false);
    assert_eq!(
        single.perform(&ProgressRange::default()).unwrap(),
        ProgressCompletion::Completed
    );
    assert_eq!(
        ShapeIterator::new(single.section.shape(), true, true).count(),
        0
    );
}
#[test]
fn section_history_policy_and_actual_parent_names_are_retained() {
    let mut case = section_case(7, false);
    let (progress, trace) = indicator(0);
    assert_eq!(
        case.perform(&progress.start()).unwrap(),
        ProgressCompletion::Completed
    );
    assert!(!case.section.has_filled_history());
    let events = trace.events.lock().unwrap();
    for expected in [
        "Performing General Fuse operation",
        "Building result of SECTION operation",
        "Building the result of Section operation",
    ] {
        assert!(
            events
                .iter()
                .any(|(_, names)| names.iter().any(|name| name == expected)),
            "missing {expected}"
        );
    }
}
