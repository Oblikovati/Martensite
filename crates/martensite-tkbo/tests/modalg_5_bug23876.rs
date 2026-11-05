//! Native replay of the OCC23876 Boolean regression.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{
    CreateFaceFromWire, CreatePolygonWire, CreateRevolution, CreateTorus, PerformCommon, Translate,
    ValidateResult,
};
use martensite_tkmath::gp::{Axis1, Direction3, Point3};
use std::f64::consts::TAU;

fn revolved_rectangle() -> martensite_tkbrep::topods::Shape {
    let profile = CreatePolygonWire(
        &[
            Point3::new(10., 0., 0.),
            Point3::new(10., 0., 50.),
            Point3::new(20., 0., 50.),
            Point3::new(20., 0., 0.),
        ],
        true,
    );
    let face = CreateFaceFromWire(&profile);
    let axis = Axis1::new(Point3::new(0., 0., 0.), Direction3::Z);
    CreateRevolution(&face, axis, TAU)
}

#[test]
fn Modalg5__Bug23876() {
    let torus = Translate(&CreateTorus(15., 5.), 0., 0., 50.);
    let result = PerformCommon(&revolved_rectangle(), &torus);
    ValidateResult(&result, 2422.92, -1., false);
}
