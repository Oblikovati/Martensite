//! Existing exact residual decisions cover entire represented intervals.
mod endpoints;
use super::{
    boundary::{Boundary, Side},
    budget::*,
    decode::*,
    types::*,
};
use martensite_tkmath::predicates::roots::{RootIsolationOptions, metric_bounds::*};
pub(super) fn certify<F: FnMut() -> bool>(
    strip: &Strip,
    boundary: &Boundary,
    budget: &mut Budget<'_, F>,
    errors: &mut Vec<ClosedFaceErrorBound>,
) -> CheckResult<()> {
    for (index, edge) in strip.edges.iter().enumerate() {
        let limit = minimum(&[strip.tolerance, edge.tolerance], edge.owner)?;
        match boundary.sides[index] {
            Side::UFirst | Side::ULast => seam(
                strip,
                edge,
                index,
                boundary.sides[index],
                limit,
                budget,
                errors,
            )?,
            Side::VFirst | Side::VLast => cap(strip, edge, index, boundary, budget, errors)?,
        }
        endpoints::certify(strip, edge, index, boundary, budget, errors)?;
    }
    Ok(())
}
pub(super) fn minimum(values: &[f64], owner: usize) -> CheckResult<f64> {
    if values.iter().any(|v| !v.is_finite() || *v < 0.) {
        return Err(invalid(
            owner,
            "finite nonnegative raw spatial limits",
            format!("{values:?}"),
        ));
    }
    Ok(values.iter().copied().fold(f64::INFINITY, f64::min))
}
fn seam<F: FnMut() -> bool>(
    strip: &Strip,
    edge: &EdgeUse,
    index: usize,
    side: Side,
    limit: f64,
    budget: &mut Budget<'_, F>,
    errors: &mut Vec<ClosedFaceErrorBound>,
) -> CheckResult<()> {
    let SpaceCurve::Line(axis) = &edge.curve else {
        return Err(unsupported(
            edge.owner,
            "seam curve",
            "spline",
            "stored line",
        ));
    };
    let pole = if side == Side::UFirst {
        strip.basis.poles().first()
    } else {
        strip.basis.poles().last()
    }
    .ok_or_else(|| invalid(edge.owner, "support endpoint", "absent"))?;
    let p = axis.location();
    let d = axis.direction();
    let uv = edge.pcurve.location();
    let ud = edge.pcurve.direction();
    let query = residual(
        strip,
        edge,
        MetricCurve::Line {
            origin: [p.x(), p.y(), p.z()],
            direction: [d.x(), d.y(), d.z()],
        },
        *pole,
        [uv.y(), ud.y()],
        edge.range,
        limit,
    );
    append(
        index,
        None,
        "seam",
        limit,
        budget.residual(query)?,
        false,
        errors,
    )
}
fn cap<F: FnMut() -> bool>(
    strip: &Strip,
    edge: &EdgeUse,
    index: usize,
    boundary: &Boundary,
    budget: &mut Budget<'_, F>,
    errors: &mut Vec<ClosedFaceErrorBound>,
) -> CheckResult<()> {
    let SpaceCurve::Spline(curve) = &edge.curve else {
        return Err(unsupported(
            edge.owner,
            "cap curve",
            "line",
            "matching stored spline",
        ));
    };
    matching_basis(&strip.basis, curve, edge.owner)?;
    let p = edge.pcurve.location();
    let d = edge.pcurve.direction();
    if p.x() != 0. || d.x() != 1. || d.y() != 0. || edge.range != strip.basis.domain() {
        return Err(unsupported(
            edge.owner,
            "cap parameter map",
            format!("{p:?},{d:?},{:?}", edge.range),
            "(t,V) over full support U domain",
        ));
    }
    let limit = minimum(&[strip.tolerance, edge.tolerance], edge.owner)?;
    let level = if boundary.sides[index] == Side::VFirst {
        boundary.bounds[2]
    } else {
        boundary.bounds[3]
    };
    for (actual, source) in curve.poles().iter().zip(strip.basis.poles()) {
        let query = residual(
            strip,
            edge,
            MetricCurve::Line {
                origin: *actual,
                direction: [0.; 3],
            },
            *source,
            [level, 0.],
            [0., 1.],
            limit,
        );
        append(
            index,
            None,
            "cap coefficient",
            limit,
            budget.residual(query)?,
            true,
            errors,
        )?;
    }
    Ok(())
}
fn matching_basis(
    first: &martensite_tkg3d::geom::StoredSplineInput,
    last: &martensite_tkg3d::geom::StoredSplineInput,
    owner: usize,
) -> CheckResult<()> {
    if first.periodic() != last.periodic()
        || first.rational() != last.rational()
        || first.degree() != last.degree()
        || first.knots() != last.knots()
        || first.multiplicities() != last.multiplicities()
        || first.weights() != last.weights()
        || first
            .flat_knots()
            .iter()
            .map(|x| x.to_bits())
            .ne(last.flat_knots().iter().map(|x| x.to_bits()))
        || first.domain() != last.domain()
        || first.poles().len() != last.poles().len()
        || last.evaluation_override()
    {
        return Err(unsupported(
            owner,
            "cap basis",
            "different stored basis",
            "identical complete knots, degree, weights and domain",
        ));
    }
    Ok(())
}
fn residual<'a>(
    strip: &Strip,
    edge: &EdgeUse,
    curve: MetricCurve<'a, 3>,
    origin: [f64; 3],
    map: [f64; 2],
    interval: [f64; 2],
    limit: f64,
) -> TransportedResidualQuery<'a> {
    TransportedResidualQuery {
        residual: PlanarResidualQuery {
            curve,
            curve_embedding: edge.placement,
            pcurve: MetricCurve::Line {
                origin: [map[0], 0.],
                direction: [map[1], 0.],
            },
            plane_embedding: MetricEmbedding {
                linear: std::array::from_fn(|i| [strip.direction[i], 0.]),
                origin,
                scale: 1.,
            },
            plane_placement: strip.placement,
            interval,
            spatial_limit: limit,
            options: RootIsolationOptions::new(1.),
        },
        maps: [MetricParameterMap::Identity; 2],
    }
}
pub(super) fn append(
    index: usize,
    vertex: Option<usize>,
    role: &'static str,
    limit: f64,
    decision: ResidualDecision,
    coefficient: bool,
    errors: &mut Vec<ClosedFaceErrorBound>,
) -> CheckResult<()> {
    match decision {
        ResidualDecision::Within {
            proven_squared_upper,
        } => {
            errors.push(ClosedFaceErrorBound {
                edge: index,
                vertex,
                role,
                spatial_limit: limit,
                squared_upper: proven_squared_upper,
            });
            Ok(())
        }
        ResidualDecision::Exceeds { witness } if !coefficient => Err(invalid(
            index,
            "spatial consistency",
            format!("role={role}, limit={limit}, witness={witness:?}"),
        )),
        ResidualDecision::Exceeds { .. } => Err(ClosedFaceCompletion::Incomplete(
            ClosedFaceIncomplete::CoefficientBound { edge: index, limit },
        )),
    }
}
pub(super) fn vertex_query<'a>(
    vertex: &VertexUse,
    curve: MetricCurve<'a, 3>,
    placement: MetricEmbedding<3>,
    parameter: f64,
    limit: f64,
) -> TransportedResidualQuery<'a> {
    TransportedResidualQuery {
        residual: PlanarResidualQuery {
            curve,
            curve_embedding: placement,
            pcurve: MetricCurve::Line {
                origin: [0.; 2],
                direction: [0.; 2],
            },
            plane_embedding: MetricEmbedding {
                linear: [[0.; 2]; 3],
                origin: vertex.point,
                scale: 1.,
            },
            plane_placement: vertex.placement,
            interval: [0., 1.],
            spatial_limit: limit,
            options: RootIsolationOptions::new(1.),
        },
        maps: [
            MetricParameterMap::Endpoints([parameter; 2]),
            MetricParameterMap::Identity,
        ],
    }
}
