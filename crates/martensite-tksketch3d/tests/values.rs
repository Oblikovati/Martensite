// SPDX-License-Identifier: GPL-2.0-only
use super::fixture::{baseline, connect, lid, line, pid, point, seed};
use martensite_tkmath::gp::Direction3;
use martensite_tksketch3d::{
    BatchPhase, Edit, EditFailure, EntityId, InvalidMinimumLength, LineGeometry, LineId,
    MinimumComparison, MinimumLength, PointId, PointRole, SketchLine, SketchPoint, Snapshot,
};

#[test]
fn owned_import_polyline_uses_xyz_and_one_shared_endpoint() {
    let snapshot = baseline();
    let horizontal = snapshot.line(lid(5)).unwrap();
    let vertical = snapshot.line(lid(6)).unwrap();
    assert_eq!(
        horizontal.points,
        [point([0., 0., 0.]), point([4., 0., 0.])]
    );
    assert_eq!(vertical.points, [point([4., 0., 0.]), point([4., 3., 0.])]);
    assert_eq!(horizontal.line.endpoints[1], vertical.line.endpoints[0]);
    assert_eq!(horizontal.carrier.direction(), Direction3::X);
    assert_eq!(vertical.carrier.direction(), Direction3::Y);
    assert_eq!(
        (horizontal.length_estimate, vertical.length_estimate),
        (4., 3.)
    );
    assert_eq!(horizontal.length_estimate + vertical.length_estimate, 7.);
}

#[test]
fn role_visibility_and_iteration_preserve_stable_identity() {
    let snapshot = baseline();
    assert_eq!(
        snapshot.owned_ids().collect::<Vec<_>>(),
        [
            EntityId::Point(pid(1)),
            EntityId::Point(pid(2)),
            EntityId::Point(pid(3)),
            EntityId::Point(pid(4)),
            EntityId::Line(lid(5)),
            EntityId::Line(lid(6)),
        ]
    );
    assert_eq!(
        snapshot.visible_ids().collect::<Vec<_>>(),
        [
            EntityId::Point(pid(2)),
            EntityId::Point(pid(4)),
            EntityId::Line(lid(5)),
            EntityId::Line(lid(6)),
        ]
    );
    assert_eq!(
        snapshot.point(pid(4)).unwrap().role,
        PointRole::CenterMarker
    );
}

#[test]
fn editing_shared_point_rebuilds_both_lines_without_mutating_old_values() {
    let snapshot = baseline();
    let before = snapshot.clone();
    let mut caller = point([4., 0., 3.]);
    let changed = snapshot
        .apply(
            &[Edit::MovePoint {
                point: pid(2),
                position: caller,
            }],
            || false,
        )
        .unwrap();
    caller.set_z(900.);
    let a = changed.line(lid(5)).unwrap();
    let b = changed.line(lid(6)).unwrap();
    assert_eq!(a.points[1], b.points[0]);
    assert_eq!(a.points[1], point([4., 0., 3.]));
    assert_eq!(a.length_estimate, 5.);
    assert_eq!(b.length_estimate, 18_f64.sqrt());
    assert_eq!(changed.point(pid(2)).unwrap().role, PointRole::SketchMarker);
    assert_eq!(snapshot, before);
    assert_eq!(caller.z(), 900.);
}

#[test]
fn unchanged_line_inputs_keep_owned_geometry_and_changed_bits_are_read_back() {
    let source = baseline();
    let changed = source
        .apply(
            &[Edit::MovePoint {
                point: pid(1),
                position: point([-0., 0., 0.]),
            }],
            || false,
        )
        .unwrap();
    let rebuilt = changed.line(lid(5)).unwrap();
    let unchanged = changed.line(lid(6)).unwrap();
    assert_eq!(rebuilt.line.endpoints, [pid(1), pid(2)]);
    assert_eq!(rebuilt.points[0].x().to_bits(), (-0_f64).to_bits());
    assert_eq!(rebuilt.carrier.location().x().to_bits(), (-0_f64).to_bits());
    assert_eq!(unchanged, source.line(lid(6)).unwrap());
    assert_eq!(unchanged.line.endpoints, [pid(2), pid(3)]);
    assert_eq!(
        changed.owned_ids().collect::<Vec<_>>(),
        source.owned_ids().collect::<Vec<_>>()
    );
    assert_eq!(
        source.line(lid(5)).unwrap().points[0].x().to_bits(),
        0_f64.to_bits()
    );
}

#[test]
fn complete_batch_can_resolve_a_transient_coincidence() {
    let previous = connect([0., 0., 0.], [2., 0., 0.], 0.05).unwrap();
    let moved = previous
        .apply(
            &[
                Edit::MovePoint {
                    point: pid(1),
                    position: point([2., 0., 0.]),
                },
                Edit::MovePoint {
                    point: pid(2),
                    position: point([5., 4., 12.]),
                },
            ],
            || false,
        )
        .unwrap();
    assert_eq!(moved.line(lid(3)).unwrap().length_estimate, 13.);
    assert_eq!(previous.line(lid(3)).unwrap().length_estimate, 2.);
}

#[test]
fn removing_lines_retires_unused_defining_points_and_keeps_markers() {
    let original = baseline();
    let marker_only = original
        .apply(
            &[Edit::RemoveLine(lid(5)), Edit::RemoveLine(lid(6))],
            || false,
        )
        .unwrap();
    assert_eq!(
        marker_only.owned_ids().collect::<Vec<_>>(),
        [EntityId::Point(pid(2)), EntityId::Point(pid(4)),]
    );
    assert_eq!(original.owned_ids().count(), 6);

    let still_connected = original
        .apply(&[Edit::RemovePoint(pid(1))], || false)
        .unwrap();
    assert!(still_connected.line(lid(5)).is_none());
    assert_eq!(still_connected.line(lid(6)), original.line(lid(6)));
    assert_eq!(still_connected.point(pid(3)), original.point(pid(3)));
}

#[test]
fn deletion_closure_retires_all_unused_defining_points() {
    let source = baseline()
        .apply(&[seed(8, [8., 9., 10.], PointRole::Defining)], || false)
        .unwrap();
    assert!(source.point(pid(8)).is_some());
    let changed = source.apply(&[Edit::RemoveLine(lid(5))], || false).unwrap();
    assert!(changed.point(pid(1)).is_none());
    assert!(changed.point(pid(8)).is_none());
    assert_eq!(changed.point(pid(2)), source.point(pid(2)));
    assert_eq!(changed.point(pid(4)), source.point(pid(4)));
}

#[test]
fn explicit_line_removal_precedes_removal_of_its_point() {
    let source = baseline();
    let redundant = [Edit::RemovePoint(pid(1)), Edit::RemoveLine(lid(5))];
    assert_eq!(
        source.apply(&redundant, || false),
        Err(EditFailure::MissingEditTarget(EntityId::Line(lid(5))))
    );
    let ordered = [redundant[1], redundant[0]];
    let result = source.apply(&ordered, || false).unwrap();
    assert!(result.line(lid(5)).is_none());
    assert!(result.point(pid(1)).is_none());
    assert_eq!(result.line(lid(6)), source.line(lid(6)));
}

#[test]
fn numeric_keys_cannot_collide_across_kinds_or_in_a_batch() {
    let original = baseline();
    let duplicate = seed(2, [1., 2., 3.], PointRole::CenterMarker);
    assert_eq!(
        original.apply(&[duplicate], || false),
        Err(EditFailure::DuplicateIdentity {
            existing: EntityId::Point(pid(2)),
            attempted: EntityId::Point(pid(2)),
        })
    );
    assert_eq!(
        original.apply(&[line(2, 1, 3)], || false),
        Err(EditFailure::DuplicateIdentity {
            existing: EntityId::Point(pid(2)),
            attempted: EntityId::Line(lid(2)),
        })
    );
    let new = seed(8, [8., 9., 10.], PointRole::SketchMarker);
    assert!(matches!(
        original.apply(&[new, new], || false),
        Err(EditFailure::DuplicateIdentity { .. })
    ));
}

#[test]
fn a_batch_cannot_reuse_a_removed_identity() {
    let snapshot = baseline();
    let before = snapshot.clone();
    assert_eq!(
        snapshot.apply(&[Edit::RemoveLine(lid(5)), line(5, 2, 3)], || false),
        Err(EditFailure::DuplicateIdentity {
            existing: EntityId::Line(lid(5)),
            attempted: EntityId::Line(lid(5)),
        })
    );
    assert_eq!(snapshot, before);
}

#[test]
fn edit_targets_must_match_both_key_and_kind() {
    let snapshot = baseline();
    assert_eq!(
        snapshot.apply(&[Edit::RemoveLine(lid(1))], || false),
        Err(EditFailure::MissingEditTarget(EntityId::Line(lid(1))))
    );
    assert_eq!(
        snapshot.apply(
            &[Edit::MovePoint {
                point: pid(5),
                position: point([0.; 3])
            }],
            || false
        ),
        Err(EditFailure::MissingEditTarget(EntityId::Point(pid(5))))
    );
}

#[test]
fn missing_and_coincident_references_fail_without_publication() {
    let before = baseline();
    let prior = before.clone();
    assert_eq!(
        before.apply(&[line(7, 1, 99)], || false),
        Err(EditFailure::MissingEndpoint {
            line: lid(7),
            point: pid(99),
        })
    );
    assert_eq!(
        before.apply(&[line(7, 2, 2)], || false),
        Err(EditFailure::SameEndpoint {
            line: lid(7),
            point: pid(2),
        })
    );
    let prefix = seed(8, [-0., 0., 0.], PointRole::CenterMarker);
    assert_eq!(
        before.apply(&[prefix, line(7, 1, 8)], || false),
        Err(EditFailure::CoincidentEndpoints(SketchLine {
            id: lid(7),
            endpoints: [pid(1), pid(8)]
        },))
    );
    assert!(before.point(pid(8)).is_none());
    assert_eq!(before, prior);
}

#[test]
fn cancellation_during_staging_validation_and_return_is_atomic() {
    let previous = baseline();
    let unchanged = previous.clone();
    let edits = [
        Edit::MovePoint {
            point: pid(2),
            position: point([4., 0., 3.]),
        },
        seed(8, [5., 6., 7.], PointRole::SketchMarker),
    ];
    for (stop, phase) in [
        (1, BatchPhase::BeforeStaging),
        (3, BatchPhase::Staging),
        (5, BatchPhase::Validation),
        (6, BatchPhase::Completion),
    ] {
        let mut polls = 0;
        let result = previous.apply(&edits, || {
            polls += 1;
            polls == stop
        });
        assert_eq!(result, Err(EditFailure::Cancelled(phase)));
        assert_eq!(polls, stop);
        assert_eq!(previous, unchanged);
    }
}

#[test]
fn deletion_scans_observe_cancellation_before_validation() {
    let source = baseline();
    let before = source.clone();
    for edit in [Edit::RemovePoint(pid(1)), Edit::RemoveLine(lid(5))] {
        let mut polls = 0;
        let result = source.apply(&[edit], || {
            polls += 1;
            polls == 3
        });
        assert_eq!(result, Err(EditFailure::Cancelled(BatchPhase::Staging)));
        assert_eq!(source, before);
    }
}

#[test]
fn public_values_can_cross_threads() {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<(
        PointId,
        LineId,
        EntityId,
        PointRole,
        SketchPoint,
        SketchLine,
    )>();
    require_send_sync::<(
        MinimumLength,
        MinimumComparison,
        LineGeometry,
        Edit,
        Snapshot,
    )>();
    require_send_sync::<(InvalidMinimumLength, BatchPhase, EditFailure)>();
}

#[test]
fn repeated_batches_on_shared_snapshots_match_sequential_results() {
    let source = baseline();
    let before = source.clone();
    let alternatives = [3., -7.].map(|z| Edit::MovePoint {
        point: pid(2),
        position: point([4., 0., z]),
    });
    let expected = alternatives.map(|edit| source.apply(&[edit], || false).unwrap());
    std::thread::scope(|scope| {
        for worker in 0..4 {
            let (source, expected) = (&source, &expected);
            scope.spawn(move || repeat_parallel_edits(worker, source, &alternatives, expected));
        }
    });
    assert_eq!(source, before);
}

fn repeat_parallel_edits(
    worker: usize,
    source: &Snapshot,
    alternatives: &[Edit; 2],
    expected: &[Snapshot; 2],
) {
    for iteration in 0..32 {
        let index = (worker + iteration) % alternatives.len();
        let changed = source.apply(&[alternatives[index]], || false).unwrap();
        assert_eq!(changed, expected[index]);
        let mut copied_endpoint = changed.point(pid(2)).unwrap();
        copied_endpoint.position.set_z(800.);
        assert_eq!(copied_endpoint.position.z(), 800.);
        assert_eq!(changed, expected[index]);
    }
}
