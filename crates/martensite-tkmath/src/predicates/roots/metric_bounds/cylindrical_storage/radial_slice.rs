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
    let polynomial = SlicePolynomial::capture(query, &mut arithmetic)?;
    // At each span parameter the signed numerator and denominator are affine
    // in depth, so their ratio stays between its two endpoint values.
    let a = polynomial.lower_at(query.depth[0], &mut arithmetic)?;
    let b = polynomial.lower_at(query.depth[1], &mut arithmetic)?;
    let lower = std::array::from_fn(|axis| a[axis].min(b[axis]));
    if lower[0] <= 0. || lower[1] <= 0. {
        return Err(CylindricalSliceRefusal::Nonpositive {
            direction: lower[0],
            radial: lower[1],
        });
    }
    Ok(CertifiedCylindricalSlice::new(
        query,
        lower,
        arithmetic.operations(),
    ))
}

struct SlicePolynomial {
    first: [Controls; 2],
    offset: [Controls; 2],
    weights: Controls,
    height_derivative: Controls,
    theta: Interval,
    height: Interval,
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
        let radius = arithmetic.scalar(query.chart_radius)?;
        let angle_direction = arithmetic.scalar(query.direction[0])?;
        let theta = arithmetic.multiply(&radius, &angle_direction)?;
        let height = arithmetic.scalar(query.direction[1])?;
        let sign = arithmetic.scalar(match query.sense {
            CylindricalRadialSense::Outward => 1.,
            CylindricalRadialSense::Inward => -1.,
        })?;
        Ok(Self {
            first: [x, y],
            offset,
            weights,
            height_derivative,
            theta,
            height,
            sign,
        })
    }

    fn lower_at(&self, depth: f64, arithmetic: &mut Arithmetic<'_>) -> Checked<[f64; 2]> {
        let xy = patch_xy(&self.first, &self.offset, depth, arithmetic)?;
        let radial = dot(&xy, &self.offset, arithmetic)?;
        let tangent = [
            tangent_numerator(&xy[0], &self.weights, arithmetic)?,
            tangent_numerator(&xy[1], &self.weights, arithmetic)?,
        ];
        let theta = self.angular_term(&tangent, arithmetic)?;
        let height = product(&self.height_derivative, &radial, arithmetic)?;
        let height = scale(&height, &self.height, arithmetic)?;
        let numerator = sum(&theta, &height, arithmetic)?;
        self.bound(numerator, radial, arithmetic)
    }

    fn angular_term(
        &self,
        tangent: &[Controls; 2],
        arithmetic: &mut Arithmetic<'_>,
    ) -> Checked<Controls> {
        // Along a fixed-radius slice the chart derivative is
        // R*det(D_xy, P'_xy)/dot(P_xy, D_xy), where D is the depth direction.
        let y = product(&self.offset[0], &tangent[1], arithmetic)?;
        let x = product(&self.offset[1], &tangent[0], arithmetic)?;
        let determinant = difference(&y, &x, arithmetic)?;
        let weighted = product(&determinant, &self.weights, arithmetic)?;
        Ok(scale(&weighted, &self.theta, arithmetic)?)
    }

    fn bound(
        &self,
        numerator: Controls,
        radial: Controls,
        arithmetic: &mut Arithmetic<'_>,
    ) -> Checked<[f64; 2]> {
        let squared_weights = product(&self.weights, &self.weights, arithmetic)?;
        let signed_radial = scale(&radial, &self.sign, arithmetic)?;
        let denominator = product(&signed_radial, &squared_weights, arithmetic)?;
        let signs = [self.sign.clone(), self.sign.clone()];
        let numerator = product(&numerator, &signs, arithmetic)?;
        let direction_lower = ratio_lower(&numerator, &denominator, arithmetic)?;
        let radial_twice = sum(&signed_radial, &signed_radial, arithmetic)?;
        let radial_lower = ratio_lower(&radial_twice, &squared_weights, arithmetic)?;
        Ok([direction_lower, radial_lower])
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
