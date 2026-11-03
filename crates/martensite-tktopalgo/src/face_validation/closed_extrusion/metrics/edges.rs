//! One edge's full interval and endpoint obligations share the same requested budget.
use super::*;
use claims::BoundClaim;
use martensite_tkg3d::geom::StoredSplineInput;
pub(super) struct EdgeMetrics<'a> {
    strip: &'a Strip,
    edge: &'a EdgeUse,
    index: usize,
    boundary: &'a Boundary,
}
impl<'a> EdgeMetrics<'a> {
    pub(super) fn new(
        strip: &'a Strip,
        edge: &'a EdgeUse,
        index: usize,
        boundary: &'a Boundary,
    ) -> Self {
        Self {
            strip,
            edge,
            index,
            boundary,
        }
    }
    pub(super) fn certify<F: FnMut() -> bool>(
        &self,
        budget: &mut Budget<'_, F>,
        errors: &mut Vec<ClosedFaceErrorBound>,
    ) -> CheckResult<()> {
        let limit = self.raw_limit()?;
        match self.boundary.sides[self.index] {
            Side::UFirst | Side::ULast => self.seam(limit, budget, errors)?,
            Side::VFirst | Side::VLast => self.cap(budget, errors)?,
        }
        self.endpoints(budget, errors)
    }

    fn raw_limit(&self) -> CheckResult<f64> {
        minimum(
            &[self.strip.tolerance, self.edge.tolerance],
            self.edge.owner,
        )
    }
    fn endpoints<F: FnMut() -> bool>(
        &self,
        budget: &mut Budget<'_, F>,
        errors: &mut Vec<ClosedFaceErrorBound>,
    ) -> CheckResult<()> {
        endpoints::certify(
            self.strip,
            self.edge,
            self.index,
            self.boundary,
            budget,
            errors,
        )
    }
    fn cap_level(&self) -> f64 {
        let side = if self.boundary.sides[self.index] == Side::VFirst {
            2
        } else {
            3
        };
        self.boundary.bounds[side]
    }
    fn seam<F: FnMut() -> bool>(
        &self,
        limit: f64,
        budget: &mut Budget<'_, F>,
        errors: &mut Vec<ClosedFaceErrorBound>,
    ) -> CheckResult<()> {
        let side = self.boundary.sides[self.index];
        let query = seam_query(self.strip, self.edge, side, limit)?;
        let claim = BoundClaim::new(self.index, None, "seam", limit, false);
        claim.record(budget.residual(query)?, errors)
    }
    fn cap<F: FnMut() -> bool>(
        &self,
        budget: &mut Budget<'_, F>,
        errors: &mut Vec<ClosedFaceErrorBound>,
    ) -> CheckResult<()> {
        let curve = self.cap_input()?;
        let limit = self.raw_limit()?;
        let level = self.cap_level();
        let claim = BoundClaim::new(self.index, None, "cap coefficient", limit, true);
        for (actual, source) in curve.poles().iter().zip(self.strip.basis.poles()) {
            let query = coefficient_query(self.strip, self.edge, *actual, *source, level, limit);
            claim.record(budget.residual(query)?, errors)?;
        }
        Ok(())
    }
    fn cap_input(&self) -> CheckResult<&StoredSplineInput> {
        let SpaceCurve::Spline(curve) = &self.edge.curve else {
            return Err(unsupported(
                self.edge.owner,
                "cap curve",
                "line",
                "matching stored spline",
            ));
        };
        matching_basis(&self.strip.basis, curve, self.edge.owner)?;
        require_cap_map(self.strip, self.edge)?;
        Ok(curve)
    }
}
fn seam_query(
    strip: &Strip,
    edge: &EdgeUse,
    side: Side,
    limit: f64,
) -> CheckResult<TransportedResidualQuery<'static>> {
    let curve = seam_curve(edge)?;
    let pole = support_pole(strip, edge, side)?;
    let uv = edge.pcurve.location();
    let ud = edge.pcurve.direction();
    Ok(residual(
        strip,
        edge,
        curve,
        pole,
        [uv.y(), ud.y()],
        edge.range,
        limit,
    ))
}
fn seam_curve(edge: &EdgeUse) -> CheckResult<MetricCurve<'static, 3>> {
    match &edge.curve {
        SpaceCurve::Line(axis) => Ok(line(*axis)),
        SpaceCurve::Spline(_) => Err(unsupported(
            edge.owner,
            "seam curve",
            "spline",
            "stored line",
        )),
    }
}
fn support_pole(strip: &Strip, edge: &EdgeUse, side: Side) -> CheckResult<[f64; 3]> {
    let pole = if side == Side::UFirst {
        strip.basis.poles().first()
    } else {
        strip.basis.poles().last()
    };
    pole.copied()
        .ok_or_else(|| invalid(edge.owner, "support endpoint", "absent"))
}
pub(super) fn line(axis: martensite_tkmath::gp::Axis1) -> MetricCurve<'static, 3> {
    let p = axis.location();
    let d = axis.direction();
    MetricCurve::Line {
        origin: [p.x(), p.y(), p.z()],
        direction: [d.x(), d.y(), d.z()],
    }
}
fn coefficient_query(
    strip: &Strip,
    edge: &EdgeUse,
    actual: [f64; 3],
    source: [f64; 3],
    level: f64,
    limit: f64,
) -> TransportedResidualQuery<'static> {
    let curve = MetricCurve::Line {
        origin: actual,
        direction: [0.; 3],
    };
    residual(strip, edge, curve, source, [level, 0.], [0., 1.], limit)
}
fn require_cap_map(strip: &Strip, edge: &EdgeUse) -> CheckResult<()> {
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
    Ok(())
}
fn matching_basis(
    first: &martensite_tkg3d::geom::StoredSplineInput,
    last: &martensite_tkg3d::geom::StoredSplineInput,
    owner: usize,
) -> CheckResult<()> {
    if !same_basis(first, last) {
        return Err(unsupported(
            owner,
            "cap basis",
            "different stored basis",
            "identical complete knots, degree, weights and domain",
        ));
    }
    Ok(())
}

fn same_basis(first: &StoredSplineInput, last: &StoredSplineInput) -> bool {
    first.periodic() == last.periodic()
        && first.rational() == last.rational()
        && first.degree() == last.degree()
        && first.knots() == last.knots()
        && first.multiplicities() == last.multiplicities()
        && first.weights() == last.weights()
        && same_flat_knots(first, last)
        && first.domain() == last.domain()
        && first.poles().len() == last.poles().len()
        && !last.evaluation_override()
}
fn same_flat_knots(first: &StoredSplineInput, last: &StoredSplineInput) -> bool {
    first
        .flat_knots()
        .iter()
        .map(|x| x.to_bits())
        .eq(last.flat_knots().iter().map(|x| x.to_bits()))
}
