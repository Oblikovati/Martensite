//! Actual coefficients, seams and vertices must be exact carrier boundary relations.
use super::*;
pub(super) fn certify<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    budget: &mut Budget<'_, F>,
) -> CheckResult<()> {
    direction(carrier, budget)?;
    for (index, edge) in carrier.strip.edges.iter().enumerate() {
        match carrier.boundary.sides[index] {
            Side::UFirst | Side::ULast => seam(carrier, edge, index, budget)?,
            Side::VFirst | Side::VLast => cap(carrier, edge, index, budget)?,
        }
        vertices(carrier, edge, index, budget)?;
    }
    Ok(())
}
pub(super) fn certify_resolved<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    budget: &mut Budget<'_, F>,
) -> CheckResult<()> {
    direction(carrier, budget)?;
    for (index, edge) in carrier.strip.edges.iter().enumerate() {
        match carrier.boundary.sides[index] {
            Side::UFirst | Side::ULast => ideal_seam(
                carrier,
                edge,
                carrier.boundary.sides[index] == Side::UFirst,
                budget,
            )?,
            Side::VFirst | Side::VLast => cap(carrier, edge, index, budget)?,
        }
        vertices(carrier, edge, index, budget)?;
    }
    Ok(())
}

pub(super) fn direction<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    budget: &mut Budget<'_, F>,
) -> CheckResult<()> {
    let first = chains::vector(carrier.strip.direction, carrier.strip.placement);
    let second = chains::vector(carrier.direction, carrier.edge.placement);
    chains::require(
        ClosedFaceQueryContext::new(carrier.edge.owner, ClosedFaceRelation::CarrierDirection),
        first,
        second,
        budget,
    )
}
fn cap<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    edge: &EdgeUse,
    index: usize,
    budget: &mut Budget<'_, F>,
) -> CheckResult<()> {
    let curve = cap_curve(edge)?;
    let level = cap_level(carrier, index);
    for (actual, pole) in curve.poles().iter().zip(carrier.curve.poles()) {
        cap_coefficient(carrier, edge, *actual, *pole, level, budget)?;
    }
    Ok(())
}
fn cap_curve(edge: &EdgeUse) -> CheckResult<&StoredSplineInput> {
    let SpaceCurve::Spline(curve) = &edge.curve else {
        return Err(invalid(edge.owner, "actual cap curve", "line"));
    };
    Ok(curve)
}
fn cap_level(carrier: &Carrier<'_>, index: usize) -> f64 {
    carrier.boundary.bounds[if carrier.boundary.sides[index] == Side::VFirst {
        2
    } else {
        3
    }]
}
fn cap_coefficient<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    edge: &EdgeUse,
    actual: [f64; 3],
    pole: [f64; 3],
    level: f64,
    budget: &mut Budget<'_, F>,
) -> CheckResult<()> {
    let poses = [carrier.edge.placement];
    let first = chains::point(actual, edge.placement);
    let second = carrier_target(carrier, pole, [level, 0.], &poses);
    chains::require(
        ClosedFaceQueryContext::new(edge.owner, ClosedFaceRelation::CarrierCap),
        first,
        second,
        budget,
    )
}
fn carrier_target<'a>(
    carrier: &Carrier<'_>,
    point: [f64; 3],
    map: [f64; 2],
    poses: &'a [MetricEmbedding<3>],
) -> MetricCurveChain<'a, 2> {
    chains::sweep(point, carrier.direction, map, carrier.reference, poses)
}
fn seam<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    edge: &EdgeUse,
    index: usize,
    budget: &mut Budget<'_, F>,
) -> CheckResult<()> {
    let axis = seam_axis(edge, "actual seam curve", "spline")?;
    let first = carrier.boundary.sides[index] == Side::UFirst;
    carrier_seam(carrier, edge, axis, first, budget)?;
    ideal_seam(carrier, edge, first, budget)
}
fn seam_axis(
    edge: &EdgeUse,
    relation: &'static str,
    actual: &'static str,
) -> CheckResult<martensite_tkmath::gp::Axis1> {
    let SpaceCurve::Line(axis) = &edge.curve else {
        return Err(invalid(edge.owner, relation, actual));
    };
    Ok(*axis)
}
fn carrier_seam<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    edge: &EdgeUse,
    axis: martensite_tkmath::gp::Axis1,
    first: bool,
    budget: &mut Budget<'_, F>,
) -> CheckResult<()> {
    let p = edge.pcurve.location();
    let d = edge.pcurve.direction();
    let poses = [carrier.edge.placement];
    let point = pole(carrier.curve, first)?;
    let second = carrier_target(carrier, point, [p.y(), d.y()], &poses);
    chains::require(
        ClosedFaceQueryContext::new(edge.owner, ClosedFaceRelation::CarrierSeam),
        chains::line(axis, edge.placement),
        second,
        budget,
    )
}
fn ideal_seam<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    edge: &EdgeUse,
    first: bool,
    budget: &mut Budget<'_, F>,
) -> CheckResult<()> {
    let axis = seam_axis(edge, "ideal seam relation", "non-line")?;
    let poses = [carrier.strip.placement];
    let second = ideal_seam_target(carrier, edge, first, &poses)?;
    chains::require(
        ClosedFaceQueryContext::new(edge.owner, ClosedFaceRelation::CanonicalSeam),
        chains::line(axis, edge.placement),
        second,
        budget,
    )
}
fn ideal_seam_target<'a>(
    carrier: &Carrier<'_>,
    edge: &EdgeUse,
    first: bool,
    poses: &'a [MetricEmbedding<3>],
) -> CheckResult<MetricCurveChain<'a, 2>> {
    let p = edge.pcurve.location();
    let d = edge.pcurve.direction();
    Ok(chains::sweep(
        pole(&carrier.strip.basis, first)?,
        carrier.strip.direction,
        [p.y(), d.y()],
        0.,
        poses,
    ))
}
fn vertices<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    edge: &EdgeUse,
    index: usize,
    budget: &mut Budget<'_, F>,
) -> CheckResult<()> {
    let poses = [carrier.edge.placement];
    for vertex in &edge.vertices {
        let corner = carrier.boundary.corners[index][vertex.side];
        vertex_relation(carrier, vertex, corner, &poses, budget)?;
    }
    Ok(())
}
fn vertex_relation<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    vertex: &VertexUse,
    corner: usize,
    poses: &[MetricEmbedding<3>],
    budget: &mut Budget<'_, F>,
) -> CheckResult<()> {
    let second = vertex_target(carrier, corner, poses)?;
    chains::require(
        ClosedFaceQueryContext::new(vertex.owner, ClosedFaceRelation::CarrierVertex),
        chains::point(vertex.point, vertex.placement),
        second,
        budget,
    )
}
fn vertex_target<'a>(
    carrier: &Carrier<'_>,
    corner: usize,
    poses: &'a [MetricEmbedding<3>],
) -> CheckResult<MetricCurveChain<'a, 2>> {
    let level = carrier.boundary.bounds[if corner < 2 { 2 } else { 3 }];
    Ok(carrier_target(
        carrier,
        pole(carrier.curve, corner == 0 || corner == 3)?,
        [level, 0.],
        poses,
    ))
}
fn pole(curve: &StoredSplineInput, first: bool) -> CheckResult<[f64; 3]> {
    let point = if first {
        curve.poles().first()
    } else {
        curve.poles().last()
    };
    point
        .copied()
        .ok_or_else(|| invalid(0, "carrier endpoint", "absent"))
}
