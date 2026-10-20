//! Retained reflection operations plus independent empty-shape lifecycle checks.
#[path = "replay_support/mod.rs"]
mod replay_support;
use martensite_tkbrep::topods::Shape;
use martensite_tkhlr::{hlr_appli::ReflectLines, hlr_brep::TypeOfResultingEdge};
use std::{ops::Range, sync::Mutex};
// The historical hidden-line pipeline may sample through its global compatibility API.
// This guard establishes replay order only, not independent-operation thread safety.
static HISTORICAL_REFLECTION_CONTEXT: Mutex<()> = Mutex::new(());
const REQUESTS: &str = include_str!("fixtures/reflection_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/reflection_numeric/reference-expected.txt");
fn run_case(request: Range<usize>, expected: Range<usize>) {
    let _context = HISTORICAL_REFLECTION_CONTEXT.lock().unwrap();
    replay_support::check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}
#[test]
fn original_reflection_count_and_order_are_complete() {
    let _context = HISTORICAL_REFLECTION_CONTEXT.lock().unwrap();
    assert_eq!(REQUESTS.lines().count(), 120);
    assert_eq!(EXPECTATIONS.lines().count(), 120);
    for (request, expected) in REQUESTS.lines().zip(EXPECTATIONS.split_inclusive('\n')) {
        assert!(request.starts_with("hb_reflect "));
        replay_support::check_request_slice(request, expected);
    }
}
#[test]
fn empty_shape_has_no_reflections_after_repeated_perform() {
    let _context = HISTORICAL_REFLECTION_CONTEXT.lock().unwrap();
    let mut lines = ReflectLines::new(&Shape::default()).unwrap();
    lines.set_axes(0., 0., 1., 0., 0., 0., 0., 1., 0.).unwrap();
    for _ in 0..2 {
        lines.perform().unwrap();
        assert!(lines.get_result().unwrap().is_null());
        for visible in [true, false] {
            assert!(
                lines
                    .get_compound_of_3d_edges(TypeOfResultingEdge::Sharp, visible, true)
                    .unwrap()
                    .is_null()
            );
        }
    }
}
#[test]
fn zero_or_parallel_view_axes_reject_the_direction_construction() {
    let mut lines = ReflectLines::new(&Shape::default()).unwrap();
    for (normal, up) in [([0., 0., 0.], [0., 1., 0.]), ([0., 0., 1.], [0., 0., 1.])] {
        let error = lines
            .set_axes(
                normal[0], normal[1], normal[2], 0., 0., 0., up[0], up[1], up[2],
            )
            .unwrap_err();
        assert_eq!(error.exception_type(), "Standard_ConstructionError");
    }
}
#[path = "reflection_numeric/cases_00.rs"]
mod cases_00;
#[path = "reflection_numeric/cases_01.rs"]
mod cases_01;
