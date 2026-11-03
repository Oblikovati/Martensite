//! Query construction retains scales and placements without rounded transformed poles.
use super::*;
pub(super) fn residual<'a>(
    strip: &Strip,
    edge: &EdgeUse,
    curve: MetricCurve<'a, 3>,
    origin: [f64; 3],
    map: [f64; 2],
    interval: [f64; 2],
    limit: f64,
) -> TransportedResidualQuery<'a> {
    let embedding = support_embedding(origin, strip.direction);
    let residual = planar(
        curve,
        edge.placement,
        scalar_line(map),
        embedding,
        strip.placement,
        interval,
        limit,
    );
    transported(residual, [MetricParameterMap::Identity; 2])
}
pub(super) fn vertex_query<'a>(
    vertex: &VertexUse,
    curve: MetricCurve<'a, 3>,
    placement: MetricEmbedding<3>,
    parameter: f64,
    limit: f64,
) -> TransportedResidualQuery<'a> {
    let point = support_embedding(vertex.point, [0.; 3]);
    let residual = planar(
        curve,
        placement,
        scalar_line([0.; 2]),
        point,
        vertex.placement,
        [0., 1.],
        limit,
    );
    transported(
        residual,
        [
            MetricParameterMap::Endpoints([parameter; 2]),
            MetricParameterMap::Identity,
        ],
    )
}
fn support_embedding(origin: [f64; 3], direction: [f64; 3]) -> MetricEmbedding<2> {
    MetricEmbedding {
        linear: std::array::from_fn(|i| [direction[i], 0.]),
        origin,
        scale: 1.,
    }
}
fn scalar_line(map: [f64; 2]) -> MetricCurve<'static, 2> {
    MetricCurve::Line {
        origin: [map[0], 0.],
        direction: [map[1], 0.],
    }
}
fn planar<'a>(
    curve: MetricCurve<'a, 3>,
    curve_embedding: MetricEmbedding<3>,
    pcurve: MetricCurve<'a, 2>,
    plane_embedding: MetricEmbedding<2>,
    plane_placement: MetricEmbedding<3>,
    interval: [f64; 2],
    spatial_limit: f64,
) -> PlanarResidualQuery<'a> {
    PlanarResidualQuery {
        curve,
        curve_embedding,
        pcurve,
        plane_embedding,
        plane_placement,
        interval,
        spatial_limit,
        options: RootIsolationOptions::new(1.),
    }
}
fn transported(
    residual: PlanarResidualQuery<'_>,
    maps: [MetricParameterMap; 2],
) -> TransportedResidualQuery<'_> {
    TransportedResidualQuery { residual, maps }
}
