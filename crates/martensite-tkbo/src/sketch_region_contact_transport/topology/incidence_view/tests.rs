// SPDX-License-Identifier: GPL-2.0-only
//! Literal native discovery remains the oracle for the immutable incidence view.
use super::super::super::test_fixture::actual_bridge;
use super::*;
mod mutations;
#[derive(Default)]
struct Counts {
    membership: usize,
    endpoints: usize,
    vertices: usize,
    lookups: usize,
    incidences: usize,
    candidates: usize,
    products: usize,
    dedup: usize,
    reports: usize,
}
impl Counts {
    fn record(&mut self, step: DiscoveryStep) {
        match step {
            DiscoveryStep::SourceMembership => self.membership += 1,
            DiscoveryStep::Endpoint => self.endpoints += 1,
            DiscoveryStep::VertexEntry => self.vertices += 1,
            DiscoveryStep::VertexLookup => self.lookups += 1,
            DiscoveryStep::Incidence => self.incidences += 1,
            DiscoveryStep::ParameterCandidate => self.candidates += 1,
            DiscoveryStep::ParameterProduct => self.products += 1,
            DiscoveryStep::VertexDedup => self.dedup += 1,
            DiscoveryStep::ReportMembership => self.reports += 1,
        }
    }
}
fn compiled<'a>(
    retained: &'a PairedContactTransaction,
    counts: &mut Counts,
) -> Option<ObservedIncidences<'a>> {
    let index = super::super::compile_source_index(retained);
    compile_observed(
        &retained.census.inputs,
        &retained.mapped.arrangement.partition,
        retained
            .current_partition
            .as_ref()
            .and_then(|row| row.as_ref().ok()),
        index.as_ref(),
        &mut |step| counts.record(step),
    )
}
fn same_rows(old: &[ObservedContactVertex], new: &[ObservedContactVertex]) {
    assert_eq!(old.len(), new.len());
    for (a, b) in old.iter().zip(new) {
        assert_eq!(a.vertex_index, b.vertex_index);
        assert_eq!(a.occurrence, b.occurrence);
        assert_eq!(a.original_parents, b.original_parents);
        assert_eq!(
            a.native_point.map(f64::to_bits),
            b.native_point.map(f64::to_bits)
        );
        assert_eq!(
            a.native_parameters.map(f64::to_bits),
            b.native_parameters.map(f64::to_bits)
        );
        assert!(a.generated_observations.is_empty() && b.generated_observations.is_empty());
        assert!(a.parameter_measurement.is_none() && b.parameter_measurement.is_none());
    }
}
fn compare(retained: &PairedContactTransaction, view: &ObservedIncidences<'_>) {
    for domain in 0..retained.census.domains.len() {
        let old = super::super::observed_domain_vertices(retained, domain);
        let Some(axes) = super::super::domain_axes(retained, domain) else {
            assert!(old.is_empty());
            continue;
        };
        let new = view.rows(axes);
        same_rows(&old, &new);
        let mut vertices = Vec::new();
        for row in old {
            if !vertices.contains(&row.vertex_index) {
                vertices.push(row.vertex_index);
            }
        }
        assert_eq!(view.cardinality(axes, &mut |_| {}), vertices.len());
    }
}
#[test]
fn complete_actual_rows_and_cardinality_equal_literal_discovery() {
    let retained = actual_bridge();
    let mut counts = Counts::default();
    let view = compiled(&retained, &mut counts).unwrap();
    compare(&retained, &view);
    assert!(counts.endpoints > 0);
    assert!(counts.incidences > 0);
    assert!(counts.candidates > 0);
}
#[test]
fn endpoint_and_vertex_discovery_are_not_repeated_for_report_queries() {
    let retained = actual_bridge();
    let mut counts = Counts::default();
    let view = compiled(&retained, &mut counts).unwrap();
    let initial = discovery_counts(&counts);
    for _ in 0..32 {
        view.cardinality([0, 1], &mut |step| counts.record(step));
    }
    assert_eq!(initial, discovery_counts(&counts));
    let (endpoints, predicates) = literal_vertex_predicates(&retained);
    assert_eq!(counts.endpoints, endpoints);
    assert_eq!(counts.lookups, endpoints);
    assert!(predicates * 32 > counts.vertices + counts.lookups);
    assert_eq!(counts.products, 0);
    assert!(counts.dedup > 0);
}
fn literal_vertex_predicates(retained: &PairedContactTransaction) -> (usize, usize) {
    let PartitionObservation::Captured(snapshot) = &retained.mapped.arrangement.partition else {
        panic!("captured snapshot required");
    };
    let current = retained
        .current_partition
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let endpoints = &current.endpoints.as_ref().unwrap().endpoints;
    let mut predicates = 0;
    for endpoint in endpoints {
        let _ = snapshot.vertices.iter().find(|vertex| {
            predicates += 1;
            vertex.shape_index == endpoint.vertex_index
        });
    }
    (endpoints.len(), predicates)
}
#[test]
fn invalid_current_capture_does_not_create_an_incidence_authority() {
    let mut retained = actual_bridge();
    retained.current_partition = None;
    assert!(compiled(&retained, &mut Counts::default()).is_none());
    retained.mapped.arrangement.partition = PartitionObservation::NotReached;
    assert!(compiled(&retained, &mut Counts::default()).is_none());
}

fn discovery_counts(counts: &Counts) -> [usize; 5] {
    [
        counts.endpoints,
        counts.vertices,
        counts.lookups,
        counts.incidences,
        counts.membership,
    ]
}
