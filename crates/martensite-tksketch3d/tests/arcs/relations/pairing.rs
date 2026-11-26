// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tkmath::gp::Vector3;
use martensite_tksketch3d::constraints::{RigidMotion, RigidMotionFailure};

fn second_arc() -> ArcId {
    ArcId::new(NonZeroU64::new(9).unwrap())
}

#[test]
fn equal_radius_drives_a_major_arc_from_a_fixed_independent_minor_circle() {
    let geometry = two_circles();
    let mut limits = policy();
    limits.preview.block_inputs = 18;
    let sketch = ConstrainedSketch::new(geometry.clone(), limits).unwrap();
    let fixed_circle = RelationEdit::Insert(rid(1), RelationSpec::FixArc(aid()));
    let relation = RelationEdit::Insert(rid(4), RelationSpec::EqualRadius([aid(), second_arc()]));
    let batch = [
        fixed_circle,
        RelationEdit::Insert(rid(2), RelationSpec::FixPoint(pid(4))),
        RelationEdit::Insert(rid(3), RelationSpec::FixPoint(pid(5))),
        relation,
    ];
    let updated = sketch.apply(&batch, || false).unwrap();
    assert_eq!(updated.geometry().arc(aid()), geometry.arc(aid()));
    let actual = updated.geometry().arc(second_arc()).unwrap();
    let [x, y, z] = *actual.points()[2].coordinates();
    assert!((x.abs() + (y - 41.).abs() + (z - 38.).abs()) <= 2e-7);
    assert!((actual.radius() - 5.).abs() <= 2e-7);
    assert!(actual.range()[1] > PI);
    assert_eq!(
        updated.relations().last(),
        Some((
            rid(4),
            Relation::EqualRadius([aid(), second_arc()], [ArcBranch::Minor, ArcBranch::Major])
        ))
    );
    let rank = updated.inspect_rank().unwrap();
    assert_eq!(
        (rank.rank, rank.degrees_of_freedom, rank.equations),
        (17, 1, 15)
    );

    let retained = updated.relations().collect::<Vec<_>>();
    let restored =
        ConstrainedSketch::restore(updated.geometry().clone(), &retained, limits, || false)
            .unwrap();
    assert_eq!(restored.relations().collect::<Vec<_>>(), retained);
    assert_eq!(restored.geometry(), updated.geometry());
}

fn two_circles() -> Snapshot {
    layout(2.)
        .apply(
            &[
                seed(4, [0., 37., 30.], PointRole::Defining),
                seed(5, [0., 45., 30.], PointRole::Defining),
                seed(6, [0., 41., 36.], PointRole::SketchMarker),
                Edit::InsertArc(SketchArc {
                    id: second_arc(),
                    endpoints: [pid(4), pid(5)],
                    definition: ArcDefinition::Through { witness: pid(6) },
                    fit_tolerance: ArcFitTolerance::new(1e-9).unwrap(),
                }),
            ],
            || false,
        )
        .unwrap()
}

#[test]
fn equal_radius_respects_the_caller_block_width_and_checks_replay_against_both_circles() {
    let geometry = two_circles();
    let mut limits = policy();
    limits.preview.block_inputs = 17;
    let sketch = ConstrainedSketch::new(geometry.clone(), limits).unwrap();
    let operation = RelationSpec::EqualRadius([aid(), second_arc()]);
    assert!(matches!(
        sketch.apply(&[RelationEdit::Insert(rid(1), operation)], || false),
        Err(ConstraintFailure::Preview(None, _))
    ));
    limits.preview.block_inputs = 18;
    let impossible =
        Relation::EqualRadius([aid(), second_arc()], [ArcBranch::Minor, ArcBranch::Major]);
    assert!(matches!(
        ConstrainedSketch::restore(geometry, &[(rid(1), impossible)], limits, || false),
        Err(ConstraintFailure::Infeasible(id, relation)) if id == rid(1) && relation == impossible
    ));
}

#[test]
fn reversed_shared_arc_triangles_get_one_intrinsic_row_and_no_redundant_radius_row() {
    let original = layout(2.);
    let reversed = SketchArc {
        id: second_arc(),
        endpoints: [pid(2), pid(1)],
        ..original.arc(aid()).unwrap().recipe()
    };
    let source = original
        .apply(&[Edit::InsertArc(reversed)], || false)
        .unwrap();
    let sketch = ConstrainedSketch::new(source, policy()).unwrap();
    let equality = RelationEdit::Insert(rid(1), RelationSpec::EqualRadius([aid(), second_arc()]));
    let constrained = sketch.apply(&[equality], || false).unwrap();
    assert_eq!(constrained.relations().count(), 1);
    let moved = constrained
        .drag(pid(3), point([7.2, 0., 2.]), || false)
        .unwrap();
    assert_eq!(
        (
            moved.rank.rank,
            moved.rank.degrees_of_freedom,
            moved.rank.equations
        ),
        (1, 8, 0)
    );
    assert_eq!(
        moved.sketch.geometry().arc(aid()).unwrap().radius(),
        moved.sketch.geometry().arc(second_arc()).unwrap().radius()
    );
}

#[test]
fn fix_arc_anchors_all_exact_points_with_eight_authored_rows() {
    let geometry = layout(8.);
    let sketch = ConstrainedSketch::new(geometry.clone(), policy())
        .unwrap()
        .apply(
            &[RelationEdit::Insert(rid(1), RelationSpec::FixArc(aid()))],
            || false,
        )
        .unwrap();
    let rank = sketch.inspect_rank().unwrap();
    assert_eq!(
        (rank.rank, rank.degrees_of_freedom, rank.equations),
        (9, 0, 8)
    );
    let anchors = geometry.arc(aid()).unwrap().points();
    let expected = Relation::FixArc(aid(), anchors, ArcBranch::Major);
    assert_eq!(sketch.relations().next(), Some((rid(1), expected)));
    assert_eq!(sketch.geometry(), &geometry);

    let moved = sketch
        .drag_preferred(pid(3), point([7.5, 0., 7.]), 1e-8, || false)
        .unwrap();
    assert_eq!(moved.update.sketch.geometry(), &geometry);
    let translation = RigidMotion::Translate(Vector3::new(0., 0., 1.));
    assert!(matches!(
        sketch.transform_points(&[pid(1), pid(2), pid(3)], translation, || false),
        Err(RigidMotionFailure::Constraint(ConstraintFailure::Infeasible(id, relation)))
            if id == rid(1) && relation == expected
    ));

    let drifted = [anchors[0], anchors[1], point([7., 0., 8.01])];
    assert!(matches!(
        ConstrainedSketch::restore(
            geometry, &[(rid(1), Relation::FixArc(aid(), drifted, ArcBranch::Major))], policy(), || false
        ),
        Err(ConstraintFailure::Infeasible(id, _)) if id == rid(1)
    ));
}
