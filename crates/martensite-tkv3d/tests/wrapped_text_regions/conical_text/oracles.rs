//! Analytic endpoint and conservative rational volume oracles are independent of the cone proofs.
use super::*;
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tksketch::surface_sketch::HistoryRole as SurfaceHistoryRole;

pub(super) fn check_volume(result: &WrappedConeNormalResult, normal: NormalSense) {
    let (mut lower, mut upper) = (0_f64, 0_f64);
    for report in result.reports().values() {
        assert!(
            report.authored_volume.lower.is_finite() && report.authored_volume.upper.is_finite()
        );
        lower = (lower + report.authored_volume.lower).next_down();
        upper = (upper + report.authored_volume.upper).next_up();
    }
    let expected = volume_bounds(normal);
    assert!(
        lower >= expected[0] && upper <= expected[1],
        "authored volume=[{lower},{upper}]; expected within {expected:?}"
    );
}

fn volume_bounds(normal: NormalSense) -> [f64; 2] {
    // Area=42000445/6291456, 95<=slant<=110, 127/16<=cot(1/8)<=3072/383.
    match normal {
        NormalSense::Outward => [
            (119340064423_f64 / 70866960384.).next_down(),
            (308862872441_f64 / 183131701248.).next_up(),
        ],
        NormalSense::Inward => [
            (302411604089_f64 / 183131701248.).next_down(),
            (39068813939_f64 / 23622320128.).next_up(),
        ],
    }
}

pub(super) fn check_vertices(request: &WrappedConeNormalRequest, native: &DetachedWrappedBodies) {
    for vertex in &request.profile.vertices {
        for (depth, role) in [
            (request.depths[0], SurfaceHistoryRole::FirstDepthVertex),
            (request.depths[1], SurfaceHistoryRole::LastDepthVertex),
        ] {
            check_vertex(request, native, vertex, depth, role);
        }
    }
}

fn check_vertex(
    request: &WrappedConeNormalRequest,
    native: &DetachedWrappedBodies,
    vertex: &ProfileVertex,
    depth: f64,
    role: SurfaceHistoryRole,
) {
    let key = WrappedHistoryKey {
        source: WrappedEntitySource::Vertex(SurfaceVertexId(vertex.id.0)),
        role,
    };
    let actual = BRepTool::pnt(&topods::vertex(&native.entities[&key]).unwrap()).unwrap();
    let expected = expected_point(vertex.point, depth, request.support.normal);
    let delta: Vec<_> = actual
        .coordinates()
        .iter()
        .zip(expected)
        .map(|(a, e)| a - e)
        .collect();
    let distance = martensite_cmath::hypot(martensite_cmath::hypot(delta[0], delta[1]), delta[2]);
    assert!(
        distance <= request.options.spatial_error,
        "vertex={key:?}; distance={distance}, expected <= {}",
        request.options.spatial_error
    );
}

fn expected_point([x, y]: [f64; 2], depth: f64, normal: NormalSense) -> [f64; 3] {
    let depth = match normal {
        NormalSense::Outward => depth,
        NormalSense::Inward => -depth,
    };
    let sine = martensite_cmath::sin(0.125);
    let cosine = martensite_cmath::cos(0.125);
    let slant = martensite_cmath::hypot(x, y);
    let theta = martensite_cmath::atan2(y, x) / sine;
    let radius = slant * sine + depth * cosine;
    [
        radius * martensite_cmath::cos(theta),
        radius * martensite_cmath::sin(theta),
        slant * cosine - depth * sine,
    ]
}
