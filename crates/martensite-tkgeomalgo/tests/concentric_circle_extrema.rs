//! Whole original bugs/modalg_7/bug31407_1..3 and bug32874_1..5 at
//!; original trims and strict predicates.
#[path = "common/isolated_case.rs"]
mod isolated_case;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomCircle, GeomCurve, GeomTrimmedCurve};
use martensite_tkgeomalgo::geom_api::ExtremaCurveCurve;
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use std::f64::consts::PI;
#[path = "support/spatial_extrema_output.rs"]
mod spatial_output;

// The command has distinct text branches, so a solver count alone is insufficient.
enum SpatialOutput {
    ParallelDistance(f64),
    Words(Vec<String>),
}

fn circle(radius: f64, normal_z: f64) -> Handle<dyn GeomCurve> {
    let normal = Direction3::new(0.0, 0.0, normal_z).unwrap();
    let frame = Axis2::new(Point3::default(), normal).unwrap();
    Handle::new(GeomCircle::new(frame, radius).unwrap()).cast()
}

fn trimmed(curve: &Handle<dyn GeomCurve>, first: f64, last: f64) -> Handle<dyn GeomCurve> {
    // Native trim uses sense=true and adjust-periodic=true, including starts at 2*pi.
    Handle::new(GeomTrimmedCurve::new(curve, first, last, true, true).unwrap()).cast()
}

fn extrema_on_bounds(a: &Handle<dyn GeomCurve>, b: &Handle<dyn GeomCurve>) -> ExtremaCurveCurve {
    ExtremaCurveCurve::with_ranges(
        a,
        b,
        (a.first_parameter(), a.last_parameter()),
        (b.first_parameter(), b.last_parameter()),
    )
    .unwrap()
}

fn command_output(a: &Handle<dyn GeomCurve>, b: &Handle<dyn GeomCurve>) -> SpatialOutput {
    let extrema = extrema_on_bounds(a, b);
    if extrema.extrema().unwrap().is_parallel().unwrap() {
        return SpatialOutput::ParallelDistance(extrema.lower_distance().unwrap());
    }
    let mut words = Vec::new();
    for index in 1..=extrema.nb_extrema() {
        let (first, second) = extrema.points(index).unwrap();
        words.extend(spatial_output::spatial_result_words(index, first, second));
    }
    if words.is_empty() {
        words.extend(["No".into(), "solutions!".into()]);
    }
    SpatialOutput::Words(words)
}

fn assert_parallel_circles(first_normal_z: f64) {
    let output = command_output(&circle(5.0, first_normal_z), &circle(10.0, 1.0));
    assert!(
        matches!(output, SpatialOutput::ParallelDistance(_)),
        "original regexp requires Infinite number of extremas"
    );
}

#[test]
fn bug31407_1_original_concentric_circles() {
    run_concentric_case(
        "concentric_circle_extrema::bug31407_1_original_concentric_circles",
        bug31407_1_original_concentric_circles_body,
    );
}

fn bug31407_1_original_concentric_circles_body() {
    assert_parallel_circles(1.0);
}

#[test]
fn bug31407_2_original_opposite_normal_circles() {
    run_concentric_case(
        "concentric_circle_extrema::bug31407_2_original_opposite_normal_circles",
        bug31407_2_original_opposite_normal_circles_body,
    );
}

fn bug31407_2_original_opposite_normal_circles_body() {
    assert_parallel_circles(-1.0);
}

#[test]
fn bug31407_3_original_disjoint_trim_result_list() {
    run_concentric_case(
        "concentric_circle_extrema::bug31407_3_original_disjoint_trim_result_list",
        bug31407_3_original_disjoint_trim_result_list_body,
    );
}

fn bug31407_3_original_disjoint_trim_result_list_body() {
    let first = trimmed(&circle(5.0, 1.0), 0.0, 2.0);
    let second = trimmed(&circle(10.0, 1.0), 2.0, 4.0);
    let SpatialOutput::Words(words) = command_output(&first, &second) else {
        panic!("original regexp rejects Infinite number of extremas");
    };
    assert_eq!(words.len(), 1, "original llength result: {words:?}");
}

fn assert_original_distance(a: &Handle<dyn GeomCurve>, b: &Handle<dyn GeomCurve>, expected: f64) {
    let SpatialOutput::ParallelDistance(distance) = command_output(a, b) else {
        panic!("original regexp requires parallel curves, expected gap {expected}");
    };
    assert!(
        (distance - expected).abs() < 1.0e-7,
        "original strict distance error: measured {distance}, expected {expected}"
    );
}

#[test]
fn bug32874_1_original_all_71_progressive_trims() {
    run_concentric_case(
        "concentric_circle_extrema::bug32874_1_original_all_71_progressive_trims",
        bug32874_1_original_all_71_progressive_trims_body,
    );
}

fn bug32874_1_original_all_71_progressive_trims_body() {
    let (first, second) = (circle(100.0, 1.0), circle(110.0, 1.0));
    let mut iterations = 0;
    for index in 1..72 {
        let last = 2.0 * PI / 72.0 * f64::from(index);
        assert_original_distance(
            &trimmed(&first, 0.0, last),
            &trimmed(&second, 0.0, last),
            10.0,
        );
        iterations += 1;
    }
    assert_eq!(iterations, 71);
}

const START_PARAMETERS: [f64; 5] = [0.0, 0.5 * PI, PI, 1.5 * PI, 2.0 * PI];
const DELTA_PARAMETERS: [f64; 7] = [
    PI / 4.0,
    PI / 3.0,
    PI / 2.0,
    0.55 * PI,
    0.99 * PI,
    PI,
    1.1 * PI,
];

fn assert_original_start_delta_grid(radius: f64, second_radius: f64, expected: f64) {
    let (first, second) = (circle(radius, 1.0), circle(second_radius, 1.0));
    let mut iterations = 0;
    for start in START_PARAMETERS {
        for delta in DELTA_PARAMETERS {
            let a = trimmed(&first, start, start + delta);
            let b = trimmed(&second, start, start + delta);
            assert_original_distance(&a, &b, expected);
            iterations += 1;
        }
    }
    assert_eq!(iterations, 35);
}

#[test]
fn bug32874_2_original_large_circle_small_gap() {
    run_concentric_case(
        "concentric_circle_extrema::bug32874_2_original_large_circle_small_gap",
        bug32874_2_original_large_circle_small_gap_body,
    );
}

fn bug32874_2_original_large_circle_small_gap_body() {
    assert_original_start_delta_grid(100.0, 100.0 + 3.5, 3.5);
}

#[test]
fn bug32874_3_original_small_circle_small_gap() {
    run_concentric_case(
        "concentric_circle_extrema::bug32874_3_original_small_circle_small_gap",
        bug32874_3_original_small_circle_small_gap_body,
    );
}

fn bug32874_3_original_small_circle_small_gap_body() {
    assert_original_start_delta_grid(0.001, 0.001 + 0.00001, 0.00001);
}

#[test]
fn bug32874_4_original_small_circle_large_gap() {
    run_concentric_case(
        "concentric_circle_extrema::bug32874_4_original_small_circle_large_gap",
        bug32874_4_original_small_circle_large_gap_body,
    );
}

fn bug32874_4_original_small_circle_large_gap_body() {
    assert_original_start_delta_grid(0.001, 0.001 + 1000.0, 1000.0);
}

#[test]
fn bug32874_5_original_large_circle_inward_small_gap() {
    run_concentric_case(
        "concentric_circle_extrema::bug32874_5_original_large_circle_inward_small_gap",
        bug32874_5_original_large_circle_inward_small_gap_body,
    );
}

fn bug32874_5_original_large_circle_inward_small_gap_body() {
    assert_original_start_delta_grid(1000.0, 1000.0 - 1.0e-5, 1.0e-5);
}

// bugs/begin installs CPU/elapsed1000 before any circle or trim constructor.
fn run_concentric_case(identity: &str, body: fn()) {
    isolated_case::run_isolated(identity, 1000, || {
        body();
        println!("TEST COMPLETED");
    });
}
