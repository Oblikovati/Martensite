//! Complete original hiding corpus with empty-geometry and owner-lifetime controls.
#[path = "replay_support/mod.rs"]
mod replay_support;
use martensite_tkernel::standard::Handle;
use martensite_tkhlr::hlr_brep::{Data, Hider};
use std::{ops::Range, sync::Mutex};
// ShapeToHLR invokes historical contour sampling: this guard owns replay only.
static HISTORICAL_HIDER_CONTEXT: Mutex<()> = Mutex::new(());
const REQUESTS: &str = include_str!("fixtures/hider_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/hider_numeric/reference-expected.txt");
fn run_case(request: Range<usize>, expected: Range<usize>) {
    let _context = HISTORICAL_HIDER_CONTEXT.lock().unwrap();
    replay_support::check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}
#[test]
fn original_hider_counts_and_complete_order_are_exact() {
    let _context = HISTORICAL_HIDER_CONTEXT.lock().unwrap();
    assert_eq!(REQUESTS.lines().count(), 400);
    assert_eq!(EXPECTATIONS.lines().count(), 400);
    for (request, expected) in REQUESTS.lines().zip(EXPECTATIONS.split_inclusive('\n')) {
        assert!(request.starts_with("hb_hider "));
        replay_support::check_request_slice(request, expected);
    }
}
#[test]
fn empty_compound_finishes_before_reading_an_uninitialized_shape_box() {
    let _context = HISTORICAL_HIDER_CONTEXT.lock().unwrap();
    assert_eq!(
        replay_support::replay::execute_request("hb_hider 2 7 0 0").unwrap(),
        "\n"
    );
}
#[test]
fn hider_keeps_records_alive_until_last_owner_and_own_hiding_preserves_empty_state() {
    let handle = Handle::new(Data::new(0, 0, 0).unwrap());
    let weak = handle.downgrade();
    let hider = Hider::new(&handle);
    drop(handle);
    assert!(!weak.upgrade().is_null());
    for index in [i32::MIN, 0, i32::MAX] {
        hider.own_hiding(index);
    }
    let retained = weak.upgrade();
    assert_eq!(
        (
            retained.nb_vertices(),
            retained.nb_edges(),
            retained.nb_faces()
        ),
        (0, 0, 0)
    );
    drop(retained);
    drop(hider);
    assert!(weak.upgrade().is_null());
}
#[path = "hider_numeric/cases_00.rs"]
mod cases_00;
#[path = "hider_numeric/cases_01.rs"]
mod cases_01;
#[path = "hider_numeric/cases_02.rs"]
mod cases_02;
#[path = "hider_numeric/cases_03.rs"]
mod cases_03;
#[path = "hider_numeric/cases_04.rs"]
mod cases_04;
