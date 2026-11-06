//! Native replay of the OCC2986 coincident trimmed-torus Boolean regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{PerformCut, PerformFuse};
use martensite_tkbrep::topods::Shape;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Axis3, Torus3};
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeFace, brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp,
};

fn trimmed_torus_faces() -> (Shape, Shape) {
    let torus = Torus3::new(Axis3::default(), 10., 4.).unwrap();
    let full = BRepBuilderAPIMakeFace::from_torus_bounds(
        torus,
        (0., std::f64::consts::TAU, 0., std::f64::consts::TAU),
    )
    .unwrap()
    .face()
    .unwrap()
    .into_shape();
    let patch = BRepBuilderAPIMakeFace::from_torus_bounds(torus, (1., 2., 1., 2.))
        .unwrap()
        .face()
        .unwrap()
        .into_shape();
    (full, patch)
}

fn assert_valid(source: &str, shape: &Shape) {
    assert!(
        BRepCheckAnalyzer::new(shape, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap(),
        "{source}: checkshape"
    );
}

fn assert_valid_area(source: &str, shape: &Shape) {
    assert_valid(source, shape);
    let mut properties = GProps::default();
    BRepGProp::surface_properties(shape, &mut properties, false, false).unwrap();
    assert!(
        (properties.mass() - 1579.81).abs() <= 1.,
        "{source}: checkprops -s 1579.81, got {}",
        properties.mass()
    );
}

#[test]
fn Modalg2__Bug2986_1() {
    let source = "tests/bugs/modalg_2/bug2986_1";
    let (torus, patch) = trimmed_torus_faces();
    let cut = PerformCut(&torus, &patch);
    assert_valid(source, &cut);
    assert_valid_area(source, &PerformFuse(&cut, &patch));
}

#[test]
fn Modalg2__Bug2986_2() {
    let source = "tests/bugs/modalg_2/bug2986_2";
    let (torus, patch) = trimmed_torus_faces();
    let cut = PerformCut(&torus, &patch);
    assert_valid(source, &cut);
    assert_valid_area(source, &PerformFuse(&cut, &patch));
}
