//! Candidate homogeneous conical edges; publication needs separate storage certificates.
use super::angular::{RationalPoles, product};
use martensite_cmath::{cos, sin};

#[derive(Clone, Copy)]
pub(super) struct ConeFit<'a> {
    pub(super) tangent: &'a [f64],
    pub(super) slant_numerator: &'a [f64],
    pub(super) slant_weights: &'a [f64],
    pub(super) semi_angle: f64,
    pub(super) chart_center: f64,
    pub(super) outward: bool,
}
#[derive(Debug)]
pub(super) enum ConeFitFailure {
    Shape {
        tangent: usize,
        numerator: usize,
        weights: usize,
    },
    Nonfinite {
        role: &'static str,
        index: usize,
        value: f64,
    },
    InvalidAngle(f64),
    NonpositiveWeight {
        index: usize,
        value: f64,
    },
    Overflow {
        index: usize,
        coordinate: usize,
        value: f64,
    },
}

impl std::fmt::Display for ConeFitFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shape {
                tangent,
                numerator,
                weights,
            } => shape_message(f, *tangent, *numerator, *weights),
            other => other.detail_message(f),
        }
    }
}

impl ConeFitFailure {
    fn detail_message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nonfinite { role, index, value } => nonfinite_message(f, role, *index, *value),
            Self::InvalidAngle(value) => {
                write!(f, "semi-angle={value}; expected finite angle in (0, pi/2)")
            }
            Self::NonpositiveWeight { index, value } => write!(
                f,
                "weight[{index}]={value}; expected finite positive weight"
            ),
            Self::Overflow {
                index,
                coordinate,
                value,
            } => overflow_message(f, *index, *coordinate, *value),
            Self::Shape {
                tangent,
                numerator,
                weights,
            } => shape_message(f, *tangent, *numerator, *weights),
        }
    }
}
fn nonfinite_message(
    f: &mut std::fmt::Formatter<'_>,
    role: &str,
    index: usize,
    value: f64,
) -> std::fmt::Result {
    write!(f, "{role}[{index}]={value}; expected finite")
}

fn shape_message(
    f: &mut std::fmt::Formatter<'_>,
    tangent: usize,
    numerator: usize,
    weights: usize,
) -> std::fmt::Result {
    write!(
        f,
        "tangent={tangent}, numerator={numerator}, weights={weights}; expected tangent 1..=4, equal slant counts 1..=10, and combined count >=3"
    )
}
fn overflow_message(
    f: &mut std::fmt::Formatter<'_>,
    index: usize,
    coordinate: usize,
    value: f64,
) -> std::fmt::Result {
    write!(
        f,
        "pole[{index}][{coordinate}]={value}; expected finite coordinate"
    )
}

struct ConePrepared<'a> {
    fit: &'a ConeFit<'a>,
    cosine: Vec<f64>,
    sine: Vec<f64>,
    angular_weight: Vec<f64>,
    weights: Vec<f64>,
}
fn prepare<'a>(fit: &'a ConeFit<'a>) -> Result<ConePrepared<'a>, ConeFitFailure> {
    admit(fit, 0.)?;
    let [cosine, sine, angular_weight] = angular_factors(fit.tangent, fit.chart_center);
    let weights = constant_weight_product(fit.slant_weights, &angular_weight);
    Ok(ConePrepared {
        fit,
        cosine,
        sine,
        angular_weight,
        weights,
    })
}
pub(super) fn cone_edges(
    fit: &ConeFit<'_>,
    depths: [f64; 2],
) -> Result<[RationalPoles; 2], ConeFitFailure> {
    let prepared = prepare(fit)?;
    Ok([prepared.edge(depths[0])?, prepared.edge(depths[1])?])
}
impl ConePrepared<'_> {
    fn edge(&self, depth: f64) -> Result<RationalPoles, ConeFitFailure> {
        finite("depth", &[depth])?;
        let [radial, axial] = meridian_factors(self.fit, depth);
        let x = product(&radial, &self.cosine);
        let y = product(&radial, &self.sine);
        let z = product(&axial, &self.angular_weight);
        rational_poles(&x, &y, &z, &self.weights)
    }
}
fn admit(fit: &ConeFit<'_>, depth: f64) -> Result<(), ConeFitFailure> {
    admit_shape(fit)?;
    if !fit.semi_angle.is_finite()
        || fit.semi_angle <= 0.
        || fit.semi_angle >= std::f64::consts::FRAC_PI_2
    {
        return Err(ConeFitFailure::InvalidAngle(fit.semi_angle));
    }
    finite("tangent", fit.tangent)?;
    finite("slant numerator", fit.slant_numerator)?;
    finite("slant weights", fit.slant_weights)?;
    finite("chart center and depth", &[fit.chart_center, depth])
}
fn admit_shape(fit: &ConeFit<'_>) -> Result<(), ConeFitFailure> {
    if !(1..=4).contains(&fit.tangent.len())
        || !(1..=10).contains(&fit.slant_numerator.len())
        || fit.slant_numerator.len() != fit.slant_weights.len()
        || fit.tangent.len() + fit.slant_numerator.len() < 3
    {
        return Err(ConeFitFailure::Shape {
            tangent: fit.tangent.len(),
            numerator: fit.slant_numerator.len(),
            weights: fit.slant_weights.len(),
        });
    }
    Ok(())
}
fn finite(role: &'static str, values: &[f64]) -> Result<(), ConeFitFailure> {
    for (index, value) in values.iter().copied().enumerate() {
        if !value.is_finite() {
            return Err(ConeFitFailure::Nonfinite { role, index, value });
        }
    }
    Ok(())
}
fn angular_factors(tangent: &[f64], center: f64) -> [Vec<f64>; 3] {
    let square = product(tangent, tangent);
    let weight: Vec<_> = square.iter().map(|value| 1. + value).collect();
    let base_cosine: Vec<_> = square.iter().map(|value| 1. - value).collect();
    let base_sine = product(tangent, &vec![2.; tangent.len()]);
    let (c, s) = (cos(center), sin(center));
    let x = combine(&base_cosine, &base_sine, c, -s);
    let y = combine(&base_cosine, &base_sine, s, c);
    [x, y, weight]
}
fn meridian_factors(fit: &ConeFit<'_>, depth: f64) -> [Vec<f64>; 2] {
    let (c, s) = (cos(fit.semi_angle), sin(fit.semi_angle));
    let signed_depth = if fit.outward { depth } else { -depth };
    [
        combine(fit.slant_numerator, fit.slant_weights, s, signed_depth * c),
        combine(fit.slant_numerator, fit.slant_weights, c, -signed_depth * s),
    ]
}
fn combine(a: &[f64], b: &[f64], left: f64, right: f64) -> Vec<f64> {
    a.iter().zip(b).map(|(a, b)| left * a + right * b).collect()
}
fn rational_poles(
    x: &[f64],
    y: &[f64],
    z: &[f64],
    weights: &[f64],
) -> Result<RationalPoles, ConeFitFailure> {
    let mut points = Vec::with_capacity(weights.len());
    for (index, weight) in weights.iter().copied().enumerate() {
        points.push(rational_point(
            index,
            weight,
            [x[index], y[index], z[index]],
        )?);
    }
    Ok(RationalPoles {
        points,
        weights: weights.to_vec(),
    })
}
fn rational_point(
    index: usize,
    weight: f64,
    homogeneous: [f64; 3],
) -> Result<[f64; 3], ConeFitFailure> {
    if !weight.is_finite() || weight <= 0. {
        return Err(ConeFitFailure::NonpositiveWeight {
            index,
            value: weight,
        });
    }
    let point = homogeneous.map(|value| value / weight);
    for (coordinate, value) in point.iter().copied().enumerate() {
        if !value.is_finite() {
            return Err(ConeFitFailure::Overflow {
                index,
                coordinate,
                value,
            });
        }
    }
    Ok(point)
}

pub(super) mod fit;

// #536: a constant Bernstein denominator has one value by partition of unity.
// General binomial accumulation can invent adjacent one-ulp weight differences.
fn constant_weight_product(left: &[f64], right: &[f64]) -> Vec<f64> {
    let Some((&a, &b)) = left.first().zip(right.first()) else {
        return Vec::new();
    };
    let constant_left = left.iter().all(|value| value.to_bits() == a.to_bits());
    let constant_right = right.iter().all(|value| value.to_bits() == b.to_bits());
    if constant_left && constant_right {
        return vec![a * b; left.len() + right.len() - 1];
    }
    product(left, right)
}
#[cfg(test)]
mod denominator_tests {
    use super::*;
    #[test]
    fn constant_denominators_follow_partition_of_unity_without_invented_weight_steps() {
        for left_degree in 0..10 {
            for right_degree in 0..10 {
                let actual = constant_weight_product(
                    &vec![0.5; left_degree + 1],
                    &vec![4.; right_degree + 1],
                );
                assert_eq!(actual, vec![2.; left_degree + right_degree + 1]);
            }
        }
        assert_eq!(constant_weight_product(&[1.; 4], &[1.; 7]), vec![1.; 10]);
    }
    #[test]
    fn nonconstant_denominators_preserve_the_existing_convolution_bits() {
        let left = [1., 1_f64.next_up(), 1.];
        let right = [1.; 4];
        assert_eq!(
            constant_weight_product(&left, &right),
            product(&left, &right)
        );
        assert_eq!(
            constant_weight_product(&right, &left),
            product(&right, &left)
        );
    }
}
