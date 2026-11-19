// SPDX-License-Identifier: GPL-2.0-only
//! #555: fresh ownership and every issued raw source operand bind the private epoch.
use super::*;
pub(super) fn matches(owner: &OwnedGraphAccess<'_>, census: &BoundaryCensus) -> bool {
    let originals = &census.inputs.originals;
    originals.len() == owner.edges.len()
        && owner.edges.len() == owner.request.edges.len()
        && policy_matches(owner, census)
        && crate::sketch_region_certification::bind_graph_policy(census, owner.request.version)
            .is_ok()
        && owner
            .edges
            .iter()
            .zip(&owner.request.edges)
            .zip(originals)
            .all(|((edge, owned), original)| source_matches(edge, owned, original))
}
fn policy_matches(owner: &OwnedGraphAccess<'_>, census: &BoundaryCensus) -> bool {
    let requested = owner.request.policy;
    let Some(source) = census.source_policy else {
        return false;
    };
    census.requested.roots == requested.census.roots
        && census.requested.max_world_error.to_bits() == requested.census.max_world_error.to_bits()
        && source.roots == requested.source.roots
        && source.original_plane_limit.to_bits() == requested.source.original_plane_limit.to_bits()
        && source.construction_limit.to_bits() == requested.source.construction_limit.to_bits()
}
fn source_matches(
    edge: &ConstructedProfileEdge,
    owned: &OwnedProfileEdge,
    original: &crate::sketch_region_source_conformance::OriginalCurveCapture,
) -> bool {
    let Ok(native) = martensite_tkbrep::topods::edge(&edge.edge) else {
        return false;
    };
    owned.source.0 == edge.identity.0
        && original.original == crate::sketch_region::original_identity(edge.identity, &native)
        && original
            .representation
            .as_ref()
            .is_ok_and(|source| matches_curve(source, owned))
}
fn matches_curve(source: &CapturedSpaceRepresentation, owned: &OwnedProfileEdge) -> bool {
    let CapturedMetricCurve::Bezier { poles, weights } = &source.curve else {
        return false;
    };
    poles.iter().flatten().map(|v| v.to_bits()).eq(owned
        .world_poles
        .iter()
        .flatten()
        .map(|v| v.to_bits()))
        && poles.len() == owned.world_poles.len()
        && same_weights(weights.as_deref(), owned.world_weights.as_deref())
        && source.interval.map(f64::to_bits) == [0f64.to_bits(), 1f64.to_bits()]
        && identity_placement(&source.placement)
}
fn identity_placement(
    actual: &martensite_tkmath::predicates::roots::metric_bounds::MetricEmbedding<3>,
) -> bool {
    let identity = martensite_tkmath::predicates::roots::metric_bounds::MetricEmbedding {
        linear: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
        origin: [0.; 3],
        scale: 1.,
    };
    crate::sketch_region_source_conformance::same_embedding(actual, &identity)
}
fn same_weights(first: Option<&[f64]>, second: Option<&[f64]>) -> bool {
    match (first, second) {
        (None, None) => true,
        (Some(a), Some(b)) => a
            .iter()
            .map(|v| v.to_bits())
            .eq(b.iter().map(|v| v.to_bits())),
        _ => false,
    }
}
