// SPDX-License-Identifier: GPL-2.0-only
use crate::fixture::{lid, line, pid, point, seed};
use martensite_tkmath::gp::{Axis1, Direction3, Point3, Vector3};
use martensite_tksketch3d::{
    Edit, EditFailure, FrozenPointSource, MinimumLength, PointRole, SketchPoint, Snapshot,
    constraints::{
        ConstrainedSketch, ConstraintFailure, RelationEdit, RelationId, RelationSpec, RigidMotion,
        RigidMotionFailure,
    },
};
use std::{f64::consts::FRAC_PI_2, num::NonZeroU64};

fn id(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).unwrap()
}

fn source() -> ConstrainedSketch {
    let shape = Snapshot::empty(MinimumLength::new(0.001).unwrap())
        .apply(
            &[
                seed(1, [1., 0., 0.], PointRole::Defining),
                seed(2, [2., 0., 0.], PointRole::Defining),
                seed(3, [-7., 5., 3.], PointRole::SketchMarker),
                line(4, 1, 2),
            ],
            || false,
        )
        .unwrap();
    let base = ConstrainedSketch::new(shape, crate::constraints::policy()).unwrap();
    base.apply(
        &[RelationEdit::Insert(
            RelationId::new(id(1)),
            RelationSpec::Length(lid(4), MinimumLength::new(1.).unwrap()),
        )],
        || false,
    )
    .unwrap()
}

fn near(actual: Point3, expected: [f64; 3]) {
    for (actual, expected) in actual.coordinates().iter().zip(expected) {
        assert!((actual - expected).abs() < 1e-12, "{actual} vs {expected}");
    }
}

#[test]
fn signed_rigid_moves_preserve_length_ids_and_the_unselected_marker() {
    let base = source();
    let selection = [pid(2), pid(1)];
    let translated = base
        .transform_points(
            &selection,
            RigidMotion::Translate(Vector3::new(3., -2., 1.)),
            || false,
        )
        .unwrap();
    let line = translated.geometry().line(lid(4)).unwrap();
    assert_eq!(line.line.endpoints, [pid(1), pid(2)]);
    assert_eq!(line.points, [[4., -2., 1.], [5., -2., 1.]].map(point));
    assert_eq!(line.length_estimate, 1.);
    assert_eq!(
        translated.geometry().point(pid(3)),
        base.geometry().point(pid(3))
    );
    assert_eq!(
        translated.relations().collect::<Vec<_>>(),
        base.relations().collect::<Vec<_>>()
    );
    let axis = Axis1::new(point([0.; 3]), Direction3::Z);
    let rotated = base
        .transform_points(
            &selection,
            RigidMotion::Rotate {
                axis,
                radians: FRAC_PI_2,
            },
            || false,
        )
        .unwrap();
    let [start, end] = rotated.geometry().line(lid(4)).unwrap().points;
    near(start, [0., 1., 0.]);
    near(end, [0., 2., 0.]);
    assert_eq!(rotated.inspect_rank().unwrap().rank, 1);
    assert_eq!(
        base.geometry().line(lid(4)).unwrap().points,
        [[1., 0., 0.], [2., 0., 0.]].map(point)
    );
}

#[test]
fn a_located_rotation_leaves_points_on_the_axis_alone() {
    let sketch = source();
    let fixed_id = RelationId::new(id(2));
    let fixed = sketch
        .apply(
            &[RelationEdit::Insert(
                fixed_id,
                RelationSpec::FixPoint(pid(1)),
            )],
            || false,
        )
        .unwrap();
    let axis = Axis1::new(point([1., 0., 0.]), Direction3::Z);
    let moved = fixed
        .transform_points(
            &[pid(1), pid(2)],
            RigidMotion::Rotate {
                axis,
                radians: -FRAC_PI_2,
            },
            || false,
        )
        .unwrap();
    assert_eq!(
        moved.geometry().point(pid(1)),
        fixed.geometry().point(pid(1))
    );
    near(
        moved.geometry().point(pid(2)).unwrap().position,
        [1., -1., 0.],
    );
}

#[test]
fn partial_rigid_edits_and_moving_a_fixed_endpoint_refuse_atomically() {
    let base = source();
    let motion = RigidMotion::Translate(Vector3::new(0., 2., 0.));
    let prior = base.geometry().clone();
    let partial = base.transform_points(&[pid(1)], motion, || false);
    assert!(
        matches!(partial, Err(RigidMotionFailure::Constraint(ConstraintFailure::Infeasible(id, _))) if id == RelationId::new(NonZeroU64::MIN))
    );
    let fixed_id = RelationId::new(id(2));
    let fixed = base
        .apply(
            &[RelationEdit::Insert(
                fixed_id,
                RelationSpec::FixPoint(pid(1)),
            )],
            || false,
        )
        .unwrap();
    let refused = fixed.transform_points(&[pid(1), pid(2)], motion, || false);
    assert!(
        matches!(refused, Err(RigidMotionFailure::Constraint(ConstraintFailure::Infeasible(id, _))) if id == fixed_id)
    );
    assert_eq!(fixed.geometry(), &prior);
    assert_eq!(base.geometry(), &prior);
}

#[test]
fn moving_a_captured_endpoint_preserves_its_read_only_source() {
    let stamp = FrozenPointSource {
        document: [7; 16],
        sketch: id(8),
        incarnation: 9,
        revision: 10,
        point: pid(11),
    };
    let snapshot = source()
        .geometry()
        .apply(
            &[Edit::InsertFrozenPoint {
                point: SketchPoint {
                    id: pid(5),
                    role: PointRole::SketchMarker,
                    position: point([1., 2., 3.]),
                },
                source: stamp,
            }],
            || false,
        )
        .unwrap();
    let sketch = ConstrainedSketch::new(snapshot, crate::constraints::policy()).unwrap();
    let moved = sketch.transform_points(
        &[pid(5)],
        RigidMotion::Translate(Vector3::new(1., 0., 0.)),
        || false,
    );
    assert!(
        matches!(moved, Err(RigidMotionFailure::Constraint(ConstraintFailure::Geometry(EditFailure::FrozenPointMove { point, source }))) if point == pid(5) && source == stamp)
    );
    assert_eq!(sketch.geometry().frozen_source(pid(5)), Some(stamp));
}

#[test]
fn invalid_selection_invalid_motion_and_staged_cancellation_are_distinct() {
    let base = source();
    let motion = RigidMotion::Translate(Vector3::new(0., 2., 0.));
    assert!(matches!(
        base.transform_points(&[], motion, || false),
        Err(RigidMotionFailure::EmptySelection)
    ));
    assert!(
        matches!(base.transform_points(&[pid(1), pid(1)], motion, || false), Err(RigidMotionFailure::RepeatedPoint(point)) if point == pid(1))
    );
    assert!(
        matches!(base.transform_points(&[pid(17)], motion, || false), Err(RigidMotionFailure::Constraint(ConstraintFailure::MissingPoint(point))) if point == pid(17))
    );
    let invalid = RigidMotion::Translate(Vector3::new(0., 0., f64::NAN));
    assert!(matches!(
        base.transform_points(&[pid(1)], invalid, || false),
        Err(RigidMotionFailure::InvalidMotion(_))
    ));
    let rotating = RigidMotion::Rotate {
        axis: Axis1::new(point([0., f64::INFINITY, 0.]), Direction3::Z),
        radians: FRAC_PI_2,
    };
    assert!(matches!(
        base.transform_points(&[pid(1)], rotating, || false),
        Err(RigidMotionFailure::InvalidMotion(_))
    ));
    let mut polls = 0;
    let cancelled = base.transform_points(&[pid(1), pid(2)], motion, || {
        polls += 1;
        polls == 3
    });
    assert!(matches!(
        cancelled,
        Err(RigidMotionFailure::Constraint(ConstraintFailure::Cancelled))
    ));
    assert_eq!(
        base.geometry().line(lid(4)).unwrap().points,
        [[1., 0., 0.], [2., 0., 0.]].map(point)
    );
}

#[test]
fn a_finite_nonunit_stored_rotation_axis_refuses_before_staging() {
    let sketch = source();
    let mut direction = Direction3::Z;
    let mut position = 1;
    let stored = r#""gp_Dir": [0, 0, 1.7976931348623157e308]"#;
    assert!(direction.init_from_json(stored, &mut position).unwrap());
    let motion = RigidMotion::Rotate {
        axis: Axis1::new(point([0.; 3]), direction),
        radians: FRAC_PI_2,
    };
    assert!(matches!(
        sketch.transform_points(&[pid(1), pid(2)], motion, || false),
        Err(RigidMotionFailure::InvalidMotion(_))
    ));
}

#[test]
fn a_rigid_rotation_rebinds_the_prepared_parallel_reference_for_later_drag() {
    use martensite_tksketch3d::constraints::ParallelDirection;

    let base = Snapshot::empty(MinimumLength::new(0.001).unwrap())
        .apply(
            &[
                seed(1, [0., 0., 0.], PointRole::Defining),
                seed(2, [2., 0., 0.], PointRole::Defining),
                seed(3, [0., 1., 0.], PointRole::Defining),
                seed(4, [2., 1., 0.], PointRole::Defining),
                line(5, 1, 2),
                line(6, 3, 4),
            ],
            || false,
        )
        .unwrap();
    let sketch = ConstrainedSketch::new(base, crate::constraints::policy()).unwrap();
    let relation = RelationEdit::Insert(
        RelationId::new(id(1)),
        RelationSpec::Parallel([lid(5), lid(6)], ParallelDirection::Same),
    );
    let sketch = sketch.apply(&[relation], || false).unwrap();
    let motion = RigidMotion::Rotate {
        axis: Axis1::new(point([0.; 3]), Direction3::Z),
        radians: FRAC_PI_2,
    };
    let rotated = sketch
        .transform_points(&[pid(1), pid(2), pid(3), pid(4)], motion, || false)
        .unwrap();
    assert_eq!(rotated.inspect_rank().unwrap().rank, 2);
    let dragged = rotated
        .drag(pid(4), point([-1., 2.1, 0.]), || false)
        .unwrap();
    assert_eq!(dragged.rank.rank, 2);
    near(
        dragged.sketch.geometry().point(pid(4)).unwrap().position,
        [-1., 2.1, 0.],
    );
}
