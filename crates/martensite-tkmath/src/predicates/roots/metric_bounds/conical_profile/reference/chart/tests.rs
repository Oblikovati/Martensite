// SPDX-License-Identifier: GPL-2.0-only
use super::*;
fn execute<T>(body: impl FnOnce(&mut Arithmetic<'_>) -> T) -> T {
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: 128,
            max_operations: 100_000,
        },
        &mut never,
    );
    body(&mut arithmetic)
}
#[test]
fn half_angle_above_one_retains_a_strict_physical_range() {
    execute(|arithmetic| {
        let range = fitted_range(-2., 2., arithmetic).unwrap();
        bounds::width(&range, arithmetic).unwrap();
        let upper = arithmetic.upper_f64(range.upper).unwrap();
        let expected = 2. * martensite_cmath::atan2(2., 1.);
        assert!(upper >= expected);
        assert!(upper < std::f64::consts::PI);
    });
}
#[test]
fn coefficient_extrema_are_owned_values_and_nonfinite_is_refused() {
    assert_eq!(coefficient_range(&[2., -3., 1.]).unwrap(), [-3., 2.]);
    assert!(coefficient_range(&[]).is_err());
    assert!(coefficient_range(&[f64::NAN]).is_err());
    assert!(coefficient_range(&[f64::INFINITY]).is_err());
}
#[test]
fn homotopy_uses_displaced_halfplane_and_refuses_axis_contact() {
    let poles = [[80., -18.], [84., 24.]];
    let spans = [poles.as_slice()];
    let proof = domain_receipt(&spans);
    execute(|arithmetic| {
        let small = arithmetic.scalar(1e-6).unwrap();
        homotopy(&proof, &small, arithmetic).unwrap();
        let contact = arithmetic.scalar(80.).unwrap();
        assert!(homotopy(&proof, &contact, arithmetic).is_err());
    });
}
#[test]
fn quantitative_angle_arithmetic_keeps_cancellation_and_work_failures() {
    let mut stop = || true;
    let policy = ArithmeticPolicy {
        fractional_bits: 128,
        max_operations: 100_000,
    };
    let mut arithmetic = Arithmetic::new(policy, &mut stop);
    assert!(matches!(
        fitted_range(-2., 2., &mut arithmetic),
        Err(ConicalReferenceFailure::Arithmetic(
            BoundedArithmeticFailure::Cancelled
        ))
    ));
}

fn domain_receipt(spans: &[&[[f64; 2]]]) -> ConicalChartDomainEvidence {
    let query = ConicalChartDomainQuery {
        spans,
        semi_angle: 0.125,
        chart_origin: 0.,
        fit_center: 0.,
        developed_turn: 0,
        physical_turn: 0,
        options: ConeClearanceOptions {
            fractional_bits: 128,
            max_operations: 100_000,
        },
    };
    certify_conical_chart_domain(query, || false).unwrap()
}
