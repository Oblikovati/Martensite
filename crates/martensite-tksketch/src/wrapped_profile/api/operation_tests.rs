//! Call the complete entry point with owned profiles and independent expected measures.
use super::*;
use crate::surface_sketch::{CylinderSupport, HistoryRole, NormalSense};
use martensite_tkbo::profile_extrusion::{
    LoopUse, ProfileCurve, ProfileCurveId, ProfileLoop, ProfilePlane, ProfileVertex,
    ProfileVertexId, SketchProfile,
};
use std::collections::BTreeMap;

pub(super) fn request() -> WrappedNormalRequest {
    let inputs = family::tests::requests();
    WrappedNormalRequest {
        profile: combined_profile(&inputs),
        selections: inputs.iter().map(selection).collect(),
        support: support(),
        depths: [-1., 2.],
        options: WrappedNormalOptions {
            planar_work: martensite_tkbo::profile_extrusion::PlanarProofWorkLimit::CurrentDefault,
            spatial_error: 1e-5,
            maximum_fit_level: 8,
            volume: WrappedVolumePolicy::Refuse(1e-4),
        },
    }
}
fn support() -> CylinderSupport {
    CylinderSupport {
        origin: [0.; 3],
        axis: [0., 0., 1.],
        radial: [1., 0., 0.],
        radius: 10.,
        chart_origin: 0.,
        normal: NormalSense::Outward,
    }
}

fn combined_profile(requests: &[Request]) -> SketchProfile {
    SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices: profile_vertices(requests),
        curves: requests
            .iter()
            .flat_map(|r| &r.loops)
            .flatten()
            .map(curve)
            .collect(),
    }
}
fn profile_vertices(requests: &[Request]) -> Vec<ProfileVertex> {
    let vertices: BTreeMap<_, _> = requests
        .iter()
        .flat_map(|r| &r.loops)
        .flatten()
        .map(|p| (vertex(p.start), p.poles[0]))
        .collect();
    vertices
        .into_iter()
        .map(|(id, point)| ProfileVertex { id, point })
        .collect()
}

fn vertex(source: CornerSource) -> ProfileVertexId {
    match source {
        CornerSource::Authored(id) => ProfileVertexId(id.0),
        other => panic!("fixture source={other:?}; expected an authored vertex"),
    }
}
fn curve(piece: &AuthoredPiece) -> ProfileCurve {
    let id = ProfileCurveId(piece.span.curve.0);
    let (start, end) = (vertex(piece.start), vertex(piece.end));
    if piece.poles.len() == 2 {
        return ProfileCurve::Line { id, start, end };
    }
    let count = piece.poles.len();
    ProfileCurve::Spline {
        id,
        start,
        end,
        degree: (count - 1) as u32,
        poles: piece.poles.clone(),
        weights: None,
        knots: std::iter::repeat_n(0., count)
            .chain(std::iter::repeat_n(1., count))
            .collect(),
    }
}
fn contour(pieces: &[AuthoredPiece]) -> ProfileLoop {
    ProfileLoop {
        uses: pieces
            .iter()
            .map(|p| LoopUse {
                curve: ProfileCurveId(p.span.curve.0),
                reversed: false,
            })
            .collect(),
    }
}
fn selection(request: &Request) -> WrappedRegionSelection {
    WrappedRegionSelection {
        region: request.region,
        outer: contour(&request.loops[0]),
        holes: request.loops[1..].iter().map(|c| contour(c)).collect(),
    }
}

#[test]
fn entry_point_returns_immutable_mixed_profiles_with_holes_and_disconnected_material() {
    let input = request();
    let result = normal_extrude_profile(&input).unwrap_or_else(|cause| panic!("{cause}"));
    assert_eq!(result.reports().len(), 2);
    check_report(&result, SurfaceRegionId(5), 11.10375);
    check_report(&result, SurfaceRegionId(100), 3.15);
    assert_eq!(result.depths(), input.depths);
    assert_eq!(result.support(), input.support);
    assert_eq!(result.options(), input.options);
    assert!(result.budget().volume.discrepancy.upper <= 1e-4);
    assert!(
        result
            .history()
            .iter()
            .any(|r| r.relation == WrappedHistoryRelation::Created)
    );
    let detached = result.export_native().unwrap();
    assert!(!detached.bodies.is_null());
    assert_eq!(detached.entities.len(), result.history().len());
    fn send_sync<T: Send + Sync>() {}
    send_sync::<WrappedNormalRequest>();
    send_sync::<WrappedNormalResult>();
    send_sync::<WrappedNormalFailure>();
}
fn check_report(result: &WrappedNormalResult, region: SurfaceRegionId, expected: f64) {
    let report = &result.reports()[&region];
    assert!(report.validation.is_valid());
    assert!(!report.validation.is_exact());
    assert!(
        report.measures.authored_volume.lower <= expected
            && expected <= report.measures.authored_volume.upper
    );
    assert!(report.measures.ideal_volume.lower <= report.measures.ideal_volume.upper);
    assert!(report.measures.retained_ideal_discrepancy.upper <= 1e-4);
    assert!(report.spatial.profile_error.upper <= result.budget().profile_allowance);
    assert!(report.spatial.normal_storage_error.upper <= result.budget().storage_allowance);
    assert!(report.spatial.cap_trace_error.upper <= result.budget().trace_allowance);
    assert!(report.spatial.native_tolerance <= result.budget().trace_allowance);
    assert!(
        report.measures.native_volume.value > 0.
            && report.measures.native_volume.integration_error >= 0.
    );
    assert!(report.measures.boundary_flux.lower <= report.measures.boundary_flux.upper);
}

#[test]
fn inward_depths_preserve_requested_roles_and_exact_physical_volume() {
    let mut input = request();
    input.support.normal = NormalSense::Inward;
    input.depths = [-2., 1.];
    let inward = normal_extrude_profile(&input).unwrap_or_else(|cause| panic!("{cause}"));
    input.support.normal = NormalSense::Outward;
    input.depths = [-1., 2.];
    let outward = normal_extrude_profile(&input).unwrap_or_else(|cause| panic!("{cause}"));
    assert_eq!(inward.encoded_brep(), outward.encoded_brep());
    assert_eq!(inward.depths(), [-2., 1.]);
    assert_opposite_faces(&inward, &outward);
    check_report(&inward, SurfaceRegionId(5), 11.10375);
}

fn assert_opposite_faces(inward: &WrappedNormalResult, outward: &WrappedNormalResult) {
    let key = WrappedHistoryKey {
        source: WrappedEntitySource::Region(SurfaceRegionId(5)),
        role: HistoryRole::FirstDepthFace,
    };
    let a = inward.export_native().unwrap();
    let b = outward.export_native().unwrap();
    let opposite = WrappedHistoryKey {
        role: HistoryRole::LastDepthFace,
        ..key
    };
    assert_eq!(
        face_bytes(&a.entities[&key]),
        face_bytes(&b.entities[&opposite])
    );
}

fn face_bytes(shape: &Shape) -> Vec<u8> {
    let bindings: Vec<(u8, Shape)> = Vec::new();
    crate::surface_sketch::encoding::encode(shape, &bindings).unwrap()
}

#[test]
fn unsupported_placement_retains_operation_stage_and_owned_cause() {
    let mut input = request();
    input.support.chart_origin = 1.;
    let error = normal_extrude_profile(&input).err().unwrap();
    assert_eq!(error.operation(), WrappedOperation::NormalExtrusion);
    assert_eq!(error.stage(), WrappedStage::Admission);
    assert_eq!(error.class(), WrappedFailureClass::Unsupported);
    assert!(std::error::Error::source(&error).is_some());
}
#[test]
fn missing_curve_retains_region_curve_and_owned_diagnostic() {
    let mut input = request();
    input.selections[0].outer.uses[0].curve = ProfileCurveId(u64::MAX);
    let error = normal_extrude_profile(&input).err().unwrap();
    assert_eq!(error.stage(), WrappedStage::Input);
    assert_eq!(
        error.entities(),
        &[
            crate::surface_sketch::HistorySource::Region(SurfaceRegionId(5)),
            crate::surface_sketch::HistorySource::Curve(SurfaceCurveId(u64::MAX)),
        ]
    );
    assert!(error.to_string().contains("expected"));
}

#[test]
fn cancellation_before_admission_never_returns_a_result() {
    let error = normal_extrude_profile_with_cancel(&request(), || true)
        .err()
        .unwrap();
    assert_eq!(error.class(), WrappedFailureClass::Cancelled);
    assert_eq!(error.stage(), WrappedStage::Admission);
}

#[test]
fn a_nonzero_angle_shared_corner_stays_closed_without_fixed_interpolation() {
    let result = normal_extrude_profile(&shifted_request()).unwrap();
    assert_shared_corner(&result.export_native().unwrap());
    let spatial = &result.reports()[&SurfaceRegionId(5)].spatial;
    assert!(spatial.profile_error.upper > 0.);
    assert!(result.budget().cap_homotopy_clearance < result.budget().required_clearance);
}
fn assert_shared_corner(native: &DetachedWrappedBodies) {
    let key = WrappedHistoryKey {
        source: WrappedEntitySource::Vertex(SurfaceVertexId(10)),
        role: HistoryRole::FirstDepthVertex,
    };
    let corner = martensite_tkbrep::topods::vertex(&native.entities[&key]).unwrap();
    let incident = shared_corner_uses(native, &corner);
    assert_eq!(incident.len(), 2);
    assert!(
        incident
            .iter()
            .all(|v| vertex_bits(v) == vertex_bits(&corner))
    );
    assert_eq!(vertex_bits(&corner), expected_corner_bits());
}
fn expected_corner_bits() -> [u64; 3] {
    [
        9. * martensite_cmath::cos(0.3),
        9. * martensite_cmath::sin(0.3),
        0.,
    ]
    .map(f64::to_bits)
}

fn shifted_request() -> WrappedNormalRequest {
    let mut input = request();
    for vertex in &mut input.profile.vertices {
        vertex.point[0] += 3.;
    }
    for curve in &mut input.profile.curves {
        shift_spline(curve);
    }
    input
}
fn shift_spline(curve: &mut ProfileCurve) {
    if let ProfileCurve::Spline { poles, .. } = curve {
        for point in poles {
            point[0] += 3.;
        }
    }
}
fn shared_corner_uses(
    native: &DetachedWrappedBodies,
    corner: &martensite_tkbrep::topods::Vertex,
) -> Vec<martensite_tkbrep::topods::Vertex> {
    native
        .entities
        .iter()
        .filter(|(key, _)| key.role == HistoryRole::FirstDepthEdge)
        .flat_map(|(_, shape)| {
            let edge = martensite_tkbrep::topods::edge(shape).unwrap();
            let (first, last) = martensite_tkbrep::topexp::vertices(&edge, false);
            [first, last]
        })
        .filter(|vertex| vertex.is_same(corner))
        .collect()
}
fn vertex_bits(vertex: &martensite_tkbrep::topods::Vertex) -> [u64; 3] {
    let point = martensite_tkbrep::brep::BRepTool::pnt(vertex).unwrap();
    [point.x(), point.y(), point.z()].map(f64::to_bits)
}

#[test]
fn explicit_planar_work_is_retained_and_replays_without_changing_geometry() {
    let mut input = request();
    input.options.planar_work =
        martensite_tkbo::profile_extrusion::PlanarProofWorkLimit::PairBisections(8192);
    let first = normal_extrude_profile(&input).unwrap();
    let replay = normal_extrude_profile(&input).unwrap();
    assert_eq!(first.options(), input.options);
    assert_eq!(first.budget(), replay.budget());
    assert_eq!(first.encoded_brep(), replay.encoded_brep());
    assert_eq!(first.budget().planar_work.requested_pair_bisections, 8192);
    assert_eq!(first.budget().planar_work.effective_pair_bisections, 8192);
    assert!(first.budget().planar_work.consumed_pair_bisections <= 8192);
    check_report(&first, SurfaceRegionId(5), 11.10375);
    check_report(&first, SurfaceRegionId(100), 3.15);
}
