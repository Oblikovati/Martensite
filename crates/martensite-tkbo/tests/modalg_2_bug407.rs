//! Native replay of OCC407 Boolean regression cases.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{
    CreateCylinder, PerformCut, PerformFuse, RotateAroundAxis, ValidateResult,
};
use martensite_tkmath::gp::Point3;

#[test]
fn Modalg2__Bug407_1() {
    let first = CreateCylinder(1., 4.);
    let mut second = CreateCylinder(1., 4.);
    RotateAroundAxis(&mut second, Point3::new(0., 0., 2.), [1., 0., 0.], 90.);
    RotateAroundAxis(&mut second, Point3::new(0., 0., 2.), [0., 1., 0.], 60.);
    ValidateResult(&PerformFuse(&second, &first), 46.8319, -1., false);
}

#[test]
fn Modalg2__Bug407_2() {
    let first = CreateCylinder(1., 4.);
    let mut second = CreateCylinder(1., 4.);
    RotateAroundAxis(&mut second, Point3::new(0., 0., 2.), [1., 0., 0.], 90.);
    RotateAroundAxis(&mut second, Point3::new(0., 0., 2.), [0., 1., 0.], 60.);
    ValidateResult(&PerformCut(&second, &first), 31.4159, -1., false);
}
