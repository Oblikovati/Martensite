//! Adjacency kinds lower to the inner primitives GL rasterizes without a geometry shader.
use super::*;
use Graphic3dTypeOfPrimitiveArray as Kind;

#[test]
fn line_adjacency_keeps_the_inner_segment_of_each_group_of_four() {
    let lines = line_indices(Kind::LinesAdjacency, &[0, 1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
    assert_eq!(lines, [1, 2, 5, 6], "the incomplete tail [8] draws nothing");
}

#[test]
fn line_strip_adjacency_drops_the_first_and_last_vertex() {
    let lines = line_indices(Kind::LineStripAdjacency, &[0, 1, 2, 3, 4]).unwrap();
    assert_eq!(lines, [1, 2, 2, 3]);
    assert!(
        line_indices(Kind::LineStripAdjacency, &[0, 1, 2])
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        line_indices(Kind::LineStripAdjacency, &[0, 1, 2, 3]).unwrap(),
        [1, 2]
    );
}

#[test]
fn triangle_adjacency_keeps_the_even_corners_of_each_group_of_six() {
    let triangles = triangle_indices(Kind::TrianglesAdjacency, &[0, 1, 2, 3, 4, 5, 6]).unwrap();
    assert_eq!(triangles, [0, 2, 4]);
}

#[test]
fn triangle_strip_adjacency_swaps_the_first_corners_of_odd_triangles() {
    let source: Vec<u32> = (0..10).collect();
    let triangles = triangle_indices(Kind::TriangleStripAdjacency, &source).unwrap();
    assert_eq!(triangles, [0, 2, 4, 4, 2, 6, 4, 6, 8]);
    assert!(
        triangle_indices(Kind::TriangleStripAdjacency, &source[..5])
            .unwrap()
            .is_empty()
    );
}

#[test]
fn points_and_undefined_arrays_are_still_rejected() {
    assert!(triangle_indices(Kind::Points, &[0, 1, 2]).is_err());
    assert!(line_indices(Kind::Undefined, &[0, 1]).is_err());
}
