//! Adjacent fits use distinct local tangents at the same source incidence.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::certify_conical_subspan_chart;

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
    check_distinct_charts(prepared, spans);
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
    let extremes =
        [left.poles[0], *right.poles.last().unwrap()].map(|[x, y]| martensite_cmath::atan2(y, x));
    let span = (extremes[1] - extremes[0]) / martensite_cmath::sin(prepared.semi_angle);
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
    plan(prepared, source, None, endpoints, work, &mut || false).unwrap()
}

fn check_distinct_charts(prepared: &Prepared, spans: [CurveSpan; 2]) {
    let [left, right] = spans.map(|span| &prepared.fits[&span]);
    let left_view = left.fit.view();
    let right_view = right.fit.view();
    assert_ne!(left_view.chart_center, right_view.chart_center);
    assert_ne!(left_view.tangent[3], right_view.tangent[0]);
}
fn check_join(prepared: &Prepared, spans: [CurveSpan; 2]) {
    let [left, right] = spans.map(|span| &prepared.fits[&span]);
    let left_view = left.fit.view();
    let right_view = right.fit.view();
    assert_eq!(left_view.slant_numerator[3], right_view.slant_numerator[0]);
    assert_eq!(left.uv[3].map(f64::to_bits), right.uv[0].map(f64::to_bits));
    for (last_row, first_row) in left.poles.iter().zip(&right.poles) {
        assert_eq!(
            last_row.points.last().unwrap().map(f64::to_bits),
            first_row.points[0].map(f64::to_bits)
        );
    }
}

#[test]
fn preassigned_local_restrictions_fit_the_wider_boundary() {
    let (inputs, mut selected, options) = fixture();
    let [left, right] = wide_weight_pair(&inputs.originals[0].loops[0]);
    let prepared = &mut selected.prepared[0];
    prepared.semi_angle = 0.125;
    prepared.fits.clear();
    assert_needs_two_charts(prepared, &left, &right, options);
    assert_unsplit_weight_refusal(prepared, &left, options);
    prepared.chart_domain = None;
    let pieces = fit_local_halves(prepared, [&left, &right], options);
    let joins = pieces
        .windows(2)
        .map(|pair| [pair[0].span, pair[1].span])
        .collect::<Vec<_>>();
    prepared.source.loops = vec![pieces];
    super::super::super::canonicalize::uv_endpoints(prepared).unwrap();
    for spans in joins {
        check_join(prepared, spans);
    }
}

fn wide_weight_pair(authored: &[AuthoredPiece]) -> [AuthoredPiece; 2] {
    let [mut left, mut right] = [authored[0].clone(), authored[1].clone()];
    left.poles = vec![[8., -5.], [10., 0.]];
    right.poles = vec![[10., 0.], [14., 8.]];
    [left, right]
}

fn assert_unsplit_weight_refusal(
    prepared: &mut Prepared,
    left: &AuthoredPiece,
    options: ConeClearanceOptions,
) {
    let mut work = Work::new(options);
    let center = -2.25;
    let spans = [&left.poles[..]];
    let query = local_query(&spans, prepared, center, &work);
    let domain = certify_conical_chart_domain_with_version(
        query,
        ConicalChartDomainVersion::RecenteredV4,
        || false,
    )
    .unwrap();
    work.charge(prepared.source.region, Some(left.span), domain.operations())
        .unwrap();
    prepared.chart_domain = Some(std::sync::Arc::new(domain));
    prepared.chart_center = center;
    let result = plan(
        prepared,
        left,
        None,
        &mut endpoints::EndpointPool::default(),
        &mut work,
        &mut || false,
    );
    let cause = result.err().expect("unsplit local fit must refuse").cause;
    let ConeCause::Coefficients(cone_mapping::ConeFitFailure::NonpositiveWeight { value, .. }) =
        cause
    else {
        panic!("expected nonpositive stored weight, got {cause:?}");
    };
    assert!(value <= 0.);
}

fn fit_local_halves(
    prepared: &mut Prepared,
    originals: [&AuthoredPiece; 2],
    options: ConeClearanceOptions,
) -> Vec<AuthoredPiece> {
    let mut work = Work::new(options);
    let mut endpoints = endpoints::EndpointPool::default();
    let mut pieces = Vec::new();
    for (original, centers) in originals.into_iter().zip([[-3.3, -1.1], [1.3, 3.4]]) {
        for (index, center) in (0..=1).zip(centers) {
            let proof = local_half(prepared, original, index, center, &mut work);
            let fitted = plan(
                prepared,
                original,
                Some(proof),
                &mut endpoints,
                &mut work,
                &mut || false,
            )
            .unwrap();
            assert!(
                fitted
                    .iter()
                    .all(|piece| u64::from(piece.span.index) >> (piece.span.level - 1) == index)
            );
            pieces.extend(fitted);
        }
    }
    pieces
}

fn local_half(
    prepared: &mut Prepared,
    source: &AuthoredPiece,
    index: u64,
    center: f64,
    work: &mut Work,
) -> ConicalChartDomainEvidence {
    let spans = [&source.poles[..]];
    let query = local_query(&spans, prepared, center, work);
    let domain = certify_conical_subspan_chart(query, index, 1, || false).unwrap();
    work.charge(
        prepared.source.region,
        Some(source.span),
        domain.operations(),
    )
    .unwrap();
    domain
}

fn local_query<'a>(
    spans: &'a [&'a [[f64; 2]]],
    prepared: &Prepared,
    center: f64,
    work: &Work,
) -> ConicalChartDomainQuery<'a> {
    ConicalChartDomainQuery {
        spans,
        semi_angle: prepared.semi_angle,
        chart_origin: prepared.chart_origin,
        fit_center: center,
        developed_turn: 0,
        physical_turn: 0,
        options: work.remaining(),
    }
}
