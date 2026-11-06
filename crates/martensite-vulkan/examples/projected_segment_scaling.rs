//! Accepted projection workload keeps timing and complete output separate from external allocation profiling.
#[path = "support/projected_scaling/workload.rs"]
mod workload;
use martensite_vulkan::{
    DashBehavior, PrimitiveDraw, ProjectedSegments, prepare_projected_segments,
};
use std::time::Instant;
fn measured(count: usize) -> (ProjectedSegments, u128) {
    let source = workload::input(count);
    let retained = source.clone();
    let identities: Vec<u64> = (0..count as u64).collect();
    let started = Instant::now();
    let output = prepared(&source, &identities);
    let elapsed = started.elapsed().as_nanos();
    assert_eq!(source, retained);
    assert_eq!(output.draws().len(), count);
    assert_eq!(output.source_identities(), identities);
    (output, elapsed)
}
fn prepared(source: &PrimitiveDraw, identities: &[u64]) -> ProjectedSegments {
    prepare_projected_segments(
        source,
        [96, 64],
        DashBehavior::FiniteProjectiveV2,
        1e-4,
        identities,
    )
    .unwrap()
}
fn requested_count() -> usize {
    let value = std::env::args()
        .nth(1)
        .expect("missing segment count: expected integer1..=16384");
    let count: usize = value
        .parse()
        .unwrap_or_else(|_| panic!("segment count {value:?}: expected integer1..=16384"));
    assert!(
        (1..=16384).contains(&count),
        "segment count {count}: expected integer1..=16384"
    );
    count
}
fn retain_record(record: &str) {
    let Some(root) = std::env::var_os("MARTENSITE_GEOMETRY_ARTIFACT_DIR") else {
        return;
    };
    let root = std::path::PathBuf::from(root);
    std::fs::create_dir_all(&root)
        .unwrap_or_else(|error| panic!("artifact directory {root:?}: expected writable: {error}"));
    let path = root.join("complete-projected-output.txt");
    std::fs::write(&path, record)
        .unwrap_or_else(|error| panic!("artifact file {path:?}: expected writable: {error}"));
}
fn main() {
    let count = requested_count();
    let (output, nanos) = measured(count);
    let record = format!("{output:?}");
    retain_record(&record);
    let digest = workload::digest(&record);
    println!(
        r#"{{"segments":{count},"accepted":true,"elapsed_ns":{nanos},"complete_output_digest":"{digest:016x}"}}"#
    );
}
