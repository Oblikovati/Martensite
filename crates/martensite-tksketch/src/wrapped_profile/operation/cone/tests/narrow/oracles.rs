//! Exact integer Green integration checks authored controls without using the cone integrators.
use super::*;
pub(super) fn area_times_480(profile: &SketchProfile, contour: &ProfileLoop) -> i128 {
    contour
        .uses
        .iter()
        .map(|source| {
            let curve = profile
                .curves
                .iter()
                .find(|curve| id(curve) == source.curve)
                .unwrap();
            let coefficients = power_coefficients(&controls(profile, curve));
            let area = integrate_cross_product(coefficients);
            if source.reversed { -area } else { area }
        })
        .sum()
}
pub(super) fn id(curve: &ProfileCurve) -> ProfileCurveId {
    match curve {
        ProfileCurve::Line { id, .. } | ProfileCurve::Spline { id, .. } => *id,
        _ => panic!("expected one polynomial O. curve"),
    }
}
pub(super) fn controls(profile: &SketchProfile, curve: &ProfileCurve) -> Vec<[f64; 2]> {
    match curve {
        ProfileCurve::Line { start, end, .. } => {
            [start, end].map(|id| vertex_point(profile, *id)).to_vec()
        }
        ProfileCurve::Spline {
            poles,
            weights,
            degree,
            ..
        } => {
            assert!(weights.is_none());
            assert_eq!(*degree, 3);
            poles.clone()
        }
        _ => panic!("expected one unweighted polynomial O. curve"),
    }
}
fn vertex_point(profile: &SketchProfile, id: ProfileVertexId) -> [f64; 2] {
    profile
        .vertices
        .iter()
        .find(|vertex| vertex.id == id)
        .unwrap()
        .point
}
fn power_coefficients(poles: &[[f64; 2]]) -> [[i128; 4]; 2] {
    std::array::from_fn(|axis| {
        let values: Vec<_> = poles.iter().map(|point| twice_exact(point[axis])).collect();
        match values.as_slice() {
            [a, b] => [*a, b - a, 0, 0],
            [a, b, c, d] => [*a, 3 * (b - a), 3 * (a - 2 * b + c), -a + 3 * b - 3 * c + d],
            _ => panic!("poles={}; expected line or cubic", poles.len()),
        }
    })
}
fn twice_exact(value: f64) -> i128 {
    let scaled = 2. * value;
    assert!(scaled.is_finite() && scaled.abs() < 1024.);
    assert_eq!(scaled, scaled.trunc());
    scaled as i128
}
fn integrate_cross_product([x, y]: [[i128; 4]; 2]) -> i128 {
    let mut coefficients = [0_i128; 6];
    for i in 0..4 {
        for j in 1..4 {
            coefficients[i + j - 1] += (x[i] * y[j] - y[i] * x[j]) * j as i128;
        }
    }
    // Twice-scaled coordinates and Green's half factor give denominator8;
    // lcm(1..=6)=60 then makes every polynomial antiderivative exact.
    coefficients
        .iter()
        .enumerate()
        .map(|(n, value)| value * (60 / (n as i128 + 1)))
        .sum()
}
