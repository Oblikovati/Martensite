// SPDX-License-Identifier: GPL-2.0-only
//! Independent allocation, angular branch and whole-depth metric controls.
use super::*;
fn execute<T>(body: impl FnOnce(&mut Arithmetic<'_>) -> T) -> T {
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: 128,
            max_operations: 10_000,
        },
        &mut never,
    );
    body(&mut arithmetic)
}
#[test]
fn storage_roundoff_has_its_own_thousandth_epsilon_gate() {
    execute(|arithmetic| {
        let admitted = arithmetic.scalar(1e-8).unwrap();
        geometry::allocated(&admitted, 1e-4, 1000, "storage", arithmetic).unwrap();
        let excessive = arithmetic.scalar(2e-7).unwrap();
        let refusal =
            geometry::allocated(&excessive, 1e-4, 1000, "storage", arithmetic).unwrap_err();
        assert!(matches!(
            refusal,
            ConicalReferenceFailure::Bound {
                field: "storage",
                ..
            }
        ));
        geometry::allocated(&excessive, 1e-4, 4, "reference fit", arithmetic).unwrap();
    });
}
#[test]
fn strict_half_angle_coefficient_bound_proves_a_single_principal_chart() {
    execute(|arithmetic| {
        let one = arithmetic.scalar(1.).unwrap();
        geometry::angular_coefficient(0.9, &one, arithmetic).unwrap();
        geometry::angular_coefficient(-0.9, &one, arithmetic).unwrap();
        assert!(geometry::angular_coefficient(1., &one, arithmetic).is_err());
        assert!(geometry::angular_coefficient(-1., &one, arithmetic).is_err());
    });
}
#[test]
fn filled_normal_metric_encloses_closed_form_distortion() {
    execute(|arithmetic| {
        let angle = arithmetic.scalar(0.7).unwrap();
        let [cosine, sine] = trigonometric::scalar(&angle, arithmetic).unwrap();
        let halfplane = arithmetic.scalar(4.).unwrap();
        let factor =
            geometry::distortion([-0.1, 0.2], &halfplane, &sine, &cosine, arithmetic).unwrap();
        let upper = arithmetic.upper_f64(factor.upper).unwrap();
        let expected = 1. + 0.2 * martensite_cmath::cos(0.7) / (4. * martensite_cmath::sin(0.7));
        assert!(upper >= expected.next_down());
        assert!(upper <= expected.next_up().next_up());
    });
}
#[test]
fn whole_depth_reference_radius_rejects_crossing_the_axis() {
    execute(|arithmetic| {
        let angle = arithmetic.scalar(0.7).unwrap();
        let [cosine, sine] = trigonometric::scalar(&angle, arithmetic).unwrap();
        let halfplane = arithmetic.scalar(4.).unwrap();
        geometry::radius([-0.1, 0.2], &halfplane, &sine, &cosine, arithmetic).unwrap();
        assert!(geometry::radius([-10., 0.2], &halfplane, &sine, &cosine, arithmetic).is_err());
    });
}
#[test]
fn reference_arithmetic_cancellation_is_never_success() {
    let mut stop = || true;
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: 128,
            max_operations: 10_000,
        },
        &mut stop,
    );
    let one = arithmetic.zero();
    let cause = geometry::angular_coefficient(0.1, &one, &mut arithmetic).unwrap_err();
    assert!(matches!(
        cause,
        ConicalReferenceFailure::Arithmetic(BoundedArithmeticFailure::Cancelled)
    ));
}
#[test]
fn reference_arithmetic_work_limit_is_never_success() {
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: 128,
            max_operations: 0,
        },
        &mut never,
    );
    let one = arithmetic.zero();
    assert!(geometry::angular_coefficient(0.1, &one, &mut arithmetic).is_err());
}

#[test]
fn quantitative_component_fingerprint_preserves_exact_original_span_order() {
    let first = [[80., -18.], [84., 0.]];
    let second = [[84., 0.], [84., 24.]];
    let forward = [first.as_slice(), second.as_slice()];
    let reverse = [second.as_slice(), first.as_slice()];
    let old = domain_receipt(&forward);
    let reordered = domain_receipt(&reverse);
    assert_eq!(old.operations(), reordered.operations());
    let mut original_bits = Vec::new();
    let mut reordered_bits = Vec::new();
    binding::chart_bits(&old, &mut original_bits);
    binding::chart_bits(&reordered, &mut reordered_bits);
    assert_ne!(original_bits, reordered_bits);
    assert_first_span(&original_bits, &first);
}
fn domain_receipt(spans: &[&[[f64; 2]]]) -> ConicalChartDomainEvidence {
    let input = ConicalChartDomainQuery {
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
    certify_conical_chart_domain(input, || false).unwrap()
}

fn assert_first_span(original_bits: &[u64], first: &[[f64; 2]]) {
    assert_eq!(
        &original_bits[7..11],
        &first
            .iter()
            .flatten()
            .map(|x| x.to_bits())
            .collect::<Vec<_>>()
    );
}
