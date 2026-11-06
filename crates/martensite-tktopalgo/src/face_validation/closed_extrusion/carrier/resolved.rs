//! The new wrapper shares geometry gates but never publishes an earlier exact-policy certificate.
use super::*;
use crate::face_validation::closed_extrusion::resolved::{self, *};
pub(in crate::face_validation::closed_extrusion) struct PreparedCarrier<'a> {
    carrier: Carrier<'a>,
}
pub(in crate::face_validation::closed_extrusion) fn geometry<'a, F: FnMut() -> bool>(
    strip: &'a Strip,
    boundary: &'a Boundary,
    report: &mut ToleranceResolvedFaceReport,
    budget: &mut Budget<'_, F>,
) -> Result<PreparedCarrier<'a>, ToleranceResolvedFaceCompletion> {
    let mut carrier = reference(strip, boundary).map_err(ToleranceResolvedFaceCompletion::Face)?;
    carrier.direction =
        direction::select(&carrier, 3).map_err(ToleranceResolvedFaceCompletion::Face)?;
    relations::certify_resolved(&carrier, budget).map_err(ToleranceResolvedFaceCompletion::Face)?;
    let axis = planes::certify(&carrier, budget).map_err(ToleranceResolvedFaceCompletion::Face)?;
    resolved_metrics::certify(&carrier, budget, &mut report.errors)
        .map_err(ToleranceResolvedFaceCompletion::Face)?;
    report.transverse_coordinate = Some(axis);
    Ok(PreparedCarrier { carrier })
}
pub(in crate::face_validation::closed_extrusion) fn certify<F: FnMut() -> bool>(
    prepared: &PreparedCarrier<'_>,
    report: &mut ToleranceResolvedFaceReport,
    budget: &mut Budget<'_, F>,
) -> ResolvedResult<()> {
    let c = &prepared.carrier;
    let proof = resolved::support::prove(
        c.curve,
        c.direction,
        [c.boundary.bounds[2], c.boundary.bounds[3]],
        c.edge.owner,
        budget,
    )?;
    resolved::support::retain(&mut report.carrier, proof, c.edge.owner, "actual carrier")?;
    // The exact nonzero common height functional excludes cap/interior interactions.
    resolved_components::certify(c, report, budget)
}
