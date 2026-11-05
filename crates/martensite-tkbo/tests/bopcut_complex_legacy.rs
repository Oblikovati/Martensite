//! Native Rust translations of simple complete-input `bopcut_complex` regressions.
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

fn cut_case(source: &str, first: &[u8], second: &[u8], expected: Option<f64>) {
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
    let mut cut = BOPAlgoBOP::new();
    cut.add_argument(&first);
    cut.add_tool(&second);
    cut.set_operation(Operation::Cut);
    cut.options_mut().set_run_parallel(false);
    cut.perform_with_filler(&mut filler);
    assert!(!cut.options().has_errors(), "{source}: bopcut failed");
    let mut properties = GProps::default();
    BRepGProp::surface_properties(cut.shape(), &mut properties, false, false)
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
            cut_case($source, $first, $second, $area);
        }
    };
}

case!(
    BOPCutComplex__F8,
    "tests/boolean/bopcut_complex/F8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b11"),
    Some(17478.5),
);

case!(
    BOPCutComplex__F9,
    "tests/boolean/bopcut_complex/F9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b13"),
    Some(18354.2),
);

case!(
    BOPCutComplex__G1,
    "tests/boolean/bopcut_complex/G1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b15"),
    Some(16226.2),
);

case!(
    BOPCutComplex__G2,
    "tests/boolean/bopcut_complex/G2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b17"),
    Some(14227.1),
);

case!(
    BOPCutComplex__G3,
    "tests/boolean/bopcut_complex/G3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b19"),
    Some(15874.1),
);

case!(
    BOPCutComplex__G4,
    "tests/boolean/bopcut_complex/G4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b21"),
    Some(8143.4),
);

case!(
    BOPCutComplex__G5,
    "tests/boolean/bopcut_complex/G5",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a57"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    Some(7199.97),
);

case!(
    BOPCutComplex__G6,
    "tests/boolean/bopcut_complex/G6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b58"),
    Some(15294.0),
);

case!(
    BOPCutComplex__G7,
    "tests/boolean/bopcut_complex/G7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b60"),
    Some(15000.0),
);

case!(
    BOPCutComplex__G8,
    "tests/boolean/bopcut_complex/G8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b62"),
    Some(19602.9),
);

case!(
    BOPCutComplex__G9,
    "tests/boolean/bopcut_complex/G9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b64"),
    Some(17745.2),
);

case!(
    BOPCutComplex__H1,
    "tests/boolean/bopcut_complex/H1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b66"),
    Some(16123.7),
);

case!(
    BOPCutComplex__H2,
    "tests/boolean/bopcut_complex/H2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a90"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b90"),
    Some(19142.1),
);

case!(
    BOPCutComplex__H3,
    "tests/boolean/bopcut_complex/H3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b102"),
    None,
);

case!(
    BOPCutComplex__H4,
    "tests/boolean/bopcut_complex/H4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b104"),
    Some(9359.94),
);

case!(
    BOPCutComplex__H5,
    "tests/boolean/bopcut_complex/H5",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a106"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b106"),
    Some(9519.3),
);

case!(
    BOPCutComplex__H6,
    "tests/boolean/bopcut_complex/H6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a106"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a157"),
    Some(9519.3),
);

case!(
    BOPCutComplex__H7,
    "tests/boolean/bopcut_complex/H7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b110"),
    Some(16073.9),
);

case!(
    BOPCutComplex__H8,
    "tests/boolean/bopcut_complex/H8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b112"),
    Some(2.32262e-10),
);

case!(
    BOPCutComplex__H9,
    "tests/boolean/bopcut_complex/H9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b114"),
    Some(11830.1),
);

case!(
    BOPCutComplex__I1,
    "tests/boolean/bopcut_complex/I1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a116"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b116"),
    Some(7853.98),
);

case!(
    BOPCutComplex__I2,
    "tests/boolean/bopcut_complex/I2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a116"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b118"),
    Some(7853.98),
);

case!(
    BOPCutComplex__I3,
    "tests/boolean/bopcut_complex/I3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a116"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b124"),
    Some(11781.0),
);

case!(
    BOPCutComplex__I4,
    "tests/boolean/bopcut_complex/I4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a116"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b126"),
    Some(11781.0),
);

case!(
    BOPCutComplex__I5,
    "tests/boolean/bopcut_complex/I5",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a138"),
    Some(25053.1),
);

case!(
    BOPCutComplex__I6,
    "tests/boolean/bopcut_complex/I6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b130"),
    Some(18298.7),
);

case!(
    BOPCutComplex__I7,
    "tests/boolean/bopcut_complex/I7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b132"),
    Some(21073.1),
);

case!(
    BOPCutComplex__I8,
    "tests/boolean/bopcut_complex/I8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b136"),
    Some(15256.6),
);

case!(
    BOPCutComplex__I9,
    "tests/boolean/bopcut_complex/I9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a138"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b130"),
    Some(15079.6),
);

case!(
    BOPCutComplex__J1,
    "tests/boolean/bopcut_complex/J1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a138"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b140"),
    Some(15079.6),
);

case!(
    BOPCutComplex__J2,
    "tests/boolean/bopcut_complex/J2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b142"),
    Some(5124.72),
);

case!(
    BOPCutComplex__J3,
    "tests/boolean/bopcut_complex/J3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a152"),
    Some(22854.0),
);

case!(
    BOPCutComplex__J4,
    "tests/boolean/bopcut_complex/J4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a158"),
    Some(13527.4),
);

case!(
    BOPCutComplex__J5,
    "tests/boolean/bopcut_complex/J5",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a148"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b148"),
    Some(23561.9),
);

case!(
    BOPCutComplex__J6,
    "tests/boolean/bopcut_complex/J6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a160"),
    Some(13036.5),
);

case!(
    BOPCutComplex__J7,
    "tests/boolean/bopcut_complex/J7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a152"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b118"),
    Some(5890.6),
);

case!(
    BOPCutComplex__J8,
    "tests/boolean/bopcut_complex/J8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a152"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a157"),
    Some(10053.1),
);

case!(
    BOPCutComplex__J9,
    "tests/boolean/bopcut_complex/J9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a157"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a152"),
    Some(4869.47),
);

case!(
    BOPCutComplex__K1,
    "tests/boolean/bopcut_complex/K1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a158"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b148"),
    Some(23561.8),
);

case!(
    BOPCutComplex__K2,
    "tests/boolean/bopcut_complex/K2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a160"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b160"),
    Some(31415.7),
);

case!(
    BOPCutComplex__K9,
    "tests/boolean/bopcut_complex/K9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b193"),
    Some(10000.0),
);

case!(
    BOPCutComplex__L1,
    "tests/boolean/bopcut_complex/L1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a206"),
    Some(19007.8),
);

case!(
    BOPCutComplex__L2,
    "tests/boolean/bopcut_complex/L2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a102"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b199"),
    Some(18083.6),
);

case!(
    BOPCutComplex__L3,
    "tests/boolean/bopcut_complex/L3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a206"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b199"),
    Some(15190.4),
);

case!(
    BOPCutComplex__L6,
    "tests/boolean/bopcut_complex/L6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/a349"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/b349"),
    Some(79.0703),
);

case!(
    BOPCutComplex__L9,
    "tests/boolean/bopcut_complex/L9",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_pro13075a.rle"
    ),
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_pro13075b.rle"
    ),
    Some(3053.84),
);

case!(
    BOPCutComplex__M3,
    "tests/boolean/bopcut_complex/M3",
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts20339-part.rle"
    ),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/cts20339-tool.rle"),
    Some(407561.0),
);

case!(
    BOPCutComplex__M4,
    "tests/boolean/bopcut_complex/M4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/cts20975a.rle"),
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts20975b.rle"
    ),
    Some(5609.09),
);
