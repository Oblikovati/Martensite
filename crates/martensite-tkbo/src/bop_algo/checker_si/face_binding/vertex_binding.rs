// SPDX-License-Identifier: GPL-2.0-only
//! Endpoint binding is one owned-packet obligation, not support injectivity or coherent source capture.
use super::{types::*, vertex_metrics, vertex_topology};

pub(super) fn inspect(
    packet: &OwnedFacePacket,
    strip: &Result<StripPrerequisites, BindingFailure>,
    cancelled: &mut impl FnMut() -> bool,
) -> VertexBindingObservation {
    if strip.is_err() {
        return VertexBindingObservation::NotAttempted;
    }
    match check(packet, cancelled) {
        Ok(proof) => VertexBindingObservation::Bound(proof),
        Err(cause) => VertexBindingObservation::Incomplete(cause),
    }
}
fn check(
    packet: &OwnedFacePacket,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<OwnedVertexBinding, BindingFailure> {
    let endpoint_uses = vertex_topology::check(packet, cancelled)?;
    let mut endpoints = Vec::new();
    for endpoint in &endpoint_uses {
        endpoints.push(vertex_metrics::check(packet, endpoint, cancelled)?);
    }
    Ok(OwnedVertexBinding {
        endpoints,
        identity_pairs_checked: packet.vertex_relations.len(),
    })
}
