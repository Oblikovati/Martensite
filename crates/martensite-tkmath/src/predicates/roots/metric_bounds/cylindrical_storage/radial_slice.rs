// SPDX-License-Identifier: GPL-2.0-only
//! Directional monotonicity along fixed-radius slices of a common-height ruled wall.
use super::{polynomial::*, *};
use crate::predicates::roots::certificate_arithmetic::bernstein_curve::derivative;
mod contract;
mod cover;
pub use contract::*;
#[cfg(test)]
mod tests;

type Checked<T> = Result<T, CylindricalSliceRefusal>;
type Controls = Vec<Interval>;

/// Bound a directional derivative on the actual fixed-radius slices through the depth interval.
/// The chart's angular direction is scaled by its declared radius. The caller binds
/// both rails to the actual ruled patch, checks complete slice existence and joins,
/// and supplies a common projection direction.
/// Example: `let proof = certify_cylindrical_slice_direction(query, || false)?;`.
pub fn certify_cylindrical_slice_direction(
    query: CylindricalSliceQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Checked<CertifiedCylindricalSlice> {
    contract::admit(query)?;
    let mut arithmetic = Arithmetic::new(arithmetic_policy(query.options), &mut cancelled);
    let endpoints = make_endpoints(query, &mut arithmetic)?;
    project(query, &endpoints, &mut arithmetic)
}

/// Certify one side direction and both join directions using the same native ruled-patch algebra.
/// Every direction is admitted and has its own strict bound over the entire declared depth range.
/// The declared work limit covers the shared endpoint construction and all three directions.
/// Example: `let proofs = certify_cylindrical_slice_with_joins(query, [incoming, outgoing], || false)?;`.
pub fn certify_cylindrical_slice_with_joins(
    query: CylindricalSliceQuery<'_>,
    joins: [[f64; 2]; 2],
    mut cancelled: impl FnMut() -> bool + Send,
) -> Checked<[CertifiedCylindricalSlice; 3]> {
    let [first, last] = joins.map(|direction| CylindricalSliceQuery { direction, ..query });
    for admitted in [query, first, last] {
        contract::admit(admitted)?;
    }
    let mut arithmetic = Arithmetic::new(arithmetic_policy(query.options), &mut cancelled);
    let endpoints = make_endpoints(query, &mut arithmetic)?;
    Ok([
        project(query, &endpoints, &mut arithmetic)?,
        project(first, &endpoints, &mut arithmetic)?,
        project(last, &endpoints, &mut arithmetic)?,
    ])
}

fn make_endpoints(
    query: CylindricalSliceQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Endpoint; 2]> {
    let polynomial = SlicePolynomial::capture(query, arithmetic)?;
    Ok([
        polynomial.at(query.depth[0], arithmetic)?,
        polynomial.at(query.depth[1], arithmetic)?,
    ])
}

fn project(
    query: CylindricalSliceQuery<'_>,
    [a, b]: &[Endpoint; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<CertifiedCylindricalSlice> {
    // At each span parameter the signed numerator and denominator are affine
    // in depth, so their ratio stays between its two endpoint values.
    let theta = arithmetic.scalar(query.chart_radius)?;
    let x = arithmetic.scalar(query.direction[0])?;
    let theta = arithmetic.multiply(&theta, &x)?;
    let height = arithmetic.scalar(query.direction[1])?;
    let direction = a
        .project([&theta, &height], arithmetic)?
        .min(b.project([&theta, &height], arithmetic)?);
    let radial = a.squared_radius_lower.min(b.squared_radius_lower);
    if direction <= 0. || radial <= 0. {
        return Err(CylindricalSliceRefusal::Nonpositive { direction, radial });
    }
    Ok(CertifiedCylindricalSlice::new(
        query,
        [direction, radial],
        arithmetic.operations(),
    ))
}

struct SlicePolynomial {
    first: [Controls; 2],
    offset: [Controls; 2],
    weights: Controls,
    weight_square: Controls,
    height_derivative: Controls,
    sign: Interval,
}

impl SlicePolynomial {
    fn capture(query: CylindricalSliceQuery<'_>, arithmetic: &mut Arithmetic<'_>) -> Checked<Self> {
        let [x, y, height, weights] = super::capture(query.rails[0], arithmetic)?;
        let [x_far, y_far, _, _] = super::capture(query.rails[1], arithmetic)?;
        let offset = [
            difference(&x_far, &x, arithmetic)?,
            difference(&y_far, &y, arithmetic)?,
        ];
        let height_derivative = tangent_numerator(&height, &weights, arithmetic)?;
        let weight_square = product(&weights, &weights, arithmetic)?;
        let sign = arithmetic.scalar(match query.sense {
            CylindricalRadialSense::Outward => 1.,
            CylindricalRadialSense::Inward => -1.,
        })?;
        Ok(Self {
            first: [x, y],
            offset,
            weights,
            weight_square,
            height_derivative,
            sign,
        })
    }

    fn at(&self, depth: f64, arithmetic: &mut Arithmetic<'_>) -> Checked<Endpoint> {
        let xy = patch_xy(&self.first, &self.offset, depth, arithmetic)?;
        let radial = dot(&xy, &self.offset, arithmetic)?;
        let angular = self.angular_term(&xy, arithmetic)?;
        let height = product(&self.height_derivative, &radial, arithmetic)?;
        let sign = [self.sign.clone(), self.sign.clone()];
        let signed_radial = scale(&radial, &self.sign, arithmetic)?;
        let squared_radius_lower = self.squared_radius_lower(&signed_radial, arithmetic)?;
        let denominator = product(&signed_radial, &self.weight_square, arithmetic)?;
        Ok(Endpoint {
            angular,
            height,
            sign,
            denominator,
            squared_radius_lower,
        })
    }

    fn angular_term(
        &self,
        xy: &[Controls; 2],
        arithmetic: &mut Arithmetic<'_>,
    ) -> Checked<Controls> {
        // R*det(D_xy, P'_xy)/dot(P_xy, D_xy) is the fixed-radius chart derivative.
        let tangent = [
            tangent_numerator(&xy[0], &self.weights, arithmetic)?,
            tangent_numerator(&xy[1], &self.weights, arithmetic)?,
        ];
        let y = product(&self.offset[0], &tangent[1], arithmetic)?;
        let x = product(&self.offset[1], &tangent[0], arithmetic)?;
        let determinant = difference(&y, &x, arithmetic)?;
        Ok(product(&determinant, &self.weights, arithmetic)?)
    }

    fn squared_radius_lower(
        &self,
        signed_radial: &[Interval],
        arithmetic: &mut Arithmetic<'_>,
    ) -> Checked<f64> {
        let radial_twice = sum(signed_radial, signed_radial, arithmetic)?;
        ratio_lower(&radial_twice, &self.weight_square, arithmetic)
    }
}

struct Endpoint {
    angular: Controls,
    height: Controls,
    sign: [Interval; 2],
    denominator: Controls,
    squared_radius_lower: f64,
}

impl Endpoint {
    fn project(&self, [theta, z]: [&Interval; 2], arithmetic: &mut Arithmetic<'_>) -> Checked<f64> {
        let theta = scale(&self.angular, theta, arithmetic)?;
        let height = scale(&self.height, z, arithmetic)?;
        let numerator = sum(&theta, &height, arithmetic)?;
        let numerator = product(&numerator, &self.sign, arithmetic)?;
        ratio_lower(&numerator, &self.denominator, arithmetic)
    }
}

fn ratio_lower(
    numerator: &[Interval],
    denominator: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<f64> {
    // Positive denominator coefficients make each ratio a convex combination
    // of like-degree Bernstein coefficient ratios.
    let ratios = numerator
        .iter()
        .zip(denominator)
        .map(|(n, d)| arithmetic.divide(n, d))
        .collect::<Result<Controls, _>>()?;
    Ok(-arithmetic.upper_f64(-lower(&ratios))?)
}

fn patch_xy(
    first: &[Controls; 2],
    offset: &[Controls; 2],
    depth: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Controls; 2]> {
    let depth = arithmetic.scalar(depth)?;
    let x = scale(&offset[0], &depth, arithmetic)?;
    let y = scale(&offset[1], &depth, arithmetic)?;
    Ok([
        sum(&first[0], &x, arithmetic)?,
        sum(&first[1], &y, arithmetic)?,
    ])
}

fn tangent_numerator(
    position: &[Interval],
    weights: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Controls> {
    let xyz_prime = derivative(position, arithmetic)?;
    let weight_prime = derivative(weights, arithmetic)?;
    let numerator = product(&xyz_prime, weights, arithmetic)?;
    let correction = product(position, &weight_prime, arithmetic)?;
    Ok(difference(&numerator, &correction, arithmetic)?)
}

fn dot(a: &[Controls; 2], b: &[Controls; 2], arithmetic: &mut Arithmetic<'_>) -> Checked<Controls> {
    let x = product(&a[0], &b[0], arithmetic)?;
    let y = product(&a[1], &b[1], arithmetic)?;
    Ok(sum(&x, &y, arithmetic)?)
}
