//! Independently injective boundary carriers supplement the canonical face surface.
mod chains;
mod planes;
mod relations;
use super::{
    boundary::{Boundary, Side},
    boundary_report::ClosedFaceBoundaryReport,
    budget::*,
    decode::*,
    report::ClosedFaceReport,
    types::*,
};
use martensite_tkg3d::geom::StoredSplineInput;
use martensite_tkmath::predicates::roots::{closed_extrusion::*, metric_bounds::*};
pub(super) struct Carrier<'a> {
    strip: &'a Strip,
    boundary: &'a Boundary,
    edge: &'a EdgeUse,
    curve: &'a StoredSplineInput,
    index: usize,
    reference: f64,
    direction: [f64; 3],
}
pub(super) fn certify<F: FnMut() -> bool>(
    strip: &Strip,
    boundary: &Boundary,
    report: &mut ClosedFaceReport,
    budget: &mut Budget<'_, F>,
) -> CheckResult<()> {
    let carrier = reference(strip, boundary)?;
    report.boundary = Some(initial(&carrier));
    let input = admit(&carrier)?;
    relations::certify(&carrier, budget)?;
    let axis = planes::certify(&carrier, budget)?;
    let witness = report.boundary.as_mut().unwrap();
    witness.transverse_coordinate = Some(axis);
    witness.relations_verified = true;
    let proof = super::support_proof::prove(input, budget);
    let completion = completion(proof.completion());
    witness.proof = Some(proof);
    completion
}
fn reference<'a>(strip: &'a Strip, boundary: &'a Boundary) -> CheckResult<Carrier<'a>> {
    let index = reference_index(boundary)?;
    let edge = &strip.edges[index];
    let curve = reference_curve(edge)?;
    Ok(Carrier {
        strip,
        boundary,
        edge,
        curve,
        index,
        reference: boundary.bounds[2],
        direction: strip.direction,
    })
}
fn reference_index(boundary: &Boundary) -> CheckResult<usize> {
    boundary
        .sides
        .iter()
        .position(|side| *side == Side::VFirst)
        .ok_or_else(|| invalid(0, "actual reference cap", "missing VFirst side"))
}
fn reference_curve(edge: &EdgeUse) -> CheckResult<&StoredSplineInput> {
    let SpaceCurve::Spline(curve) = &edge.curve else {
        return Err(unsupported(
            edge.owner,
            "actual carrier",
            "line",
            "full matching spline cap",
        ));
    };
    Ok(curve)
}

fn admit(carrier: &Carrier<'_>) -> CheckResult<ClosedQuadraticExtrusionInput> {
    let interval = [carrier.boundary.bounds[2], carrier.boundary.bounds[3]];
    let input = carrier
        .curve
        .clone()
        .into_closed_extrusion_input(carrier.direction, interval)
        .map_err(|error| admission_error(carrier, &error))?;
    super::operation::require_captured(input)
}
fn admission_error(carrier: &Carrier<'_>, error: &impl std::fmt::Debug) -> ClosedFaceCompletion {
    unsupported(
        carrier.edge.owner,
        "actual carrier input",
        format!("{error:?}"),
        "closed clamped quadratic with positive constant weights",
    )
}

fn initial(carrier: &Carrier<'_>) -> ClosedFaceBoundaryReport {
    ClosedFaceBoundaryReport {
        edge_owner: carrier.edge.owner,
        occurrence: carrier.index,
        reference_parameter: carrier.reference,
        transverse_coordinate: None,
        relations_verified: false,
        proof: None,
    }
}
fn completion(completion: &ClosedExtrusionCompletion) -> CheckResult<()> {
    match completion {
        ClosedExtrusionCompletion::Certified => Ok(()),
        ClosedExtrusionCompletion::Incomplete(cause) => Err(ClosedFaceCompletion::Incomplete(
            ClosedFaceIncomplete::BoundarySupport(cause.clone()),
        )),
        ClosedExtrusionCompletion::NotExcluded(contact) => Err(ClosedFaceCompletion::Incomplete(
            ClosedFaceIncomplete::BoundaryProjection(*contact),
        )),
    }
}
