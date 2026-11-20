//! The automatic plan is consumed by the same fitter and native cap constructor.
use super::*;
use crate::surface_sketch::wrapped_profile::{
    planar_clearance, selection as periodic_selection, storage::MAX_CERTIFIED_SPANS,
};
use martensite_tkbo::{
    body_check::validate_body,
    profile_extrusion::{PlanarProofVersion, ProfileCurve, ProfileCurveId, ProfileVertexId},
};

const STRIP: [[f64; 2]; 4] = [
    [10., -5.5],
    [10.625, -5.84375],
    [31.875, 17.53125],
    [30., 16.5],
];

#[test]
fn planned_charts_construct_a_closed_cone_strip_wider_than_one_turn() {
    let request = strip_request();
    let run = ConeRun::from(&request);
    let inputs = input::capture(&request, &mut || false).unwrap();
    let original = &inputs.originals[0];
    let mut work = Work::new(request.proof.arithmetic);
    let planar =
        super::super::super::selection::planar(&run, &inputs.originals, &mut work, &mut || false)
            .unwrap();
    assert!(planar.members[&original.region].area.encloses(5445. / 256.));
    let mut prepared = prepare_strip(&run, original, &mut work);
    super::super::super::canonicalize::uv_endpoints(&mut prepared).unwrap();
    assert_closed_wide_uv(&prepared);
    let clearance = super::super::super::clearance::certify(
        &inputs.originals,
        std::slice::from_ref(&prepared),
        &mut work,
        &mut || false,
    )
    .unwrap();
    assert!(clearance.operations() > 0);
    let body = decode_strip(&prepared, request.support);
    let resolution =
        crate::surface_sketch::wrapped_profile::pipeline::resolution(original.tolerance);
    let report = validate_body(&body.shape, &resolution).unwrap();
    assert!(report.is_valid(), "{report:?}");
    let stored = storage::capture(original, &prepared, body, &mut work, &mut || false).unwrap();
    assert_per_span_authority(original, &prepared, &stored.spans);
    assert_certified_contour(original, &prepared, &stored);
    assert_periodic_cap_material(&prepared, &request, &stored.spans);
}

fn decode_strip(prepared: &Prepared, support: ConeSupport) -> coverage::MemberView {
    let built = native::construct(prepared, support).unwrap();
    let keys: Vec<_> = built.bindings.iter().map(|(key, _)| *key).collect();
    let bytes = crate::surface_sketch::encoding::encode(&built.solid, &built.bindings).unwrap();
    let (shape, bindings) =
        crate::surface_sketch::wrapped_profile::pipeline::decode(&bytes, &keys).unwrap();
    coverage::MemberView {
        region: prepared.source.region,
        shape,
        bindings,
    }
}

fn assert_per_span_authority(
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    spans: &[storage::SpanProof],
) {
    assert!(prepared.chart_domain.is_none());
    assert!(prepared.local_charts.len() > STRIP.len());
    assert_eq!(spans.len(), prepared.fits.len());
    for (piece, stored) in prepared.source.loops[0].iter().zip(spans) {
        assert_eq!(piece.span, stored.span);
        assert_eq!(stored.cap_pcurves, 2);
        assert!(stored.developed_boundary.is_some());
        let slot = prepared.fits[&piece.span].local_chart;
        assert!(slot.is_some());
        let metric = prepared.boundary_metric(piece.span, slot).unwrap();
        let query = storage::boundary_query(original, prepared, piece, &stored.curves).unwrap();
        assert!(metric.matches(&stored.boundary, &query));
    }
    reject_other_chart(prepared, spans);
    reject_sibling_chart(prepared, spans);
}

fn assert_certified_contour(
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    stored: &storage::Stored,
) {
    let spans = storage::cap_join_spans(original, prepared, &stored.spans).unwrap();
    let lengths = [spans.len()];
    let query = ConicalCapJoinsQuery {
        spans: &spans,
        contour_lengths: &lengths,
        reference_slant: prepared.reference_slant,
        options: stored.joins.cap_join_options().unwrap(),
    };
    assert!(stored.joins.matches_cap_joins(&query));
    assert_eq!(stored.joins.directions().len(), spans.len());
    let mut reordered = spans.clone();
    reordered.rotate_left(1);
    let changed = ConicalCapJoinsQuery {
        spans: &reordered,
        ..query
    };
    assert!(!stored.joins.matches_cap_joins(&changed));
    assert_broken_order(&spans, &changed);
}

fn assert_broken_order(spans: &[ConicalCapJoinSpan<'_>], original: &ConicalCapJoinsQuery<'_>) {
    let mut broken = spans.to_vec();
    broken.swap(0, 1);
    let query = ConicalCapJoinsQuery {
        spans: &broken,
        contour_lengths: original.contour_lengths,
        reference_slant: original.reference_slant,
        options: original.options,
    };
    let error =
        certify_conical_cap_joins(query, || false).expect_err("an open incidence must refuse");
    assert_eq!(error.index, Some(0));
    assert!(
        matches!(error.cause.as_ref(), ConicalJoinsFailure::Proof { cause, .. }
        if cause.kind() == ConicalJoinFailureKind::Join)
    );
}

fn reject_other_chart(prepared: &Prepared, spans: &[storage::SpanProof]) {
    let first = spans[0].span;
    let wrong_owner = spans
        .iter()
        .find(|stored| stored.span.curve != first.curve)
        .unwrap()
        .span;
    assert!(prepared.boundary_metric(first, None).is_err());
    assert!(
        prepared
            .boundary_metric(first, prepared.fits[&wrong_owner].local_chart)
            .is_err()
    );
}

fn reject_sibling_chart(prepared: &Prepared, spans: &[storage::SpanProof]) {
    let spans = spans.iter().map(|stored| stored.span);
    let (first, sibling) = spans
        .clone()
        .zip(spans.skip(1))
        .find(|(a, b)| {
            a.curve == b.curve && prepared.fits[a].local_chart != prepared.fits[b].local_chart
        })
        .expect("the wide source has sibling charts");
    let Err(error) = prepared.boundary_metric(first, prepared.fits[&sibling].local_chart) else {
        panic!("a chart must reject its sibling");
    };
    assert_eq!(error.span, Some(first));
}

fn strip_request() -> WrappedConeNormalRequest {
    let mut request = super::super::super::super::tests::request();
    request.support.semi_angle = 0.125;
    request.selections[0].holes.clear();
    request.profile.vertices.truncate(STRIP.len());
    for (vertex, point) in request.profile.vertices.iter_mut().zip(STRIP) {
        vertex.point = point;
    }
    request.profile.curves = (0..STRIP.len())
        .map(|index| ProfileCurve::Line {
            id: ProfileCurveId(index as u64 + 1),
            start: ProfileVertexId(index as u64 + 1),
            end: ProfileVertexId(((index + 1) % STRIP.len()) as u64 + 1),
        })
        .collect();
    request
}

fn prepare_strip(
    run: &ConeRun<'_>,
    original: &DevelopedRegionRequest,
    work: &mut Work,
) -> Prepared {
    let chart = super::super::super::AdmittedConeChart {
        wedge: ConicalSectorWedge::PositiveX,
        center: run.support.chart_origin,
        proof: None,
    };
    let execution = WrappedConeExecutionSelection::RoundedProxyNativeSearchV1 {
        domain: WrappedConeDomainPolicy::PositiveXRecenteredChartV4,
    };
    let mut prepared = super::super::super::initial(run, original, &chart, execution).unwrap();
    let mut endpoints = endpoints::EndpointPool::default();
    let mut pieces = Vec::new();
    for edge in &original.loops[0] {
        super::super::super::radius(&mut prepared, edge, work, &mut || false).unwrap();
        let parts = fit_edge(&mut prepared, edge, &mut endpoints, work);
        pieces.extend(parts);
    }
    prepared.source.loops = vec![pieces];
    prepared
}

fn fit_edge(
    prepared: &mut Prepared,
    edge: &AuthoredPiece,
    endpoints: &mut endpoints::EndpointPool,
    work: &mut Work,
) -> Vec<AuthoredPiece> {
    let query = ConicalChartPlanQuery {
        authored: &edge.poles,
        semi_angle: prepared.semi_angle,
        chart_origin: prepared.chart_origin,
        max_level: prepared.source.max_fit_level,
        max_charts: MAX_CERTIFIED_SPANS as usize,
        options: work.remaining(),
    };
    let chart_plan = plan_conical_subspan_charts(query, || false).unwrap();
    work.charge(
        prepared.source.region,
        Some(edge.span),
        chart_plan.operations(),
    )
    .unwrap();
    let mut fitted = Vec::new();
    for proof in chart_plan.into_charts() {
        fitted.extend(plan(prepared, edge, Some(proof), endpoints, work, &mut || false).unwrap());
    }
    fitted
}

fn assert_closed_wide_uv(prepared: &Prepared) {
    let contour = &prepared.source.loops[0];
    assert!(contour.len() > STRIP.len());
    let mut phase = [f64::INFINITY, f64::NEG_INFINITY];
    for (piece, next) in contour.iter().zip(contour.iter().cycle().skip(1)) {
        let fit = &prepared.fits[&piece.span];
        let successor = &prepared.fits[&next.span];
        assert_eq!(
            fit.uv[3].map(f64::to_bits),
            successor.uv[0].map(f64::to_bits)
        );
        for [u, _] in &fit.uv {
            phase = [phase[0].min(*u), phase[1].max(*u)];
        }
    }
    assert!(phase[1] - phase[0] > std::f64::consts::TAU);
}

fn assert_periodic_cap_material(
    prepared: &Prepared,
    request: &WrappedConeNormalRequest,
    spans: &[storage::SpanProof],
) {
    let uv = periodic_uv_request(prepared, spans);
    let profiles = std::slice::from_ref(&uv);
    let planar = planar_cap_material(profiles, request);
    let boundary_clearance =
        planar_clearance::certify_many_with_cancel(profiles, profiles, || false).unwrap();
    let prepared = periodic_selection::PreparedSelection {
        planar,
        prepared: vec![uv.clone()],
        boundary_clearance,
    };
    let material = periodic_selection::material::certify_with_cancel(
        &[uv],
        &prepared,
        periodic_selection::MaterialOptions::default(),
        || false,
    )
    .unwrap();
    let images = prepared.boundary_clearance.periodic.images();
    assert_eq!(images.len(), 1);
    assert_eq!(images[0].translation().turn(), 1);
    assert_eq!(material.relations.len(), 2);
}

fn planar_cap_material(
    profiles: &[Request],
    request: &WrappedConeNormalRequest,
) -> periodic_selection::PlanarSelectionEvidence {
    let version = PlanarProofVersion::GeometricCompositeV6 {
        max_components: 64,
        max_total_subintervals: 100_000,
    };
    let cap = &profiles[0];
    let set = planar::capture_with_proof_and_cancel(
        profiles,
        cap.tolerance,
        request.options.planar_work,
        version,
        || false,
    )
    .unwrap();
    periodic_selection::PlanarSelectionEvidence {
        members: [(cap.region, planar::select(cap, &set).unwrap())].into(),
        certificate: set.certificate(),
        proof_work: set.proof_work(),
    }
}

fn periodic_uv_request(prepared: &Prepared, spans: &[storage::SpanProof]) -> Request {
    let loops = vec![
        spans
            .iter()
            .enumerate()
            .map(|(i, span)| {
                assert_eq!(span.pcurves[0], span.pcurves[1]);
                periodic_uv_piece(i, spans.len(), &span.pcurves[0])
            })
            .collect(),
    ];
    Request {
        source: DevelopedRegionRequest {
            region: prepared.source.region,
            loops,
            depths: prepared.source.depths,
            tolerance: prepared.source.tolerance,
            max_fit_level: prepared.source.max_fit_level,
        },
        radius: 1.,
    }
}

fn periodic_uv_piece(index: usize, count: usize, uv: &[[f64; 2]]) -> AuthoredPiece {
    AuthoredPiece {
        span: CurveSpan {
            curve: SurfaceCurveId(index as u64 + 1),
            index: 0,
            level: 0,
        },
        start: CornerSource::Authored(SurfaceVertexId(index as u64 + 1)),
        end: CornerSource::Authored(SurfaceVertexId(((index + 1) % count) as u64 + 1)),
        // Keep angle on the periodic axis and reflect slant to preserve winding.
        poles: uv.iter().map(|&[u, v]| [u, -v]).collect(),
        chart_center: 0.,
    }
}
