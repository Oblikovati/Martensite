// SPDX-License-Identifier: GPL-2.0-only
use super::{coords, insertion, pid, point, policy, rid, seed};
use martensite_tksketch3d::{
    ControlSplineId, ControlSplineProofPolicy, Edit, MinimumLength, PointRole, SketchControlSpline,
    Snapshot,
    constraints::{
        ConstrainedSketch, ConstraintFailure, ContinuityJoin, CubicCubicJoin, CurvatureTolerance,
        Relation, RelationEdit, RelationSpec,
    },
};
use martensite_tksolve::preview::{Incomplete, Limit, PreviewError};
use std::num::NonZeroU64;

#[path = "pair_replay.rs"]
mod replay;

fn join() -> ContinuityJoin {
    ContinuityJoin::CubicCubic(CubicCubicJoin {
        reference: ControlSplineId::new(NonZeroU64::new(10).unwrap()),
        adjusted: ControlSplineId::new(NonZeroU64::new(11).unwrap()),
        endpoint: pid(2),
    })
}

fn sample(left: [[f64; 3]; 3], right: [[f64; 3]; 3], reverse: [bool; 2]) -> Snapshot {
    let ContinuityJoin::CubicCubic(join) = join() else {
        unreachable!()
    };
    let mut edits = vec![seed(2, [0.; 3], PointRole::Defining)];
    for (index, xyz) in [1, 3, 4, 5, 6, 7]
        .into_iter()
        .zip(left.into_iter().chain(right))
    {
        edits.push(seed(index, xyz, PointRole::Defining));
    }
    for ((id, mut poles), reversed) in [
        (join.reference, [2, 1, 3, 4]),
        (join.adjusted, [2, 5, 6, 7]),
    ]
    .into_iter()
    .zip(reverse)
    {
        if reversed {
            poles.reverse();
        }
        edits.push(Edit::InsertControlSpline(SketchControlSpline {
            id,
            poles: poles.map(pid),
            proof: ControlSplineProofPolicy::new(1e-8, 128).unwrap(),
        }));
    }
    Snapshot::empty(MinimumLength::new(0.001).unwrap())
        .apply(&edits, || false)
        .unwrap()
}

fn policy_15() -> martensite_tksketch3d::constraints::ConstraintPolicy {
    let mut bounded = policy();
    bounded.preview.block_inputs = 15;
    bounded
}

#[test]
fn paired_cubic_g2_establishes_different_speed_stencils_without_moving_reference_controls() {
    for reversal in [[true, false], [false, true]] {
        let shape = sample(
            [[-1., 0., 0.], [-2., 1., 0.], [-3., 1., 1.]],
            [[2., 0.25, 0.15], [4., 1., 1.], [6., 4., 2.]],
            reversal,
        );
        let base = ConstrainedSketch::new(shape.clone(), policy_15()).unwrap();
        let g2 = RelationSpec::G2(join(), CurvatureTolerance::new(2e-7).unwrap());
        let accepted = base.apply(&insertion(g2), || false).unwrap();
        for index in [1, 2, 3, 4, 7] {
            assert_eq!(
                coords(accepted.geometry(), pid(index)),
                coords(&shape, pid(index))
            );
        }
        assert_ne!(coords(accepted.geometry(), pid(6)), coords(&shape, pid(6)));
        let rank = accepted.inspect_rank().unwrap();
        assert_eq!(
            (rank.rank, rank.equations, rank.degrees_of_freedom),
            (4, 4, 17)
        );
        let target = point([-1.2, 0., 0.]);
        let moved = accepted.drag(pid(1), target, || false).unwrap();
        let selected = moved.sketch.geometry().point(pid(1)).unwrap().position;
        assert!(selected.distance(&target) < 1e-8);
        assert_eq!(moved.rank, moved.sketch.inspect_rank().unwrap());
        assert_eq!(base.geometry(), &shape);
    }
}

#[test]
fn paired_cubic_g2_uses_a_vector_difference_and_therefore_checks_curvature_orientation() {
    let original = sample(
        [[-1., 0., 0.], [-2., 1., 0.], [-3., 1., 1.]],
        [[2., 0., 0.], [4., 4., 0.], [6., 4., 2.]],
        [true, false],
    );
    let zero = CurvatureTolerance::new(0.).unwrap();
    let persisted = [(rid(1), Relation::G2(join(), zero))];
    let restored =
        ConstrainedSketch::restore(original.clone(), &persisted, policy_15(), || false).unwrap();
    assert_eq!(restored.geometry(), &original);

    let mirrored = sample(
        [[-1., 0., 0.], [-2., 1., 0.], [-3., 1., 1.]],
        [[2., 0., 0.], [4., -4., 0.], [6., -4., 2.]],
        [true, false],
    );
    let boundary = |cap| {
        ConstrainedSketch::restore(
            mirrored.clone(),
            &[(
                rid(1),
                Relation::G2(join(), CurvatureTolerance::new(cap).unwrap()),
            )],
            policy_15(),
            || false,
        )
    };
    let below = 4_f64 / 3.;
    assert!(matches!(boundary(below), Err(ConstraintFailure::Infeasible(id, _)) if id == rid(1)));
    assert_eq!(boundary(below.next_up()).unwrap().geometry(), &mirrored);
}

#[test]
fn g2_uses_fifteen_distinct_slots_and_refuses_a_smaller_structural_budget() {
    let shape = sample(
        [[-1., 0., 0.], [-2., 1., 0.], [-3., 1., 1.]],
        [[2., 0., 0.], [4., 4., 0.], [6., 4., 2.]],
        [true, false],
    );
    let mut limit = policy_15();
    limit.preview.block_inputs = 14;
    let base = ConstrainedSketch::new(shape.clone(), limit).unwrap();
    let tangent = base
        .apply(&insertion(RelationSpec::Tangent(join())), || false)
        .unwrap();
    assert_eq!(tangent.inspect_rank().unwrap().equations, 2);
    assert!(matches!(
        base.apply(
            &insertion(RelationSpec::G2(
                join(),
                CurvatureTolerance::new(1e-7).unwrap()
            )),
            || false
        ),
        Err(ConstraintFailure::Preview(
            None,
            PreviewError::Incomplete(Incomplete::Limit {
                quantity: Limit::BlockInputs,
                limit: 14,
                requested: 15
            })
        ))
    ));
    assert_eq!(base.geometry(), &shape);
    assert_eq!(base.relations().count(), 0);
}

#[test]
fn paired_g2_refreshes_the_moving_reference_cubic_chart() {
    let shape = sample(
        [[-1., 0., 0.], [-2., 0., 0.], [-3., 1., 1.]],
        [[2., 0., 0.], [4., 0., 0.], [6., 4., 2.]],
        [true, false],
    );
    let base = ConstrainedSketch::new(shape, policy_15()).unwrap();
    let mut sketch = base
        .apply(
            &[
                RelationEdit::Insert(rid(2), RelationSpec::FixPoint(pid(2))),
                insertion(RelationSpec::G2(
                    join(),
                    CurvatureTolerance::new(1e-7).unwrap(),
                ))[0],
            ],
            || false,
        )
        .unwrap();
    for point_on_drag in [[-0.8, -0.6, 0.], [-0.6, -0.8, 0.], [-0.28, -0.96, 0.]] {
        let target = point(point_on_drag);
        let updated = sketch.drag(pid(1), target, || false).unwrap();
        let moved = updated.sketch.geometry().point(pid(1)).unwrap().position;
        assert!(moved.distance(&target) < 2e-7);
        assert_eq!(updated.rank, updated.sketch.inspect_rank().unwrap());
        assert_eq!(updated.rank.rank, 7);
        sketch = updated.sketch;
    }
}

#[test]
#[ignore = "release receipt for active cubic-pair endpoint moves"]
fn paired_cubic_active_warm_p95() {
    let shape = sample(
        [[-1., 0., 0.], [-2., 1., 0.], [-3., 1., 1.]],
        [[2., 0., 0.], [4., 4., 0.], [6., 4., 2.]],
        [true, false],
    );
    let base = ConstrainedSketch::new(shape, policy_15()).unwrap();
    let tangent = base
        .apply(&insertion(RelationSpec::Tangent(join())), || false)
        .unwrap();
    let cap = CurvatureTolerance::new(1e-7).unwrap();
    let g2 = base
        .apply(&insertion(RelationSpec::G2(join(), cap)), || false)
        .unwrap();
    super::bench::matched_moves([&base, &tangent, &g2], pid(1), 2e-7, |sample| {
        let step = f64::from(sample % 17) / 2000.;
        [-1.02 - step, -0.02 + 2. * step, 0.01 + step]
    });
}
