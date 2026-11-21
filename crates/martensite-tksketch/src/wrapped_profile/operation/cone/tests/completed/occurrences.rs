//! Actual detached topology supplies orientation provenance independently of semantic wrappers.
use super::normals::{cap, children, faces, key, physical_cap_role};
use super::*;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{
    geom::{GeomConicalSurface, GeomElementarySurface},
    top_abs::Orientation,
};
pub(super) fn check(result: &WrappedConeNormalResult, detached: &DetachedWrappedBodies) {
    for face in faces(&detached.bodies) {
        let owner = key(detached, &face);
        if owner.role == HistoryRole::SideFace {
            assert_eq!(
                proof(result, owner).side_reversed,
                reversed(face.orientation())
            );
        }
        if cap(owner.role) {
            cap_edges(result, detached, &face, owner.role);
        }
    }
    bind_flux(result, detached);
}
fn cap_edges(
    result: &WrappedConeNormalResult,
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
                proof(result, key).cap_reversed[depth],
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
fn proof(result: &WrappedConeNormalResult, key: WrappedHistoryKey) -> &storage::SpanProof {
    let WrappedEntitySource::Curve { curve, span } = key.source else {
        panic!("expected source curve occurrence");
    };
    assert!(
        !span.reversed,
        "owned witness uses forward original curve declarations"
    );
    let geometry = &result.family.family.members[&SurfaceRegionId(1)].geometry;
    let proofs: Vec<_> = geometry
        .spans
        .iter()
        .filter(|proof| {
            proof.span.curve == curve
                && proof.span.level == span.level
                && u64::from(proof.span.index) == span.index
        })
        .collect();
    assert_eq!(proofs.len(), 1);
    proofs[0]
}
fn reversed(orientation: Orientation) -> bool {
    match orientation {
        Orientation::Forward => false,
        Orientation::Reversed => true,
        other => panic!("actual occurrence {other:?}"),
    }
}
fn bind_flux(result: &WrappedConeNormalResult, detached: &DetachedWrappedBodies) {
    let receipt = result.flux_receipt(SurfaceRegionId(1)).unwrap();
    let geometry = &result.family.family.members[&SurfaceRegionId(1)].geometry;
    let spans: Vec<_> = geometry
        .spans
        .iter()
        .map(storage::SpanProof::flux_span)
        .collect();
    let roles = [HistoryRole::FirstDepthFace, HistoryRole::LastDepthFace];
    let carriers =
        roles.map(|role| carrier(detached, physical_cap_role(role, result.support().normal)));
    let query = AnalyticConicalFluxQuery {
        carriers,
        spans: &spans,
        options: receipt.requested_options(),
        arithmetic: receipt.requested_arithmetic(),
    };
    assert!(receipt.matches_source(query));
}
fn carrier(detached: &DetachedWrappedBodies, role: HistoryRole) -> ConicalTrimCarrier {
    let key = WrappedHistoryKey {
        source: WrappedEntitySource::Region(SurfaceRegionId(1)),
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
