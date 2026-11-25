// SPDX-License-Identifier: GPL-2.0-only
use crate::fixture::{lid, line, pid, point, seed};
use martensite_tkmath::gp::Direction3;
use martensite_tksketch3d::{
    ArcDefinition, ArcFailure, ArcFitTolerance, ArcGeometry, ArcId, BatchPhase, Edit, EditFailure,
    EntityId, MinimumLength, PointRole, SketchArc, Snapshot,
};
use std::{
    f64::consts::{FRAC_PI_2, PI},
    num::NonZeroU64,
};

fn aid() -> ArcId {
    ArcId::new(NonZeroU64::new(8).unwrap())
}

fn sketch() -> Snapshot {
    Snapshot::empty(MinimumLength::new(0.01).unwrap())
}

fn through() -> ArcDefinition {
    ArcDefinition::Through { witness: pid(3) }
}

fn centered(normal: Direction3) -> ArcDefinition {
    ArcDefinition::Center {
        center: pid(3),
        normal,
    }
}

fn insert_arc(definition: ArcDefinition, tolerance: f64) -> Edit {
    Edit::InsertArc(SketchArc {
        id: aid(),
        endpoints: [pid(1), pid(2)],
        definition,
        fit_tolerance: ArcFitTolerance::new(tolerance).unwrap(),
    })
}

fn create(
    points: [[f64; 3]; 3],
    definition: ArcDefinition,
    tolerance: f64,
) -> Result<Snapshot, EditFailure> {
    sketch().apply(
        &[
            insert_arc(definition, tolerance),
            seed(1, points[0], PointRole::Defining),
            seed(2, points[1], PointRole::SketchMarker),
            seed(3, points[2], PointRole::Defining),
        ],
        || false,
    )
}

fn assert_xyz(actual: martensite_tkmath::gp::Point3, expected: [f64; 3]) {
    for (actual, expected) in actual.coordinates().iter().zip(expected) {
        assert!((actual - expected).abs() < 1e-12, "{actual} != {expected}");
    }
}

fn assert_circle(arc: ArcGeometry, center: [f64; 3], radius: f64, sweep: f64) {
    assert_xyz(arc.frame().location(), center);
    assert!((arc.radius() - radius).abs() < 1e-12);
    assert!((arc.range()[1] - sweep).abs() < 1e-12);
    assert!(arc.deviation_estimate() < 1e-12);
    assert_eq!(arc.range()[0], 0.);
    assert!(arc.value(-1.).is_none());
    assert!(arc.value(arc.range()[1].next_up()).is_none());
    assert!(arc.value(f64::NAN).is_none());
}

#[test]
fn three_point_arc_replays_stated_semicircle_and_non_xy_placement() {
    let sketch_2d = create(
        [[10., 0., 0.], [30., 0., 0.], [20., 10., 0.]],
        through(),
        1e-9,
    )
    .unwrap();
    let arc = sketch_2d.arc(aid()).unwrap();
    assert_circle(arc, [20., 0., 0.], 10., PI);
    assert_xyz(arc.value(FRAC_PI_2).unwrap(), [20., 10., 0.]);
    let spatial = [[7., 10., 0.], [7., -10., 0.], [7., 0., 10.]];
    let sketch = create(spatial, through(), 1e-9).unwrap();
    let arc = sketch.arc(aid()).unwrap();
    assert_eq!(arc.points(), spatial.map(point));
    assert_eq!(arc.recipe().endpoints, [pid(1), pid(2)]);
    assert_circle(arc, [7., 0., 0.], 10., PI);
    assert_xyz(arc.value(FRAC_PI_2).unwrap(), spatial[2]);
    assert_eq!(
        sketch.visible_ids().collect::<Vec<_>>(),
        [EntityId::Point(pid(2)), EntityId::Arc(aid())]
    );
}

#[test]
fn witness_selects_major_or_minor_trim_of_same_circle() {
    let a = [5., 0., 0.];
    let b = [0., 5., 0.];
    let major = create([a, b, [0., -5., 0.]], through(), 1e-9)
        .unwrap()
        .arc(aid())
        .unwrap();
    assert_circle(major, [0.; 3], 5., 3. * FRAC_PI_2);
    assert_xyz(major.value(PI).unwrap(), [-5., 0., 0.]);
    let minor = create([a, b, [3., 4., 0.]], through(), 1e-9)
        .unwrap()
        .arc(aid())
        .unwrap();
    assert_circle(minor, [0.; 3], 5., FRAC_PI_2);
}

#[test]
fn center_and_oriented_normal_select_directed_branch_and_semicircle_plane() {
    let points = [[30., 0., 0.], [20., 10., 0.], [20., 0., 0.]];
    let minor = create(points, centered(Direction3::Z), 1e-9)
        .unwrap()
        .arc(aid())
        .unwrap();
    assert_circle(minor, points[2], 10., FRAC_PI_2);
    let major = create(points, centered(Direction3::Z.reversed()), 1e-9)
        .unwrap()
        .arc(aid())
        .unwrap();
    assert_circle(major, points[2], 10., 3. * FRAC_PI_2);
    let half = [[5., 0., 0.], [-5., 0., 0.], [0.; 3]];
    let xy = create(half, centered(Direction3::Z), 1e-9)
        .unwrap()
        .arc(aid())
        .unwrap();
    let xz = create(half, centered(Direction3::Y), 1e-9)
        .unwrap()
        .arc(aid())
        .unwrap();
    assert_circle(xy, [0.; 3], 5., PI);
    assert_circle(xz, [0.; 3], 5., PI);
    assert_xyz(xy.value(FRAC_PI_2).unwrap(), [0., 5., 0.]);
    assert_xyz(xz.value(FRAC_PI_2).unwrap(), [0., 0., -5.]);
}

#[test]
fn off_radius_and_off_plane_center_fixtures_refuse_beyond_requested_fit() {
    for end in [[0., 6., 0.], [0., 5., 1.]] {
        let result = create([[5., 0., 0.], end, [0.; 3]], centered(Direction3::Z), 0.25);
        assert!(
            matches!(result, Err(EditFailure::Arc { arc, source: ArcFailure::FitExceeded { point, estimate, .. } }) if arc == aid() && point == pid(2) && estimate >= 1.)
        );
    }
    let round = create(
        [[5., 0., 0.], [0., 5.001, 0.], [0.; 3]],
        centered(Direction3::Z),
        0.01,
    )
    .unwrap();
    let arc = round.arc(aid()).unwrap();
    assert_eq!(arc.points()[1], point([0., 5.001, 0.]));
    assert!((arc.deviation_estimate() - 0.001).abs() < 1e-12);
    assert_xyz(arc.value(arc.range()[1]).unwrap(), [0., 5., 0.]);
}

#[test]
fn degenerate_placement_and_identity_failures_are_typed() {
    let is_arc_error = |result: Result<Snapshot, EditFailure>, source| {
        assert_eq!(result, Err(EditFailure::Arc { arc: aid(), source }));
    };
    is_arc_error(
        create([[0.; 3], [0.; 3], [0., 1., 0.]], through(), 1e-9),
        ArcFailure::ShortChord,
    );
    is_arc_error(
        create([[0.; 3], [2., 0., 0.], [1., 0.005, 0.]], through(), 1e-9),
        ArcFailure::SmallWitnessAltitude,
    );
    is_arc_error(
        create(
            [[0.; 3], [2., 0., 0.], [0.; 3]],
            centered(Direction3::Z),
            1e-9,
        ),
        ArcFailure::ShortRadius(pid(1)),
    );
    is_arc_error(
        sketch().apply(
            &[insert_arc(ArcDefinition::Through { witness: pid(1) }, 1e-9)],
            || false,
        ),
        ArcFailure::RepeatedPoint(pid(1)),
    );
    is_arc_error(
        sketch().apply(&[insert_arc(through(), 1e-9)], || false),
        ArcFailure::MissingPoint(pid(1)),
    );
    assert_eq!(
        ArcFitTolerance::new(0.),
        Err(ArcFailure::InvalidFitTolerance(0.))
    );
}

#[test]
fn point_edits_revalidate_the_arc_and_deletion_prunes_only_unused_points() {
    let before = create([[0.; 3], [10., 0., 0.], [5., 5., 0.]], through(), 1e-9).unwrap();
    let baseline = before.arc(aid()).unwrap();
    let failed = before.apply(
        &[Edit::MovePoint {
            point: pid(3),
            position: point([5., 0., 0.]),
        }],
        || false,
    );
    assert_eq!(
        failed,
        Err(EditFailure::Arc {
            arc: aid(),
            source: ArcFailure::SmallWitnessAltitude
        })
    );
    let changed = before
        .apply(
            &[
                Edit::MovePoint {
                    point: pid(3),
                    position: point([5., 0., 5.]),
                },
                line(4, 1, 2),
            ],
            || false,
        )
        .unwrap();
    assert_circle(changed.arc(aid()).unwrap(), [5., 0., 0.], 5., PI);
    assert_xyz(
        changed.arc(aid()).unwrap().value(FRAC_PI_2).unwrap(),
        [5., 0., 5.],
    );
    assert_eq!(before.arc(aid()), Some(baseline));
    let removed = changed
        .apply(&[Edit::RemovePoint(pid(3))], || false)
        .unwrap();
    assert!(removed.arc(aid()).is_none());
    assert!(removed.line(lid(4)).is_some());
    assert!(removed.point(pid(1)).is_some());
    let removed = before.apply(&[Edit::RemoveArc(aid())], || false).unwrap();
    assert_eq!(
        removed.owned_ids().collect::<Vec<_>>(),
        [EntityId::Point(pid(2))]
    );
}

#[test]
fn arc_validation_can_cancel_after_staging() {
    let before = create([[0.; 3], [10., 0., 0.], [5., 5., 0.]], through(), 1e-9).unwrap();
    let mut polls = 0;
    let result = before.apply(
        &[Edit::MovePoint {
            point: pid(3),
            position: point([5., 6., 0.]),
        }],
        || {
            polls += 1;
            polls == 3
        },
    );
    assert_eq!(result, Err(EditFailure::Cancelled(BatchPhase::Validation)));
    assert_eq!(before.arc(aid()).unwrap().points()[2], point([5., 5., 0.]));
}

#[test]
fn carrier_construction_removes_scale_from_squared_norms() {
    for scale in [
        f64::from_bits(423_u64 << 52),
        f64::from_bits(1623_u64 << 52),
    ] {
        let snapshot = Snapshot::empty(MinimumLength::new(scale / 16.).unwrap());
        let sample = [
            [2. * scale, 0., 0.],
            [-2. * scale, 0., 0.],
            [0., 2. * scale, 0.],
        ];
        let edits = [
            insert_arc(through(), scale * 1e-10),
            seed(1, sample[0], PointRole::Defining),
            seed(2, sample[1], PointRole::Defining),
            seed(3, sample[2], PointRole::Defining),
        ];
        let sketch = snapshot.apply(&edits, || false).unwrap();
        let arc = sketch.arc(aid()).unwrap();
        assert!((arc.radius() / scale - 2.).abs() < 1e-12);
        assert!((arc.range()[1] - PI).abs() < 1e-12);
        assert_eq!(arc.points(), sample.map(point));
        assert!(arc.deviation_estimate() < scale * 1e-10);
    }
}

#[test]
fn finite_endpoints_with_overflowing_displacement_refuse() {
    let points = [[f64::MAX, 0., 0.], [-f64::MAX, 0., 0.], [0., 1., 0.]];
    assert_eq!(
        create(points, through(), 1e-9),
        Err(EditFailure::Arc {
            arc: aid(),
            source: ArcFailure::UnrepresentableCarrier
        }),
    );
}

#[test]
fn point_and_line_solver_refuses_an_arc_layout_it_cannot_rank() {
    use martensite_tksketch3d::constraints::{ConstrainedSketch, ConstraintFailure};

    let geometry = create([[0.; 3], [10., 0., 0.], [5., 5., 0.]], through(), 1e-9).unwrap();
    let result = ConstrainedSketch::new(geometry, crate::constraints::policy());
    assert!(
        matches!(result, Err(ConstraintFailure::UnsupportedEntity(EntityId::Arc(id))) if id == aid())
    );
}

#[test]
fn a_rounded_oblique_frame_preserves_the_exact_antipodal_branch() {
    let normal = Direction3::new(13., 241., 343.).unwrap();
    let start = [187691., 145488., -109337.];
    let placements = [start, start.map(|component| -component), [0.; 3]];
    let arc = create(placements, centered(normal), 1e-8)
        .unwrap()
        .arc(aid())
        .unwrap();
    assert_eq!(arc.range(), [0., PI]);
    assert_eq!(arc.points(), placements.map(point));
    assert!(arc.deviation_estimate() < 1e-8);
}
