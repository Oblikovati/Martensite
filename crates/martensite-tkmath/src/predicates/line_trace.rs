// SPDX-License-Identifier: GPL-2.0-only
//! Outward comparison of stored affine line traces.
use crate::gp::{Line3, Point3, Transform3};

/// Enclose the Euclidean deviation of one trimmed line from authored finite endpoints.
/// Both segments are compared at the same normalized trim parameter. Nonfinite inputs
/// and an unrepresentable enclosure return infinity.
/// Example: `affine_line_trace_bound(line, [0., 1.], [start, end])`.
pub fn affine_line_trace_bound(line: Line3, parameters: [f64; 2], points: [Point3; 2]) -> f64 {
    if line
        .direction()
        .coordinates()
        .iter()
        .any(|coordinate| !coordinate.is_finite())
        || parameters.iter().any(|parameter| !parameter.is_finite())
        || [line.location(), points[0], points[1]].iter().any(|point| {
            point
                .coordinates()
                .iter()
                .any(|coordinate| !coordinate.is_finite())
        })
    {
        return f64::INFINITY;
    }
    // The difference from the authored segment is affine under the normalized trim.
    // Its L1 norm is bounded by the larger endpoint bound and bounds Euclidean error.
    let first = endpoint_bound(line, parameters[0], points[0]);
    let last = endpoint_bound(line, parameters[1], points[1]);
    first.max(last)
}

/// Enclose the Euclidean distance between two supplied model-space points.
/// Example: `point_distance_upper_bound(a, b)`.
pub fn point_distance_upper_bound(a: Point3, b: Point3) -> f64 {
    let mut bound = 0.0;
    for (&left, &right) in a.coordinates().iter().zip(b.coordinates()) {
        if !left.is_finite() || !right.is_finite() {
            return f64::INFINITY;
        }
        bound = add_up(bound, abs_difference(left, right));
    }
    bound
}

/// Enclose the pointwise error of a stored affine line trim after applying its location.
/// Example: `placed_affine_line_trace_bound(line, trim, location, authored)`.
pub fn placed_affine_line_trace_bound(
    line: Line3,
    parameters: [f64; 2],
    location: Transform3,
    authored: [Point3; 2],
) -> f64 {
    if location.is_identity() {
        return affine_line_trace_bound(line, parameters, authored);
    }
    let first = placed_endpoint(line, parameters[0], location, authored[0]);
    let last = placed_endpoint(line, parameters[1], location, authored[1]);
    first.max(last)
}

/// Enclose the distance of a stored point after applying its location.
/// Example: `placed_point_distance_upper_bound(local, location, authored)`.
pub fn placed_point_distance_upper_bound(
    local: Point3,
    location: Transform3,
    authored: Point3,
) -> f64 {
    if location.is_identity() {
        return point_distance_upper_bound(local, authored);
    }
    let local = local.coordinates().map(|value| [value, value]);
    placed_bound(local, location, authored)
}

fn placed_endpoint(line: Line3, t: f64, location: Transform3, authored: Point3) -> f64 {
    let local = std::array::from_fn(|axis| {
        let base = line.location().coordinates()[axis];
        let direction = line.direction().coordinates()[axis];
        product_sum([t, t], direction, [base, base])
    });
    placed_bound(local, location, authored)
}

fn placed_bound(local: [[f64; 2]; 3], location: Transform3, authored: Point3) -> f64 {
    let matrix = location.vectorial_part();
    let mut bound = 0.0;
    for (axis, row) in matrix.coefficients().iter().enumerate() {
        let base = location.translation_part().coordinates()[axis];
        let target = authored.coordinates()[axis];
        let mut component = [base, base];
        for (coefficient, interval) in row.iter().zip(local) {
            component = product_sum(interval, *coefficient, component);
        }
        if !target.is_finite() || component.iter().any(|end| !end.is_finite()) {
            return f64::INFINITY;
        }
        let error = abs_difference(component[0], target).max(abs_difference(component[1], target));
        bound = add_up(bound, error);
    }
    bound
}

fn product_sum(mut interval: [f64; 2], multiplier: f64, offset: [f64; 2]) -> [f64; 2] {
    if interval.iter().chain(&offset).any(|end| !end.is_finite()) || !multiplier.is_finite() {
        return [f64::NEG_INFINITY, f64::INFINITY];
    }
    if multiplier == 0.0 || interval == [0.0; 2] {
        return offset;
    }
    if multiplier.is_sign_negative() {
        interval.swap(0, 1);
    }
    [
        multiplier.mul_add(interval[0], offset[0]).next_down(),
        multiplier.mul_add(interval[1], offset[1]).next_up(),
    ]
}

fn endpoint_bound(line: Line3, t: f64, point: Point3) -> f64 {
    let origin = line.location();
    let direction = line.direction();
    let mut bound = 0.0;
    for ((&base, &dir), &target) in origin
        .coordinates()
        .iter()
        .zip(direction.coordinates())
        .zip(point.coordinates())
    {
        let component = component_bound(base, dir, t, target);
        bound = add_up(bound, component);
    }
    bound
}

fn component_bound(base: f64, direction: f64, t: f64, target: f64) -> f64 {
    if t == 0. || direction == 0. {
        return abs_difference(base, target);
    }
    // FMA performs one rounding. Neighboring floats enclose the exact affine value,
    // even if the unrounded product and origin nearly cancel.
    let rounded = direction.mul_add(t, base);
    if !rounded.is_finite() {
        return f64::INFINITY;
    }
    abs_difference(rounded.next_down(), target).max(abs_difference(rounded.next_up(), target))
}

fn abs_difference(left: f64, right: f64) -> f64 {
    if left == right {
        return 0.;
    }
    (left - right).abs().next_up()
}

fn add_up(a: f64, b: f64) -> f64 {
    if a == 0. || b == 0. {
        return a + b;
    }
    (a + b).next_up()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::{Direction3, Vector3};

    #[test]
    fn affine_enclosure_binds_endpoints_and_propagates_unavailable_arithmetic() {
        let start = Point3::new(2., 3., 4.);
        let last = Point3::new(7., 3., 4.);
        let line = Line3::new(start, Direction3::X);
        assert!(affine_line_trace_bound(line, [0., 5.], [start, last]) < 2e-15);
        assert!(affine_line_trace_bound(line, [0., 6.], [start, last]) >= 1.);
        assert_eq!(
            affine_line_trace_bound(line, [0., 5.], [Point3::new(f64::NAN, 3., 4.), last]),
            f64::INFINITY,
        );
        assert_eq!(point_distance_upper_bound(start, start), 0.);
        assert!(point_distance_upper_bound(start, last) >= 5.);
    }

    #[test]
    fn placed_enclosure_covers_exact_translation_and_detects_shifted_targets() {
        let start = Point3::default();
        let line = Line3::new(start, Direction3::X);
        let shift = Transform3::translation_vector(Vector3::new(-2., 3., 4.));
        let authored = [Point3::new(-2., 3., 4.), Point3::new(2., 3., 4.)];
        assert_eq!(
            placed_point_distance_upper_bound(start, shift, authored[0]),
            0.
        );
        let bound = placed_affine_line_trace_bound(line, [0., 4.], shift, authored);
        assert!(bound < 4e-15);
        let wrong = placed_affine_line_trace_bound(line, [0., 5.], shift, authored);
        assert!(wrong >= 1.);
    }
}
