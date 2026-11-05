//! Shared construction for direct box-cut regressions.

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::*;
use martensite_tkbo::bop_algo::Operation;
use martensite_tkmath::gp::Point3;

#[allow(dead_code)]
pub fn cut_with_box(second: [f64; 6], expected_area: f64) {
    let first = CreateBox(Point3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0);
    let second = CreateBox(
        Point3::new(second[0], second[1], second[2]),
        second[3],
        second[4],
        second[5],
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        expected_area,
        -1.0,
        false,
    );
}

pub fn cut_with_rotated_box(
    second: [f64; 6],
    pivot: Point3,
    angle_degrees: f64,
    expected_area: f64,
) {
    let first = CreateBox(Point3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0);
    let mut second = CreateBox(
        Point3::new(second[0], second[1], second[2]),
        second[3],
        second[4],
        second[5],
    );
    RotateAroundAxis(&mut second, pivot, [0.0, 0.0, 1.0], angle_degrees);
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        expected_area,
        -1.0,
        false,
    );
}
