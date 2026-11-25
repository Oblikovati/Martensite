// SPDX-License-Identifier: GPL-2.0-only
use super::{AxisDirection, CoordinateAxis, DualNumber, Equation, LineAngle, ResidualSource};
use martensite_cmath::{atan2, sin, sincos};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

type Vector = [f64; 3];

#[test]
fn normalized_axis_and_plane_rows_match_centered_differences_with_independent_scalar_geometry() {
    for axis in [CoordinateAxis::X, CoordinateAxis::Y, CoordinateAxis::Z] {
        let index = axis.index();
        for (sign, direction) in [
            (1.0, AxisDirection::Positive),
            (-1.0, AxisDirection::Negative),
        ] {
            let mut d = [2.5, 3., 4.];
            d[index] *= sign;
            verify(
                Equation::Axis(axis, direction),
                &coordinates(d, None),
                |points| {
                    let d = displacement(points, 0);
                    let len = length(d);
                    vec![d[(index + 1) % 3] / len, d[(index + 2) % 3] / len]
                },
            );
        }
    }
    for normal in [[1., 2., -2.], [-1., -2., 2.]] {
        verify(
            Equation::Plane(normal),
            &coordinates([1., 2., 3.], None),
            |points| {
                let d = displacement(points, 0);
                vec![dot(d, normal) / (length(d) * length(normal))]
            },
        );
    }
}

#[test]
fn perpendicular_and_equal_length_rows_match_centered_differences_in_both_endpoint_orders() {
    for (first, second) in [
        ([1., 2., 3.], [4., -2., 0.]),
        ([-1., -2., -3.], [3., 2., 1.]),
        ([1., 2., 3.], [-4., 2., 1.]),
    ] {
        let points = coordinates(first, Some(second));
        verify(Equation::Perpendicular, &points, |points| {
            let (a, b) = (displacement(points, 0), displacement(points, 6));
            vec![dot(a, b) / (length(a) * length(b))]
        });
        verify(Equation::EqualLength { scale: 0.125 }, &points, |points| {
            vec![(length(displacement(points, 0)) - length(displacement(points, 6))) / 8.]
        });
    }
}

#[test]
fn directed_angle_rows_match_centered_differences_at_both_azimuth_signs_and_near_both_endpoints() {
    for theta in [0.01, FRAC_PI_4, FRAC_PI_2, PI - 0.01] {
        let target = LineAngle::new(theta).unwrap();
        for sign in [1., -1.] {
            let (sine, cosine) = sincos(theta);
            let points = coordinates([2., 0., 0.], Some([4. * cosine, sign * 4. * sine, 0.]));
            verify(
                Equation::Angle {
                    target,
                    minimum_sine: 2e-8,
                },
                &points,
                |points| {
                    let (a, b) = (displacement(points, 0), displacement(points, 6));
                    let actual = atan2(length(cross(a, b)), dot(a, b));
                    // The independent angle form avoids subtracting nearly equal cosines.
                    vec![
                        -2. * sin((actual + theta) * 0.5) * sin((actual - theta) * 0.5)
                            / sin(theta),
                    ]
                },
            );
        }
    }
}

fn verify(source: Equation, points: &[f64], expected: impl Fn(&[f64]) -> Vec<f64>) {
    let seeded = DualNumber::seed(points).unwrap();
    let rows = source.evaluate(&seeded).unwrap();
    let values = expected(points);
    assert_eq!(rows.len(), values.len());
    for (row, expected) in rows.iter().zip(values) {
        assert!((row.value() - expected).abs() <= 3e-12 * expected.abs().max(1.));
        assert_eq!(row.gradient().len(), points.len());
    }
    let mut left = points.to_vec();
    let mut right = points.to_vec();
    for (column, &coordinate) in points.iter().enumerate() {
        left[column] = coordinate - 1e-6;
        right[column] = coordinate + 1e-6;
        let spacing = right[column] - left[column];
        for (row, (a, b)) in rows
            .iter()
            .zip(expected(&left).into_iter().zip(expected(&right)))
        {
            let finite_difference = (b - a) / spacing;
            let automatic = row.gradient()[column];
            assert!(
                (automatic - finite_difference).abs() <= 2e-7 * finite_difference.abs().max(1.),
                "column {column}: AD {automatic}, centered difference {finite_difference}"
            );
        }
        left[column] = coordinate;
        right[column] = coordinate;
    }
}

fn coordinates(first: Vector, second: Option<Vector>) -> Vec<f64> {
    let a = [1., -3., 2.];
    let b = [-4., 6., 5.];
    let mut out = a.to_vec();
    out.extend(a.into_iter().zip(first).map(|(p, d)| p + d));
    if let Some(second) = second {
        out.extend(b);
        out.extend(b.into_iter().zip(second).map(|(p, d)| p + d));
    }
    out
}

fn displacement(points: &[f64], index: usize) -> Vector {
    std::array::from_fn(|i| points[index + i + 3] - points[index + i])
}

fn length(v: Vector) -> f64 {
    dot(v, v).sqrt()
}
fn dot(a: Vector, b: Vector) -> f64 {
    a.into_iter().zip(b).map(|(a, b)| a * b).sum()
}
fn cross([a, b, c]: Vector, [d, e, f]: Vector) -> Vector {
    [b * f - c * e, c * d - a * f, a * e - b * d]
}

#[test]
fn incidence_rows_differentiate_the_active_transverse_frame_and_mixed_units() {
    use super::{FrameReference, ParallelDirection};

    let reference = [0., 1., 0.];
    let points = coordinates([1., 2., 3.], Some([-3., -2., -4.]));
    verify(
        Equation::PointOnLine {
            scale: 0.125,
            reference: FrameReference::Axis(1),
        },
        &points[..9],
        |points| incidence_expected(points, reference, true, false),
    );
    for independent_offset in [true, false] {
        let root_half = std::f64::consts::FRAC_1_SQRT_2;
        let normal = [root_half, -root_half, 0.];
        verify(
            Equation::Colinear {
                direction: ParallelDirection::Opposite,
                reference: FrameReference::Normal(normal),
                scale: 0.125,
                independent_offset,
            },
            &points,
            |points| incidence_expected(points, normal, independent_offset, true),
        );
    }
}

fn incidence_expected(
    points: &[f64],
    reference: Vector,
    offset: bool,
    direction: bool,
) -> Vec<f64> {
    let delta = displacement(points, 0);
    let u = delta.map(|d| d / length(delta));
    // Two independently constructed transverse unit axes. A frozen basis would fail
    // the centered differences when varying the first line's coordinates.
    let second = cross(u, reference);
    let second = second.map(|d| d / length(second));
    let first = cross(second, u);
    let mut rows = Vec::new();
    if direction {
        let other = displacement(points, 6);
        rows.extend([first, second].map(|r| dot(r, other) / length(other)));
    }
    if offset {
        let v = std::array::from_fn(|i| points[i + 6] - points[i]);
        rows.extend([first, second].map(|r| dot(r, v) / 8.));
    }
    rows
}
