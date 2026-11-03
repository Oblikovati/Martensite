//! Query construction retains scales and placements without rounded transformed poles.
use super::*;
struct ResidualFrame {
    curve: MetricEmbedding<3>,
    plane: MetricEmbedding<2>,
    placement: MetricEmbedding<3>,
}
pub(super) fn residual<'a>(
    strip: &Strip,
    edge: &EdgeUse,
    curve: MetricCurve<'a, 3>,
    origin: [f64; 3],
    map: [f64; 2],
    interval: [f64; 2],
    limit: f64,
) -> TransportedResidualQuery<'a> {
    let frame = support_frame(strip, edge, origin);
    let residual = frame.query(curve, scalar_line(map), interval, limit);
    transported(residual, [MetricParameterMap::Identity; 2])
}
pub(super) fn vertex_query<'a>(
    vertex: &VertexUse,
    curve: MetricCurve<'a, 3>,
    placement: MetricEmbedding<3>,
    parameter: f64,
    limit: f64,
) -> TransportedResidualQuery<'a> {
    let frame = vertex_frame(vertex, placement);
    let residual = frame.query(curve, scalar_line([0.; 2]), [0., 1.], limit);
    let maps = [
        MetricParameterMap::Endpoints([parameter; 2]),
        MetricParameterMap::Identity,
    ];
    transported(residual, maps)
}
fn support_frame(strip: &Strip, edge: &EdgeUse, origin: [f64; 3]) -> ResidualFrame {
    ResidualFrame {
        curve: edge.placement,
        plane: support_embedding(origin, strip.direction),
        placement: strip.placement,
    }
}
fn vertex_frame(vertex: &VertexUse, curve: MetricEmbedding<3>) -> ResidualFrame {
    ResidualFrame {
        curve,
        plane: support_embedding(vertex.point, [0.; 3]),
        placement: vertex.placement,
    }
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
impl ResidualFrame {
    fn query<'a>(
        &self,
        curve: MetricCurve<'a, 3>,
        pcurve: MetricCurve<'a, 2>,
        interval: [f64; 2],
        spatial_limit: f64,
    ) -> PlanarResidualQuery<'a> {
        PlanarResidualQuery {
            curve,
            curve_embedding: self.curve,
            pcurve,
            plane_embedding: self.plane,
            plane_placement: self.placement,
            interval,
            spatial_limit,
            options: RootIsolationOptions::new(1.),
        }
    }
}
fn transported(
    residual: PlanarResidualQuery<'_>,
    maps: [MetricParameterMap; 2],
) -> TransportedResidualQuery<'_> {
    TransportedResidualQuery { residual, maps }
}
