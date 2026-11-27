//! Exact source-float arithmetic separates clipping signs from upload rounding.
use num_rational::BigRational;
use num_traits::{One, ToPrimitive, Zero};
pub(super) type Rational = BigRational;
pub(super) type Point = [Rational; 4];
pub(super) fn scalar(value: f32) -> Rational {
    // Validated draw components are finite; conversion preserves their exact binary value.
    Rational::from_float(value).expect("finite validated f32 must have an exact ratio")
}
pub(super) fn transform(vertex: &crate::RenderVertex, matrix: &[f32; 16]) -> Point {
    let input = [
        vertex.position[0],
        vertex.position[1],
        vertex.position[2],
        vertex.weight,
    ];
    std::array::from_fn(|row| {
        (0..4)
            .map(|column| scalar(matrix[column * 4 + row]) * scalar(input[column]))
            .sum()
    })
}
pub(super) fn interpolate(a: &Point, b: &Point, parameter: &Rational) -> Point {
    // One exact parameter preserves the common incidence of every coordinate.
    std::array::from_fn(|i| &a[i] + (&b[i] - &a[i]) * parameter)
}
pub(super) fn clip(a: &Point, b: &Point, clamp: bool) -> Option<[Rational; 2]> {
    let mut interval = [Rational::zero(), Rational::one()];
    for axis in 0..if clamp { 2 } else { 3 } {
        for sign in [-1, 1] {
            let fa = &a[3] + &a[axis] * Rational::from_integer(sign.into());
            let fb = &b[3] + &b[axis] * Rational::from_integer(sign.into());
            if !clip_plane(&fa, &fb, &mut interval) {
                return None;
            }
        }
    }
    Some(interval)
}
fn clip_plane(a: &Rational, b: &Rational, interval: &mut [Rational; 2]) -> bool {
    if a < &Rational::zero() && b < &Rational::zero() {
        return false;
    }
    if (a < &Rational::zero()) != (b < &Rational::zero()) {
        let crossing = -a / (b - a);
        let endpoint = usize::from(a >= &Rational::zero());
        if endpoint == 0 {
            interval[0] = interval[0].clone().max(crossing);
        } else {
            interval[1] = interval[1].clone().min(crossing);
        }
    }
    interval[0] <= interval[1]
}
pub(super) fn axis(a: &Point, b: &Point, extent: [u32; 2]) -> Option<u32> {
    let dx = (&b[0] / &b[3] - &a[0] / &a[3]) * Rational::from_integer(extent[0].into());
    let dy = (&b[1] / &b[3] - &a[1] / &a[3]) * Rational::from_integer(extent[1].into());
    let xx = &dx * &dx;
    let yy = &dy * &dy;
    if (&xx + &yy).is_zero() {
        return None;
    }
    // This owned rule is not asserted equivalent to every implicit derivative implementation.
    Some(
        if (&xx - &yy) * Rational::from_integer(1000.into()) > xx + yy {
            1
        } else {
            2
        },
    )
}
pub(super) fn upload(points: &[Point; 2]) -> Option<[[f32; 4]; 2]> {
    let scale = points
        .iter()
        .flat_map(|p| p.iter())
        .map(|v| if v < &Rational::zero() { -v } else { v.clone() })
        .max()?;
    if scale.is_zero() {
        return None;
    }
    let converted: [[Option<f32>; 4]; 2] =
        std::array::from_fn(|n| std::array::from_fn(|i| (&points[n][i] / &scale).to_f32()));
    let result: [[f32; 4]; 2] =
        std::array::from_fn(|n| std::array::from_fn(|i| converted[n][i].unwrap_or(f32::NAN)));
    result
        .iter()
        .all(|p| p[3] >= 2. * f32::MIN_POSITIVE && p.iter().all(|v| v.is_finite()))
        .then_some(result)
}
pub(super) fn pixel_error(
    original: &[Point; 2],
    upload: &[[f32; 4]; 2],
    extent: [u32; 2],
) -> Rational {
    (0..2)
        .flat_map(|n| (0..2).map(move |axis| (n, axis)))
        .map(|(n, axis)| coordinate_error(&original[n], &upload[n], axis, extent[axis]))
        .max()
        .unwrap_or_else(Rational::zero)
}
pub(super) fn upward(value: &Rational) -> Option<f64> {
    let rounded = value.to_f64()?;
    if !rounded.is_finite() {
        return None;
    }
    if Rational::from_float(rounded)? >= *value {
        return Some(rounded);
    }
    Some(f64::from_bits(rounded.to_bits() + 1))
}

fn coordinate_error(original: &Point, upload: &[f32; 4], axis: usize, extent: u32) -> Rational {
    let delta = scalar(upload[axis]) / scalar(upload[3]) - &original[axis] / &original[3];
    let absolute = if delta < Rational::zero() {
        -delta
    } else {
        delta
    };
    absolute * Rational::from_integer(extent.into()) / Rational::from_integer(2.into())
}
