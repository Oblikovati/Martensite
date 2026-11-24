//! History controls use authored incidence and exact intervals as independent expectations.
use super::super::tests::request;
use super::*;
fn fixture() -> (Vec<Request>, selection::QualifiedSelection, Vec<BindingKey>) {
    let originals = vec![request()];
    let selection = selection::certify(&originals, Default::default()).unwrap();
    let built = native::construct(&selection.prepared.prepared[0]).unwrap();
    let keys = built.bindings.iter().map(|(key, _)| *key).collect();
    (originals, selection, keys)
}
fn record(proof: &HistoryEvidence, source: Source, role: HistoryRole) -> &HistoryRecord {
    proof
        .records
        .iter()
        .find(|r| r.result == BindingKey { source, role })
        .unwrap()
}
#[test]
fn original_corner_has_both_incident_curves_and_no_traversal_parent() {
    let (originals, selection, keys) = fixture();
    let proof = certify(&originals, &selection, &keys).unwrap();
    let source = Source::Corner(CornerSource::Authored(SurfaceVertexId(10)));
    let actual = record(&proof, source, HistoryRole::NormalEdge);
    assert_eq!(
        actual.parents,
        [
            HistorySource::Region(SurfaceRegionId(5)),
            HistorySource::Curve(SurfaceCurveId(10)),
            HistorySource::Curve(SurfaceCurveId(13)),
            HistorySource::Vertex(SurfaceVertexId(10))
        ]
    );
    assert_eq!(actual.relation, HistoryRelation::Derived);
    assert_eq!(actual.owner, SurfaceRegionId(5));
    assert_eq!(proof.records.len(), keys.len());
}
#[test]
fn split_corner_is_created_from_its_curve_and_region() {
    let (originals, selection, keys) = fixture();
    let proof = certify(&originals, &selection, &keys).unwrap();
    let actual = first_split_record(&proof);
    let curve = split_parent(actual.result.source);
    assert_eq!(
        actual.parents,
        [
            HistorySource::Region(SurfaceRegionId(5)),
            HistorySource::Curve(curve)
        ]
    );
    assert_eq!(actual.relation, HistoryRelation::Created);
    assert!(
        proof
            .records
            .iter()
            .any(|r| r.relation == HistoryRelation::Split)
    );
}
#[test]
fn region_images_include_every_authored_curve_and_vertex() {
    let (originals, selection, keys) = fixture();
    let proof = certify(&originals, &selection, &keys).unwrap();
    let expected: Vec<_> = std::iter::once(HistorySource::Region(SurfaceRegionId(5)))
        .chain([10, 11, 12, 13, 20, 21, 22, 23].map(|n| HistorySource::Curve(SurfaceCurveId(n))))
        .chain([10, 11, 12, 13, 20, 21, 22, 23].map(|n| HistorySource::Vertex(SurfaceVertexId(n))))
        .collect();
    for role in [
        HistoryRole::Solid,
        HistoryRole::FirstDepthFace,
        HistoryRole::LastDepthFace,
    ] {
        assert_eq!(
            record(&proof, Source::Region(SurfaceRegionId(5)), role).parents,
            expected
        );
    }
}
#[test]
fn missing_extra_and_duplicate_frozen_targets_are_refused() {
    let (originals, selection, mut keys) = fixture();
    let removed = keys.pop().unwrap();
    assert!(matches!(
        certify(&originals, &selection, &keys),
        Err(HistoryFailure::Targets { .. })
    ));
    keys.push(removed);
    keys.push(keys[0]);
    assert!(matches!(
        certify(&originals, &selection, &keys),
        Err(HistoryFailure::DuplicateTarget(_))
    ));
    keys.pop();
    keys[0].source = Source::Region(SurfaceRegionId(999));
    assert!(matches!(
        certify(&originals, &selection, &keys),
        Err(HistoryFailure::Targets { .. })
    ));
}
#[test]
fn original_geometry_and_prepared_corner_relabelling_are_refused() {
    let (mut originals, mut selection, keys) = fixture();
    originals[0].loops[0][0].poles[1][0] += 0.01;
    assert!(matches!(
        certify(&originals, &selection, &keys),
        Err(HistoryFailure::SourceBinding)
    ));
    originals[0] = request();
    selection.prepared.prepared[0].loops[0][0].start = CornerSource::Authored(SurfaceVertexId(999));
    assert!(matches!(
        certify(&originals, &selection, &keys),
        Err(HistoryFailure::Corners { .. })
    ));
}
#[test]
fn semantic_history_ignores_frozen_reference_order_and_loop_start() {
    let (originals, mut selection, mut keys) = fixture();
    let first = certify(&originals, &selection, &keys).unwrap();
    keys.reverse();
    for contour in &mut selection.prepared.prepared[0].loops {
        contour.rotate_left(1);
    }
    // Reorder boundary receipts with the prepared intervals before testing source/key independence.
    selection.prepared.boundary_clearance =
        planar_clearance::certify_many(&originals, &selection.prepared.prepared).unwrap();
    assert_eq!(certify(&originals, &selection, &keys).unwrap(), first);
    fn send_sync<T: Send + Sync>() {}
    send_sync::<HistoryEvidence>();
}

fn split_parent(source: Source) -> SurfaceCurveId {
    let Source::Corner(CornerSource::Split { curve, .. }) = source else {
        panic!("expected split source")
    };
    curve
}
#[test]
fn equivalent_noncanonical_split_labels_do_not_change_persistent_identity() {
    let (originals, mut selection, keys) = fixture();
    let pieces = &mut selection.prepared.prepared[0].loops[0];
    let old = first_split_corner(pieces);
    let alias = noncanonical_alias(old);
    for piece in pieces {
        if piece.start == old {
            piece.start = alias;
        }
        if piece.end == old {
            piece.end = alias;
        }
    }
    assert!(matches!(
        certify(&originals, &selection, &keys),
        Err(HistoryFailure::Corners { .. })
    ));
}

fn first_split_record(proof: &HistoryEvidence) -> &HistoryRecord {
    proof
        .records
        .iter()
        .find(|r| matches!(r.result.source, Source::Corner(CornerSource::Split { .. })))
        .unwrap()
}

fn first_split_corner(pieces: &[AuthoredPiece]) -> CornerSource {
    pieces
        .iter()
        .find_map(|p| match p.end {
            CornerSource::Split { .. } => Some(p.end),
            _ => None,
        })
        .unwrap()
}
fn noncanonical_alias(source: CornerSource) -> CornerSource {
    let CornerSource::Split {
        curve,
        numerator,
        level,
    } = source
    else {
        panic!("split")
    };
    CornerSource::Split {
        curve,
        numerator: numerator * 2,
        level: level + 1,
    }
}

#[test]
fn developed_history_borrows_distinct_owners_and_retains_cylinder_records() {
    let (originals, selection, keys) = fixture();
    let expected = certify(&originals, &selection, &keys).unwrap();
    let originals: Vec<_> = originals.into_iter().map(|r| r.source).collect();
    let prepared: Vec<_> = selection
        .prepared
        .prepared
        .into_iter()
        .map(|r| DevelopedHistoryOwner(r.source))
        .collect();
    assert_eq!(
        certify_developed(&originals, &prepared, &keys).unwrap(),
        expected
    );
}

#[test]
fn developed_history_refuses_a_gap_in_the_original_parameter_cover() {
    let (originals, mut prepared, keys) = developed_fixture();
    prepared[0].loops[0].pop();
    assert!(matches!(
        certify_developed(&originals, &prepared, &keys),
        Err(HistoryFailure::SourceBinding)
    ));
}

#[test]
fn developed_history_refuses_duplicate_parameter_intervals() {
    let (originals, mut prepared, keys) = developed_fixture();
    let duplicate = prepared[0].loops[0][0].clone();
    prepared[0].loops[0].push(duplicate);
    assert!(matches!(
        certify_developed(&originals, &prepared, &keys),
        Err(HistoryFailure::SourceBinding)
    ));
}

#[test]
fn developed_history_refuses_changed_or_missing_region_owners() {
    let (originals, mut prepared, keys) = developed_fixture();
    prepared[0].region = SurfaceRegionId(999);
    assert!(matches!(
        certify_developed(&originals, &prepared, &keys),
        Err(HistoryFailure::SourceBinding)
    ));
    prepared.clear();
    assert!(matches!(
        certify_developed(&originals, &prepared, &keys),
        Err(HistoryFailure::SourceBinding)
    ));
}

#[test]
fn developed_history_refuses_relabelled_corners_and_duplicate_targets() {
    let (originals, mut prepared, mut keys) = developed_fixture();
    keys.push(keys[0]);
    assert!(matches!(
        certify_developed(&originals, &prepared, &keys),
        Err(HistoryFailure::DuplicateTarget(_))
    ));
    keys.pop();
    prepared[0].loops[0][0].start = CornerSource::Authored(SurfaceVertexId(999));
    assert!(matches!(
        certify_developed(&originals, &prepared, &keys),
        Err(HistoryFailure::Corners { .. })
    ));
}

fn developed_fixture() -> (
    Vec<DevelopedRegionRequest>,
    Vec<DevelopedRegionRequest>,
    Vec<BindingKey>,
) {
    let (originals, selection, keys) = fixture();
    let originals = originals.into_iter().map(|r| r.source).collect();
    let prepared = selection
        .prepared
        .prepared
        .into_iter()
        .map(|r| r.source)
        .collect();
    (originals, prepared, keys)
}

struct DevelopedHistoryOwner(DevelopedRegionRequest);
impl AsRef<DevelopedRegionRequest> for DevelopedHistoryOwner {
    fn as_ref(&self) -> &DevelopedRegionRequest {
        &self.0
    }
}
