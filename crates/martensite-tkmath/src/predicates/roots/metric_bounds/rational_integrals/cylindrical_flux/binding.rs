// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) fn source_bits(query: CylindricalFluxQuery<'_>) -> Vec<u64> {
    let mut bits = vec![
        query.radii[0].to_bits(),
        query.radii[1].to_bits(),
        query.spans.len() as u64,
    ];
    for span in query.spans {
        span_bits(&mut bits, *span);
    }
    bits
}
fn span_bits(bits: &mut Vec<u64>, span: CylindricalFluxSpan<'_>) {
    bits.push(u64::from(span.side_reversed));
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
    bits.extend(curve.poles.iter().flatten().map(|v| v.to_bits()));
    bits.push(curve.weights.map_or(u64::MAX, |w| w.len() as u64));
    bits.extend(curve.weights.into_iter().flatten().map(|v| v.to_bits()));
}
