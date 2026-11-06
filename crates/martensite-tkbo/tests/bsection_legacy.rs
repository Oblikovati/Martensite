//! Native Rust translations of complete-input `bsection` regressions.
#![allow(non_snake_case)]
use martensite_tkbo::brep_algo_api::BRepAlgoAPISection;
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools,
    topods::{Shape, ShapeIterator},
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{self, Axis2, Circle3, Direction3, Plane3, Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::{
    brep_builder_api::{
        BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape,
        BRepBuilderAPIMakeWire,
    },
    brep_check::BRepCheckAnalyzer,
    brep_gprop::BRepGProp,
};
fn restored_shape(bytes: &[u8]) -> Shape {
    let end = bytes
        .iter()
        .position(u8::is_ascii_whitespace)
        .expect("BRep fixture header");
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(bytes[end..].to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .expect("BRep fixture read");
    shape
}
fn section_case(source: &str, first: &[u8], second: &[u8], expected: Option<f64>) {
    let first = restored_shape(first);
    let second = restored_shape(second);
    let section = BRepAlgoAPISection::from_shapes(&first, &second, None, true)
        .unwrap_or_else(|failure| panic!("{source}: bsection failed: {failure:?}"));
    assert!(section.is_done(), "{source}: bsection incomplete");
    let mut properties = GProps::default();
    BRepGProp::linear_properties(section.shape(), &mut properties, false, false)
        .unwrap_or_else(|failure| panic!("{source}: length failed: {failure:?}"));
    if let Some(expected) = expected {
        assert!(
            (properties.mass() - expected).abs() / expected <= 0.01,
            "{source}: length {}; expected {expected}",
            properties.mass()
        );
    } else {
        assert!(
            properties.mass().abs() <= 1e-6,
            "{source}: expected empty length {}",
            properties.mass()
        );
    }
}
macro_rules! case {
    ($name:ident,$source:literal,$first:expr,$second:expr,$length:expr $(,)?) => {
        #[test]
        fn $name() {
            section_case($source, $first, $second, $length);
        }
    };
}

#[path = "bsection_compound_cases.rs"]
mod compound_cases;
#[path = "bsection_m3.rs"]
mod m3;

fn compound_section_case(source: &str, bytes: &[u8], expected: f64) {
    let compound = restored_shape(bytes);
    let members: Vec<_> = ShapeIterator::new(&compound, false, true).collect();
    assert_eq!(members.len(), 2, "{source}: expected two operands");
    let section = BRepAlgoAPISection::from_shapes(&members[0], &members[1], None, true)
        .unwrap_or_else(|failure| panic!("{source}: bsection failed: {failure:?}"));
    assert!(section.is_done(), "{source}: bsection incomplete");
    let mut properties = GProps::default();
    BRepGProp::linear_properties(section.shape(), &mut properties, false, false)
        .unwrap_or_else(|failure| panic!("{source}: length failed: {failure:?}"));
    let length = properties.mass();
    assert!(
        (length - expected).abs() / expected <= 0.01,
        "{source}: length {length}; expected {expected} within 1%"
    );
}

#[test]
fn BSection__H3() {
    compound_section_case(
        "tests/boolean/bsection/H3",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/BUGDATA_2293.rle"
        ),
        285.489,
    );
}

#[test]
fn BSection__H4() {
    compound_section_case(
        "tests/boolean/bsection/H4",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/BUGDATA_2293_nofog.rle"
        ),
        285.489,
    );
}

#[test]
fn BSection__Q3() {
    let first = BRepPrimAPIMakeCylinder::new(10.0, 20.0)
        .shape()
        .unwrap()
        .clone();
    let second = BRepPrimAPIMakeCylinder::with_axes(
        Axis2::new(Point3::new(5.0, 0.0, 10.0), gp::dz()).unwrap(),
        5.0,
        20.0,
    )
    .shape()
    .unwrap()
    .clone();
    let section = BRepAlgoAPISection::from_shapes(&first, &second, None, true).unwrap();
    assert!(
        section.is_done(),
        "tests/boolean/bsection/Q3: section incomplete"
    );
    let mut properties = GProps::default();
    BRepGProp::linear_properties(section.shape(), &mut properties, false, false).unwrap();
    let length = properties.mass();
    assert!(
        (length - 41.4159).abs() / 41.4159 <= 0.01,
        "length {length}"
    );
}

#[test]
fn BSection__S1() {
    let first = BRepPrimAPIMakeCylinder::new(20.0, 100.0)
        .shape()
        .unwrap()
        .clone();
    let second = BRepPrimAPIMakeCylinder::with_axes(
        Axis2::new(Point3::new(0.0, 0.0, 50.0), gp::dz()).unwrap(),
        20.0,
        100.0,
    )
    .shape()
    .unwrap()
    .clone();
    let section = BRepAlgoAPISection::from_shapes(&first, &second, None, true).unwrap();
    assert!(
        section.is_done(),
        "tests/boolean/bsection/S1: section incomplete"
    );
    let mut properties = GProps::default();
    BRepGProp::linear_properties(section.shape(), &mut properties, false, false).unwrap();
    let length = properties.mass();
    assert!(
        (length - 301.327).abs() / 301.327 <= 0.01,
        "length {length}"
    );
}

#[test]
fn BSection__P9() {
    let first =
        BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-1.0, 1.0, -1.0, 1.0))
            .unwrap()
            .face()
            .unwrap()
            .into_shape();
    let azimuth = 20.0_f64.to_radians();
    let dip = 80.0_f64.to_radians();
    let normal = Direction3::new(
        martensite_cmath::sin(dip) * martensite_cmath::sin(azimuth),
        martensite_cmath::sin(dip) * martensite_cmath::cos(azimuth),
        martensite_cmath::cos(dip),
    )
    .unwrap();
    let plane = Plane3::new(Point3::new(0.5, 0.5, 0.0), normal).unwrap();
    let circle =
        Circle3::new(Axis2::new(Point3::new(0.5, 0.5, 0.0), normal).unwrap(), 0.5).unwrap();
    let edge = BRepBuilderAPIMakeEdge::from_circ(circle)
        .unwrap()
        .edge()
        .unwrap();
    let wire = BRepBuilderAPIMakeWire::from_edge(&edge)
        .unwrap()
        .wire()
        .unwrap();
    let second = BRepBuilderAPIMakeFace::from_plane_wire(plane, &wire, true)
        .unwrap()
        .face()
        .unwrap()
        .into_shape();
    let section = BRepAlgoAPISection::from_shapes(&first, &second, None, true).unwrap();
    assert!(
        section.is_done(),
        "tests/boolean/bsection/P9: section incomplete"
    );
    let mut properties = GProps::default();
    BRepGProp::linear_properties(section.shape(), &mut properties, false, false).unwrap();
    assert!(
        (properties.mass() - 1.0).abs() <= 0.01,
        "length {}",
        properties.mass()
    );
}

case!(
    BSection__A2,
    "tests/boolean/bsection/A2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CIN001_a10.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CIN001_b10.rle"),
    Some(25.4961)
);
case!(
    BSection__A4,
    "tests/boolean/bsection/A4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a3"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b3"),
    Some(10.0)
);
case!(
    BSection__A5,
    "tests/boolean/bsection/A5",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a4"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b4"),
    Some(125.664)
);
case!(
    BSection__A6,
    "tests/boolean/bsection/A6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CIN001_a5.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CIN001_b5.rle"),
    Some(15.8345)
);
case!(
    BSection__A7,
    "tests/boolean/bsection/A7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a7"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b7"),
    Some(1.75)
);
case!(
    BSection__A8,
    "tests/boolean/bsection/A8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CIN001_a8.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b8"),
    Some(43.3411)
);
case!(
    BSection__B1,
    "tests/boolean/bsection/B1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a11_bsect"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b11_bsect"),
    Some(1.21315)
);
case!(
    BSection__B2,
    "tests/boolean/bsection/B2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a12"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b12"),
    Some(140.043)
);
case!(
    BSection__B3,
    "tests/boolean/bsection/B3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a13"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b13_bsect"),
    Some(7.47641)
);
case!(
    BSection__B4,
    "tests/boolean/bsection/B4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a14"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b14"),
    Some(10.6749)
);

case!(
    BSection__B7,
    "tests/boolean/bsection/B7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CIN001_a17.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b17_bsect"),
    Some(10.0)
);
case!(
    BSection__B8,
    "tests/boolean/bsection/B8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a18"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b18"),
    Some(53.6115)
);
case!(
    BSection__B9,
    "tests/boolean/bsection/B9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a19"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b19_bsect"),
    Some(0.392404)
);
case!(
    BSection__C1,
    "tests/boolean/bsection/C1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a30"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b30"),
    Some(12.5664)
);
case!(
    BSection__C2,
    "tests/boolean/bsection/C2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a23"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b23"),
    Some(51.7591)
);
case!(
    BSection__C4,
    "tests/boolean/bsection/C4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a25"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b25"),
    Some(1.51691)
);
case!(
    BSection__C5,
    "tests/boolean/bsection/C5",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a26"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CIN001_b26.rle"),
    Some(251.672)
);
case!(
    BSection__C6,
    "tests/boolean/bsection/C6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a47"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b27"),
    Some(7.94247)
);
case!(
    BSection__C7,
    "tests/boolean/bsection/C7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a28"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b28"),
    Some(176.127)
);
case!(
    BSection__C8,
    "tests/boolean/bsection/C8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a29"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b29"),
    Some(6.23599)
);
case!(
    BSection__C9,
    "tests/boolean/bsection/C9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a40"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b40"),
    Some(50.8667)
);
case!(
    BSection__D1,
    "tests/boolean/bsection/D1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a31"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b31"),
    Some(17041.4)
);
case!(
    BSection__D2,
    "tests/boolean/bsection/D2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a32"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b32"),
    Some(12.4984)
);
case!(
    BSection__D3,
    "tests/boolean/bsection/D3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a33"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b33"),
    Some(2.00019)
);
case!(
    BSection__D4,
    "tests/boolean/bsection/D4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a34"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b34"),
    Some(45.4729)
);
case!(
    BSection__D5,
    "tests/boolean/bsection/D5",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a35"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b35"),
    Some(15.0786)
);
case!(
    BSection__D6,
    "tests/boolean/bsection/D6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a37"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b37"),
    Some(6.28319)
);
case!(
    BSection__D7,
    "tests/boolean/bsection/D7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a38"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b38"),
    Some(8.24621)
);
case!(
    BSection__D8,
    "tests/boolean/bsection/D8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a49"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b50"),
    Some(254.014)
);
case!(
    BSection__D9,
    "tests/boolean/bsection/D9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a41"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b41"),
    Some(19.9279)
);
case!(
    BSection__E1,
    "tests/boolean/bsection/E1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a42"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b42"),
    Some(2.99998)
);
case!(
    BSection__E2,
    "tests/boolean/bsection/E2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a43"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b43"),
    Some(29.2833)
);
case!(
    BSection__E3,
    "tests/boolean/bsection/E3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a44"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b44"),
    Some(4.91577)
);
case!(
    BSection__E4,
    "tests/boolean/bsection/E4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CIN001_a45.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CIN001_b45.rle"),
    Some(0.374089)
);
case!(
    BSection__E5,
    "tests/boolean/bsection/E5",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a46"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b46"),
    Some(0.278859)
);
case!(
    BSection__E6,
    "tests/boolean/bsection/E6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a47"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b27"),
    Some(7.94247)
);
case!(
    BSection__E7,
    "tests/boolean/bsection/E7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a48"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b48"),
    Some(71.2757)
);
case!(
    BSection__E8,
    "tests/boolean/bsection/E8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a49"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b49"),
    Some(122.155)
);
case!(
    BSection__E9,
    "tests/boolean/bsection/E9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a60"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b60_bsect"),
    Some(1.64804)
);
case!(
    BSection__F1,
    "tests/boolean/bsection/F1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a51"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b51"),
    Some(9.94056)
);
case!(
    BSection__F2,
    "tests/boolean/bsection/F2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a52"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b52"),
    Some(164.908)
);
case!(
    BSection__F3,
    "tests/boolean/bsection/F3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a53"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a47"),
    Some(15.2808)
);
case!(
    BSection__F4,
    "tests/boolean/bsection/F4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a54"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b54"),
    Some(3872.37)
);
case!(
    BSection__F5,
    "tests/boolean/bsection/F5",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a55"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b55"),
    Some(15.7507)
);
case!(
    BSection__F6,
    "tests/boolean/bsection/F6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a57_bsect"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b57_bsect"),
    Some(117.749)
);
case!(
    BSection__F7,
    "tests/boolean/bsection/F7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a58_bsect"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b58_bsect"),
    Some(3.46962)
);
case!(
    BSection__F8,
    "tests/boolean/bsection/F8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a59"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b59"),
    Some(23.7344)
);
case!(
    BSection__F9,
    "tests/boolean/bsection/F9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CIN001_a61.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b61"),
    Some(773.793)
);
case!(
    BSection__G1,
    "tests/boolean/bsection/G1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a62"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b62_bsect"),
    Some(1.17784)
);
case!(
    BSection__G2,
    "tests/boolean/bsection/G2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a64"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b64_bsect"),
    Some(1.58211)
);
case!(
    BSection__G3,
    "tests/boolean/bsection/G3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a65"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b65"),
    Some(0.885026)
);
case!(
    BSection__G4,
    "tests/boolean/bsection/G4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a66"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b66_bsect"),
    Some(20.2081)
);
case!(
    BSection__G5,
    "tests/boolean/bsection/G5",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a67"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b67"),
    Some(5.75983)
);
case!(
    BSection__G6,
    "tests/boolean/bsection/G6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a68"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b68"),
    Some(10.448)
);
case!(
    BSection__G7,
    "tests/boolean/bsection/G7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a69"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b69"),
    Some(0.00170879)
);
case!(
    BSection__G8,
    "tests/boolean/bsection/G8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a71"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b71"),
    Some(10.8828)
);
case!(
    BSection__G9,
    "tests/boolean/bsection/G9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a72"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b72"),
    Some(0.5)
);
case!(
    BSection__H1,
    "tests/boolean/bsection/H1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a73"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b73"),
    Some(13.1087)
);
case!(
    BSection__H2,
    "tests/boolean/bsection/H2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a74"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b74"),
    Some(94.0179)
);
case!(
    BSection__N8,
    "tests/boolean/bsection/N8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CIN900_cts20hlk.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CIN900_cts20hll.rle"),
    Some(119.639)
);
case!(
    BSection__N9,
    "tests/boolean/bsection/N9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/fra61083a.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CIN900_fra61hlt.rle"),
    Some(10.0)
);
case!(
    BSection__O1,
    "tests/boolean/bsection/O1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro12053a.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CIN900_pro11hlu.rle"),
    Some(132.564)
);
case!(
    BSection__O2,
    "tests/boolean/bsection/O2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro14150a.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro14150b.rle"),
    Some(267.619)
);
case!(
    BSection__O3,
    "tests/boolean/bsection/O3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro14151a.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CFI_pro14fjq.rle"),
    Some(49.9609)
);
case!(
    BSection__O4,
    "tests/boolean/bsection/O4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CFI_pro14fjq.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro14151a.rle"),
    Some(49.9609)
);
case!(
    BSection__O5,
    "tests/boolean/bsection/O5",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro14151a.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CFI_pro14fjq.rle"),
    Some(49.9609)
);
case!(
    BSection__O6,
    "tests/boolean/bsection/O6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CFI_pro14fjq.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro14151a.rle"),
    Some(49.9609)
);
case!(
    BSection__O7,
    "tests/boolean/bsection/O7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CIN900_cts21hlo.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CIN900_cts21hlp.rle"),
    Some(751.43)
);
case!(
    BSection__O8,
    "tests/boolean/bsection/O8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CIN900_cts21hlq.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CIN900_cts21hlr.rle"),
    Some(79.1327)
);
case!(
    BSection__Q1,
    "tests/boolean/bsection/Q1",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO902_cts20339a.rle"
    ),
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO902_cts20339b.rle"
    ),
    Some(1676.89)
);
case!(
    BSection__Q2,
    "tests/boolean/bsection/Q2",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO902_cts20527-tool.rle"
    ),
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts20527b.rle"
    ),
    Some(935.487)
);
case!(
    BSection__Q4,
    "tests/boolean/bsection/Q4",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts20370-part.rle"
    ),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/cts20370-tool.rle"),
    Some(1113.89)
);
case!(
    BSection__Q5,
    "tests/boolean/bsection/Q5",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO902_cts20482a.rle"
    ),
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO902_cts20482b.rle"
    ),
    Some(408.37)
);
case!(
    BSection__Q6,
    "tests/boolean/bsection/Q6",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO902_cts20482a.rle"
    ),
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO902_cts20482c.rle"
    ),
    Some(492.637)
);
case!(
    BSection__Q7,
    "tests/boolean/bsection/Q7",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO902_cts20484a.rle"
    ),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/cts20484-tool1.rle"),
    Some(386.561)
);
case!(
    BSection__Q8,
    "tests/boolean/bsection/Q8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/cts20549a.rle"),
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts20549b.rle"
    ),
    Some(381.332)
);
case!(
    BSection__Q9,
    "tests/boolean/bsection/Q9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/spo0139a.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/spo0139b.rle"),
    Some(202.576)
);
case!(
    BSection__R3,
    "tests/boolean/bsection/R3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/heal-Face1.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/Plan.rle"),
    Some(1686.21)
);
case!(
    BSection__R4,
    "tests/boolean/bsection/R4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CTO909_objects.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CTO909_tool_4.brep"),
    Some(28.14)
);
case!(
    BSection__R7,
    "tests/boolean/bsection/R7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/buc60092a.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/buc60092b.rle"),
    Some(162.012)
);
case!(
    BSection__R9,
    "tests/boolean/bsection/R9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/buc60462a.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/buc60462b.brep"),
    Some(282.421)
);

#[test]
fn BSection__H5() {
    compound_section_case(
        "tests/boolean/bsection/H5",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/BUGDATA_2381.rle"
        ),
        691.399,
    );
}

#[test]
fn BSection__H6() {
    compound_section_case(
        "tests/boolean/bsection/H6",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/BUGDATA_2381_nofog.rle"
        ),
        691.399,
    );
}

#[test]
fn BSection__H9() {
    compound_section_case(
        "tests/boolean/bsection/H9",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/GEN465.rle"),
        49.9609,
    );
}

#[test]
fn BSection__I1() {
    compound_section_case(
        "tests/boolean/bsection/I1",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/GEN465_nofog.rle"
        ),
        49.9609,
    );
}

#[test]
fn BSection__I2() {
    compound_section_case(
        "tests/boolean/bsection/I2",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/GEN510-3454.rle"),
        8.10428,
    );
}

#[test]
fn BSection__I3() {
    compound_section_case(
        "tests/boolean/bsection/I3",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/GEN510-3454_nofog.rle"
        ),
        8.14819,
    );
}

#[test]
fn BSection__I6() {
    compound_section_case(
        "tests/boolean/bsection/I6",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/GEN510-3681.rle"),
        38.4985,
    );
}

#[test]
fn BSection__I7() {
    compound_section_case(
        "tests/boolean/bsection/I7",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/GEN510-3681_nofog.rle"
        ),
        38.4985,
    );
}

/// Native replay of `tests/bugs/modalg_1/buc60865`.
#[test]
fn Modalg1__BUC60865() {
    let source = "tests/bugs/modalg_1/buc60865";
    let shell = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/BUC60865_sh6.brep"
    ));
    assert!(
        BRepCheckAnalyzer::new(&shell, true, false, false)
            .expect("source checkshape shell")
            .is_valid()
            .expect("source checkshape shell result"),
        "{source}: source checkshape sh"
    );
    let plane = Plane3::new(
        Point3::new(1.1, 0.99999, 1.0),
        Direction3::new(0.0, 1.0, 0.0).expect("plane normal"),
    )
    .expect("source plane");
    let face = BRepBuilderAPIMakeFace::from_plane(plane)
        .expect("plane face")
        .face()
        .expect("plane")
        .into_shape();
    let section = BRepAlgoAPISection::from_shapes(&shell, &face, None, true).expect("bsection");
    assert!(section.is_done(), "{source}: bsection incomplete");
    let mut edges = martensite_tkbrep::toptools::IndexedMapOfShape::default();
    martensite_tkbrep::topexp::map_shapes_of_type(
        section.shape(),
        martensite_tkg3d::top_abs::ShapeEnum::Edge,
        &mut edges,
    )
    .expect("edge traversal");
    assert_eq!(edges.extent(), 5, "{source}: section edge count");
}
