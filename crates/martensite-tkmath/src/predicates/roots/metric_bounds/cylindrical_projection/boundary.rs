// SPDX-License-Identifier: GPL-2.0-only
//! Whole-interval validation of an original cylindrical meridian representation.
use super::{
    AxialProjectionLine, Checked, CylinderProjectionBranch as Branch,
    CylinderProjectionError as Failure, ProjectionCylinder, ProjectionVertex,
    affine::{Chart, Ratio, Vector, bounds, sub, zero},
    native_net::finite,
    trim::angle,
    verify::{abs, sqrt_bound},
};
use crate::predicates::roots::metric_bounds::placed_plane::{
    ExactPlacement, dot, exact_point, exact_vector,
};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// Bound a native cardinal meridian's positional consistency with the actual cylinder and vertices.
/// Example: `check_projection_meridian(cylinder, line, angle, vertices, allowance)?`.
pub fn check_projection_meridian(
    support: ProjectionCylinder,
    line: AxialProjectionLine,
    theta: f64,
    vertices: [ProjectionVertex; 2],
    allowance: f64,
) -> Checked<f64> {
    if !allowance.is_finite() || allowance < 0. {
        return Err(Failure::FittedLimits);
    }
    let chart = Chart::capture(support)?;
    let [a, b] = finite(&line.range)?;
    if a >= b {
        return Err(Failure::Trim);
    }
    let meridian = Meridian::capture(&chart, line, theta)?;
    let a = meridian.error(&chart, &a, vertices[0])?;
    let b = meridian.error(&chart, &b, vertices[1])?;
    let upper = bounds(&a.max(b))?[1];
    if upper > allowance {
        return Err(Failure::Trim);
    }
    Ok(upper)
}

/// Verify two actual native vertices against the original complete affine source range.
/// Example: `check_projection_line_vertices(line, vertices)?`.
pub fn check_projection_line_vertices(
    line: AxialProjectionLine,
    vertices: [ProjectionVertex; 2],
) -> Checked<()> {
    let [a, b] = finite(&line.range)?;
    if a >= b {
        return Err(Failure::Source);
    }
    let placement = ExactPlacement::capture(line.placement)?;
    let origin = placement.point(exact_point(line.line.location(), 0)?);
    let direction = placement.vector(exact_vector(*line.line.direction().coordinates(), 1)?);
    for (t, vertex) in [a, b].into_iter().zip(vertices) {
        let actual = ExactPlacement::capture(vertex.placement)?.point(finite(&vertex.point)?);
        let expected: Vector = std::array::from_fn(|i| origin[i].add(&direction[i].multiply(&t)));
        if actual != expected {
            return Err(Failure::Source);
        }
    }
    Ok(())
}

struct Meridian {
    origin: Vector,
    direction: Vector,
    cardinal: Vector,
    angle_transport: Ratio,
}

impl Meridian {
    fn capture(chart: &Chart, line: AxialProjectionLine, theta: f64) -> Checked<Self> {
        let placement = ExactPlacement::capture(line.placement)?;
        let origin = placement.point(exact_point(line.line.location(), 0)?);
        let direction = placement.vector(exact_vector(*line.line.direction().coordinates(), 1)?);
        let (branch, angular) = meridian_angle(theta)?;
        let radial = cardinal_radius(chart, branch);
        let cardinal = std::array::from_fn(|axis| chart.center[axis].add(&radial[axis]));
        let value = Ratio::from_finite(theta);
        let error = abs(value.subtract(&angular[0])).max(abs(value.subtract(&angular[1])));
        Ok(Self {
            origin,
            direction,
            cardinal,
            angle_transport: chart.radius.multiply(&error),
        })
    }

    fn error(&self, chart: &Chart, t: &Ratio, vertex: ProjectionVertex) -> Checked<Ratio> {
        let expected =
            std::array::from_fn(|axis| self.cardinal[axis].add(&chart.axes[2][axis].multiply(t)));
        let actual =
            std::array::from_fn(|axis| self.origin[axis].add(&self.direction[axis].multiply(t)));
        let endpoint = ExactPlacement::capture(vertex.placement)?.point(finite(&vertex.point)?);
        let root = distance(&expected, &actual)?.add(&self.angle_transport);
        Ok(root.add(&distance(&actual, &endpoint)?))
    }
}

fn distance(a: &Vector, b: &Vector) -> Checked<Ratio> {
    let delta = sub(a, b);
    let squared = dot(&delta, &delta);
    if zero(&squared) {
        return Ok(Ratio::from_finite(0.));
    }
    sqrt_bound(&squared).map(Ratio::from_finite)
}

fn cardinal_radius(chart: &Chart, branch: Branch) -> Vector {
    let (axis, sign) = match branch {
        Branch::PositiveU => (0, true),
        Branch::NegativeU => (0, false),
        Branch::PositiveV => (1, true),
        Branch::NegativeV => (1, false),
    };
    let radius = if sign {
        chart.radius.clone()
    } else {
        chart.radius.negated()
    };
    chart.axes[axis]
        .each_ref()
        .map(|direction| direction.multiply(&radius))
}

fn meridian_angle(value: f64) -> Checked<(Branch, [Ratio; 2])> {
    let branch = if value == 0. || value == TAU {
        Branch::PositiveU
    } else if value == FRAC_PI_2 {
        Branch::PositiveV
    } else if value == PI {
        Branch::NegativeU
    } else if value == 3. * FRAC_PI_2 {
        Branch::NegativeV
    } else {
        return Err(Failure::Trim);
    };
    if value == TAU {
        return Ok((
            branch,
            [PI, PI.next_up()].map(|q| Ratio::from_finite(q).multiply(&Ratio::from_finite(2.))),
        ));
    }
    Ok((branch, angle(branch)))
}
