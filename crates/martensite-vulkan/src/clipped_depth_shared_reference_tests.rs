use super::*;
fn clip() -> [[f32; 4]; 3] {
    [[-1., -1., 0., 1.], [1., -1., 0., 1.], [1., 1., 0., 1.]]
}
fn distances(first: [f32; 3]) -> [[f32; 8]; 3] {
    std::array::from_fn(|i| {
        let mut row = [1.; 8];
        row[0] = first[i];
        row
    })
}
fn require_weights(vertex: SharedIncidenceVertex, original: [[f32; 4]; 3]) {
    for (axis, clip) in vertex.source.clip.iter().enumerate() {
        let expected: f64 = (0..3)
            .map(|i| f64::from(vertex.source.weights[i]) * f64::from(original[i][axis]))
            .sum();
        assert_eq!(f64::from(*clip), expected);
    }
}
#[test]
fn exact_binary_cut_has_independent_closed_form_vertices_and_parent_weights() {
    let result = clipped_shared_incidence(clip(), distances([-1., 1., 1.]), 1).unwrap();
    assert_eq!(result.len(), 4);
    assert_eq!(result[0].source.clip, [0., -1., 0., 1.]);
    assert_eq!(result[0].source.weights, [0.5, 0.5, 0.]);
    assert_eq!(result[3].source.clip, [0., 0., 0., 1.]);
    assert_eq!(result[3].source.weights, [0.5, 0., 0.5]);
    for vertex in result {
        require_weights(vertex, clip());
    }
}
#[test]
fn actual_incidence_controls_topology_even_when_no_inverse_plane_can_reproduce_it() {
    let output = clipped_shared_incidence(clip(), distances([0., 0., 0.]), 1).unwrap();
    assert_eq!(output.len(), 3);
    for (i, vertex) in output.iter().enumerate() {
        assert_eq!(vertex.source.clip, clip()[i]);
    }
    assert!(
        clipped_shared_incidence(clip(), distances([-1.; 3]), 1)
            .unwrap()
            .is_empty()
    );
}
#[test]
fn ordered_multi_plane_chain_propagates_each_actual_incidence_with_the_same_edge_parameter() {
    let mut incidence = distances([-1., 1., 1.]);
    for (i, row) in incidence.iter_mut().enumerate() {
        row[1] = clip()[i][1];
    }
    let result = clipped_shared_incidence(clip(), incidence, 2).unwrap();
    assert_eq!(result.len(), 4);
    assert_eq!(result[2].source.clip, result[3].source.clip);
    for vertex in result {
        assert!(vertex.source.clip[0] >= 0. && vertex.source.clip[1] >= 0.);
        assert_eq!(vertex.incidences[0], vertex.source.clip[0]);
        assert_eq!(vertex.incidences[1], vertex.source.clip[1]);
        require_weights(vertex, clip());
    }
}
#[test]
fn bounds_nonfinite_and_overflow_refuse_without_returning_partial_geometry() {
    for count in [0, 9] {
        assert!(clipped_shared_incidence(clip(), distances([1.; 3]), count).is_err());
    }
    let mut invalid = distances([1.; 3]);
    invalid[2][7] = f32::NAN;
    assert!(clipped_shared_incidence(clip(), invalid, 1).is_err());
    assert!(clipped_shared_incidence(clip(), distances([-f32::MAX, f32::MAX, 1.]), 1).is_err());
}
#[test]
fn input_rows_remain_owned_and_unchanged_after_success_and_failure() {
    let original = clip();
    let incidence = distances([-1., 1., 1.]);
    let mut result = clipped_shared_incidence(original, incidence, 1).unwrap();
    result[0].source.clip[0] = 99.;
    result[0].incidences[0] = 88.;
    assert_eq!(original, clip());
    assert_eq!(incidence, distances([-1., 1., 1.]));
    let _ = clipped_shared_incidence(original, incidence, 9);
    assert_eq!(original, clip());
    assert_eq!(incidence, distances([-1., 1., 1.]));
}
