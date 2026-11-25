// SPDX-License-Identifier: GPL-2.0-only
mod replay;

use crate::fixture::{lid, line, pid, point, seed};
use martensite_tksketch3d::{
    BatchPhase, ControlSplineFailure, ControlSplineProofPolicy, CurveId, CurveRole, Edit,
    EditFailure, EntityId, FitSplineFailure, FitSplineId, FitSplineParameters, FitSplinePolicy,
    FitSplineQuantity, MinimumLength, PointRole, SketchFitSpline, Snapshot,
    constraints::{ConstrainedSketch, ConstraintFailure},
};
use std::{num::NonZeroU64, ptr};

const PARAMETERS: [f64; 4] = [0., 0.25, 0.75, 1.];

fn fit_id() -> FitSplineId {
    FitSplineId::new(NonZeroU64::new(5).unwrap())
}

fn recipe() -> SketchFitSpline {
    SketchFitSpline {
        id: fit_id(),
        stations: [1, 2, 3, 4].map(pid),
        parameters: FitSplineParameters::new(PARAMETERS).unwrap(),
        policy: FitSplinePolicy {
            max_distortion: 8.,
            max_inverse_gain: 32.,
            max_station_identity_error: 1e-12,
            max_station_miss: MinimumLength::new(1e-10).unwrap(),
            regularity: ControlSplineProofPolicy::new(1e-8, 128).unwrap(),
        },
    }
}

fn empty() -> Snapshot {
    Snapshot::empty(MinimumLength::new(0.001).unwrap())
}

fn stations(t: f64) -> [f64; 3] {
    [3. * t, 3. * t * t, t * t * t]
}

fn fit(values: [[f64; 3]; 4], recipe: SketchFitSpline) -> Result<Snapshot, EditFailure> {
    empty().apply(&batch(values, recipe), || false)
}

fn batch(values: [[f64; 3]; 4], recipe: SketchFitSpline) -> [Edit; 5] {
    let [a, b, c, d] = values;
    [
        Edit::InsertFitSpline(recipe),
        seed(1, a, PointRole::Defining),
        seed(2, b, PointRole::Defining),
        seed(3, c, PointRole::Defining),
        seed(4, d, PointRole::Defining),
    ]
}

fn assert_near(actual: f64, expected: f64, max_error: f64) {
    assert!(
        (actual - expected).abs() <= max_error,
        "{actual} vs {expected}"
    );
}

#[test]
fn known_spatial_cubic_recovers_value_and_both_analytic_derivatives() {
    let fitted = fit(PARAMETERS.map(stations), recipe()).unwrap();
    let spline = fitted.fit_spline(fit_id()).unwrap();
    assert_eq!(spline.recipe(), recipe());
    assert_eq!(spline.stations(), PARAMETERS.map(stations).map(point));
    assert_eq!(spline.recipe().endpoints(), [pid(1), pid(4)]);
    assert_eq!(spline.value(0.).unwrap(), point([0.; 3]));
    assert_eq!(spline.value(1.).unwrap(), point([3., 3., 1.]));
    assert_eq!(spline.regularity().source().curve_id(), 5);
    assert_eq!(spline.station_miss_upper()[0], 0.);
    assert_eq!(spline.station_miss_upper()[3], 0.);
    assert!(spline.pole_rounding_upper() < 1e-12);
    assert!(
        spline
            .station_miss_upper()
            .iter()
            .all(|&value| value <= 1e-10)
    );
    assert!(spline.operator_report().station_identity_upper <= 1e-12);
    let t = 0.5;
    let jet = spline.jet(t).unwrap();
    for [actual, expected] in [
        [*jet.position.coordinates(), stations(t)],
        [*jet.first.xyz().coordinates(), [3., 6. * t, 3. * t * t]],
        [*jet.second.xyz().coordinates(), [0., 6., 6. * t]],
    ] {
        for (actual, expected) in actual.into_iter().zip(expected) {
            assert_near(actual, expected, 2e-12);
        }
    }
    assert_eq!(spline.first_derivative(t).unwrap(), jet.first);
}

#[test]
fn station_weights_match_independent_lagrange_jets_and_an_actual_fixed_parameter_edit() {
    let source = fit(PARAMETERS.map(stations), recipe()).unwrap();
    let old = source.fit_spline(fit_id()).unwrap();
    let t = 0.375;
    let weights = old.station_weights(t).unwrap();
    for station in 0..4 {
        let analytic = lagrange(t, station);
        for order in 0..3 {
            assert_near(weights[order][station], analytic[order], 2e-12);
        }
        let epsilon = 1. / 16384.;
        let [x, y, z] = stations(PARAMETERS[station]);
        let edit = Edit::MovePoint {
            point: recipe().stations[station],
            position: point([x, y + epsilon, z]),
        };
        let varied = source.apply(&[edit], || false).unwrap();
        let after = varied.fit_spline(fit_id()).unwrap();
        assert_eq!(after.recipe().parameters.values(), PARAMETERS);
        assert!(ptr::eq(old.operator_report(), after.operator_report()));
        let a = old.jet(t).unwrap();
        let b = after.jet(t).unwrap();
        for (order, delta) in [
            b.position.y() - a.position.y(),
            b.first.y() - a.first.y(),
            b.second.y() - a.second.y(),
        ]
        .into_iter()
        .enumerate()
        {
            assert_near(delta / epsilon, weights[order][station], 5e-10);
        }
    }
    assert_eq!(
        source.fit_spline(fit_id()).unwrap().stations(),
        PARAMETERS.map(stations).map(point)
    );
}

fn lagrange(t: f64, station: usize) -> [f64; 3] {
    let point = PARAMETERS[station];
    let others: Vec<_> = PARAMETERS.into_iter().filter(|&x| x != point).collect();
    let a = t - others[0];
    let b = t - others[1];
    let c = t - others[2];
    let scale = 1.
        / others
            .into_iter()
            .map(|other| point - other)
            .product::<f64>();
    [
        a * b * c * scale,
        (a * b + b * c + a * c) * scale,
        2. * (a + b + c) * scale,
    ]
}

#[test]
fn distortion_guard_is_inclusive_on_insert_and_keeps_parameters_fixed_on_edit() {
    let mut authored = recipe();
    authored.policy.max_distortion = 2.;
    let stations = [0., 1., 3., 5.].map(|x| [x, 0., 0.]);
    let source = fit(stations, authored).unwrap();
    let current = source.fit_spline(fit_id()).unwrap();
    assert_eq!(current.distortion().ratio_upper, 2.);
    assert_eq!(current.distortion().fastest, 2);
    let mut stricter = authored;
    stricter.policy.max_distortion = 2_f64.next_down();
    assert!(matches!(
        fit(stations, stricter),
        Err(EditFailure::FitSpline {
            source: FitSplineFailure::Limit {
                quantity: FitSplineQuantity::Distortion,
                upper: 2.,
                ..
            },
            ..
        })
    ));
    let accepted = source
        .apply(
            &[Edit::MovePoint {
                point: pid(4),
                position: point([4.5, 0., 0.]),
            }],
            || false,
        )
        .unwrap();
    let accepted = accepted.fit_spline(fit_id()).unwrap();
    assert_eq!(accepted.distortion().ratio_upper, 1.5);
    assert_eq!(accepted.recipe().parameters.values(), PARAMETERS);
    assert!(ptr::eq(
        current.operator_report(),
        accepted.operator_report()
    ));
    let refused = source.apply(
        &[Edit::MovePoint {
            point: pid(4),
            position: point([5.25, 0., 0.]),
        }],
        || false,
    );
    assert!(matches!(
        refused,
        Err(EditFailure::FitSpline {
            source: FitSplineFailure::Limit {
                quantity: FitSplineQuantity::Distortion,
                upper: 2.25,
                ..
            },
            ..
        })
    ));
    assert_eq!(source.point(pid(4)).unwrap().position.x(), 5.);
}

#[test]
fn huge_translation_preserves_exact_station_and_endpoint_bits_without_refitting_parameters() {
    let base = f64::from_bits((1023_u64 + 60) << 52);
    let stations = [0., 768., 2304., 3072.].map(|dx| [base + dx, -base, -0.]);
    let source = fit(stations, recipe()).unwrap();
    let spline = source.fit_spline(fit_id()).unwrap();
    assert_eq!(
        spline.poles().map(|point| point.x()),
        [base, base + 1024., base + 2048., base + 3072.]
    );
    assert_eq!(spline.poles().map(|point| point.y()), [-base; 4]);
    assert_eq!(spline.station_miss_upper(), [0.; 4]);
    assert_eq!(spline.distortion().ratio_upper, 1.);
    assert_eq!(spline.value(0.).unwrap().z().to_bits(), (-0_f64).to_bits());
    assert_eq!(spline.value(1.).unwrap().z().to_bits(), (-0_f64).to_bits());
    assert_eq!(spline.jet(0.5).unwrap().first.x(), 3072.);
}

#[test]
fn fit_checks_exact_stored_station_miss_even_when_rounded_value_matches() {
    let base = f64::from_bits((1023_u64 + 60) << 52);
    let stations = [0., 1024., 2304., 3072.].map(|dx| [base + dx, 0., 0.]);
    let mut authored = recipe();
    authored.policy.max_station_miss = MinimumLength::new(32.).unwrap();
    let source = fit(stations, authored).unwrap();
    let curve = source.fit_spline(fit_id()).unwrap();
    assert_eq!(curve.value(0.25).unwrap(), point(stations[1]));
    assert_eq!(curve.station_miss_upper(), [0., 32., 0., 0.]);
    authored.policy.max_station_miss = MinimumLength::new(32_f64.next_down()).unwrap();
    assert!(matches!(
        fit(stations, authored),
        Err(EditFailure::FitSpline {
            source: FitSplineFailure::Limit {
                quantity: FitSplineQuantity::StationMiss(1),
                upper: 32.,
                ..
            },
            ..
        })
    ));
}

#[test]
fn conditioning_and_station_failure_are_separate_from_distortion() {
    let mut conditioned = recipe();
    conditioned.policy.max_inverse_gain = 1.;
    assert!(matches!(
        fit(PARAMETERS.map(stations), conditioned),
        Err(EditFailure::FitSpline {
            source: FitSplineFailure::Limit {
                quantity: FitSplineQuantity::InverseGain,
                ..
            },
            ..
        })
    ));
    let mut strict = recipe();
    strict.policy.max_station_miss = MinimumLength::new(1e-20).unwrap();
    assert!(matches!(
        fit(PARAMETERS.map(|t| [t, t * t, t * t * t]), strict),
        Err(EditFailure::FitSpline {
            source: FitSplineFailure::Limit {
                quantity: FitSplineQuantity::StationMiss(1 | 2),
                ..
            },
            ..
        })
    ));
}

#[test]
fn stationary_curve_and_cancellation_have_distinct_atomic_outcomes() {
    let folded = [0., 3., 1., 4.].map(|x| [x, 0., 0.]);
    assert!(matches!(
        fit(folded, recipe()),
        Err(EditFailure::FitSpline {
            source: FitSplineFailure::Regularity(ControlSplineFailure::Stationary { .. }),
            ..
        })
    ));
    let mut bounded = recipe();
    bounded.policy.regularity = ControlSplineProofPolicy::new(1e-8, 0).unwrap();
    assert!(matches!(
        fit(folded, bounded),
        Err(EditFailure::FitSpline {
            source: FitSplineFailure::Regularity(ControlSplineFailure::WorkLimit {
                limit: 0,
                bisections: 0
            }),
            ..
        })
    ));
    let input = empty();
    let mut polls = 0;
    let cancelled = input.apply(&batch(PARAMETERS.map(stations), recipe()), || {
        polls += 1;
        polls == 10
    });
    assert_eq!(
        cancelled,
        Err(EditFailure::Cancelled(BatchPhase::Validation))
    );
    assert_eq!(input.owned_ids().count(), 0);
}

#[test]
fn stations_cascade_once_and_shared_endpoint_survives_a_fit_deletion() {
    let source = fit(PARAMETERS.map(stations), recipe()).unwrap();
    let curve = CurveId::FitSpline(fit_id());
    let wired = source
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
    assert_eq!(wired.curve_role(curve), Some(CurveRole::Construction));
    assert_eq!(
        wired.line(lid(7)).unwrap().line.endpoints[1],
        wired.fit_spline(fit_id()).unwrap().recipe().endpoints()[0]
    );
    let duplicate = wired.apply(
        &[Edit::RemovePoint(pid(2)), Edit::RemoveFitSpline(fit_id())],
        || false,
    );
    assert_eq!(
        duplicate,
        Err(EditFailure::MissingEditTarget(
            EntityId::FitSpline(fit_id())
        ))
    );
    let removed = wired.apply(&[Edit::RemovePoint(pid(2))], || false).unwrap();
    assert_eq!(removed.curve_role(curve), None);
    assert_eq!(
        removed.owned_ids().collect::<Vec<_>>(),
        [
            EntityId::Point(pid(1)),
            EntityId::Point(pid(6)),
            EntityId::Line(lid(7))
        ]
    );
    assert!(source.fit_spline(fit_id()).is_some());
    assert!(matches!(
        ConstrainedSketch::new(source, crate::constraints::policy()),
        Err(ConstraintFailure::UnsupportedEntity(EntityId::FitSpline(id))) if id == fit_id()
    ));
}

#[test]
fn invalid_station_recipe_and_ill_conditioning_refuse_with_typed_context() {
    assert_eq!(
        FitSplineParameters::new([0., 0.5, 0.5, 1.]),
        Err(FitSplineFailure::Parameters([0., 0.5, 0.5, 1.]))
    );
    let mut request = recipe();
    request.stations[2] = pid(2);
    assert!(matches!(
        fit(PARAMETERS.map(stations), request),
        Err(EditFailure::FitSpline { source: FitSplineFailure::RepeatedStation(id), .. }) if id == pid(2)
    ));
    request.stations[2] = pid(94);
    assert!(matches!(
        fit(PARAMETERS.map(stations), request),
        Err(EditFailure::FitSpline { source: FitSplineFailure::MissingStation(id), .. }) if id == pid(94)
    ));
    let mut points = PARAMETERS.map(stations);
    points[2] = points[0];
    assert!(matches!(
        fit(points, recipe()),
        Err(EditFailure::FitSpline { source: FitSplineFailure::CoincidentStations(ids), .. }) if ids == [pid(1), pid(3)]
    ));
    request = recipe();
    request.parameters = FitSplineParameters::new([0., 1e-18, 2e-18, 1.]).unwrap();
    request.policy.max_distortion = 1e20;
    assert!(matches!(
        fit(PARAMETERS.map(stations), request),
        Err(EditFailure::FitSpline {
            source: FitSplineFailure::Banded { .. },
            ..
        }) | Err(EditFailure::FitSpline {
            source: FitSplineFailure::Limit {
                quantity: FitSplineQuantity::InverseGain,
                ..
            },
            ..
        })
    ));
}
