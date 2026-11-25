// SPDX-License-Identifier: GPL-2.0-only
use crate::fixture::{lid, line, pid, point, seed};
use martensite_tksketch3d::{
    EditFailure, EntityId, MinimumLength, PointRole, Snapshot,
    constraints::{
        ConstrainedSketch, ConstraintFailure, ConstraintPolicy, ParallelDirection, Relation,
        RelationEdit, RelationId, RelationSpec, SolveOptions, SolverLimits,
    },
};
use std::{num::NonZeroU64, time::Instant};

fn rid(id: u64) -> RelationId {
    RelationId::new(NonZeroU64::new(id).unwrap())
}
fn length(value: f64) -> MinimumLength {
    MinimumLength::new(value).unwrap()
}

fn policy() -> ConstraintPolicy {
    ConstraintPolicy {
        length_scale: length(10.),
        length_tolerance: length(2e-7),
        parallel_sine_tolerance: 2e-8,
        solve: SolveOptions {
            max_iterations: 100,
            max_trial_steps: 24,
            relative_tolerance: 1e-10,
            rank_tolerance: 1e-9,
        },
        preview: SolverLimits {
            coordinates: 240,
            blocks: 118,
            equations: 118,
            block_inputs: 12,
            targets: 3,
            gram_terms: 8000,
            factor_updates: 2200,
            work: 250_000,
        },
        validation_items: 400,
        residual_work: 64_000_000,
    }
}

fn geometry() -> Snapshot {
    Snapshot::empty(length(0.01))
        .apply(
            &[
                seed(1, [-0., 0., 0.], PointRole::Defining),
                seed(2, [2., 3., 6.], PointRole::Defining),
                seed(3, [0., 0., 2.], PointRole::SketchMarker),
                seed(4, [-4., -6., -10.], PointRole::Defining),
                seed(5, [1., 1., 1.], PointRole::CenterMarker),
                seed(6, [1.5, 1.1, 1.], PointRole::SketchMarker),
                line(10, 1, 2),
                line(11, 3, 4),
            ],
            || false,
        )
        .unwrap()
}

fn relation_batch() -> [RelationEdit; 5] {
    [
        RelationEdit::Insert(rid(1), RelationSpec::FixLine(lid(10))),
        RelationEdit::Insert(rid(2), RelationSpec::FixPoint(pid(5))),
        RelationEdit::Insert(rid(3), RelationSpec::Coincident([pid(5), pid(6)])),
        RelationEdit::Insert(
            rid(4),
            RelationSpec::Parallel([lid(10), lid(11)], ParallelDirection::Opposite),
        ),
        RelationEdit::Insert(rid(5), RelationSpec::Length(lid(11), length(12.))),
    ]
}

#[test]
fn directed_spatial_constraints_establish_and_drag_with_exact_anchors_and_owned_identity() {
    let original = geometry();
    let base = ConstrainedSketch::new(original.clone(), policy()).unwrap();
    let sketch = base.apply(&relation_batch(), || false).unwrap();
    let rank = sketch.inspect_rank().unwrap();
    assert_eq!(
        (rank.rank, rank.degrees_of_freedom, rank.equations),
        (15, 3, 15)
    );
    assert_eq!(base.geometry(), &original);
    assert_eq!(
        sketch.geometry().owned_ids().collect::<Vec<_>>(),
        original.owned_ids().collect::<Vec<_>>()
    );
    let anchor = sketch.geometry().point(pid(1)).unwrap().position;
    assert_eq!(anchor.coordinates()[0].to_bits(), (-0_f64).to_bits());
    let target = sketch
        .geometry()
        .point(pid(3))
        .unwrap()
        .position
        .coordinates()
        .to_owned();
    let target = point([target[0] + 0.3, target[1] - 0.4, target[2] + 0.2]);
    let update = sketch.drag(pid(3), target, || false).unwrap();
    assert_eq!(rank, update.rank);
    assert_eq!(update.rank, update.sketch.inspect_rank().unwrap());
    assert!(update.residual_work_bound > 0 && update.residual_work_bound <= policy().residual_work);
    assert_eq!(
        sketch.relations().collect::<Vec<_>>(),
        update.sketch.relations().collect::<Vec<_>>()
    );
    let moved = update.sketch.geometry();
    assert!(moved.point(pid(3)).unwrap().position.distance(&target) < 2e-7);
    assert!(
        moved
            .point(pid(5))
            .unwrap()
            .position
            .distance(&moved.point(pid(6)).unwrap().position)
            < 2e-7
    );
    assert_eq!(
        moved
            .point(pid(1))
            .unwrap()
            .position
            .coordinates()
            .map(f64::to_bits),
        anchor.coordinates().map(f64::to_bits)
    );
    independent_parallel_and_length(moved);
}

fn independent_parallel_and_length(snapshot: &Snapshot) {
    let a = snapshot.line(lid(10)).unwrap().points;
    let b = snapshot.line(lid(11)).unwrap().points;
    let delta = |pair: [martensite_tkmath::gp::Point3; 2]| {
        std::array::from_fn::<_, 3, _>(|i| pair[1].coordinates()[i] - pair[0].coordinates()[i])
    };
    let first = delta(a);
    let second = delta(b);
    let dot = first
        .into_iter()
        .zip(second)
        .map(|(a, b)| a * b)
        .sum::<f64>();
    assert!(dot < 0.);
    for (i, j) in [(0, 1), (1, 2), (2, 0)] {
        assert!((first[i] * second[j] - first[j] * second[i]).abs() < 2e-6);
    }
    assert!((second.iter().map(|x| x * x).sum::<f64>().sqrt() - 12.).abs() < 2e-7);
}

#[test]
fn wrong_branch_refuses_with_the_authored_relation_and_cannot_mutate_geometry() {
    let original = geometry();
    let base = ConstrainedSketch::new(original.clone(), policy()).unwrap();
    let edits = [RelationEdit::Insert(
        rid(6),
        RelationSpec::Parallel([lid(10), lid(11)], ParallelDirection::Same),
    )];
    assert!(
        matches!(base.apply(&edits,|| false), Err(ConstraintFailure::Solve(Some(id), _)) if id == rid(6))
    );
    assert_eq!(base.geometry(), &original);
    assert_eq!(base.relations().count(), 0);
}

#[test]
fn separately_admitted_geometry_rejects_solved_line_collapse_and_loose_solver_residuals() {
    let original = geometry();
    let base = ConstrainedSketch::new(original.clone(), policy()).unwrap();
    let anchor = original.point(pid(1)).unwrap().position;
    assert!(matches!(
        base.drag(pid(2), anchor, || false),
        Err(ConstraintFailure::Geometry(
            EditFailure::CoincidentEndpoints(_)
        ))
    ));
    let mut loose = policy();
    loose.solve.relative_tolerance = 0.2;
    let base = ConstrainedSketch::new(original, loose).unwrap();
    let edits = [RelationEdit::Insert(
        rid(9),
        RelationSpec::Length(lid(10), length(8.)),
    )];
    assert!(
        matches!(base.apply(&edits,|| false), Err(ConstraintFailure::Infeasible(id,Relation::Length(_, _))) if id == rid(9))
    );
}

#[test]
fn driving_length_admits_representable_closed_tolerance_endpoints() {
    let mut policy = policy();
    policy.length_tolerance = length(0.25);
    policy.solve.relative_tolerance = 0.2;
    let base = ConstrainedSketch::new(geometry(), policy).unwrap();
    for value in [6.75, 7.25] {
        let edits = [RelationEdit::Insert(
            rid(20),
            RelationSpec::Length(lid(10), length(value)),
        )];
        let accepted = base.apply(&edits, || false).unwrap();
        assert_eq!(accepted.geometry(), base.geometry());
    }
    let edits = [RelationEdit::Insert(
        rid(20),
        RelationSpec::Length(lid(10), length(6.75_f64.next_down())),
    )];
    assert!(matches!(
        base.apply(&edits, || false),
        Err(ConstraintFailure::Infeasible(..))
    ));
}

#[test]
fn edits_and_drag_cancellation_have_no_partial_adoption() {
    let original = geometry();
    let base = ConstrainedSketch::new(original.clone(), policy()).unwrap();
    let snapshot = base.apply(&relation_batch(), || false).unwrap();
    assert!(matches!(
        base.apply(&relation_batch(), || true),
        Err(ConstraintFailure::Cancelled)
    ));
    let target = point([0.5, 1., 2.]);
    let mut completed_polls = 0;
    snapshot
        .drag(pid(3), target, || {
            completed_polls += 1;
            false
        })
        .unwrap();
    let mut polls = 0;
    assert!(matches!(
        snapshot.drag(pid(3), target, || {
            polls += 1;
            polls == completed_polls
        }),
        Err(ConstraintFailure::Cancelled)
    ));
    assert_eq!(polls, completed_polls);
    assert_eq!(snapshot.relations().count(), 5);
    assert_eq!(base.geometry(), &original);
}

#[test]
fn relation_edits_refuse_repeated_missing_and_retired_ids_and_recompute_rank_after_removal() {
    let base = ConstrainedSketch::new(geometry(), policy()).unwrap();
    let fixed = base
        .apply(
            &[
                RelationEdit::Insert(rid(1), RelationSpec::FixLine(lid(10))),
                RelationEdit::Insert(rid(2), RelationSpec::FixPoint(pid(1))),
            ],
            || false,
        )
        .unwrap();
    let rank = fixed.inspect_rank().unwrap();
    assert_eq!((rank.rank, rank.equations), (6, 9));
    let removed = fixed
        .apply(&[RelationEdit::Remove(rid(1))], || false)
        .unwrap();
    assert_eq!(removed.inspect_rank().unwrap().rank, 3);
    let replay = [
        RelationEdit::Remove(rid(1)),
        RelationEdit::Insert(rid(1), RelationSpec::FixPoint(pid(1))),
    ];
    assert!(matches!(
        fixed.apply(&replay, || false),
        Err(ConstraintFailure::DuplicateId(_))
    ));
    assert!(matches!(
        base.apply(
            &[RelationEdit::Insert(
                rid(1),
                RelationSpec::FixPoint(pid(99))
            )],
            || false
        ),
        Err(ConstraintFailure::MissingOperand(_, EntityId::Point(_)))
    ));
    assert!(matches!(
        base.apply(
            &[RelationEdit::Insert(
                rid(1),
                RelationSpec::Coincident([pid(1); 2])
            )],
            || false
        ),
        Err(ConstraintFailure::RepeatedOperand(_, EntityId::Point(_)))
    ));
}

#[test]
fn preview_target_and_work_refusals_are_typed() {
    let mut policy = policy();
    policy.residual_work = 0;
    let base = ConstrainedSketch::new(geometry(), policy).unwrap();
    assert!(matches!(
        base.drag(pid(1), point([1., 0., 0.]), || false),
        Err(ConstraintFailure::Preview(_, _))
    ));
    assert!(matches!(
        base.drag(pid(99), point([1., 0., 0.]), || false),
        Err(ConstraintFailure::MissingPoint(_))
    ));
    assert!(matches!(
        base.drag(pid(1), point([f64::INFINITY, 0., 0.]), || false),
        Err(ConstraintFailure::InvalidTarget(_, _))
    ));
}

#[test]
#[ignore = "release p95 fixture; pin to an idle physical core"]
fn measured_bounded_drag_includes_native_geometry_and_independent_relation_adoption() {
    for count in [5, 10, 20, 40, 80] {
        let (mut sketch, original) = chain(count);
        let point_id = pid(count);
        let mut ticks = Vec::with_capacity(100);
        let mut work = 0;
        for i in 0..100 {
            let delta = [
                triangle(i, 31) * 0.08,
                triangle(i, 23) * 0.07,
                triangle(i, 37) * 0.06,
            ];
            let target = point(std::array::from_fn(|axis| original[axis] + delta[axis]));
            let start = Instant::now();
            let updated = sketch.drag(point_id, target, || false).unwrap();
            ticks.push(start.elapsed().as_nanos());
            assert_eq!(updated.rank.rank, count as usize + 2);
            work = work.max(updated.residual_work_bound);
            sketch = updated.sketch;
        }
        ticks.sort_unstable();
        println!(
            "{{\"coordinates\":{},\"samples\":100,\"p95_ns\":{},\"residual_work_bound\":{work}}}",
            count * 3,
            ticks[94]
        );
    }
}

fn triangle(index: i32, period: i32) -> f64 {
    let phase = f64::from((index + 1) % period) / f64::from(period);
    1.0 - 4.0 * (phase - 0.5).abs()
}

fn chain(count: u64) -> (ConstrainedSketch, [f64; 3]) {
    let mut cursor = [0.; 3];
    let mut edits = vec![seed(1, cursor, PointRole::Defining)];
    let mut relations = vec![RelationEdit::Insert(rid(1), RelationSpec::FixPoint(pid(1)))];
    for i in 1..count {
        cursor[(i as usize - 1) % 3] += 10.;
        edits.push(seed(i + 1, cursor, PointRole::Defining));
        edits.push(line(count + i, i, i + 1));
        relations.push(RelationEdit::Insert(
            rid(i + 1),
            RelationSpec::Length(lid(count + i), length(10.)),
        ));
    }
    let snapshot = Snapshot::empty(length(0.01))
        .apply(&edits, || false)
        .unwrap();
    let sketch = ConstrainedSketch::new(snapshot, policy())
        .unwrap()
        .apply(&relations, || false)
        .unwrap();
    (sketch, cursor)
}
