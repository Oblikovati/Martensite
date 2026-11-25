// SPDX-License-Identifier: GPL-2.0-only
use crate::{
    constraints::{length, policy, rid},
    fixture::{lid, line, pid, point, seed},
};
use martensite_tkmath::gp::Direction3;
use martensite_tksketch3d::{
    Edit, FrozenPointSource, LineId, PointRole, SketchPoint, Snapshot,
    constraints::{ConstrainedSketch, ConstraintFailure, RelationEdit, RelationSpec},
};
use std::num::NonZeroU64;

#[test]
fn perpendicular_authoring_slopes_either_parallel_sign_without_moving_fixed_or_frozen_anchors() {
    for (sign, frozen, transverse) in [(1., false, 0.), (-1., true, 0.), (1., true, 1e-12)] {
        let (geometry, source) = parallel_lines(sign, frozen, transverse);
        let prior = ConstrainedSketch::new(geometry.clone(), policy()).unwrap();
        let mut edits = vec![
            RelationEdit::Insert(rid(1), RelationSpec::FixLine(lid(10))),
            RelationEdit::Insert(rid(2), RelationSpec::Perpendicular([lid(10), lid(11)])),
        ];
        if !frozen {
            edits.push(RelationEdit::Insert(rid(3), RelationSpec::FixPoint(pid(3))));
        }
        let result = prior.apply(&edits, || false).unwrap();
        let result_geometry = result.geometry();
        assert!(
            cosine(
                displacement(result_geometry, lid(10)),
                displacement(result_geometry, lid(11))
            )
            .abs()
                < 2e-8
        );
        assert_eq!(result_geometry.line(lid(10)), geometry.line(lid(10)));
        assert_eq!(result_geometry.point(pid(3)), geometry.point(pid(3)));
        assert_eq!(
            result_geometry
                .point(pid(3))
                .unwrap()
                .position
                .coordinates()
                .map(f64::to_bits),
            geometry
                .point(pid(3))
                .unwrap()
                .position
                .coordinates()
                .map(f64::to_bits),
        );
        assert_eq!(result_geometry.frozen_source(pid(3)), source);
        let rank = result.inspect_rank().unwrap();
        assert_eq!((rank.rank, rank.degrees_of_freedom), (10, 2));
        assert_eq!(rank.frozen_coordinates, usize::from(frozen) * 3);
        assert_eq!(prior.geometry(), &geometry);
        assert_eq!(prior.relations().count(), 0);
        assert_eq!(
            prior.apply(&edits, || false).unwrap().geometry(),
            result_geometry
        );
    }
}

#[test]
fn plane_parallel_authoring_tips_both_normal_signs_and_keeps_the_fixed_start() {
    let n = Direction3::new(1., 2., 2.).unwrap();
    for sign in [1., -1.] {
        let start = [5., 6., 7.];
        let geometry = Snapshot::empty(length(0.01))
            .apply(
                &[
                    seed(1, start, PointRole::Defining),
                    seed(
                        2,
                        [5. + sign, 6. + 2. * sign, 7. + 2. * sign],
                        PointRole::Defining,
                    ),
                    line(3, 1, 2),
                ],
                || false,
            )
            .unwrap();
        let base = ConstrainedSketch::new(geometry.clone(), policy()).unwrap();
        let sketch = base
            .apply(
                &[
                    RelationEdit::Insert(rid(1), RelationSpec::FixPoint(pid(1))),
                    RelationEdit::Insert(rid(2), RelationSpec::ParallelToPlane(lid(3), n)),
                ],
                || false,
            )
            .unwrap();
        assert!(cosine(displacement(sketch.geometry(), lid(3)), [1., 2., 2.]).abs() < 2e-8);
        assert_eq!(
            sketch.geometry().point(pid(1)).unwrap().position,
            point(start)
        );
        let rank = sketch.inspect_rank().unwrap();
        assert_eq!((rank.rank, rank.degrees_of_freedom), (4, 2));
        assert_eq!(base.geometry(), &geometry);
    }
}

#[test]
fn perpendicular_can_initialize_the_sole_mobile_shared_point_from_either_side_of_its_fixed_anchors()
{
    for shared in [-2., 2.] {
        let geometry = Snapshot::empty(length(0.01))
            .apply(
                &[
                    seed(1, [shared, 0., 0.], PointRole::Defining),
                    seed(2, [0., 0., 0.], PointRole::Defining),
                    seed(3, [4., 0., 0.], PointRole::Defining),
                    line(4, 1, 2),
                    line(5, 1, 3),
                ],
                || false,
            )
            .unwrap();
        let sketch = ConstrainedSketch::new(geometry.clone(), policy())
            .unwrap()
            .apply(
                &[
                    RelationEdit::Insert(rid(1), RelationSpec::FixPoint(pid(2))),
                    RelationEdit::Insert(rid(2), RelationSpec::FixPoint(pid(3))),
                    RelationEdit::Insert(rid(3), RelationSpec::Perpendicular([lid(4), lid(5)])),
                ],
                || false,
            )
            .unwrap();
        assert!(
            cosine(
                displacement(sketch.geometry(), lid(4)),
                displacement(sketch.geometry(), lid(5))
            )
            .abs()
                < 2e-8
        );
        assert_eq!(sketch.geometry().point(pid(2)), geometry.point(pid(2)));
        assert_eq!(sketch.geometry().point(pid(3)), geometry.point(pid(3)));
        assert_eq!(
            sketch.geometry().line(lid(4)).unwrap().line.endpoints,
            [pid(1), pid(2)]
        );
        assert_eq!(
            sketch.geometry().line(lid(5)).unwrap().line.endpoints,
            [pid(1), pid(3)]
        );
    }
}

#[test]
fn fully_fixed_flat_relations_keep_their_authored_conflict_and_cancelled_seeds_stay_private() {
    let (geometry, _) = parallel_lines(1., false, 0.);
    let base = ConstrainedSketch::new(geometry.clone(), policy()).unwrap();
    for spec in [
        RelationSpec::Perpendicular([lid(10), lid(11)]),
        RelationSpec::ParallelToPlane(lid(11), Direction3::X),
    ] {
        let edits = [
            RelationEdit::Insert(rid(1), RelationSpec::FixLine(lid(10))),
            RelationEdit::Insert(rid(2), RelationSpec::FixLine(lid(11))),
            RelationEdit::Insert(rid(3), spec),
        ];
        assert!(
            matches!(base.apply(&edits,|| false),Err(ConstraintFailure::Unsettled(_,ids)) if ids.contains(&rid(3)))
        );
    }
    let edits = [RelationEdit::Insert(
        rid(1),
        RelationSpec::Perpendicular([lid(10), lid(11)]),
    )];
    let mut total = 0;
    base.apply(&edits, || {
        total += 1;
        false
    })
    .unwrap();
    let mut polls = 0;
    assert!(matches!(
        base.apply(&edits, || {
            polls += 1;
            polls == total
        }),
        Err(ConstraintFailure::Cancelled)
    ));
    assert_eq!(base.geometry(), &geometry);
    assert_eq!(base.relations().count(), 0);
}

fn parallel_lines(
    sign: f64,
    frozen: bool,
    transverse: f64,
) -> (Snapshot, Option<FrozenPointSource>) {
    let anchor = [-0., 2., 1.];
    let source = frozen.then(|| FrozenPointSource {
        document: [7; 16],
        sketch: NonZeroU64::new(50).unwrap(),
        incarnation: 1,
        revision: 3,
        point: pid(32),
    });
    let third = source.map_or_else(
        || seed(3, anchor, PointRole::SketchMarker),
        |source| Edit::InsertFrozenPoint {
            point: SketchPoint {
                id: pid(3),
                role: PointRole::SketchMarker,
                position: point(anchor),
            },
            source,
        },
    );
    let geometry = Snapshot::empty(length(0.01))
        .apply(
            &[
                seed(1, [0., 0., 0.], PointRole::Defining),
                seed(2, [5., 0., 0.], PointRole::Defining),
                third,
                seed(4, [4. * sign, 2. + transverse, 1.], PointRole::Defining),
                line(10, 1, 2),
                line(11, 3, 4),
            ],
            || false,
        )
        .unwrap();
    (geometry, source)
}

fn displacement(geometry: &Snapshot, id: LineId) -> [f64; 3] {
    let [start, end] = geometry.line(id).unwrap().points;
    std::array::from_fn(|i| end.coordinates()[i] - start.coordinates()[i])
}

fn cosine(a: [f64; 3], b: [f64; 3]) -> f64 {
    let len = |v: [f64; 3]| v.into_iter().map(|x| x * x).sum::<f64>().sqrt();
    a.into_iter().zip(b).map(|(a, b)| a * b).sum::<f64>() / (len(a) * len(b))
}

#[test]
fn plane_seed_preserves_an_existing_incident_length_when_a_tip_would_collapse_its_line() {
    let geometry = Snapshot::empty(length(0.01))
        .apply(
            &[
                seed(1, [0., 0., 0.], PointRole::Defining),
                seed(2, [8., 0., 0.], PointRole::Defining),
                seed(3, [8., 1., 0.], PointRole::Defining),
                line(10, 1, 2),
                line(11, 2, 3),
            ],
            || false,
        )
        .unwrap();
    let prior = ConstrainedSketch::new(geometry, policy())
        .unwrap()
        .apply(
            &[
                RelationEdit::Insert(rid(1), RelationSpec::FixPoint(pid(3))),
                RelationEdit::Insert(rid(2), RelationSpec::Length(lid(11), length(1.))),
            ],
            || false,
        )
        .unwrap();
    let next = prior.apply(&[plane_parallel_edit()], || false).unwrap();
    assert_plane_x(next.geometry());
    assert!((next.geometry().line(lid(11)).unwrap().length_estimate - 1.).abs() < 2e-7);
    assert_eq!(
        next.geometry().point(pid(3)),
        prior.geometry().point(pid(3))
    );
    assert_eq!(
        prior.geometry().point(pid(2)).unwrap().position,
        point([8., 0., 0.])
    );
}

#[test]
fn plane_seed_checks_both_incident_blockers_when_only_the_tip_is_movable() {
    let frozen = frozen_anchor(1, [0., 0., 0.]);
    let geometry = Snapshot::empty(length(0.01))
        .apply(
            &[
                frozen,
                seed(2, [8., 0., 0.], PointRole::Defining),
                seed(3, [8., 1., 0.], PointRole::Defining),
                seed(4, [8., -1., 0.], PointRole::Defining),
                line(10, 1, 2),
                line(11, 2, 3),
                line(12, 2, 4),
            ],
            || false,
        )
        .unwrap();
    let base = ConstrainedSketch::new(geometry.clone(), policy()).unwrap();
    let edits = [
        RelationEdit::Insert(rid(1), RelationSpec::Length(lid(11), length(1.))),
        RelationEdit::Insert(rid(2), RelationSpec::Length(lid(12), length(1.))),
        plane_parallel_edit(),
    ];
    let mut polls = 0;
    let sketch = base
        .apply(&edits, || {
            polls += 1;
            false
        })
        .unwrap();
    assert_cancelled_at_every_poll(&base, &edits, polls);
    assert_plane_x(sketch.geometry());
    assert_eq!(sketch.geometry().point(pid(1)), geometry.point(pid(1)));
    assert_eq!(
        sketch.geometry().frozen_source(pid(1)),
        geometry.frozen_source(pid(1))
    );
    for line in [11, 12] {
        assert!((sketch.geometry().line(lid(line)).unwrap().length_estimate - 1.).abs() < 2e-7);
    }
}

#[test]
fn noncollapsing_candidate_that_breaks_an_existing_angle_domain_is_skipped() {
    use martensite_tksketch3d::constraints::LineAngle;
    let geometry = Snapshot::empty(length(0.01))
        .apply(
            &[
                frozen_anchor(1, [0., 0., 0.]),
                seed(2, [8., 0., 0.], PointRole::Defining),
                seed(3, [8., 1., 1.], PointRole::Defining),
                seed(4, [20., 0., 0.], PointRole::Defining),
                seed(5, [20., 0., 4.], PointRole::Defining),
                line(10, 1, 2),
                line(11, 2, 3),
                line(12, 4, 5),
            ],
            || false,
        )
        .unwrap();
    let old = ConstrainedSketch::new(geometry.clone(), policy())
        .unwrap()
        .apply(
            &[
                RelationEdit::Insert(rid(1), RelationSpec::FixPoint(pid(3))),
                RelationEdit::Insert(rid(2), RelationSpec::FixLine(lid(12))),
                RelationEdit::Insert(
                    rid(3),
                    RelationSpec::Angle(
                        [lid(11), lid(12)],
                        LineAngle::new(std::f64::consts::FRAC_PI_4).unwrap(),
                    ),
                ),
            ],
            || false,
        )
        .unwrap();
    let current = old.apply(&[plane_parallel_edit()], || false).unwrap();
    assert_plane_x(current.geometry());
    let [dx, dy, dz] = displacement(current.geometry(), lid(11));
    assert!(dz > 0.);
    assert!(
        (martensite_cmath::atan2((dx * dx + dy * dy).sqrt(), dz) - std::f64::consts::FRAC_PI_4)
            .abs()
            < 2e-8
    );
    assert_eq!(
        current.geometry().frozen_source(pid(1)),
        geometry.frozen_source(pid(1))
    );
    assert_eq!(current.geometry().point(pid(3)), geometry.point(pid(3)));
    assert_eq!(
        old.geometry().point(pid(2)).unwrap().position,
        point([8., 0., 0.])
    );
}

#[test]
fn shared_point_sphere_selection_skips_an_equator_point_that_collapses_a_third_line() {
    let geometry = Snapshot::empty(length(0.01))
        .apply(
            &[
                seed(1, [2., 0., 0.], PointRole::Defining),
                frozen_anchor(2, [0., 0., 0.]),
                seed(3, [4., 0., 0.], PointRole::Defining),
                seed(4, [2., 0., -2.], PointRole::Defining),
                line(10, 1, 2),
                line(11, 1, 3),
                line(12, 1, 4),
            ],
            || false,
        )
        .unwrap();
    let next = ConstrainedSketch::new(geometry.clone(), policy())
        .unwrap()
        .apply(
            &[
                RelationEdit::Insert(rid(1), RelationSpec::Length(lid(12), length(2.))),
                RelationEdit::Insert(rid(2), RelationSpec::FixPoint(pid(3))),
                RelationEdit::Insert(rid(3), RelationSpec::Perpendicular([lid(10), lid(11)])),
            ],
            || false,
        )
        .unwrap();
    assert!(
        cosine(
            displacement(next.geometry(), lid(10)),
            displacement(next.geometry(), lid(11))
        )
        .abs()
            < 2e-8
    );
    assert!((next.geometry().line(lid(12)).unwrap().length_estimate - 2.).abs() < 2e-7);
    assert_eq!(next.geometry().point(pid(2)), geometry.point(pid(2)));
    assert_eq!(
        next.geometry().frozen_source(pid(2)),
        geometry.frozen_source(pid(2))
    );
}

fn assert_cancelled_at_every_poll(base: &ConstrainedSketch, edits: &[RelationEdit], total: usize) {
    let prior = base.geometry().clone();
    for cancellation in 1..=total {
        let mut polls = 0;
        let result = base.apply(edits, || {
            polls += 1;
            polls >= cancellation
        });
        assert!(
            matches!(result, Err(ConstraintFailure::Cancelled)),
            "poll {cancellation} of {total}: {:?}",
            result.err()
        );
        assert_eq!(base.geometry(), &prior);
        assert_eq!(base.relations().count(), 0);
    }
}

fn plane_parallel_edit() -> RelationEdit {
    RelationEdit::Insert(
        rid(9),
        RelationSpec::ParallelToPlane(lid(10), Direction3::X),
    )
}

fn assert_plane_x(geometry: &Snapshot) {
    assert!(cosine(displacement(geometry, lid(10)), [1., 0., 0.]).abs() < 2e-8);
}

fn frozen_anchor(id: u64, location: [f64; 3]) -> Edit {
    Edit::InsertFrozenPoint {
        point: SketchPoint {
            id: pid(id),
            role: PointRole::Defining,
            position: point(location),
        },
        source: FrozenPointSource {
            document: [8; 16],
            sketch: NonZeroU64::new(50).unwrap(),
            incarnation: 2,
            revision: 5,
            point: pid(33),
        },
    }
}
