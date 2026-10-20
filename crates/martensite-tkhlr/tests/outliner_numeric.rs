//! Whole retained outlined topology corpus with independent empty-shape controls.
#[path = "replay_support/mod.rs"]
mod replay_support;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Shape},
};
use martensite_tkhlr::{
    hlr_algo::Projector,
    hlr_topo_brep::{MapOfShapeTool, OutLiner},
};
use std::{ops::Range, sync::Mutex};
// Compatibility replay retains the historical sampler called by the contour pipeline.
// This guard establishes no independent-production operation or thread-safety claim.
static HISTORICAL_OUTLINER_CONTEXT: Mutex<()> = Mutex::new(());
const REQUESTS: &str = include_str!("fixtures/outliner_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/outliner_numeric/reference-expected.txt");
fn run_case(request: Range<usize>, expected: Range<usize>) {
    let _context = HISTORICAL_OUTLINER_CONTEXT.lock().unwrap();
    replay_support::check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}
#[test]
fn complete_original_outliner_count_and_order_are_exact() {
    let _context = HISTORICAL_OUTLINER_CONTEXT.lock().unwrap();
    assert_eq!(REQUESTS.lines().count(), 240);
    assert_eq!(EXPECTATIONS.lines().count(), 240);
    for (request, expected) in REQUESTS.lines().zip(EXPECTATIONS.split_inclusive('\n')) {
        assert!(request.starts_with("ht_outliner "));
        replay_support::check_request_slice(request, expected);
    }
}
fn empty_compound() -> Shape {
    let mut compound = Compound::default();
    BRepBuilder::default().make_compound(&mut compound);
    Shape::from(compound)
}
#[test]
fn null_original_does_not_initialize_outline_or_tools() {
    let outliner = OutLiner::new();
    let mut tools = MapOfShapeTool::default();
    outliner
        .fill(&Projector::new().unwrap(), &mut tools, 3)
        .unwrap();
    assert!(outliner.original_shape().is_null());
    assert!(outliner.out_lined_shape().is_null());
    assert!(tools.is_empty());
}
#[test]
fn empty_original_builds_one_stable_empty_outline() {
    let _context = HISTORICAL_OUTLINER_CONTEXT.lock().unwrap();
    let original = empty_compound();
    let outliner = OutLiner::with_original(&original);
    let mut tools = MapOfShapeTool::default();
    outliner
        .fill(&Projector::new().unwrap(), &mut tools, 0)
        .unwrap();
    let outlined = outliner.out_lined_shape();
    assert!(!outlined.is_null());
    assert!(outliner.original_shape().is_same(&original));
    outliner
        .fill(&Projector::new().unwrap(), &mut tools, 4)
        .unwrap();
    assert!(outliner.out_lined_shape().is_same(&outlined));
    assert!(tools.is_empty());
}
#[test]
fn supplied_outline_is_retained_without_processing_original() {
    let original = empty_compound();
    let supplied = empty_compound();
    let outliner = OutLiner::with_shapes(&original, &supplied);
    let mut tools = MapOfShapeTool::default();
    outliner
        .fill(&Projector::new().unwrap(), &mut tools, 7)
        .unwrap();
    assert!(outliner.out_lined_shape().is_same(&supplied));
    assert!(outliner.original_shape().is_same(&original));
    assert!(tools.is_empty());
}
#[path = "outliner_numeric/cases_00.rs"]
mod cases_00;
#[path = "outliner_numeric/cases_01.rs"]
mod cases_01;
#[path = "outliner_numeric/cases_02.rs"]
mod cases_02;
