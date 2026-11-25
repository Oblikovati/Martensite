//! Actual detached topology supplies orientation provenance independently of semantic wrappers.
use super::normals::{cap, children, faces, key, physical_cap_role};
use super::*;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{
    geom::{GeomConicalSurface, GeomElementarySurface},
    top_abs::Orientation,
};
pub(super) fn check(result: &WrappedConeNormalResult, detached: &DetachedWrappedBodies) {
    check_requested(result, &super::super::request(), detached);
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone::tests) fn check_requested(
    result: &WrappedConeNormalResult,
    request: &WrappedConeNormalRequest,
    detached: &DetachedWrappedBodies,
) {
    for face in faces(&detached.bodies) {
        let owner = key(detached, &face);
        if owner.role == HistoryRole::SideFace {
            assert_eq!(
                proof(result, request, owner).side_reversed,
                reversed(face.orientation())
            );
        }
        if cap(owner.role) {
            cap_edges(result, request, detached, &face, owner.role);
        }
    }
    for region in result.reports().keys().copied() {
        bind_flux(result, detached, region);
    }
}
fn cap_edges(
    result: &WrappedConeNormalResult,
    request: &WrappedConeNormalRequest,
    detached: &DetachedWrappedBodies,
    face: &Shape,
    role: HistoryRole,
) {
    let role = physical_cap_role(role, result.support().normal);
    let depth = if role == HistoryRole::FirstDepthFace {
        0
    } else {
        1
    };
    for wire in children(face) {
        for edge in children(&wire) {
            let key = edge_key(detached, &edge);
            assert_eq!(
                proof(result, request, key).cap_reversed[depth],
                reversed(edge.orientation())
            );
        }
    }
}
fn edge_key(detached: &DetachedWrappedBodies, edge: &Shape) -> WrappedHistoryKey {
    let keys: Vec<_> = detached
        .entities
        .iter()
        .filter(|(key, shape)| {
            matches!(
                key.role,
                HistoryRole::FirstDepthEdge | HistoryRole::LastDepthEdge
            ) && shape.is_same(edge)
        })
        .map(|(key, _)| *key)
        .collect();
    assert_eq!(keys.len(), 1);
    keys[0]
}
fn proof<'a>(
    result: &'a WrappedConeNormalResult,
    request: &WrappedConeNormalRequest,
    key: WrappedHistoryKey,
) -> &'a storage::SpanProof {
    let WrappedEntitySource::Curve { curve, span } = key.source else {
        panic!("expected source curve occurrence");
    };
    assert_eq!(span.reversed, authored_direction(request, curve));
    let index = directed_index(span);
    let geometry = &result.family.family.members[&owner(result, key)].geometry;
    let proofs: Vec<_> = geometry
        .spans
        .iter()
        .filter(|proof| {
            proof.span.curve == curve
                && proof.span.level == span.level
                && u64::from(proof.span.index) == index
        })
        .collect();
    assert_eq!(proofs.len(), 1);
    proofs[0]
}
fn directed_index(span: WrappedSourceSpan) -> u64 {
    if span.reversed {
        (1_u64 << span.level) - span.index - 1
    } else {
        span.index
    }
}
fn authored_direction(request: &WrappedConeNormalRequest, curve: SurfaceCurveId) -> bool {
    let uses: Vec<_> = request
        .selections
        .iter()
        .flat_map(|selection| std::iter::once(&selection.outer).chain(&selection.holes))
        .flat_map(|contour| &contour.uses)
        .filter(|source| {
            source.curve == martensite_tkbo::profile_extrusion::ProfileCurveId(curve.0)
        })
        .collect();
    assert_eq!(uses.len(), 1);
    uses[0].reversed
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone::tests) fn owner(
    result: &WrappedConeNormalResult,
    key: WrappedHistoryKey,
) -> SurfaceRegionId {
    let records: Vec<_> = result
        .history()
        .iter()
        .filter(|record| record.result == key)
        .collect();
    assert_eq!(records.len(), 1);
    if let WrappedEntitySource::Region(region) = key.source {
        assert_eq!(records[0].owner, region);
    }
    records[0].owner
}
fn reversed(orientation: Orientation) -> bool {
    match orientation {
        Orientation::Forward => false,
        Orientation::Reversed => true,
        other => panic!("actual occurrence {other:?}"),
    }
}
fn bind_flux(
    result: &WrappedConeNormalResult,
    detached: &DetachedWrappedBodies,
    region: SurfaceRegionId,
) {
    let receipt = result.flux_receipt(region).unwrap();
    let geometry = &result.family.family.members[&region].geometry;
    let spans: Vec<_> = geometry
        .spans
        .iter()
        .map(storage::SpanProof::flux_span)
        .collect();
    let carriers = carriers(result, detached, region);
    let query = AnalyticConicalFluxQuery {
        carriers,
        spans: &spans,
        options: receipt.requested_options(),
        arithmetic: receipt.requested_arithmetic(),
    };
    assert!(receipt.matches_source(query));
}
fn carriers(
    result: &WrappedConeNormalResult,
    detached: &DetachedWrappedBodies,
    region: SurfaceRegionId,
) -> [ConicalTrimCarrier; 2] {
    let roles = [HistoryRole::FirstDepthFace, HistoryRole::LastDepthFace];
    roles.map(|role| {
        carrier(
            detached,
            region,
            physical_cap_role(role, result.support().normal),
        )
    })
}
fn carrier(
    detached: &DetachedWrappedBodies,
    region: SurfaceRegionId,
    role: HistoryRole,
) -> ConicalTrimCarrier {
    let key = WrappedHistoryKey {
        source: WrappedEntitySource::Region(region),
        role,
    };
    let face = martensite_tkbrep::topods::face(&detached.entities[&key]).unwrap();
    let (surface, location) =
        martensite_tkbrep::brep::BRepTool::surface_with_location(&face).unwrap();
    assert!(location.is_identity());
    let cone: Handle<GeomConicalSurface> = surface.downcast();
    let cone = cone.get().unwrap();
    ConicalTrimCarrier {
        radius: cone.ref_radius(),
        axial_origin: cone.position().location().z(),
        semi_angle: cone.semi_angle(),
    }
}
