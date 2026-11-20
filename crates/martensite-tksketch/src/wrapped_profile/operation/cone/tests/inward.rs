//! #540: inward caller names bind the opposite physical caps without changing shell handedness.
use super::{completed::normals, frozen::FrozenNative, *};
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkmath::gp::Vector3;
#[test]
fn inward_native_uses_and_projected_depths_retain_source_provenance() {
    let mut request = request();
    request.support.normal = NormalSense::Inward;
    let before = request.clone();
    let frozen = frozen::capture(&request);
    assert_eq!(request, before);
    assert_eq!(frozen.inputs.originals[0].depths, [-0.2, 0.1]);
    assert_eq!(
        frozen.source.depths,
        [-request.depths[1], -request.depths[0]]
    );
    check_uses(&request, &frozen);
    check_projection(&request, &frozen);
}
fn check_uses(request: &WrappedConeNormalRequest, frozen: &FrozenNative) {
    let handedness = super::super::super::topology::DevelopedMapHandedness::Reversing;
    coverage::certify_developed_oriented(
        &frozen.source,
        &frozen.shape,
        &frozen.bindings,
        handedness,
    )
    .unwrap();
    let mut checked = [0; 3];
    for face in normals::children(&frozen.shape)
        .iter()
        .flat_map(normals::children)
    {
        check_face(request, frozen, &face, &mut checked);
    }
    assert_eq!(checked[0], 2);
    assert!(checked[1] > 0 && checked[2] > 0, "checked={checked:?}");
}
fn check_face(
    request: &WrappedConeNormalRequest,
    frozen: &FrozenNative,
    face: &Shape,
    checked: &mut [u32; 3],
) {
    let keys: Vec<_> = frozen
        .bindings
        .iter()
        .filter(|(_, shape)| shape.is_same(face))
        .map(|(key, _)| *key)
        .collect();
    assert_eq!(keys.len(), 1);
    let key = keys[0];
    if normals::cap(key.role) {
        normals::check_physical_cap(face, key.role, request.support.semi_angle);
        checked[0] += 1;
    }
    check_side(request.support.semi_angle, face, key, checked);
}
fn check_side(alpha: f64, face: &Shape, key: BindingKey, checked: &mut [u32; 3]) {
    let Source::Span(span) = key.source else {
        return;
    };
    if key.role != HistoryRole::SideFace {
        return;
    }
    if span.curve == SurfaceCurveId(1) {
        normals::check_side(face, Vector3::new(0., -1., 0.));
        checked[1] += 1;
    }
    if span.curve == SurfaceCurveId(5) {
        normals::check_hole(face, alpha);
        checked[2] += 1;
    }
}
fn check_projection(request: &WrappedConeNormalRequest, frozen: &FrozenNative) {
    let keys: Vec<_> = frozen.bindings.iter().map(|(key, _)| *key).collect();
    let projected = history_projection::prepare_physical(
        &keys,
        &frozen.history,
        &frozen.inputs.directions,
        request.support.normal,
    )
    .unwrap();
    let mut vertices = 0;
    for ((physical, shape), caller) in frozen.bindings.iter().zip(&projected.bindings) {
        check_role(*physical, *caller);
        vertices += check_vertex(request, shape, *caller);
    }
    assert_eq!(vertices, request.profile.vertices.len() * 2);
    assert_eq!(projected.records.len(), frozen.history.records.len());
}
fn check_role(physical: BindingKey, caller: WrappedHistoryKey) {
    let expected = match physical.role {
        HistoryRole::FirstDepthFace => HistoryRole::LastDepthFace,
        HistoryRole::LastDepthFace => HistoryRole::FirstDepthFace,
        HistoryRole::FirstDepthEdge => HistoryRole::LastDepthEdge,
        HistoryRole::LastDepthEdge => HistoryRole::FirstDepthEdge,
        HistoryRole::FirstDepthVertex => HistoryRole::LastDepthVertex,
        HistoryRole::LastDepthVertex => HistoryRole::FirstDepthVertex,
        role => role,
    };
    assert_eq!(
        caller.role, expected,
        "physical={physical:?}, caller={caller:?}"
    );
}
pub(super) fn check_vertex(
    request: &WrappedConeNormalRequest,
    shape: &Shape,
    key: WrappedHistoryKey,
) -> usize {
    let WrappedEntitySource::Vertex(id) = key.source else {
        return 0;
    };
    let depth = match key.role {
        HistoryRole::FirstDepthVertex => request.depths[0],
        HistoryRole::LastDepthVertex => request.depths[1],
        _ => return 0,
    };
    let source = authored_vertex(request, id);
    let physical_depth = match request.support.normal {
        NormalSense::Outward => depth,
        NormalSense::Inward => -depth,
    };
    let expected = expected_point(source.point, physical_depth, request.support.semi_angle);
    check_point(request, shape, key, source, expected);
    1
}
fn authored_vertex(request: &WrappedConeNormalRequest, id: SurfaceVertexId) -> &ProfileVertex {
    request
        .profile
        .vertices
        .iter()
        .find(|vertex| vertex.id == ProfileVertexId(id.0))
        .unwrap()
}
fn check_point(
    request: &WrappedConeNormalRequest,
    shape: &Shape,
    key: WrappedHistoryKey,
    source: &ProfileVertex,
    expected: [f64; 3],
) {
    let point = BRepTool::pnt(&topods::vertex(shape).unwrap()).unwrap();
    for (actual, expected) in point.coordinates().iter().zip(expected) {
        assert!(
            (actual - expected).abs() <= 1e-12,
            "source={source:?}, support={:?}, depths={:?}, caller={key:?}, actual={actual}, expected={expected}",
            request.support,
            request.depths
        );
    }
}

pub(super) fn expected_point([x, y]: [f64; 2], physical_depth: f64, alpha: f64) -> [f64; 3] {
    let (s, c) = (martensite_cmath::sin(alpha), martensite_cmath::cos(alpha));
    let slant = martensite_cmath::hypot(x, y);
    let theta = martensite_cmath::atan2(y, x) / s;
    let radius = slant * s + physical_depth * c;
    [
        radius * martensite_cmath::cos(theta),
        radius * martensite_cmath::sin(theta),
        slant * c - physical_depth * s,
    ]
}
