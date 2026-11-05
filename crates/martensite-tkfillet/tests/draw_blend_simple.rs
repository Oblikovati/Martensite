//! Native regression for tests/blend/simple/A1 and the shared validity check.
use martensite_tkbrep::{
    topexp,
    topods::{self, Edge, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::message::ProgressRange;
use martensite_tkfillet::{brep_fillet_api::BRepFilletAPIMakeFillet, chfi3d::FilletShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp,
};

fn fifth_unique_edge(shape: &Shape) -> Edge {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut edges).unwrap();
    topods::edge(edges.find_key(5).unwrap()).unwrap()
}

fn rounded_cube() -> Shape {
    let cube = BRepPrimAPIMakeBox::from_sizes(100., 100., 100.)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let mut fillet = BRepFilletAPIMakeFillet::new(&cube, FilletShape::Rational).unwrap();
    fillet.set_params(1e-2, 1e-4, 1e-5, 1e-4, 1e-5, 1e-3);
    fillet.set_continuity(Continuity::C1, 1e-2);
    fillet
        .add_with_radius(10., &fifth_unique_edge(&cube))
        .unwrap();
    fillet.build(&ProgressRange::default()).unwrap();
    assert!(fillet.is_done());
    fillet.shape().unwrap().clone()
}

#[test]
fn blend_simple_a1() {
    let result = rounded_cube();
    let mut properties = GProps::new();
    BRepGProp::surface_properties_eps(&result, &mut properties, 1e-4, false).unwrap();
    let area = properties.mass();
    assert_ne!(area, 0.);
    assert!(((59527.9 - area) / 59527.9).abs() <= 1e-2);
    assert!(
        BRepCheckAnalyzer::new(&result, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    let closed_form =
        60000. - 2000. + 500. * std::f64::consts::PI - 2. * (100. - 25. * std::f64::consts::PI);
    assert!((area - closed_form).abs() < 1e-7);
}
