//! Actual input tuples; expected intersections are source-defined f32, not observed post-clip vertices.
use super::*;
use crate::clipped_depth_partition::{self as partition, PartitionFailure};
const CLIP_2190: [u32; 12] = [
    3203878806, 1057175532, 3189907495, 1065353216, 3202306936, 1057757990, 3177967354, 1065353216,
    3202369030, 1057673038, 3177359442, 1065353216,
];
const CLIP_15640: [u32; 12] = [
    3206336698, 1046710936, 3197954358, 1065353216, 3202852572, 1052776084, 1041014682, 1065353216,
    3202977632, 1052618428, 1041174906, 1065353216,
];
const GRID_2190: [[i64; 2]; 4] = [
    [813225, 2379144],
    [886806, 2433676],
    [883896, 2425711],
    [813220, 2379133],
];
const GRID_15640: [[i64; 2]; 4] = [
    [735389, 2042738],
    [861230, 2162959],
    [855368, 2155569],
    [732456, 2039041],
];
fn coordinates(bits: [u32; 12]) -> [[f32; 4]; 3] {
    std::array::from_fn(|i| std::array::from_fn(|j| f32::from_bits(bits[i * 4 + j])))
}
fn incidences(bits: [u32; 3]) -> [[f32; 8]; 3] {
    std::array::from_fn(|i| {
        let mut row = [1.; 8];
        row[0] = f32::from_bits(bits[i]);
        row
    })
}
fn grid_point(vertex: SharedIncidenceVertex, grid: u32) -> [i64; 2] {
    let reciprocal = 1. / vertex.source.clip[3];
    std::array::from_fn(|i| {
        let normalized = vertex.source.clip[i] * reciprocal;
        let product = normalized * 384.;
        let window = product + 384.;
        (window * (1u32 << grid) as f32).round_ties_even() as i64
    })
}
fn grid_polygon(clip: [[f32; 4]; 3], bits: [u32; 3], grid: u32) -> Vec<[i64; 2]> {
    clipped_shared_incidence(clip, incidences(bits), 1)
        .unwrap()
        .into_iter()
        .map(|v| grid_point(v, grid))
        .collect()
}
fn owner(
    clip: [[f32; 4]; 3],
    bits: [u32; 3],
    grid: u32,
    point: [i64; 2],
) -> Result<Option<usize>, PartitionFailure> {
    let polygon = grid_polygon(clip, bits, grid);
    partition::selected(&polygon, point)
}
#[test]
fn captured_2190_q8_orientation_flip_refuses_but_explicit_q12_has_one_owner() {
    let clip = coordinates(CLIP_2190);
    let actual = [3164080454, 1096506738, 1096544025];
    assert_eq!(
        owner(clip, actual, 8, [53776, 150640]),
        Err(PartitionFailure::Overlap)
    );
    assert_eq!(grid_polygon(clip, actual, 12), GRID_2190);
    assert_eq!(owner(clip, actual, 12, [860416, 2410240]), Ok(Some(0)));
}
#[test]
fn captured_15640_shared_incidence_keeps_the_legacy_gap_visible() {
    let clip = coordinates(CLIP_15640);
    let actual = [3253478871, 1105975849, 1105975849];
    let reference = [3253478868, 1105975854, 1105975855];
    assert_eq!(owner(clip, reference, 8, [51280, 132656]), Ok(None));
    assert_eq!(owner(clip, actual, 8, [51280, 132656]), Ok(Some(1)));
    assert_eq!(grid_polygon(clip, actual, 12), GRID_15640);
    assert_eq!(owner(clip, actual, 12, [820480, 2122496]), Ok(Some(1)));
}
