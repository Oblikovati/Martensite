//! The automatic plan is consumed by the same fitter and native cap constructor.
use super::*;
use crate::surface_sketch::wrapped_profile::{planar_clearance, selection as periodic_selection};
use martensite_tkbo::{
    body_check::{CheckMode, ValidationStage, validate_body},
    profile_extrusion::{PlanarProofVersion, ProfileCurve, ProfileCurveId, ProfileVertexId},
};

mod directed_rows;
mod family;
mod periodic;
mod placement;

const STRIP: [[f64; 2]; 4] = [
    [10., -5.5],
    [10.625, -5.84375],
    [31.875, 17.53125],
    [30., 16.5],
];

#[test]
fn public_connected_atlas_publishes_strip_wider_than_one_turn() {
    let mut request = strip_request();
    select_uv_program(&mut request);
    let result = execute_atlas(&request).unwrap();
    assert_eq!(
        result.domain_policy(),
        WrappedConeDomainPolicy::PositiveXConnectedAtlasV6
    );
    let report = &result.reports()[&SurfaceRegionId(1)];
    assert!(report.validation.is_valid());
    assert!(report.coverage.faces > 2 + STRIP.len());
    assert_eq!(report.coverage.cap_pcurves, 2 * (report.coverage.faces - 2));
    assert!(result.budget().achieved_spatial_error.upper <= request.options.spatial_error);
    let caps = result.reference_body_receipt().decoded_caps().unwrap();
    assert!(result.reference_body_receipt().has_separated_decoded_fill());
    assert_eq!(
        report
            .validation
            .stage(ValidationStage::SelfIntersection)
            .mode,
        CheckMode::Exact,
    );
    assert_eq!(caps.len(), 1);
    assert_decoded_boundary(&caps[0], request.options.spatial_error);
    assert!(
        result
            .reference_volume_receipt(SurfaceRegionId(1))
            .is_none()
    );
    assert_eq!(
        result.export_native().unwrap().entities.len(),
        result.history().len()
    );
}

#[test]
fn public_connected_atlas_keeps_the_narrow_outlined_profile() {
    let mut request = super::super::super::super::tests::supported_common_chart_request();
    request.proof.arithmetic.max_operations = 1_250_000;
    let started = std::time::Instant::now();
    let result = execute_atlas(&request).unwrap();
    eprintln!(
        r#"{{"event":"connected_atlas_api","seconds":{}}}"#,
        started.elapsed().as_secs_f64()
    );
    assert_eq!(result.reports().len(), request.selections.len());
    assert_eq!(
        result
            .actual_self_intersection_coverage()
            .unwrap()
            .side_self_domains,
        132
    );
    for report in result.reports().values() {
        assert!(report.validation.is_valid());
        assert_eq!(
            report
                .validation
                .stage(ValidationStage::SelfIntersection)
                .mode,
            CheckMode::Exact,
        );
    }
    assert!(result.budget().achieved_spatial_error.upper <= request.options.spatial_error);
    assert!(
        result
            .reference_body_receipt()
            .decoded_solid_separation()
            .is_some()
    );
    assert!(result.reference_body_receipt().has_separated_decoded_fill());
}

fn select_uv_program(request: &mut WrappedConeNormalRequest) {
    request.proof.planar_proof = PlanarProofVersion::GeometricCompositeV6 {
        max_components: 64,
        max_total_subintervals: 100_000,
    };
}

fn execute_atlas(request: &WrappedConeNormalRequest) -> Checked<WrappedConeNormalResult> {
    crate::surface_sketch::normal_extrude_cone_profile_with_selection(request, atlas_selection())
}

fn atlas_selection() -> WrappedConeExecutionSelection {
    WrappedConeExecutionSelection::DirectedConnectedChartAtlasV7 {
        arithmetic: WrappedConeArithmeticGrantPolicy::PhysicalAngleBoundV2,
    }
}

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
    let prepared = prepare_local(&run, original, None, &mut work);
    assert_closed_wide_uv(&prepared);
    let clearance = super::super::super::clearance::certify(
        &inputs.originals,
        std::slice::from_ref(&prepared),
        &mut work,
        &mut || false,
    )
    .unwrap();
    assert!(clearance.operations() > 0);
    let body = decode_member(&prepared, request.support);
    let resolution =
        crate::surface_sketch::wrapped_profile::pipeline::resolution(original.tolerance);
    let report = validate_body(&body.shape, &resolution).unwrap();
    assert!(report.is_valid(), "{report:?}");
    let mut stored = storage::capture(original, &prepared, body, &mut work, &mut || false).unwrap();
    storage::bind_cap_joins(original, &prepared, &stored).unwrap();
    reject_changed_cap(original, &prepared, &mut stored);
    assert_per_span_authority(original, &prepared, &stored.spans);
    assert_certified_contour(original, &prepared, &stored);
    // Decoded span iteration order is not curve identity.
    stored.spans.reverse();
    let error = storage::bind_cap_joins(original, &prepared, &stored).unwrap_err();
    assert_eq!(error.region, Some(original.region));
    let cap = assert_periodic_cap_material(&prepared, &request, &stored, &mut work);
    stored.spans.reverse();
    let homotopy = assert_homotopy(&stored.joins, &cap, &clearance);
    assert_closed_strip_measures(&run, original, &stored, &homotopy, &mut work);
}

#[test]
fn local_chart_level_refusal_keeps_its_source_and_typed_cause() {
    let request = strip_request();
    let run = ConeRun::from(&request);
    let mut inputs = input::capture(&request, &mut || false).unwrap();
    let original = &mut inputs.originals[0];
    original.max_fit_level = 0;
    let mut work = Work::new(request.proof.arithmetic);
    let Err(error) = plan_local_region(&run, original, None, &mut work, &mut || false) else {
        panic!("the wide strip needs subdivided charts");
    };
    assert_eq!(error.region, Some(original.region));
    assert!(
        original.loops[0]
            .iter()
            .any(|piece| Some(piece.span) == error.span)
    );
    let ConeCause::ChartPlan(cause) = &error.cause else {
        panic!("expected the local chart planning refusal: {error}");
    };
    assert!(matches!(
        cause.cause,
        ConicalChartPlanFailure::Level { index: 0, level: 0 }
    ));
    assert!(cause.operations > 0 && work.consumed >= cause.operations);
}

fn decode_member(prepared: &Prepared, support: ConeSupport) -> coverage::MemberView {
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
    let spans =
        storage::cap_join_spans(original, prepared, &stored.spans, stored.carriers).unwrap();
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

fn reject_changed_cap(
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    stored: &mut storage::Stored,
) {
    let bound = stored.spans[0].pcurves[1][1][0];
    stored.spans[0].pcurves[1][1][0] = bound.next_up();
    let error = storage::bind_cap_joins(original, prepared, stored).unwrap_err();
    assert_eq!(error.region, Some(original.region));
    assert_eq!(error.span, Some(stored.spans[0].span));
    assert!(matches!(
        error.cause,
        ConeCause::Binding {
            field: "exact decoded cap polynomials and carrier binding",
            ..
        }
    ));
    stored.spans[0].pcurves[1][1][0] = bound;
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
    // The long edges need 256 Simpson panels to meet the original enclosure width.
    request.proof.authored_volume.subdivision_level = 8;
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

fn prepare_local(
    run: &ConeRun<'_>,
    original: &DevelopedRegionRequest,
    shared_slant: Option<f64>,
    work: &mut Work,
) -> Prepared {
    plan_local_region(run, original, shared_slant, work, &mut || false).unwrap()
}

fn plan_local_region(
    run: &ConeRun<'_>,
    original: &DevelopedRegionRequest,
    shared_slant: Option<f64>,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Prepared> {
    let chart = super::super::super::AdmittedConeChart {
        wedge: ConicalSectorWedge::PositiveX,
        center: run.support.chart_origin,
        proof: None,
        local_reference_slant: Some(match shared_slant {
            Some(slant) => slant,
            None => super::super::super::reference(original)?,
        }),
    };
    let execution = WrappedConeExecutionSelection::RoundedProxyNativeSearchV1 {
        domain: WrappedConeDomainPolicy::PositiveXRecenteredChartV4,
    };
    let mut endpoints = endpoints::EndpointPool::default();
    super::super::super::prepare_region(
        run,
        original,
        &chart,
        execution,
        &mut endpoints,
        work,
        cancelled,
    )
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
    stored: &storage::Stored,
    work: &mut Work,
) -> planar_clearance::OriginalClearance {
    let profiles = periodic::project(&[(prepared, &stored.spans)]);
    let planar = periodic::planar(&profiles, request, work);
    let boundary = periodic::clearance(&profiles, work);
    let material = periodic::classify(&profiles, &planar, &boundary, work).unwrap();
    let images = boundary.periodic.images();
    assert_eq!(images.len(), 1);
    assert_eq!(images[0].translation().turn(), 1);
    assert_eq!(material.relations.len(), 2);
    boundary
}

fn assert_closed_strip_measures(
    run: &ConeRun<'_>,
    original: &DevelopedRegionRequest,
    stored: &storage::Stored,
    homotopy: &ConicalCapHomotopyEvidence<'_>,
    work: &mut Work,
) {
    use crate::surface_sketch::wrapped_profile::operation::cone::measures;
    let options = work.remaining();
    let decoded = homotopy
        .certify_decoded_reference(original.tolerance, options, || false)
        .unwrap();
    work.charge(original.region, None, decoded.operations())
        .unwrap();
    assert_decoded_boundary(&decoded, original.tolerance);
    let proof = decoded.normal();
    assert_eq!(proof.requested_spatial_limit(), original.tolerance);
    assert_eq!(proof.options(), options);
    assert!(proof.radius_bounds()[0].compare_f64(0.).unwrap().is_gt());
    assert!(proof.filled_error().compare_f64(0.).unwrap().is_gt());
    assert!(
        proof
            .filled_error()
            .compare_f64(original.tolerance)
            .unwrap()
            .is_le()
    );
    let reference = measures::range(proof.volume().clone()).unwrap();
    assert!(reference.lower > 0.);
    assert!(reference.upper - reference.lower < 1e-10);
    let flux = measures::native_flux(run, original.region, stored, work, &mut || false).unwrap();
    let authored =
        measures::authored_volume(run, original, None, flux.enclosure(), work, &mut || false)
            .unwrap();
    let authored = measures::range(authored.volume()).unwrap();
    assert_volume_agreement([reference.lower, reference.upper], authored, &flux);
}

fn assert_decoded_boundary(proof: &ConicalDecodedCapEvidence, limit: f64) {
    assert!(proof.operations() > proof.normal().operations());
    assert!(proof.composed_error().compare_f64(limit).unwrap().is_le());
    assert!(proof.composed_error() >= proof.normal().filled_error());
    assert!(proof.ruled_side_error().compare_f64(0.).unwrap().is_gt());
    for bound in proof.cap_errors() {
        assert!(bound.compare_f64(1e-12).unwrap().is_lt());
    }
}

fn assert_volume_agreement(
    reference: [f64; 2],
    authored: WrappedCertifiedRange,
    flux: &CertifiedAnalyticConicalFlux,
) {
    use crate::surface_sketch::wrapped_profile::operation::cone::measures::enclosure_distance;
    const MAX_VOLUME_DIFFERENCE: f64 = 5e-5;
    let authored_gap = enclosure_distance(reference, [authored.lower, authored.upper]).unwrap();
    let proxy_gap = enclosure_distance(reference, flux.enclosure()).unwrap();
    assert!(
        authored_gap <= MAX_VOLUME_DIFFERENCE,
        "authored gap={authored_gap}"
    );
    assert!(
        proxy_gap <= MAX_VOLUME_DIFFERENCE,
        "decoded occurrence flux gap={proxy_gap}"
    );
}

fn assert_homotopy<'a>(
    joins: &'a ConicalJoinsEvidence,
    cap_clearance: &planar_clearance::OriginalClearance,
    authored_clearance: &CertifiedCylindricalClearance,
) -> ConicalCapHomotopyEvidence<'a> {
    let homotopy = joins
        .certify_cap_homotopy(&cap_clearance.certificate, &cap_clearance.periodic)
        .unwrap();
    assert!(
        homotopy
            .motion_upper_bound()
            .compare_f64(0.)
            .unwrap()
            .is_gt()
    );
    for gap in homotopy.separation_lower_bounds() {
        assert!(gap.compare_f64(0.).unwrap().is_gt());
    }
    assert!(matches!(
        joins.certify_cap_homotopy(authored_clearance, &cap_clearance.periodic),
        Err(ConicalCapHomotopyFailure::Binding(
            ConicalCapClearanceRelation::Nonadjacent
        ))
    ));
    homotopy
}
