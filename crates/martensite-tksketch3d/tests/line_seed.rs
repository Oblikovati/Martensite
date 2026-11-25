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
