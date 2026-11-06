//! Regression of the reversed-cavity shell common regression.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::*;
use martensite_tkbrep::{brep::BRepBuilder, topexp::Explorer, topods::Shape};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::gp::Point3;

fn shells(shape: &Shape) -> Vec<Shape> {
    Explorer::new(shape, ShapeEnum::Shell, ShapeEnum::Shape)
        .unwrap()
        .collect()
}

#[test]
fn BOPCommonSimpleTest__ReversedCavityShell_ZP8() {
    let outer = CreateBox(Point3::new(0.0, 0.0, 0.0), 10.0, 10.0, 10.0);
    let inner = Translate(
        &CreateBox(Point3::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0),
        2.0,
        2.0,
        2.0,
    );
    let cut = PerformCut(&outer, &inner);
    let parts = shells(&cut);
    assert_eq!(
        parts.len(),
        2,
        "cut must retain its outer and cavity shells"
    );
    let mut reversed_cavity = parts[1].clone();
    reversed_cavity.set_orientation(Orientation::Reversed);
    let mut carrier = outer.empty_copied().unwrap();
    BRepBuilder::default()
        .add(&mut carrier, &reversed_cavity)
        .unwrap();
    let result = PerformCommon(&parts[0], &carrier);
    ValidateResult(&result, -1.0, -1.0, true);
}
