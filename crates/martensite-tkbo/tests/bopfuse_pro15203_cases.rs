//! Native Rust translations of the pro15203 face-selection fuse regressions.
#![allow(non_snake_case)]

use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkg3d::top_abs::ShapeEnum;

use super::{assert_surface_area, bop_fuse, restored_shape};

fn first_explored_face(source: &str, shape: &Shape) -> Shape {
    let explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap_or_else(|failure| panic!("{source}: face exploration failed: {failure:?}"));
    assert!(explorer.more(), "{source}: expected an exploded face");
    explorer.current().clone()
}

fn pro15203_fuse_case(source: &str, second: &[u8]) {
    let first = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/pro15203a.rle"
    ));
    let second = restored_shape(second);
    let first_face = first_explored_face(source, &first);
    let second_face = first_explored_face(source, &second);
    let result = bop_fuse(source, &first_face, &second_face);
    assert_surface_area(source, &result, Some(570.047));
}

#[test]
fn BOPFuseComplex__L5() {
    pro15203_fuse_case(
        "tests/boolean/bopfuse_complex/L5",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro15203c.rle"),
    );
}

#[test]
fn BOPFuseComplex__L6() {
    pro15203_fuse_case(
        "tests/boolean/bopfuse_complex/L6",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro15203d.rle"),
    );
}

#[test]
fn BOPFuseComplex__M3() {
    pro15203_fuse_case(
        "tests/boolean/bopfuse_complex/M3",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro15203c.rle"),
    );
}

#[test]
fn BOPFuseComplex__M4() {
    pro15203_fuse_case(
        "tests/boolean/bopfuse_complex/M4",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro15203d.rle"),
    );
}
