//! Native Rust translations of small stored-shape BOP fuse regressions.
#![allow(non_snake_case)]

use super::fuse_case;

/// Replays `tests/boolean/bopfuse_complex/N5` without command-host at runtime.
#[test]
fn BOPFuseComplex__N5() {
    fuse_case(
        "tests/boolean/bopfuse_complex/N5",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh2"),
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh9"),
        Some(1200.0),
    );
}

/// Replays `tests/boolean/bopfuse_complex/N6` without command-host at runtime.
#[test]
fn BOPFuseComplex__N6() {
    fuse_case(
        "tests/boolean/bopfuse_complex/N6",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh10"),
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/shx"),
        Some(800.0),
    );
}

/// Replays `tests/boolean/bopfuse_complex/N7` without command-host at runtime.
#[test]
fn BOPFuseComplex__N7() {
    fuse_case(
        "tests/boolean/bopfuse_complex/N7",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh2"),
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sh5"),
        Some(1200.0),
    );
}

/// Replays `tests/boolean/bopfuse_complex/N9` without command-host at runtime.
#[test]
fn BOPFuseComplex__N9() {
    fuse_case(
        "tests/boolean/bopfuse_complex/N9",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/g1"),
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/g2"),
        Some(11868.2),
    );
}

/// Replays `tests/boolean/bopfuse_complex/O1` without command-host at runtime.
#[test]
fn BOPFuseComplex__O1() {
    fuse_case(
        "tests/boolean/bopfuse_complex/O1",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sg1"),
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sg2"),
        Some(13962.6),
    );
}

/// Replays `tests/boolean/bopfuse_complex/O2` without command-host at runtime.
#[test]
fn BOPFuseComplex__O2() {
    fuse_case(
        "tests/boolean/bopfuse_complex/O2",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sf4"),
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sf5"),
        Some(62831.9),
    );
}

/// Replays `tests/boolean/bopfuse_complex/O3` without command-host at runtime.
#[test]
fn BOPFuseComplex__O3() {
    fuse_case(
        "tests/boolean/bopfuse_complex/O3",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sf5"),
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sf3"),
        Some(62831.9),
    );
}

/// Replays `tests/boolean/bopfuse_complex/O4` without command-host at runtime.
#[test]
fn BOPFuseComplex__O4() {
    fuse_case(
        "tests/boolean/bopfuse_complex/O4",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sf5"),
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/sf55"),
        Some(125664.0),
    );
}
