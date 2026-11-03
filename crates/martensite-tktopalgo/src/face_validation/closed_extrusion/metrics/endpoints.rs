//! Complete clamping binds spline endpoints to their actual endpoint control poles.
use super::*;
struct EndpointCheck<'a> {
    strip: &'a Strip,
    edge: &'a EdgeUse,
    boundary: &'a Boundary,
    indices: [usize; 2],
    vertex: &'a VertexUse,
}
pub(super) fn certify<F: FnMut() -> bool>(
    strip: &Strip,
    edge: &EdgeUse,
    index: usize,
    boundary: &Boundary,
    budget: &mut Budget<'_, F>,
    errors: &mut Vec<ClosedFaceErrorBound>,
) -> CheckResult<()> {
    for (position, vertex) in edge.vertices.iter().enumerate() {
        let check = EndpointCheck {
            strip,
            edge,
            boundary,
            indices: [index, position],
            vertex,
        };
        check.certify(budget, errors)?;
    }
    Ok(())
}
impl EndpointCheck<'_> {
    fn certify<F: FnMut() -> bool>(
        &self,
        budget: &mut Budget<'_, F>,
        errors: &mut Vec<ClosedFaceErrorBound>,
    ) -> CheckResult<()> {
        let limit = minimum(
            &[
                self.strip.tolerance,
                self.edge.tolerance,
                self.vertex.tolerance,
            ],
            self.vertex.owner,
        )?;
        self.curve(limit, budget, errors)?;
        self.support(limit, budget, errors)
    }
    fn curve<F: FnMut() -> bool>(
        &self,
        limit: f64,
        budget: &mut Budget<'_, F>,
        errors: &mut Vec<ClosedFaceErrorBound>,
    ) -> CheckResult<()> {
        let parameter = self.edge.range[self.vertex.side];
        let (curve, map_parameter) = curve_endpoint(self.edge, self.vertex.side, parameter)?;
        let query = vertex_query(
            self.vertex,
            curve,
            self.edge.placement,
            map_parameter,
            limit,
        );
        let context = self.context(ClosedFaceRelation::VertexCurveConformance);
        self.record(context, limit, budget.residual(query, context)?, errors)
    }
    fn support<F: FnMut() -> bool>(
        &self,
        limit: f64,
        budget: &mut Budget<'_, F>,
        errors: &mut Vec<ClosedFaceErrorBound>,
    ) -> CheckResult<()> {
        let corner = self.boundary.corners[self.indices[0]][self.vertex.side];
        let query = support_endpoint(self.strip, self.vertex, corner, self.boundary, limit)?;
        let context = self.context(ClosedFaceRelation::VertexSupportConformance);
        self.record(context, limit, budget.residual(query, context)?, errors)
    }
    fn context(&self, relation: ClosedFaceRelation) -> ClosedFaceQueryContext {
        ClosedFaceQueryContext::new(self.vertex.owner, relation)
    }
    fn record(
        &self,
        context: ClosedFaceQueryContext,
        limit: f64,
        decision: ResidualDecision,
        errors: &mut Vec<ClosedFaceErrorBound>,
    ) -> CheckResult<()> {
        append(
            self.indices[0],
            Some(self.indices[1]),
            context,
            limit,
            decision,
            false,
            errors,
        )
    }
}
fn curve_endpoint(
    edge: &EdgeUse,
    side: usize,
    parameter: f64,
) -> CheckResult<(MetricCurve<'static, 3>, f64)> {
    match &edge.curve {
        SpaceCurve::Line(axis) => Ok((edges::line(*axis), parameter)),
        SpaceCurve::Spline(input) => spline_endpoint(input, edge, side),
    }
}
fn spline_endpoint(
    input: &martensite_tkg3d::geom::StoredSplineInput,
    edge: &EdgeUse,
    side: usize,
) -> CheckResult<(MetricCurve<'static, 3>, f64)> {
    require_complete_range(input, edge)?;
    let point = spline_pole(input, edge.owner, side)?;
    Ok((
        MetricCurve::Line {
            origin: point,
            direction: [0.; 3],
        },
        0.,
    ))
}
fn support_endpoint<'a>(
    strip: &Strip,
    vertex: &VertexUse,
    corner: usize,
    boundary: &Boundary,
    limit: f64,
) -> CheckResult<TransportedResidualQuery<'a>> {
    let pole = support_corner_pole(strip, corner)?;
    let v = boundary.bounds[if corner < 2 { 2 } else { 3 }];
    let curve = MetricCurve::Line {
        origin: pole,
        direction: strip.direction,
    };
    let mut query = vertex_query(vertex, curve, strip.placement, v, limit);
    query.residual.interval = [0., 1.];
    Ok(query)
}
fn support_corner_pole(strip: &Strip, corner: usize) -> CheckResult<[f64; 3]> {
    let pole = if corner == 0 || corner == 3 {
        strip.basis.poles().first()
    } else {
        strip.basis.poles().last()
    };
    pole.copied()
        .ok_or_else(|| invalid(0, "support endpoint", "absent"))
}

fn require_complete_range(
    input: &martensite_tkg3d::geom::StoredSplineInput,
    edge: &EdgeUse,
) -> CheckResult<()> {
    if input.domain() != edge.range {
        return Err(unsupported(
            edge.owner,
            "endpoint parameter",
            format!("{:?}", edge.range),
            "complete spline domain",
        ));
    }
    Ok(())
}

fn spline_pole(
    input: &martensite_tkg3d::geom::StoredSplineInput,
    owner: usize,
    side: usize,
) -> CheckResult<[f64; 3]> {
    let point = if side == 0 {
        input.poles().first()
    } else {
        input.poles().last()
    }
    .ok_or_else(|| invalid(owner, "spline endpoint", "absent"))?;
    Ok(*point)
}
