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
fn direction<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    budget: &mut Budget<'_, F>,
) -> CheckResult<()> {
    let first = chains::vector(carrier.strip.direction, carrier.strip.placement);
    let second = chains::vector(carrier.strip.direction, carrier.edge.placement);
    chains::require(
        carrier.edge.owner,
        "same actual world extrusion direction",
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
    let SpaceCurve::Spline(curve) = &edge.curve else {
        return Err(invalid(edge.owner, "actual cap curve", "line"));
    };
    let level = carrier.boundary.bounds[if carrier.boundary.sides[index] == Side::VFirst {
        2
    } else {
        3
    }];
    let poses = [carrier.edge.placement];
    for (actual, pole) in curve.poles().iter().zip(carrier.curve.poles()) {
        let first = chains::point(*actual, edge.placement);
        let second = chains::sweep(
            *pole,
            carrier.strip.direction,
            [level, 0.],
            carrier.reference,
            &poses,
        );
        chains::require(
            edge.owner,
            "actual cap coefficient on carrier",
            first,
            second,
            budget,
        )?;
    }
    Ok(())
}
fn seam<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    edge: &EdgeUse,
    index: usize,
    budget: &mut Budget<'_, F>,
) -> CheckResult<()> {
    let SpaceCurve::Line(axis) = &edge.curve else {
        return Err(invalid(edge.owner, "actual seam curve", "spline"));
    };
    let first = carrier.boundary.sides[index] == Side::UFirst;
    let p = edge.pcurve.location();
    let d = edge.pcurve.direction();
    let poses = [carrier.edge.placement];
    let second = chains::sweep(
        pole(carrier.curve, first)?,
        carrier.strip.direction,
        [p.y(), d.y()],
        carrier.reference,
        &poses,
    );
    chains::require(
        edge.owner,
        "actual seam on boundary carrier",
        chains::line(*axis, edge.placement),
        second,
        budget,
    )?;
    ideal_seam(carrier, edge, first, budget)
}
fn ideal_seam<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    edge: &EdgeUse,
    first: bool,
    budget: &mut Budget<'_, F>,
) -> CheckResult<()> {
    let SpaceCurve::Line(axis) = &edge.curve else {
        return Err(invalid(edge.owner, "ideal seam relation", "non-line"));
    };
    let p = edge.pcurve.location();
    let d = edge.pcurve.direction();
    let poses = [carrier.strip.placement];
    let second = chains::sweep(
        pole(&carrier.strip.basis, first)?,
        carrier.strip.direction,
        [p.y(), d.y()],
        0.,
        &poses,
    );
    chains::require(
        edge.owner,
        "actual seam on canonical support",
        chains::line(*axis, edge.placement),
        second,
        budget,
    )
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
        let level = carrier.boundary.bounds[if corner < 2 { 2 } else { 3 }];
        let second = chains::sweep(
            pole(carrier.curve, corner == 0 || corner == 3)?,
            carrier.strip.direction,
            [level, 0.],
            carrier.reference,
            &poses,
        );
        chains::require(
            vertex.owner,
            "actual vertex on carrier corner",
            chains::point(vertex.point, vertex.placement),
            second,
            budget,
        )?;
    }
    Ok(())
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
