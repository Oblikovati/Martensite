// SPDX-License-Identifier: GPL-2.0-only
//! Independent analytic fixtures for the Cartesian equation operation and its native export.
use martensite_tkg3d::{
    geom::GeomCurve,
    geom_eval::{CartesianEquationExport, CartesianEquationExportError, export_cartesian_equation},
};
use martensite_tkmath::predicates::roots::cartesian_equation::{
    CartesianEquationCurve, CartesianEquationError as Error, EquationBezierCertificate,
    EquationExpressionError,
    EquationInstruction::{self as Op, *},
    MAX_CARTESIAN_EQUATION_INSTRUCTIONS,
};

const T: [Op; 1] = [Parameter];
const ZERO: [Op; 1] = [Literal(0.)];
const T2: [Op; 3] = [Parameter, Parameter, Multiply];

fn parabola() -> CartesianEquationCurve {
    CartesianEquationCurve::compile([&T, &T2, &ZERO], [-1., 1.], || false).unwrap()
}

#[test]
fn owned_parabola_analytic_jet_and_native_curve_match() {
    fn shared<T: Send + Sync>() {}
    shared::<CartesianEquationCurve>();
    shared::<CartesianEquationExport>();
    shared::<EquationBezierCertificate>();
    shared::<Error>();
    shared::<CartesianEquationExportError>();
    let curve = parabola();
    let bounds = curve.certificate();
    assert_eq!(curve.domain(), [-1., 1.]);
    assert_eq!(bounds.speed_lower, 1.);
    let export = export_cartesian_equation(&curve, 0., || false).unwrap();
    assert_eq!(export.certificate().authored_domain(), [-1., 1.]);
    assert_eq!(export.certificate().distance_bound(), 0.);
    assert_eq!(
        export.certificate().poles(),
        &[[-1., 1., 0.], [0., -1., 0.], [1., 1., 0.]]
    );
    let native = export.into_native();
    assert_eq!(native.degree(), 2);
    assert_eq!(
        [native.first_parameter(), native.last_parameter()],
        [0., 1.]
    );
    for t in [-1., -0.5, 0., 0.5, 1.] {
        let jet = curve.jet(t, || false).unwrap();
        assert_eq!(jet.position, [t, t * t, 0.]);
        assert_eq!(jet.first, [1., 2. * t, 0.]);
        assert_eq!(jet.second, [0., 2., 0.]);
        assert_eq!(jet.third, [0.; 3]);
        assert_eq!(
            *native.d0((t + 1.) * 0.5).unwrap().coordinates(),
            jet.position
        );
        for (axis, [lo, hi]) in bounds.coordinate_bounds[0].into_iter().enumerate() {
            assert!((lo..=hi).contains(&jet.position[axis]));
        }
    }
}

#[test]
fn cubic_uses_the_authored_parameter_for_all_derivatives() {
    let cubic = [
        Parameter,
        Literal(-1.),
        Add,
        Parameter,
        Parameter,
        Multiply,
        Multiply,
    ];
    let z = [Literal(1.), Literal(3.), Subtract, Negate];
    let curve = CartesianEquationCurve::compile([&T, &cubic, &z], [2., 4.], || false).unwrap();
    for t in [2., 2.5, 4.] {
        let jet = curve.jet(t, || false).unwrap();
        assert_eq!(jet.position, [t, t * t * (t - 1.), 2.]);
        assert_eq!(jet.first, [1., 3. * t * t - 2. * t, 0.]);
        assert_eq!(jet.second, [0., 6. * t - 2., 0.]);
        assert_eq!(jet.third, [0., 6., 0.]);
    }
    let export = export_cartesian_equation(&curve, 1e-12, || false).unwrap();
    assert!(
        export
            .certificate()
            .normalized_domain_certificate()
            .speed_lower
            > 0.
    );
    assert_eq!(export.into_native().degree(), 3);
}

#[test]
fn interval_regular_curve_without_a_fixed_sign_coordinate_subdivides() {
    let y = [
        Parameter,
        Literal(0.5),
        Subtract,
        Parameter,
        Literal(0.5),
        Subtract,
        Multiply,
    ];
    let curve = CartesianEquationCurve::compile([&T2, &y, &ZERO], [-1., 1.], || false).unwrap();
    assert!(curve.certificate().speed_lower > 0.);
    let speed = curve.certificate().speed_lower;
    assert!(speed * speed <= 0.5);
    for t in [-0.5, 0., 0.25, 0.5, 1.] {
        assert_eq!(
            curve.jet(t, || false).unwrap().first,
            [2. * t, 2. * t - 1., 0.]
        );
    }
}

#[test]
fn singularities_refuse_and_nondyadic_roots_exhaust_the_bounded_proof() {
    for programs in [[&T2[..], &ZERO, &ZERO], [&ZERO[..], &ZERO, &ZERO]] {
        assert!(matches!(
            CartesianEquationCurve::compile(programs, [-1., 1.], || false),
            Err(Error::Singular)
        ));
    }
    let nondyadic = [
        Parameter,
        Parameter,
        Multiply,
        Parameter,
        Literal(2.),
        Multiply,
        Literal(3.),
        Divide,
        Subtract,
    ];
    assert!(matches!(
        CartesianEquationCurve::compile([&nondyadic, &ZERO, &ZERO], [0., 1.], || false),
        Err(Error::RegularityUnproven { .. })
    ));
}

#[test]
fn outward_rounded_native_discrepancy_binds_the_inclusive_limit() {
    let fraction = [Parameter, Literal(3.), Divide];
    let curve =
        CartesianEquationCurve::compile([&T, &fraction, &ZERO], [0., 1.], || false).unwrap();
    let export = export_cartesian_equation(&curve, 1e-15, || false).unwrap();
    let bound = export.certificate().distance_bound();
    // The binary64 value of 1/3 is its lower neighbor. Its exact error
    // is 1 / (3 * 2^54), which itself needs the next upward binary64 value.
    assert_eq!(bound, (1.0_f64 / 54_043_195_528_445_952.).next_up());
    assert!(export_cartesian_equation(&curve, bound, || false).is_ok());
    assert!(matches!(
        export_cartesian_equation(&curve, bound.next_down(), || false),
        Err(CartesianEquationExportError::Certificate(Error::ExportDistance { bound: Some(value), .. }))
            if value == bound
    ));
    let native = export.into_native();
    assert_eq!(native.degree(), 1);
    assert_eq!(*native.pole(2).unwrap().coordinates(), [1., 1. / 3., 0.]);
}

#[test]
fn native_rounding_and_parameter_scaling_cannot_invalidate_regularity_silently() {
    let far = [Literal(18_014_398_509_481_984.), Parameter, Add];
    let curve = CartesianEquationCurve::compile([&far, &ZERO, &ZERO], [0., 1.], || false).unwrap();
    assert_eq!(curve.certificate().speed_lower, 1.);
    assert!(matches!(
        curve.certify_bezier(1., || false),
        Err(Error::Singular)
    ));
    let wide = CartesianEquationCurve::compile([&T, &ZERO, &ZERO], [-f64::MAX, f64::MAX], || false)
        .unwrap();
    assert_eq!(wide.jet(0.5, || false).unwrap().position, [0.5, 0., 0.]);
    assert!(matches!(
        wide.certify_bezier(0., || false),
        Err(Error::Unrepresentable { order: 1, axis: 0 })
    ));
    let tiny = [
        Parameter,
        Literal(f64::from_bits(1)),
        Multiply,
        Literal(2.),
        Divide,
    ];
    assert!(matches!(
        CartesianEquationCurve::compile([&tiny, &ZERO, &ZERO], [0., 1.], || false),
        Err(Error::SpeedUnderflow)
    ));
    let overflow = [Literal(f64::MAX), Parameter, Add];
    assert!(matches!(
        CartesianEquationCurve::compile([&overflow, &T, &ZERO], [0., 1.], || false),
        Err(Error::Unrepresentable { order: 0, axis: 0 })
    ));
}

#[test]
fn malformed_domain_ir_and_unadmitted_operators_are_typed() {
    for domain in [[1., -1.], [0., 0.], [0., f64::INFINITY]] {
        assert!(matches!(
            CartesianEquationCurve::compile([&T, &T2, &ZERO], domain, || false),
            Err(Error::Domain(_))
        ));
    }
    let too_many = [Parameter; MAX_CARTESIAN_EQUATION_INSTRUCTIONS];
    assert!(matches!(
        CartesianEquationCurve::compile([&too_many, &T, &ZERO], [0., 1.], || false),
        Err(Error::InstructionLimit { .. })
    ));
    let cases: [(&[Op], EquationExpressionError); 6] = [
        (&[Parameter, Add], EquationExpressionError::MissingOperand),
        (
            &[Parameter, Parameter],
            EquationExpressionError::ResultCount(2),
        ),
        (
            &[Literal(f64::INFINITY)],
            EquationExpressionError::NonFiniteLiteral(f64::INFINITY),
        ),
        (
            &[Parameter, Literal(-0.), Divide],
            EquationExpressionError::ZeroDivisor,
        ),
        (
            &[Parameter, Parameter, Divide],
            EquationExpressionError::NonConstantDivisor,
        ),
        (
            &[
                Parameter, Parameter, Multiply, Parameter, Parameter, Multiply, Multiply,
            ],
            EquationExpressionError::Degree(4),
        ),
    ];
    for (program, expected) in cases {
        assert!(matches!(
            CartesianEquationCurve::compile([&T, program, &ZERO], [0., 1.], || false),
            Err(Error::Expression { axis: 1, cause, .. }) if cause == expected
        ));
    }
}

#[test]
fn cancellation_is_checked_during_compile_eval_and_before_export_publication() {
    let mut polls = 0;
    assert!(matches!(
        CartesianEquationCurve::compile([&T, &T2, &ZERO], [0., 1.], || {
            polls += 1;
            polls == 5
        }),
        Err(Error::Cancelled)
    ));
    let curve = parabola();
    assert!(matches!(
        curve.jet(f64::NAN, || false),
        Err(Error::Parameter { .. })
    ));
    assert!(matches!(
        curve.jet(1.0_f64.next_up(), || false),
        Err(Error::Parameter { .. })
    ));
    let mut eval = 0;
    assert!(matches!(
        curve.jet(0., || {
            eval += 1;
            eval == 3
        }),
        Err(Error::Cancelled)
    ));
    assert_eq!(eval, 3);
    let mut export = 0;
    assert!(
        export_cartesian_equation(&curve, 0., || {
            export += 1;
            false
        })
        .is_ok()
    );
    let mut cancelled = 0;
    assert!(matches!(
        export_cartesian_equation(&curve, 0., || {
            cancelled += 1;
            cancelled == export
        }),
        Err(CartesianEquationExportError::Certificate(Error::Cancelled))
    ));
}

#[test]
fn equation_and_native_pole_bits_replay_across_processes() {
    let receipt = || {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "geom_eval_equation_curve::emit_equation_receipt",
                "--nocapture",
            ])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8(output.stdout).unwrap();
        stdout
            .split_once("EQUATION_CURVE=")
            .unwrap()
            .1
            .lines()
            .next()
            .unwrap()
            .to_owned()
    };
    assert_eq!(receipt(), receipt());
}

#[test]
fn emit_equation_receipt() {
    let fractional = [Parameter, Parameter, Multiply, Literal(3.), Divide];
    let curve =
        CartesianEquationCurve::compile([&T, &fractional, &ZERO], [-1., 2.], || false).unwrap();
    let jet = curve.jet(0.25, || false).unwrap();
    let export = export_cartesian_equation(&curve, 1e-15, || false).unwrap();
    let achieved = export.certificate().distance_bound().to_bits();
    let native = export.into_native();
    let poles: Vec<_> = (1..=native.degree() + 1)
        .map(|index| native.pole(index).unwrap().coordinates().map(f64::to_bits))
        .collect();
    let jet = [jet.position, jet.first, jet.second, jet.third].map(|row| row.map(f64::to_bits));
    println!("EQUATION_CURVE={achieved:016x} {jet:016x?} {poles:016x?}");
}

#[test]
fn shared_equation_has_independent_owned_exports_and_parallel_jets() {
    let curve = parabola();
    let expected = curve.jet(0.25, || false).unwrap();
    let serial = export_cartesian_equation(&curve, 0., || false).unwrap();
    let controls = serial.certificate().clone();
    serial.into_native().reverse().unwrap();
    std::thread::scope(|scope| {
        for _ in 0..4 {
            let curve = &curve;
            let controls = &controls;
            scope.spawn(move || {
                for _ in 0..8 {
                    let jet = curve.jet(0.25, || false).unwrap();
                    let output = export_cartesian_equation(curve, 0., || false).unwrap();
                    assert_eq!(jet, expected);
                    assert_eq!(output.certificate(), controls);
                }
            });
        }
    });
}
