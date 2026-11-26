// SPDX-License-Identifier: GPL-2.0-only
mod pairing;
mod semicircle;

use super::*;
use crate::constraints::{length, policy, rid};
use martensite_tksketch3d::constraints::{
    ArcBranch, ArcConstraintFailure, ConstrainedSketch, ConstraintFailure, ConstraintRank,
    Relation, RelationEdit, RelationSpec,
};

const START: [f64; 3] = [7., -4., 0.];
const END: [f64; 3] = [7., 4., 0.];

fn layout(height: f64) -> Snapshot {
    create([START, END, [7., 0., height]], through(), 1e-9).unwrap()
}

fn radius_batch(target: f64) -> [RelationEdit; 3] {
    [
        RelationEdit::Insert(rid(1), RelationSpec::FixPoint(pid(1))),
        RelationEdit::Insert(rid(2), RelationSpec::FixPoint(pid(2))),
        RelationEdit::Insert(rid(3), RelationSpec::Radius(aid(), length(target))),
    ]
}

#[test]
fn radius_with_fixed_endpoints_retains_both_spatial_branches_and_point_identities() {
    for (seed_height, branch, sign) in [(2., ArcBranch::Minor, -1.), (8., ArcBranch::Major, 1.)] {
        let original = layout(seed_height);
        let sketch = ConstrainedSketch::new(original.clone(), policy()).unwrap();
        assert_eq!(
            sketch.inspect_rank().unwrap(),
            ConstraintRank {
                rank: 1,
                degrees_of_freedom: 8,
                equations: 0,
                frozen_coordinates: 0
            }
        );
        let updated = sketch.apply(&radius_batch(6.), || false).unwrap();
        let arc = updated.geometry().arc(aid()).unwrap();
        let height = 6. + sign * 20.0_f64.sqrt();
        assert_eq!(arc.recipe(), original.arc(aid()).unwrap().recipe());
        assert_eq!(&arc.points()[..2], &[point(START), point(END)]);
        let witness = *arc.points()[2].coordinates();
        for (actual, expected) in witness.into_iter().zip([7., 0., height]) {
            assert!((actual - expected).abs() <= 2e-7, "{witness:?}");
        }
        assert!((arc.radius() - 6.).abs() <= 2e-7);
        assert_eq!(arc.range()[1] < PI, branch == ArcBranch::Minor);
        assert_eq!(
            updated.relations().last(),
            Some((rid(3), Relation::Radius(aid(), length(6.), branch)))
        );
        assert_eq!(
            updated.inspect_rank().unwrap(),
            ConstraintRank {
                rank: 8,
                degrees_of_freedom: 1,
                equations: 7,
                frozen_coordinates: 0
            }
        );
        assert_eq!(sketch.geometry(), &original);
    }
}

#[test]
fn warm_radius_drag_rotates_the_sagitta_while_preserving_the_model_circle() {
    let sketch = ConstrainedSketch::new(layout(2.), policy())
        .unwrap()
        .apply(&radius_batch(5.), || false)
        .unwrap();
    let (sine, cosine) = martensite_cmath::sincos(0.1);
    let target = point([7. + 2. * sine, 0., 2. * cosine]);
    let moved = sketch.drag(pid(3), target, || false).unwrap();
    let arc = moved.sketch.geometry().arc(aid()).unwrap();
    let [x, y, z] = *arc.points()[2].coordinates();
    assert!(x > 7.15);
    assert!(y.abs() <= 2e-7);
    assert!((martensite_cmath::hypot(x - 7., z) - 2.).abs() <= 2e-7);
    assert!((arc.radius() - 5.).abs() <= 2e-7);
    assert_eq!(moved.rank.degrees_of_freedom, 1);
    assert!(moved.residual_work_bound <= policy().residual_work);
    let persisted = moved.sketch.relations().collect::<Vec<_>>();
    let restored = ConstrainedSketch::restore(
        moved.sketch.geometry().clone(),
        &persisted,
        policy(),
        || false,
    )
    .unwrap();
    assert_eq!(restored.geometry(), moved.sketch.geometry());
    assert_eq!(restored.relations().collect::<Vec<_>>(), persisted);
}

#[test]
fn unsupported_witnesses_keep_their_standalone_geometry_and_typed_layout_refusal() {
    let cases = [
        (
            create([START, END, [7., 1., 2.]], through(), 1e-9).unwrap(),
            ArcConstraintFailure::WitnessOffset(policy().length_tolerance),
        ),
        (
            create(
                [START, END, [7., 0., 0.]],
                centered(Direction3::new(1., 0., 0.).unwrap()),
                1e-9,
            )
            .unwrap(),
            ArcConstraintFailure::CenterDefinition,
        ),
    ];
    for (source, expected) in cases {
        assert!(source.arc(aid()).is_some());
        assert!(matches!(ConstrainedSketch::new(source, policy()),
                Err(ConstraintFailure::Arc(id, cause)) if id == aid() && cause == expected));
    }
}

#[test]
fn invalid_or_cancelled_radius_requests_do_not_change_the_source() {
    let original = layout(2.);
    let sketch = ConstrainedSketch::new(original.clone(), policy()).unwrap();
    assert!(matches!(
        sketch.apply(&radius_batch(0.001), || false),
        Err(ConstraintFailure::RadiusBelowMinimum(id, arc, value))
            if id == rid(3) && arc == aid() && value == 0.001
    ));
    assert!(sketch.apply(&radius_batch(3.), || false).is_err());
    let mut polls = 0;
    assert!(matches!(
        sketch.apply(&radius_batch(6.), || {
            polls += 1;
            polls == 8
        }),
        Err(ConstraintFailure::Cancelled)
    ));
    assert_eq!(sketch.geometry(), &original);
    assert_eq!(sketch.relations().count(), 0);
}

#[test]
fn restored_radius_rejects_a_forged_branch_or_radius_without_rewriting_source() {
    let geometry = layout(2.);
    let wrong_branch = [(
        rid(1),
        Relation::Radius(aid(), length(5.), ArcBranch::Major),
    )];
    assert!(matches!(
        ConstrainedSketch::restore(geometry.clone(), &wrong_branch, policy(), || false),
        Err(ConstraintFailure::Arc(id, ArcConstraintFailure::BranchChanged {
            expected: ArcBranch::Major,
            actual: ArcBranch::Minor
        })) if id == aid()
    ));
    let wrong_radius = Relation::Radius(aid(), length(6.), ArcBranch::Minor);
    assert!(matches!(
        ConstrainedSketch::restore(geometry, &[(rid(1), wrong_radius)], policy(), || false),
        Err(ConstraintFailure::Infeasible(id, relation)) if id == rid(1) && relation == wrong_radius
    ));
}

#[test]
fn intrinsic_and_authored_radius_rows_count_against_prepared_limits() {
    let mut limits = policy();
    limits.preview.equations = 1;
    let prepared = ConstrainedSketch::new(layout(2.), limits).unwrap();
    assert!(matches!(
        prepared.apply(&radius_batch(6.), || false),
        Err(ConstraintFailure::Preview(None, _))
    ));
    limits.preview.block_inputs = 8;
    assert!(matches!(
        ConstrainedSketch::new(layout(2.), limits),
        Err(ConstraintFailure::Preview(None, _))
    ));
}

#[test]
fn direct_placement_cannot_exchange_equal_radius_minor_and_major_branches() {
    use martensite_tkmath::gp::Vector3;
    use martensite_tksketch3d::constraints::{RigidMotion, RigidMotionFailure};

    let original = layout(2.);
    let sketch = ConstrainedSketch::new(original.clone(), policy()).unwrap();
    let motion = RigidMotion::Translate(Vector3::new(0., 0., 6.));
    assert!(matches!(
        sketch.transform_points(&[pid(3)], motion, || false),
        Err(RigidMotionFailure::Constraint(ConstraintFailure::Arc(
            id,
            ArcConstraintFailure::BranchChanged {
                expected: ArcBranch::Minor,
                actual: ArcBranch::Major,
            }
        ))) if id == aid()
    ));
    assert_eq!(sketch.geometry(), &original);
}

#[test]
fn a_line_can_share_a_rigid_arc_endpoint_without_extra_point_slots() {
    let geometry = layout(2.)
        .apply(
            &[seed(4, [11., 4., 0.], PointRole::Defining), line(9, 2, 4)],
            || false,
        )
        .unwrap();
    let sketch = ConstrainedSketch::new(geometry, policy()).unwrap();
    let updates = [
        RelationEdit::Insert(rid(1), RelationSpec::Radius(aid(), length(5.))),
        RelationEdit::Insert(rid(2), RelationSpec::Length(lid(9), length(4.))),
    ];
    let accepted = sketch.apply(&updates, || false).unwrap();
    let rank = accepted.inspect_rank().unwrap();
    assert_eq!(
        (rank.rank, rank.degrees_of_freedom, rank.equations),
        (3, 9, 2)
    );
    let arc = accepted.geometry().arc(aid()).unwrap();
    let segment = accepted.geometry().line(lid(9)).unwrap();
    assert_eq!(arc.recipe().endpoints[1], segment.line.endpoints[0]);
    assert_eq!(arc.points()[1], segment.points[0]);
}

#[test]
fn preferred_arc_drag_projects_an_off_plane_cursor_onto_the_sagitta_circle() {
    let sketch = ConstrainedSketch::new(layout(2.), policy())
        .unwrap()
        .apply(&radius_batch(5.), || false)
        .unwrap();
    let (sine, cosine) = martensite_cmath::sincos(0.1);
    let cursor = point([7. + 3. * sine, 0.3, 3. * cosine]);
    let moved = sketch
        .drag_preferred(pid(3), cursor, 1e-8, || false)
        .unwrap();
    let arc = moved.update.sketch.geometry().arc(aid()).unwrap();
    let expected = [7. + 2. * sine, 0., 2. * cosine];
    for (&actual, expected) in arc.points()[2].coordinates().iter().zip(expected) {
        assert!((actual - expected).abs() <= 2e-7);
    }
    assert!(moved.preference.max_tangential_gradient <= 1e-8);
    assert_eq!(moved.update.rank.degrees_of_freedom, 1);
    assert!(moved.update.residual_work_bound <= policy().residual_work);
}
