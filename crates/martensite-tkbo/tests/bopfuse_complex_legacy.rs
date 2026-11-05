//! Native Rust translations of simple complete-input `bopfuse_complex` regressions.
#![allow(non_snake_case)]

use martensite_tkbo::bop_algo::{BOPAlgoBOP, BOPAlgoPaveFiller, Operation};
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools,
    topods::{self, Shape},
};
use martensite_tkernel::{message::ProgressRange, ncollection::List, storage::IoStream};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::Vector3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakePrism;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape};
use martensite_tktopalgo::brep_gprop::BRepGProp;

#[path = "bopfuse_chained_fixture_case.rs"]
mod chained_fixture_case;
#[path = "bopfuse_exploded_shell_case.rs"]
mod exploded_shell_case;
#[path = "bopfuse_prism_box_cases.rs"]
mod prism_box_cases;
#[path = "bopfuse_pro15203_cases.rs"]
mod pro15203_cases;
#[path = "bopfuse_simple_fixture_cases.rs"]
mod simple_fixture_cases;

pub(super) fn restored_shape(bytes: &[u8]) -> Shape {
    let end = bytes
        .iter()
        .position(u8::is_ascii_whitespace)
        .expect("BRep fixture: expected drawable header");
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(bytes[end..].to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .expect("BRep fixture: expected readable shape");
    shape
}

pub(super) fn bop_fuse(source: &str, first: &Shape, second: &Shape) -> Shape {
    let arguments: List<Shape> = [first.clone(), second.clone()].into_iter().collect();
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(&arguments);
    filler.options_mut().set_run_parallel(false);
    filler.perform();
    assert!(
        !filler.options().has_errors(),
        "{source}: bop preparation failed"
    );
    let mut fuse = BOPAlgoBOP::new();
    fuse.add_argument(first);
    fuse.add_tool(second);
    fuse.set_operation(Operation::Fuse);
    fuse.options_mut().set_run_parallel(false);
    fuse.perform_with_filler(&mut filler);
    assert!(!fuse.options().has_errors(), "{source}: bopfuse failed");
    fuse.shape().clone()
}

pub(super) fn assert_surface_area(source: &str, shape: &Shape, expected: Option<f64>) {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(shape, &mut properties, false, false)
        .unwrap_or_else(|failure| panic!("{source}: area failed: {failure:?}"));
    match expected {
        Some(expected) => assert!(
            (properties.mass() - expected).abs() / expected <= 0.01,
            "{source}: area {}; expected {expected} within 1%",
            properties.mass()
        ),
        None => assert!(
            properties.mass().abs() <= 1e-6,
            "{source}: expected empty area, got {}",
            properties.mass()
        ),
    }
}

pub(super) fn fuse_case(source: &str, first: &[u8], second: &[u8], expected: Option<f64>) {
    let first = restored_shape(first);
    let second = restored_shape(second);
    let result = bop_fuse(source, &first, &second);
    assert_surface_area(source, &result, expected);
}

macro_rules! case {
    ($name:ident, $source:literal, $first:expr, $second:expr, $area:expr $(,)?) => {
        #[test]
        fn $name() {
            fuse_case($source, $first, $second, $area);
        }
    };
}

case!(
    BOPFuseComplex__E8,
    "tests/boolean/bopfuse_complex/E8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a57"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    Some(22200.0),
);
case!(
    BOPFuseComplex__E9,
    "tests/boolean/bopfuse_complex/E9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b58"),
    Some(19521.9),
);
case!(
    BOPFuseComplex__F2,
    "tests/boolean/bopfuse_complex/F2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b62"),
    Some(20389.2),
);
case!(
    BOPFuseComplex__F3,
    "tests/boolean/bopfuse_complex/F3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b64"),
    Some(30735.6),
);
case!(
    BOPFuseComplex__F4,
    "tests/boolean/bopfuse_complex/F4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b66"),
    Some(19659.3),
);
case!(
    BOPFuseComplex__F5,
    "tests/boolean/bopfuse_complex/F5",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a90"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b90"),
    Some(24142.1),
);
case!(
    BOPFuseComplex__F6,
    "tests/boolean/bopfuse_complex/F6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b102"),
    Some(18961.2),
);
case!(
    BOPFuseComplex__F7,
    "tests/boolean/bopfuse_complex/F7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b104"),
    Some(22567.7),
);
case!(
    BOPFuseComplex__F8,
    "tests/boolean/bopfuse_complex/F8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a106"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a157"),
    Some(10933.0),
);
case!(
    BOPFuseComplex__F9,
    "tests/boolean/bopfuse_complex/F9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b110"),
    Some(16240.9),
);
case!(
    BOPFuseComplex__G1,
    "tests/boolean/bopfuse_complex/G1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b112"),
    Some(32686.4),
);
case!(
    BOPFuseComplex__G2,
    "tests/boolean/bopfuse_complex/G2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b114"),
    Some(21046.7),
);
case!(
    BOPFuseComplex__G3,
    "tests/boolean/bopfuse_complex/G3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a116"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b116"),
    Some(12566.4),
);
case!(
    BOPFuseComplex__G4,
    "tests/boolean/bopfuse_complex/G4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a116"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b118"),
    Some(12566.4),
);
case!(
    BOPFuseComplex__G5,
    "tests/boolean/bopfuse_complex/G5",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a116"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b124"),
    Some(17907.1),
);
case!(
    BOPFuseComplex__G6,
    "tests/boolean/bopfuse_complex/G6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a116"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b126"),
    Some(13561.9),
);
case!(
    BOPFuseComplex__G7,
    "tests/boolean/bopfuse_complex/G7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a138"),
    Some(15000.0),
);
case!(
    BOPFuseComplex__G8,
    "tests/boolean/bopfuse_complex/G8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b130"),
    Some(17420.9),
);
case!(
    BOPFuseComplex__G9,
    "tests/boolean/bopfuse_complex/G9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b132"),
    Some(24401.3),
);
case!(
    BOPFuseComplex__H2,
    "tests/boolean/bopfuse_complex/H2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a138"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b130"),
    Some(14201.9),
);
case!(
    BOPFuseComplex__H3,
    "tests/boolean/bopfuse_complex/H3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a138"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b140"),
    Some(19534.6),
);
case!(
    BOPFuseComplex__H4,
    "tests/boolean/bopfuse_complex/H4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b142"),
    Some(17021.3),
);
case!(
    BOPFuseComplex__H5,
    "tests/boolean/bopfuse_complex/H5",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a152"),
    Some(15000.0),
);
case!(
    BOPFuseComplex__H6,
    "tests/boolean/bopfuse_complex/H6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a158"),
    Some(31198.7),
);
case!(
    BOPFuseComplex__H7,
    "tests/boolean/bopfuse_complex/H7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a148"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b148"),
    Some(35342.8),
);
case!(
    BOPFuseComplex__H8,
    "tests/boolean/bopfuse_complex/H8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a160"),
    Some(36598.4),
);
case!(
    BOPFuseComplex__H9,
    "tests/boolean/bopfuse_complex/H9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a152"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b118"),
    Some(10603.0),
);
case!(
    BOPFuseComplex__I1,
    "tests/boolean/bopfuse_complex/I1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a152"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a157"),
    Some(9581.86),
);
case!(
    BOPFuseComplex__I2,
    "tests/boolean/bopfuse_complex/I2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a157"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a152"),
    Some(9581.86),
);
case!(
    BOPFuseComplex__I3,
    "tests/boolean/bopfuse_complex/I3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a158"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b148"),
    Some(47123.5),
);
case!(
    BOPFuseComplex__I4,
    "tests/boolean/bopfuse_complex/I4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a160"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b160"),
    Some(58622.4),
);
case!(
    BOPFuseComplex__J2,
    "tests/boolean/bopfuse_complex/J2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a206"),
    Some(15000.0),
);
case!(
    BOPFuseComplex__J3,
    "tests/boolean/bopfuse_complex/J3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b199"),
    Some(33784.8),
);
case!(
    BOPFuseComplex__J4,
    "tests/boolean/bopfuse_complex/J4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a206"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b199"),
    Some(35044.1),
);
case!(
    BOPFuseComplex__J6,
    "tests/boolean/bopfuse_complex/J6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a350"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b350"),
    Some(120.576),
);
case!(
    BOPFuseComplex__J9,
    "tests/boolean/bopfuse_complex/J9",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts20578a.rle"
    ),
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts20578b.rle"
    ),
    Some(759758.0),
);
case!(
    BOPFuseComplex__K1,
    "tests/boolean/bopfuse_complex/K1",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_buc60093a.rle"
    ),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/buc60093b.rle"),
    Some(4356.19),
);
case!(
    BOPFuseComplex__K2,
    "tests/boolean/bopfuse_complex/K2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro11615a.rle"),
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_pro11615b.rle"
    ),
    Some(418.68),
);
case!(
    BOPFuseComplex__K3,
    "tests/boolean/bopfuse_complex/K3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/cts17786a.rle"),
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts17786b.rle"
    ),
    Some(0.00372003),
);
case!(
    BOPFuseComplex__K4,
    "tests/boolean/bopfuse_complex/K4",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_fra50047a.rle"
    ),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/fra50047b.rle"),
    Some(508496.0),
);
case!(
    BOPFuseComplex__K5,
    "tests/boolean/bopfuse_complex/K5",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro9481a.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro9481b.rle"),
    Some(448769.0),
);
case!(
    BOPFuseComplex__K7,
    "tests/boolean/bopfuse_complex/K7",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts21143a.rle"
    ),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/cts21143b.rle"),
    Some(37364.9),
);
case!(
    BOPFuseComplex__K9,
    "tests/boolean/bopfuse_complex/K9",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO909_fus-object1.brep"
    ),
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO909_fus-object2.brep"
    ),
    Some(67333.1),
);
case!(
    BOPFuseComplex__L9,
    "tests/boolean/bopfuse_complex/L9",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO909_fus-object1.brep"
    ),
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO909_fus-object2.brep"
    ),
    Some(67333.1),
);
case!(
    BOPFuseComplex__N2,
    "tests/boolean/bopfuse_complex/N2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh2"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh2a"),
    Some(1200.0),
);
case!(
    BOPFuseComplex__N3,
    "tests/boolean/bopfuse_complex/N3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh2"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh3"),
    Some(1000.0),
);
case!(
    BOPFuseComplex__N4,
    "tests/boolean/bopfuse_complex/N4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh2"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh4"),
    Some(900.0),
);

case!(
    BOPFuseComplex__L2,
    "tests/boolean/bopfuse_complex/L2",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts20769a.rle"
    ),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/cts20769a.brep"),
    Some(92385.1),
);
case!(
    BOPFuseComplex__L7,
    "tests/boolean/bopfuse_complex/L7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CTO909_outil.brep"),
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO902_cts20528-part.rle"
    ),
    Some(98808.8),
);

#[test]
fn BOPFuseComplex__J8() {
    let source = "tests/boolean/bopfuse_complex/J8";
    let solid = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_fra60275a.rle"
    ));
    let contour = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/fra60275b.rle"
    ));
    let wire = topods::wire(&contour)
        .unwrap_or_else(|failure| panic!("{source}: contour wire failed: {failure:?}"));
    let mut face = BRepBuilderAPIMakeFace::from_wire(&wire, false)
        .unwrap_or_else(|failure| panic!("{source}: mkplane failed: {failure:?}"));
    let face = face
        .shape()
        .unwrap_or_else(|failure| panic!("{source}: mkplane result failed: {failure:?}"));
    let mut prism = BRepPrimAPIMakePrism::new(face, Vector3::new(0.0, 0.0, 100.0), false, true)
        .unwrap_or_else(|failure| panic!("{source}: prism failed: {failure:?}"));
    let result = bop_fuse(source, &solid, prism.shape().unwrap());
    assert_surface_area(source, &result, Some(358551.0));
}
