//! Native Rust translations of complete-input `bopfuse_2d` command-host regressions.
#![allow(non_snake_case)]

use martensite_tkbo::bop_algo::{BOPAlgoBOP, BOPAlgoPaveFiller, Operation};
use martensite_tkbrep::{
    brep::BRepBuilder, brep_tools, topexp, topods::Shape, toptools::IndexedMapOfShape,
};
use martensite_tkernel::{message::ProgressRange, ncollection::List, storage::IoStream};
use martensite_tkg3d::top_abs::ShapeEnum;
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

fn count(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut found = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, kind, &mut found).expect("result shape traversal");
    found.extent()
}

fn fuse_case(
    source: &str,
    first: &[u8],
    second: &[u8],
    expected: Option<f64>,
    vertices: usize,
    edges: usize,
) {
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
    let mut fuse = BOPAlgoBOP::new();
    fuse.add_argument(&first);
    fuse.add_tool(&second);
    fuse.set_operation(Operation::Fuse);
    fuse.options_mut().set_run_parallel(false);
    fuse.perform_with_filler(&mut filler);
    assert!(!fuse.options().has_errors(), "{source}: bopfuse failed");
    let result = fuse.shape();
    let mut properties = GProps::default();
    BRepGProp::linear_properties(result, &mut properties, false, false)
        .unwrap_or_else(|failure| panic!("{source}: length failed: {failure:?}"));
    if let Some(expected) = expected {
        assert!(
            (properties.mass() - expected).abs() / expected <= 0.01,
            "{source}: length {}; expected {expected} within 1%",
            properties.mass()
        );
    } else {
        assert!(
            properties.mass().abs() <= 1e-6,
            "{source}: expected empty length, got {}",
            properties.mass()
        );
    }
    assert_eq!(
        count(result, ShapeEnum::Vertex),
        vertices,
        "{source}: vertex count"
    );
    assert_eq!(
        count(result, ShapeEnum::Edge),
        edges,
        "{source}: edge count"
    );
}

macro_rules! case {
    ($name:ident, $source:literal, $first:expr, $second:expr, $length:expr, $vertices:expr, $edges:expr $(,)?) => {
        #[test]
        fn $name() {
            fuse_case($source, $first, $second, $length, $vertices, $edges);
        }
    };
}

case!(
    BOPFuse2d__A1,
    "tests/boolean/bopfuse_2d/A1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_3_wire.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_3_edge1.brep"),
    Some(635.826),
    12,
    12,
);

case!(
    BOPFuse2d__A2,
    "tests/boolean/bopfuse_2d/A2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_3_wire.brep"),
    include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/offset_wire_034.brep"
    ),
    Some(1085.29),
    14,
    14,
);

case!(
    BOPFuse2d__A3,
    "tests/boolean/bopfuse_2d/A3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_3_wire.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_3_wire2.brep"),
    Some(1085.29),
    20,
    20,
);

case!(
    BOPFuse2d__A4,
    "tests/boolean/bopfuse_2d/A4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_3_wire.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_3_wire5.brep"),
    Some(928.558),
    12,
    13,
);

case!(
    BOPFuse2d__A5,
    "tests/boolean/bopfuse_2d/A5",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge1.brep"),
    Some(1045.95),
    5,
    5,
);

case!(
    BOPFuse2d__A6,
    "tests/boolean/bopfuse_2d/A6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge2.brep"),
    Some(1157.7),
    6,
    6,
);

case!(
    BOPFuse2d__A7,
    "tests/boolean/bopfuse_2d/A7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge3.brep"),
    Some(1252.16),
    7,
    7,
);

case!(
    BOPFuse2d__A8,
    "tests/boolean/bopfuse_2d/A8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge4.brep"),
    Some(1127.57),
    7,
    7,
);

case!(
    BOPFuse2d__A9,
    "tests/boolean/bopfuse_2d/A9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge5.brep"),
    Some(1316.05),
    7,
    7,
);

case!(
    BOPFuse2d__B1,
    "tests/boolean/bopfuse_2d/B1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire1.brep"),
    Some(1953.57),
    8,
    9,
);

case!(
    BOPFuse2d__B2,
    "tests/boolean/bopfuse_2d/B2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire2.brep"),
    Some(2126.59),
    10,
    11,
);

case!(
    BOPFuse2d__B3,
    "tests/boolean/bopfuse_2d/B3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire3.brep"),
    Some(2126.59),
    10,
    11,
);

case!(
    BOPFuse2d__B4,
    "tests/boolean/bopfuse_2d/B4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire_a.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge1.brep"),
    Some(613.678),
    4,
    3,
);

case!(
    BOPFuse2d__B5,
    "tests/boolean/bopfuse_2d/B5",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire_a.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge2.brep"),
    Some(725.427),
    4,
    3,
);

case!(
    BOPFuse2d__B6,
    "tests/boolean/bopfuse_2d/B6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire_a.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge3.brep"),
    Some(963.678),
    5,
    4,
);

case!(
    BOPFuse2d__B7,
    "tests/boolean/bopfuse_2d/B7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire_a.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge4.brep"),
    Some(695.295),
    4,
    3,
);

case!(
    BOPFuse2d__B8,
    "tests/boolean/bopfuse_2d/B8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire_a.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge5.brep"),
    Some(963.678),
    4,
    3,
);

case!(
    BOPFuse2d__B9,
    "tests/boolean/bopfuse_2d/B9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire_a.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire1.brep"),
    Some(1521.3),
    7,
    7,
);

case!(
    BOPFuse2d__C1,
    "tests/boolean/bopfuse_2d/C1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire_a.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire2.brep"),
    Some(1694.32),
    8,
    8,
);

case!(
    BOPFuse2d__C2,
    "tests/boolean/bopfuse_2d/C2",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire_a.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire3.brep"),
    Some(1694.32),
    7,
    7,
);

case!(
    BOPFuse2d__C3,
    "tests/boolean/bopfuse_2d/C3",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge1.brep"),
    Some(613.678),
    4,
    3,
);

case!(
    BOPFuse2d__C4,
    "tests/boolean/bopfuse_2d/C4",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge2.brep"),
    Some(725.427),
    4,
    3,
);

case!(
    BOPFuse2d__C5,
    "tests/boolean/bopfuse_2d/C5",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge3.brep"),
    Some(963.678),
    5,
    4,
);

case!(
    BOPFuse2d__C6,
    "tests/boolean/bopfuse_2d/C6",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge4.brep"),
    Some(695.295),
    4,
    3,
);

case!(
    BOPFuse2d__C7,
    "tests/boolean/bopfuse_2d/C7",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge5.brep"),
    Some(963.678),
    4,
    3,
);

case!(
    BOPFuse2d__C8,
    "tests/boolean/bopfuse_2d/C8",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire1.brep"),
    Some(1377.51),
    7,
    7,
);

case!(
    BOPFuse2d__C9,
    "tests/boolean/bopfuse_2d/C9",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire2.brep"),
    Some(1694.32),
    8,
    8,
);

case!(
    BOPFuse2d__D1,
    "tests/boolean/bopfuse_2d/D1",
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_edge.brep"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/case_9_wire3.brep"),
    Some(1694.32),
    7,
    7,
);
