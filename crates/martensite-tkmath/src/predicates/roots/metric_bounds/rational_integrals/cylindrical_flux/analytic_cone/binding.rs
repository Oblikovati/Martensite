// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) fn source_bits(query: AnalyticConicalFluxQuery<'_>) -> Vec<u64> {
    let mut bits = vec![query.spans.len() as u64];
    for carrier in query.carriers {
        bits.extend([
            carrier.radius.to_bits(),
            carrier.axial_origin.to_bits(),
            carrier.semi_angle.to_bits(),
        ]);
    }
    for span in query.spans {
        span_bits(&mut bits, *span);
    }
    bits
}
fn span_bits(bits: &mut Vec<u64>, span: AnalyticConicalFluxSpan<'_>) {
    bits.extend([
        u64::from(span.side_reversed),
        u64::from(span.cap_reversed[0]),
        u64::from(span.cap_reversed[1]),
    ]);
    for curve in span.boundaries {
        curve_bits(bits, curve);
    }
    for poles in span.cap_pcurves {
        curve_bits(
            bits,
            MetricBezier {
                poles,
                weights: None,
            },
        );
    }
}
fn curve_bits<const DIM: usize>(bits: &mut Vec<u64>, curve: MetricBezier<'_, DIM>) {
    bits.push(curve.poles.len() as u64);
    bits.extend(curve.poles.iter().flatten().map(|value| value.to_bits()));
    bits.push(
        curve
            .weights
            .map_or(u64::MAX, |weights| weights.len() as u64),
    );
    bits.extend(
        curve
            .weights
            .into_iter()
            .flatten()
            .map(|value| value.to_bits()),
    );
}

pub(super) fn policy_bits(
    options: CylindricalFluxOptions,
    arithmetic: CylindricalProfileOptions,
) -> Vec<u64> {
    vec![
        u64::from(options.behavior_version),
        u64::from(options.max_degree),
        u64::from(options.max_spans),
        options.absolute_error_limit.to_bits(),
        u64::from(options.max_subintervals_per_span),
        u64::from(options.max_total_subintervals),
        u64::from(arithmetic.fractional_bits),
        u64::from(arithmetic.max_operations),
        arithmetic.behavior as u64,
    ]
}
