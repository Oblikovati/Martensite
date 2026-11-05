//! Native replay of OCC2784's face Boolean regression.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{CreateBox, PerformCommon, PerformCut, Translate};
use martensite_tkbrep::{topexp, topods::Shape, toptools::IndexedMapOfShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::{brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp};

fn source_face(source: &str, shape: &Shape, one_based_index: usize) -> Shape {
    let mut faces = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Face, &mut faces).expect("source explode face");
    faces
        .find_key(one_based_index)
        .unwrap_or_else(|_| panic!("{source}: source explode face index {one_based_index}"))
        .clone()
}

fn source_result(source: &str) -> Shape {
    let first_box = CreateBox(Point3::default(), 10., 20., 10.);
    let second_box = Translate(&CreateBox(Point3::default(), 1., 1., 1.), 5., 5., 0.);
    let cut = PerformCut(
        &source_face(source, &first_box, 5),
        &source_face(source, &second_box, 5),
    );
    let check =
        BRepCheckAnalyzer::new(&cut, true, false, false).expect("source checkshape rcut1 analyzer");
    assert!(
        check.is_valid().expect("source checkshape rcut1"),
        "{source}: checkshape rcut1"
    );
    PerformCommon(&cut, &source_face(source, &second_box, 5))
}

#[test]
fn Moddata2__Bug2784_1() {
    let result = source_result("tests/bugs/moddata_2/bug2784_1");
    let mut properties = GProps::default();
    BRepGProp::surface_properties(&result, &mut properties, false, false)
        .expect("source sprops result");
    assert!(
        properties.mass().abs() <= 1.0e-12,
        "tests/bugs/moddata_2/bug2784_1: source sprops result Mass 0; got {}",
        properties.mass()
    );
}

#[test]
fn Moddata2__Bug2784_2() {
    // The old-topology script has the same commands and assertion as Bug2784_1.
    let result = source_result("tests/bugs/moddata_2/bug2784_2");
    let mut properties = GProps::default();
    BRepGProp::surface_properties(&result, &mut properties, false, false)
        .expect("source sprops result");
    assert!(
        properties.mass().abs() <= 1.0e-12,
        "tests/bugs/moddata_2/bug2784_2: expected empty common"
    );
}
