//! Native
#![allow(non_snake_case)]

use super::{assert_surface_area, bop_fuse, restored_shape};

/// Replays `tests/boolean/bopfuse_complex/N8` without command-host at runtime.
#[test]
fn BOPFuseComplex__N8() {
    let source = "tests/boolean/bopfuse_complex/N8";
    let first = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/af"
    ));
    let second = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/bf"
    ));
    let third = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/cf"
    ));
    let first_result = bop_fuse(source, &first, &second);
    let result = bop_fuse(source, &first_result, &third);
    assert_surface_area(source, &result, Some(12677.1));
}
