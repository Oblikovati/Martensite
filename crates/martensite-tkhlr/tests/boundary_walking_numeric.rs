//! Whole boundary and contour-walking requests with owned analytic lifecycle checks.
use replay_support::replay::geometry_records;
#[path = "replay_support/mod.rs"]
mod replay_support;
use geometry_records::contour_surface_fixture;
use martensite_tkg3d::adaptor3d::{Adaptor3dTopolTool, h_surface_tool::Adaptor3dSurfaceHandle};
use martensite_tkhlr::contap::{
    ContapArcFunction, ContapSurfFunction, ContapTheIWalking, ContapTheSearch,
};
use martensite_tkmath::gp::{Axis3, Direction3, Point3};
use replay_support::check_request_slice;
use std::{ops::Range, sync::Mutex};
// These operations still invoke the historical sampler through SetSurface.
// Serializing compatibility replay proves no independent-production thread-safety claim.
static HISTORICAL_BOUNDARY_WALK_SAMPLE_CONTEXT: Mutex<()> = Mutex::new(());
const REQUESTS: &str = include_str!("fixtures/boundary_walking_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/boundary_walking_numeric/reference-expected.txt");

fn run_case(request: Range<usize>, expected: Range<usize>) {
    let _context = HISTORICAL_BOUNDARY_WALK_SAMPLE_CONTEXT.lock().unwrap();
    check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}

#[test]
fn complete_original_boundary_walking_request_counts_are_exact() {
    assert_eq!(REQUESTS.lines().count(), 432);
    assert_eq!(EXPECTATIONS.lines().count(), 432);
    for family in ["ct_boundary ", "ct_walking "] {
        assert_eq!(
            REQUESTS
                .lines()
                .filter(|line| line.starts_with(family))
                .count(),
            216
        );
    }
}

#[test]
fn original_boundary_and_walking_order_matches_immutable_observations() {
    let _context = HISTORICAL_BOUNDARY_WALK_SAMPLE_CONTEXT.lock().unwrap();
    for (request, expected) in REQUESTS.lines().zip(EXPECTATIONS.split_inclusive('\n')) {
        check_request_slice(request, expected);
    }
}

fn horizontal_patch() -> Adaptor3dSurfaceHandle {
    contour_surface_fixture::from_fields(0, Axis3::default(), 2., 0., || (-1., 1., -1., 1.))
        .unwrap()
}

#[test]
fn plane_boundary_transitions_between_empty_and_four_complete_segments() {
    let _context = HISTORICAL_BOUNDARY_WALK_SAMPLE_CONTEXT.lock().unwrap();
    let surface = horizontal_patch();
    let domain = Adaptor3dTopolTool::with_surface(&surface).unwrap();
    let mut function = ContapArcFunction::new();
    function.set_surface(&surface).unwrap();
    let mut search = ContapTheSearch::new();
    for direction in [Direction3::Z, Direction3::X, Direction3::Z] {
        function.set_direction(direction);
        search
            .perform(&mut function, &domain, 1e-7, 1e-7, false)
            .unwrap();
        assert!(search.is_done());
        assert_boundary_corners(&search, direction == Direction3::X);
        assert_eq!(
            search.all_arc_solution().unwrap(),
            direction == Direction3::X
        );
        assert_eq!(
            search.nb_segments().unwrap(),
            if direction == Direction3::X { 4 } else { 0 }
        );
    }
}

fn assert_boundary_corners(search: &ContapTheSearch, coincident: bool) {
    // Each fully coincident restriction retains its own two endpoint records.
    assert_eq!(search.nb_points().unwrap(), if coincident { 8 } else { 0 });
    if !coincident {
        return;
    }
    for (x, y) in [(-1., -1.), (1., -1.), (1., 1.), (-1., 1.)] {
        let corner = Point3::new(x, y, 0.);
        let occurrences = (1..=8)
            .filter(|index| search.point(*index).unwrap().value() == corner)
            .count();
        assert_eq!(occurrences, 2);
    }
}

#[test]
fn empty_walking_reuses_both_overloads_without_publishing_stale_results() {
    let _context = HISTORICAL_BOUNDARY_WALK_SAMPLE_CONTEXT.lock().unwrap();
    let surface = horizontal_patch();
    let mut function = ContapSurfFunction::new();
    function.set_surface(&surface).unwrap();
    function.set_direction(Direction3::Z);
    let mut walking = ContapTheIWalking::new(1e-7, 0.01, 0.1, true).unwrap();
    assert!(!walking.is_done());
    for reversed in [false, true, false] {
        walking
            .perform(&[], &[], &mut function, &surface, reversed)
            .unwrap();
        assert_empty_walking(&walking);
        walking
            .perform_open(&[], &mut function, &surface, reversed)
            .unwrap();
        assert_empty_walking(&walking);
    }
}

fn assert_empty_walking(walking: &ContapTheIWalking) {
    assert!(walking.is_done());
    assert_eq!(walking.nb_lines().unwrap(), 0);
    assert_eq!(walking.nb_single_pnts().unwrap(), 0);
    assert!(walking.value(1).is_err());
    assert!(walking.single_pnt(1).is_err());
}

#[path = "boundary_walking_numeric/cases_00.rs"]
mod cases_00;

#[path = "boundary_walking_numeric/cases_01.rs"]
mod cases_01;

#[path = "boundary_walking_numeric/cases_02.rs"]
mod cases_02;

#[path = "boundary_walking_numeric/cases_03.rs"]
mod cases_03;

#[path = "boundary_walking_numeric/cases_04.rs"]
mod cases_04;

#[path = "boundary_walking_numeric/cases_05.rs"]
mod cases_05;
