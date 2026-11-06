//! Regressions of cylinder and cut-result fuse regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;

#[test]
fn BFuseSimpleTest__OverlappingCylinders_L2() {
    let first = CreateCylinder(20.0, 100.0);
    let second = Translate(&CreateCylinder(20.0, 100.0), 0.0, 0.0, 50.0);
    ValidateResult(&PerformFuse(&first, &second), 21362.8, -1.0, false);
}

#[test]
fn BFuseSimpleTest__CutResultWithSource_L3() {
    let source = CreateBox(Point3::new(0.0, 0.0, 0.0), 10.0, 10.0, 10.0);
    let cylinder = Translate(&CreateCylinder(2.0, 4.0), 5.0, 5.0, -2.0);
    let cut = PerformCut(&source, &cylinder);
    ValidateResult(&PerformFuse(&cut, &source), 600.0, -1.0, false);
}
