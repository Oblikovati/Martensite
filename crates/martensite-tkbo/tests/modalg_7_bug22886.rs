//! Native replay of the OCC22886 Boolean common regression.
#![allow(non_snake_case)]

use martensite_tkbo::brep_algo_api::BRepAlgoAPICommon;
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools::{self, BRepToolsShapeSet},
    topods::Shape,
    toptools::ShapeSet,
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp,
};

fn restored_shape(bytes: &[u8]) -> Shape {
    let header_end = bytes
        .iter()
        .position(u8::is_ascii_whitespace)
        .expect("fixture: expected BREP header");
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(bytes[header_end..].to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .expect("fixture: expected readable BREP");
    shape
}

fn assert_valid(source: &str, shape: &Shape) {
    let valid = BRepCheckAnalyzer::new(shape, true, false, false)
        .expect("checkshape analyzer")
        .is_valid()
        .expect("checkshape result");
    assert!(valid, "{source}: checkshape result");
}

/// Counts `checknbshapes -t` entries through the source shape-set traversal.
fn topology_count(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut entries = BRepToolsShapeSet::default();
    entries.add(shape).expect("topology entry set");
    (1..=entries.nb_shapes())
        .filter(|index| {
            entries
                .shape(*index)
                .expect("topology entry")
                .shape_type()
                .expect("topology type")
                == kind
        })
        .count()
}

fn draw_mass(actual: f64) -> f64 {
    if actual == 0.0 {
        return actual;
    }
    let scale = martensite_cmath::powi(
        10_f64,
        5 - martensite_cmath::log10(actual.abs()).floor() as i32,
    );
    (actual * scale).round() / scale
}

fn assert_draw_mass(actual: f64, expected: f64, label: &str) {
    let actual = draw_mass(actual);
    assert!(
        (actual - expected).abs() / expected <= 1.0e-7,
        "{label}: {actual}; expected {expected}"
    );
}

#[test]
fn Modalg7__Bug22886() {
    let source = "tests/bugs/modalg_7/bug22886";
    let first = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/bug22886_solid1.brep"
    ));
    let second = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/bug22886_solid2.brep"
    ));
    let result = BRepAlgoAPICommon::from_shapes(&first, &second)
        .unwrap_or_else(|failure| panic!("{source}: bcommon failed: {failure:?}"));
    assert!(result.is_done(), "{source}: bcommon did not complete");
    let result = result.shape();
    assert_valid(source, result);

    let mut properties = GProps::default();
    BRepGProp::surface_properties(result, &mut properties, false, false)
        .expect("source checkprops -s");
    assert_draw_mass(properties.mass(), 1.40191e9, "{source}: surface area");
    BRepGProp::volume_properties(result, &mut properties, false, false, false)
        .expect("source checkprops -v");
    assert_draw_mass(properties.mass(), 3.59571e12, "{source}: volume");

    for (kind, expected) in [
        (ShapeEnum::Vertex, 78),
        (ShapeEnum::Edge, 135),
        (ShapeEnum::Wire, 56),
        (ShapeEnum::Face, 56),
        (ShapeEnum::Shell, 1),
        (ShapeEnum::Solid, 1),
    ] {
        assert_eq!(topology_count(result, kind), expected, "{source}: {kind:?}");
    }
}
