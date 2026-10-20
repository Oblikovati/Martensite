//! Complete initialized data corpus with defined straight-edge projection controls.
#[path = "replay_support/mod.rs"]
mod replay_support;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Handle;
use martensite_tkhlr::{
    hlr_algo::Projector,
    hlr_brep::{Data, ShapeToHLR},
    hlr_topo_brep::{MapOfShapeTool, OutLiner},
};
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;
use std::{ops::Range, sync::Mutex};
// ShapeToHLR invokes the historical contour pipeline; guard replay only, not production operations.
static HISTORICAL_DATA_CONTEXT: Mutex<()> = Mutex::new(());
const REQUESTS: &str = include_str!("fixtures/data_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/data_numeric/reference-expected.txt");
fn run_case(request: Range<usize>, expected: Range<usize>) {
    let _context = HISTORICAL_DATA_CONTEXT.lock().unwrap();
    replay_support::check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}
#[test]
fn original_data_counts_and_complete_order_are_exact() {
    let _context = HISTORICAL_DATA_CONTEXT.lock().unwrap();
    assert_eq!(REQUESTS.lines().count(), 320);
    assert_eq!(EXPECTATIONS.lines().count(), 320);
    for (request, expected) in REQUESTS.lines().zip(EXPECTATIONS.split_inclusive('\n')) {
        assert!(request.starts_with("hb_data "));
        replay_support::check_request_slice(request, expected);
    }
}
fn straight_data() -> Handle<Data> {
    let edge = BRepBuilderAPIMakeEdge::from_points(Point3::default(), Point3::new(2., 0., 0.))
        .unwrap()
        .edge()
        .unwrap();
    let outliner = Handle::new(OutLiner::with_original(&Shape::from(edge)));
    ShapeToHLR::load(
        &outliner,
        &Projector::new().unwrap(),
        &mut MapOfShapeTool::default(),
        0,
    )
    .unwrap()
}
fn count_and_bounds(records: &Data) {
    assert_eq!(
        (
            records.nb_vertices(),
            records.nb_edges(),
            records.nb_faces()
        ),
        (2, 1, 0)
    );
    let edges = records.e_data_array();
    let (first, _, last, _) = edges[1].status().bounds();
    assert_eq!((first, last), (0., 2.));
    assert!(edges[1].v_sta() > 0 && edges[1].v_end() > 0);
}
#[test]
fn initialized_open_edge_retains_count_range_and_single_edge_box_union() {
    let _context = HISTORICAL_DATA_CONTEXT.lock().unwrap();
    let handle = straight_data();
    let records = handle.get().unwrap();
    count_and_bounds(records);
    records.update(&Projector::new().unwrap()).unwrap();
    assert!(!records.e_data_array()[1].vertical());
    let union = replay_support::replay::brep_data_bounds::shape_box(records, 1, 0).unwrap();
    assert_eq!(union, *records.e_data_array()[1].min_max());
    count_and_bounds(records);
}
#[test]
fn end_on_projection_changes_visibility_flag_without_losing_topological_records() {
    let _context = HISTORICAL_DATA_CONTEXT.lock().unwrap();
    let handle = straight_data();
    let records = handle.get().unwrap();
    let end_on =
        Projector::with_frame(Axis2::new(Point3::default(), Direction3::X).unwrap()).unwrap();
    records.update(&end_on).unwrap();
    assert!(records.e_data_array()[1].vertical());
    count_and_bounds(records);
    records.update(&Projector::new().unwrap()).unwrap();
    assert!(!records.e_data_array()[1].vertical());
    count_and_bounds(records);
}
#[path = "data_numeric/cases_00.rs"]
mod cases_00;
#[path = "data_numeric/cases_01.rs"]
mod cases_01;
#[path = "data_numeric/cases_02.rs"]
mod cases_02;
#[path = "data_numeric/cases_03.rs"]
mod cases_03;
