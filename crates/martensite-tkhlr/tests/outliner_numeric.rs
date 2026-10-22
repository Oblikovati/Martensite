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
use std::{
    ops::Range,
    sync::{
        Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};
// Compatibility replay retains the historical sampler called by the contour pipeline.
// This guard establishes no independent-production operation or thread-safety claim.
static HISTORICAL_OUTLINER_CONTEXT: Mutex<()> = Mutex::new(());
static OUTLINER_REPLAY_COUNT: AtomicUsize = AtomicUsize::new(0);
const REQUESTS: &str = include_str!("fixtures/outliner_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/outliner_numeric/reference-expected.txt");
const CASE_SLICES: &str = include_str!("fixtures/outliner_numeric/case-slices.csv");

#[derive(Clone)]
struct OutlinerCaseSlice {
    request: Range<usize>,
    expected: Range<usize>,
}

fn parse_slice_boundary(value: Option<&str>, index: usize, field: &str) -> usize {
    value
        .unwrap_or_else(|| panic!("case {index} lacks {field} boundary"))
        .parse()
        .unwrap_or_else(|_| panic!("case {index} has an invalid {field} boundary"))
}

fn parse_case_slice(index: usize, line: &str) -> OutlinerCaseSlice {
    let mut boundaries = line.split(',');
    let request_start = parse_slice_boundary(boundaries.next(), index, "request start");
    let request_end = parse_slice_boundary(boundaries.next(), index, "request end");
    let expected_start = parse_slice_boundary(boundaries.next(), index, "expected start");
    let expected_end = parse_slice_boundary(boundaries.next(), index, "expected end");
    assert!(
        boundaries.next().is_none(),
        "case {index} has extra boundaries"
    );
    OutlinerCaseSlice {
        request: request_start..request_end,
        expected: expected_start..expected_end,
    }
}

fn outliner_case(index: usize) -> OutlinerCaseSlice {
    let line = CASE_SLICES
        .lines()
        .nth(index)
        .unwrap_or_else(|| panic!("outliner case index {index} is not mapped"));
    parse_case_slice(index, line)
}

fn all_outliner_cases() -> Vec<OutlinerCaseSlice> {
    CASE_SLICES
        .lines()
        .enumerate()
        .map(|(index, line)| parse_case_slice(index, line))
        .collect()
}

fn run_case(case: OutlinerCaseSlice) {
    let _context = HISTORICAL_OUTLINER_CONTEXT.lock().unwrap();
    OUTLINER_REPLAY_COUNT.fetch_add(1, Ordering::Relaxed);
    replay_support::check_request_slice(&REQUESTS[case.request], &EXPECTATIONS[case.expected]);
}

fn assert_complete_original_outliner_layout(cases: &[OutlinerCaseSlice]) {
    assert_eq!(cases.len(), 240);
    assert_eq!(REQUESTS.lines().count(), cases.len());
    assert_eq!(EXPECTATIONS.lines().count(), cases.len());

    let mut request_end = 0;
    let mut expected_end = 0;
    for case in cases {
        assert_eq!(case.request.start, request_end);
        assert_eq!(case.expected.start, expected_end);
        assert!(REQUESTS[case.request.clone()].starts_with("ht_outliner "));
        assert!(EXPECTATIONS[case.expected.clone()].ends_with('\n'));
        request_end = case.request.end;
        expected_end = case.expected.end;
    }
    assert_eq!(request_end, REQUESTS.len());
    assert_eq!(expected_end, EXPECTATIONS.len());
}

#[test]
fn complete_original_outliner_count_and_order_are_exact() {
    let _context = HISTORICAL_OUTLINER_CONTEXT.lock().unwrap();
    let replay_count = OUTLINER_REPLAY_COUNT.load(Ordering::Relaxed);
    assert_complete_original_outliner_layout(&all_outliner_cases());
    assert_eq!(OUTLINER_REPLAY_COUNT.load(Ordering::Relaxed), replay_count);
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
