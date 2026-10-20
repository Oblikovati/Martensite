//! Frozen frame/projector/container/native record requests with direct owned execution.
use super::foundation_numeric::check_request_slice;
use std::ops::Range;
const REQUESTS: &str = include_str!("fixtures/algorithm_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/algorithm_numeric/reference-expected.txt");

fn run_case(request: Range<usize>, expected: Range<usize>) {
    check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}

#[test]
fn original_algorithm_request_and_result_counts_are_exact() {
    assert_eq!(REQUESTS.lines().count(), 548);
    assert_eq!(EXPECTATIONS.lines().count(), 548);
}

#[path = "algorithm_numeric/ha_bipoint_00.rs"]
mod ha_bipoint_00;

#[path = "algorithm_numeric/ha_edges_00.rs"]
mod ha_edges_00;

#[path = "algorithm_numeric/ha_encode_00.rs"]
mod ha_encode_00;

#[path = "algorithm_numeric/ha_grow_00.rs"]
mod ha_grow_00;

#[path = "algorithm_numeric/ha_internal_00.rs"]
mod ha_internal_00;

#[path = "algorithm_numeric/ha_minmax_00.rs"]
mod ha_minmax_00;

#[path = "algorithm_numeric/ha_polyalgo_00.rs"]
mod ha_polyalgo_00;

#[path = "algorithm_numeric/ha_polyalgo_01.rs"]
mod ha_polyalgo_01;

#[path = "algorithm_numeric/ha_polydata_00.rs"]
mod ha_polydata_00;

#[path = "algorithm_numeric/ha_polydata_01.rs"]
mod ha_polydata_01;

#[path = "algorithm_numeric/ha_projector_00.rs"]
mod ha_projector_00;

#[path = "algorithm_numeric/ha_records_00.rs"]
mod ha_records_00;

#[path = "algorithm_numeric/ha_status_00.rs"]
mod ha_status_00;

#[path = "algorithm_numeric/ha_status_01.rs"]
mod ha_status_01;

#[path = "algorithm_numeric/ha_wires_00.rs"]
mod ha_wires_00;

#[path = "algorithm_numeric/hl_frame_00.rs"]
mod hl_frame_00;
