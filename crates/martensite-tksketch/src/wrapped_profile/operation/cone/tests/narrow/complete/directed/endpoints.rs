//! Numerical oracle agreement uses achieved certified source error, without libm certification.
use super::*;
use martensite_tkbrep::{brep::BRepTool, topods};
pub(super) fn check(
    result: &WrappedConeNormalResult,
    request: &WrappedConeNormalRequest,
    detached: &DetachedWrappedBodies,
) {
    let count: usize = detached
        .entities
        .iter()
        .map(|(key, shape)| vertex(result, request, *key, shape))
        .sum();
    assert_eq!(count, request.profile.vertices.len() * 2);
}
fn vertex(
    result: &WrappedConeNormalResult,
    request: &WrappedConeNormalRequest,
    key: WrappedHistoryKey,
    shape: &Shape,
) -> usize {
    let WrappedEntitySource::Vertex(id) = key.source else {
        return 0;
    };
    let depth = match key.role {
        HistoryRole::FirstDepthVertex => request.depths[0],
        HistoryRole::LastDepthVertex => request.depths[1],
        _ => return 0,
    };
    let source = request
        .profile
        .vertices
        .iter()
        .find(|vertex| vertex.id.0 == id.0)
        .unwrap();
    compare(request, key, shape, source, depth, achieved(result, id));
    1
}
fn compare(
    request: &WrappedConeNormalRequest,
    key: WrappedHistoryKey,
    shape: &Shape,
    source: &ProfileVertex,
    depth: f64,
    bound: WrappedCertifiedBound,
) {
    let physical = match request.support.normal {
        NormalSense::Outward => depth,
        NormalSense::Inward => -depth,
    };
    let expected = point(source.point, physical, request.support);
    let actual = BRepTool::pnt(&topods::vertex(shape).unwrap()).unwrap();
    assert!(bound.upper.is_finite() && bound.upper <= request.options.spatial_error);
    for (coordinate, expected) in actual.coordinates().iter().zip(expected) {
        assert!(
            (coordinate - expected).abs() <= bound.upper,
            "numerical normal-map agreement source={source:?} key={key:?} actual={coordinate} oracle={expected} certified_source_bound={bound:?}"
        );
    }
}
fn achieved(result: &WrappedConeNormalResult, vertex: SurfaceVertexId) -> WrappedCertifiedBound {
    let prepared = owner(result, vertex);
    let spans: Vec<_> = prepared
        .source
        .loops
        .iter()
        .flatten()
        .filter(|piece| incident(piece, vertex))
        .map(|piece| piece.span)
        .collect();
    let geometry = &result.family.family.members[&prepared.source.region].geometry;
    let upper = geometry
        .spans
        .iter()
        .filter(|proof| spans.contains(&proof.span))
        .map(|proof| measures::upper(proof.boundary.total()).unwrap())
        .reduce(f64::max)
        .unwrap();
    WrappedCertifiedBound { upper }
}
fn owner(result: &WrappedConeNormalResult, vertex: SurfaceVertexId) -> &Prepared {
    let owners: Vec<_> = result
        .family
        .family
        .selection
        .prepared
        .iter()
        .filter(|prepared| owns_vertex(prepared, vertex))
        .collect();
    assert_eq!(
        owners.len(),
        1,
        "authored endpoint must retain exactly one selected source owner"
    );
    owners[0]
}
fn incident(piece: &AuthoredPiece, vertex: SurfaceVertexId) -> bool {
    piece.start == CornerSource::Authored(vertex) || piece.end == CornerSource::Authored(vertex)
}
fn point([x, y]: [f64; 2], physical: f64, support: ConeSupport) -> [f64; 3] {
    let sine = martensite_cmath::sin(support.semi_angle);
    let cosine = martensite_cmath::cos(support.semi_angle);
    let slant = martensite_cmath::hypot(x, y);
    let theta = support.chart_origin + martensite_cmath::atan2(y, x) / sine;
    let radius = slant * sine + physical * cosine;
    [
        radius * martensite_cmath::cos(theta),
        radius * martensite_cmath::sin(theta),
        slant * cosine - physical * sine,
    ]
}

fn owns_vertex(prepared: &Prepared, vertex: SurfaceVertexId) -> bool {
    prepared
        .source
        .loops
        .iter()
        .flatten()
        .any(|piece| incident(piece, vertex))
}
