// SPDX-License-Identifier: GPL-2.0-only
//! Positive contact Hessians in the full chart and on each directed own trim.
use super::interval::{Range, add, derivative, mul, positive};
use super::{CylindricalTrimQuery, EndBounds};

pub(super) fn qualify(
    bounds: EndBounds,
    query: CylindricalTrimQuery<'_>,
    directions: &[[f64; 2]],
    contact: f64,
) -> Option<()> {
    let collar = Collar::new(bounds, query.radius, contact)?;
    collar.full_chart(bounds.motion)?;
    let metric = Metric::capture(query.pcurve, query.radius, directions)?;
    collar.own_trim(bounds, metric)
}

struct Collar {
    radius: f64,
    anchor: f64,
    reach: f64,
    angle_from_anchor: f64,
    angle_from_comparator: f64,
    strength: f64,
}

impl Collar {
    fn new(bounds: EndBounds, radius: f64, contact: f64) -> Option<Self> {
        let anchor = add(bounds.error, mul(bounds.prefix, bounds.motion));
        let reach = add(contact, anchor);
        let strength = (bounds.rate / mul(2., bounds.radius_ceiling)).next_down();
        Some(Self {
            radius,
            anchor,
            reach,
            angle_from_anchor: angle_upper(bounds.radius_floor, anchor)?,
            angle_from_comparator: angle_upper(radius, reach)?,
            strength,
        })
    }

    fn full_chart(&self, motion: f64) -> Option<()> {
        let angular_pad = add(self.angle_from_anchor, self.angle_from_comparator);
        let distance = add(
            add(self.anchor, self.reach),
            mul(self.radius, self.angle_from_comparator),
        );
        let radial = (self.strength - mul(motion, angular_pad)).next_down();
        // In (depth,R*u) the determinant is (D·e_r)^2 + |D|^2 (S-C)·e_r/R.
        // Height contributes a unit block because the ruled side has common height.
        let curvature = (distance / self.radius).next_up();
        let determinant = difference(square_lower(radial), mul(mul(motion, motion), curvature));
        let trace = add(mul(motion, motion), add(1., curvature));
        (positive(radial) && positive(determinant) && positive((determinant / trace).next_down()))
            .then_some(())
    }

    fn own_trim(&self, bounds: EndBounds, metric: Metric) -> Option<()> {
        let distance = self.trace_travel(&metric);
        let angle = add(self.angle_from_anchor, mul(metric.angular_speed, distance));
        let radial = difference(self.strength, mul(bounds.motion, angle));
        let tangent = mul(
            (bounds.radius_ceiling / bounds.radius_floor).next_up(),
            bounds.tangent_error,
        );
        let coupling = mul(
            add(tangent, mul(bounds.motion, angle)),
            mul(self.radius, metric.angular_speed),
        );
        let curvature = mul(
            add(self.anchor, mul(metric.speed_upper, distance)),
            metric.acceleration,
        );
        let trace_second = difference(square_lower(metric.speed_lower), curvature);
        let determinant = difference(
            (square_lower(radial) * trace_second).next_down(),
            mul(coupling, coupling),
        );
        let trace = add(
            mul(bounds.motion, bounds.motion),
            add(mul(metric.speed_upper, metric.speed_upper), curvature),
        );
        (positive(radial)
            && positive(trace_second)
            && positive(determinant)
            && positive((determinant / trace).next_down()))
        .then_some(())
    }

    fn trace_travel(&self, metric: &Metric) -> f64 {
        let [angular, axial] = metric.direction.map(f64::abs);
        add(
            mul(angular, mul(self.radius, self.angle_from_comparator)),
            mul(axial, self.reach),
        )
    }
}

fn difference(a: f64, b: f64) -> f64 {
    (a - b).next_down()
}

fn square_lower(value: f64) -> f64 {
    (value * value).next_down()
}

fn angle_upper(radius: f64, reach: f64) -> Option<f64> {
    let gap = difference(radius, reach);
    positive(gap).then(|| (reach / gap).next_up())
}

struct Metric {
    direction: [f64; 2],
    speed_lower: f64,
    speed_upper: f64,
    angular_speed: f64,
    acceleration: f64,
}

impl Metric {
    fn capture(controls: &[[f64; 2]], radius: f64, candidates: &[[f64; 2]]) -> Option<Self> {
        let first = developed_velocity(controls, radius);
        let second = derivative(&first);
        let [a, b] = [controls.first()?, controls.last()?];
        let chord = [radius * (b[0] - a[0]), b[1] - a[1]];
        let (direction, margin) = choose_direction(&first, candidates, chord)?;
        let norm = norm_upper(direction);
        let [x, z] = graph_speeds(&first, direction);
        let angular_speed = (x / radius).next_up();
        let lower_cube = (square_lower(margin) * margin).next_down();
        let planar_curvature =
            (mul(norm, determinant_upper(&first, &second)) / lower_cube).next_up();
        Some(Self {
            direction,
            speed_lower: (1. / norm).next_down(),
            speed_upper: add(x, z),
            angular_speed,
            acceleration: add(
                planar_curvature,
                mul(radius, mul(angular_speed, angular_speed)),
            ),
        })
    }
}

fn choose_direction(
    first: &[[Range; 2]],
    candidates: &[[f64; 2]],
    chord: [f64; 2],
) -> Option<([f64; 2], f64)> {
    let axes = [[1., 0.], [-1., 0.], [0., 1.], [0., -1.], chord];
    candidates
        .iter()
        .copied()
        .chain(axes)
        .filter_map(|direction| {
            let norm = norm_upper(direction);
            if !positive(norm) {
                return None;
            }
            let direction = direction.map(|value| value / norm);
            let margin = minimum_projection(first, direction);
            positive(margin).then_some((direction, margin))
        })
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
}

pub(super) fn projection_margin(controls: &[[f64; 2]], radius: f64, direction: [f64; 2]) -> f64 {
    minimum_projection(&developed_velocity(controls, radius), direction)
}

fn developed_velocity(controls: &[[f64; 2]], radius: f64) -> Vec<[Range; 2]> {
    let controls = controls
        .iter()
        .map(|[u, z]| [Range::point(*u).scale(radius), Range::point(*z)])
        .collect::<Vec<_>>();
    derivative(&controls)
}

fn determinant_upper(first: &[[Range; 2]], second: &[[Range; 2]]) -> f64 {
    first
        .iter()
        .flat_map(|a| {
            second.iter().map(|b| {
                a[0].product(b[1])
                    .difference(a[1].product(b[0]))
                    .magnitude()
            })
        })
        .fold(0., f64::max)
}

fn minimum_projection(first: &[[Range; 2]], direction: [f64; 2]) -> f64 {
    first
        .iter()
        .map(|sample| projection_lower(*sample, direction))
        .fold(f64::INFINITY, f64::min)
}

fn projection_lower([x, y]: [Range; 2], direction: [f64; 2]) -> f64 {
    x.scale(direction[0]).sum(y.scale(direction[1])).0[0]
}

fn graph_speeds(first: &[[Range; 2]], direction: [f64; 2]) -> [f64; 2] {
    first.iter().fold([0_f64; 2], |upper, p| {
        let lower = projection_lower(*p, direction);
        std::array::from_fn(|axis| upper[axis].max((p[axis].magnitude() / lower).next_up()))
    })
}

fn norm_upper([x, z]: [f64; 2]) -> f64 {
    add(mul(x, x), mul(z, z)).sqrt().next_up()
}
