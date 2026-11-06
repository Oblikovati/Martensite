//! Regressions of complete-input model-data Boolean regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::{ConvertToNurbs, CreateBox, CreateSphere, PerformCommon, PerformFuse};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::{brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp};

// Migrated from tests/bugs/moddata_2/bug408.  A converted box must remain a
// valid operand for a partial common operation.
#[test]
fn BCommon__ModDataBug_408_NurbsBoxCommonIsTopologicallyValid() {
    let first = ConvertToNurbs(&CreateBox(Point3::default(), 1., 1., 1.));
    let second = CreateBox(Point3::new(0., -0.5, 0.), 1.5, 0.5, 0.5);
    let result = PerformCommon(&first, &second);
    let checker = BRepCheckAnalyzer::new(&result, true, false, false).unwrap();
    assert!(checker.is_valid().unwrap());
}

// Migrated from tests/bugs/moddata_1/bug217.  Fusing a primitive sphere with
// a box must retain a valid result and the independently asserted area.
#[test]
fn BFuse__ModDataBug_217_SphereBoxArea() {
    let box_shape = CreateBox(Point3::new(-5., -5., -5.), 10., 10., 10.);
    let sphere = CreateSphere(Point3::default(), 6.);
    let result = PerformFuse(&sphere, &box_shape);
    assert!(
        BRepCheckAnalyzer::new(&result, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    let mut properties = GProps::default();
    BRepGProp::surface_properties(&result, &mut properties, false, false).unwrap();
    assert!((properties.mass() - 618.85).abs() / 618.85 <= 0.01);
}
