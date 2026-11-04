use super::*;

fn source_records() -> Vec<u8> {
    (0..4u32)
        .flat_map(|vertex| {
            (0..10u32).flat_map(move |component| (100 * vertex + component).to_ne_bytes())
        })
        .collect()
}

#[test]
fn two_shared_triangles_preserve_source_order_count_and_all_original_components() {
    let base = source_records();
    let input = [3, 1, 0, 3, 0, 2];
    let packed = encode(&base, &input);
    assert_eq!(packed.indices, [0, 1, 2, 3, 4, 5]);
    assert_eq!(packed.sources, input.map(|i| (i as usize, i as usize)));
    assert_eq!(packed.vertices.len(), input.len() * STRIDE);
    for (corner, &source) in input.iter().enumerate() {
        assert_eq!(
            &packed.vertices[corner * STRIDE..corner * STRIDE + 40],
            vertex_record(&base, source)
        );
    }
    assert_eq!(base, source_records());
}

#[test]
fn every_provoking_corner_contains_the_exact_same_ordered_triangle_payload() {
    let base = source_records();
    let packed = encode(&base, &[2, 0, 3]);
    let expected = expected_payload(&base, [2, 0, 3]);
    for corner in 0..3 {
        assert_eq!(
            &packed.vertices[corner * STRIDE + 40..(corner + 1) * STRIDE],
            expected
        );
    }
}

fn expected_payload(base: &[u8], indices: [u32; 3]) -> Vec<u8> {
    indices
        .into_iter()
        .flat_map(|i| vertex_record(base, i)[..16].to_vec())
        .chain(
            indices
                .into_iter()
                .flat_map(|i| vertex_record(base, i)[32..40].to_vec()),
        )
        .collect()
}

#[test]
fn winding_and_repeated_indices_are_preserved_without_geometric_substitution() {
    let base = source_records();
    for indices in [[0, 1, 2], [0, 2, 1], [2, 2, 0]] {
        let packed = encode(&base, &indices);
        assert_eq!(packed.sources, indices.map(|i| (i as usize, i as usize)));
        assert_eq!(
            &packed.vertices[40..88],
            indices
                .into_iter()
                .flat_map(|i| vertex_record(&base, i)[..16].to_vec())
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn empty_input_preserves_an_empty_draw() {
    let packed = encode(&[], &[]);
    assert!(packed.vertices.is_empty() && packed.indices.is_empty() && packed.sources.is_empty());
}
