// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::sketch_region_partition_bindings::{
    EndpointIncidenceOrigin, PartitionEndpointIncidence,
};
fn incidence(source_index: i32, parameter: f64) -> PartitionEndpointIncidence {
    PartitionEndpointIncidence {
        source_index,
        parameter,
        origin: EndpointIncidenceOrigin::OwnPaveBoundary,
        own_edge: source_index,
        representative: None,
    }
}
fn expand_actual_endpoint(retained: &mut PairedContactTransaction) {
    let current = retained
        .current_partition
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    let endpoints = &mut current.endpoints.as_mut().unwrap().endpoints;
    let endpoint = endpoints
        .iter_mut()
        .find(|row| row.incidences.len() >= 2)
        .unwrap();
    let first = extend_parameters(endpoint);
    append_duplicate_endpoint(endpoints, first);
}
#[test]
fn many_to_many_signed_zero_and_duplicate_endpoint_rows_match_literal_order() {
    let mut retained = actual_bridge();
    expand_actual_endpoint(&mut retained);
    let mut counts = Counts::default();
    let view = compiled(&retained, &mut counts).unwrap();
    compare(&retained, &view);
    let rows = view.rows([0, 1]);
    assert!(rows.len() > 1);
    assert!(rows.iter().any(|row| {
        row.native_parameters
            .iter()
            .any(|p| p.to_bits() == (-0f64).to_bits())
    }));
}
#[test]
fn self_axis_raw_bit_filtering_is_derived_from_actual_parameter_lists() {
    let mut retained = actual_bridge();
    expand_actual_endpoint(&mut retained);
    let view = compiled(&retained, &mut Counts::default()).unwrap();
    let rows = view.rows([0, 0]);
    assert!(!rows.is_empty());
    assert!(
        rows.iter()
            .all(|row| row.native_parameters[0].to_bits() != row.native_parameters[1].to_bits())
    );
    let mut vertices = BTreeSet::new();
    for row in &rows {
        vertices.insert(row.vertex_index);
    }
    assert_eq!(view.cardinality([0, 0], &mut |_| {}), vertices.len());
}
#[test]
fn duplicate_native_vertex_ids_keep_the_first_snapshot_observation() {
    let mut retained = actual_bridge();
    let PartitionObservation::Captured(snapshot) = &mut retained.mapped.arrangement.partition
    else {
        panic!("captured snapshot required");
    };
    let first = &snapshot.vertices[0];
    snapshot.vertices.push(PartitionVertexObservation {
        shape_index: first.shape_index,
        occurrence: first.occurrence.clone(),
        point: [123., 456., 789.],
        original_parents: first.original_parents.clone(),
    });
    let view = compiled(&retained, &mut Counts::default()).unwrap();
    compare(&retained, &view);
    assert!(
        view.rows([0, 1])
            .iter()
            .all(|row| row.native_point != [123., 456., 789.])
    );
}

fn append_duplicate_endpoint(endpoints: &mut Vec<PartitionEndpointBinding>, first: i32) {
    let endpoint = endpoints
        .iter()
        .find(|row| row.incidences.len() >= 2)
        .unwrap();
    let duplicate = PartitionEndpointBinding {
        vertex_index: endpoint.vertex_index,
        incidences: endpoint
            .incidences
            .iter()
            .map(|row| incidence(row.source_index, row.parameter))
            .collect(),
    };
    endpoints.push(duplicate);
    endpoints.push(PartitionEndpointBinding {
        vertex_index: i32::MAX,
        incidences: vec![incidence(first, 0.25)],
    });
}

fn extend_parameters(endpoint: &mut PartitionEndpointBinding) -> i32 {
    let first = endpoint.incidences[0].source_index;
    let second = endpoint
        .incidences
        .iter()
        .find(|row| row.source_index != first)
        .unwrap()
        .source_index;
    endpoint.incidences.extend([
        incidence(first, -0.),
        incidence(second, 0.),
        incidence(first, 0.),
        incidence(second, -0.),
        incidence(first, -0.),
    ]);
    first
}
#[test]
fn duplicate_and_permuted_source_rows_preserve_native_observation_order() {
    let mut retained = actual_bridge();
    let PartitionObservation::Captured(snapshot) = &mut retained.mapped.arrangement.partition
    else {
        panic!("captured snapshot required");
    };
    let alias = aliased_source(&snapshot.sources[0]);
    snapshot.sources.push(alias);
    snapshot.sources.reverse();
    let view = compiled(&retained, &mut Counts::default()).unwrap();
    compare(&retained, &view);
}
fn aliased_source(source: &SourcePartitionObservation) -> SourcePartitionObservation {
    SourcePartitionObservation {
        source_index: source.source_index,
        parents: source.parents.clone(),
        occurrence: source.occurrence.clone(),
        range: source.range,
        space: None,
        has_reference: source.has_reference,
        source_vertices: Vec::new(),
        blocks: Vec::new(),
        images: None,
    }
}
#[test]
fn changed_parent_orientation_and_location_cannot_authorize_incidence() {
    use martensite_tkmath::{
        gp::{Transform3, Vector3},
        toploc::Location,
    };
    let mut retained = actual_bridge();
    let current = retained
        .current_partition
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    let parent = current.domains[0].parent.as_mut().unwrap();
    parent.orientation = martensite_tkg3d::top_abs::Orientation::Reversed;
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::new(1., 2., 3.));
    parent.location = Location::new(transform);
    let view = compiled(&retained, &mut Counts::default()).unwrap();
    compare(&retained, &view);
    assert_eq!(view.cardinality([0, 1], &mut |_| {}), 0);
}
#[test]
fn unrelated_source_incidence_at_a_valid_endpoint_changes_no_observation() {
    let mut retained = actual_bridge();
    let before = compiled(&retained, &mut Counts::default())
        .unwrap()
        .rows([0, 1]);
    let ids = axis_source_ids(&retained);
    assert_unrelated_source_absent(&retained, i32::MAX);
    shared_endpoint(&mut retained, ids)
        .incidences
        .push(incidence(i32::MAX, 0.875));
    let view = compiled(&retained, &mut Counts::default()).unwrap();
    same_rows(&before, &view.rows([0, 1]));
    compare(&retained, &view);
}
#[test]
fn aliased_sources_have_an_independent_expected_signed_zero_parameter_order() {
    let mut retained = actual_bridge();
    let ids = axis_source_ids(&retained);
    append_axis_alias(&mut retained, ids[0]);
    let endpoint = shared_endpoint(&mut retained, ids);
    let vertex = endpoint.vertex_index;
    endpoint.incidences = vec![
        incidence(ids[1], 0.75),
        incidence(ids[0], -0.),
        incidence(ids[1], 0.75),
        incidence(ids[0], 0.),
        incidence(ids[0], -0.),
    ];
    let view = compiled(&retained, &mut Counts::default()).unwrap();
    assert_expected_order(&view, vertex);
    compare(&retained, &view);
}
fn axis_source_ids(retained: &PairedContactTransaction) -> [i32; 2] {
    let index = super::super::super::compile_source_index(retained).unwrap();
    let PartitionObservation::Captured(snapshot) = &retained.mapped.arrangement.partition else {
        panic!("captured snapshot required");
    };
    std::array::from_fn(|axis| snapshot.sources[index.matching_source_rows(axis)[0]].source_index)
}
fn shared_endpoint(
    retained: &mut PairedContactTransaction,
    ids: [i32; 2],
) -> &mut PartitionEndpointBinding {
    let current = retained
        .current_partition
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    current
        .endpoints
        .as_mut()
        .unwrap()
        .endpoints
        .iter_mut()
        .find(|endpoint| has_axis_sources(endpoint, ids))
        .unwrap()
}
fn assert_unrelated_source_absent(retained: &PairedContactTransaction, id: i32) {
    let PartitionObservation::Captured(snapshot) = &retained.mapped.arrangement.partition else {
        panic!("captured snapshot required");
    };
    assert!(
        snapshot
            .sources
            .iter()
            .all(|source| source.source_index != id)
    );
}
fn append_axis_alias(retained: &mut PairedContactTransaction, id: i32) {
    let PartitionObservation::Captured(snapshot) = &mut retained.mapped.arrangement.partition
    else {
        panic!("captured snapshot required");
    };
    let source = snapshot
        .sources
        .iter()
        .find(|row| row.source_index == id)
        .unwrap();
    let alias = aliased_source(source);
    snapshot.sources.push(alias);
    snapshot.sources.reverse();
}

fn assert_expected_order(view: &ObservedIncidences<'_>, vertex: i32) {
    let actual = view
        .rows([0, 1])
        .into_iter()
        .filter(|row| row.vertex_index == vertex)
        .map(|row| row.native_parameters.map(f64::to_bits))
        .collect::<Vec<_>>();
    assert_eq!(
        actual,
        vec![
            [(-0f64).to_bits(), 0.75f64.to_bits()],
            [0f64.to_bits(), 0.75f64.to_bits()]
        ]
    );
}

fn has_axis_sources(endpoint: &PartitionEndpointBinding, ids: [i32; 2]) -> bool {
    ids.iter().all(|id| {
        endpoint
            .incidences
            .iter()
            .any(|row| row.source_index == *id)
    })
}
