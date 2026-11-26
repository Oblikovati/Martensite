// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tkmath::{gp::Vector3, predicates::ArcTripleMetrics};
use martensite_tksketch3d::constraints::{
    AxisDirection, CoordinateAxis, LineArcJoin, RigidMotion, RigidMotionFailure,
};
use std::cmp::Ordering;

const S: [f64; 3] = [-5., 0., 0.];

fn shape(half_chord: f64, sagitta: f64) -> Snapshot {
    create(
        [
            S,
            [S[0] + 2. * half_chord, 0., 0.],
            [S[0] + half_chord, sagitta, 0.],
        ],
        through(),
        1e-9,
    )
    .unwrap()
}

fn radius(target: f64) -> RelationEdit {
    RelationEdit::Insert(rid(1), RelationSpec::Radius(aid(), length(target)))
}

fn recorded(target: f64) -> Relation {
    Relation::Radius(aid(), length(target), ArcBranch::CrossHalfTurn)
}

fn radius_and_sweep(shape: &Snapshot, target: f64) -> Ordering {
    let arc = shape.arc(aid()).unwrap();
    let input = arc.points().map(|point| *point.coordinates());
    let metrics = ArcTripleMetrics::new(input).unwrap();
    let tolerance = policy().length_tolerance.value();
    assert_eq!(metrics.radius_in_band(target, tolerance), Ok(true));
    assert_ne!(metrics.midpoint_offset(tolerance), Ok(Ordering::Greater));
    assert!((arc.radius() - target).abs() <= tolerance);
    let sweep = arc.range()[1].total_cmp(&PI);
    assert_eq!(sweep, metrics.branch());
    sweep
}

#[test]
fn free_half_turn_radius_has_two_hard_rows_and_drives_the_chord() {
    let original = shape(4., 4.);
    assert_eq!(radius_and_sweep(&original, 4.), Ordering::Equal);
    let sketch = ConstrainedSketch::new(original.clone(), policy()).unwrap();
    let rank = sketch.inspect_rank().unwrap();
    assert_eq!(
        (rank.rank, rank.degrees_of_freedom, rank.equations),
        (1, 8, 0)
    );

    let solved = sketch.apply(&[radius(5.)], || false).unwrap();
    assert_eq!(solved.relations().next(), Some((rid(1), recorded(5.))));
    radius_and_sweep(solved.geometry(), 5.);
    let rank = solved.inspect_rank().unwrap();
    assert_eq!(
        (rank.rank, rank.degrees_of_freedom, rank.equations),
        (2, 7, 1)
    );
    let before = original.arc(aid()).unwrap();
    let after = solved.geometry().arc(aid()).unwrap();
    assert_eq!(before.recipe(), after.recipe());
    assert_ne!(before.points()[..2], after.points()[..2]);
    assert_eq!(sketch.geometry(), &original);
}

#[test]
fn persisted_crossing_rebinds_the_exact_minor_half_turn_and_major_radius_family() {
    let retained = [(rid(1), recorded(5.))];
    for (a, b, expected) in [
        (4., 2., Ordering::Less),
        (5., 5., Ordering::Equal),
        (4., 8., Ordering::Greater),
    ] {
        let original = shape(a, b);
        let restored =
            ConstrainedSketch::restore(original.clone(), &retained, policy(), || false).unwrap();
        assert_eq!(restored.geometry(), &original);
        assert_eq!(radius_and_sweep(restored.geometry(), 5.), expected);
        assert_eq!(restored.inspect_rank().unwrap().degrees_of_freedom, 7);
        assert_eq!(restored.relations().collect::<Vec<_>>(), retained);
    }
}

#[test]
fn warm_fixed_radius_drag_crosses_the_half_turn_with_one_endpoint_free() {
    let original = shape(5., 5.).apply(&[line(9, 1, 2)], || false).unwrap();
    let controls = [
        radius(5.),
        RelationEdit::Insert(rid(2), RelationSpec::FixPoint(pid(1))),
        RelationEdit::Insert(
            rid(3),
            RelationSpec::ParallelToAxis(lid(9), CoordinateAxis::X, AxisDirection::Positive),
        ),
    ];
    let mut sketch = ConstrainedSketch::new(original, policy())
        .unwrap()
        .apply(&controls, || false)
        .unwrap();
    assert_eq!(radius_and_sweep(sketch.geometry(), 5.), Ordering::Equal);
    for index in [-4, -3, -2, -1, 1, 2, 3, 4] {
        sketch = walk_radius(sketch, f64::from(index) / 16.);
    }
    let saved = sketch.relations().collect::<Vec<_>>();
    assert_eq!(saved[0], (rid(1), recorded(5.)));
    let restored =
        ConstrainedSketch::restore(sketch.geometry().clone(), &saved, policy(), || false).unwrap();
    walk_radius(restored, -1. / 16.);
}

fn walk_radius(sketch: ConstrainedSketch, t: f64) -> ConstrainedSketch {
    let denominator = 1. + t * t;
    let a = 5. * (1. - t * t) / denominator;
    let b = 5. + 10. * t / denominator;
    let target = point([S[0] + a, b, 0.]);
    let dragged = sketch
        .drag_preferred(pid(3), target, 1e-8, || false)
        .unwrap();
    assert!(dragged.preference.scaled_cursor_error < 1e-7);
    assert_eq!(
        (
            dragged.update.rank.rank,
            dragged.update.rank.degrees_of_freedom
        ),
        (7, 2)
    );
    let geometry = dragged.update.sketch.geometry();
    assert_eq!(radius_and_sweep(geometry, 5.), t.total_cmp(&0.));
    let arc = geometry.arc(aid()).unwrap();
    assert_eq!(arc.points()[0], point(S));
    let terminal = [S[0] + 2. * a, 0., 0.];
    let endpoint = arc.points()[1];
    assert!(
        endpoint
            .coordinates()
            .iter()
            .zip(terminal)
            .all(|(actual, expected)| (actual - expected).abs() < 2e-7)
    );
    assert!(dragged.update.residual_work_bound <= policy().residual_work);
    dragged.update.sketch
}

#[test]
fn locking_the_exact_half_turn_chord_refuses_the_radius_atomically() {
    let geometry = shape(5., 5.).apply(&[line(9, 1, 2)], || false).unwrap();
    let sketch = ConstrainedSketch::new(geometry.clone(), policy()).unwrap();
    let chord_lock = RelationEdit::Insert(rid(2), RelationSpec::FixLine(lid(9)));
    let edits = [radius(5.), chord_lock];
    assert!(matches!(
        sketch.apply(&edits, || false),
        Err(ConstraintFailure::Arc(id, ArcConstraintFailure::LockedCrossing(points)))
            if id == aid() && points == [pid(1), pid(2)]
    ));
    assert!(matches!(
        ConstrainedSketch::restore(
            geometry.clone(),
            &[(rid(1), recorded(5.)), (rid(2), Relation::FixLine(lid(9), [point(S), point([5., 0., 0.])]))],
            policy(),
            || false,
        ),
        Err(ConstraintFailure::Arc(id, ArcConstraintFailure::LockedCrossing(_))) if id == aid()
    ));
    assert_eq!(sketch.geometry(), &geometry);
    assert_eq!(sketch.relations().count(), 0);
}

#[test]
fn a_bare_half_turn_keeps_its_branch_and_other_arc_dimensions_keep_their_domain() {
    let geometry = shape(5., 5.)
        .apply(
            &[seed(4, [-5., -2., 0.], PointRole::Defining), line(9, 4, 1)],
            || false,
        )
        .unwrap();
    let sketch = ConstrainedSketch::new(geometry.clone(), policy()).unwrap();
    assert!(matches!(
        sketch.transform_points(&[pid(3)], RigidMotion::Translate(Vector3::new(0., -1., 0.)), || false),
        Err(RigidMotionFailure::Constraint(ConstraintFailure::Arc(id,
            ArcConstraintFailure::HalfTurnDeparture(ArcBranch::Minor)))) if id == aid()
    ));
    let tangent = LineArcJoin {
        line: lid(9),
        arc: aid(),
        endpoint: pid(1),
    };
    for spec in [
        RelationSpec::FixArc(aid()),
        RelationSpec::Tangent(tangent.into()),
    ] {
        assert!(matches!(
            sketch.apply(&[RelationEdit::Insert(rid(1), spec)], || false),
            Err(ConstraintFailure::Arc(id, ArcConstraintFailure::Semicircle)) if id == aid()
        ));
    }
    assert_eq!(sketch.geometry(), &geometry);
}

#[test]
fn crossing_permission_refuses_aliases_forged_nonradius_and_conflicting_strict_policies() {
    let first = shape(4., 2.);
    let restored =
        ConstrainedSketch::restore(first.clone(), &[(rid(1), recorded(5.))], policy(), || false)
            .unwrap();
    let strict = RelationEdit::Insert(rid(2), RelationSpec::Radius(aid(), length(5.)));
    assert!(matches!(
        restored.apply(&[strict], || false),
        Err(ConstraintFailure::Arc(id, ArcConstraintFailure::ConflictingSweep(ArcBranch::Minor))) if id == aid()
    ));
    let fixed = Relation::FixArc(
        aid(),
        first.arc(aid()).unwrap().points(),
        ArcBranch::CrossHalfTurn,
    );
    assert!(matches!(
        ConstrainedSketch::restore(first, &[(rid(1), fixed)], policy(), || false),
        Err(ConstraintFailure::Arc(id, ArcConstraintFailure::CrossingRequiresRadius)) if id == aid()
    ));

    let half_turn = shape(5., 5.);
    let sibling = ArcId::new(NonZeroU64::new(10).unwrap());
    let alias = SketchArc {
        id: sibling,
        endpoints: [pid(2), pid(1)],
        ..half_turn.arc(aid()).unwrap().recipe()
    };
    let aliased = half_turn
        .apply(&[Edit::InsertArc(alias)], || false)
        .unwrap();
    let bound = ConstrainedSketch::new(aliased, policy()).unwrap();
    assert!(matches!(
        bound.apply(&[radius(5.)], || false),
        Err(ConstraintFailure::Arc(id, ArcConstraintFailure::AliasedCrossing(alias))) if id == aid() && alias == sibling
    ));
}

#[test]
fn half_turn_admission_requires_the_exact_authored_and_native_sweep_classes_to_agree() {
    let points = [[-2., -23., 0.], [2., 23., 0.], [-23., 2., 0.]];
    let geometry = create(points, through(), 1e-9).unwrap();
    let authored = ArcTripleMetrics::new(points).unwrap();
    assert_eq!(authored.branch(), Ordering::Equal);
    assert_eq!(authored.midpoint_offset(0.), Ok(Ordering::Equal));
    let sweep = geometry.arc(aid()).unwrap().range()[1];
    let result = ConstrainedSketch::new(geometry.clone(), policy());
    if sweep == PI {
        assert_eq!(result.unwrap().geometry(), &geometry);
    } else {
        assert!(matches!(
            result,
            Err(ConstraintFailure::Arc(id, ArcConstraintFailure::HalfTurnCarrier(actual)))
                if id == aid() && actual == sweep
        ));
    }
}

#[test]
fn crossing_requires_a_free_chord_even_when_reopened_off_the_half_turn() {
    let geometry = shape(4., 8.).apply(&[line(9, 1, 2)], || false).unwrap();
    let arc = geometry.arc(aid()).unwrap();
    let fixed = (
        rid(2),
        Relation::FixLine(lid(9), [arc.points()[0], arc.points()[1]]),
    );
    assert!(matches!(
        ConstrainedSketch::restore(geometry.clone(), &[(rid(1), recorded(5.)), fixed], policy(), || false),
        Err(ConstraintFailure::Arc(id, ArcConstraintFailure::LockedCrossing(_))) if id == aid()
    ));
    let strict = Relation::Radius(aid(), length(5.), ArcBranch::Major);
    let locked =
        ConstrainedSketch::restore(geometry, &[(rid(1), strict), fixed], policy(), || false)
            .unwrap();
    assert_eq!(locked.inspect_rank().unwrap().degrees_of_freedom, 1);
}

#[test]
fn a_chord_length_or_coincident_endpoint_cannot_indirectly_lock_a_crossing() {
    let geometry = shape(5., 5.)
        .apply(
            &[line(9, 1, 2), seed(4, S, PointRole::SketchMarker)],
            || false,
        )
        .unwrap();
    let sketch = ConstrainedSketch::new(geometry.clone(), policy()).unwrap();
    for spec in [
        RelationSpec::Length(lid(9), length(10.)),
        RelationSpec::Coincident([pid(1), pid(4)]),
    ] {
        let coupling = RelationEdit::Insert(rid(2), spec);
        assert!(matches!(
            sketch.apply(&[radius(5.), coupling], || false),
            Err(ConstraintFailure::Arc(id, ArcConstraintFailure::CrossingCoupledRelation(relation)))
                if id == aid() && relation == rid(2)
        ));
    }
    assert!(matches!(
        sketch.apply(&[radius(5.), RelationEdit::Insert(rid(2), RelationSpec::FixPoint(pid(3)))], || false),
        Err(ConstraintFailure::Arc(id, ArcConstraintFailure::CrossingPinnedWitness(point)))
            if id == aid() && point == pid(3)
    ));
    assert_eq!(sketch.geometry(), &geometry);
}

#[test]
fn crossing_rejects_an_incident_curve_other_than_the_chord() {
    let geometry = shape(5., 5.)
        .apply(
            &[seed(4, [-6., -1., 0.], PointRole::Defining), line(9, 1, 4)],
            || false,
        )
        .unwrap();
    let sketch = ConstrainedSketch::new(geometry, policy()).unwrap();
    assert!(matches!(
        sketch.apply(&[radius(5.)], || false),
        Err(ConstraintFailure::Arc(id, ArcConstraintFailure::CrossingSharedEntity(EntityId::Line(line))))
            if id == aid() && line == lid(9)
    ));
}

#[test]
fn rigid_edits_preserve_sweep_and_removing_radius_retires_crossing_permission() {
    let geometry = shape(4., 8.);
    let restored = ConstrainedSketch::restore(
        geometry.clone(),
        &[(rid(1), recorded(5.))],
        policy(),
        || false,
    )
    .unwrap();
    let direct = restored.transform_points(
        &[pid(3)],
        RigidMotion::Translate(Vector3::new(0., -6., 0.)),
        || false,
    );
    assert!(matches!(direct,
        Err(RigidMotionFailure::Constraint(ConstraintFailure::Arc(id, ArcConstraintFailure::BranchChanged {
            expected: ArcBranch::Major, actual: ArcBranch::Minor,
        }))) if id == aid()
    ));
    assert_eq!(restored.geometry(), &geometry);

    let ordinary = restored
        .apply(&[RelationEdit::Remove(rid(1))], || false)
        .unwrap();
    let relation = ordinary.apply(&[radius(5.)], || false).unwrap();
    assert_eq!(
        relation.relations().next(),
        Some((
            rid(1),
            Relation::Radius(aid(), length(5.), ArcBranch::Major)
        ))
    );
}
