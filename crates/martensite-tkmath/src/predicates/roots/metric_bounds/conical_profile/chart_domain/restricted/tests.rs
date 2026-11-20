// SPDX-License-Identifier: GPL-2.0-only
use super::super::super::boundary_chart::ChartBinding;
use super::super::tests::query;
use super::*;

#[test]
fn exact_restriction_encloses_original_polynomial_extrema() {
    let poles = [[80., -70.], [80., -20.], [80., 70.]];
    let spans = [poles.as_slice()];
    let input = ConicalChartDomainQuery {
        fit_center: -2.8,
        ..query(&spans)
    };
    let version = ConicalChartDomainVersion::RecenteredV4;
    assert!(certify_conical_chart_domain_with_version(input, version, || false).is_err());

    let proof = certify_conical_subspan_chart(input, 1, 2, || false).unwrap();
    assert_eq!(proof.source_restriction(), Some((1, 2)));
    assert_eq!(proof.span_limit(), 1);
    let [lower, upper] = proof
        .relative_physical_bounds()
        .map(|v| v.binary64_bounds().unwrap());
    let physical = |t| {
        let y = -70. + 100. * t + 40. * t * t;
        martensite_cmath::atan2(y, 80.) / martensite_cmath::sin(input.semi_angle)
    };
    assert!(lower[0] <= physical(0.25) + f64::EPSILON);
    assert!(upper[1] >= physical(0.5) - f64::EPSILON);
    assert!((lower[1] - physical(0.25)).abs() < 1e-12);
    assert!((upper[0] - physical(0.5)).abs() < 1e-12);
    assert!(!proof.matches_with_version(&input, version));
}

#[test]
fn restriction_binding_refuses_ancestors_siblings_and_changed_domains() {
    let poles = [[80., -8.], [80., 8.]];
    let spans = [poles.as_slice()];
    let input = query(&spans);
    let left = certify_conical_subspan_chart(input, 0, 1, || false).unwrap();
    let right = certify_conical_subspan_chart(input, 1, 1, || false).unwrap();
    check_scope(&left, &poles);
    let rebuilt = certify_conical_subspan_chart(input, 0, 1, || false).unwrap();
    let binding = ChartBinding::capture(&left);
    assert!(binding.matches(&rebuilt));
    assert!(!binding.matches(&right));
    assert_ne!(
        binding.diagnostic_bits(),
        ChartBinding::capture(&right).diagnostic_bits()
    );
}

fn check_scope(chart: &ConicalChartDomainEvidence, poles: &[[f64; 2]]) {
    for (index, level, expected) in [
        (0, 1, true),
        (1, 2, true),
        (0, 0, false),
        (1, 1, false),
        (3, 1, false),
        (0, 64, false),
    ] {
        let span = ConicalOriginalSpan {
            poles,
            index,
            level,
        };
        assert_eq!(chart.covers_restriction(Some(span)), expected);
    }
    assert!(!chart.covers_restriction(None));
}

#[test]
fn invalid_interval_cancellation_and_budget_remain_distinct() {
    let poles = [[80., -8.], [80., 8.]];
    let spans = [poles.as_slice()];
    let mut input = query(&spans);
    assert!(matches!(
        certify_conical_subspan_chart(input, 2, 1, || false),
        Err(ConicalSubspanChartError::Interval(_))
    ));
    assert!(
        matches!(certify_conical_subspan_chart(input, 0, 1, || true), Err(ConicalSubspanChartError::Domain(cause)) if cause.is_cancelled())
    );
    input.options.max_operations = 1;
    assert!(
        matches!(certify_conical_subspan_chart(input, 0, 1, || false), Err(ConicalSubspanChartError::Domain(cause)) if !cause.is_cancelled() && cause.operations == 1)
    );
}

#[test]
fn interval_keys_keep_leading_zeroes_and_cover_the_full_u64_address_space() {
    assert_eq!(decode_restriction(0), None);
    for (index, level) in [(0, 0), (0, 1), (1, 1), (1, 63), (u64::MAX >> 1, 63)] {
        let span = ConicalOriginalSpan {
            poles: &[],
            index,
            level,
        };
        assert_eq!(
            decode_restriction(restriction_key(span)),
            Some((index, level))
        );
    }
}
