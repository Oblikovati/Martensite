// SPDX-License-Identifier: GPL-2.0-only
//! Quantitative single-center composition shares legacy topology and metric proofs.
use super::super::chart_domain::bounds;
use super::*;
#[cfg(test)]
mod diagnostics_tests;
#[cfg(test)]
mod tests;
/// Complete reference-body inputs using the quantitative PositiveX chart domain.
pub struct ConicalChartReferenceQuery<'a> {
    /// Source-ordered selected members and executed ideal join proofs.
    pub members: &'a [ConicalReferenceMember<'a>],
    /// Complete original nonadjacent separation.
    pub clearance: &'a CertifiedCylindricalClearance,
    /// Executed source-bound quantitative chart domain.
    pub domain: &'a ConicalChartDomainEvidence,
    /// Complete unsplit originals used by the domain proof.
    pub domain_spans: &'a [&'a [[f64; 2]]],
    /// Original requested physical spatial error.
    pub spatial_limit: f64,
    /// Remaining global arithmetic resources.
    pub options: ConeClearanceOptions,
}
/// Certify the same q/L body under a quantitative chart; `certify_conical_chart_reference(query, || false)?`.
pub fn certify_conical_chart_reference(
    query: ConicalChartReferenceQuery<'_>,
    cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalReferenceEvidence, ConicalReferenceError> {
    certify_conical_chart_reference_with_refinement(
        query,
        ConicalPrimitiveRefinement::UnsplitV1,
        cancelled,
    )
}
/// Certify a reference body with explicit primitive-refinement authority.
/// Example: `certify_conical_chart_reference_with_refinement(query, refinement, || false)?`.
pub fn certify_conical_chart_reference_with_refinement(
    query: ConicalChartReferenceQuery<'_>,
    refinement: ConicalPrimitiveRefinement,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalReferenceEvidence, ConicalReferenceError> {
    let mut query = ReferenceInputs::from(&query);
    query.refinement = refinement;
    binding::admit(&query)?;
    let mut arithmetic = Arithmetic::new(sector_api::policy(query.options), &mut cancelled);
    let result = geometry::certify(&query, &mut arithmetic)
        .map_err(|cause| error(cause, arithmetic.operations()))?;
    Ok(binding::evidence(&query, result, arithmetic.operations()))
}
impl ConicalReferenceEvidence {
    /// Rebind the executed quantitative domain and every reference input; `proof.matches_chart(&query)`.
    pub fn matches_chart(&self, query: &ConicalChartReferenceQuery<'_>) -> bool {
        self.matches_chart_with_refinement(query, ConicalPrimitiveRefinement::UnsplitV1)
    }
    /// Rebind complete reference inputs and selected refinement; `proof.matches_chart_with_refinement(&query, refinement)`.
    pub fn matches_chart_with_refinement(
        &self,
        query: &ConicalChartReferenceQuery<'_>,
        refinement: ConicalPrimitiveRefinement,
    ) -> bool {
        let mut input = ReferenceInputs::from(query);
        input.refinement = refinement;
        binding::matches(self, &input)
    }
}
pub(super) fn bind_domain(query: &ReferenceInputs<'_>) -> Result<(), ConicalReferenceFailure> {
    let ReferenceDomain::Chart(domain) = query.sector else {
        return Ok(());
    };
    if domain.options().fractional_bits != query.options.fractional_bits {
        return Err(binding_failure(
            "same fixed-grid precision for domain and reference",
        ));
    }
    let input = domain_input(query, domain);
    if !domain.matches_with_version(&input, domain.version()) {
        return Err(binding_failure("actual chart domain and common lift"));
    }
    Ok(())
}
fn domain_input<'a>(
    query: &ReferenceInputs<'a>,
    domain: &ConicalChartDomainEvidence,
) -> ConicalChartDomainQuery<'a> {
    let first = query.members[0].spans[0].query;
    ConicalChartDomainQuery {
        spans: query.sector_spans,
        semi_angle: first.semi_angle,
        chart_origin: first.chart_origin,
        fit_center: first.fit_center,
        developed_turn: first.developed_turn,
        physical_turn: first.physical_turn,
        options: domain.options(),
    }
}
pub(super) fn angular(
    query: &ReferenceInputs<'_>,
    domain: &ConicalChartDomainEvidence,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), ConicalReferenceFailure> {
    let mut values = Vec::new();
    for value in query
        .members
        .iter()
        .flat_map(|m| m.spans)
        .flat_map(|s| s.query.angular_fit)
    {
        values.push(*value);
    }
    let [lower, upper] = coefficient_range(&values)?;
    let range = fitted_range(lower, upper, arithmetic)?;
    bounds::width(&range, arithmetic).map_err(project)?;
    developed_branch(&range, domain, arithmetic)
}
fn coefficient_range(values: &[f64]) -> Result<[f64; 2], ConicalReferenceFailure> {
    if values.is_empty() || values.iter().any(|v| !v.is_finite()) {
        return Err(binding_failure(
            "nonempty finite common-chart q coefficients",
        ));
    }
    Ok([
        values.iter().copied().fold(f64::INFINITY, f64::min),
        values.iter().copied().fold(f64::NEG_INFINITY, f64::max),
    ])
}
fn fitted_range(
    lower: f64,
    upper: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, ConicalReferenceFailure> {
    let first = super::super::angle::initial_angle([1., lower], arithmetic)
        .map_err(|cause| bounds::angle_failure(cause, [1., lower]))
        .map_err(project)?;
    let last = super::super::angle::initial_angle([1., upper], arithmetic)
        .map_err(|cause| bounds::angle_failure(cause, [1., upper]))
        .map_err(project)?;
    let range = Interval {
        lower: first.lower,
        upper: last.upper,
    };
    Ok(arithmetic.fraction(&range, 2, 1)?)
}
fn developed_branch(
    range: &Interval,
    domain: &ConicalChartDomainEvidence,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), ConicalReferenceFailure> {
    let phase = arithmetic.multiply(range, &domain.sine)?;
    let pi = crate::predicates::roots::certificate_arithmetic::circular_constants::pi(arithmetic)?;
    let half = arithmetic.fraction(&pi, 1, 2)?;
    if phase.lower <= -half.lower.clone() || phase.upper >= half.lower {
        let limits = Interval {
            lower: -half.lower.clone(),
            upper: half.lower,
        };
        let expected = ConicalChartDomainRequirement::StrictlyInside(exact_enclosure(
            &limits,
            arithmetic.policy().fractional_bits,
        ));
        return Err(project(bounds::unresolved(
            "common ideal development in the PositiveX principal halfplane",
            &phase,
            expected,
            arithmetic,
        )));
    }
    Ok(())
}
pub(super) fn homotopy(
    domain: &ConicalChartDomainEvidence,
    displacement: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), ConicalReferenceFailure> {
    let minimum = arithmetic.subtract(&domain.minimum_x, displacement)?;
    let minimum = Interval {
        lower: minimum.lower.clone(),
        upper: minimum.lower,
    };
    bounds::positive_lower(
        &minimum,
        "positive projected halfplane across the homotopy",
        arithmetic,
    )
    .map_err(project)?;
    let denominator = arithmetic.multiply(&minimum, &domain.sine)?;
    let margin = arithmetic.divide(displacement, &denominator)?;
    let expanded = Interval {
        lower: &domain.physical.lower - &margin.upper,
        upper: &domain.physical.upper + &margin.upper,
    };
    bounds::width(&expanded, arithmetic).map_err(project)?;
    bounds::chart_membership(&expanded, arithmetic).map_err(project)
}
fn project(cause: ConicalChartDomainFailure) -> ConicalReferenceFailure {
    match cause {
        ConicalChartDomainFailure::Arithmetic(cause) => ConicalReferenceFailure::Arithmetic(cause),
        ConicalChartDomainFailure::Trigonometric(cause) => {
            ConicalReferenceFailure::Trigonometric(cause)
        }
        ConicalChartDomainFailure::Unresolved { bound } => project_bound(*bound),
        _ => binding_failure("executed quantitative PositiveX chart proof"),
    }
}

fn project_bound(bound: ConicalChartDomainBound) -> ConicalReferenceFailure {
    let zero = DyadicBound::new(0.into(), 0);
    match bound.expected {
        ConicalChartDomainRequirement::PositiveLower => reference_bound(
            "negated projected halfplane lower bound across the homotopy",
            zero.difference(&bound.actual[0]),
            zero,
        ),
        ConicalChartDomainRequirement::WidthStrictlyBelow(limit) => reference_bound(
            bound.field,
            bound.actual[1].difference(&bound.actual[0]),
            limit,
        ),
        ConicalChartDomainRequirement::StrictlyInside(limits) => {
            inside_bound(bound.field, bound.actual, limits)
        }
    }
}
fn inside_bound(
    field: &'static str,
    actual: [DyadicBound; 2],
    limits: [DyadicBound; 2],
) -> ConicalReferenceFailure {
    if actual[0] <= limits[0] {
        let zero = DyadicBound::new(0.into(), 0);
        return reference_bound(
            "negated lower endpoint for strict chart membership",
            zero.difference(&actual[0]),
            zero.difference(&limits[0]),
        );
    }
    reference_bound(field, actual[1].clone(), limits[1].clone())
}
fn reference_bound(
    field: &'static str,
    actual: DyadicBound,
    limit: DyadicBound,
) -> ConicalReferenceFailure {
    ConicalReferenceFailure::Bound {
        field,
        actual: projected_upper(&actual),
        limit: projected_lower(&limit),
    }
}
fn projected_upper(value: &DyadicBound) -> f64 {
    match value.binary64_bounds() {
        Some([_, upper]) => upper,
        None if value < &DyadicBound::new(0.into(), 0) => -f64::MAX,
        None => f64::INFINITY,
    }
}
fn projected_lower(value: &DyadicBound) -> f64 {
    match value.binary64_bounds() {
        Some([lower, _]) => lower,
        None if value < &DyadicBound::new(0.into(), 0) => f64::NEG_INFINITY,
        None => f64::MAX,
    }
}
