// SPDX-License-Identifier: GPL-2.0-only
use super::tests::query;
use super::*;

#[test]
fn recentered_proof_keeps_the_authored_lift_through_the_cardinal_cut() {
    let poles = [[80., 30.], [80., 40.]];
    let spans = [poles.as_slice()];
    let original = query(&spans);
    assert!(certify_conical_chart_domain(original, || false).is_err());
    let input = ConicalChartDomainQuery {
        fit_center: 3.3,
        ..original
    };
    let version = ConicalChartDomainVersion::RecenteredV4;
    let proof = certify_conical_chart_domain_with_version(input, version, || false).unwrap();
    let [min, max] = proof.relative_physical_bounds();
    assert!(min.binary64_bounds().unwrap()[1] < std::f64::consts::PI);
    assert!(max.binary64_bounds().unwrap()[0] > std::f64::consts::PI);
    assert!(proof.matches_with_version(&input, version));
    assert!(!proof.matches_with_version(&original, version));
    assert!(!proof.matches_with_version(&input, ConicalChartDomainVersion::FamilyV3));
}

#[test]
fn recentering_refuses_a_second_revolution_and_the_wrong_lift() {
    let wide = [[80., -40.], [80., 40.]];
    let spans = [wide.as_slice()];
    let version = ConicalChartDomainVersion::RecenteredV4;
    assert!(certify_conical_chart_domain_with_version(query(&spans), version, || false).is_err());
    let narrow = [[80., 30.], [80., 40.]];
    let spans = [narrow.as_slice()];
    let input = ConicalChartDomainQuery {
        fit_center: 3.3 - std::f64::consts::TAU,
        ..query(&spans)
    };
    assert!(certify_conical_chart_domain_with_version(input, version, || false).is_err());
}
