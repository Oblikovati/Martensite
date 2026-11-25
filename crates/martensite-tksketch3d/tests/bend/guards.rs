// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::CardinalBendFailure;
use martensite_tksketch3d::{
    EditFailure, FrozenPointSource, FrozenSourceRevision, SketchPoint,
    constraints::{ConstrainedSketch, Relation, RelationId},
};

#[test]
fn existing_unselected_incidence_and_frozen_corner_refuse_without_rewiring() {
    let shared = fixture(CAPTURED)
        .apply(
            &[seed(7, [60., 0., 12.], PointRole::Defining), line(9, 2, 7)],
            || false,
        )
        .unwrap();
    assert!(matches!(shared.bend_lines(bend(5.), || false),
        Err(BendFailure::ProtectedCorner { corner, protection: EndpointProtection::Shared(EntityId::Line(other)) }) if corner == pid(2) && other == lid(9)));
    assert_eq!(
        shared.line(lid(9)).unwrap().line.endpoints,
        [pid(2), pid(7)]
    );
    let frozen_source = FrozenPointSource {
        document: [7; 16],
        sketch: NonZeroU64::new(10).unwrap(),
        revision: FrozenSourceRevision::SessionGeneration { generation: 14 },
        point: pid(22),
    };
    let frozen = Snapshot::empty(MinimumLength::new(0.01).unwrap())
        .apply(
            &[
                seed(1, CAPTURED[0], PointRole::Defining),
                Edit::InsertFrozenPoint {
                    point: SketchPoint {
                        id: pid(2),
                        position: point(CAPTURED[1]),
                        role: PointRole::Defining,
                    },
                    source: frozen_source,
                },
                seed(3, CAPTURED[2], PointRole::Defining),
                line(4, 1, 2),
                line(5, 2, 3),
            ],
            || false,
        )
        .unwrap();
    assert!(matches!(frozen.bend_lines(bend(5.), || false),
        Err(BendFailure::ProtectedCorner { corner, protection: EndpointProtection::Frozen(actual) }) if corner == pid(2) && actual == frozen_source));
    assert_eq!(frozen.frozen_source(pid(2)), Some(frozen_source));
}

#[test]
fn geometry_role_and_identity_limits_are_explicit() {
    let source = fixture(CAPTURED);
    let noncardinal = fixture([CAPTURED[0], CAPTURED[1], [59., 30., 0.]]);
    assert!(matches!(
        noncardinal.bend_lines(bend(5.), || false),
        Err(BendFailure::Placement(CardinalBendFailure::NonCardinalLeg(
            1
        )))
    ));
    assert!(matches!(
        source.bend_lines(bend(30.), || false),
        Err(BendFailure::Placement(
            CardinalBendFailure::RadiusReachesEnd(0, 30.)
        ))
    ));
    let mut request = bend(5.);
    request.ids.center = pid(1);
    assert!(matches!(
        source.bend_lines(request, || false),
        Err(BendFailure::Geometry(EditFailure::DuplicateIdentity { .. }))
    ));
    let construction = |id| Edit::SetCurveRole {
        curve: CurveId::Line(id),
        role: CurveRole::Construction,
    };
    let mixed = source.apply(&[construction(lid(4))], || false).unwrap();
    assert!(matches!(
        mixed.bend_lines(bend(5.), || false),
        Err(BendFailure::MixedParticipation(_))
    ));
    let agreed = mixed.apply(&[construction(lid(5))], || false).unwrap();
    let result = agreed.bend_lines(bend(5.), || false).unwrap();
    let ids = result.request.ids;
    for curve in [
        CurveId::Line(ids.lines[0]),
        CurveId::Line(ids.lines[1]),
        CurveId::Arc(ids.arc),
    ] {
        assert_eq!(
            result.sketch.curve_role(curve),
            Some(CurveRole::Construction)
        );
    }
    assert!(source.arc(ids.arc).is_none());
}

#[test]
fn successive_commits_keep_the_first_bend_and_the_original_value_for_replay() {
    // bend.feature:46 uses a separate connector and processes both corners in order.
    let source = fixture(CAPTURED)
        .apply(
            &[seed(7, [30., 30., 0.], PointRole::Defining), line(9, 3, 7)],
            || false,
        )
        .unwrap();
    let first = source.bend_lines(bend(5.), || false).unwrap();
    let connector = first.request.ids.lines[1];
    let next = BendRequest {
        lines: [connector, lid(9)],
        corner: pid(3),
        ids: ids(101),
        ..bend(5.)
    };
    let second = first.sketch.bend_lines(next, || false).unwrap();
    assert_eq!(
        second.sketch.arc(first.request.ids.arc),
        first.sketch.arc(first.request.ids.arc)
    );
    assert_eq!(
        second.sketch.arc(next.ids.arc).unwrap().points(),
        [[60., 25., 0.], [55., 30., 0.], [55., 25., 0.]].map(point)
    );
    assert!(first.sketch.arc(next.ids.arc).is_none());
    assert!(source.line(lid(5)).is_some());
    assert_eq!(first, source.bend_lines(bend(5.), || false).unwrap());
    assert_eq!(second, first.sketch.bend_lines(next, || false).unwrap());
}

#[test]
fn isolated_value_edit_leaves_an_independent_source_constraint_program_readable() {
    let source = fixture(CAPTURED);
    let id = RelationId::new(NonZeroU64::new(21).unwrap());
    let relations = [(id, Relation::FixPoint(pid(2), point(CAPTURED[1])))];
    let constrained = ConstrainedSketch::restore(
        source.clone(),
        &relations,
        crate::constraints::policy(),
        || false,
    )
    .unwrap();
    let prior = constrained.inspect_rank().unwrap();
    let edited = source.bend_lines(bend(5.), || false).unwrap();
    assert!(edited.sketch.point(pid(2)).is_none());
    assert_eq!(constrained.geometry(), &source);
    assert_eq!(constrained.relations().collect::<Vec<_>>(), relations);
    assert_eq!(constrained.inspect_rank().unwrap(), prior);
}

#[test]
fn cancellation_at_any_poll_preserves_original_geometry() {
    let source = fixture(CAPTURED);
    let unchanged = source.clone();
    let mut polls = 0;
    source
        .bend_lines(bend(5.), || {
            polls += 1;
            false
        })
        .unwrap();
    for cancelled_at in 1..=polls {
        let mut seen = 0;
        let result = source.bend_lines(bend(5.), || {
            seen += 1;
            seen == cancelled_at
        });
        assert!(
            matches!(result, Err(BendFailure::Cancelled)),
            "poll {cancelled_at}"
        );
        assert_eq!(source, unchanged);
    }
}

#[test]
fn immutable_bend_replays_in_parallel_and_in_independent_processes() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<(BendRequest, BendResult, BendFailure)>();
    let source = fixture(CAPTURED);
    let first = source.bend_lines(bend(5.), || false).unwrap();
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                for _ in 0..12 {
                    assert_eq!(first, source.bend_lines(bend(5.), || false).unwrap());
                }
            });
        }
    });
    assert_eq!(process_receipt(), process_receipt());
}

fn process_receipt() -> String {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "bend::guards::emit_bend_receipt",
            "--nocapture",
            "--test-threads=1",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .unwrap()
        .split_once("BEND_REPLAY=")
        .unwrap()
        .1
        .lines()
        .next()
        .unwrap()
        .to_owned()
}

#[test]
fn emit_bend_receipt() {
    let result = fixture(CAPTURED).bend_lines(bend(5.), || false).unwrap();
    let arc = result.sketch.arc(result.request.ids.arc).unwrap();
    let points = arc
        .points()
        .map(|point| point.coordinates().map(f64::to_bits));
    println!(
        "BEND_REPLAY={:?} {:?} {points:016x?}",
        result.history, result.evidence
    );
}

#[test]
fn original_marker_and_far_endpoint_reuse_keep_their_roles_and_incidence() {
    let source = fixture_with_corner_role(CAPTURED, PointRole::CenterMarker)
        .apply(
            &[seed(7, [17., 0., 0.], PointRole::Defining), line(9, 7, 1)],
            || false,
        )
        .unwrap();
    let result = source.bend_lines(bend(5.), || false).unwrap();
    assert_eq!(
        result.history.corner,
        SupersededLineEndpoint::RetainedMarker(pid(2))
    );
    assert_eq!(result.sketch.point(pid(2)), source.point(pid(2)));
    assert_eq!(result.sketch.point(pid(1)), source.point(pid(1)));
    assert_eq!(result.sketch.line(lid(9)), source.line(lid(9)));
}

#[test]
fn a_retained_segment_below_snapshot_minimum_refuses_the_whole_draft() {
    let source = fixture([[-1., 0., 0.], [0.; 3], [0., 1., 0.]]);
    let request = bend(0.995);
    assert!(matches!(source.bend_lines(request, || false),
        Err(BendFailure::Geometry(EditFailure::BelowMinimumLength { line, .. }))
        if request.ids.lines.contains(&line)));
    assert!(source.arc(request.ids.arc).is_none());
    assert_eq!(
        source.line(lid(4)).unwrap().line.endpoints,
        [pid(1), pid(2)]
    );
}
