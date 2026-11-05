//! Regressions of profile-prism and box common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::*;
use martensite_tkbrep::topods::Shape;
use martensite_tkmath::gp::{Point3, Vector3};

fn profile_prism(points: [[f64; 3]; 4], direction: [f64; 3]) -> Shape {
    let vertices = points.map(|point| Point3::new(point[0], point[1], point[2]));
    let wire = CreatePolygonWire(&vertices, true);
    CreatePrism(
        &CreateFaceFromWire(&wire),
        Vector3::new(direction[0], direction[1], direction[2]),
    )
}

fn common_profile_box(points: [[f64; 3]; 4], direction: [f64; 3]) {
    let box_shape = CreateBox(Point3::new(3.0, 3.0, 0.0), 5.0, 7.0, 4.0);
    let result = PerformCommon(&box_shape, &profile_prism(points, direction));
    ValidateResult(&result, 14.0, -1.0, false);
}

#[test]
fn BCommonSimpleTest__ProfileBox_I6() {
    common_profile_box(
        [
            [7.0, 0.0, 2.0],
            [8.0, 0.0, 2.0],
            [8.0, 0.0, 3.0],
            [7.0, 0.0, 3.0],
        ],
        [0.0, 6.0, 0.0],
    );
}
#[test]
fn BCommonSimpleTest__ProfileBox_I7() {
    common_profile_box(
        [
            [7.0, 0.0, 2.0],
            [8.0, 0.0, 2.0],
            [8.0, 6.0, 2.0],
            [7.0, 6.0, 2.0],
        ],
        [0.0, 0.0, 1.0],
    );
}
#[test]
fn BCommonSimpleTest__ProfileBox_I8() {
    common_profile_box(
        [
            [8.0, 0.0, 2.0],
            [8.0, 6.0, 2.0],
            [8.0, 6.0, 3.0],
            [8.0, 0.0, 3.0],
        ],
        [-1.0, 0.0, 0.0],
    );
}
