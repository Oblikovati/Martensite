// SPDX-License-Identifier: GPL-2.0-only
use crate::{
    constraints::{length, policy, rid},
    fixture::{lid, line, pid, point, seed},
};
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tksketch3d::{
    LineId, PointRole, Snapshot,
    constraints::{
        AxisDirection, ConstrainedSketch, ConstraintFailure, CoordinateAxis, Relation,
        RelationEdit, RelationSpec,
    },
};
use martensite_tksolve::preview::{Incomplete, PreviewError};
use std::time::Instant;

#[test]
fn axis_normalization_solves_each_model_axis_and_keeps_both_signs_and_one_free_length() {
    for (index, axis) in [CoordinateAxis::X, CoordinateAxis::Y, CoordinateAxis::Z]
        .into_iter()
        .enumerate()
    {
        for (sign, direction) in [
            (1., AxisDirection::Positive),
            (-1., AxisDirection::Negative),
        ] {
            let start = [10., -20., 30.];
            let mut end = [11., -19., 31.];
            end[index] = start[index] + 4. * sign;
            let geometry = Snapshot::empty(length(0.01))
                .apply(
                    &[
                        seed(1, start, PointRole::Defining),
                        seed(2, end, PointRole::Defining),
                        line(3, 1, 2),
                    ],
                    || false,
                )
                .unwrap();
            let base = ConstrainedSketch::new(geometry, policy()).unwrap();
            let sketch = base
                .apply(
                    &[
                        RelationEdit::Insert(rid(10), RelationSpec::FixPoint(pid(1))),
                        RelationEdit::Insert(
                            rid(11),
                            RelationSpec::ParallelToAxis(lid(3), axis, direction),
                        ),
                    ],
                    || false,
                )
                .unwrap();
            let mut target = start;
            target[index] += 7. * sign;
            let moved = sketch.drag(pid(2), point(target), || false).unwrap();
            assert_eq!(
                (
                    moved.rank.rank,
                    moved.rank.degrees_of_freedom,
                    moved.rank.equations
                ),
                (5, 1, 5)
            );
            let delta = displacement(moved.sketch.geometry(), lid(3));
            assert!((delta[index] - 7. * sign).abs() < 1e-8);
            assert!(delta[(index + 1) % 3].abs() < 1e-8 && delta[(index + 2) % 3].abs() < 1e-8);
            assert!(
                moved
                    .sketch
                    .relations()
                    .any(|(_, relation)| relation
                        == Relation::ParallelToAxis(lid(3), axis, direction))
            );
            let mut blocked = start;
            blocked[index] -= sign;
            assert!(matches!(
                moved.sketch.drag(pid(2), point(blocked), || false),
                Err(ConstraintFailure::Preview(_, PreviewError::Incomplete(_)))
            ));
        }
    }
}

#[test]
fn plane_parallel_uses_only_the_normal_and_keeps_the_offset_free() {
    let start = [5., 6., 7.];
    let end = [7., 5., 7.];
    let geometry = Snapshot::empty(length(0.01))
        .apply(
            &[
                seed(1, start, PointRole::Defining),
                seed(2, end, PointRole::Defining),
                line(3, 1, 2),
            ],
            || false,
        )
        .unwrap();
    let raw_normal = [1., 2., 2.];
    let n = Direction3::new(1., 2., 2.).unwrap();
    let sketch = ConstrainedSketch::new(geometry, policy())
        .unwrap()
        .apply(
            &[RelationEdit::Insert(
                rid(1),
                RelationSpec::ParallelToPlane(lid(3), n),
            )],
            || false,
        )
        .unwrap();
    let target = point(std::array::from_fn(|i| end[i] + raw_normal[i] * 0.2));
    let moved = sketch.drag(pid(2), target, || false).unwrap();
    assert_eq!((moved.rank.rank, moved.rank.degrees_of_freedom), (1, 5));
    let geometry = moved.sketch.geometry();
    assert!(dot(displacement(geometry, lid(3)), raw_normal).abs() < 1e-8);
    assert!(
        dot(
            *geometry.point(pid(1)).unwrap().position.coordinates(),
            raw_normal
        ) - dot(start, raw_normal)
            > 1.
    );
    assert!(geometry.point(pid(2)).unwrap().position.distance(&target) < 1e-8);
}

#[test]
fn skew_perpendicular_and_equal_length_admit_the_motion_of_a_one_degree_component() {
    let sketch = ConstrainedSketch::new(skew_lines(), policy())
        .unwrap()
        .apply(
            &[
                RelationEdit::Insert(rid(1), RelationSpec::FixLine(lid(10))),
                RelationEdit::Insert(rid(2), RelationSpec::FixPoint(pid(3))),
                RelationEdit::Insert(rid(3), RelationSpec::Perpendicular([lid(10), lid(11)])),
                RelationEdit::Insert(rid(4), RelationSpec::EqualLength([lid(10), lid(11)])),
            ],
            || false,
        )
        .unwrap();
    let target = point([0., 5., 6.]);
    let moved = sketch.drag(pid(4), target, || false).unwrap();
    let geometry = moved.sketch.geometry();
    let a = displacement(geometry, lid(10));
    let b = displacement(geometry, lid(11));
    assert_eq!(
        (
            moved.rank.rank,
            moved.rank.degrees_of_freedom,
            moved.rank.equations
        ),
        (11, 1, 11)
    );
    assert_eq!(moved.rank, moved.sketch.inspect_rank().unwrap());
    assert!(dot(a, b).abs() < 1e-8);
    assert!((dot(a, a).sqrt() - dot(b, b).sqrt()).abs() < 1e-8);
    assert!(geometry.point(pid(4)).unwrap().position.distance(&target) < 1e-8);
    assert_eq!(
        geometry.line(lid(10)).unwrap(),
        sketch.geometry().line(lid(10)).unwrap()
    );
    assert_eq!(
        geometry.point(pid(3)).unwrap().position,
        point([0., 2., 2.])
    );
    assert!(matches!(
        sketch.drag(pid(4), target, || true),
        Err(ConstraintFailure::Cancelled)
    ));
}

#[test]
fn fixed_conflicts_and_exact_redundancy_have_distinct_outcomes() {
    let base = ConstrainedSketch::new(skew_lines(), policy()).unwrap();
    let fixed_conflict = base.apply(
        &[
            RelationEdit::Insert(rid(1), RelationSpec::FixLine(lid(10))),
            RelationEdit::Insert(rid(2), RelationSpec::FixLine(lid(11))),
            RelationEdit::Insert(rid(3), RelationSpec::EqualLength([lid(10), lid(11)])),
        ],
        || false,
    );
    assert!(
        matches!(fixed_conflict,Err(ConstraintFailure::Unsettled(_, ids)) if ids.contains(&rid(3)))
    );
    assert_eq!(base.relations().count(), 0);
    let overlapping = base.geometry().apply(&[line(12, 2, 1)], || false).unwrap();
    let redundant = ConstrainedSketch::new(overlapping, policy())
        .unwrap()
        .apply(
            &[RelationEdit::Insert(
                rid(4),
                RelationSpec::EqualLength([lid(10), lid(12)]),
            )],
            || false,
        )
        .unwrap();
    let rank = redundant.inspect_rank().unwrap();
    assert_eq!((rank.rank, rank.equations), (0, 1));
    assert!(matches!(
        redundant.drag(pid(2), point([4., 1., 0.]), || false),
        Err(ConstraintFailure::Preview(
            _,
            PreviewError::Incomplete(Incomplete::RankRecovery { .. })
        ))
    ));
}

#[test]
fn axis_seed_direction_plane_normal_and_physical_equality_are_checked_independently() {
    let base = ConstrainedSketch::new(skew_lines(), policy()).unwrap();
    for axis in [CoordinateAxis::X, CoordinateAxis::Y] {
        assert!(
            matches!(base.apply(&[RelationEdit::Insert(rid(1),RelationSpec::ParallelToAxis(lid(10),axis,AxisDirection::Negative))],|| false),Err(ConstraintFailure::AxisSeedBranch(id,line,actual,AxisDirection::Negative)) if id==rid(1)&&line==lid(10)&&actual==axis)
        );
    }
    let invalid = Direction3::new(f64::MAX, 0., 0.).unwrap();
    assert!(
        matches!(base.apply(&[RelationEdit::Insert(rid(2),RelationSpec::ParallelToPlane(lid(10),invalid))],|| false),Err(ConstraintFailure::InvalidPlaneNormal(id,_)) if id==rid(2))
    );
    let mut loose = policy();
    loose.solve.relative_tolerance = 0.4;
    let base = ConstrainedSketch::new(skew_lines(), loose).unwrap();
    assert!(
        matches!(base.apply(&[RelationEdit::Insert(rid(3),RelationSpec::EqualLength([lid(10),lid(11)]))],|| false),Err(ConstraintFailure::Infeasible(id,Relation::EqualLength(_))) if id==rid(3))
    );
}

fn skew_lines() -> Snapshot {
    Snapshot::empty(length(0.01))
        .apply(
            &[
                seed(1, [0., 0., 0.], PointRole::Defining),
                seed(2, [5., 0., 0.], PointRole::Defining),
                seed(3, [0., 2., 2.], PointRole::Defining),
                seed(4, [0., 5., 2.], PointRole::Defining),
                line(10, 1, 2),
                line(11, 3, 4),
            ],
            || false,
        )
        .unwrap()
}

fn displacement(geometry: &Snapshot, id: LineId) -> [f64; 3] {
    let [start, end] = geometry.line(id).unwrap().points;
    std::array::from_fn(|i| end.coordinates()[i] - start.coordinates()[i])
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.into_iter().zip(b).map(|(x, y)| x * y).sum()
}

#[test]
#[ignore = "release p95 fixture for independently adopted new line relations"]
fn measured_native_drag_of_axis_plane_perpendicular_and_equal_length_components() {
    for pair_count in [5, 10, 20] {
        let (mut sketch, end) = parallel_components(pair_count);
        let mut durations = Vec::with_capacity(100);
        let mut work = 0;
        let mut seed_error = 0.0_f64;
        for i in 0..100 {
            let phase = |period: i32| {
                1.0 - 4.0 * (f64::from((i + 1) % period) / f64::from(period) - 0.5).abs()
            };
            let delta = [0.08 * phase(31), 0.07 * phase(23), 0.06 * phase(37)];
            let target = point(std::array::from_fn(|i| end.coordinates()[i] + delta[i]));
            let start = Instant::now();
            let update = sketch.drag(pid(4 * pair_count), target, || false).unwrap();
            durations.push(start.elapsed().as_nanos());
            assert_eq!(
                (update.rank.rank, update.rank.equations),
                (5 * pair_count as usize, 5 * pair_count as usize)
            );
            work = work.max(update.residual_work_bound);
            seed_error = seed_error.max(update.solver_hard_error);
            sketch = update.sketch;
        }
        durations.sort_unstable();
        println!(
            "{{\"coordinates\":{},\"samples\":100,\"p95_ns\":{},\"work\":{work},\"maximum_scaled_error\":{seed_error}}}",
            12 * pair_count,
            durations[94]
        );
    }
}

fn parallel_components(pair_count: u64) -> (ConstrainedSketch, Point3) {
    let mut geometry = Vec::new();
    let mut relations = Vec::new();
    let mut end = point([0.; 3]);
    for pair in 0..pair_count {
        let x = 20. * f64::from(u32::try_from(pair).unwrap());
        let first = 4 * pair + 1;
        let a = 1000 + 2 * pair;
        let b = a + 1;
        end = point([x, 7., 2.]);
        geometry.extend([
            seed(first, [x, 0., 0.], PointRole::Defining),
            seed(first + 1, [x + 5., 0., 0.], PointRole::Defining),
            seed(first + 2, [x, 2., 2.], PointRole::Defining),
            seed(first + 3, *end.coordinates(), PointRole::Defining),
            line(a, first, first + 1),
            line(b, first + 2, first + 3),
        ]);
        let r = 4 * pair + 1;
        relations.extend([
            RelationEdit::Insert(
                rid(r),
                RelationSpec::ParallelToAxis(lid(a), CoordinateAxis::X, AxisDirection::Positive),
            ),
            RelationEdit::Insert(
                rid(r + 1),
                RelationSpec::ParallelToPlane(lid(b), Direction3::Z),
            ),
            RelationEdit::Insert(rid(r + 2), RelationSpec::Perpendicular([lid(a), lid(b)])),
            RelationEdit::Insert(rid(r + 3), RelationSpec::EqualLength([lid(a), lid(b)])),
        ]);
    }
    let snapshot = Snapshot::empty(length(0.01))
        .apply(&geometry, || false)
        .unwrap();
    (
        ConstrainedSketch::new(snapshot, policy())
            .unwrap()
            .apply(&relations, || false)
            .unwrap(),
        end,
    )
}
