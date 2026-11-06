use super::*;
fn convex_closed(polygon: &[Point], point: Point) -> bool {
    let signs: Vec<_> = polygon
        .iter()
        .enumerate()
        .map(|(i, &a)| edge(a, polygon[(i + 1) % polygon.len()], point).signum())
        .collect();
    signs.iter().all(|&s| s >= 0) || signs.iter().all(|&s| s <= 0)
}
fn prove_partition(
    polygon: &[Point],
    x: std::ops::RangeInclusive<i64>,
    y: std::ops::RangeInclusive<i64>,
) {
    for px in x {
        for py in y.clone() {
            assert_eq!(
                selected(polygon, [px, py])
                    .unwrap_or_else(|e| panic!("{e:?} at {:?} polygon={polygon:?}", [px, py]))
                    .is_some(),
                convex_closed(polygon, [px, py]),
                "point={:?}, polygon={polygon:?}",
                [px, py]
            );
        }
    }
}
#[test]
fn actual_failing_q8_outer_edge_is_closed_without_changing_any_coverage() {
    let triangle = [[98489, 53567], [98530, 53232], [98420, 53232]];
    let sample = [98480, 53232];
    assert!(!included(triangle[2], triangle[1], sample, false));
    assert_eq!(selected(&triangle, sample), Ok(Some(0)));
    assert_eq!(edge(triangle[2], triangle[1], sample), 0);
    assert_eq!(selected(&triangle, [98480, 53231]), Ok(None));
}
#[test]
fn all_square_edges_corners_and_internal_separator_samples_have_one_owner() {
    let square = [[-4, -4], [4, -4], [4, 4], [-4, 4]];
    prove_partition(&square, -5..=5, -5..=5);
    let reversed: Vec<_> = square.into_iter().rev().collect();
    prove_partition(&reversed, -5..=5, -5..=5);
}
#[test]
fn collapsed_first_middle_last_fans_never_steal_boundary_ownership() {
    for polygon in [
        vec![[0, 0], [1, 0], [4, 0], [4, 4], [0, 4]],
        vec![[0, 0], [4, 0], [4, 4], [4, 4], [0, 4]],
        vec![[0, 0], [4, 0], [4, 4], [0, 4], [0, 1]],
        vec![[0, 0], [0, 0], [4, 0], [4, 4], [0, 4], [0, 0]],
    ] {
        prove_partition(&polygon, -1..=5, -1..=5);
        let reversed: Vec<_> = polygon.into_iter().rev().collect();
        prove_partition(&reversed, -1..=5, -1..=5);
    }
}
#[test]
fn maximum_polygon_and_generated_subpolygons_match_independent_closed_halfspaces() {
    let mut polygon: Vec<Point> = (-7i64..=7).map(|x| [x, x * x]).collect();
    polygon.extend([[7, 64], [-7, 64]]);
    assert_eq!(polygon.len(), 17);
    for count in 3..=17 {
        prove_partition(&polygon[..count], -8..=8, -1..=65);
        let reversed: Vec<_> = polygon[..count].iter().copied().rev().collect();
        prove_partition(&reversed, -8..=8, -1..=65);
    }
}
#[test]
fn degenerate_and_unsupported_polygons_do_not_invent_a_depth_plane() {
    assert_eq!(selected(&[[0, 0], [1, 1], [2, 2]], [1, 1]), Ok(None));
    assert_eq!(
        selected(&[[0, 0]; 2], [0, 0]),
        Err(PartitionFailure::Bounds)
    );
    assert_eq!(
        selected(&[[0, 0]; 18], [0, 0]),
        Err(PartitionFailure::Bounds)
    );
    assert_eq!(
        selected(&[[0, 0], [1, 0], [0, 1]], [16777216, 0]),
        Err(PartitionFailure::Bounds)
    );
    assert_eq!(
        selected(&[[0, 0], [4, 0], [4, 4], [0, 4], [4, 0]], [2, 1]),
        Err(PartitionFailure::Overlap)
    );
}
