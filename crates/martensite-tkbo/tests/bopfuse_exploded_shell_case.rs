//! Native
#![allow(non_snake_case)]

use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkg3d::top_abs::ShapeEnum;

use super::{assert_surface_area, bop_fuse, restored_shape};

fn first_explored_shell(source: &str, shape: &Shape) -> Shape {
    let explorer = Explorer::new(shape, ShapeEnum::Shell, ShapeEnum::Shape)
        .unwrap_or_else(|failure| panic!("{source}: shell exploration failed: {failure:?}"));
    assert!(explorer.more(), "{source}: expected an exploded shell");
    explorer.current().clone()
}

fn exploded_shell_fuse_case(source: &str, first: &[u8], second: &[u8], expected_area: f64) {
    let first = restored_shape(first);
    let second = restored_shape(second);
    let first_shell = first_explored_shell(source, &first);
    let second_shell = first_explored_shell(source, &second);
    let result = bop_fuse(source, &first_shell, &second_shell);
    assert_surface_area(source, &result, Some(expected_area));
}

/// Replays `tests/boolean/bopfuse_complex/K8` without command-host at runtime.
#[test]
fn BOPFuseComplex__K8() {
    exploded_shell_fuse_case(
        "tests/boolean/bopfuse_complex/K8",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO907_topo002-o1.brep"
        ),
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO907_topo002-o2.brep"
        ),
        3950.89,
    );
}

/// Replays `tests/boolean/bopfuse_complex/L1` without command-host at runtime.
#[test]
fn BOPFuseComplex__L1() {
    exploded_shell_fuse_case(
        "tests/boolean/bopfuse_complex/L1",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/Jyls3"),
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/Jyls4"),
        45600.0,
    );
}

/// Replays `tests/boolean/bopfuse_complex/M1` without command-host at runtime.
#[test]
fn BOPFuseComplex__M1() {
    exploded_shell_fuse_case(
        "tests/boolean/bopfuse_complex/M1",
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO909_object.brep"
        ),
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO909_tool_4.brep"
        ),
        22963.8,
    );
}
