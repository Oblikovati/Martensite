//! Full actual seam/carrier residuals retain the original reference and raw limits.
use super::*;
use crate::face_validation::closed_extrusion::metrics;
pub(super) fn certify<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    budget: &mut Budget<'_, F>,
    errors: &mut Vec<ClosedFaceErrorBound>,
) -> CheckResult<()> {
    for (index, edge) in carrier.strip.edges.iter().enumerate() {
        let side = carrier.boundary.sides[index];
        if matches!(side, Side::VFirst | Side::VLast) {
            continue;
        }
        seam_bound(carrier, index, edge, budget, errors)?;
    }
    Ok(())
}
fn seam_bound<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    index: usize,
    edge: &EdgeUse,
    budget: &mut Budget<'_, F>,
    errors: &mut Vec<ClosedFaceErrorBound>,
) -> CheckResult<()> {
    let limits = [
        carrier.strip.tolerance,
        edge.tolerance,
        carrier.edge.tolerance,
    ];
    let limit = metrics::minimum(&limits, edge.owner)?;
    let query = query(carrier, edge, carrier.boundary.sides[index], limit)?;
    metrics::record_carrier_bound([index, edge.owner], limit, query, budget, errors)
}

fn query<'a>(
    carrier: &Carrier<'_>,
    edge: &EdgeUse,
    side: Side,
    limit: f64,
) -> CheckResult<TransportedResidualQuery<'a>> {
    let SpaceCurve::Line(axis) = &edge.curve else {
        return Err(invalid(edge.owner, "seam carrier residual", "non-line"));
    };
    let pole = endpoint(carrier, side, edge.owner)?;
    let curve = chains::line(*axis, edge.placement).curve;
    Ok(TransportedResidualQuery {
        residual: residual(carrier, edge, pole, curve, limit),
        maps: [MetricParameterMap::Identity; 2],
    })
}
fn residual<'a>(
    carrier: &Carrier<'_>,
    edge: &EdgeUse,
    pole: [f64; 3],
    curve: MetricCurve<'a, 3>,
    spatial_limit: f64,
) -> PlanarResidualQuery<'a> {
    PlanarResidualQuery {
        curve,
        curve_embedding: edge.placement,
        pcurve: parameter_curve(edge, carrier.reference),
        plane_embedding: carrier_embedding(carrier.direction, pole),
        plane_placement: carrier.edge.placement,
        interval: edge.range,
        spatial_limit,
        options: martensite_tkmath::predicates::roots::RootIsolationOptions::new(1.),
    }
}

fn endpoint(carrier: &Carrier<'_>, side: Side, owner: usize) -> CheckResult<[f64; 3]> {
    let poles = carrier.curve.poles();
    let point = if side == Side::UFirst {
        poles.first()
    } else {
        poles.last()
    };
    point
        .copied()
        .ok_or_else(|| invalid(owner, "carrier endpoint", "absent"))
}
fn parameter_curve(edge: &EdgeUse, reference: f64) -> MetricCurve<'static, 2> {
    let p = edge.pcurve.location();
    let d = edge.pcurve.direction();
    MetricCurve::Line {
        origin: [p.y(), reference],
        direction: [d.y(), 0.],
    }
}
fn carrier_embedding(direction: [f64; 3], pole: [f64; 3]) -> MetricEmbedding<2> {
    MetricEmbedding {
        linear: direction.map(|v| [v, -v]),
        origin: pole,
        scale: 1.,
    }
}
