// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) fn source_bits(query: ConicalReferenceVolumeQuery<'_>) -> Vec<u64> {
    let mut bits = query_bits(query);
    for span in query.spans {
        bits.push(u64::from(span.reversed));
        for values in [span.angular_fit, span.slant_numerator, span.slant_weights] {
            bits.push(values.len() as u64);
            bits.extend(values.iter().map(|v| v.to_bits()));
        }
    }
    bits
}
fn query_bits(query: ConicalReferenceVolumeQuery<'_>) -> Vec<u64> {
    let o = query.options;
    vec![
        query.spans.len() as u64,
        query.semi_angle.to_bits(),
        query.chart_origin.to_bits(),
        query.fit_center.to_bits(),
        query.depths[0].to_bits(),
        query.depths[1].to_bits(),
        u64::from(query.outward),
        u64::from(o.behavior_version),
        u64::from(o.max_degree),
        u64::from(o.max_spans),
        o.absolute_error_limit.to_bits(),
        u64::from(o.max_subintervals_per_span),
        u64::from(o.max_total_subintervals),
        u64::from(query.arithmetic.fractional_bits),
        u64::from(query.arithmetic.max_operations),
        query.arithmetic.behavior as u64,
    ]
}
