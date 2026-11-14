//! Transport flux itself; determinant drift is part of the retained/ideal discrepancy.
use super::*;
pub(super) fn transport(result: &mut WrappedNormalResult, plan: &Plan) -> Checked<()> {
    let determinant = plan.budget.frame().determinant_bounds();
    let mut totals = publication_budget::FamilyVolumeBounds::default();
    for (region, report) in &mut result.reports {
        let member = retained(&result.family.family, *region)?;
        transport_member(&mut totals, member, report, determinant)?;
    }
    error::budget(totals.enforce(result.options.volume))?;
    result.budget.volume = reports::volume_report(&totals);
    Ok(())
}
fn transport_member(
    totals: &mut publication_budget::FamilyVolumeBounds,
    member: &pipeline::RetainedMember,
    report: &mut WrappedMemberReport,
    determinant: [f64; 2],
) -> Checked<()> {
    let flux = member.measures.comparison.flux_certificate();
    report.measures.boundary_flux = product(report.measures.boundary_flux, determinant)?;
    let representative = product(reports::range([flux.representative(); 2]), determinant)?;
    let discrepancy = difference(report.measures.boundary_flux, report.measures.ideal_volume)?;
    report.measures.retained_ideal_discrepancy = reports::bound(discrepancy);
    add_totals(
        totals,
        member,
        determinant,
        discrepancy,
        difference(representative, report.measures.ideal_volume)?,
    )
}
fn add_totals(
    totals: &mut publication_budget::FamilyVolumeBounds,
    member: &pipeline::RetainedMember,
    determinant: [f64; 2],
    discrepancy: f64,
    representative: f64,
) -> Checked<()> {
    use publication_budget::Quantity as Q;
    totals.discrepancy = add(totals.discrepancy, discrepancy, Q::RetainedIdeal)?;
    totals.representative_error = add(
        totals.representative_error,
        representative,
        Q::RepresentativeIdeal,
    )?;
    let comparison = &member.measures.comparison;
    let integration = error::budget(publication_budget::scale_upper(
        comparison.flux_certificate().absolute_error_bound(),
        determinant[1],
        Q::Integration,
    ))?;
    totals.integration_error = add(totals.integration_error, integration, Q::Integration)?;
    totals.profile_error = add(
        totals.profile_error,
        comparison.normal_certificate().volume_discrepancy_bound(),
        Q::AuthoredIdeal,
    )?;
    Ok(())
}
fn add(a: f64, b: f64, quantity: publication_budget::Quantity) -> Checked<f64> {
    error::budget(publication_budget::add_upper(a, b, quantity))
}
fn product(range: WrappedCertifiedRange, determinant: [f64; 2]) -> Checked<WrappedCertifiedRange> {
    let mut lower = f64::INFINITY;
    let mut upper = f64::NEG_INFINITY;
    for value in [range.lower, range.upper] {
        for factor in determinant {
            let bounds = scalar_product(value, factor)?;
            lower = lower.min(bounds[0]);
            upper = upper.max(bounds[1]);
        }
    }
    Ok(reports::range([lower, upper]))
}
fn scalar_product(value: f64, factor: f64) -> Checked<[f64; 2]> {
    if value == 0. || factor == 1. {
        return Ok([value; 2]);
    }
    let product = value * factor;
    finite([product.next_down(), product.next_up()])
}
fn finite(actual: [f64; 2]) -> Checked<[f64; 2]> {
    if !actual.iter().all(|value| value.is_finite()) || actual[0] > actual[1] {
        return Err(fail(PlacementFailure::Interval { actual }));
    }
    Ok(actual)
}
fn difference(actual: WrappedCertifiedRange, ideal: WrappedCertifiedRange) -> Checked<f64> {
    let low = actual.lower - ideal.upper;
    let high = actual.upper - ideal.lower;
    let bound = low.abs().max(high.abs());
    let bound = if bound == 0. { 0. } else { bound.next_up() };
    Ok(finite([0., bound])?[1])
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn determinant_drift_changes_flux_even_when_canonical_discrepancy_is_zero() {
        let ideal = reports::range([8.; 2]);
        let actual = product(ideal, [63. / 64.; 2]).unwrap();
        assert!(actual.lower <= 7.875 && 7.875 <= actual.upper);
        assert!(difference(actual, ideal).unwrap() >= 0.125);
    }
    #[test]
    fn interval_transport_covers_negative_flux_and_rejects_overflow() {
        let actual = product(reports::range([-4., -2.]), [0.5, 2.]).unwrap();
        assert!(actual.lower <= -8. && actual.upper >= -1.);
        assert!(product(reports::range([f64::MAX; 2]), [2.; 2]).is_err());
    }
}
