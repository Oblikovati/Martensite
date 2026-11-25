// SPDX-License-Identifier: GPL-2.0-only
mod precision;
mod replay;

use crate::fixture::{lid, line, pid, point, seed};
use martensite_tkmath::predicates::roots::metric_bounds::BezierRegularityDecision;
use martensite_tksketch3d::{
    BatchPhase, ControlSplineFailure, ControlSplineGeometry, ControlSplineId, ControlSplineJet,
    ControlSplineJetFailure, ControlSplineProofPolicy, CurveId, CurveRole, Edit, EditFailure,
    EntityId, MinimumLength, PointRole, SketchControlSpline, Snapshot,
    constraints::{ConstrainedSketch, ConstraintFailure},
};
use std::{num::NonZeroU64, ptr, sync::Barrier, thread};

const NONPLANAR: [[f64; 3]; 4] = [[0., 0., 0.], [1., 0., 0.], [2., 1., 2.], [3., 1., 0.]];

fn sid(value: u64) -> ControlSplineId {
    ControlSplineId::new(NonZeroU64::new(value).unwrap())
}

fn recipe() -> SketchControlSpline {
    SketchControlSpline {
        id: sid(5),
        poles: [1, 2, 3, 4].map(pid),
        proof: ControlSplineProofPolicy::new(1e-8, 128).unwrap(),
    }
}

fn empty(minimum: f64) -> Snapshot {
    Snapshot::empty(MinimumLength::new(minimum).unwrap())
}

fn batch(poles: [[f64; 3]; 4], recipe: SketchControlSpline) -> [Edit; 5] {
    let [a, b, c, d] = poles;
    [
        Edit::InsertControlSpline(recipe),
        seed(1, a, PointRole::Defining),
        seed(2, b, PointRole::Defining),
        seed(3, c, PointRole::Defining),
        seed(4, d, PointRole::Defining),
    ]
}

fn snapshot(poles: [[f64; 3]; 4]) -> Snapshot {
    empty(0.001)
        .apply(&batch(poles, recipe()), || false)
        .unwrap()
}

fn rows(jet: ControlSplineJet) -> [[f64; 3]; 3] {
    [
        *jet.position.coordinates(),
        *jet.first.xyz().coordinates(),
        *jet.second.xyz().coordinates(),
    ]
}

fn assert_curve_failure(result: Result<Snapshot, EditFailure>, expected: ControlSplineFailure) {
    assert_eq!(
        result,
        Err(EditFailure::ControlSpline {
            spline: sid(5),
            source: expected,
        })
    );
}

#[test]
fn clamped_cubic_retains_exact_source_and_evaluates_the_spatial_bernstein_polynomial() {
    let sketch = snapshot(NONPLANAR);
    let spline = sketch.control_spline(sid(5)).unwrap();
    let proof = spline.regularity();
    assert_eq!(spline.recipe(), recipe());
    assert_eq!(spline.recipe().endpoints(), [pid(1), pid(4)]);
    assert_eq!(spline.poles(), NONPLANAR.map(point));
    assert_eq!(spline.range(), [0., 1.]);
    assert_eq!(proof.source().poles(), &NONPLANAR);
    assert_eq!(proof.source().curve_id(), 5);
    assert_eq!(proof.source().interval(), [0., 1.]);
    assert!(matches!(
        proof.decision(),
        BezierRegularityDecision::Regular { .. }
    ));
    assert_eq!(
        proof.source().options().roots.max_bisections,
        recipe().proof.max_bisections()
    );
    assert_eq!(
        proof.source().options().roots.max_width,
        recipe().proof.parameter_width()
    );
    assert_eq!(spline.value(0.).unwrap(), point(NONPLANAR[0]));
    assert_eq!(spline.value(1.).unwrap(), point(NONPLANAR[3]));
    assert_eq!(spline.value(0.5).unwrap(), point([1.5, 0.5, 0.75]));
    assert_eq!(
        spline.first_derivative(0.5).unwrap().xyz().coordinates(),
        &[3., 1.5, 1.5]
    );
    assert_eq!(
        rows(spline.jet(0.5).unwrap()),
        [[1.5, 0.5, 0.75], [3., 1.5, 1.5], [0., 0., -6.]]
    );
}

#[test]
fn reversing_control_order_reverses_velocity_and_preserves_acceleration() {
    let mut reversed = recipe();
    reversed.poles.reverse();
    let sketch = empty(0.001)
        .apply(&batch(NONPLANAR, reversed), || false)
        .unwrap();
    let spline = sketch.control_spline(sid(5)).unwrap();
    assert_eq!(spline.recipe().endpoints(), [pid(4), pid(1)]);
    assert_eq!(spline.value(0.).unwrap(), point(NONPLANAR[3]));
    assert_eq!(spline.value(1.).unwrap(), point(NONPLANAR[0]));
    assert_eq!(
        rows(spline.jet(0.5).unwrap()),
        [[1.5, 0.5, 0.75], [-3., -1.5, -1.5], [0., 0., -6.]]
    );
}

#[test]
fn g2_owned_line_spline_and_spline_spline_endpoint_jets() {
    let source = snapshot([[0., 0., 0.], [1., 0., 0.], [2., 0., 0.], [3., 1., 0.]]);
    let connected = source
        .apply(
            &[seed(6, [-1., 0., 0.], PointRole::Defining), line(7, 6, 1)],
            || false,
        )
        .unwrap();
    let original = connected.control_spline(sid(5)).unwrap();
    assert_eq!(
        connected.line(lid(7)).unwrap().line.endpoints[1],
        original.recipe().endpoints()[0]
    );
    assert_eq!(
        rows(original.jet(0.).unwrap()),
        [[0.; 3], [3., 0., 0.], [0.; 3]]
    );
    let moved = connected
        .apply(
            &[Edit::MovePoint {
                point: pid(3),
                position: point([2., 1., 0.]),
            }],
            || false,
        )
        .unwrap();
    let right = moved.control_spline(sid(5)).unwrap();
    let expected = [[0.; 3], [3., 0., 0.], [0., 6., 0.]];
    assert_eq!(rows(right.jet(0.).unwrap()), expected);
    let paired = left_spline(&moved);
    let left = paired.control_spline(sid(11)).unwrap();
    assert_eq!(left.recipe().endpoints()[1], right.recipe().endpoints()[0]);
    let left_jet = left.jet(1.).unwrap();
    assert_eq!(rows(left_jet), expected);
    // Here the second derivative is already perpendicular to the velocity.
    let speed = left_jet.first.x();
    assert_eq!(left_jet.second.y() / (speed * speed), 2. / 3.);
    assert_eq!(rows(original.jet(0.).unwrap())[2], [0.; 3]);
}

fn left_spline(source: &Snapshot) -> Snapshot {
    let left = SketchControlSpline {
        id: sid(11),
        poles: [8, 9, 10, 1].map(pid),
        ..recipe()
    };
    source
        .apply(
            &[
                Edit::InsertControlSpline(left),
                seed(8, [-3., 0., 0.], PointRole::Defining),
                seed(9, [-2., 1., 0.], PointRole::Defining),
                seed(10, [-1., 0., 0.], PointRole::Defining),
            ],
            || false,
        )
        .unwrap()
}

#[test]
fn stationary_speed_in_the_interior_refuses_an_entire_coordinate_edit() {
    let source = snapshot([[0., 0., 0.], [1., 0., 0.], [2., 0., 0.], [3., 0., 0.]]);
    let recipe = [
        Edit::MovePoint {
            point: pid(3),
            position: point([0., 0., 0.]),
        },
        Edit::MovePoint {
            point: pid(4),
            position: point([1., 0., 0.]),
        },
    ];
    // The proposed derivative is 3(1-2t)^2: positive at both endpoints and zero at t=1/2.
    assert!(matches!(
        source.apply(&recipe, || false),
        Err(EditFailure::ControlSpline {
            spline,
            source: ControlSplineFailure::Stationary { isolated_parameters: Some(1), .. },
        }) if spline == sid(5)
    ));
    assert_eq!(source.point(pid(3)).unwrap().position.x(), 2.);
    assert_eq!(source.point(pid(4)).unwrap().position.x(), 3.);
}

#[test]
fn work_exhaustion_and_cancellation_never_publish_a_partial_proof() {
    let no_splits = ControlSplineProofPolicy::new(1e-8, 0).unwrap();
    let recipe = SketchControlSpline {
        proof: no_splits,
        ..recipe()
    };
    let source = empty(0.001);
    let admitted = source.apply(&batch(NONPLANAR, recipe), || false).unwrap();
    assert_eq!(
        admitted
            .control_spline(sid(5))
            .unwrap()
            .regularity()
            .bisections(),
        0
    );
    // The derivative has two nondyadic roots, so classification needs subdivision.
    let poles = [[0., 0., 0.], [1., 0., 0.], [-1., 0., 0.], [1., 0., 0.]];
    assert_curve_failure(
        source.apply(&batch(poles, recipe), || false),
        ControlSplineFailure::WorkLimit {
            limit: 0,
            bisections: 0,
        },
    );
    let edits = batch(NONPLANAR, recipe);
    let first_proof_poll = 1 + edits.len() + 1 + 1;
    let mut polls = 0;
    let cancelled = source.apply(&edits, || {
        polls += 1;
        polls == first_proof_poll
    });
    assert_eq!(
        cancelled,
        Err(EditFailure::Cancelled(BatchPhase::Validation))
    );
    assert_eq!(polls, first_proof_poll);
    assert_eq!(source.owned_ids().count(), 0);
}

#[test]
fn invalid_recipes_and_parameters_report_the_offending_operand() {
    assert!(matches!(
        ControlSplineProofPolicy::new(f64::INFINITY, 3),
        Err(ControlSplineFailure::InvalidParameterWidth(f64::INFINITY))
    ));
    let source = empty(0.001);
    let mut invalid = recipe();
    invalid.poles[3] = pid(1);
    assert_curve_failure(
        source.apply(&batch(NONPLANAR, invalid), || false),
        ControlSplineFailure::SameEndpoint(pid(1)),
    );
    invalid = recipe();
    invalid.poles[1] = pid(91);
    assert_curve_failure(
        source.apply(&batch(NONPLANAR, invalid), || false),
        ControlSplineFailure::MissingPole(pid(91)),
    );
    let minimum = empty(4.);
    assert_curve_failure(
        minimum.apply(&batch(NONPLANAR, recipe()), || false),
        ControlSplineFailure::ShortChord(MinimumLength::new(4.).unwrap()),
    );
    let accepted = snapshot(NONPLANAR);
    let spline = accepted.control_spline(sid(5)).unwrap();
    for parameter in [f64::NAN, -0.001, 1.001] {
        assert!(
            matches!(spline.value(parameter), Err(ControlSplineJetFailure::Parameter(actual)) if actual.to_bits() == parameter.to_bits())
        );
        assert!(
            matches!(spline.jet(parameter), Err(ControlSplineJetFailure::Parameter(actual)) if actual.to_bits() == parameter.to_bits())
        );
        assert!(
            matches!(spline.first_derivative(parameter), Err(ControlSplineJetFailure::Parameter(actual)) if actual.to_bits() == parameter.to_bits())
        );
    }
}

#[test]
fn exact_regularity_does_not_hide_a_nonfinite_native_jet() {
    let large = f64::MAX;
    let spline = snapshot([
        [0., 0., 0.],
        [large / 2., 0., 0.],
        [large / 2., 0., 0.],
        [large, 0., 0.],
    ]);
    let spline = spline.control_spline(sid(5)).unwrap();
    assert_eq!(spline.value(0.).unwrap(), point([0.; 3]));
    assert!(matches!(
        spline.jet(0.),
        Err(ControlSplineJetFailure::NonFinite {
            parameter: 0.,
            order: 1,
            axis: 0,
            ..
        })
    ));
}

#[test]
fn role_only_reuses_the_proof_and_a_bit_change_rebinds_the_exact_poles() {
    let source = snapshot(NONPLANAR);
    let curve = CurveId::ControlSpline(sid(5));
    let construction = source
        .apply(
            &[Edit::SetCurveRole {
                curve,
                role: CurveRole::Construction,
            }],
            || false,
        )
        .unwrap();
    let old = source.control_spline(sid(5)).unwrap();
    let unchanged = construction.control_spline(sid(5)).unwrap();
    assert_eq!(source.curve_role(curve), Some(CurveRole::Model));
    assert_eq!(
        construction.curve_role(curve),
        Some(CurveRole::Construction)
    );
    assert!(ptr::eq(old.regularity(), unchanged.regularity()));
    let moved = construction
        .apply(
            &[Edit::MovePoint {
                point: pid(1),
                position: point([-0., 0., 0.]),
            }],
            || false,
        )
        .unwrap();
    let new = moved.control_spline(sid(5)).unwrap();
    assert_eq!(moved.curve_role(curve), Some(CurveRole::Construction));
    assert!(!ptr::eq(old.regularity(), new.regularity()));
    assert_eq!(old.value(0.).unwrap().x().to_bits(), 0_f64.to_bits());
    assert_eq!(new.value(0.).unwrap().x().to_bits(), (-0_f64).to_bits());
    assert_eq!(
        new.regularity().source().poles()[0][0].to_bits(),
        (-0_f64).to_bits()
    );
}

#[test]
fn removing_a_pole_cascades_once_and_retains_the_shared_endpoint() {
    let repeated = SketchControlSpline {
        poles: [1, 2, 2, 4].map(pid),
        ..recipe()
    };
    let source = empty(0.001)
        .apply(&batch(NONPLANAR, repeated), || false)
        .unwrap();
    let curve = CurveId::ControlSpline(sid(5));
    let connected = source
        .apply(
            &[
                seed(6, [-1., 0., 0.], PointRole::SketchMarker),
                line(7, 6, 1),
                Edit::SetCurveRole {
                    curve,
                    role: CurveRole::Construction,
                },
            ],
            || false,
        )
        .unwrap();
    let again = connected.apply(
        &[Edit::RemovePoint(pid(2)), Edit::RemoveControlSpline(sid(5))],
        || false,
    );
    assert_eq!(
        again,
        Err(EditFailure::MissingEditTarget(EntityId::ControlSpline(
            sid(5)
        )))
    );
    let duplicate = connected.apply(
        &[
            Edit::RemovePoint(pid(2)),
            seed(5, [4., 5., 6.], PointRole::SketchMarker),
        ],
        || false,
    );
    assert_eq!(
        duplicate,
        Err(EditFailure::DuplicateIdentity {
            existing: EntityId::ControlSpline(sid(5)),
            attempted: EntityId::Point(pid(5)),
        })
    );
    let pruned = connected
        .apply(&[Edit::RemovePoint(pid(2))], || false)
        .unwrap();
    assert_eq!(
        pruned.owned_ids().collect::<Vec<_>>(),
        [
            EntityId::Point(pid(1)),
            EntityId::Point(pid(6)),
            EntityId::Line(lid(7))
        ]
    );
    assert_eq!(pruned.curve_role(curve), None);
    assert!(connected.control_spline(sid(5)).is_some());
    let explicit = connected
        .apply(&[Edit::RemoveControlSpline(sid(5))], || false)
        .unwrap();
    assert_eq!(pruned, explicit);
}

#[test]
fn retaining_a_spline_keeps_its_defining_poles_during_unrelated_deletion() {
    let source = snapshot(NONPLANAR);
    let seeded = source
        .apply(
            &[seed(6, [8., 1., 0.], PointRole::Defining), line(7, 4, 6)],
            || false,
        )
        .unwrap();
    let pruned = seeded.apply(&[Edit::RemoveLine(lid(7))], || false).unwrap();
    assert_eq!(pruned, source);
    assert_eq!(
        pruned.visible_ids().collect::<Vec<_>>(),
        [EntityId::ControlSpline(sid(5))]
    );
    assert_eq!(pruned.owned_ids().count(), 5);
}

#[test]
fn point_line_constraint_compilation_reports_the_unimplemented_curve_kind() {
    let shape = snapshot(NONPLANAR);
    assert!(matches!(
        ConstrainedSketch::new(shape, crate::constraints::policy()),
        Err(ConstraintFailure::UnsupportedEntity(EntityId::ControlSpline(id))) if id == sid(5)
    ));
}

#[test]
fn owned_spline_readers_share_the_same_coefficients_concurrently() {
    fn require_copy<T: Copy>() {}
    fn require_send_sync<T: Send + Sync>() {}
    require_copy::<Edit>();
    require_copy::<EditFailure>();
    require_copy::<SketchControlSpline>();
    require_send_sync::<ControlSplineGeometry>();
    require_send_sync::<Snapshot>();
    let source = snapshot(NONPLANAR);
    let edit = Edit::MovePoint {
        point: pid(3),
        position: point([2., 1., 2.5]),
    };
    let expected = source.apply(&[edit], || false).unwrap();
    let expected_jet = expected.control_spline(sid(5)).unwrap().jet(0.5).unwrap();
    let start = Barrier::new(4);
    thread::scope(|scope| {
        let workers = (0..4)
            .map(|_| {
                scope.spawn(|| {
                    start.wait();
                    for _ in 0..32 {
                        assert_eq!(source, source.apply(&[], || false).unwrap());
                        let rebuilt = source.apply(&[edit], || false).unwrap();
                        assert_eq!(expected, rebuilt);
                        assert_eq!(
                            rebuilt.control_spline(sid(5)).unwrap().jet(0.5).unwrap(),
                            expected_jet
                        );
                    }
                })
            })
            .collect::<Vec<_>>();
        for worker in workers {
            worker.join().unwrap();
        }
    });
}
