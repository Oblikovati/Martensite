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
    let body = native::construct(&prepared, request.support).unwrap();
    let resolution =
        crate::surface_sketch::wrapped_profile::pipeline::resolution(original.tolerance);
    let report = validate_body(&body.solid, &resolution).unwrap();
    assert!(report.is_valid(), "{report:?}");
    assert_periodic_cap_material(&prepared, &request);
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
        prepared.chart_center = proof.fit_center();
        prepared.chart_domain = Some(std::sync::Arc::new(proof));
        fitted.extend(plan(prepared, edge, endpoints, work, &mut || false).unwrap());
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

fn assert_periodic_cap_material(prepared: &Prepared, request: &WrappedConeNormalRequest) {
    let uv = periodic_uv_request(prepared);
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

fn periodic_uv_request(prepared: &Prepared) -> Request {
    let contour = &prepared.source.loops[0];
    let loops = vec![
        contour
            .iter()
            .enumerate()
            .map(|(i, piece)| periodic_uv_piece(i, contour.len(), &prepared.fits[&piece.span].uv))
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

fn periodic_uv_piece(index: usize, count: usize, uv: &[[f64; 2]; 4]) -> AuthoredPiece {
    AuthoredPiece {
        span: CurveSpan {
            curve: SurfaceCurveId(index as u64 + 1),
            index: 0,
            level: 0,
        },
        start: CornerSource::Authored(SurfaceVertexId(index as u64 + 1)),
        end: CornerSource::Authored(SurfaceVertexId(((index + 1) % count) as u64 + 1)),
        // The native builder checked these exact controls on both caps. Keep angle
        // on the periodic axis and reflect slant to preserve the original winding.
        poles: uv.iter().map(|&[u, v]| [u, -v]).collect(),
        chart_center: 0.,
    }
}
