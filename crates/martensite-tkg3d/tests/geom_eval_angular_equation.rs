// SPDX-License-Identifier: GPL-2.0-only
//! Owned analytic quarter-turns and independent native-envelope checks.
use martensite_tkg3d::{
    geom::GeomCurve,
    geom_eval::{AngularEquationExport, AngularEquationExportError, export_angular_equation},
};
use martensite_tkmath::predicates::roots::{
    cartesian_equation::{
        CartesianEquationError,
        EquationInstruction::{self as Op, *},
    },
    metric_bounds::angular_equation::{
        AngularChartQuantity, AngularEquationCurve, AngularEquationError as Error,
        AngularEquationMode as Mode,
    },
};
use std::f64::consts::{FRAC_PI_2, PI};

const T: [Op; 1] = [Parameter];
const TWO: [Op; 1] = [Literal(2.)];
const ZERO: [Op; 1] = [Literal(0.)];

fn quarter(mode: Mode) -> AngularEquationCurve {
    let inclination = [Literal(FRAC_PI_2)];
    let second: &[Op] = match mode {
        Mode::Cylindrical => &T,
        Mode::Spherical => &inclination,
    };
    AngularEquationCurve::compile(mode, [&TWO, second, &T], [0., FRAC_PI_2], || false).unwrap()
}

#[test]
fn cylindrical_owned_quarter_uses_exact_binary64_radians() {
    let curve = quarter(Mode::Cylindrical);
    let start = curve.jet(0., || false).unwrap();
    assert_eq!(curve.mode(), Mode::Cylindrical);
    assert_eq!(curve.domain(), [0., FRAC_PI_2]);
    assert_eq!(start.nominal.position, [2., 0., 0.]);
    assert_eq!(start.nominal.first, [0., 2., 1.]);
    assert_eq!(start.nominal.second, [-2., 0., 0.]);
    assert_eq!(start.nominal.third, [0., -2., 0.]);
    assert_eq!(start.bounds[1], [[0.; 2], [2.; 2], [1.; 2]]);

    let end = curve.jet(FRAC_PI_2, || false).unwrap();
    close(end.nominal.position, [0., 2., FRAC_PI_2], 1e-15);
    close(end.nominal.first, [-2., 0., 1.], 1e-15);
    close(end.nominal.second, [0., -2., 0.], 1e-15);
    close(end.nominal.third, [2., 0., 0.], 1e-15);
    // Independent decimal oracle of 2*cos(binary64 FRAC_PI_2), not of exact pi/2.
    let x = end.bounds[0][0];
    assert!(x[0] >= 1.224_646_799_147_352e-16 && x[1] <= 1.224_646_799_147_355e-16);
    assert_eq!(curve.certificate().speed_lower, 2.);
    assert!(curve.certificate().coordinate_bounds[4][0][1] >= 2.);
}

#[test]
fn spherical_owned_equator_and_variable_inclination_follow_analytic_jets() {
    let curve = quarter(Mode::Spherical);
    let start = curve.jet(0., || false).unwrap();
    let end = curve.jet(FRAC_PI_2, || false).unwrap();
    close(start.nominal.position, [2., 0., 0.], 1e-15);
    close(end.nominal.position, [0., 2., 0.], 1e-15);
    close(start.nominal.first, [0., 2., 0.], 1e-15);
    close(end.nominal.first, [-2., 0., 0.], 1e-15);
    close(end.nominal.second, [0., -2., 0.], 1e-15);
    close(end.nominal.third, [2., 0., 0.], 1e-15);
    assert_eq!(start.bounds[0][2], end.bounds[0][2]);
    assert!(start.bounds[0][2][0] > 0.);
    assert!(curve.certificate().speed_lower > 1.999_999_999_999_99);

    let inclination = [Literal(0.5), Parameter, Add];
    let azimuth = [Parameter, Parameter, Multiply];
    let moving = AngularEquationCurve::compile(
        Mode::Spherical,
        [&TWO, &inclination, &azimuth],
        [0., 0.5],
        || false,
    )
    .unwrap();
    let jet = moving.jet(0., || false).unwrap().nominal;
    let [sine, cosine] = [
        2. * martensite_cmath::sin(0.5),
        2. * martensite_cmath::cos(0.5),
    ];
    close(jet.position, [sine, 0., cosine], 1e-15);
    close(jet.first, [cosine, 0., -sine], 1e-15);
    close(jet.second, [-sine, 2. * sine, -cosine], 1e-15);
    close(jet.third, [-cosine, 6. * cosine, sine], 4e-15);
}

#[test]
fn nonlinear_cylindrical_laws_preserve_product_and_chain_coefficients() {
    let radius = [Literal(2.), Parameter, Add];
    let angle = [Parameter, Parameter, Multiply];
    let axial = [Parameter, Parameter, Multiply, Parameter, Multiply];
    let curve = AngularEquationCurve::compile(
        Mode::Cylindrical,
        [&radius, &angle, &axial],
        [0., 0.5],
        || false,
    )
    .unwrap();
    let start = curve.jet(0., || false).unwrap().nominal;
    assert_eq!(start.position, [2., 0., 0.]);
    assert_eq!(start.first, [1., 0., 0.]);
    assert_eq!(start.second, [0., 4., 0.]);
    assert_eq!(start.third, [0., 6., 6.]);
}

fn close(actual: [f64; 3], expected: [f64; 3], tolerance: f64) {
    for (a, b) in actual.into_iter().zip(expected) {
        assert!((a - b).abs() <= tolerance, "actual {a:?}, expected {b:?}");
    }
}

#[test]
fn angular_exports_verify_native_poles_joins_and_achieved_error() {
    fn shared<T: Send + Sync>() {}
    shared::<AngularEquationCurve>();
    shared::<AngularEquationExport>();
    shared::<AngularEquationExportError>();
    for mode in [Mode::Cylindrical, Mode::Spherical] {
        let curve = quarter(mode);
        let limit = 1e-6;
        let export = export_angular_equation(&curve, limit, || false).unwrap();
        let certificate = export.certificate().clone();
        assert_eq!(certificate.authored_domain(), [0., FRAC_PI_2]);
        assert!(certificate.distance_bound() > 0. && certificate.distance_bound() <= limit);
        assert!(certificate.spans().len() > 1);
        let native = export.into_native();
        let mut end = 0.;
        for (i, (span, curve)) in certificate.spans().iter().zip(native.iter()).enumerate() {
            let [low, high] = span.normalized_interval();
            assert_eq!(low, end);
            end = high;
            assert_eq!(curve.degree(), 3);
            assert_eq!([curve.first_parameter(), curve.last_parameter()], [0., 1.]);
            assert!(span.local_speed_lower() > 0.);
            assert!(span.remainder_bound() > 0.);
            assert!(span.control_error_bound() <= span.distance_bound());
            for (p, expected) in (1..=4).zip(span.poles()) {
                assert_eq!(
                    curve.pole(p).unwrap().coordinates().map(f64::to_bits),
                    expected.map(f64::to_bits)
                );
            }
            if i > 0 {
                assert_eq!(native[i - 1].pole(4).unwrap(), curve.pole(1).unwrap());
            }
            check_fit_samples(mode, curve, [low, high], span.distance_bound());
        }
        assert_eq!(end, 1.);
    }
}

fn check_fit_samples(mode: Mode, curve: &impl GeomCurve, [low, high]: [f64; 2], bound: f64) {
    for u in [0., 0.125, 0.375, 0.5, 0.875, 1.] {
        let t = (low + u * (high - low)) * FRAC_PI_2;
        let (sine, cosine) = martensite_cmath::sincos(t);
        let z = if mode == Mode::Cylindrical { t } else { 0. };
        let actual = curve.d0(u).unwrap();
        let residual = [2. * cosine, 2. * sine, z]
            .into_iter()
            .zip(actual.coordinates())
            .map(|(a, b)| (a - b) * (a - b))
            .sum::<f64>();
        let cap = bound + 1e-14;
        assert!(residual <= cap * cap);
    }
}

#[test]
fn rejects_singular_charts_nonfinite_and_uncertifiable_distance() {
    assert!(matches!(
        AngularEquationCurve::compile(Mode::Cylindrical, [&T, &T, &T], [0., 1.], || false),
        Err(Error::ChartViolation(AngularChartQuantity::Radius))
    ));
    for inclination in [0., PI.next_up()] {
        assert!(matches!(
            AngularEquationCurve::compile(
                Mode::Spherical,
                [&TWO, &[Literal(inclination)], &T],
                [0., 1.],
                || false
            ),
            Err(Error::ChartViolation(AngularChartQuantity::Inclination))
        ));
    }
    assert!(matches!(
        AngularEquationCurve::compile(Mode::Cylindrical, [&TWO, &ZERO, &ZERO], [0., 1.], || false),
        Err(Error::Singular)
    ));
    assert!(matches!(
        AngularEquationCurve::compile(
            Mode::Cylindrical,
            [&TWO, &[Literal(f64::MAX)], &T],
            [0., 1.],
            || false
        ),
        Err(Error::ArgumentReduction { .. })
    ));
    assert!(matches!(
        AngularEquationCurve::compile(Mode::Cylindrical, [&TWO, &T, &T], [1., 0.], || false),
        Err(Error::Law(CartesianEquationError::Domain(_)))
    ));
    let curve = quarter(Mode::Cylindrical);
    assert!(matches!(
        curve.jet(f64::NAN, || false),
        Err(Error::Parameter { .. })
    ));
    assert!(matches!(
        export_angular_equation(&curve, 0., || false),
        Err(AngularEquationExportError::Certificate(Error::FitLimit {
            limit: 0.,
            ..
        }))
    ));
}

#[test]
fn cancelling_the_last_native_checkpoint_cannot_publish_a_cover() {
    let curve = quarter(Mode::Cylindrical);
    let mut steps = 0;
    let limit = 1e-3;
    assert!(
        export_angular_equation(&curve, limit, || {
            steps += 1;
            false
        })
        .is_ok()
    );
    let mut cancelled = 0;
    assert!(matches!(
        export_angular_equation(&curve, limit, || {
            cancelled += 1;
            cancelled == steps
        }),
        Err(AngularEquationExportError::Certificate(Error::Cancelled))
    ));
    let mut evaluate = 0;
    assert!(matches!(
        curve.jet(0.25, || {
            evaluate += 1;
            evaluate == 10
        }),
        Err(Error::Cancelled)
    ));
}

#[test]
fn a_small_model_error_cannot_certify_a_stationary_rounded_span() {
    let radius = [Literal(18_014_398_509_481_984.), Parameter, Add];
    let curve =
        AngularEquationCurve::compile(Mode::Cylindrical, [&radius, &ZERO, &ZERO], [0., 1.], || {
            false
        })
        .unwrap();
    assert_eq!(curve.certificate().speed_lower, 1.);
    assert!(matches!(
        export_angular_equation(&curve, 2., || false),
        Err(AngularEquationExportError::Certificate(
            Error::RoundedCarrier(CartesianEquationError::Singular)
        ))
    ));
}

#[test]
fn angular_fit_receipts_match_across_processes() {
    let replay = || {
        let result = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "geom_eval_angular_equation::emit_angular_receipt",
                "--nocapture",
            ])
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let stdout = String::from_utf8(result.stdout).unwrap();
        stdout
            .split_once("ANGULAR_EQ=")
            .unwrap()
            .1
            .lines()
            .next()
            .unwrap()
            .to_owned()
    };
    assert_eq!(replay(), replay());
}

#[test]
fn emit_angular_receipt() {
    let curve = quarter(Mode::Spherical);
    let export = export_angular_equation(&curve, 1e-3, || false).unwrap();
    let proof = export.certificate().clone();
    let cubics = export.into_native();
    let controls: Vec<_> = cubics
        .iter()
        .map(|cubic| [1, 2, 3, 4].map(|i| cubic.pole(i).unwrap().coordinates().map(f64::to_bits)))
        .collect();
    let spans: Vec<_> = proof
        .spans()
        .iter()
        .map(|span| {
            [
                span.normalized_interval()[0].to_bits(),
                span.normalized_interval()[1].to_bits(),
                span.distance_bound().to_bits(),
            ]
        })
        .collect();
    println!("ANGULAR_EQ={spans:016x?} {controls:016x?}");
}
