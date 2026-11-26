// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::constraints::{length, policy, rid};
use martensite_tkmath::gp::{Axis1, Direction3};
use martensite_tksketch3d::constraints::{
    ConstraintFailure, ConstraintPolicy, ContinuityFailure, CurvatureTolerance, LineFitCubicJoin,
    Relation, RelationEdit, RelationSpec, RigidMotion,
};
use martensite_tksolve::preview::{Incomplete, Limit, PreviewError};

mod latency;

const OTHER: u64 = 6;
const TANGENT_ID: u64 = 11;

fn joined_geometry(
    positions: [[f64; 3]; 4],
    fit_recipe: SketchFitSpline,
    endpoint: u64,
    other: [f64; 3],
) -> (Snapshot, LineFitCubicJoin) {
    let geometry = fit(positions, fit_recipe)
        .unwrap()
        .apply(
            &[
                seed(OTHER, other, PointRole::Defining),
                line(7, endpoint, OTHER),
            ],
            || false,
        )
        .unwrap();
    let join = LineFitCubicJoin {
        line: lid(7),
        fit: fit_id(),
        endpoint: pid(endpoint),
    };
    (geometry, join)
}

fn tangent_policy() -> ConstraintPolicy {
    let mut options = policy();
    options.preview.block_inputs = 15;
    options
}

fn tangent_edit(join: LineFitCubicJoin) -> RelationEdit {
    RelationEdit::Insert(rid(TANGENT_ID), RelationSpec::Tangent(join.into()))
}

fn fitted(sketch: &ConstrainedSketch, join: LineFitCubicJoin) -> [f64; 3] {
    let shape = sketch.geometry();
    let curve = shape.fit_spline(join.fit).unwrap();
    let forward = join.endpoint == curve.recipe().stations[0];
    let t = if forward { 0. } else { 1. };
    let shared = shape.point(join.endpoint).unwrap().position;
    let native = curve.jet(t).unwrap();
    assert_eq!(
        native.position.coordinates().map(f64::to_bits),
        shared.coordinates().map(f64::to_bits)
    );
    let oriented_poles = if forward {
        [curve.poles()[0], curve.poles()[1]]
    } else {
        [curve.poles()[3], curve.poles()[2]]
    };
    let incoming = shape.point(pid(OTHER)).unwrap().position;
    let direction = difference(*shared.coordinates(), *incoming.coordinates());
    let outgoing = difference(
        *oriented_poles[1].coordinates(),
        *oriented_poles[0].coordinates(),
    );
    let jet = native
        .first
        .xyz()
        .coordinates()
        .map(|value| if forward { value } else { -value });
    for (actual, delta) in jet.into_iter().zip(outgoing) {
        assert_near(actual, 3. * delta, 3e-14);
    }
    let (sign, sine) = tangent_measure(direction, jet);
    assert!(sign > 0.);
    assert!(sine <= tangent_policy().parallel_sine_tolerance, "{sine}");
    jet
}

fn difference(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|axis| a[axis] - b[axis])
}

fn tangent_measure(a: [f64; 3], b: [f64; 3]) -> (f64, f64) {
    let dot = |a: [f64; 3], b: [f64; 3]| (0..3).map(|i| a[i] * b[i]).sum::<f64>();
    let cross = std::array::from_fn(|axis| {
        let i = (axis + 1) % 3;
        let j = (axis + 2) % 3;
        a[i] * b[j] - a[j] * b[i]
    });
    (
        dot(a, b),
        (dot(cross, cross) / (dot(a, a) * dot(b, b))).sqrt(),
    )
}

#[test]
fn authored_directed_fit_tangent_solves_from_both_ends_and_holds_the_line_during_insertion() {
    for (end, other) in [(1, [-2., -0.4, 0.2]), (4, [4., 4.9, 1.8])] {
        let (geometry, join) = joined_geometry(PARAMETERS.map(stations), recipe(), end, other);
        let sketch = ConstrainedSketch::new(geometry.clone(), tangent_policy()).unwrap();
        let accepted = sketch.apply(&[tangent_edit(join)], || false).unwrap();
        assert_eq!(
            accepted.geometry().line(join.line),
            geometry.line(join.line)
        );
        let before = geometry.fit_spline(join.fit).unwrap();
        let after = accepted.geometry().fit_spline(join.fit).unwrap();
        assert_eq!(after.recipe(), before.recipe());
        assert!(ptr::eq(after.operator_report(), before.operator_report()));
        assert_ne!(after.stations(), before.stations());
        fitted(&accepted, join);
        let rank = accepted.inspect_rank().unwrap();
        assert_eq!(
            (rank.rank, rank.equations, rank.degrees_of_freedom),
            (2, 2, 13)
        );
        assert_eq!(sketch.geometry(), &geometry);
        let archived = [(rid(TANGENT_ID), Relation::Tangent(join.into()))];
        let restored = ConstrainedSketch::restore(
            accepted.geometry().clone(),
            &archived,
            tangent_policy(),
            || false,
        )
        .unwrap();
        assert_eq!(restored.geometry(), accepted.geometry());
        assert_eq!(restored.relations().collect::<Vec<_>>(), archived);
    }
}

#[test]
fn rotated_fit_tangent_keeps_the_branch_during_a_later_preferred_drag() {
    let (geometry, join) = joined_geometry(PARAMETERS.map(stations), recipe(), 1, [-2., 0., 0.]);
    let sketch = ConstrainedSketch::new(geometry, tangent_policy())
        .unwrap()
        .apply(&[tangent_edit(join)], || false)
        .unwrap();
    let rotated = rotate(&sketch);
    let direction = fitted(&rotated, join);
    assert!(direction[1] > 0. && direction[0].abs() < 1e-10);
    let mut target = *rotated
        .geometry()
        .point(pid(2))
        .unwrap()
        .position
        .coordinates();
    target[0] -= 0.02;
    target[2] += 0.01;
    let preview = rotated
        .drag_preferred(pid(2), point(target), 1e-8, || false)
        .unwrap();
    fitted(&preview.update.sketch, join);
    assert_eq!(preview.update.rank.degrees_of_freedom, 13);
    assert!(preview.preference.scaled_cursor_error < 1e-7);
    assert!(preview.update.residual_work_bound <= tangent_policy().residual_work);
}

fn rotate(sketch: &ConstrainedSketch) -> ConstrainedSketch {
    let axis = Axis1::new(point([0.; 3]), Direction3::new(0., 0., 1.).unwrap());
    sketch
        .transform_points(
            &[pid(1), pid(2), pid(3), pid(4), pid(OTHER)],
            RigidMotion::Rotate {
                axis,
                radians: std::f64::consts::FRAC_PI_2,
            },
            || false,
        )
        .unwrap()
}

#[test]
fn fit_relation_refuses_curvature_backward_unshared_or_aliased_controls_and_small_width() {
    let (geometry, join) = joined_geometry(PARAMETERS.map(stations), recipe(), 1, [-2., 0., 0.]);
    let geometry = geometry.apply(&[line(8, 2, 1)], || false).unwrap();
    let sketch = ConstrainedSketch::new(geometry.clone(), tangent_policy()).unwrap();
    let g2 = RelationSpec::G2(join.into(), CurvatureTolerance::new(1e-8).unwrap());
    assert!(
        matches!(sketch.apply(&[RelationEdit::Insert(rid(1), g2)], || false),
        Err(ConstraintFailure::Continuity(_, ContinuityFailure::FitCurvatureUnsupported(id))) if id == fit_id())
    );
    let absent = LineFitCubicJoin {
        endpoint: pid(2),
        ..join
    };
    assert!(matches!(sketch.apply(&[tangent_edit(absent)], || false),
        Err(ConstraintFailure::Continuity(_, ContinuityFailure::UnsharedEndpoint(value))) if value == absent.into()));
    let alias = LineFitCubicJoin {
        line: lid(8),
        ..join
    };
    assert!(matches!(sketch.apply(&[tangent_edit(alias)], || false),
        Err(ConstraintFailure::Continuity(_, ContinuityFailure::RepeatedControl(point))) if point == pid(2)));
    let mut narrow = tangent_policy();
    narrow.preview.block_inputs = 14;
    let bounded = ConstrainedSketch::new(geometry, narrow).unwrap();
    assert!(matches!(
        bounded.apply(&[tangent_edit(join)], || false),
        Err(ConstraintFailure::Preview(
            _,
            PreviewError::Incomplete(Incomplete::Limit {
                quantity: Limit::BlockInputs,
                ..
            })
        ))
    ));
    let (backwards, join) = joined_geometry(PARAMETERS.map(stations), recipe(), 1, [2., 0., 0.]);
    let backwards = ConstrainedSketch::new(backwards, tangent_policy()).unwrap();
    assert!(matches!(
        backwards.apply(&[tangent_edit(join)], || false),
        Err(ConstraintFailure::Continuity(
            _,
            ContinuityFailure::BackwardTangent
        ))
    ));
}

#[test]
fn adopted_fit_drag_preserves_geometry_admission_and_is_atomic_on_distortion_or_cancel() {
    let mut authored = recipe();
    authored.policy.max_distortion = 2.;
    let positions = [0., 1., 3., 5.].map(|x| [x, 0., 0.]);
    let (geometry, join) = joined_geometry(positions, authored, 1, [-1., 0., 0.]);
    let sketch = fixed_stations(geometry, join);
    let preview = sketch
        .drag(pid(4), point([4.75, 0., 0.]), || false)
        .unwrap();
    let accepted = &preview.sketch;
    let after = accepted.geometry().fit_spline(join.fit).unwrap();
    assert!(after.distortion().ratio_upper < 2.);
    assert!(ptr::eq(
        after.operator_report(),
        sketch
            .geometry()
            .fit_spline(join.fit)
            .unwrap()
            .operator_report()
    ));
    fitted(accepted, join);
    let refusal = accepted.drag(pid(4), point([5.25, 0., 0.]), || false);
    assert!(matches!(
        refusal,
        Err(ConstraintFailure::Geometry(EditFailure::FitSpline {
            source: FitSplineFailure::Limit {
                quantity: FitSplineQuantity::Distortion,
                ..
            },
            ..
        }))
    ));
    let mut polls = 0;
    assert!(matches!(
        accepted.drag(pid(4), point([4.5, 0., 0.]), || {
            polls += 1;
            polls == 9
        }),
        Err(ConstraintFailure::Cancelled)
    ));
    assert_eq!(
        accepted.geometry().point(pid(4)).unwrap().position,
        point([4.75, 0., 0.])
    );
    assert_eq!(
        sketch.geometry().point(pid(4)).unwrap().position,
        point([5., 0., 0.])
    );
}

fn fixed_stations(geometry: Snapshot, join: LineFitCubicJoin) -> ConstrainedSketch {
    let batch = [
        RelationEdit::Insert(rid(1), RelationSpec::FixLine(join.line)),
        RelationEdit::Insert(rid(2), RelationSpec::FixPoint(pid(2))),
        RelationEdit::Insert(rid(3), RelationSpec::FixPoint(pid(3))),
        tangent_edit(join),
    ];
    ConstrainedSketch::new(geometry, tangent_policy())
        .unwrap()
        .apply(&batch, || false)
        .unwrap()
}

#[test]
fn a_drag_to_zero_fit_endpoint_velocity_is_declined() {
    let base = (1_u64 << 60) as f64;
    let positions = [0., 256., 2304., 4608.].map(|x| [base + x, 0., 0.]);
    let mut authored = recipe();
    authored.policy.max_distortion = 16.;
    authored.policy.max_station_miss = length(128.);
    let (geometry, join) = joined_geometry(positions, authored, 1, [base - 1024., 0., 0.]);
    let stationary = point([base + 4096., 0., 0.]);
    assert!(matches!(
        geometry.apply(
            &[Edit::MovePoint {
                point: pid(4),
                position: stationary
            }],
            || false
        ),
        Err(EditFailure::FitSpline {
            source: FitSplineFailure::Regularity(ControlSplineFailure::Stationary { .. }),
            ..
        })
    ));
    let sketch = fixed_stations(geometry.clone(), join);
    let refusal = sketch.drag(pid(4), stationary, || false);
    assert!(matches!(
        refusal,
        Err(ConstraintFailure::Preview(_, _))
            | Err(ConstraintFailure::Geometry(EditFailure::FitSpline {
                source: FitSplineFailure::Regularity(_),
                ..
            }))
    ));
    assert_eq!(sketch.geometry(), &geometry);
}

#[test]
fn affine_fit_tangency_does_not_certify_rounded_stored_poles_at_large_translation() {
    let base = (1_u64 << 60) as f64;
    let positions = [
        [base, 0., -0.],
        [base + 1024., 0.25, 0.],
        [base + 2304., 0.75, 0.],
        [base + 3072., 1., -0.],
    ];
    let mut authored = recipe();
    authored.policy.max_station_miss = length(64.);
    let (geometry, join) = joined_geometry(positions, authored, 1, [base - 1024., -0.2, 0.]);
    let curve = geometry.fit_spline(join.fit).unwrap();
    let actual = *curve.first_derivative(0.).unwrap().xyz().coordinates();
    let (_, native_sine) = tangent_measure([1024., 0.2, 0.], actual);
    // The exact interpolation derivative in x is 8*1024 - 8/3*2304 + 3072.
    let (_, affine_sine) = tangent_measure([1024., 0.2, 0.], [5120., 1., 0.]);
    assert_eq!(actual[0], 5376.);
    assert!(native_sine > 1e-6);
    assert!(affine_sine < 1e-14);
    let relation = Relation::Tangent(join.into());
    let sketch = ConstrainedSketch::new(geometry.clone(), tangent_policy()).unwrap();
    assert!(matches!(
        sketch.apply(&[tangent_edit(join)], || false),
        Err(ConstraintFailure::Infeasible(id, failure)) if id == rid(TANGENT_ID) && failure == relation
    ));
    assert!(matches!(
        ConstrainedSketch::restore(geometry.clone(), &[(rid(TANGENT_ID), relation)], tangent_policy(), || false),
        Err(ConstraintFailure::Infeasible(id, failure)) if id == rid(TANGENT_ID) && failure == relation
    ));
    assert_eq!(sketch.geometry(), &geometry);
}

#[test]
fn replay_checks_the_warm_fit_rows_even_when_the_stored_curve_is_tangent() {
    let base = (1_u64 << 60) as f64;
    let positions = [
        [base, 0., 0.],
        [base + 1024., 0.25, 0.],
        [base + 2304., 0.75, 0.],
        [base + 3072., 1., 0.],
    ];
    let mut authored = recipe();
    authored.policy.max_station_miss = length(64.);
    let (geometry, join) = joined_geometry(positions, authored, 1, [base - 5376., -1., 0.]);
    let curve = geometry.fit_spline(join.fit).unwrap();
    let native = *curve.first_derivative(0.).unwrap().xyz().coordinates();
    let (_, stored_sine) = tangent_measure([5376., 1., 0.], native);
    let (_, affine_sine) = tangent_measure([5376., 1., 0.], [5120., 1., 0.]);
    assert!(stored_sine < tangent_policy().solve.relative_tolerance);
    assert!(affine_sine > 1e-6);
    let saved = Relation::Tangent(join.into());
    assert!(matches!(
        ConstrainedSketch::restore(geometry, &[(rid(TANGENT_ID), saved)], tangent_policy(), || false),
        Err(ConstraintFailure::Infeasible(id, relation)) if id == rid(TANGENT_ID) && relation == saved
    ));
}

#[test]
fn prepared_fit_tangents_rebuild_independent_curves_from_shared_input() {
    let (geometry, join) = joined_geometry(PARAMETERS.map(stations), recipe(), 1, [-2., 0., 0.]);
    let accepted = ConstrainedSketch::new(geometry, tangent_policy())
        .unwrap()
        .apply(&[tangent_edit(join)], || false)
        .unwrap();
    let target = point([0.76, 0.21, 0.02]);
    let tick = || {
        accepted
            .drag_preferred(pid(2), target, 1e-8, || false)
            .unwrap()
            .update
    };
    let serial = tick();
    std::thread::scope(|scope| {
        let first = scope.spawn(tick);
        let second = scope.spawn(tick);
        for result in [first.join().unwrap(), second.join().unwrap()] {
            assert_eq!(result.sketch.geometry(), serial.sketch.geometry());
            assert_eq!(result.rank, serial.rank);
            assert_eq!(result.residual_work_bound, serial.residual_work_bound);
        }
    });
    fitted(&serial.sketch, join);
    assert_ne!(serial.sketch.geometry(), accepted.geometry());
}
