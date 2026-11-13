//! One angular polynomial and one weight vector serve both physical depths.
use martensite_cmath::{cos, sin};

pub(super) struct RationalPoles {
    pub points: Vec<[f64; 3]>,
    pub weights: Vec<f64>,
}

fn choose(n: usize, k: usize) -> f64 {
    (0..k).fold(1., |value, i| value * (n - i) as f64 / (i + 1) as f64)
}

fn product(a: &[f64], b: &[f64]) -> Vec<f64> {
    let (m, n) = (a.len() - 1, b.len() - 1);
    let mut result = vec![0.; m + n + 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            result[i + j] += x * y * choose(m, i) * choose(n, j) / choose(m + n, i + j);
        }
    }
    result
}

fn half_angle(s: f64, radius: f64) -> f64 {
    let half = s / (2. * radius);
    sin(half) / cos(half)
}

fn fit(source: &[[f64; 2]], radius: f64) -> [f64; 4] {
    let n = source.len() - 1;
    let a = half_angle(source[0][0], radius);
    let b = half_angle(source[n][0], radius);
    let d0 = n as f64 * (source[1][0] - source[0][0]) / radius;
    let d1 = n as f64 * (source[n][0] - source[n - 1][0]) / radius;
    [a, a + d0 * (1. + a * a) / 6., b - d1 * (1. + b * b) / 6., b]
}

pub(super) fn coefficients(source: &[[f64; 2]], radius: f64) -> Vec<[f64; 4]> {
    let t = fit(source, radius);
    let square = product(&t, &t);
    let weights: Vec<_> = square.iter().map(|q| 1. + q).collect();
    let z: Vec<_> = source.iter().map(|p| p[1]).collect();
    let zw = product(&z, &weights);
    let elevate = vec![1.; z.len()];
    let x = product(&square.iter().map(|q| 1. - q).collect::<Vec<_>>(), &elevate);
    let y = product(&product(&t, &[2.; 4]), &elevate);
    let w = product(&weights, &elevate);
    (0..w.len()).map(|i| [x[i], y[i], zw[i], w[i]]).collect()
}

pub(super) fn at_radius(coefficients: &[[f64; 4]], radius: f64) -> RationalPoles {
    let points = coefficients
        .iter()
        .map(|[x, y, z, w]| [radius * (x / w), radius * (y / w), z / w])
        .collect();
    RationalPoles {
        points,
        weights: coefficients.iter().map(|p| p[3]).collect(),
    }
}

pub(super) fn endpoint([s, z]: [f64; 2], support: f64, radius: f64) -> [f64; 3] {
    let t = half_angle(s, support);
    let w = 1. + t * t;
    [radius * ((1. - t * t) / w), radius * ((2. * t) / w), z]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evaluate(poles: &[[f64; 4]], u: f64) -> [f64; 4] {
        let mut work = poles.to_vec();
        for level in (1..work.len()).rev() {
            for i in 0..level {
                work[i] = std::array::from_fn(|k| (1. - u) * work[i][k] + u * work[i + 1][k]);
            }
        }
        work[0]
    }

    #[test]
    fn rational_identity_and_shared_depth_scaling_match_closed_form() {
        let coefficients = coefficients(&[[0., 1.], [0.4, 2.], [0.8, 1.]], 10.);
        let [inner, outer] = [9., 12.].map(|r| at_radius(&coefficients, r));
        assert_eq!(inner.weights, outer.weights);
        for i in 0..=32 {
            let [x, y, z, w] = evaluate(&coefficients, i as f64 / 32.);
            assert!((x * x + y * y - w * w).abs() < 3e-15);
            assert!((z / w - (1. + 2. * (i as f64 / 32.) * (1. - i as f64 / 32.))).abs() < 2e-15);
        }
    }
}
