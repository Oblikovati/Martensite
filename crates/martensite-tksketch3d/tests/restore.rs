// SPDX-License-Identifier: GPL-2.0-only
use crate::fixture::{lid, line, pid, point, seed};
use martensite_tksketch3d::{
    Edit, EntityId, FrozenPointSource, FrozenSourceRevision, MinimumLength, PointRole, SketchPoint,
    Snapshot,
    constraints::{ConstrainedSketch, ConstraintFailure, PolicyQuantity, Relation, RelationId},
};
use std::num::NonZeroU64;

fn key(raw: u64) -> NonZeroU64 {
    NonZeroU64::new(raw).unwrap()
}

fn rid(raw: u64) -> RelationId {
    RelationId::new(key(raw))
}

fn minimum(value: f64) -> MinimumLength {
    MinimumLength::new(value).unwrap()
}

fn geometry() -> Snapshot {
    Snapshot::empty(minimum(0.01))
        .apply(
            &[
                seed(1, [-0., 0., 0.], PointRole::Defining),
                seed(2, [3., 4., 0.], PointRole::Defining),
                line(3, 1, 2),
            ],
            || false,
        )
        .unwrap()
}

#[test]
fn stored_anchors_and_positions_are_preserved_before_the_first_warm_drag() {
    let snapshot = geometry();
    let anchors = [
        (rid(7), Relation::Length(lid(3), minimum(5.))),
        (rid(4), Relation::FixPoint(pid(1), point([-0., 0., 0.]))),
    ];
    let sketch = ConstrainedSketch::restore(
        snapshot.clone(),
        &anchors,
        crate::constraints::policy(),
        || false,
    )
    .unwrap();
    assert_eq!(sketch.geometry(), &snapshot);
    assert_eq!(
        sketch.relations().collect::<Vec<_>>(),
        [anchors[1], anchors[0]]
    );
    assert_eq!(
        sketch
            .geometry()
            .point(pid(1))
            .unwrap()
            .position
            .x()
            .to_bits(),
        (-0_f64).to_bits()
    );
    assert_eq!(sketch.inspect_rank().unwrap().rank, 4);
    let dragged = sketch.drag(pid(2), point([0., 5., 0.]), || false).unwrap();
    assert_eq!(dragged.rank.rank, 4);
    let moved = dragged.sketch.geometry().point(pid(2)).unwrap().position;
    assert!(moved.x().abs() < 1e-7 && (moved.y() - 5.).abs() < 1e-7);
    assert_eq!(sketch.geometry(), &snapshot);
}

#[test]
fn restore_refuses_a_value_that_authoring_would_move_or_recapture() {
    let signed_zero = Relation::FixPoint(pid(1), point([0.; 3]));
    let old_line = Relation::FixLine(lid(3), [point([-0., 0., 0.]), point([5., 0., 0.])]);
    let old_length = Relation::Length(lid(3), minimum(6.));
    let snapshot = geometry();
    for relation in [signed_zero, old_line, old_length] {
        let restored = ConstrainedSketch::restore(
            snapshot.clone(),
            &[(rid(4), relation)],
            crate::constraints::policy(),
            || false,
        );
        assert!(
            matches!(restored, Err(ConstraintFailure::Infeasible(id, value)) if id == rid(4) && value == relation)
        );
    }
    assert_eq!(snapshot, geometry());
}

#[test]
fn restore_checks_structure_ids_limits_and_late_cancellation() {
    let snapshot = geometry();
    let policy = crate::constraints::policy();
    let length = (rid(6), Relation::Length(lid(3), minimum(5.)));
    let duplicate =
        ConstrainedSketch::restore(snapshot.clone(), &[length, length], policy, || false);
    assert!(matches!(duplicate, Err(ConstraintFailure::DuplicateId(id)) if id == rid(6)));
    let repeated = (rid(6), Relation::Coincident([pid(1), pid(1)]));
    let result = ConstrainedSketch::restore(snapshot.clone(), &[repeated], policy, || false);
    assert!(
        matches!(result, Err(ConstraintFailure::RepeatedOperand(id, EntityId::Point(point))) if id == rid(6) && point == pid(1))
    );
    let short = (rid(6), Relation::Length(lid(3), minimum(0.005)));
    let result = ConstrainedSketch::restore(snapshot.clone(), &[short], policy, || false);
    assert!(
        matches!(result, Err(ConstraintFailure::LengthBelowMinimum(id, value)) if id == rid(6) && value == 0.005)
    );
    let mut limited = policy;
    limited.validation_items = 3;
    let result = ConstrainedSketch::restore(snapshot.clone(), &[length], limited, || false);
    assert!(matches!(
        result,
        Err(ConstraintFailure::Limit {
            quantity: PolicyQuantity::ValidationItems,
            requested: 4,
            maximum: 3
        })
    ));
    let mut polls = 0;
    let result = ConstrainedSketch::restore(snapshot.clone(), &[length], policy, || {
        polls += 1;
        polls == 4
    });
    assert!(matches!(result, Err(ConstraintFailure::Cancelled)));
    assert_eq!(snapshot, geometry());
}

#[test]
fn restored_frozen_history_keeps_generation_without_inventing_incarnation() {
    let source = FrozenPointSource {
        document: [42; 16],
        sketch: key(19),
        revision: FrozenSourceRevision::SessionGeneration { generation: 33 },
        point: pid(12),
    };
    let snapshot = Snapshot::empty(minimum(0.01))
        .apply(
            &[
                Edit::InsertFrozenPoint {
                    point: SketchPoint {
                        id: pid(1),
                        role: PointRole::Defining,
                        position: point([-0., 0., 0.]),
                    },
                    source,
                },
                seed(2, [3., 4., 0.], PointRole::Defining),
                line(3, 1, 2),
            ],
            || false,
        )
        .unwrap();
    let sketch = ConstrainedSketch::restore(
        snapshot,
        &[(rid(6), Relation::Length(lid(3), minimum(5.)))],
        crate::constraints::policy(),
        || false,
    )
    .unwrap();
    assert_eq!(sketch.inspect_rank().unwrap().frozen_coordinates, 3);
    assert_eq!(sketch.geometry().frozen_source(pid(1)), Some(source));
    assert!(
        matches!(sketch.drag(pid(1), point([0.; 3]), || false), Err(ConstraintFailure::FrozenDrag(point, stamp)) if point == pid(1) && stamp == source)
    );
    let updated = sketch.drag(pid(2), point([4., 3., 0.]), || false).unwrap();
    assert_eq!(
        updated.sketch.geometry().frozen_source(pid(1)),
        Some(source)
    );
    assert_eq!(
        updated
            .sketch
            .geometry()
            .point(pid(1))
            .unwrap()
            .position
            .x()
            .to_bits(),
        (-0_f64).to_bits()
    );
}

#[test]
fn all_directed_line_relations_prepare_from_their_saved_operands() {
    use martensite_tkmath::gp::Direction3;
    use martensite_tksketch3d::constraints::{AxisDirection, CoordinateAxis, LineAngle};

    let snapshot = Snapshot::empty(minimum(0.01))
        .apply(
            &[
                seed(1, [-0., 0., 0.], PointRole::Defining),
                seed(2, [3., 0., 0.], PointRole::Defining),
                seed(3, [0., 0., 1.], PointRole::Defining),
                seed(4, [0., 3., 1.], PointRole::Defining),
                line(5, 1, 2),
                line(6, 3, 4),
            ],
            || false,
        )
        .unwrap();
    let lines = [lid(5), lid(6)];
    let saved = [
        (
            rid(10),
            Relation::Angle(lines, LineAngle::new(std::f64::consts::FRAC_PI_2).unwrap()),
        ),
        (
            rid(7),
            Relation::ParallelToAxis(lines[0], CoordinateAxis::X, AxisDirection::Positive),
        ),
        (rid(9), Relation::Perpendicular(lines)),
        (rid(6), Relation::ParallelToPlane(lines[0], Direction3::Z)),
        (rid(8), Relation::EqualLength(lines)),
    ];
    let sketch = ConstrainedSketch::restore(
        snapshot.clone(),
        &saved,
        crate::constraints::policy(),
        || false,
    )
    .unwrap();
    let mut ordered = saved;
    ordered.sort_by_key(|item| item.0);
    assert_eq!(sketch.relations().collect::<Vec<_>>(), ordered);
    assert_eq!(sketch.geometry(), &snapshot);
    assert_eq!(
        sketch
            .geometry()
            .point(pid(1))
            .unwrap()
            .position
            .x()
            .to_bits(),
        (-0_f64).to_bits()
    );
}
