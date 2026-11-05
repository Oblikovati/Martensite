//! Reference enum values, independent of the incomplete offset construction pipeline.
use martensite_tkoffset::brep_offset::{BRepOffsetError, BRepOffsetMode};
#[test]
fn offset_modes_match_reference_values() {
    assert_values(
        0,
        &[
            BRepOffsetMode::Skin as i32,
            BRepOffsetMode::Pipe as i32,
            BRepOffsetMode::RectoVerso as i32,
        ],
    );
}
#[test]
fn offset_errors_match_reference_values() {
    use BRepOffsetError::*;
    let errors = [
        NoError,
        UnknownError,
        BadNormalsOnGeometry,
        C0Geometry,
        NullOffset,
        NotConnectedShell,
        CannotTrimEdges,
        CannotFuseVertices,
        CannotExtentEdge,
        UserBreak,
        MixedConnectivity,
    ];
    assert_values(1, &errors.map(|error| error as i32));
}
fn assert_values(row: usize, actual: &[i32]) {
    let expected: Vec<i32> = include_str!("fixtures/classification.expected")
        .lines()
        .nth(row)
        .unwrap()
        .split_whitespace()
        .map(|field| field.parse().unwrap())
        .collect();
    assert_eq!(actual, expected);
}
