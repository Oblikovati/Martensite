// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) fn bits(profile: &SketchProfile) -> Vec<u64> {
    let plane = profile.plane;
    let mut values: Vec<f64> = plane
        .origin
        .into_iter()
        .chain(plane.x_direction)
        .chain(plane.normal)
        .collect();
    values.extend(profile.vertices.iter().flat_map(|vertex| vertex.point));
    for curve in &profile.curves {
        append_curve(&mut values, curve);
    }
    values.into_iter().map(f64::to_bits).collect()
}

fn append_curve(values: &mut Vec<f64>, curve: &ProfileCurve) {
    match curve {
        ProfileCurve::Spline {
            poles,
            weights,
            knots,
            ..
        } => append_spline(values, poles, weights.as_deref(), knots),
        other => append_conic(values, other),
    }
}
fn append_conic(values: &mut Vec<f64>, curve: &ProfileCurve) {
    match curve {
        ProfileCurve::Line { .. } | ProfileCurve::Spline { .. } => {}
        ProfileCurve::Arc { center, .. } => values.extend(center),
        ProfileCurve::Circle { center, radius, .. } => {
            values.extend(center);
            values.push(*radius);
        }
        ProfileCurve::Ellipse { .. } | ProfileCurve::EllipticArc { .. } => {
            append_ellipse(values, curve)
        }
    }
}
fn append_ellipse(values: &mut Vec<f64>, curve: &ProfileCurve) {
    match curve {
        ProfileCurve::Ellipse {
            center,
            major_axis,
            minor_radius,
            ..
        } => append_ellipse_fields(values, center, major_axis, *minor_radius),
        ProfileCurve::EllipticArc {
            center,
            major_axis,
            minor_radius,
            ..
        } => append_ellipse_fields(values, center, major_axis, *minor_radius),
        _ => {}
    }
}
fn append_ellipse_fields(values: &mut Vec<f64>, center: &[f64; 2], axis: &[f64; 2], radius: f64) {
    values.extend(center);
    values.extend(axis);
    values.push(radius);
}

fn append_spline(
    values: &mut Vec<f64>,
    poles: &[[f64; 2]],
    weights: Option<&[f64]>,
    knots: &[f64],
) {
    values.extend(poles.iter().flatten());
    values.extend(weights.iter().flat_map(|weights| weights.iter()));
    values.extend(knots);
}
