//! Regressions of nested primitive common regressions.
#![allow(non_snake_case)]
#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
#[test]
fn BOPCommonSimpleTest__CylinderCone_ZJ6() {
    ValidateResult(
        &PerformCommon(&CreateCylinder(4.0, 8.0), &CreateCone(2.0, 1.0, 8.0)),
        91.693,
        -1.0,
        false,
    );
}
#[test]
fn BOPCommonSimpleTest__Cones_ZM3() {
    ValidateResult(
        &PerformCommon(&CreateCone(8.0, 4.0, 8.0), &CreateCone(2.0, 1.0, 8.0)),
        91.693,
        -1.0,
        false,
    );
}
