//! A certified common height functional separates the caps from the open face interior.
use super::*;
pub(super) fn certify<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    budget: &mut Budget<'_, F>,
) -> CheckResult<usize> {
    let mut failures = std::array::from_fn(|_| ClosedFacePlaneRefusal::ZeroDirection);
    for axis in 0..3 {
        if let Some(cause) = candidate(carrier, axis, budget)? {
            failures[axis] = cause;
        } else {
            return Ok(axis);
        }
    }
    Err(ClosedFaceCompletion::Incomplete(
        ClosedFaceIncomplete::BoundaryPlane(failures),
    ))
}
fn candidate<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    axis: usize,
    budget: &mut Budget<'_, F>,
) -> CheckResult<Option<ClosedFacePlaneRefusal>> {
    if !transverse(carrier, axis, budget)? {
        return Ok(Some(ClosedFacePlaneRefusal::ZeroDirection));
    }
    if let Some(cause) = ideal_constant(carrier, axis, budget)? {
        return Ok(Some(cause));
    }
    carrier_level(carrier, axis, budget)
}
fn transverse<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    axis: usize,
    budget: &mut Budget<'_, F>,
) -> CheckResult<bool> {
    let projection = [chains::projection(axis)];
    let mut first = chains::vector(carrier.strip.direction, carrier.strip.placement);
    first.placements = &projection;
    let second = chains::point([0.; 3], super::super::placements::identity());
    Ok(chains::compare(first, second, budget)? != CurveIdentityDecision::Identical)
}
fn ideal_constant<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    axis: usize,
    budget: &mut Budget<'_, F>,
) -> CheckResult<Option<ClosedFacePlaneRefusal>> {
    let projection = [chains::projection(axis)];
    let reference = reference_pole(carrier)?;
    for (coefficient, pole) in carrier.strip.basis.poles().iter().enumerate() {
        let mut first = chains::point(*pole, carrier.strip.placement);
        let mut second = chains::point(reference, carrier.strip.placement);
        first.placements = &projection;
        second.placements = &projection;
        let actual = chains::compare(first, second, budget)?;
        if actual != CurveIdentityDecision::Identical {
            return Ok(Some(ClosedFacePlaneRefusal::IdealVariation {
                coefficient,
                actual,
            }));
        }
    }
    Ok(None)
}
fn carrier_level<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    axis: usize,
    budget: &mut Budget<'_, F>,
) -> CheckResult<Option<ClosedFacePlaneRefusal>> {
    let projection = [chains::projection(axis)];
    let placements = [carrier.strip.placement, projection[0]];
    let reference = reference_pole(carrier)?;
    for (coefficient, pole) in carrier.curve.poles().iter().enumerate() {
        let mut first = chains::point(*pole, carrier.edge.placement);
        first.placements = &projection;
        let second = chains::sweep(
            reference,
            carrier.strip.direction,
            [carrier.reference, 0.],
            0.,
            &placements,
        );
        let actual = chains::compare(first, second, budget)?;
        if actual != CurveIdentityDecision::Identical {
            return Ok(Some(ClosedFacePlaneRefusal::CarrierLevel {
                coefficient,
                actual,
            }));
        }
    }
    Ok(None)
}
fn reference_pole(carrier: &Carrier<'_>) -> CheckResult<[f64; 3]> {
    carrier
        .strip
        .basis
        .poles()
        .first()
        .copied()
        .ok_or_else(|| invalid(0, "ideal basal reference", "missing coefficient"))
}
