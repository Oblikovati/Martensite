//! Author a solved loop from native UV, then publish a placed normal tool.
use super::*;
use crate::surface_sketch::{
    HistoryRole, SurfaceRegionId, WrappedConeArithmeticGrantPolicy, WrappedConeExecutionSelection,
    WrappedConeNormalRequest, WrappedConeProofOptions, WrappedEntitySource, WrappedNormalOptions,
    WrappedPlacedConeResult, WrappedRegionSelection, WrappedVolumePolicy,
    normal_extrude_cone_profile_on_chart,
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
    let selection = WrappedConeExecutionSelection::DirectedRecenteredChartRequireCertifiedV3 {
        arithmetic: WrappedConeArithmeticGrantPolicy::OriginalV1,
    };
    let result = normal_extrude_cone_profile_on_chart(&proposed, selection)
        .unwrap_or_else(|cause| panic!("native-hit extrusion refused: {cause:?}"));
    let support = attachment.export_native(REVISION).unwrap();
    assert!(
        result.placement().members[&SurfaceRegionId(19)]
            .validation
            .is_valid()
    );
    verify_normals(&result, &support.face, proposed.options.spatial_error);
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
    let mut vertices = Vec::new();
    let mut curves = Vec::new();
    for (index, uv) in UV.into_iter().enumerate() {
        let hit = chart
            .develop_parameters(REVISION, request(uv, OutOfSupportPolicy::Refuse))
            .unwrap();
        let id = index as u64 + 1;
        vertices.push(ProfileVertex {
            id: ProfileVertexId(id),
            point: hit.coordinates.map(|p| p.value),
        });
        curves.push(ProfileCurve::Line {
            id: ProfileCurveId(id),
            start: ProfileVertexId(id),
            end: ProfileVertexId(id % 4 + 1),
        });
    }
    SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices,
        curves,
    }
}

fn selection() -> WrappedRegionSelection {
    WrappedRegionSelection {
        region: SurfaceRegionId(19),
        outer: ProfileLoop {
            uses: (1..=4)
                .map(|id| LoopUse {
                    curve: ProfileCurveId(id),
                    reversed: false,
                })
                .collect(),
        },
        holes: Vec::new(),
    }
}

fn verify_normals(result: &WrappedPlacedConeResult, face: &Face, tolerance: f64) {
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
        let expected = native_normal_offset(&native, UV[(id.0 - 1) as usize], depth);
        let actual = BRepTool::pnt(&topods::vertex(&vertex).unwrap()).unwrap();
        for (actual, expected) in actual.coordinates().iter().zip(expected) {
            assert!((*actual - expected).abs() <= tolerance);
        }
        checked += 1;
    }
    assert_eq!(checked, 8);
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
