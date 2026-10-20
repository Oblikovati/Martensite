//! Complete algorithm corpus with empty-state and extraction ownership controls.
#[path = "replay_support/mod.rs"]
mod replay_support;
use martensite_tkernel::standard::Handle;
use martensite_tkhlr::hlr_brep::{Algo, HLRToShape};
use std::{ops::Range, sync::Mutex};
// ShapeToHLR invokes historical contour sampling; this guard owns compatibility replay only.
static HISTORICAL_ALGO_CONTEXT: Mutex<()> = Mutex::new(());
const REQUESTS: &str = include_str!("fixtures/algo_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/algo_numeric/reference-expected.txt");
fn run_case(request: Range<usize>, expected: Range<usize>) {
    let _context = HISTORICAL_ALGO_CONTEXT.lock().unwrap();
    replay_support::check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}
#[test]
fn original_algo_counts_and_complete_order_are_exact() {
    let _context = HISTORICAL_ALGO_CONTEXT.lock().unwrap();
    assert_eq!(REQUESTS.lines().count(), 360);
    assert_eq!(EXPECTATIONS.lines().count(), 360);
    for (request, expected) in REQUESTS.lines().zip(EXPECTATIONS.split_inclusive('\n')) {
        assert!(request.starts_with("hb_algo "));
        replay_support::check_request_slice(request, expected);
    }
}
#[test]
fn fresh_empty_algorithm_has_no_shape_or_records_after_repeated_update() {
    let algorithm = Algo::new().unwrap();
    for _ in 0..2 {
        algorithm.update().unwrap();
        assert_eq!(algorithm.nb_shapes(), 0);
        assert!(algorithm.data_structure().is_null());
        assert_eq!(
            algorithm.index(&martensite_tkbrep::topods::Shape::default()),
            0
        );
    }
}
#[test]
fn result_extractor_keeps_algorithm_alive_until_last_owner() {
    let handle = Handle::new(Algo::new().unwrap());
    let weak = handle.downgrade();
    let extraction = HLRToShape::new(&handle);
    drop(handle);
    assert!(!weak.upgrade().is_null());
    let retained = weak.upgrade();
    assert_eq!(retained.nb_shapes(), 0);
    assert!(retained.data_structure().is_null());
    drop(retained);
    drop(extraction);
    assert!(weak.upgrade().is_null());
}
#[path = "algo_numeric/cases_00.rs"]
mod cases_00;
#[path = "algo_numeric/cases_01.rs"]
mod cases_01;
#[path = "algo_numeric/cases_02.rs"]
mod cases_02;
#[path = "algo_numeric/cases_03.rs"]
mod cases_03;
#[path = "algo_numeric/cases_04.rs"]
mod cases_04;
