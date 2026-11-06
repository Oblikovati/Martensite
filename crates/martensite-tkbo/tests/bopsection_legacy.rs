//! Native Rust translations of complete-input `bopsection` regressions.
#![allow(non_snake_case)]
use martensite_tkbo::bop_algo::{BOPAlgoPaveFiller, BOPAlgoSection};
use martensite_tkbrep::{brep::BRepBuilder, brep_tools, topods::Shape};
use martensite_tkernel::{message::ProgressRange, ncollection::List, storage::IoStream};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Point3, Vector3};
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeCylinder, BRepPrimAPIMakePrism};
use martensite_tktopalgo::{
    brep_builder_api::{
        BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape,
    },
    brep_gprop::BRepGProp,
};
#[path = "bopsection_exploded_shell_case.rs"]
mod exploded_shell_case;
#[path = "bopsection_pro15203_cases.rs"]
mod pro15203_cases;
#[path = "bopsection_simple_fixture_cases.rs"]
mod simple_fixture_cases;
pub(super) fn restored_shape(bytes: &[u8]) -> Shape {
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
pub(super) fn section_case(source: &str, first: &[u8], second: &[u8], expected: f64) {
    let first = restored_shape(first);
    let second = restored_shape(second);
    let arguments: List<Shape> = [first, second].into_iter().collect();
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(&arguments);
    filler.options_mut().set_run_parallel(false);
    filler.perform();
    assert!(
        !filler.options().has_errors(),
        "{source}: bop preparation failed"
    );
    let mut section = BOPAlgoSection::new();
    section.set_arguments(&arguments);
    section.options_mut().set_run_parallel(false);
    section.perform_with_filler(&mut filler);
    assert!(
        !section.options().has_errors(),
        "{source}: bopsection failed"
    );
    let mut properties = GProps::default();
    BRepGProp::linear_properties(section.shape(), &mut properties, false, false)
        .unwrap_or_else(|failure| panic!("{source}: length failed: {failure:?}"));
    assert!(
        (properties.mass() - expected).abs() / expected <= 0.01,
        "{source}: length {}; expected {expected}",
        properties.mass()
    );
}
macro_rules! case {
    ($name:ident,$source:literal,$first:expr,$second:expr,$length:expr $(,)?) => {
        #[test]
        fn $name() {
            section_case($source, $first, $second, $length);
        }
    };
}
#[test]
fn BOPSection__A1() {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    for point in [
        Point3::new(0.0, -1.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 1.0, 2.0),
        Point3::new(0.0, -1.0, 2.0),
    ] {
        polygon.add_point(point).unwrap();
    }
    polygon.close().unwrap();
    let face = BRepBuilderAPIMakeFace::from_wire(&polygon.wire().unwrap(), true)
        .unwrap()
        .face()
        .unwrap();
    let prism =
        BRepPrimAPIMakePrism::new(&face.into_shape(), Vector3::new(1.0, 0.0, 0.0), false, true)
            .unwrap()
            .shape()
            .unwrap()
            .clone();
    let cylinder = BRepPrimAPIMakeCylinder::new(1.0, 4.0)
        .shape()
        .unwrap()
        .clone();
    let mut section = BOPAlgoSection::new();
    section.set_arguments(&[prism, cylinder].into_iter().collect());
    section.options_mut().set_run_parallel(false);
    section.perform();
    assert!(
        !section.options().has_errors(),
        "tests/boolean/bopsection/A1: section failed"
    );
    let mut properties = GProps::default();
    BRepGProp::linear_properties(section.shape(), &mut properties, false, false).unwrap();
    let length = properties.mass();
    assert!(
        (length - 14.2832).abs() / 14.2832 <= 0.01,
        "length {length}"
    );
}
case!(
    BOPSection__A4,
    "tests/boolean/bopsection/A4",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_pro13075a.rle"
    ),
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_pro13075b.rle"
    ),
    99.3262
);
case!(
    BOPSection__A7,
    "tests/boolean/bopsection/A7",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts20578a.rle"
    ),
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts20578b.rle"
    ),
    728.763
);
case!(
    BOPSection__A8,
    "tests/boolean/bopsection/A8",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_buc60093a.rle"
    ),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/buc60093b.rle"),
    151.416
);
case!(
    BOPSection__A9,
    "tests/boolean/bopsection/A9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/buc60172a.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/buc60172b.rle"),
    878.56
);
case!(
    BOPSection__B1,
    "tests/boolean/bopsection/B1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro11615a.rle"),
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_pro11615b.rle"
    ),
    11.6
);
case!(
    BOPSection__B2,
    "tests/boolean/bopsection/B2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/cts17786a.rle"),
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts17786b.rle"
    ),
    0.0976741
);
case!(
    BOPSection__B3,
    "tests/boolean/bopsection/B3",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_fra50047a.rle"
    ),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/fra50047b.rle"),
    628.319
);
case!(
    BOPSection__B4,
    "tests/boolean/bopsection/B4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro9481a.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro9481b.rle"),
    900.779
);

case!(
    BOPSection__B7,
    "tests/boolean/bopsection/B7",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts21143a.rle"
    ),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/cts21143b.rle"),
    301.584
);
case!(
    BOPSection__C2,
    "tests/boolean/bopsection/C2",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO909_fus-object1.brep"
    ),
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO909_fus-object2.brep"
    ),
    120.859
);
case!(
    BOPSection__C5,
    "tests/boolean/bopsection/C5",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts20769a.rle"
    ),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/cts20769a.brep"),
    312.73
);
case!(
    BOPSection__C8,
    "tests/boolean/bopsection/C8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CTO909_outil.brep"),
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO902_cts20528-part.rle"
    ),
    382.621
);

/// Replays B6 after its source p-curve and tolerance preparation commands.
#[test]
fn BOPSection__B6() {
    let source = "tests/boolean/bopsection/B6";
    let first = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/cts20975a.rle"
    ));
    let second = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts20975b.rle"
    ));
    martensite_tkbo::bop_tools::prepare_boolean_operand(&first)
        .unwrap_or_else(|failure| panic!("{source}: first preparation failed: {failure:?}"));
    martensite_tkbo::bop_tools::prepare_boolean_operand(&second)
        .unwrap_or_else(|failure| panic!("{source}: second preparation failed: {failure:?}"));
    let arguments: List<Shape> = [first, second].into_iter().collect();
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(&arguments);
    filler.options_mut().set_run_parallel(false);
    filler.perform();
    assert!(
        !filler.options().has_errors(),
        "{source}: bop preparation failed"
    );
    let mut section = BOPAlgoSection::new();
    section.set_arguments(&arguments);
    section.options_mut().set_run_parallel(false);
    section.perform_with_filler(&mut filler);
    assert!(
        !section.options().has_errors(),
        "{source}: bopsection failed"
    );
    let mut properties = GProps::default();
    BRepGProp::linear_properties(section.shape(), &mut properties, false, false).unwrap();
    let length = properties.mass();
    assert!(
        (length - 128.364).abs() / 128.364 <= 0.01,
        "{source}: length {length}; expected 128.364"
    );
}
