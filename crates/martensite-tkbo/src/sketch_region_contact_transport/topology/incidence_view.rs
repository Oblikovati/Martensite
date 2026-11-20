// SPDX-License-Identifier: GPL-2.0-only
//! #566: one immutable native-incidence discovery serves every report in a phase.
use super::*;
use crate::sketch_region::{OwnedPartitionSnapshot, PartitionVertexObservation};
use crate::sketch_region_candidates::CandidateSegment;
use crate::sketch_region_certification::BoundaryInputs;
use crate::sketch_region_partition_bindings::PartitionTransactionInspection;
use std::collections::{BTreeMap, BTreeSet};
mod capture;
mod query;
mod sources;
#[cfg(test)]
mod tests;
#[derive(Clone, Copy)]
pub(in crate::sketch_region_contact_transport) enum DiscoveryStep {
    SourceMembership,
    Endpoint,
    VertexEntry,
    VertexLookup,
    Incidence,
    ParameterCandidate,
    ParameterProduct,
    VertexDedup,
    ReportMembership,
}
pub(in crate::sketch_region_contact_transport) struct ObservedIncidences<'a> {
    axes: Vec<Vec<AxisEndpoint<'a>>>,
}
struct AxisEndpoint<'a> {
    // Row position joins two views of one binding; native identity remains vertex.shape_index.
    endpoint: usize,
    vertex: &'a PartitionVertexObservation,
    parameters: Vec<f64>,
}
pub(in crate::sketch_region_contact_transport) fn compile<'a>(
    inputs: &'a BoundaryInputs,
    partition: &'a PartitionObservation,
    current: Option<&'a PartitionTransactionInspection>,
    index: Option<&SourceIndex>,
) -> Option<ObservedIncidences<'a>> {
    compile_observed(inputs, partition, current, index, &mut |_| {})
}
pub(in crate::sketch_region_contact_transport) fn compile_observed<'a>(
    inputs: &'a BoundaryInputs,
    partition: &'a PartitionObservation,
    current: Option<&'a PartitionTransactionInspection>,
    index: Option<&SourceIndex>,
    observe: &mut impl FnMut(DiscoveryStep),
) -> Option<ObservedIncidences<'a>> {
    let PartitionObservation::Captured(snapshot) = partition else {
        return None;
    };
    let current = current?;
    let endpoints = &current.endpoints.as_ref()?.endpoints;
    let bindings = sources::capture(&inputs.segments, &snapshot.sources, current, index, observe);
    Some(capture::capture(
        inputs.segments.len(),
        snapshot,
        endpoints,
        &bindings,
        observe,
    ))
}
impl ObservedIncidences<'_> {
    pub(super) fn rows(&self, axes: [usize; 2]) -> Vec<ObservedContactVertex> {
        self.rows_observed(axes, &mut |_| {})
    }
    pub(super) fn rows_observed(
        &self,
        axes: [usize; 2],
        observe: &mut impl FnMut(DiscoveryStep),
    ) -> Vec<ObservedContactVertex> {
        let mut rows = Vec::new();
        for (first, second) in query::common(self, axes) {
            rows.extend(super::distinct_parameter_rows_observed(
                axes,
                [&first.parameters, &second.parameters],
                first.vertex,
                observe,
            ));
        }
        rows
    }
    pub(in crate::sketch_region_contact_transport) fn cardinality(
        &self,
        axes: [usize; 2],
        observe: &mut impl FnMut(DiscoveryStep),
    ) -> usize {
        let mut vertices = BTreeSet::new();
        for (first, _) in query::common(self, axes) {
            if axes[0] == axes[1] && first.parameters.len() < 2 {
                continue;
            }
            observe(DiscoveryStep::VertexDedup);
            vertices.insert(first.vertex.shape_index);
        }
        vertices.len()
    }
}
