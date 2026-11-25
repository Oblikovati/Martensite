// SPDX-License-Identifier: GPL-2.0-only
use super::super::super::MetricCurve;
use super::IntegralProgramLoop;

pub(super) fn bits(loops: &[IntegralProgramLoop<'_>], face: bool) -> Vec<u64> {
    let mut bits = vec![u64::from(face), loops.len() as u64];
    for current in loops {
        bits.push(current.components.len() as u64);
        for component in current.components {
            component_bits(component, &mut bits);
        }
    }
    bits
}
fn component_bits(component: &[super::super::OrientedMetricCurve<'_>], bits: &mut Vec<u64>) {
    bits.push(component.len() as u64);
    for occurrence in component {
        bits.push(u64::from(occurrence.reversed));
        match occurrence.curve {
            MetricCurve::Line { origin, direction } => {
                bits.push(0);
                bits.extend(origin.into_iter().chain(direction).map(f64::to_bits));
            }
            MetricCurve::Bezier(curve) => bezier_bits(curve, bits),
        }
    }
}
fn bezier_bits(curve: super::super::super::MetricBezier<'_, 2>, bits: &mut Vec<u64>) {
    bits.extend([1, curve.poles.len() as u64]);
    bits.extend(curve.poles.iter().flatten().map(|v| v.to_bits()));
    bits.push(u64::from(curve.weights.is_some()));
    if let Some(weights) = curve.weights {
        bits.push(weights.len() as u64);
        bits.extend(weights.iter().map(|v| v.to_bits()));
    }
}

pub(super) fn same_options(
    left: super::IntegralProgramOptions,
    right: super::IntegralProgramOptions,
) -> bool {
    left == right
        && left.integration.absolute_error_limit.to_bits()
            == right.integration.absolute_error_limit.to_bits()
}
