// SPDX-License-Identifier: GPL-2.0-only
use crate::{
    constraints::{length, policy, rid},
    fixture::{lid, line, pid, point, seed},
};
use martensite_tkmath::gp::Direction3;
use martensite_tksketch3d::{
    Edit, FrozenPointSource, FrozenSourceRevision, PointRole, SketchPoint, Snapshot,
    constraints::{
        AxisDirection, ConstrainedSketch, ConstraintFailure, CoordinateAxis, PreferenceSettlement,
        RelationEdit, RelationSpec,
    },
};
use martensite_tksolve::preview::PreviewError;

#[path = "preferred_drag/bench.rs"]
mod bench;

const TANGENT: f64 = 1e-8;

#[test]
fn preferred_pointer_along_a_owned_rail_tracks_the_analytic_foot() {
    let initial = rail();
    let off_rail = point([1., 1.6, -0.3]);
    assert!(matches!(
        initial.drag(pid(3), off_rail, || false),
        Err(ConstraintFailure::Preview(_, PreviewError::Incomplete(_)))
    ));
    assert_eq!(where_point(&initial, 3), [0.5, 0., 0.]);
    let mut sketch = initial.clone();
    for i in 1..=48 {
        let x = f64::from(i) * 0.125;
        let cursor = [x, 1. + x / 4., -0.2];
        let moved = sketch
            .drag_preferred(pid(3), point(cursor), TANGENT, || false)
            .unwrap();
        assert_eq!(
            moved.preference.settlement,
            PreferenceSettlement::FirstOrderStationary
        );
        assert!(moved.preference.max_tangential_gradient <= TANGENT);
        assert_eq!(
            moved.update.rank,
            moved.update.sketch.inspect_rank().unwrap()
        );
        assert!(moved.update.solver_hard_error <= policy().solve.relative_tolerance);
        near(
            moved
                .update
                .sketch
                .geometry()
                .point(pid(3))
                .unwrap()
                .position
                .x(),
            x,
            1e-9,
        );
        assert_eq!(
            moved.update.sketch.geometry().line(lid(10)),
            initial.geometry().line(lid(10))
        );
        sketch = moved.update.sketch;
    }
    assert_eq!(where_point(&initial, 3), [0.5, 0., 0.]);
}

#[test]
fn preferred_pointer_follows_a_fixed_length_circle_with_inside_outside_and_off_plane_targets() {
    let initial = circle();
    let mut sketch = initial.clone();
    for i in 1..=60 {
        let angle = f64::from(i) * std::f64::consts::TAU / 60.;
        let (sine, cosine) = martensite_cmath::sincos(angle);
        let cursor_radius = if i % 2 == 0 { 3. } else { 7. };
        let cursor = point([cursor_radius * cosine, cursor_radius * sine, 0.75]);
        let previous_error = point(where_point(&sketch, 2)).distance(&cursor);
        let moved = sketch
            .drag_preferred(pid(2), cursor, TANGENT, || false)
            .unwrap();
        assert_eq!(
            moved.preference.settlement,
            PreferenceSettlement::FirstOrderStationary
        );
        assert!(moved.preference.max_tangential_gradient <= TANGENT);
        let actual = where_point(&moved.update.sketch, 2);
        assert!(point(actual).distance(&cursor) < previous_error);
        near(
            moved.preference.scaled_cursor_error,
            martensite_cmath::hypot(cursor_radius - 5., 0.75) / 10.,
            2e-10,
        );
        for (actual, expected) in actual.into_iter().zip([5. * cosine, 5. * sine, 0.]) {
            near(actual, expected, 3e-7);
        }
        assert_eq!(where_point(&moved.update.sketch, 1), [0.; 3]);
        sketch = moved.update.sketch;
    }
    assert_eq!(where_point(&initial, 2), [5., 0., 0.]);
}

#[test]
fn oblique_rail_pointer_foot_preserves_the_previous_pose_for_normal_only_motion() {
    let initial = author(
        &[[2., 1., 4.], [5., 5., 4.], [2.75, 2., 4.]],
        &[
            RelationSpec::FixLine(lid(10)),
            RelationSpec::PointOnLine(pid(3), lid(10)),
        ],
    );
    let foot = [3.5, 3., 4.];
    let cursor = [foot[0] - 1.2, foot[1] + 0.9, foot[2] + 0.5];
    let moved = initial
        .drag_preferred(pid(3), point(cursor), TANGENT, || false)
        .unwrap();
    for (coordinate, expected) in where_point(&moved.update.sketch, 3).into_iter().zip(foot) {
        near(coordinate, expected, 1e-8);
    }
    let current = moved.update.sketch;
    let normal = [foot[0] + 0.4, foot[1] - 0.3, foot[2] - 2.];
    let stationary = current
        .drag_preferred(pid(3), point(normal), TANGENT, || false)
        .unwrap();
    assert_eq!(stationary.update.sketch.geometry(), current.geometry());
    assert_eq!(
        stationary.preference.settlement,
        PreferenceSettlement::FirstOrderStationary
    );
}

#[test]
fn preferred_circle_reports_first_order_center_normal_and_antipodal_stops_without_motion() {
    let sketch = circle();
    for x in [0., -5., 2., 9.] {
        let moved = sketch
            .drag_preferred(pid(2), point([x, 0., 0.]), TANGENT, || false)
            .unwrap();
        assert_eq!(where_point(&moved.update.sketch, 2), [5., 0., 0.]);
        assert_eq!(
            moved.preference.settlement,
            PreferenceSettlement::FirstOrderStationary
        );
        assert_eq!(moved.preference.max_tangential_gradient, 0.);
    }
    let fixed = sketch
        .drag_preferred(pid(1), point([12., 2., 3.]), TANGENT, || false)
        .unwrap();
    assert_eq!(fixed.update.sketch.geometry(), sketch.geometry());
    assert_eq!(
        fixed.preference.settlement,
        PreferenceSettlement::FirstOrderStationary
    );
}

#[test]
fn mathematically_reached_cursor_respects_independent_line_geometry_admission() {
    let prior = author(
        &[[0.; 3], [5., 0., 0.]],
        &[
            RelationSpec::FixPoint(pid(1)),
            RelationSpec::ParallelToAxis(lid(10), CoordinateAxis::X, AxisDirection::Positive),
        ],
    );
    let target = point([0.00025, 0., 0.]);
    assert!(matches!(
        prior.drag_preferred(pid(2), target, TANGENT, || false),
        Err(ConstraintFailure::Geometry(_))
    ));
    assert_eq!(where_point(&prior, 2), [5., 0., 0.]);
}

#[test]
fn preferred_publication_can_refresh_a_chart_and_recheck_same_pose_within_the_tick() {
    let mut sketch = author(
        &[[0.; 3], [2., 0., 0.], [4., 0., 0.]],
        &[
            RelationSpec::FixPoint(pid(1)),
            RelationSpec::PointOnLine(pid(3), lid(10)),
        ],
    );
    for target in [[2., 0.8, 0.], [1.4, 1.6, 0.], [1.1, 1.65, 0.], [0., 2., 0.]] {
        let moved = sketch
            .drag_preferred(pid(2), point(target), TANGENT, || false)
            .unwrap();
        assert_eq!(
            moved.preference.settlement,
            PreferenceSettlement::TargetReached
        );
        assert!(moved.update.residual_work_bound <= policy().residual_work);
        assert!(
            moved
                .update
                .sketch
                .geometry()
                .point(pid(2))
                .unwrap()
                .position
                .distance(&point(target))
                < 1e-8
        );
        sketch = moved.update.sketch;
    }
}

#[test]
fn a_captured_external_source_still_requires_explicit_detach_before_pointer_motion() {
    use std::num::NonZeroU64;
    let source = FrozenPointSource {
        document: [7; 16],
        sketch: NonZeroU64::new(50).unwrap(),
        revision: FrozenSourceRevision::Snapshot {
            incarnation: 1,
            revision: 3,
        },
        point: pid(50),
    };
    let edit = Edit::InsertFrozenPoint {
        point: SketchPoint {
            id: pid(1),
            role: PointRole::SketchMarker,
            position: point([-0., 1., 2.]),
        },
        source,
    };
    let data = Snapshot::empty(length(0.001))
        .apply(&[edit], || false)
        .unwrap();
    let sketch = ConstrainedSketch::new(data, policy()).unwrap();
    assert!(matches!(
        sketch.drag_preferred(pid(1), point([2., 1., 2.]), TANGENT, || false),
        Err(ConstraintFailure::FrozenDrag(id, recorded)) if id == pid(1) && recorded == source
    ));
    assert_eq!(sketch.geometry().frozen_source(pid(1)), Some(source));
    assert_eq!(where_point(&sketch, 1)[0].to_bits(), (-0.0_f64).to_bits());
}

#[test]
fn preferred_circle_replays_exactly_in_independent_threads() {
    let accepted = circle();
    let expected = circle_receipt(&accepted);
    println!("preferred gesture bits {expected:?}");
    std::thread::scope(|scope| {
        let workers = (0..4)
            .map(|_| {
                scope.spawn(|| {
                    for _ in 0..16 {
                        assert_eq!(circle_receipt(&accepted), expected);
                    }
                })
            })
            .collect::<Vec<_>>();
        for worker in workers {
            worker.join().unwrap();
        }
    });
    assert_eq!(where_point(&accepted, 2), [5., 0., 0.]);
}

fn circle_receipt(initial: &ConstrainedSketch) -> Vec<([u64; 3], u64, u64, usize)> {
    let mut sketch = initial.clone();
    [0.1, 0.2, 0.3, 0.4]
        .into_iter()
        .map(|angle| {
            let (s, c) = martensite_cmath::sincos(angle);
            let moved = sketch
                .drag_preferred(pid(2), point([7. * c, 7. * s, 0.25]), TANGENT, || false)
                .unwrap();
            sketch = moved.update.sketch;
            (
                where_point(&sketch, 2).map(f64::to_bits),
                moved.update.solver_hard_error.to_bits(),
                moved.preference.max_tangential_gradient.to_bits(),
                moved.update.residual_work_bound,
            )
        })
        .collect()
}

fn rail() -> ConstrainedSketch {
    author(
        &[[0.; 3], [4., 0., 0.], [0.5, 0., 0.]],
        &[
            RelationSpec::FixLine(lid(10)),
            RelationSpec::PointOnLine(pid(3), lid(10)),
        ],
    )
}

fn circle() -> ConstrainedSketch {
    author(
        &[[0.; 3], [5., 0., 0.]],
        &[
            RelationSpec::FixPoint(pid(1)),
            RelationSpec::Length(lid(10), length(5.)),
            RelationSpec::ParallelToPlane(lid(10), Direction3::Z),
        ],
    )
}

fn author(points: &[[f64; 3]], relations: &[RelationSpec]) -> ConstrainedSketch {
    let mut edits = points
        .iter()
        .enumerate()
        .map(|(i, &p)| seed(i as u64 + 1, p, PointRole::Defining))
        .collect::<Vec<_>>();
    edits.push(line(10, 1, 2));
    let geometry = Snapshot::empty(length(0.001))
        .apply(&edits, || false)
        .unwrap();
    let additions = relations
        .iter()
        .enumerate()
        .map(|(i, &spec)| RelationEdit::Insert(rid(i as u64 + 1), spec))
        .collect::<Vec<_>>();
    ConstrainedSketch::new(geometry, policy())
        .unwrap()
        .apply(&additions, || false)
        .unwrap()
}

fn where_point(sketch: &ConstrainedSketch, id: u64) -> [f64; 3] {
    *sketch
        .geometry()
        .point(pid(id))
        .unwrap()
        .position
        .coordinates()
}

fn near(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{actual} versus {expected}, tolerance {tolerance}"
    );
}
