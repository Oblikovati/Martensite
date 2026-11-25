// SPDX-License-Identifier: GPL-2.0-only
use crate::{
    constraints::{policy, rid},
    fixture::{lid, line, pid, point, seed},
};
use martensite_tksketch3d::{
    ControlSplineId, ControlSplineProofPolicy, Edit, MinimumLength, PointId, PointRole,
    SketchControlSpline, Snapshot,
    constraints::{
        ConstrainedSketch, ConstraintFailure, ContinuityFailure, CurvatureTolerance, LineCubicJoin,
        Relation, RelationEdit, RelationSpec,
    },
};
use martensite_tksolve::preview::{Incomplete, Limit, PreviewError};
use std::num::NonZeroU64;

#[path = "continuity/bench.rs"]
mod bench;

fn join() -> LineCubicJoin {
    LineCubicJoin {
        line: lid(10),
        cubic: ControlSplineId::new(NonZeroU64::new(11).unwrap()),
        endpoint: pid(2),
    }
}

fn source(line_end: [f64; 3], controls: [[f64; 3]; 3], reversed: bool) -> Snapshot {
    let mut poles = [2, 3, 4, 5].map(pid);
    let incoming = if reversed {
        poles.reverse();
        line(10, 2, 1)
    } else {
        line(10, 1, 2)
    };
    let [a, b, c] = controls;
    Snapshot::empty(MinimumLength::new(0.001).unwrap())
        .apply(
            &[
                seed(1, line_end, PointRole::Defining),
                seed(2, [0.; 3], PointRole::Defining),
                seed(3, a, PointRole::Defining),
                seed(4, b, PointRole::Defining),
                seed(5, c, PointRole::Defining),
                incoming,
                Edit::InsertControlSpline(SketchControlSpline {
                    id: join().cubic,
                    poles,
                    proof: ControlSplineProofPolicy::new(1e-8, 128).unwrap(),
                }),
            ],
            || false,
        )
        .unwrap()
}

fn insertion(spec: RelationSpec) -> [RelationEdit; 1] {
    [RelationEdit::Insert(rid(1), spec)]
}

fn coords(geometry: &Snapshot, id: PointId) -> [f64; 3] {
    *geometry.point(id).unwrap().position.coordinates()
}

fn close(value: f64, expected: f64, tolerance: f64) {
    assert!(
        (value - expected).abs() <= tolerance,
        "{value} vs {expected}"
    );
}

#[test]
fn tangent_keeps_the_reference_line_during_insertion_in_both_traversals() {
    for reversed in [false, true] {
        let original = source(
            [-1., 0., 0.],
            [[1., 0.2, 0.1], [2., 1., 0.], [3., 1., 0.]],
            reversed,
        );
        let base = ConstrainedSketch::new(original.clone(), policy()).unwrap();
        let accepted = base
            .apply(&insertion(RelationSpec::Tangent(join())), || false)
            .unwrap();
        assert_eq!(
            accepted.geometry().line(join().line),
            original.line(join().line)
        );
        let [x, y, z] = coords(accepted.geometry(), pid(3));
        assert!(x > 0.);
        close(y, 0., 1e-8);
        close(z, 0., 1e-8);
        // Tangency uses only the control adjacent to the shared endpoint.
        for id in [pid(4), pid(5)] {
            assert_eq!(coords(accepted.geometry(), id), coords(&original, id));
        }
        let analysis = accepted.inspect_rank().unwrap();
        assert_eq!(
            (
                analysis.rank,
                analysis.equations,
                analysis.degrees_of_freedom
            ),
            (2, 2, 13)
        );
        assert_eq!(base.geometry(), &original);
    }
}

#[test]
fn exact_line_cubic_g2_restore_enforces_the_closed_inverse_length_cap() {
    let shape = source(
        [-1., 0., 0.],
        [[1., 0., 0.], [2., 1., 0.], [3., 1., 0.]],
        false,
    );
    let run = |cap| {
        let relation = Relation::G2(join(), CurvatureTolerance::new(cap).unwrap());
        ConstrainedSketch::restore(shape.clone(), &[(rid(1), relation)], policy(), || false)
    };
    let below = 2. / 3.;
    assert!(matches!(run(below), Err(ConstraintFailure::Infeasible(id, _)) if id == rid(1)));
    assert_eq!(run(below.next_up()).unwrap().geometry(), &shape);
    let half = source(
        [-1., 0., 0.],
        [[2., 0., 0.], [4., 3., 0.], [6., 3., 0.]],
        false,
    );
    let exact = Relation::G2(join(), CurvatureTolerance::new(0.5).unwrap());
    assert_eq!(
        ConstrainedSketch::restore(half.clone(), &[(rid(1), exact)], policy(), || false)
            .unwrap()
            .geometry(),
        &half
    );
    assert!(matches!(
        CurvatureTolerance::new(-0.5),
        Err(ContinuityFailure::InvalidCurvatureTolerance(value)) if value == -0.5
    ));
}

#[test]
fn establishes_g2_from_curved_seed_then_transient_line_anchors_do_not_block_a_drag() {
    let original = source(
        [-1., 0., 0.],
        [[1., 0.1, 0.2], [2., 1., 0.1], [3., 1., 0.]],
        false,
    );
    let base = ConstrainedSketch::new(original.clone(), policy()).unwrap();
    let recipe = RelationSpec::G2(join(), CurvatureTolerance::new(1e-7).unwrap());
    let edits = insertion(recipe);
    let constrained = base.apply(&edits, || false).unwrap();
    assert_eq!(
        constrained.geometry().line(join().line),
        original.line(join().line)
    );
    for id in [pid(3), pid(4)] {
        let [_, y, z] = coords(constrained.geometry(), id);
        close(y, 0., 1e-7);
        close(z, 0., 1e-7);
    }
    assert_eq!(
        coords(constrained.geometry(), pid(5)),
        coords(&original, pid(5)),
    );
    let rank = constrained.inspect_rank().unwrap();
    assert_eq!(
        (rank.rank, rank.equations, rank.degrees_of_freedom),
        (4, 4, 11)
    );
    let moved = constrained
        .drag(pid(1), point([-1.4, 0., 0.]), || false)
        .unwrap();
    assert_eq!(moved.rank, moved.sketch.inspect_rank().unwrap());
    close(coords(moved.sketch.geometry(), pid(1))[0], -1.4, 1e-8);
    assert_eq!(
        moved.sketch.relations().collect::<Vec<_>>(),
        constrained.relations().collect::<Vec<_>>()
    );
    assert_eq!(base.geometry(), &original);
}

#[test]
fn tangent_declines_unshared_identity_and_the_backtracking_tangent_branch() {
    let controls = [[1., 0.2, 0.1], [2., 1., 0.], [3., 1., 0.]];
    let shape = source([-1., 0., 0.], controls, false);
    let joined = ConstrainedSketch::new(shape.clone(), policy()).unwrap();
    let wrong = LineCubicJoin {
        endpoint: pid(1),
        ..join()
    };
    assert!(matches!(
        joined.apply(&insertion(RelationSpec::Tangent(wrong)), || false),
        Err(ConstraintFailure::Continuity(id, ContinuityFailure::UnsharedEndpoint(join)))
            if id == rid(1) && join == wrong
    ));
    assert_eq!(joined.geometry(), &shape);
    let backwards = source([1., 0., 0.], controls, false);
    let sketch = ConstrainedSketch::new(backwards.clone(), policy()).unwrap();
    assert!(matches!(
        sketch.apply(&insertion(RelationSpec::Tangent(join())), || false),
        Err(ConstraintFailure::Continuity(id, ContinuityFailure::BackwardTangent)) if id == rid(1)
    ));
    assert_eq!(sketch.geometry(), &backwards);
}

#[test]
fn incompatible_fixed_curvature_authoring_and_cancellation_leave_the_original_revision() {
    let shape = source(
        [-1., 0., 0.],
        [[1., 0., 0.], [2., 1., 0.], [3., 1., 0.]],
        false,
    );
    let base = ConstrainedSketch::new(shape.clone(), policy()).unwrap();
    let fixed = base
        .apply(
            &[
                RelationEdit::Insert(rid(2), RelationSpec::FixPoint(pid(3))),
                RelationEdit::Insert(rid(3), RelationSpec::FixPoint(pid(4))),
            ],
            || false,
        )
        .unwrap();
    let edits = insertion(RelationSpec::G2(
        join(),
        CurvatureTolerance::new(1e-9).unwrap(),
    ));
    assert!(matches!(
        fixed.apply(&edits, || false),
        Err(ConstraintFailure::Unsettled(..) | ConstraintFailure::Infeasible(..))
    ));
    let mut polls = 0;
    assert!(matches!(
        base.apply(&edits, || {
            polls += 1;
            polls > 3
        }),
        Err(ConstraintFailure::Cancelled)
    ));
    assert_eq!(fixed.geometry(), &shape);
    assert_eq!(fixed.relations().count(), 2);
    assert_eq!(base.relations().count(), 0);
}

#[test]
fn prepared_g2_refreshes_the_frame_before_the_next_accepted_drag() {
    let shape = source(
        [-1., 0., 0.],
        [[1., 0., 0.], [2., 0., 0.], [3., 1., 0.]],
        false,
    );
    let base = ConstrainedSketch::new(shape.clone(), policy()).unwrap();
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
    for target in [[-0.8, -0.6, 0.], [-0.6, -0.8, 0.], [-0.28, -0.96, 0.]] {
        let accepted = sketch.drag(pid(1), point(target), || false).unwrap();
        assert_eq!(accepted.rank, accepted.sketch.inspect_rank().unwrap());
        assert_eq!(accepted.rank.rank, 7);
        assert_eq!(accepted.rank.degrees_of_freedom, 8);
        assert!(accepted.residual_work_bound <= policy().residual_work);
        for (actual, requested) in coords(accepted.sketch.geometry(), pid(1))
            .into_iter()
            .zip(target)
        {
            close(actual, requested, 2e-7);
        }
        assert_eq!(coords(accepted.sketch.geometry(), pid(2)), [0.; 3]);
        sketch = accepted.sketch;
    }
    assert_eq!(base.geometry(), &shape);
}

#[test]
fn preview_block_limit_counts_distinct_endpoint_stencil_coordinates() {
    let shape = source(
        [-1., 0., 0.],
        [[1., 0., 0.], [2., 0., 0.], [3., 1., 0.]],
        false,
    );
    let mut limited = policy();
    limited.preview.block_inputs = 11;
    let base = ConstrainedSketch::new(shape.clone(), limited).unwrap();
    base.apply(&insertion(RelationSpec::Tangent(join())), || false)
        .unwrap();
    let cap = CurvatureTolerance::new(1e-7).unwrap();
    assert!(matches!(
        base.apply(&insertion(RelationSpec::G2(join(), cap)), || false),
        Err(ConstraintFailure::Preview(
            None,
            PreviewError::Incomplete(Incomplete::Limit {
                quantity: Limit::BlockInputs,
                limit: 11,
                requested: 12
            })
        ))
    ));
    assert_eq!(base.geometry(), &shape);
}
