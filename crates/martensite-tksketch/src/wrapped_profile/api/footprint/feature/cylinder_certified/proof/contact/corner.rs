// SPDX-License-Identifier: GPL-2.0-only
//! Positive directed derivative of the squared corner distance outside its small core.
use super::MetricBezier;
use super::interval::{Range, add, derivative, hull, mul, positive, prefix};

type Jet = [Range; 3];

pub(super) struct Arm<'a> {
    pub(super) near: MetricBezier<'a, 3>,
    pub(super) far: MetricBezier<'a, 3>,
    pub(super) target: &'a [[f64; 2]],
    pub(super) radius: f64,
    pub(super) windows: [f64; 2],
    pub(super) depth: f64,
    pub(super) reverse: bool,
    pub(super) anchor: [f64; 3],
    pub(super) gap: f64,
    pub(super) floor: f64,
}

pub(super) fn qualify(arm: Arm<'_>) -> Option<()> {
    let source = Rational::capture(&arm, false)?;
    let offset = Rational::capture(&arm, true)?;
    let target = target_velocity(&arm).map(|axis| axis.scale(-1.));
    let ruling = offset.position.map(|axis| axis.scale(arm.depth));
    let velocity =
        std::array::from_fn(|i| ruling[i].sum(offset.velocity[i].product(Range([0., arm.depth]))));
    let displacement = [source.chord, target, ruling];
    let rates = [source.velocity, target, velocity];
    let gain = positive_form(displacement, rates)?;
    let rate = rates.into_iter().map(magnitude).fold(0., add);
    let extent = displacement.into_iter().map(magnitude).fold(0., add);
    let core_radius = (mul(mul(2., arm.gap), rate) / gain).next_up();
    let core = add(arm.gap, mul(core_radius, extent));
    (core.is_finite() && core < arm.floor).then_some(())
}

fn positive_form(a: [Jet; 3], b: [Jet; 3]) -> Option<f64> {
    let diagonal: [f64; 3] = std::array::from_fn(|i| dot(a[i], b[i]).0[0]);
    if !diagonal.into_iter().all(positive) {
        return None;
    }
    let cross = negative_crosses(a, b, diagonal)?;
    // Amortize each normalized negative cross term using 2xy <= x^2+y^2.
    let gain = diagonal
        .into_iter()
        .zip(cross)
        .map(|(d, c)| ((1. - mul(0.5, c)).next_down() * d).next_down())
        .fold(f64::INFINITY, f64::min);
    positive(gain).then_some(gain)
}

fn negative_crosses(a: [Jet; 3], b: [Jet; 3], diagonal: [f64; 3]) -> Option<[f64; 3]> {
    let mut cross = [0.; 3];
    for [i, j] in [[0, 1], [0, 2], [1, 2]] {
        let lower = dot(a[i], b[j]).sum(dot(a[j], b[i])).0[0];
        let denominator = (diagonal[i] * diagonal[j]).next_down().sqrt().next_down();
        if !positive(denominator) || !lower.is_finite() {
            return None;
        }
        let negative = ((-lower).max(0.) / denominator).next_up();
        cross[i] = add(cross[i], negative);
        cross[j] = add(cross[j], negative);
    }
    Some(cross)
}

fn dot(a: Jet, b: Jet) -> Range {
    a.into_iter()
        .zip(b)
        .map(|(a, b)| a.product(b))
        .fold(Range::point(0.), Range::sum)
}

fn magnitude(vector: Jet) -> f64 {
    vector.into_iter().map(Range::magnitude).fold(0., add)
}

struct Rational {
    position: Jet,
    velocity: Jet,
    chord: Jet,
}

impl Rational {
    fn capture(arm: &Arm<'_>, offset: bool) -> Option<Self> {
        let controls = prefix(homogeneous(arm, offset), arm.windows[0], arm.reverse);
        let source = hull(&controls);
        let speed = hull(&derivative(&controls));
        let weight = source[3];
        if !positive(weight.0[0]) {
            return None;
        }
        let position = std::array::from_fn(|axis| source[axis].divide(weight));
        let velocity = std::array::from_fn(|axis| {
            speed[axis]
                .difference(position[axis].product(speed[3]))
                .divide(weight)
        });
        let chord = if offset {
            // Only the position and derivative of the ruling enter the mixed term.
            [Range::point(0.); 3]
        } else {
            chord_hull(&controls, arm.anchor, weight)
        };
        Some(Self {
            position,
            velocity,
            chord,
        })
    }
}

fn homogeneous(arm: &Arm<'_>, offset: bool) -> Vec<[Range; 4]> {
    arm.near
        .poles
        .iter()
        .enumerate()
        .map(|(i, pole)| {
            let weight = Range::point(arm.near.weights.map_or(1., |weights| weights[i]));
            let [x, y, z] = std::array::from_fn(|axis| {
                let coordinate = Range::point(pole[axis]);
                let value = if offset {
                    Range::point(arm.far.poles[i][axis]).difference(coordinate)
                } else {
                    coordinate
                };
                value.product(weight)
            });
            [x, y, z, weight]
        })
        .collect()
}

fn chord_hull(controls: &[[Range; 4]], anchor: [f64; 3], weight: Range) -> Jet {
    let degree = controls.len() - 1;
    // The constant coefficient of N-anchor*W vanishes exactly. Division by
    // u moves coefficient i to i-1 and multiplies it by degree/i.
    let controls = controls
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, p)| {
            let factor = Range::point(degree as f64).divide(Range::point(i as f64));
            std::array::from_fn(|axis| p[axis].difference(p[3].scale(anchor[axis])).product(factor))
        })
        .collect::<Vec<_>>();
    hull(&controls).map(|value| value.divide(weight))
}

fn target_velocity(arm: &Arm<'_>) -> Jet {
    let controls = arm.target.iter().map(|p| p.map(Range::point)).collect();
    let uv = prefix(controls, arm.windows[1], !arm.reverse);
    let [angular, height] = hull(&derivative(&uv));
    let angle_change = hull(&uv)[0].difference(uv[0][0]).magnitude();
    let radial = stock_direction(arm, angle_change);
    let tangential = angular.scale(arm.radius);
    // The same box bounds the outgoing chord quotient by integrating the
    // velocity over the interval from the exact shared corner.
    [
        radial[1].product(tangential).scale(-1.),
        radial[0].product(tangential),
        height,
    ]
}

fn stock_direction(arm: &Arm<'_>, angle_change: f64) -> [Range; 2] {
    let anchor_error = Range([(-arm.gap).next_down(), arm.gap.next_up()]);
    let drift = Range([(-angle_change).next_down(), angle_change.next_up()]);
    [arm.anchor[0], arm.anchor[1]].map(|coordinate| {
        Range::point(coordinate)
            .sum(anchor_error)
            .divide(Range::point(arm.radius))
            .sum(drift)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn near_orthogonal_columns_retain_a_positive_retraction_margin() {
        let columns =
            [[1., 0., 0.], [-0.2, 1., 0.], [0., 0.1, 1.]].map(|column| column.map(Range::point));
        assert!(positive_form(columns, columns).unwrap() > 0.7);
    }

    #[test]
    fn a_parallel_reversal_does_not_issue_a_strict_margin() {
        let columns =
            [[1., 0., 0.], [-1., 0., 0.], [0., 0., 1.]].map(|column| column.map(Range::point));
        assert!(positive_form(columns, columns).is_none());
    }
}
