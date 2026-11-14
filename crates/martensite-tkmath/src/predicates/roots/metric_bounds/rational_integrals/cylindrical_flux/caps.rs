// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::metric_bounds::polynomial_integrals::arithmetic;

pub(super) fn integrate(
    span: CylindricalFluxSpan<'_>,
    radii: [f64; 2],
    options: CylindricalFluxOptions,
    index: usize,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<ExactRatio> {
    let mut total = ExactRatio::from_finite(0.);
    for (depth, radius) in radii.into_iter().enumerate() {
        let area = cap_area(span.cap_pcurves[depth], options, cancelled).map_err(|cause| {
            CylindricalFluxFailure::Cap {
                span: index,
                depth,
                cause: Box::new(cause),
            }
        })?;
        let radius = ExactRatio::from_finite(radius);
        let factor = radius
            .multiply(&radius)
            .divide(&ExactRatio::from_finite(3.));
        let flux = area.multiply(&factor);
        total = if depth == 0 {
            total.subtract(&flux)
        } else {
            total.add(&flux)
        };
    }
    Ok(total)
}
fn cap_area(
    poles: &[[f64; 2]],
    options: CylindricalFluxOptions,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<ExactRatio, PolynomialIntegralFailure> {
    let options = PolynomialIntegralOptions {
        behavior_version: options.behavior_version,
        max_degree: options.max_degree,
        max_curves: 1,
        absolute_error_limit: options.absolute_error_limit,
    };
    arithmetic::curve_integral(
        MetricCurve::Bezier(MetricBezier {
            poles,
            weights: None,
        }),
        options,
        cancelled,
    )
}
