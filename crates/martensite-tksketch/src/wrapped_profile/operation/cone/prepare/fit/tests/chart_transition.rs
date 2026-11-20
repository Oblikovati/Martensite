//! Adjacent fits use distinct local tangents at the same source incidence.
use super::*;

#[test]
fn adjacent_local_charts_certify_one_shared_physical_join() {
    let (inputs, mut selected, options) = fixture();
    let [left, right] = wide_pair(&inputs.originals[0].loops[0]);
    let prepared = &mut selected.prepared[0];
    prepared.semi_angle = 0.125;
    prepared.fits.clear();
    assert_needs_two_charts(prepared, &left, &right, options);
    let mut endpoints = endpoints::EndpointPool::default();
    let mut work = Work::new(options);
    let first = prepare(prepared, &left, -1.625, &mut endpoints, &mut work);
    let second = prepare(prepared, &right, 1.625, &mut endpoints, &mut work);
    let spans = [first.last().unwrap().span, second[0].span];
    prepared.source.loops = vec![first.into_iter().chain(second).collect()];
    super::super::super::canonicalize::uv_endpoints(prepared).unwrap();
    check_join(prepared, spans);
}

fn wide_pair(authored: &[AuthoredPiece]) -> [AuthoredPiece; 2] {
    let [mut left, mut right] = [authored[0].clone(), authored[1].clone()];
    left.poles = vec![[8., -3.5], [10., 0.]];
    right.poles = vec![[10., 0.], [14., 6.25]];
    [left, right]
}

fn assert_needs_two_charts(
    prepared: &Prepared,
    left: &AuthoredPiece,
    right: &AuthoredPiece,
    options: ConeClearanceOptions,
) {
    let span = (martensite_cmath::atan(6.25 / 14.) + martensite_cmath::atan(3.5 / 8.))
        / martensite_cmath::sin(prepared.semi_angle);
    assert!(span > std::f64::consts::TAU);
    let query = ConicalChartDomainQuery {
        spans: &[&left.poles, &right.poles],
        semi_angle: prepared.semi_angle,
        chart_origin: prepared.chart_origin,
        fit_center: 0.,
        developed_turn: 0,
        physical_turn: 0,
        options,
    };
    assert!(
        certify_conical_chart_domain_with_version(
            query,
            ConicalChartDomainVersion::RecenteredV4,
            || false
        )
        .is_err()
    );
}

fn prepare(
    prepared: &mut Prepared,
    source: &AuthoredPiece,
    center: f64,
    endpoints: &mut endpoints::EndpointPool,
    work: &mut Work,
) -> Vec<AuthoredPiece> {
    let query = ConicalChartDomainQuery {
        spans: &[&source.poles],
        semi_angle: prepared.semi_angle,
        chart_origin: prepared.chart_origin,
        fit_center: center,
        developed_turn: 0,
        physical_turn: 0,
        options: work.remaining(),
    };
    let domain = certify_conical_chart_domain_with_version(
        query,
        ConicalChartDomainVersion::RecenteredV4,
        || false,
    )
    .unwrap();
    work.charge(
        prepared.source.region,
        Some(source.span),
        domain.operations(),
    )
    .unwrap();
    prepared.chart_domain = Some(std::sync::Arc::new(domain));
    prepared.chart_center = center;
    plan(prepared, source, endpoints, work, &mut || false).unwrap()
}

fn check_join(prepared: &Prepared, spans: [CurveSpan; 2]) {
    let [left, right] = spans.map(|span| &prepared.fits[&span]);
    let left_view = left.fit.view();
    let right_view = right.fit.view();
    assert_ne!(left_view.chart_center, right_view.chart_center);
    assert_ne!(left_view.tangent[3], right_view.tangent[0]);
    assert_eq!(left_view.slant_numerator[3], right_view.slant_numerator[0]);
    assert_eq!(left.uv[3].map(f64::to_bits), right.uv[0].map(f64::to_bits));
    for (last_row, first_row) in left.poles.iter().zip(&right.poles) {
        assert_eq!(
            last_row.points.last().unwrap().map(f64::to_bits),
            first_row.points[0].map(f64::to_bits)
        );
    }
}
