//! Author a solved loop from native UV, then publish a placed normal tool.
use super::*;
use crate::surface_sketch::{
    ConeFootprintFailure, ConeTrimmedTool, HistoryRole, SurfaceRegionId,
    WrappedConeArithmeticGrantPolicy, WrappedConeExecutionSelection, WrappedConeNormalRequest,
    WrappedConeProofOptions, WrappedEntitySource, WrappedNormalOptions, WrappedPlacedConeResult,
    WrappedRegionSelection, WrappedVolumePolicy, normal_extrude_cone_profile_on_chart,
};
use martensite_tkbo::profile_extrusion::{
    LoopUse, PlanarProofVersion, PlanarProofWorkLimit, ProfileCurve, ProfileCurveId, ProfileLoop,
    ProfilePlane, ProfileVertex, ProfileVertexId, SketchProfile,
};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkmath::gp::{Axis2, Direction3, Point3};

const UV: [[f64; 2]; 4] = [[0.5, 3.], [0.5, 4.], [0.9, 4.], [0.9, 3.]];
const DEPTHS: [f64; 2] = [-0.1, 0.2];

#[test]
fn native_cone_chart_can_author_a_physical_normal_extrusion() {
    verify_workflow(&attach([2., 4.]));
}

#[test]
fn oblique_selected_cone_authors_a_tool_in_world_coordinates() {
    let origin = Point3::new(3., -5., 7.);
    let axis = Direction3::new(0., 3., 4.).unwrap();
    let radial = Direction3::new(1., 0., 0.).unwrap();
    let frame = Axis2::with_x_direction(origin, axis, radial).unwrap();
    let mut maker = BRepPrimAPIMakeCone::with_axes_angle(frame, 2., 4., 10., 2.).unwrap();
    let body = maker.shape().unwrap();
    let face = select_kind(body, SurfaceType::Cone);
    let resolution = super::super::super::tests::resolution();
    let attachment = ConeFaceAttachment::capture(ID, REVISION, body, &face, resolution).unwrap();
    verify_workflow(&attachment);
}

fn verify_workflow(attachment: &ConeFaceAttachment) {
    let chart = attachment.authoring_chart(REVISION, 0.7).unwrap();
    let proposed = proposal(&chart);
    let tool = chart
        .extrude_within_trim_band(REVISION, &proposed, directed())
        .unwrap_or_else(|cause| panic!("native-hit extrusion refused: {cause:?}"));
    let result = tool.extrusion();
    let support = attachment.export_native(REVISION).unwrap();
    assert!(
        result.placement().members[&SurfaceRegionId(19)]
            .validation
            .is_valid()
    );
    verify_normals(result, &support.face, proposed.options.spatial_error);
    verify_footprint(&tool);
}

fn directed() -> WrappedConeExecutionSelection {
    WrappedConeExecutionSelection::DirectedRecenteredChartRequireCertifiedV3 {
        arithmetic: WrappedConeArithmeticGrantPolicy::OriginalV1,
    }
}

fn verify_footprint(tool: &ConeTrimmedTool) {
    fn shared<T: Send + Sync>() {}
    shared::<ConeTrimmedTool>();
    let report = tool.footprint();
    assert_eq!(report.attachment, ID);
    assert_eq!(report.revision, REVISION);
    assert_eq!(report.native_angular, Some([0., 2.]));
    let footprint = &report.regions[&SurfaceRegionId(19)];
    let coverage = &tool.extrusion().canonical().reports()[&SurfaceRegionId(19)].coverage;
    assert_eq!(coverage.cap_pcurves, 2 * footprint.fitted_spans);
    assert!(footprint.authored.slant[0] > report.limits.slant[0]);
    assert!(
        footprint
            .caps
            .iter()
            .all(|cap| cap.slant[1] < report.limits.slant[1])
    );
}

#[test]
fn trimmed_cone_can_reject_geometry_admitted_by_standalone_extrusion() {
    let attachment = attach([2., 4.]);
    let chart = attachment.authoring_chart(REVISION, 0.7).unwrap();
    let mut proposed = proposal(&chart);
    for vertex in &mut proposed.profile.vertices {
        vertex.point[1] -= 3.0;
    }
    assert!(normal_extrude_cone_profile_on_chart(&proposed, directed()).is_ok());
    // A zero arithmetic grant would refuse fitting if the footprint check ran too late.
    proposed.proof.arithmetic.max_operations = 0;
    let result = chart.extrude_within_trim_band(REVISION, &proposed, directed());
    assert!(matches!(
        result,
        Err(ConeFootprintFailure::Metric {
            region: Some(SurfaceRegionId(19)),
            depth_row: None,
            ..
        })
    ));
}

#[test]
fn inward_cone_tool_crosses_a_full_bands_native_seam() {
    let mut maker = BRepPrimAPIMakeCone::new(2., 4., 10.).unwrap();
    let body = maker.shape().unwrap();
    let face = select_kind(body, SurfaceType::Cone);
    let resolution = super::super::super::tests::resolution();
    let attachment = ConeFaceAttachment::capture(ID, REVISION, body, &face, resolution).unwrap();
    let chart = attachment.authoring_chart(REVISION, 0.).unwrap();
    let mut proposed = proposal(&chart);
    proposed.support.normal = NormalSense::Inward;
    let crossing = [[-0.12, 3.], [-0.12, 4.], [0.12, 4.], [0.12, 3.]];
    proposed.profile = profile_at(&chart, crossing, OutOfSupportPolicy::Report);
    let tool = chart
        .extrude_within_trim_band(REVISION, &proposed, directed())
        .unwrap();
    let footprint = tool.footprint();
    assert_eq!(footprint.native_angular, None);
    assert_eq!(
        footprint.regions[&SurfaceRegionId(19)].authored.angular,
        None
    );
    assert_eq!(tool.extrusion().support().normal, NormalSense::Inward);
}

#[test]
fn trim_qualified_cone_preserves_a_selected_hole() {
    let attachment = attach([4., 2.]);
    let chart = attachment.authoring_chart(REVISION, 0.7).unwrap();
    let mut authored = proposal(&chart);
    let hole = [[0.6, 3.25], [0.8, 3.25], [0.8, 3.75], [0.6, 3.75]];
    append_loop(
        &chart,
        hole,
        OutOfSupportPolicy::Refuse,
        5,
        &mut authored.profile,
    );
    authored.selections[0].holes.push(contour(5));
    let tool = chart
        .extrude_within_trim_band(REVISION, &authored, directed())
        .unwrap();
    let result = &tool.extrusion().canonical().reports()[&SurfaceRegionId(19)];
    assert_eq!(result.coverage.euler_characteristic, 0);
    verify_footprint(&tool);
    let support = attachment.export_native(REVISION).unwrap();
    let hits: Vec<_> = UV.into_iter().chain(hole).collect();
    verify_depth_vertices(
        tool.extrusion(),
        &support.face,
        authored.options.spatial_error,
        &hits,
    );
}

fn proposal(chart: &ConeAuthoringChart) -> WrappedConeNormalRequest {
    let support = chart
        .extrusion_support(REVISION, NormalSense::Outward)
        .unwrap();
    WrappedConeNormalRequest {
        support,
        profile: profile(chart),
        selections: vec![selection()],
        depths: DEPTHS,
        options: WrappedNormalOptions {
            spatial_error: 1e-4,
            maximum_fit_level: 8,
            planar_work: PlanarProofWorkLimit::CurrentDefault,
            volume: WrappedVolumePolicy::Report,
        },
        proof: WrappedConeProofOptions {
            planar_proof: PlanarProofVersion::GeometricCompositeV6 {
                max_components: 64,
                max_total_subintervals: 100_000,
            },
            ..WrappedConeProofOptions::default()
        },
    }
}

fn profile(chart: &ConeAuthoringChart) -> SketchProfile {
    profile_at(chart, UV, OutOfSupportPolicy::Refuse)
}

fn profile_at(
    chart: &ConeAuthoringChart,
    hits: [[f64; 2]; 4],
    policy: OutOfSupportPolicy,
) -> SketchProfile {
    let mut profile = SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices: Vec::new(),
        curves: Vec::new(),
    };
    append_loop(chart, hits, policy, 1, &mut profile);
    profile
}

fn append_loop(
    chart: &ConeAuthoringChart,
    hits: [[f64; 2]; 4],
    policy: OutOfSupportPolicy,
    first: u64,
    profile: &mut SketchProfile,
) {
    for (index, uv) in hits.into_iter().enumerate() {
        let hit = chart
            .develop_parameters(REVISION, request(uv, policy))
            .unwrap();
        let id = first + index as u64;
        profile.vertices.push(ProfileVertex {
            id: ProfileVertexId(id),
            point: hit.coordinates.map(|p| p.value),
        });
        profile.curves.push(ProfileCurve::Line {
            id: ProfileCurveId(id),
            start: ProfileVertexId(id),
            end: ProfileVertexId(first + ((index + 1) % 4) as u64),
        });
    }
}

fn selection() -> WrappedRegionSelection {
    WrappedRegionSelection {
        region: SurfaceRegionId(19),
        outer: contour(1),
        holes: Vec::new(),
    }
}

fn contour(first: u64) -> ProfileLoop {
    ProfileLoop {
        uses: (first..first + 4)
            .map(|id| LoopUse {
                curve: ProfileCurveId(id),
                reversed: false,
            })
            .collect(),
    }
}

fn verify_normals(result: &WrappedPlacedConeResult, face: &Face, tolerance: f64) {
    verify_depth_vertices(result, face, tolerance, &UV);
}

fn verify_depth_vertices(
    result: &WrappedPlacedConeResult,
    face: &Face,
    tolerance: f64,
    hits: &[[f64; 2]],
) {
    let native = BRepAdaptorSurface::with_face(face, false).unwrap();
    let exported = result.export_native().unwrap();
    let mut checked = 0;
    for (key, vertex) in exported.entities {
        let WrappedEntitySource::Vertex(id) = key.source else {
            continue;
        };
        let depth = match key.role {
            HistoryRole::FirstDepthVertex => DEPTHS[0],
            HistoryRole::LastDepthVertex => DEPTHS[1],
            _ => continue,
        };
        let expected = native_normal_offset(&native, hits[(id.0 - 1) as usize], depth);
        let actual = BRepTool::pnt(&topods::vertex(&vertex).unwrap()).unwrap();
        for (actual, expected) in actual.coordinates().iter().zip(expected) {
            assert!((*actual - expected).abs() <= tolerance);
        }
        checked += 1;
    }
    assert_eq!(checked, 2 * hits.len());
}

fn native_normal_offset(native: &BRepAdaptorSurface, uv: [f64; 2], depth: f64) -> [f64; 3] {
    let native = native.eval_d1(uv[0], uv[1]).unwrap();
    let normal = native
        .d1u
        .xyz()
        .crossed(*native.d1v.xyz())
        .normalized()
        .unwrap();
    let normal = [normal.x(), normal.y(), normal.z()];
    std::array::from_fn(|i| native.point.coordinates()[i] + depth * normal[i])
}
