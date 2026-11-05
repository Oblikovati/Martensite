//! Regression of a cut-result common regression.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;

#[test]
fn BCommonSimpleTest__CutResultAgainstSource_J2() {
    let source = CreateBox(Point3::new(0.0, 0.0, 0.0), 10.0, 10.0, 10.0);
    let cylinder = Translate(&CreateCylinder(2.0, 4.0), 5.0, 5.0, -2.0);
    let cut_result = PerformCut(&source, &cylinder);
    let result = PerformCommon(&cut_result, &source);
    ValidateResult(&result, 625.133, -1.0, false);
}
