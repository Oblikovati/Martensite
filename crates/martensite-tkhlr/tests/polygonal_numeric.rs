//! Complete polygonal HLR corpus using the original deterministic mesh factory.
use replay_support::replay::polygon_mesh_fixture::mesh;
#[path = "replay_support/mod.rs"]
mod replay_support;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Shape},
};
use martensite_tkernel::standard::Handle;
use martensite_tkhlr::{
    hlr_algo::Projector,
    hlr_brep::{PolyAlgo, PolyHLRToShape},
};
use std::ops::Range;
const REQUESTS: &str = include_str!("fixtures/polygonal_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/polygonal_numeric/reference-expected.txt");
fn run_case(request: Range<usize>, expected: Range<usize>) {
    replay_support::check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}
#[test]
fn original_polygonal_counts_and_complete_order_are_exact() {
    assert_eq!(REQUESTS.lines().count(), 300);
    assert_eq!(EXPECTATIONS.lines().count(), 300);
    for (request, expected) in REQUESTS.lines().zip(EXPECTATIONS.split_inclusive('\n')) {
        assert!(request.starts_with("hb_poly "));
        replay_support::check_request_slice(request, expected);
    }
}
fn empty_compound() -> Shape {
    let mut result = Compound::default();
    BRepBuilder::default().make_compound(&mut result);
    Shape::from(result)
}
fn no_segments(algo: &PolyAlgo) {
    algo.init_hide().unwrap();
    assert!(!algo.more_hide());
    algo.init_show().unwrap();
    assert!(!algo.more_show());
}
#[test]
fn repeated_empty_compound_updates_have_no_hidden_or_shown_segments() {
    let source = empty_compound();
    mesh(&source, 0, 0).unwrap();
    let handle = Handle::new(PolyAlgo::with_shape(&source).unwrap());
    let algo = handle.get().unwrap();
    algo.set_projector(&Projector::new().unwrap());
    for _ in 0..2 {
        algo.update().unwrap();
        no_segments(algo);
        let mut conversion = PolyHLRToShape::new();
        conversion.update(&handle).unwrap();
        assert!(conversion.v_compound().unwrap().is_null());
        assert!(conversion.h_compound().unwrap().is_null());
    }
}
#[test]
fn removing_and_reloading_empty_shapes_does_not_retain_iteration_state() {
    let source = empty_compound();
    let algo = PolyAlgo::with_shape(&source).unwrap();
    algo.update().unwrap();
    no_segments(&algo);
    assert_eq!((algo.nb_shapes(), algo.index(&source)), (1, 1));
    algo.remove(1).unwrap();
    assert_eq!(algo.nb_shapes(), 0);
    algo.load(&source);
    algo.update().unwrap();
    no_segments(&algo);
    assert_eq!((algo.nb_shapes(), algo.index(&source)), (1, 1));
}
#[path = "polygonal_numeric/cases_00.rs"]
mod cases_00;
#[path = "polygonal_numeric/cases_01.rs"]
mod cases_01;
#[path = "polygonal_numeric/cases_02.rs"]
mod cases_02;
#[path = "polygonal_numeric/cases_03.rs"]
mod cases_03;
