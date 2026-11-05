//! Native Rust translations of the box, prism, and BOP fuse regressions.
#![allow(non_snake_case)]

use martensite_tkbo::brep_algo_api::BRepAlgoAPIFuse;
use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Point3, Vector3};
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeBox, BRepPrimAPIMakePrism};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

use super::{assert_surface_area, bop_fuse, restored_shape};

fn box_shape(source: &str, origin: Point3) -> Shape {
    BRepPrimAPIMakeBox::at_point(origin, 4.0, 4.0, 4.0)
        .unwrap_or_else(|failure| panic!("{source}: small box failed: {failure:?}"))
        .shape()
        .unwrap_or_else(|failure| panic!("{source}: small box result failed: {failure:?}"))
        .clone()
}

fn base_box(source: &str) -> Shape {
    BRepPrimAPIMakeBox::from_sizes(10.0, 10.0, 10.0)
        .unwrap_or_else(|failure| panic!("{source}: base box failed: {failure:?}"))
        .shape()
        .unwrap_or_else(|failure| panic!("{source}: base box result failed: {failure:?}"))
        .clone()
}

fn first_solid(source: &str, shape: &Shape) -> Shape {
    let mut explorer = Explorer::new(shape, ShapeEnum::Solid, ShapeEnum::Shape)
        .unwrap_or_else(|failure| panic!("{source}: solid exploration failed: {failure:?}"));
    Iterator::next(&mut explorer).unwrap_or_else(|| panic!("{source}: expected first solid"))
}

fn fused_boxes(source: &str, translation: Vector3, legacy_bfuse: bool) -> Shape {
    let base = base_box(source);
    let small = box_shape(
        source,
        Point3::new(translation.x(), translation.y(), translation.z()),
    );
    if !legacy_bfuse {
        return bop_fuse(source, &base, &small);
    }
    let fused = BRepAlgoAPIFuse::from_shapes(&base, &small)
        .unwrap_or_else(|failure| panic!("{source}: bfuse failed: {failure:?}"));
    assert!(fused.is_done(), "{source}: bfuse incomplete");
    first_solid(source, fused.shape())
}

fn fixture_prism(source: &str, bytes: &[u8], direction: Vector3) -> Shape {
    let fixture = restored_shape(bytes);
    let mut prism = BRepPrimAPIMakePrism::new(&fixture, direction, false, true)
        .unwrap_or_else(|failure| panic!("{source}: prism failed: {failure:?}"));
    prism
        .shape()
        .unwrap_or_else(|failure| panic!("{source}: prism result failed: {failure:?}"))
        .clone()
}

fn prism_box_case(
    source: &str,
    translation: Vector3,
    fixture: &[u8],
    direction: Vector3,
    legacy_bfuse: bool,
    expected_area: f64,
) {
    let boxes = fused_boxes(source, translation, legacy_bfuse);
    let prism = fixture_prism(source, fixture, direction);
    let result = bop_fuse(source, &boxes, &prism);
    assert_surface_area(source, &result, Some(expected_area));
}

macro_rules! prism_box_case {
    ($name:ident, $source:literal, $translation:expr, $fixture:expr, $direction:expr, $bfuse:expr, $area:expr) => {
        #[test]
        fn $name() {
            prism_box_case($source, $translation, $fixture, $direction, $bfuse, $area);
        }
    };
}

prism_box_case!(
    BOPFuseComplex__M5,
    "tests/boolean/bopfuse_complex/M5",
    Vector3::new(3., 3., 6.),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/A1_rr_7.brep"),
    Vector3::new(0., 0., 4.),
    false,
    664.
);
prism_box_case!(
    BOPFuseComplex__M6,
    "tests/boolean/bopfuse_complex/M6",
    Vector3::new(3., 3., 6.),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/A1_rr_7.brep"),
    Vector3::new(0., 0., -4.),
    true,
    600.
);
prism_box_case!(
    BOPFuseComplex__M7,
    "tests/boolean/bopfuse_complex/M7",
    Vector3::new(0., 0., 6.),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/C1_rr_8.brep"),
    Vector3::new(0., 0., -4.),
    true,
    600.
);
prism_box_case!(
    BOPFuseComplex__M8,
    "tests/boolean/bopfuse_complex/M8",
    Vector3::new(0., 0., 6.),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/C1_rr_8.brep"),
    Vector3::new(0., 0., 4.),
    true,
    664.
);
prism_box_case!(
    BOPFuseComplex__M9,
    "tests/boolean/bopfuse_complex/M9",
    Vector3::new(0., 0., 2.),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/E1_rr_7.brep"),
    Vector3::new(0., -4., 0.),
    true,
    664.
);
prism_box_case!(
    BOPFuseComplex__N1,
    "tests/boolean/bopfuse_complex/N1",
    Vector3::new(0., 0., 2.),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/E1_rr_7.brep"),
    Vector3::new(0., 4., 0.),
    true,
    600.
);
