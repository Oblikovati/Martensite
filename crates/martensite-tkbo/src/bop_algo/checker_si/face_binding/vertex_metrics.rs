// SPDX-License-Identifier: GPL-2.0-only
//! Constant parameter maps preserve exact affine endpoint operands, never rounded evaluated points.
use super::{capture_curves::unsupported, exact, refine, types::*};
use martensite_tkmath::predicates::roots::RootIsolationOptions;
use martensite_tkmath::predicates::roots::metric_bounds::*;

pub(super) fn check(
    packet: &OwnedFacePacket,
    endpoint: &EndpointUse<'_>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<VertexEndpointCertificate, BindingFailure> {
    let edge = &packet.edges[endpoint.edge_address];
    let limit = raw_limit(packet, edge, endpoint.vertex)
        .map_err(|cause| context(endpoint, "stored endpoint budget", cause))?;
    let curve = bound(
        endpoint,
        limit,
        |radius| curve_query(edge, endpoint, radius),
        cancelled,
    )
    .map_err(|cause| context(endpoint, "vertex versus 3D curve", cause))?;
    let support = bound(
        endpoint,
        limit,
        |radius| Ok(support_query(packet, edge, endpoint, radius)),
        cancelled,
    )
    .map_err(|cause| context(endpoint, "vertex versus support pcurve", cause))?;
    Ok(certificate(endpoint, curve, support))
}
fn certificate(
    endpoint: &EndpointUse<'_>,
    curve: EndpointBound,
    support: EndpointBound,
) -> VertexEndpointCertificate {
    VertexEndpointCertificate {
        edge_address: endpoint.edge_address,
        vertex_address: endpoint.vertex_address,
        source_vertex_position: endpoint.vertex.position,
        parameter: endpoint.parameter,
        side: endpoint.side,
        corner: endpoint.corner,
        curve,
        support,
    }
}
fn context(
    endpoint: &EndpointUse<'_>,
    check: &'static str,
    cause: BindingFailure,
) -> BindingFailure {
    BindingFailure::VertexEndpoint {
        edge: endpoint.edge_address,
        vertex: endpoint.vertex_address,
        check,
        cause: Box::new(cause),
    }
}
fn raw_limit(
    packet: &OwnedFacePacket,
    edge: &OwnedEdgeUse,
    vertex: &OwnedVertexUse,
) -> Result<f64, BindingFailure> {
    let raw = [
        packet.raw_face_tolerance,
        edge.raw_tolerance,
        vertex.raw_tolerance,
    ];
    let effective = [
        packet.face_tolerance,
        edge.tolerance,
        vertex.effective_tolerance,
    ];
    require_budget(raw, effective, vertex.local_point)?;
    let limit = raw.into_iter().fold(f64::INFINITY, f64::min);
    Ok(if limit == 0. { 0. } else { limit })
}
fn require_budget(
    raw: [f64; 3],
    effective: [f64; 3],
    point: [f64; 3],
) -> Result<(), BindingFailure> {
    if raw
        .iter()
        .chain(effective.iter())
        .any(|v| !v.is_finite() || *v < 0.)
        || raw
            .iter()
            .zip(effective)
            .any(|(raw, effective)| *raw > effective)
        || point.iter().any(|v| !v.is_finite())
    {
        return Err(unsupported(
            "endpoint raw operands",
            format!(
                "raw={raw:?}, effective={effective:?}, point={:?}; expected finite nonnegative stored limits without widening",
                point
            ),
        ));
    }
    Ok(())
}

fn bound(
    endpoint: &EndpointUse<'_>,
    limit: f64,
    mut query: impl FnMut(f64) -> Result<TransportedResidualQuery<'static>, BindingFailure>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<EndpointBound, BindingFailure> {
    let squared = exact::squared_limit(limit, cancelled)?;
    let mut bisections = 0;
    let (upper, work) = refine::certify_decisions(endpoint.vertex_address, limit, |radius| {
        let report = certify_transported_residual(query(radius)?, &mut *cancelled);
        bisections += report.bisections;
        Ok(report.decision?)
    })?;
    if upper > squared {
        return Err(unsupported(
            "endpoint certified bound",
            format!(
                "upper={upper}, requested={squared}; expected certified upper <= raw squared limit"
            ),
        ));
    }
    Ok(EndpointBound {
        requested_spatial_limit: limit,
        requested_squared_limit: squared,
        certified_squared_upper: upper,
        threshold_queries: work,
        root_bisections: bisections,
    })
}
fn curve_query(
    edge: &OwnedEdgeUse,
    endpoint: &EndpointUse<'_>,
    limit: f64,
) -> Result<TransportedResidualQuery<'static>, BindingFailure> {
    let curve = curve_endpoint(edge, endpoint)?;
    Ok(TransportedResidualQuery {
        residual: PlanarResidualQuery {
            curve,
            curve_embedding: edge.placement.embedding(),
            pcurve: MetricCurve::Line {
                origin: [0.; 2],
                direction: [0.; 2],
            },
            plane_embedding: MetricEmbedding {
                linear: [[0.; 2]; 3],
                origin: endpoint.vertex.local_point,
                scale: 1.,
            },
            plane_placement: endpoint.vertex.placement.embedding(),
            interval: [0., 1.],
            spatial_limit: limit,
            options: RootIsolationOptions::new(1.),
        },
        maps: [
            MetricParameterMap::Endpoints([endpoint.parameter; 2]),
            MetricParameterMap::Identity,
        ],
    })
}
fn curve_endpoint(
    edge: &OwnedEdgeUse,
    endpoint: &EndpointUse<'_>,
) -> Result<MetricCurve<'static, 3>, BindingFailure> {
    match &edge.curve {
        OwnedSpaceCurve::Line(line) => Ok(MetricCurve::Line {
            origin: line.origin,
            direction: line.direction,
        }),
        OwnedSpaceCurve::Spline(curve) => spline_endpoint(curve, endpoint),
    }
}
fn spline_endpoint(
    curve: &OwnedSpline,
    endpoint: &EndpointUse<'_>,
) -> Result<MetricCurve<'static, 3>, BindingFailure> {
    let index = if endpoint.side == EndpointSide::First {
        0
    } else {
        1
    };
    if endpoint.parameter != curve.domain[index] {
        return Err(unsupported(
            "clamped endpoint parameter",
            format!("{}; expected {}", endpoint.parameter, curve.domain[index]),
        ));
    }
    // The preceding strip proof establishes identical complete clamped bases.
    let pole = if index == 0 {
        curve.poles[0]
    } else {
        *curve.poles.last().expect("validated poles")
    };
    Ok(MetricCurve::Line {
        origin: pole,
        direction: [0.; 3],
    })
}

fn support_query(
    packet: &OwnedFacePacket,
    edge: &OwnedEdgeUse,
    endpoint: &EndpointUse<'_>,
    limit: f64,
) -> TransportedResidualQuery<'static> {
    TransportedResidualQuery {
        residual: PlanarResidualQuery {
            curve: MetricCurve::Line {
                origin: endpoint.vertex.local_point,
                direction: [0.; 3],
            },
            curve_embedding: endpoint.vertex.placement.embedding(),
            pcurve: MetricCurve::Line {
                origin: [edge.pcurve.origin[1], 0.],
                direction: [edge.pcurve.direction[1], 0.],
            },
            plane_embedding: support_embedding(packet, endpoint.corner),
            plane_placement: packet.support_placement.embedding(),
            interval: [0., 1.],
            spatial_limit: limit,
            options: RootIsolationOptions::new(1.),
        },
        maps: [
            MetricParameterMap::Identity,
            MetricParameterMap::Endpoints([endpoint.parameter; 2]),
        ],
    }
}

fn support_embedding(packet: &OwnedFacePacket, corner: usize) -> MetricEmbedding<2> {
    let pole = if matches!(corner, 0 | 3) {
        packet.basis.poles[0]
    } else {
        *packet.basis.poles.last().expect("validated poles")
    };
    MetricEmbedding {
        linear: std::array::from_fn(|i| [packet.direction[i], 0.]),
        origin: pole,
        scale: 1.,
    }
}
