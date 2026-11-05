//! Native
#![allow(non_snake_case)]

use martensite_tkbo::brep_algo_api::{BRepAlgoAPICommon, BRepAlgoAPICut};
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Shape},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp,
};

const SOURCE: &str = "tests/bugs/modalg_4/bug693_1";

fn box_at(origin: Point3, dx: f64, dy: f64, dz: f64) -> Shape {
    BRepPrimAPIMakeBox::at_point(origin, dx, dy, dz)
        .expect("source box")
        .shape()
        .expect("source box shape")
        .clone()
}

fn compound(parts: &[Shape]) -> Shape {
    let builder = BRepBuilder::default();
    let mut result = Compound::default();
    builder.make_compound(&mut result);
    for part in parts {
        builder.add(&mut result, part).expect("source compound");
    }
    result.into_shape()
}

fn common(first: &Shape, second: &Shape) -> Shape {
    let operation = BRepAlgoAPICommon::from_shapes(first, second)
        .unwrap_or_else(|failure| panic!("{SOURCE}: bopcommon failed: {failure:?}"));
    assert!(operation.is_done(), "{SOURCE}: bopcommon incomplete");
    operation.shape().clone()
}

fn cut(first: &Shape, second: &Shape) -> Shape {
    let operation = BRepAlgoAPICut::from_shapes(first, second)
        .unwrap_or_else(|failure| panic!("{SOURCE}: bopcut failed: {failure:?}"));
    assert!(operation.is_done(), "{SOURCE}: bopcut incomplete");
    operation.shape().clone()
}

fn assert_valid(label: &str, shape: &Shape) {
    let analyzer = BRepCheckAnalyzer::new(shape, true, false, false)
        .unwrap_or_else(|failure| panic!("{SOURCE}: checkshape {label}: {failure:?}"));
    assert!(
        analyzer.is_valid().expect("source checkshape"),
        "{SOURCE}: checkshape -top {label}"
    );
}

fn assert_area(shape: &Shape) {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(shape, &mut properties, false, false)
        .expect("source checkprops result -s 33196");
    assert!(
        (properties.mass() - 33_196.).abs() / 33_196. <= 0.01,
        "{SOURCE}: source checkprops result -s 33196; got {}",
        properties.mass()
    );
}

fn assert_nonempty_topology(shape: &Shape) {
    use martensite_tkbrep::{topexp, toptools::IndexedMapOfShape};
    for kind in [ShapeEnum::Solid, ShapeEnum::Face, ShapeEnum::Edge] {
        let mut indexed = IndexedMapOfShape::default();
        topexp::map_shapes_of_type(shape, kind, &mut indexed).expect("source shape traversal");
        assert!(
            indexed.extent() > 0,
            "{SOURCE}: expected {kind:?} in result"
        );
    }
}

#[test]
fn Modalg4__Bug693_1() {
    let horizontal = box_at(Point3::new(234., 52., 0.), 41., 98., 40.);
    let vertical = box_at(Point3::new(241., 136., 20.), 31., 31., 15.);
    let third = box_at(Point3::new(215., 120., 20.), 78., 15., 15.);
    let first = compound(&[horizontal]);
    let second = compound(&[vertical]);
    let first_common = common(&first, &second);
    let first_cut = cut(&first, &second);
    let second_cut = cut(&second, &first);
    assert_valid("int1", &first_common);
    assert_valid("cut1", &first_cut);
    assert_valid("cut2", &second_cut);
    let combined = compound(&[first_common, first_cut, second_cut]);
    assert_valid("comp3", &combined);
    let second_common = common(&combined, &third);
    let third_cut = cut(&combined, &third);
    let fourth_cut = cut(&third, &combined);
    assert_valid("int2", &second_common);
    assert_valid("cut3", &third_cut);
    assert_valid("cut4", &fourth_cut);
    let result = compound(&[second_common, third_cut, fourth_cut]);
    assert_valid("result", &result);
    assert_area(&result);
    assert_nonempty_topology(&result);
}
