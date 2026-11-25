// SPDX-License-Identifier: GPL-2.0-only
use crate::{
    constraints::{length, policy, rid},
    fixture::{lid, line, pid, point, seed},
};
use martensite_tksketch3d::{
    LineId, PointId, PointRole, Snapshot,
    constraints::{
        ConstrainedSketch, ConstraintFailure, ParallelDirection, Relation, RelationEdit,
        RelationSpec,
    },
};
use martensite_tksolve::preview::{Incomplete, PreviewError};
use std::time::Instant;

const CORPUS_FIRST_LINE: u64 = 1000;

#[test]
fn point_on_line_solves_the_rotated_infinite_support_with_one_free_axial_coordinate() {
    let data = geometry(
        &[[4., -3., 8.], [4., -2.99, 8.], [4.3, -9., 7.8]],
        &[(1, 2)],
    );
    let sketch = author(
        data,
        &[
            RelationSpec::FixLine(lid(10)),
            RelationSpec::PointOnLine(pid(3), lid(10)),
        ],
    );
    assert_rank(&sketch, (8, 1, 8));
    assert_incident(sketch.geometry(), lid(10), pid(3));
    let target = point([4., -11., 8.]);
    let moved = sketch.drag(pid(3), target, || false).unwrap();
    assert_eq!(moved.rank, moved.sketch.inspect_rank().unwrap());
    assert!(
        moved
            .sketch
            .geometry()
            .point(pid(3))
            .unwrap()
            .position
            .distance(&target)
            < 1e-8
    );
    assert_eq!(
        moved.sketch.geometry().line(lid(10)),
        sketch.geometry().line(lid(10))
    );
    assert_incident(moved.sketch.geometry(), lid(10), pid(3));
}

#[test]
fn point_on_line_refuses_its_own_endpoint_and_restore_certifies_the_metric_boundary() {
    let data = geometry(&[[0.; 3], [1., 0., 0.], [0., 0.5, 0.]], &[(1, 2)]);
    let base = ConstrainedSketch::new(data.clone(), policy()).unwrap();
    let endpoint = [(rid(1), Relation::PointOnLine(pid(2), lid(10)))];
    let refused = |result| {
        assert!(
            matches!(result, Err(ConstraintFailure::PointIsLineEndpoint(r, p, l))
            if (r, p, l) == (rid(1), pid(2), lid(10)))
        );
    };
    refused(base.apply(
        &[insert(1, RelationSpec::PointOnLine(pid(2), lid(10)))],
        || false,
    ));
    refused(ConstrainedSketch::restore(
        data.clone(),
        &endpoint,
        policy(),
        || false,
    ));
    let record = [(rid(3), Relation::PointOnLine(pid(3), lid(10)))];
    let mut boundary = policy();
    boundary.length_tolerance = length(0.5);
    let restored = ConstrainedSketch::restore(data.clone(), &record, boundary, || false).unwrap();
    assert_eq!(restored.geometry(), &data);
    boundary.length_tolerance = length(0.5_f64.next_down());
    assert!(matches!(
        ConstrainedSketch::restore(data, &record, boundary, || false),
        Err(ConstraintFailure::Infeasible(r, _)) if r == rid(3)
    ));
}

#[test]
fn disjoint_colinear_supports_need_four_rows_even_when_the_reference_line_is_short() {
    let data = geometry(
        &[[3., 4., 5.], [3.01, 4., 5.], [7., 4.2, 5.1], [5., 4.1, 4.9]],
        &[(1, 2), (3, 4)],
    );
    let relation = RelationSpec::Colinear([lid(10), lid(11)], ParallelDirection::Opposite);
    let sketch = author(data, &[RelationSpec::FixLine(lid(10)), relation]);
    assert_rank(&sketch, (10, 2, 10));
    assert_incident(sketch.geometry(), lid(10), pid(3));
    assert_incident(sketch.geometry(), lid(10), pid(4));
    let moved = sketch.drag(pid(4), point([2., 4., 5.]), || false).unwrap();
    assert!(signed_alignment(moved.sketch.geometry()) < -0.999999);
    assert_incident(moved.sketch.geometry(), lid(10), pid(3));
    assert_incident(moved.sketch.geometry(), lid(10), pid(4));
    assert_restore(moved.sketch);
}

#[test]
fn colinear_shared_endpoint_is_matched_by_identity_in_all_endpoint_orders() {
    for (second, direction) in [
        ((1, 3), ParallelDirection::Same),
        ((3, 1), ParallelDirection::Opposite),
        ((2, 3), ParallelDirection::Same),
        ((3, 2), ParallelDirection::Opposite),
    ] {
        let data = geometry(&[[0.; 3], [0., 2., 0.], [0.1, 3., 0.2]], &[(1, 2), second]);
        let sketch = author(
            data,
            &[
                RelationSpec::FixLine(lid(10)),
                RelationSpec::Colinear([lid(10), lid(11)], direction),
            ],
        );
        assert_rank(&sketch, (8, 1, 8));
        let moved = sketch.drag(pid(3), point([0., 5., 0.]), || false).unwrap();
        assert_eq!(
            moved
                .sketch
                .geometry()
                .line(lid(11))
                .unwrap()
                .line
                .endpoints,
            [pid(second.0), pid(second.1)]
        );
        assert_incident(moved.sketch.geometry(), lid(10), pid(3));
        assert_restore(moved.sketch);
    }
}

#[test]
fn colinear_two_shared_endpoints_retain_authored_identity_with_zero_solver_rows() {
    for (ends, branch, wrong) in [
        ((1, 2), ParallelDirection::Same, ParallelDirection::Opposite),
        ((2, 1), ParallelDirection::Opposite, ParallelDirection::Same),
    ] {
        let data = geometry(&[[1., 2., 3.], [4., 5., 6.]], &[(1, 2), ends]);
        let lines = [lid(10), lid(11)];
        let base = ConstrainedSketch::new(data.clone(), policy()).unwrap();
        assert!(matches!(
            base.apply(&[insert(1, RelationSpec::Colinear(lines, wrong))], || false),
            Err(ConstraintFailure::ColinearSeedBranch(id, accepted, direction))
                if (id, accepted, direction) == (rid(1), lines, wrong)
        ));
        let sketch = author(data.clone(), &[RelationSpec::Colinear(lines, branch)]);
        assert_eq!(sketch.geometry(), &data);
        assert_rank(&sketch, (0, 6, 0));
        assert_eq!(
            sketch.relations().collect::<Vec<_>>(),
            [(rid(1), Relation::Colinear(lines, branch))]
        );
        let moved = sketch.drag(pid(2), point([2., 4., 9.]), || false).unwrap();
        assert_eq!(moved.rank.rank, 0);
        assert_restore(moved.sketch);
    }
}

#[test]
fn colinear_uses_identity_for_rank_and_refuses_a_disagreeing_retained_branch() {
    let data = geometry(
        &[[0.; 3], [1., 0., 0.], [0.; 3], [2., 0., 0.]],
        &[(1, 2), (3, 4)],
    );
    let lines = [lid(10), lid(11)];
    let same = Relation::Colinear(lines, ParallelDirection::Same);
    let wrong = Relation::Colinear(lines, ParallelDirection::Opposite);
    let sketch =
        ConstrainedSketch::restore(data.clone(), &[(rid(9), same)], policy(), || false).unwrap();
    assert_rank(&sketch, (4, 8, 4));
    assert!(matches!(
        ConstrainedSketch::restore(data, &[(rid(9), wrong)], policy(), || false),
        Err(ConstraintFailure::ColinearSeedBranch(
            _,
            _,
            ParallelDirection::Opposite
        ))
    ));
    let repeated = RelationSpec::Colinear([lid(10); 2], ParallelDirection::Same);
    assert!(matches!(
        sketch.apply(&[insert(7, repeated)], || false),
        Err(ConstraintFailure::RepeatedOperand(_, _))
    ));
}

#[test]
fn prepared_chart_crossing_rolls_forward_in_one_gesture_and_declines_an_oversized_jump() {
    let initial = rotating_sketch();
    let target = point([0., 2., 0.]);
    assert!(matches!(
        initial.drag(pid(2), target, || false),
        Err(ConstraintFailure::Preview(
            _,
            PreviewError::Incomplete(Incomplete::BlockedTarget | Incomplete::Iterations)
        ))
    ));
    assert_eq!(
        initial.geometry().point(pid(2)).unwrap().position,
        point([2., 0., 0.])
    );
    let next = roll_forward(initial.clone());
    assert!(
        next.geometry()
            .point(pid(2))
            .unwrap()
            .position
            .distance(&target)
            < 1e-8
    );
    assert_incident(next.geometry(), lid(10), pid(3));
    assert_rank(&next, (5, 4, 5));
    assert_restore(next);
}

#[test]
fn colinear_rolls_across_the_same_old_pole_with_a_free_second_line() {
    let initial = author(
        geometry(
            &[[0.; 3], [2., 0., 0.], [3., 0., 0.], [5., 0., 0.]],
            &[(1, 2), (3, 4)],
        ),
        &[
            RelationSpec::FixPoint(pid(1)),
            RelationSpec::Colinear([lid(10), lid(11)], ParallelDirection::Same),
        ],
    );
    let next = roll_forward(initial);
    assert_rank(&next, (7, 5, 7));
    assert!(signed_alignment(next.geometry()) > 0.999999);
    assert_incident(next.geometry(), lid(10), pid(4));
}

#[test]
fn a_chart_crossing_that_exhausts_the_tick_keeps_the_prior_revision() {
    let initial = rotating_sketch();
    let [first, second, ..] = path();
    let first_update = initial.drag(pid(2), point(first), || false).unwrap();
    let complete = first_update
        .sketch
        .drag(pid(2), point(second), || false)
        .unwrap();
    let bound = complete.residual_work_bound - 1;
    assert!(first_update.residual_work_bound <= bound);
    let mut guarded = policy();
    guarded.residual_work = bound;
    let initial = ConstrainedSketch::restore(
        initial.geometry().clone(),
        &initial.relations().collect::<Vec<_>>(),
        guarded,
        || false,
    )
    .unwrap();
    let before = initial.drag(pid(2), point(first), || false).unwrap().sketch;
    assert!(matches!(
        before.drag(pid(2), point(second), || false),
        Err(ConstraintFailure::Preview(
            _,
            PreviewError::Incomplete(Incomplete::Limit { .. })
        ))
    ));
    assert_eq!(before.geometry(), first_update.sketch.geometry());
    assert_eq!(
        before.inspect_rank().unwrap(),
        first_update.sketch.inspect_rank().unwrap()
    );
}

#[test]
fn incidence_chart_replay_matches_on_independent_threads() {
    let accepted = rotating_sketch();
    let expected = gesture_receipt(&accepted);
    println!("incidence gesture bits {expected:?}");
    std::thread::scope(|scope| {
        let workers = (0..4)
            .map(|_| {
                scope.spawn(|| {
                    for _ in 0..48 {
                        assert_eq!(gesture_receipt(&accepted), expected);
                    }
                })
            })
            .collect::<Vec<_>>();
        for worker in workers {
            worker.join().unwrap();
        }
    });
    assert_eq!(
        accepted.geometry().point(pid(2)).unwrap().position,
        point([2., 0., 0.])
    );
}

fn gesture_receipt(initial: &ConstrainedSketch) -> Vec<([[u64; 3]; 3], u64, usize)> {
    let mut sketch = initial.clone();
    path()
        .into_iter()
        .map(|target| {
            let moved = sketch.drag(pid(2), point(target), || false).unwrap();
            sketch = moved.sketch;
            let bits = [1, 2, 3].map(|id| {
                sketch
                    .geometry()
                    .point(pid(id))
                    .unwrap()
                    .position
                    .coordinates()
                    .map(f64::to_bits)
            });
            (
                bits,
                moved.solver_hard_error.to_bits(),
                moved.residual_work_bound,
            )
        })
        .collect()
}

#[test]
#[ignore = "release pointer latency, run single-threaded on a pinned core"]
fn measured_native_incidence_chart_crossings_and_prepared_drag() {
    for groups in [1, 13, 26] {
        let mut steady = Vec::new();
        let mut crossing = Vec::new();
        let initial = incidence_corpus(groups);
        for _ in 0..100 {
            let mut sketch = initial.clone();
            for (step, target) in path().into_iter().enumerate() {
                let start = Instant::now();
                let moved = sketch.drag(pid(2), point(target), || false).unwrap();
                let sample = (start.elapsed().as_nanos(), moved.residual_work_bound);
                if step == 1 {
                    crossing.push(sample);
                } else {
                    steady.push(sample);
                }
                assert_incident(moved.sketch.geometry(), lid(CORPUS_FIRST_LINE), pid(3));
                sketch = moved.sketch;
            }
        }
        print_times("steady", 9 * groups, steady);
        print_times("crossing", 9 * groups, crossing);
    }
}

fn print_times(mode: &str, coordinates: usize, mut samples: Vec<(u128, usize)>) {
    samples.sort_unstable();
    let p95 = samples[(95 * samples.len()).div_ceil(100) - 1].0;
    let max = samples.last().unwrap().0;
    let work = samples.iter().map(|sample| sample.1).max().unwrap();
    println!(
        "{{\"mode\":\"{mode}\",\"coordinates\":{coordinates},\"samples\":{},\"p95_ns\":{p95},\"max_ns\":{max},\"work\":{work}}}",
        samples.len()
    );
}

fn incidence_corpus(groups: usize) -> ConstrainedSketch {
    let mut points = Vec::new();
    let mut lines = Vec::new();
    let mut specs = Vec::new();
    for i in 0..groups as u64 {
        let x = 8. * i as f64;
        points.extend([[x, 0., 0.], [x + 2., 0., 0.], [x + 4., 0., 0.]]);
        lines.push((3 * i + 1, 3 * i + 2));
        specs.push(RelationSpec::FixPoint(pid(3 * i + 1)));
        specs.push(RelationSpec::PointOnLine(
            pid(3 * i + 3),
            lid(CORPUS_FIRST_LINE + i),
        ));
    }
    author(geometry_from(&points, &lines, CORPUS_FIRST_LINE), &specs)
}

fn rotating_sketch() -> ConstrainedSketch {
    author(
        geometry(&[[0.; 3], [2., 0., 0.], [4., 0., 0.]], &[(1, 2)]),
        &[
            RelationSpec::FixPoint(pid(1)),
            RelationSpec::PointOnLine(pid(3), lid(10)),
        ],
    )
}

fn roll_forward(mut sketch: ConstrainedSketch) -> ConstrainedSketch {
    for target in path() {
        let next = sketch.drag(pid(2), point(target), || false).unwrap().sketch;
        assert!(
            next.geometry()
                .point(pid(2))
                .unwrap()
                .position
                .distance(&point(target))
                < 1e-8
        );
        assert_incident(next.geometry(), lid(10), pid(3));
        sketch = next;
    }
    sketch
}

fn path() -> [[f64; 3]; 4] {
    [[2., 0.8, 0.], [1.4, 1.6, 0.], [1.1, 1.65, 0.], [0., 2., 0.]]
}

fn assert_restore(sketch: ConstrainedSketch) {
    let records = sketch.relations().collect::<Vec<_>>();
    let restored =
        ConstrainedSketch::restore(sketch.geometry().clone(), &records, policy(), || false)
            .unwrap();
    assert_eq!(restored.geometry(), sketch.geometry());
    assert_eq!(
        restored.inspect_rank().unwrap(),
        sketch.inspect_rank().unwrap()
    );
    assert_eq!(restored.relations().collect::<Vec<_>>(), records);
}

fn geometry(points: &[[f64; 3]], lines: &[(u64, u64)]) -> Snapshot {
    geometry_from(points, lines, 10)
}

fn geometry_from(points: &[[f64; 3]], lines: &[(u64, u64)], first_line: u64) -> Snapshot {
    let edits = points
        .iter()
        .enumerate()
        .map(|(i, &xyz)| seed(i as u64 + 1, xyz, PointRole::Defining))
        .chain(
            lines
                .iter()
                .enumerate()
                .map(|(i, &(a, b))| line(i as u64 + first_line, a, b)),
        )
        .collect::<Vec<_>>();
    Snapshot::empty(length(0.001))
        .apply(&edits, || false)
        .unwrap()
}

fn insert(id: u64, spec: RelationSpec) -> RelationEdit {
    RelationEdit::Insert(rid(id), spec)
}

fn author(data: Snapshot, specs: &[RelationSpec]) -> ConstrainedSketch {
    let edits = specs
        .iter()
        .enumerate()
        .map(|(i, &spec)| insert(i as u64 + 1, spec))
        .collect::<Vec<_>>();
    ConstrainedSketch::new(data, policy())
        .unwrap()
        .apply(&edits, || false)
        .unwrap()
}

fn assert_rank(sketch: &ConstrainedSketch, expected: (usize, usize, usize)) {
    let rank = sketch.inspect_rank().unwrap();
    assert_eq!(
        (rank.rank, rank.degrees_of_freedom, rank.equations),
        expected
    );
}

fn difference(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|i| a[i] - b[i])
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.into_iter().zip(b).map(|(a, b)| a * b).sum()
}

fn signed_alignment(data: &Snapshot) -> f64 {
    let [a, b] = [lid(10), lid(11)].map(|id| {
        let [a, b] = data.line(id).unwrap().points.map(|p| *p.coordinates());
        difference(b, a)
    });
    dot(a, b) / (dot(a, a) * dot(b, b)).sqrt()
}

fn assert_incident(data: &Snapshot, line: LineId, point: PointId) {
    let [a, b] = data.line(line).unwrap().points.map(|p| *p.coordinates());
    let p = *data.point(point).unwrap().position.coordinates();
    let delta = difference(b, a);
    let offset = difference(p, a);
    let u = dot(delta, offset) / dot(delta, delta);
    let projected = std::array::from_fn(|i| a[i] + u * delta[i]);
    let error = difference(p, projected);
    assert!(dot(error, error).sqrt() < 2e-8, "{error:?}");
}
