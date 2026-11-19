// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(in crate::predicates::roots::metric_bounds) fn domain_bits(
    domain: &ConicalChartDomainEvidence,
) -> Vec<u64> {
    super::super::boundary_chart::ChartBinding::capture(domain).diagnostic_bits()
}
pub(super) fn domain_bits_selected(
    domain: &ConicalChartDomainEvidence,
    refinement: ConicalPrimitiveRefinement,
) -> Vec<u64> {
    super::super::boundary_chart::ChartBinding::capture_selected(domain, refinement)
        .diagnostic_bits()
}
pub(super) fn pair_bits(pair: &ConicalTrimPairQuery<'_>) -> Vec<u64> {
    let mut bits = vec![
        u64::from(pair.options.fractional_bits),
        u64::from(pair.options.max_operations),
        pair.options.behavior as u64,
    ];
    for query in pair.queries() {
        query_bits(query, &mut bits);
    }
    bits
}
fn query_bits(query: ConicalTrimQuery<'_>, bits: &mut Vec<u64>) {
    bits.extend([
        query.carrier.radius.to_bits(),
        query.carrier.axial_origin.to_bits(),
        query.carrier.semi_angle.to_bits(),
        query.spatial_limit.to_bits(),
    ]);
    bits.push(query.curve.poles.len() as u64);
    bits.extend(query.curve.poles.iter().flatten().map(|v| v.to_bits()));
    bits.push(u64::from(query.curve.weights.is_some()));
    if let Some(weights) = query.curve.weights {
        bits.push(weights.len() as u64);
        bits.extend(weights.iter().map(|v| v.to_bits()));
    }
    bits.push(query.pcurve.len() as u64);
    bits.extend(query.pcurve.iter().flatten().map(|v| v.to_bits()));
}
