//! Complete retained contour-function requests and independent planar equations.
use replay_support::replay::geometry_records;
#[path = "replay_support/mod.rs"]
mod replay_support;
use replay_support::{check_request_slice, replay};
use std::sync::Mutex;
// Native contour sampling stores a process-wide range; isolate this executable and
// serialize complete calls rather than changing production state or expected values.
static CONTOUR_FUNCTION_SAMPLE_CONTEXT: Mutex<()> = Mutex::new(());
use geometry_records::{common::Reader, contour_surface_fixture, curve2d};
use martensite_tkg3d::adaptor3d::h_surface_tool::Adaptor3dSurfaceHandle;
use martensite_tkhlr::contap::{ContapArcFunction, ContapSurfFunction};
use martensite_tkmath::{
    gp::{Axis3, Point3},
    math::{Matrix, Vector},
};
use std::ops::Range;
const REQUESTS: &str = include_str!("fixtures/function_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/function_numeric/reference-expected.txt");

fn run_case(request: Range<usize>, expected: Range<usize>) {
    let _context = CONTOUR_FUNCTION_SAMPLE_CONTEXT.lock().unwrap();
    check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}

fn plane() -> Adaptor3dSurfaceHandle {
    contour_surface_fixture::from_fields(0, Axis3::default(), 1., 0., || (0., 1., 0., 1.)).unwrap()
}

#[test]
fn original_function_request_and_result_counts_are_exact() {
    assert_eq!(
        REQUESTS
            .lines()
            .filter(|line| line.starts_with("ct_function "))
            .count(),
        432
    );
    assert_eq!(
        REQUESTS
            .lines()
            .filter(|line| line.starts_with("ct_arc_function "))
            .count(),
        864
    );
    assert_eq!(EXPECTATIONS.lines().count(), 1296);
}

#[test]
fn planar_surface_equation_has_unit_normal_and_constant_eye_projection() {
    let _context = CONTOUR_FUNCTION_SAMPLE_CONTEXT.lock().unwrap();
    let mut function = ContapSurfFunction::new();
    function.set_surface(&plane()).unwrap();
    let x = Vector::filled(1, 2, 0.25).unwrap();
    let mut value = Vector::filled(1, 1, 17.).unwrap();
    function.value(&x, &mut value).unwrap();
    assert_eq!(value[1], 1.);
    function.set_eye(Point3::new(1., 2., 3.));
    let mut gradient = Matrix::filled(1, 1, 1, 2, 19.).unwrap();
    function.values(&x, &mut value, &mut gradient).unwrap();
    assert_eq!(value[1], -3.);
    assert_eq!(function.root(), -3.);
    assert_eq!(function.point(), Point3::new(0.25, 0.25, 0.));
    assert_eq!((gradient[(1, 1)], gradient[(1, 2)]), (0., 0.));
}

#[test]
fn planar_arc_equation_is_constant_and_saved_points_follow_the_line() {
    let _context = CONTOUR_FUNCTION_SAMPLE_CONTEXT.lock().unwrap();
    let mut function = planar_arc_function();
    assert_eq!(function.values_at(0.25, 17.).unwrap(), (1., 0.));
    function.set_eye(Point3::new(1., 2., 3.));
    assert_eq!(function.values_at(0.25, 17.).unwrap(), (-3., 0.));
    let saved = function.get_state_number();
    assert_eq!(saved, 1);
    assert_eq!(function.valpoint(saved).unwrap(), Point3::new(0.25, 0., 0.));
    assert_eq!(function.values_at(0.75, 17.).unwrap(), (-3., 0.));
    assert_eq!(function.valpoint(saved).unwrap(), Point3::new(0.25, 0., 0.));
    assert_eq!(function.last_computed_point(), Point3::new(0.75, 0., 0.));
}

fn planar_arc_function() -> ContapArcFunction {
    let mut reader = Reader::new(&[0., 0., 0., 1., 0.]);
    let curve = curve2d::curve2d(&mut reader).unwrap();
    let arc = replay::arc_fixture::from_fields(&curve, false, || (0., 1.)).unwrap();
    let mut function = ContapArcFunction::new();
    function.set_surface(&plane()).unwrap();
    function.set_arc(&arc);
    function
}

#[test]
fn explicit_sampling_context_preserves_all_original_function_observations() {
    let mut context = martensite_tkhlr::contap::ContapSamplingContext::new();
    for (request, expected) in REQUESTS
        .split_inclusive('\n')
        .zip(EXPECTATIONS.split_inclusive('\n'))
    {
        replay_support::check_request_slice_with(request, expected, |line| {
            replay::execute_request_with_context(line, &mut context)
        });
    }
    assert_eq!(replay::execute_request_with_context("", &mut context), None);
    assert_eq!(
        replay::execute_request_with_context("unknown 0", &mut context),
        None
    );
}

#[test]
fn explicit_context_parallel_replay_matches_the_serial_bytes() {
    let requests: Vec<_> = REQUESTS.split_inclusive('\n').collect();
    let serial = explicit_request_outputs(&requests, 0, 1);
    let mut parallel = std::thread::scope(|scope| {
        let workers: Vec<_> = (0..4)
            .map(|worker| {
                let requests = &requests;
                scope.spawn(move || explicit_request_outputs(requests, worker, 4))
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|worker| worker.join().unwrap())
            .collect::<Vec<_>>()
    });
    parallel.sort_by_key(|(index, _)| *index);
    assert_eq!(parallel, serial);
}

fn explicit_request_outputs(requests: &[&str], first: usize, step: usize) -> Vec<(usize, String)> {
    let mut context = martensite_tkhlr::contap::ContapSamplingContext::new();
    (first..requests.len())
        .step_by(step)
        .map(|index| {
            (
                index,
                replay::execute_request_with_context(requests[index], &mut context).unwrap(),
            )
        })
        .collect()
}

#[path = "function_numeric/cases_00.rs"]
mod cases_00;

#[path = "function_numeric/cases_01.rs"]
mod cases_01;

#[path = "function_numeric/cases_02.rs"]
mod cases_02;

#[path = "function_numeric/cases_03.rs"]
mod cases_03;

#[path = "function_numeric/cases_04.rs"]
mod cases_04;

#[path = "function_numeric/cases_05.rs"]
mod cases_05;

#[path = "function_numeric/cases_06.rs"]
mod cases_06;

#[path = "function_numeric/cases_07.rs"]
mod cases_07;

#[path = "function_numeric/cases_08.rs"]
mod cases_08;

#[path = "function_numeric/cases_09.rs"]
mod cases_09;

#[path = "function_numeric/cases_10.rs"]
mod cases_10;

#[path = "function_numeric/cases_11.rs"]
mod cases_11;

#[path = "function_numeric/cases_12.rs"]
mod cases_12;

#[path = "function_numeric/cases_13.rs"]
mod cases_13;

#[path = "function_numeric/cases_14.rs"]
mod cases_14;

#[path = "function_numeric/cases_15.rs"]
mod cases_15;

#[path = "function_numeric/cases_16.rs"]
mod cases_16;
