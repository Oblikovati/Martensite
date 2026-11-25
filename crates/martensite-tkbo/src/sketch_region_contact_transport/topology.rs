// SPDX-License-Identifier: GPL-2.0-only
//! Shared DS topology supplies observations; a metric bound never supplies contact identity.
use super::*;
pub(super) mod incidence_view;
mod source_index;
use crate::sketch_region::{PartitionObservation, SourcePartitionObservation};
use crate::sketch_region_partition_bindings::PartitionEndpointBinding;
use martensite_tkmath::predicates::roots::metric_bounds::MetricParameterMap;
pub(super) use source_index::SourceIndex;

pub(super) fn compile_source_index(retained: &PairedContactTransaction) -> Option<SourceIndex> {
    let PartitionObservation::Captured(snapshot) = &retained.mapped.arrangement.partition else {
        return None;
    };
    let current = retained.current_partition.as_ref()?.as_ref().ok()?;
    Some(SourceIndex::new(
        retained
            .census
            .inputs
            .segments
            .iter()
            .map(|segment| segment.parents.as_slice()),
        &snapshot.sources,
        current
            .domains
            .iter()
            .map(|row| (row.source_index, row.parent.as_ref(), row.decision.is_ok())),
    ))
}
pub(super) fn retain_topological_incidences(retained: &mut PairedContactTransaction) {
    let sources = compile_source_index(retained);
    let observations = incidence_view::compile(
        &retained.census.inputs,
        &retained.mapped.arrangement.partition,
        retained
            .current_partition
            .as_ref()
            .and_then(|row| row.as_ref().ok()),
        sources.as_ref(),
    );
    for index in 0..retained.inspection.contacts.len() {
        let domain = retained.inspection.contacts[index].occurrence_domains[0];
        let rows = retained_domain_rows(retained, domain, observations.as_ref());
        retained.inspection.contacts[index].observed_vertices = rows;
        // WHY: even a unique report-local root and shared DS vertex need independent source correspondence.
        let identity = retained.inspection.contacts[index].identity;
        retained.inspection.contacts[index]
            .missing
            .push(ContactTransportGap::MissingCanonicalTopologyCorrespondence { identity });
    }
}
fn retained_domain_rows(
    retained: &PairedContactTransaction,
    domain: usize,
    observations: Option<&incidence_view::ObservedIncidences<'_>>,
) -> Vec<ObservedContactVertex> {
    observations
        .and_then(|view| domain_axes(retained, domain).map(|axes| view.rows(axes)))
        .unwrap_or_default()
}
#[cfg(test)]
pub(super) fn observed_domain_vertices(
    retained: &PairedContactTransaction,
    domain_index: usize,
) -> Vec<ObservedContactVertex> {
    observed_domain_vertices_with_index(retained, domain_index, None)
}
#[cfg(test)]
pub(super) fn observed_domain_vertices_with_index(
    retained: &PairedContactTransaction,
    domain_index: usize,
    sources: Option<&SourceIndex>,
) -> Vec<ObservedContactVertex> {
    let PartitionObservation::Captured(snapshot) = &retained.mapped.arrangement.partition else {
        return Vec::new();
    };
    let Some(Ok(current)) = &retained.current_partition else {
        return Vec::new();
    };
    let Some(endpoints) = &current.endpoints else {
        return Vec::new();
    };
    endpoints
        .endpoints
        .iter()
        .flat_map(|endpoint| {
            observed_endpoint_rows(retained, domain_index, snapshot, endpoint, sources)
        })
        .collect()
}
#[cfg(test)]
fn observed_endpoint_rows(
    retained: &PairedContactTransaction,
    domain_index: usize,
    snapshot: &crate::sketch_region::OwnedPartitionSnapshot,
    endpoint: &PartitionEndpointBinding,
    sources: Option<&SourceIndex>,
) -> Vec<ObservedContactVertex> {
    let Some(vertex) = snapshot
        .vertices
        .iter()
        .find(|row| row.shape_index == endpoint.vertex_index)
    else {
        return Vec::new();
    };
    parameter_rows(retained, domain_index, endpoint, vertex, sources)
}

pub(super) fn canonical_occurrences(
    retained: &PairedContactTransaction,
    contact: usize,
) -> Option<[usize; 2]> {
    let row = &retained.inspection.contacts[contact];
    let domain_index = *row.occurrence_domains.first()?;
    domain_axes(retained, domain_index)
}
pub(super) fn domain_axes(
    retained: &PairedContactTransaction,
    domain_index: usize,
) -> Option<[usize; 2]> {
    let domain = &retained.census.domains[domain_index];
    let source_slots = match domain.evidence {
        crate::sketch_region_certification::DomainEvidence::PairedReport {
            source_slots, ..
        }
        | crate::sketch_region_certification::DomainEvidence::BernsteinReport {
            source_slots,
            ..
        } => source_slots,
        _ => return None,
    };
    let mut axes = [0; 2];
    for (position, axis) in source_slots.into_iter().enumerate() {
        axes[axis] = domain.occurrences[position];
    }
    Some(axes)
}
#[cfg(test)]
fn parameter_rows(
    retained: &PairedContactTransaction,
    domain_index: usize,
    endpoint: &PartitionEndpointBinding,
    vertex: &crate::sketch_region::PartitionVertexObservation,
    sources: Option<&SourceIndex>,
) -> Vec<ObservedContactVertex> {
    let Some(axes) = domain_axes(retained, domain_index) else {
        return Vec::new();
    };
    let parameters =
        axes.map(|occurrence| native_axis_parameters(retained, occurrence, endpoint, sources));
    distinct_parameter_rows(axes, &parameters, vertex)
}
#[cfg(test)]
fn distinct_parameter_rows(
    axes: [usize; 2],
    parameters: &[Vec<f64>; 2],
    vertex: &crate::sketch_region::PartitionVertexObservation,
) -> Vec<ObservedContactVertex> {
    distinct_parameter_rows_observed(axes, [&parameters[0], &parameters[1]], vertex, &mut |_| {})
}
fn distinct_parameter_rows_observed(
    axes: [usize; 2],
    parameters: [&[f64]; 2],
    vertex: &crate::sketch_region::PartitionVertexObservation,
    observe: &mut impl FnMut(incidence_view::DiscoveryStep),
) -> Vec<ObservedContactVertex> {
    let mut rows = Vec::new();
    for first in parameters[0] {
        for second in parameters[1] {
            observe(incidence_view::DiscoveryStep::ParameterProduct);
            rows.extend(distinct_parameter_row(axes, [*first, *second], vertex));
        }
    }
    rows
}
fn distinct_parameter_row(
    axes: [usize; 2],
    parameters: [f64; 2],
    vertex: &crate::sketch_region::PartitionVertexObservation,
) -> Option<ObservedContactVertex> {
    if axes[0] == axes[1] && parameters[0].to_bits() == parameters[1].to_bits() {
        return None;
    }
    Some(observe_vertex(vertex, parameters))
}

fn native_maps(
    retained: &PairedContactTransaction,
    axes: [usize; 2],
) -> Option<[MetricParameterMap; 2]> {
    let mapped: Vec<_> = axes
        .into_iter()
        .map(|index| {
            let id = retained.census.inputs.segments[index].occurrence;
            let row = retained
                .census
                .inputs
                .constructions
                .iter()
                .find(|row| row.occurrence == id)?;
            crate::sketch_region_source_conformance::captured_map(row).ok()
        })
        .collect::<Option<_>>()?;
    Some([mapped[0], mapped[1]])
}
#[cfg(test)]
fn native_axis_parameters(
    retained: &PairedContactTransaction,
    occurrence: usize,
    endpoint: &PartitionEndpointBinding,
    sources: Option<&SourceIndex>,
) -> Vec<f64> {
    let PartitionObservation::Captured(snapshot) = &retained.mapped.arrangement.partition else {
        return Vec::new();
    };
    let mut parameters = Vec::new();
    for (row, source) in snapshot.sources.iter().enumerate() {
        if matches_row(retained, occurrence, row, source, sources) {
            append_source_parameters(source.source_index, endpoint, &mut parameters);
        }
    }
    parameters
}
#[cfg(test)]
fn append_source_parameters(
    source_index: i32,
    endpoint: &PartitionEndpointBinding,
    parameters: &mut Vec<f64>,
) {
    for row in endpoint
        .incidences
        .iter()
        .filter(|row| row.source_index == source_index)
    {
        if !parameters
            .iter()
            .any(|value| value.to_bits() == row.parameter.to_bits())
        {
            parameters.push(row.parameter);
        }
    }
}

fn source_matches_occurrence(
    retained: &PairedContactTransaction,
    occurrence: usize,
    source: &SourcePartitionObservation,
) -> bool {
    let parents = &retained.census.inputs.segments[occurrence].parents;
    let Some(Ok(current)) = &retained.current_partition else {
        return false;
    };
    source_matches_parents(parents, source, current)
}
fn source_matches_parents(
    parents: &[crate::sketch_region::OriginalMemberIdentity],
    source: &SourcePartitionObservation,
    current: &crate::sketch_region_partition_bindings::PartitionTransactionInspection,
) -> bool {
    !parents.is_empty()
        && parents.len() == source.parents.len()
        && parents.iter().all(|parent| source.parents.contains(parent))
        && source.parents.iter().all(|parent| {
            current.domains.iter().any(|row| {
                row.source_index == source.source_index
                    && row.parent.as_ref() == Some(parent)
                    && row.decision.is_ok()
            })
        })
}
fn observe_vertex(
    vertex: &crate::sketch_region::PartitionVertexObservation,
    native_parameters: [f64; 2],
) -> ObservedContactVertex {
    ObservedContactVertex {
        vertex_index: vertex.shape_index,
        occurrence: vertex.occurrence.clone(),
        native_parameters,
        original_parents: vertex.original_parents.clone(),
        native_point: vertex.point,
        generated_observations: Vec::new(),
        parameter_measurement: None,
    }
}

pub(super) fn retain_parameter_measurements(retained: &mut PairedContactTransaction) {
    retain_parameter_measurements_with_access(
        retained,
        crate::sketch_region_certification::SourceAccess::Live,
    );
}
pub(super) fn retain_parameter_measurements_with_access(
    retained: &mut PairedContactTransaction,
    access: crate::sketch_region_certification::SourceAccess<'_>,
) {
    let proofs = access.owned_certificates(&retained.census);
    for contact in 0..retained.inspection.contacts.len() {
        let measurements = retained.inspection.contacts[contact]
            .observed_vertices
            .iter()
            .map(|vertex| {
                measure_parameters(retained, contact, vertex.native_parameters, proofs.as_ref())
            })
            .collect::<Vec<_>>();
        for (vertex, measurement) in retained.inspection.contacts[contact]
            .observed_vertices
            .iter_mut()
            .zip(measurements)
        {
            vertex.parameter_measurement = measurement;
        }
    }
}
fn measure_parameters(
    retained: &PairedContactTransaction,
    contact: usize,
    native: [f64; 2],
    proofs: Option<&crate::sketch_region_certification::DomainCertificates<'_>>,
) -> Option<NativeParameterMeasurement> {
    let axes = canonical_occurrences(retained, contact)?;
    let maps = native_maps(retained, axes)?;
    let row = &retained.inspection.contacts[contact];
    let domain = &retained.census.domains[row.occurrence_domains[0]];
    let proof = crate::sketch_region_certification::unified_certificate_in(
        &retained.census,
        domain,
        proofs,
    )?;
    let index = (0..proof.contact_count())
        .find(|&i| proof.contact(i).unwrap().key() == row.identity.proof_key)?;
    Some(proof.measure_native_parameters(
        index,
        maps,
        native,
        retained.requested.native_parameter_limits,
    ))
}

pub(super) fn original_source_endpoint(
    retained: &PairedContactTransaction,
    occurrence: usize,
    vertex: i32,
    parameter: f64,
) -> bool {
    original_source_endpoint_with_index(retained, occurrence, vertex, parameter, None)
}
pub(super) fn original_source_endpoint_with_index(
    retained: &PairedContactTransaction,
    occurrence: usize,
    vertex: i32,
    parameter: f64,
    sources: Option<&SourceIndex>,
) -> bool {
    let PartitionObservation::Captured(snapshot) = &retained.mapped.arrangement.partition else {
        return false;
    };
    snapshot.sources.iter().enumerate().any(|(row, source)| {
        matches_row(retained, occurrence, row, source, sources)
            && source.source_vertices.iter().any(|child| {
                child.shape_index == vertex && child.parameter.to_bits() == parameter.to_bits()
            })
    })
}

pub(super) fn only_axis_incidences(
    retained: &PairedContactTransaction,
    axes: [usize; 2],
    vertex: i32,
) -> bool {
    only_axis_incidences_with_index(retained, axes, vertex, None)
}
pub(super) fn only_axis_incidences_with_index(
    retained: &PairedContactTransaction,
    axes: [usize; 2],
    vertex: i32,
    sources: Option<&SourceIndex>,
) -> bool {
    unrelated_source_indices_with_index(retained, axes, vertex, sources)
        .is_some_and(|sources| sources.is_empty())
}
pub(super) fn unrelated_source_indices(
    retained: &PairedContactTransaction,
    axes: [usize; 2],
    vertex: i32,
) -> Option<Vec<i32>> {
    unrelated_source_indices_with_index(retained, axes, vertex, None)
}
pub(super) fn unrelated_source_indices_with_index(
    retained: &PairedContactTransaction,
    axes: [usize; 2],
    vertex: i32,
    sources: Option<&SourceIndex>,
) -> Option<Vec<i32>> {
    let PartitionObservation::Captured(snapshot) = &retained.mapped.arrangement.partition else {
        return None;
    };
    let endpoint = retained_endpoint(retained, vertex)?;
    let mut unrelated = Vec::new();
    for incidence in &endpoint.incidences {
        let admitted =
            source_index_admitted(retained, axes, snapshot, incidence.source_index, sources);
        if !admitted && !unrelated.contains(&incidence.source_index) {
            unrelated.push(incidence.source_index);
        }
    }
    Some(unrelated)
}
fn retained_endpoint(
    retained: &PairedContactTransaction,
    vertex: i32,
) -> Option<&PartitionEndpointBinding> {
    let current = retained.current_partition.as_ref()?.as_ref().ok()?;
    current
        .endpoints
        .as_ref()?
        .endpoints
        .iter()
        .find(|row| row.vertex_index == vertex)
}

fn source_index_admitted(
    retained: &PairedContactTransaction,
    axes: [usize; 2],
    snapshot: &crate::sketch_region::OwnedPartitionSnapshot,
    source_index: i32,
    sources: Option<&SourceIndex>,
) -> bool {
    snapshot.sources.iter().enumerate().any(|(row, source)| {
        source.source_index == source_index
            && axes
                .into_iter()
                .any(|axis| matches_row(retained, axis, row, source, sources))
    })
}

fn matches_row(
    retained: &PairedContactTransaction,
    occurrence: usize,
    row: usize,
    source: &SourcePartitionObservation,
    index: Option<&SourceIndex>,
) -> bool {
    match index {
        Some(index) => index
            .matching_source_rows(occurrence)
            .binary_search(&row)
            .is_ok(),
        None => source_matches_occurrence(retained, occurrence, source),
    }
}
