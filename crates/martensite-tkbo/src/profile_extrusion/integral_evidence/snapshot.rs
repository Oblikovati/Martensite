// SPDX-License-Identifier: GPL-2.0-only
use super::*;

pub(super) fn bits(profile: &SketchProfile) -> Vec<u64> {
    let mut bits = Vec::new();
    visit(profile, |value| bits.push(value.to_bits()));
    bits
}

pub(super) fn matches(profile: &SketchProfile, expected: &[u64]) -> bool {
    let mut expected = expected.iter().copied();
    let mut matches = true;
    visit(profile, |value| {
        matches &= expected.next() == Some(value.to_bits())
    });
    matches && expected.next().is_none()
}

fn visit(profile: &SketchProfile, mut value: impl FnMut(f64)) {
    let plane = profile.plane;
    plane
        .origin
        .into_iter()
        .chain(plane.x_direction)
        .chain(plane.normal)
        .chain(profile.vertices.iter().flat_map(|vertex| vertex.point))
        .for_each(&mut value);
    for curve in &profile.curves {
        visit_curve(curve, &mut value);
    }
}

fn visit_curve(curve: &ProfileCurve, mut value: impl FnMut(f64)) {
    match curve {
        ProfileCurve::Spline {
            poles,
            weights,
            knots,
            ..
        } => poles
            .iter()
            .flatten()
            .chain(weights.iter().flatten())
            .chain(knots)
            .copied()
            .for_each(value),
        ProfileCurve::Line { .. } => {}
        ProfileCurve::Arc { center, .. } => center.iter().copied().for_each(value),
        ProfileCurve::Circle { center, radius, .. } => {
            center.iter().copied().for_each(&mut value);
            value(*radius);
        }
        ProfileCurve::Ellipse {
            center,
            major_axis,
            minor_radius,
            ..
        }
        | ProfileCurve::EllipticArc {
            center,
            major_axis,
            minor_radius,
            ..
        } => {
            center
                .iter()
                .chain(major_axis)
                .copied()
                .for_each(&mut value);
            value(*minor_radius);
        }
    }
}
