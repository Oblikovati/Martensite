//! Complete clamping binds spline endpoints to their actual endpoint control poles.
use super::*;
pub(super) fn certify<F: FnMut() -> bool>(
    strip: &Strip,
    edge: &EdgeUse,
    index: usize,
    boundary: &Boundary,
    budget: &mut Budget<'_, F>,
    errors: &mut Vec<ClosedFaceErrorBound>,
) -> CheckResult<()> {
    for (position, vertex) in edge.vertices.iter().enumerate() {
        let limit = minimum(
            &[strip.tolerance, edge.tolerance, vertex.tolerance],
            vertex.owner,
        )?;
        let parameter = edge.range[vertex.side];
        let (curve, map_parameter) = curve_endpoint(edge, vertex.side, parameter)?;
        let query = vertex_query(vertex, curve, edge.placement, map_parameter, limit);
        append(
            index,
            Some(position),
            "vertex/curve",
            limit,
            budget.residual(query)?,
            false,
            errors,
        )?;
        let corner = boundary.corners[index][vertex.side];
        let query = support_endpoint(strip, vertex, corner, boundary, limit)?;
        append(
            index,
            Some(position),
            "vertex/support",
            limit,
            budget.residual(query)?,
            false,
            errors,
        )?;
    }
    Ok(())
}
fn curve_endpoint(
    edge: &EdgeUse,
    side: usize,
    parameter: f64,
) -> CheckResult<(MetricCurve<'static, 3>, f64)> {
    match &edge.curve {
        SpaceCurve::Line(axis) => {
            let p = axis.location();
            let d = axis.direction();
            Ok((
                MetricCurve::Line {
                    origin: [p.x(), p.y(), p.z()],
                    direction: [d.x(), d.y(), d.z()],
                },
                parameter,
            ))
        }
        SpaceCurve::Spline(input) => {
            if input.domain() != edge.range {
                return Err(unsupported(
                    edge.owner,
                    "endpoint parameter",
                    format!("{:?}", edge.range),
                    "complete spline domain",
                ));
            }
            let point = if side == 0 {
                input.poles().first()
            } else {
                input.poles().last()
            }
            .ok_or_else(|| invalid(edge.owner, "spline endpoint", "absent"))?;
            Ok((
                MetricCurve::Line {
                    origin: *point,
                    direction: [0.; 3],
                },
                0.,
            ))
        }
    }
}
fn support_endpoint<'a>(
    strip: &Strip,
    vertex: &VertexUse,
    corner: usize,
    boundary: &Boundary,
    limit: f64,
) -> CheckResult<TransportedResidualQuery<'a>> {
    let first = corner == 0 || corner == 3;
    let bottom = corner < 2;
    let pole = if first {
        strip.basis.poles().first()
    } else {
        strip.basis.poles().last()
    }
    .ok_or_else(|| invalid(0, "support endpoint", "absent"))?;
    let v = boundary.bounds[if bottom { 2 } else { 3 }];
    let mut query = vertex_query(
        vertex,
        MetricCurve::Line {
            origin: *pole,
            direction: strip.direction,
        },
        strip.placement,
        v,
        limit,
    );
    query.residual.interval = [0., 1.];
    Ok(query)
}
