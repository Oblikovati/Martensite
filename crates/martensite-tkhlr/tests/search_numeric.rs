//! Whole retained interior-search corpus and independent analytic seed checks.
use replay_support::replay::geometry_records;
#[path = "replay_support/mod.rs"]
mod replay_support;
use geometry_records::contour_surface_fixture;
use martensite_tkg3d::adaptor3d::h_surface_tool::Adaptor3dSurfaceHandle;
use martensite_tkhlr::contap::{ContapSurfFunction, ContapTheSearchInside};
use martensite_tkmath::gp::{Axis3, Direction3, Point3};
use replay_support::check_request_slice;
use std::{ops::Range, sync::Mutex};
// SearchInside resets its function through the historical process-wide sampler.
// This guard supports compatibility replay only, not independent production operations.
static HISTORICAL_INSIDE_SAMPLE_CONTEXT: Mutex<()> = Mutex::new(());
const REQUESTS: &str = include_str!("fixtures/search_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/search_numeric/reference-expected.txt");

fn run_case(request: Range<usize>, expected: Range<usize>) {
    let _context = HISTORICAL_INSIDE_SAMPLE_CONTEXT.lock().unwrap();
    check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}

#[test]
fn original_interior_request_and_expected_counts_are_exact() {
    assert_eq!(REQUESTS.lines().count(), 144);
    assert_eq!(EXPECTATIONS.lines().count(), 144);
    assert!(REQUESTS.lines().all(|line| line.starts_with("ct_inside ")));
}

#[test]
fn complete_original_interior_order_matches_immutable_observations() {
    let _context = HISTORICAL_INSIDE_SAMPLE_CONTEXT.lock().unwrap();
    for (request, expected) in REQUESTS.lines().zip(EXPECTATIONS.split_inclusive('\n')) {
        check_request_slice(request, expected);
    }
}

fn analytic_surface(kind: i32) -> Adaptor3dSurfaceHandle {
    contour_surface_fixture::from_fields(kind, Axis3::default(), 2., 0., || (-1., 1., -1., 1.))
        .unwrap()
}

fn orthographic_function(surface: &Adaptor3dSurfaceHandle) -> ContapSurfFunction {
    let mut function = ContapSurfFunction::new();
    function.set_surface(surface).unwrap();
    function.set_direction(Direction3::Z);
    function.set_tolerance(1e-7);
    function
}

#[test]
fn fresh_interior_search_distinguishes_unperformed_state() {
    let search = ContapTheSearchInside::new();
    assert!(!search.is_done());
    assert_eq!(
        search.nb_points().unwrap_err().exception_type(),
        "StdFail_NotDone"
    );
    assert_eq!(
        search.value(1).unwrap_err().exception_type(),
        "StdFail_NotDone"
    );
}

#[test]
fn spherical_equator_seed_and_repeated_outside_start_follow_closed_form() {
    let _context = HISTORICAL_INSIDE_SAMPLE_CONTEXT.lock().unwrap();
    let surface = analytic_surface(1);
    let mut function = orthographic_function(&surface);
    let mut search = ContapTheSearchInside::new();
    search
        .perform_from(&mut function, &surface, 0., 0.)
        .unwrap();
    assert!(search.is_done());
    assert_eq!(search.nb_points().unwrap(), 1);
    assert_equatorial_seed(search.value(1).unwrap());
    search
        .perform_from(&mut function, &surface, 2., 2.)
        .unwrap();
    assert!(search.is_done());
    assert_eq!(search.nb_points().unwrap(), 0);
    assert_eq!(
        search.value(1).unwrap_err().exception_type(),
        "Standard_OutOfRange"
    );
}

fn assert_equatorial_seed(seed: &martensite_tkgeomalgo::int_surf::IntSurfInteriorPoint) {
    // For p(u,v)=2(cos(v)cos(u),cos(v)sin(u),sin(v)), the Z silhouette is v=0.
    assert_eq!(seed.value(), Point3::new(2., 0., 0.));
    assert_eq!(seed.parameters(), (0., 0.));
    assert_eq!((seed.direction().x(), seed.direction().z()), (0., 0.));
    assert_ne!(seed.direction().y(), 0.);
    assert_eq!(seed.direction2d().y(), 0.);
    assert_eq!(seed.direction2d().x().abs(), 1.);
}

#[test]
fn horizontal_plane_has_no_orthographic_silhouette_seed() {
    let _context = HISTORICAL_INSIDE_SAMPLE_CONTEXT.lock().unwrap();
    let surface = analytic_surface(0);
    let mut function = orthographic_function(&surface);
    let mut search = ContapTheSearchInside::new();
    for (u, v) in [(0., 0.), (0.5, -0.5), (2., 2.)] {
        search.perform_from(&mut function, &surface, u, v).unwrap();
        assert!(search.is_done());
        assert_eq!(search.nb_points().unwrap(), 0);
    }
}

#[path = "search_numeric/cases_00.rs"]
mod cases_00;

#[path = "search_numeric/cases_01.rs"]
mod cases_01;
