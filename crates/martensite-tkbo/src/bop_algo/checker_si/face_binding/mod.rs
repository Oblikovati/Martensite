// SPDX-License-Identifier: GPL-2.0-only
//! Owned bug25777 prerequisites; no face validity or completed self-search is returned.
mod capture;
mod capture_curves;
mod capture_edges;
mod capture_vertices;
mod conformance;
mod domain;
mod exact;
mod layout;
mod refine;
mod types;
mod vertex_binding;
mod vertex_metrics;
mod vertex_topology;
pub(crate) use capture::capture_face;
pub(crate) use types::*;
#[cfg(test)]
mod tests;

/// Inspect an owned strip without upgrading CheckerSI completion.
/// Example: `inspect_owned_strip(&packet, || false)` still requires a released support certificate.
pub(crate) fn inspect_owned_strip(
    packet: &OwnedFacePacket,
    mut cancelled: impl FnMut() -> bool,
) -> FaceBindingObservation {
    let prerequisites = domain::check(packet, &mut cancelled)
        .and_then(|roles| conformance::check(packet, roles, &mut cancelled));
    let vertex_binding = vertex_binding::inspect(packet, &prerequisites, &mut cancelled);
    let topology_observation = if matches!(&vertex_binding, VertexBindingObservation::Bound(_)) {
        TopologyObservation::OwnedVertexEndpointIncidenceBound
    } else {
        TopologyObservation::EdgeOccurrencesOnlyVerticesUnbound
    };
    FaceBindingObservation {
        vertex_binding,
        prerequisites,
        completion: BindingIncomplete::ReleasedSupportCertificateNotBound,
        source_atomicity: SourceAtomicity::Unestablished,
        topology_observation,
    }
}
