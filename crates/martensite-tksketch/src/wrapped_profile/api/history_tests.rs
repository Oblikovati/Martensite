//! Pure owned projection fixtures exercise source identity without constructing geometry.
use super::history_projection::{HistoryMapFailure, prepare, role};
use super::*;
use crate::surface_sketch::{HistoryRole, HistorySource, NormalSense};
use martensite_tkbo::profile_extrusion::{
    LoopUse, ProfileCurve, ProfileCurveId, ProfileLoop, ProfilePlane, ProfileVertex,
    ProfileVertexId, SketchProfile,
};
use std::collections::BTreeMap;
type Fixture = (
    Vec<BindingKey>,
    history::HistoryEvidence,
    BTreeMap<SurfaceRegionId, profile_input::CurveDirections>,
);

fn directions(reversed: bool) -> profile_input::CurveDirections {
    profile_input::from_profile(&direction_profile(), &direction_selection(reversed))
        .unwrap()
        .directions
}
fn direction_profile() -> SketchProfile {
    let vertices = [(10, [0., 0.]), (11, [1., 0.])]
        .into_iter()
        .map(|(id, point)| ProfileVertex {
            id: ProfileVertexId(id),
            point,
        })
        .collect();
    SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices,
        curves: vec![ProfileCurve::Line {
            id: ProfileCurveId(2),
            start: ProfileVertexId(10),
            end: ProfileVertexId(11),
        }],
    }
}
fn direction_selection(reversed: bool) -> profile_input::ProfileSelection {
    profile_input::ProfileSelection {
        region: SurfaceRegionId(1),
        outer: ProfileLoop {
            uses: vec![LoopUse {
                curve: ProfileCurveId(2),
                reversed,
            }],
        },
        holes: Vec::new(),
    }
}

fn fixture() -> Fixture {
    let keys = vec![split_key(1, 3), region_key(1), span_key(), vertex_key()];
    let records = keys.iter().copied().map(record).collect();
    (
        keys,
        history::HistoryEvidence { records },
        BTreeMap::from([(SurfaceRegionId(1), directions(true))]),
    )
}
fn region_key(id: u64) -> BindingKey {
    BindingKey {
        source: Source::Region(SurfaceRegionId(id)),
        role: HistoryRole::Solid,
    }
}
fn span_key() -> BindingKey {
    BindingKey {
        source: Source::Span(CurveSpan {
            curve: SurfaceCurveId(2),
            level: 3,
            index: 2,
        }),
        role: HistoryRole::FirstDepthEdge,
    }
}
fn vertex_key() -> BindingKey {
    BindingKey {
        source: Source::Corner(CornerSource::Authored(SurfaceVertexId(10))),
        role: HistoryRole::FirstDepthVertex,
    }
}
fn split_key(numerator: u32, level: u8) -> BindingKey {
    BindingKey {
        source: Source::Corner(CornerSource::Split {
            curve: SurfaceCurveId(2),
            numerator,
            level,
        }),
        role: HistoryRole::FirstDepthVertex,
    }
}
fn record(result: BindingKey) -> history::HistoryRecord {
    let relation = match result.source {
        Source::Span(_) => history::HistoryRelation::Split,
        Source::Corner(CornerSource::Split { .. }) => history::HistoryRelation::Created,
        _ => history::HistoryRelation::Derived,
    };
    history::HistoryRecord {
        result,
        owner: SurfaceRegionId(1),
        relation,
        parents: vec![
            HistorySource::Region(SurfaceRegionId(1)),
            HistorySource::Curve(SurfaceCurveId(2)),
        ],
    }
}

#[test]
fn public_sources_use_original_parameters_and_keep_native_binding_correspondence() {
    let (keys, original, directions) = fixture();
    let projected = prepare(&keys, &original, &directions, NormalSense::Inward).unwrap();
    assert_eq!(projected.records.len(), 4);
    assert_eq!(projected.bindings.len(), keys.len());
    assert_sources(&projected.bindings);
    assert_records(&projected.records, &original.records);
    fn send_sync<T: Send + Sync>() {}
    send_sync::<history_projection::HistoryPublication>();
}
fn assert_sources(keys: &[WrappedHistoryKey]) {
    assert_split_source(keys[0]);
    assert_eq!(
        keys[1].source,
        WrappedEntitySource::Region(SurfaceRegionId(1))
    );
    assert_curve_source(keys[2]);
    assert_eq!(
        keys[3].source,
        WrappedEntitySource::Vertex(SurfaceVertexId(10))
    );
}
fn assert_split_source(key: WrappedHistoryKey) {
    let point = WrappedSourcePoint {
        level: 3,
        numerator: 7,
    };
    assert_eq!(
        key.source,
        WrappedEntitySource::SplitVertex {
            curve: SurfaceCurveId(2),
            point
        }
    );
    assert_eq!(key.role, HistoryRole::LastDepthVertex);
}
fn assert_curve_source(key: WrappedHistoryKey) {
    let span = WrappedSourceSpan {
        level: 3,
        index: 5,
        reversed: true,
    };
    assert_eq!(
        key.source,
        WrappedEntitySource::Curve {
            curve: SurfaceCurveId(2),
            span
        }
    );
    assert_eq!(key.role, HistoryRole::LastDepthEdge);
}

fn assert_records(actual: &[WrappedHistoryRecord], original: &[history::HistoryRecord]) {
    assert!(
        actual
            .windows(2)
            .all(|pair| pair[0].result < pair[1].result)
    );
    assert!(
        actual
            .iter()
            .all(|r| r.owner == SurfaceRegionId(1) && r.parents == original[0].parents)
    );
    assert_eq!(actual[0].relation, WrappedHistoryRelation::Derived);
    assert_eq!(actual[1].relation, WrappedHistoryRelation::Split);
    assert_eq!(actual[2].relation, WrappedHistoryRelation::Derived);
    assert_eq!(actual[3].relation, WrappedHistoryRelation::Created);
}

#[test]
fn first_last_roles_follow_the_requested_normal_sense() {
    for (first, last) in [
        (HistoryRole::FirstDepthFace, HistoryRole::LastDepthFace),
        (HistoryRole::FirstDepthEdge, HistoryRole::LastDepthEdge),
        (HistoryRole::FirstDepthVertex, HistoryRole::LastDepthVertex),
    ] {
        assert_eq!(role(first, NormalSense::Outward), first);
        assert_eq!(role(last, NormalSense::Outward), last);
        assert_eq!(role(first, NormalSense::Inward), last);
        assert_eq!(role(last, NormalSense::Inward), first);
    }
    for unchanged in [
        HistoryRole::Solid,
        HistoryRole::SideFace,
        HistoryRole::NormalEdge,
    ] {
        assert_eq!(role(unchanged, NormalSense::Inward), unchanged);
    }
}

#[test]
fn missing_owner_and_curve_mapping_refuse_publication() {
    let (keys, mut records, directions) = fixture();
    assert!(matches!(
        prepare(&keys, &records, &BTreeMap::new(), NormalSense::Outward),
        Err(HistoryMapFailure::Owner(_))
    ));
    records.records[2].result.source = Source::Span(CurveSpan {
        curve: SurfaceCurveId(99),
        level: 0,
        index: 0,
    });
    assert!(matches!(
        prepare(&keys, &records, &directions, NormalSense::Outward),
        Err(HistoryMapFailure::Parameter { .. })
    ));
}

#[test]
fn duplicate_or_missing_frozen_targets_cannot_pass_on_counts() {
    let (mut keys, records, directions) = fixture();
    keys[0] = keys[1];
    assert!(matches!(
        prepare(&keys, &records, &directions, NormalSense::Outward),
        Err(HistoryMapFailure::Count { .. })
    ));
    keys[0] = region_key(99);
    assert!(matches!(
        prepare(&keys, &records, &directions, NormalSense::Outward),
        Err(HistoryMapFailure::Binding { count: 0, .. })
    ));
}

#[test]
fn noncanonical_point_aliases_cannot_create_duplicate_public_keys() {
    let (keys, mut records, directions) = fixture();
    records.records.push(record(split_key(2, 4)));
    assert!(matches!(
        prepare(&keys, &records, &directions, NormalSense::Outward),
        Err(HistoryMapFailure::Duplicate(_))
    ));
}

#[test]
fn a_private_binding_cannot_map_to_two_different_public_identities() {
    let (keys, mut records, mut directions) = fixture();
    let mut duplicate = records.records[2].clone();
    duplicate.owner = SurfaceRegionId(2);
    records.records.push(duplicate);
    directions.insert(SurfaceRegionId(2), self::directions(false));
    assert!(matches!(
        prepare(&keys, &records, &directions, NormalSense::Outward),
        Err(HistoryMapFailure::Binding { count: 2, .. })
    ));
}
