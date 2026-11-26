// SPDX-License-Identifier: GPL-2.0-only
use super::fixture::{lid, line, pid, point, seed};
use martensite_tksketch3d::{
    BatchPhase, Edit, EditFailure, EntityId, FrozenPointSource, FrozenSourceRevision,
    MinimumLength, PointRole, SketchPoint, Snapshot,
};
use std::num::NonZeroU64;

fn empty() -> Snapshot {
    Snapshot::empty(MinimumLength::new(0.01).unwrap())
}

fn source() -> FrozenPointSource {
    FrozenPointSource {
        document: [27; 16],
        sketch: NonZeroU64::new(3).unwrap(),
        revision: FrozenSourceRevision::Snapshot {
            incarnation: 2,
            revision: 4,
        },
        point: pid(6),
    }
}

fn capture(point: SketchPoint) -> Edit {
    Edit::InsertFrozenPoint {
        point: SketchPoint {
            id: pid(1),
            ..point
        },
        source: source(),
    }
}

fn frozen_at(position: [f64; 3]) -> Edit {
    capture(SketchPoint {
        id: source().point,
        role: PointRole::Defining,
        position: point(position),
    })
}

#[test]
fn a_connected_frozen_point_survives_source_motion_and_deletion() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Snapshot>();
    assert_send_sync::<FrozenPointSource>();

    let upstream = empty()
        .apply(&[seed(6, [0., 20., 30.], PointRole::Defining)], || false)
        .unwrap();
    let destination = empty()
        .apply(
            &[
                capture(upstream.point(pid(6)).unwrap()),
                seed(2, [100., 25., 50.], PointRole::SketchMarker),
                line(3, 1, 2),
            ],
            || false,
        )
        .unwrap();
    let moved = upstream
        .apply(
            &[Edit::MovePoint {
                point: pid(6),
                position: point([1., 20., 30.]),
            }],
            || false,
        )
        .unwrap();
    let deleted = moved.apply(&[Edit::RemovePoint(pid(6))], || false).unwrap();

    assert!(deleted.point(pid(6)).is_none());
    assert_eq!(destination.frozen_source(pid(1)), Some(source()));
    let connected = destination.line(lid(3)).unwrap();
    assert_eq!(connected.line.endpoints, [pid(1), pid(2)]);
    assert_eq!(
        connected.points,
        [point([0., 20., 30.]), point([100., 25., 50.])]
    );
    assert!((connected.length_estimate * connected.length_estimate - 10425.).abs() < 1e-10);
}

#[test]
fn moving_a_frozen_point_refuses_the_batch_until_it_is_detached() {
    let original = empty()
        .apply(
            &[
                frozen_at([-0., 0., 0.]),
                seed(2, [4., 0., 0.], PointRole::SketchMarker),
                line(3, 1, 2),
            ],
            || false,
        )
        .unwrap();
    let move_point = Edit::MovePoint {
        point: pid(1),
        position: point([1., 0., 0.]),
    };
    assert_eq!(
        original.apply(
            &[seed(8, [0.; 3], PointRole::SketchMarker), move_point],
            || false
        ),
        Err(EditFailure::FrozenPointMove {
            point: pid(1),
            source: source(),
        })
    );
    assert!(original.point(pid(8)).is_none());
    let detached = original
        .apply(&[Edit::DetachFrozenPoint(pid(1))], || false)
        .unwrap();
    assert_eq!(
        detached.line(lid(3)).unwrap().points[0].x().to_bits(),
        (-0.0_f64).to_bits()
    );
    assert_eq!(
        detached.apply(&[Edit::DetachFrozenPoint(pid(1))], || false),
        Err(EditFailure::PointIsNotFrozen(pid(1)))
    );
    assert_eq!(
        detached.apply(&[Edit::DetachFrozenPoint(pid(3))], || false),
        Err(EditFailure::MissingEditTarget(EntityId::Point(pid(3))))
    );
    let moved = detached.apply(&[move_point], || false).unwrap();
    assert_eq!(moved.frozen_source(pid(1)), None);
    assert_eq!(moved.line(lid(3)).unwrap().length_estimate, 3.);
    assert_eq!(original.frozen_source(pid(1)), Some(source()));
    assert_eq!(original.line(lid(3)).unwrap().length_estimate, 4.);
}

#[test]
fn explicit_and_orphan_removal_clear_frozen_provenance() {
    let original = empty()
        .apply(
            &[
                frozen_at([0.; 3]),
                seed(2, [4., 0., 0.], PointRole::SketchMarker),
                line(3, 1, 2),
            ],
            || false,
        )
        .unwrap();
    for edit in [Edit::RemovePoint(pid(1)), Edit::RemoveLine(lid(3))] {
        let removed = original.apply(&[edit], || false).unwrap();
        assert!(removed.point(pid(1)).is_none());
        assert!(removed.frozen_source(pid(1)).is_none());
        assert!(removed.line(lid(3)).is_none());
        assert!(removed.point(pid(2)).is_some());
    }
    assert_eq!(original.frozen_source(pid(1)), Some(source()));
}

#[test]
fn invalid_and_cancelled_captures_leave_the_destination_unchanged() {
    let before = empty();
    assert!(matches!(
        before.apply(&[frozen_at([f64::INFINITY, 0., 0.])], || false),
        Err(EditFailure::NonFinitePoint { point, axis: 0, .. }) if point == pid(1)
    ));
    assert!(matches!(
        before.apply(&[frozen_at([0.; 3]), line(2, 1, 9)], || false),
        Err(EditFailure::MissingEndpoint { point, .. }) if point == pid(9)
    ));
    let mut polls = 0;
    assert_eq!(
        before.apply(&[frozen_at([0.; 3])], || {
            polls += 1;
            polls == 3
        }),
        Err(EditFailure::Cancelled(BatchPhase::Completion))
    );
    assert_eq!(before.owned_ids().count(), 0);
    assert_eq!(before.frozen_source(pid(1)), None);
}
