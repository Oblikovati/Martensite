// SPDX-License-Identifier: GPL-2.0-only
use super::*;
type ParameterCandidate = (usize, usize, f64);
pub(super) fn capture<'a>(
    occurrences: usize,
    snapshot: &'a OwnedPartitionSnapshot,
    endpoints: &[PartitionEndpointBinding],
    bindings: &sources::SourceBindings,
    observe: &mut impl FnMut(DiscoveryStep),
) -> ObservedIncidences<'a> {
    let vertices = vertices(snapshot, observe);
    let mut axes = (0..occurrences).map(|_| Vec::new()).collect::<Vec<_>>();
    for (position, endpoint) in endpoints.iter().enumerate() {
        observe(DiscoveryStep::Endpoint);
        observe(DiscoveryStep::VertexLookup);
        let Some(vertex) = vertices.get(&endpoint.vertex_index) else {
            continue;
        };
        append_endpoint(&mut axes, position, vertex, endpoint, bindings, observe);
    }
    ObservedIncidences { axes }
}
fn vertices<'a>(
    snapshot: &'a OwnedPartitionSnapshot,
    observe: &mut impl FnMut(DiscoveryStep),
) -> BTreeMap<i32, &'a PartitionVertexObservation> {
    let mut vertices = BTreeMap::new();
    for vertex in &snapshot.vertices {
        observe(DiscoveryStep::VertexEntry);
        vertices.entry(vertex.shape_index).or_insert(vertex);
    }
    vertices
}
fn append_endpoint<'a>(
    axes: &mut [Vec<AxisEndpoint<'a>>],
    position: usize,
    vertex: &'a PartitionVertexObservation,
    endpoint: &PartitionEndpointBinding,
    bindings: &sources::SourceBindings,
    observe: &mut impl FnMut(DiscoveryStep),
) {
    for (occurrence, mut candidates) in candidates(endpoint, bindings, observe) {
        candidates.sort_by_key(|(row, incidence, _)| (*row, *incidence));
        let parameters = parameters(candidates);
        if let Some(axis) = axes.get_mut(occurrence) {
            axis.push(AxisEndpoint {
                endpoint: position,
                vertex,
                parameters,
            });
        }
    }
}
fn candidates(
    endpoint: &PartitionEndpointBinding,
    bindings: &sources::SourceBindings,
    observe: &mut impl FnMut(DiscoveryStep),
) -> BTreeMap<usize, Vec<ParameterCandidate>> {
    let mut grouped = BTreeMap::new();
    for (incidence, row) in endpoint.incidences.iter().enumerate() {
        observe(DiscoveryStep::Incidence);
        let Some(bindings) = bindings.get(&row.source_index) else {
            continue;
        };
        for (occurrence, source) in bindings {
            observe(DiscoveryStep::ParameterCandidate);
            grouped.entry(*occurrence).or_insert_with(Vec::new).push((
                *source,
                incidence,
                row.parameter,
            ));
        }
    }
    grouped
}
fn parameters(candidates: Vec<ParameterCandidate>) -> Vec<f64> {
    let mut seen = BTreeSet::new();
    candidates
        .into_iter()
        .filter_map(|(_, _, parameter)| seen.insert(parameter.to_bits()).then_some(parameter))
        .collect()
}
