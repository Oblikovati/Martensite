// SPDX-License-Identifier: GPL-2.0-only
//! Owned finite-span controls use direct points and derivatives independently of fitter internals.
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topexp,
};
use martensite_tkg3d::geom::GeomBSplineCurve;
use martensite_tkhelix::authored::*;
use martensite_tkmath::{
    gp::{Axis2, Point3, StandardDirection3 as Direction},
    predicates::roots::metric_bounds::HelixTraceFailure,
};
use std::f64::consts::{FRAC_1_PI, FRAC_PI_2, PI, TAU};

const CYLINDER: HelixRadius = HelixRadius::Cylinder { radius: 4. };
const ONE_TURN: HelixLead = HelixLead::PitchAndTurns {
    pitch: 2.,
    turns: 1.,
};
const POLICY: HelixFitOptions = HelixFitOptions {
    max_turns: 8.,
    approximation_tolerance: 1e-8,
    max_segments: 128,
    max_evaluations: 32_768,
    max_topology_tolerance: 1e-6,
    trace: HelixTraceLimits {
        max_position_error: 1e-6,
        max_native_spans: 96,
        max_cells: 4096,
        max_operations: 5_000_000,
    },
};

fn request(radius: HelixRadius, rotation: HelixRotation) -> ConstantHelixRequest {
    ConstantHelixRequest {
        placement: Axis2::default(),
        radius,
        lead: ONE_TURN,
        rotation,
    }
}

fn near(actual: &[f64; 3], expected: [f64; 3], tolerance: f64) {
    let error = actual
        .iter()
        .zip(expected)
        .map(|(a, e)| (a - e).abs())
        .sum::<f64>();
    assert!(
        error <= tolerance,
        "{actual:?} vs {expected:?}: {error:e} > {tolerance:e}"
    );
}

#[test]
fn cylinder_cardinal_points_and_authoring_leads_are_canonical() {
    for pair in [
        ONE_TURN,
        HelixLead::HeightAndTurns {
            height: 2.,
            turns: 1.,
        },
        HelixLead::PitchAndHeight {
            pitch: 2.,
            height: 2.,
        },
    ] {
        for (rotation, sign) in [
            (HelixRotation::Positive, 1.),
            (HelixRotation::Negative, -1.),
        ] {
            let law = ConstantHelix::new(ConstantHelixRequest {
                lead: pair,
                ..request(CYLINDER, rotation)
            })
            .unwrap();
            let resolved = law.resolved();
            assert_eq!(
                [
                    resolved.pitch,
                    resolved.height,
                    resolved.turns,
                    resolved.parameter_end
                ],
                [2., 2., 1., TAU]
            );
            let controls = [
                [4., 0., 0.],
                [0., sign * 4., 0.5],
                [-4., 0., 1.],
                [0., -sign * 4., 1.5],
                [4., 0., 2.],
            ];
            for (n, expected) in controls.into_iter().enumerate() {
                near(
                    law.evaluate(n as f64 * FRAC_PI_2)
                        .unwrap()
                        .position
                        .coordinates(),
                    expected,
                    2e-15,
                );
            }
            let initial = law.evaluate(0.).unwrap();
            near(
                initial.first.xyz().coordinates(),
                [0., sign * 4., FRAC_1_PI],
                1e-15,
            );
            near(initial.second.xyz().coordinates(), [-4., 0., 0.], 1e-15);
        }
    }
}

#[test]
fn taper_sign_retains_shallow_end_and_corrects_authored_second_derivative() {
    for closing in [3., 5., 4.00001] {
        for (rotation, sign) in [
            (HelixRotation::Positive, 1.),
            (HelixRotation::Negative, -1.),
        ] {
            let law = ConstantHelix::new(request(
                HelixRadius::Cone {
                    opening: 4.,
                    closing,
                },
                rotation,
            ))
            .unwrap();
            let initial = law.evaluate(0.).unwrap();
            let radial = (closing - 4.) / TAU;
            near(
                initial.first.xyz().coordinates(),
                [radial, sign * 4., FRAC_1_PI],
                1e-15,
            );
            near(
                initial.second.xyz().coordinates(),
                [-4., sign * 2. * radial, 0.],
                1e-15,
            );
            near(
                law.evaluate(PI).unwrap().position.coordinates(),
                [-(4. + closing) / 2., 0., 1.],
                2e-15,
            );
            near(
                law.evaluate(TAU).unwrap().position.coordinates(),
                [closing, 0., 2.],
                3e-15,
            );
        }
    }
}

fn fitted(request: ConstantHelixRequest) -> FittedHelix {
    ConstantHelix::new(request)
        .unwrap()
        .fit(POLICY, || false)
        .unwrap()
}

fn actual_check(fit: &FittedHelix, parameter: f64, expected: [f64; 3]) {
    let native = fit.export_brep(|| false).unwrap();
    assert_eq!(native.history, fit.history());
    let curve = BRepTool::curve_with_location(&native.edge).unwrap();
    assert!(curve.curve.downcast_ref::<GeomBSplineCurve>().is_some());
    assert_eq!([curve.first, curve.last], fit.history().native_parameter);
    near(
        curve
            .curve
            .get()
            .unwrap()
            .d0(parameter)
            .unwrap()
            .coordinates(),
        expected,
        fit.receipt().trace.position_error_upper + 2e-14,
    );
}

#[test]
fn actual_detached_native_traces_cylinder_inward_outward_shallow_and_short_tail() {
    let cylinder = fitted(request(CYLINDER, HelixRotation::Positive));
    actual_check(&cylinder, FRAC_PI_2, [0., 4., 0.5]);
    for (closing, rotation) in [
        (3., HelixRotation::Positive),
        (5., HelixRotation::Negative),
        (4.00001, HelixRotation::Positive),
    ] {
        let fit = fitted(request(
            HelixRadius::Cone {
                opening: 4.,
                closing,
            },
            rotation,
        ));
        actual_check(&fit, PI, [-(4. + closing) / 2., 0., 1.]);
        actual_check(&fit, TAU, [closing, 0., 2.]);
        check_receipt(&fit);
    }
    let turns = 1. + 5e-8;
    let remainder = fitted(ConstantHelixRequest {
        lead: HelixLead::PitchAndTurns { pitch: 2., turns },
        ..request(CYLINDER, HelixRotation::Positive)
    });
    let end = turns * TAU;
    assert_eq!(remainder.history().native_parameter, [0., end]);
    let theta = (turns - 1.) * TAU;
    actual_check(
        &remainder,
        end,
        [
            4. * martensite_cmath::cos(theta),
            4. * martensite_cmath::sin(theta),
            2. * turns,
        ],
    );
    assert!(remainder.receipt().terminal_error_upper[1] < 4. * martensite_cmath::sin(theta));
    check_receipt(&remainder);
}

fn check_receipt(fit: &FittedHelix) {
    let receipt = fit.receipt();
    assert_eq!(
        receipt.effective_max_segments,
        POLICY.trace.max_native_spans
    );
    assert!(receipt.native_estimate <= POLICY.approximation_tolerance);
    assert!((0. ..=POLICY.trace.max_position_error).contains(&receipt.trace.position_error_upper));
    assert!(receipt.trace.axial_derivative_lower > 0.);
    assert!(receipt.stored_topology_tolerance <= POLICY.max_topology_tolerance);
    assert!(
        receipt
            .terminal_error_upper
            .into_iter()
            .all(|error| error <= POLICY.trace.max_position_error)
    );
    assert!(receipt.evaluations > 0 && receipt.evaluations <= POLICY.max_evaluations);
}

#[test]
fn checked_domain_work_tolerance_and_cancellation_are_typed() {
    let request = request(CYLINDER, HelixRotation::Negative);
    let law = ConstantHelix::new(request).unwrap();
    for parameter in [f64::NAN, f64::INFINITY, -0.01, TAU.next_up()] {
        assert!(matches!(
            law.evaluate(parameter),
            Err(HelixFailure::Parameter { .. })
        ));
    }
    for closing in [0., -1., f64::INFINITY] {
        let radius = HelixRadius::Cone {
            opening: 4.,
            closing,
        };
        assert!(matches!(
            ConstantHelix::new(ConstantHelixRequest { radius, ..request }),
            Err(HelixFailure::InvalidScalar {
                quantity: HelixQuantity::ClosingRadius,
                ..
            })
        ));
    }
    let lead = HelixLead::PitchAndTurns {
        pitch: f64::MAX,
        turns: 2.,
    };
    assert!(matches!(
        ConstantHelix::new(ConstantHelixRequest { lead, ..request }),
        Err(HelixFailure::DerivedScalar {
            quantity: HelixQuantity::Height,
            ..
        })
    ));
    assert!(matches!(
        law.fit(
            HelixFitOptions {
                max_turns: 0.5,
                ..POLICY
            },
            || false
        ),
        Err(HelixFailure::TurnLimit {
            requested: 1.,
            limit: 0.5
        })
    ));
    assert!(matches!(
        law.fit(
            HelixFitOptions {
                max_evaluations: 1,
                ..POLICY
            },
            || false
        ),
        Err(HelixFailure::EvaluationLimit {
            completed: 1,
            limit: 1
        })
    ));
    assert!(matches!(
        law.fit(
            HelixFitOptions {
                max_topology_tolerance: 1e-12,
                ..POLICY
            },
            || false
        ),
        Err(HelixFailure::TopologyTolerance { .. })
    ));
    let trace = HelixTraceLimits {
        max_cells: 1,
        max_position_error: 0.,
        ..POLICY.trace
    };
    assert!(matches!(
        law.fit(HelixFitOptions { trace, ..POLICY }, || false),
        Err(HelixFailure::Trace(cause)) if matches!(*cause, HelixTraceFailure::CellLimit { completed: 1, limit: 1 })
    ));
    assert!(matches!(
        law.fit(POLICY, || true),
        Err(HelixFailure::Cancelled(HelixStage::Admission))
    ));
    let mut calls = 0;
    assert!(matches!(
        law.fit(POLICY, || {
            calls += 1;
            calls == 5
        }),
        Err(HelixFailure::Cancelled(HelixStage::Fit))
    ));
}

#[test]
fn translated_rotated_frame_and_independent_native_exports() {
    let placement =
        Axis2::with_standard_frame(Point3::new(20., 30., 40.), Direction::X, Direction::Y);
    let fit = fitted(ConstantHelixRequest {
        placement,
        ..request(CYLINDER, HelixRotation::Negative)
    });
    let expected = [20.5, 30., 36.];
    near(
        fit.law()
            .evaluate(FRAC_PI_2)
            .unwrap()
            .position
            .coordinates(),
        expected,
        1e-14,
    );
    actual_check(&fit, FRAC_PI_2, expected);
    let one = fit.export_brep(|| false).unwrap();
    let other = fit.export_brep(|| false).unwrap();
    let (one_start, _) = topexp::vertices(&one.edge, true);
    let (other_start, _) = topexp::vertices(&other.edge, true);
    assert!(!one_start.is_same(&other_start));
    BRepBuilder::default()
        .set_vertex_point_and_tolerance(&one_start, Point3::default(), 0.01)
        .unwrap();
    assert_eq!(BRepTool::pnt(&one_start).unwrap(), Point3::default());
    let expected_start = Point3::new(20., 34., 40.);
    let current = BRepTool::pnt(&other_start).unwrap();
    near(
        current.coordinates(),
        *expected_start.coordinates(),
        fit.receipt().terminal_error_upper[0],
    );
    let fresh = fit.export_brep(|| false).unwrap();
    let (fresh_start, _) = topexp::vertices(&fresh.edge, true);
    assert_eq!(BRepTool::pnt(&fresh_start).unwrap(), current);
    assert!(matches!(
        fit.export_brep(|| true),
        Err(HelixFailure::Cancelled(HelixStage::Export))
    ));
}

#[test]
fn frame_deserialization_cannot_admit_a_collapsed_spatial_helix() {
    use martensite_tkernel::standard::dump::{DumpStream, dump_vector};
    let mut stored = DumpStream::default();
    for (name, coefficients) in [
        ("Location", [2., 3., 4.]),
        ("Direction", [0., 0., 1.]),
        ("XDirection", [1., 0., 0.]),
        ("YDirection", [1., 0., 0.]),
    ] {
        dump_vector(&mut stored, name, &coefficients).unwrap();
    }
    let mut placement = Axis2::default();
    assert!(placement.init_from_json(stored.text(), &mut 1).unwrap());
    assert!(matches!(
        ConstantHelix::new(ConstantHelixRequest {
            placement,
            ..request(CYLINDER, HelixRotation::Negative)
        }),
        Err(HelixFailure::InvalidFrame(_))
    ));
}

#[path = "authored_helix/replay.rs"]
mod replay;
