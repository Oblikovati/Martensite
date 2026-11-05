//! Native Rust translations of simple complete-input `boptuc_complex` regressions.
#![allow(non_snake_case)]

use martensite_tkbo::bop_algo::{BOPAlgoBOP, BOPAlgoPaveFiller, Operation};
use martensite_tkbrep::{brep::BRepBuilder, brep_tools, topods::Shape};
use martensite_tkernel::{message::ProgressRange, ncollection::List, storage::IoStream};
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;

fn restored_shape(bytes: &[u8]) -> Shape {
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

fn tuc_case(source: &str, first: &[u8], second: &[u8], expected: Option<f64>) {
    let first = restored_shape(first);
    let second = restored_shape(second);
    let arguments: List<Shape> = [first.clone(), second.clone()].into_iter().collect();
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(&arguments);
    filler.options_mut().set_run_parallel(false);
    filler.perform();
    assert!(
        !filler.options().has_errors(),
        "{source}: bop preparation failed"
    );
    let mut tuc = BOPAlgoBOP::new();
    tuc.add_argument(&first);
    tuc.add_tool(&second);
    tuc.set_operation(Operation::Cut21);
    tuc.options_mut().set_run_parallel(false);
    tuc.perform_with_filler(&mut filler);
    assert!(!tuc.options().has_errors(), "{source}: boptuc failed");
    let mut properties = GProps::default();
    BRepGProp::surface_properties(tuc.shape(), &mut properties, false, false)
        .unwrap_or_else(|failure| panic!("{source}: area failed: {failure:?}"));
    if let Some(expected) = expected {
        assert!(
            (properties.mass() - expected).abs() / expected <= 0.01,
            "{source}: area {}; expected {expected} within 1%",
            properties.mass()
        );
    } else {
        assert!(
            properties.mass().abs() <= 1e-6,
            "{source}: expected empty area, got {}",
            properties.mass()
        );
    }
}

macro_rules! case {
    ($name:ident, $source:literal, $first:expr, $second:expr, $area:expr $(,)?) => {
        #[test]
        fn $name() {
            tuc_case($source, $first, $second, $area);
        }
    };
}

case!(
    BOPTucComplex__A7,
    "tests/boolean/boptuc_complex/A7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b11"),
    Some(1280.22),
);
case!(
    BOPTucComplex__A8,
    "tests/boolean/boptuc_complex/A8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b13"),
    Some(2518.43),
);
case!(
    BOPTucComplex__A9,
    "tests/boolean/boptuc_complex/A9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b15"),
    Some(4785.54),
);
case!(
    BOPTucComplex__B1,
    "tests/boolean/boptuc_complex/B1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b17"),
    Some(9241.17),
);
case!(
    BOPTucComplex__B2,
    "tests/boolean/boptuc_complex/B2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b19"),
    Some(13297.0),
);
case!(
    BOPTucComplex__B3,
    "tests/boolean/boptuc_complex/B3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a57"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    Some(15000.0),
);
case!(
    BOPTucComplex__B4,
    "tests/boolean/boptuc_complex/B4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b58"),
    Some(9140.77),
);
case!(
    BOPTucComplex__B6,
    "tests/boolean/boptuc_complex/B6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b62"),
    Some(9196.86),
);
case!(
    BOPTucComplex__B7,
    "tests/boolean/boptuc_complex/B7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b64"),
    Some(23235.6),
);
case!(
    BOPTucComplex__B8,
    "tests/boolean/boptuc_complex/B8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b66"),
    Some(14659.3),
);
case!(
    BOPTucComplex__B9,
    "tests/boolean/boptuc_complex/B9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a90"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b90"),
    Some(19142.1),
);
case!(
    BOPTucComplex__C1,
    "tests/boolean/boptuc_complex/C1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b102"),
    Some(23961.2),
);
case!(
    BOPTucComplex__C2,
    "tests/boolean/boptuc_complex/C2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b104"),
    Some(25134.0),
);
case!(
    BOPTucComplex__C3,
    "tests/boolean/boptuc_complex/C3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a106"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a157"),
    Some(6240.45),
);
case!(
    BOPTucComplex__C4,
    "tests/boolean/boptuc_complex/C4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b110"),
    Some(5994.67),
);
case!(
    BOPTucComplex__C5,
    "tests/boolean/boptuc_complex/C5",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b112"),
    Some(47686.4),
);
case!(
    BOPTucComplex__C6,
    "tests/boolean/boptuc_complex/C6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b114"),
    Some(21046.7),
);
case!(
    BOPTucComplex__C7,
    "tests/boolean/boptuc_complex/C7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a116"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b116"),
    Some(12566.4),
);
case!(
    BOPTucComplex__C8,
    "tests/boolean/boptuc_complex/C8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a116"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b118"),
    Some(4712.39),
);
case!(
    BOPTucComplex__C9,
    "tests/boolean/boptuc_complex/C9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a116"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b124"),
    Some(6126.11),
);
case!(
    BOPTucComplex__D1,
    "tests/boolean/boptuc_complex/D1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a116"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b126"),
    Some(11781.0),
);
case!(
    BOPTucComplex__D2,
    "tests/boolean/boptuc_complex/D2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a138"),
    None,
);
case!(
    BOPTucComplex__D3,
    "tests/boolean/boptuc_complex/D3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b130"),
    Some(6347.92),
);
case!(
    BOPTucComplex__D4,
    "tests/boolean/boptuc_complex/D4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b132"),
    Some(19918.1),
);
case!(
    BOPTucComplex__D6,
    "tests/boolean/boptuc_complex/D6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a138"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b130"),
    Some(11060.3),
);
case!(
    BOPTucComplex__D7,
    "tests/boolean/boptuc_complex/D7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a138"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b140"),
    Some(4454.93),
);
case!(
    BOPTucComplex__D8,
    "tests/boolean/boptuc_complex/D8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b142"),
    Some(25583.2),
);
case!(
    BOPTucComplex__D9,
    "tests/boolean/boptuc_complex/D9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a152"),
    None,
);
case!(
    BOPTucComplex__E1,
    "tests/boolean/boptuc_complex/E1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a158"),
    Some(25034.4),
);
case!(
    BOPTucComplex__E2,
    "tests/boolean/boptuc_complex/E2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a148"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b148"),
    Some(23561.9),
);
case!(
    BOPTucComplex__E3,
    "tests/boolean/boptuc_complex/E3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a160"),
    Some(33379.4),
);
case!(
    BOPTucComplex__E4,
    "tests/boolean/boptuc_complex/E4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a152"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b118"),
    Some(10602.8),
);
case!(
    BOPTucComplex__E5,
    "tests/boolean/boptuc_complex/E5",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a152"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a157"),
    Some(4869.47),
);
case!(
    BOPTucComplex__E6,
    "tests/boolean/boptuc_complex/E6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a157"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a152"),
    Some(10053.1),
);
case!(
    BOPTucComplex__E7,
    "tests/boolean/boptuc_complex/E7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a158"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b148"),
    Some(23561.8),
);
case!(
    BOPTucComplex__E8,
    "tests/boolean/boptuc_complex/E8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a160"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b160"),
    Some(31415.7),
);
case!(
    BOPTucComplex__F7,
    "tests/boolean/boptuc_complex/F7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a206"),
    None,
);
case!(
    BOPTucComplex__F8,
    "tests/boolean/boptuc_complex/F8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b199"),
    Some(23498.7),
);
case!(
    BOPTucComplex__F9,
    "tests/boolean/boptuc_complex/F9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a206"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b199"),
    Some(25398.9),
);
case!(
    BOPTucComplex__G1,
    "tests/boolean/boptuc_complex/G1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a208"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b210"),
    Some(8997.65),
);

fn two_step_cut21(first: Shape, second: Shape) -> Shape {
    let arguments: List<Shape> = [first.clone(), second.clone()].into_iter().collect();
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(&arguments);
    filler.options_mut().set_run_parallel(false);
    filler.perform();
    assert!(!filler.options().has_errors(), "B5: bop preparation failed");
    let mut tuc = BOPAlgoBOP::new();
    tuc.add_argument(&first);
    tuc.add_tool(&second);
    tuc.set_operation(Operation::Cut21);
    tuc.options_mut().set_run_parallel(false);
    tuc.perform_with_filler(&mut filler);
    assert!(!tuc.options().has_errors(), "B5: boptuc failed");
    tuc.shape().clone()
}

fn b5_properties(result: &Shape) {
    let mut area = GProps::default();
    let mut volume = GProps::default();
    BRepGProp::surface_properties(result, &mut area, false, false).expect("B5 area");
    BRepGProp::volume_properties(result, &mut volume, false, false, false).expect("B5 volume");
    assert!(
        (area.mass() - 15000.0).abs() / 15000.0 <= 0.01,
        "B5 area {}",
        area.mass()
    );
    assert!(
        (volume.mass() - 31250.0).abs() / 31250.0 <= 0.01,
        "B5 volume {}",
        volume.mass()
    );
}

fn b5_shape_counts(result: &Shape) {
    use martensite_tkbrep::topexp;
    use martensite_tkbrep::toptools::IndexedMapOfShape;
    use martensite_tkg3d::top_abs::ShapeEnum;
    for (kind, expected) in [
        (ShapeEnum::Wire, 24),
        (ShapeEnum::Face, 24),
        (ShapeEnum::Shell, 6),
        (ShapeEnum::Solid, 6),
    ] {
        let mut found = IndexedMapOfShape::default();
        topexp::map_shapes_of_type(result, kind, &mut found).expect("B5 shape count");
        assert_eq!(found.extent(), expected, "B5 {kind:?} count");
    }
}

fn b5_solids_are_valid(result: &Shape) {
    use martensite_tkbrep::topexp;
    use martensite_tkbrep::toptools::IndexedMapOfShape;
    use martensite_tkg3d::top_abs::ShapeEnum;
    use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
    let mut solids = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(result, ShapeEnum::Solid, &mut solids).expect("B5 solids");
    for solid in solids.iter() {
        let checker = BRepCheckAnalyzer::new(solid, true, false, false).expect("B5 checker");
        assert!(checker.is_valid().expect("B5 validity"));
    }
}

#[test]
fn BOPTucComplex__B5() {
    let first = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/a102"
    ));
    let second = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/b60"
    ));
    let result = two_step_cut21(first, second);
    b5_properties(&result);
    b5_shape_counts(&result);
    b5_solids_are_valid(&result);
}

case!(
    BOPTucComplex__G8,
    "tests/boolean/boptuc_complex/G8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh2"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh2a"),
    Some(250.0),
);

case!(
    BOPTucComplex__G9,
    "tests/boolean/boptuc_complex/G9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh2"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh3"),
    Some(400.0),
);

case!(
    BOPTucComplex__H1,
    "tests/boolean/boptuc_complex/H1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh2"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh4"),
    Some(300.0),
);

case!(
    BOPTucComplex__H2,
    "tests/boolean/boptuc_complex/H2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh2"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh7"),
    Some(300.0),
);

case!(
    BOPTucComplex__H3,
    "tests/boolean/boptuc_complex/H3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh2"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh8"),
    Some(304.289),
);

case!(
    BOPTucComplex__H4,
    "tests/boolean/boptuc_complex/H4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh2"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh9"),
    Some(200.0),
);

case!(
    BOPTucComplex__H5,
    "tests/boolean/boptuc_complex/H5",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh10"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/shx"),
    Some(68.6292),
);

case!(
    BOPTucComplex__H6,
    "tests/boolean/boptuc_complex/H6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh2"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh5"),
    Some(600.0),
);

case!(
    BOPTucComplex__H7,
    "tests/boolean/boptuc_complex/H7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/g1"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/g2"),
    Some(4886.92),
);

case!(
    BOPTucComplex__H8,
    "tests/boolean/boptuc_complex/H8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sg1"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sg2"),
    Some(4886.92),
);

case!(
    BOPTucComplex__H9,
    "tests/boolean/boptuc_complex/H9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sf4"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sf5"),
    Some(61221.0),
);

case!(
    BOPTucComplex__I1,
    "tests/boolean/boptuc_complex/I1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sf5"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sf3"),
    None,
);

case!(
    BOPTucComplex__I2,
    "tests/boolean/boptuc_complex/I2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sf5"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sf55"),
    Some(25132.7),
);
