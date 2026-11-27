// SPDX-License-Identifier: GPL-2.0-only
//! Raw coefficients preserve magnitude; primitive normalization belongs at solver boundaries.
use super::super::{
    RootIsolationError, certificate_arithmetic::convolution::raw_coefficient_product, error::poll,
    polynomial::IntegerPolynomial,
};
use num_bigint::{BigInt, Sign};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ExactCoefficientPolynomial(pub Vec<BigInt>);
impl ExactCoefficientPolynomial {
    pub fn new(mut coefficients: Vec<BigInt>) -> Self {
        while coefficients
            .last()
            .is_some_and(|term| term.sign() == Sign::NoSign)
        {
            coefficients.pop();
        }
        Self(coefficients)
    }
    #[cfg(test)]
    pub fn constant(value: BigInt) -> Self {
        let coefficients = vec![value];
        Self::new(coefficients)
    }
    pub fn coefficient(&self, power: usize) -> BigInt {
        // Missing powers are the additive identity, not a discarded geometric failure.
        self.0
            .get(power)
            .cloned()
            .unwrap_or_else(|| BigInt::from(0))
    }
    pub fn degree(&self) -> Option<usize> {
        let length = self.0.len();
        length.checked_sub(1)
    }
    pub fn add(&self, other: &Self) -> Self {
        let length = self.0.len().max(other.0.len());
        let coefficients = (0..length)
            .map(|i| self.coefficient(i) + other.coefficient(i))
            .collect();
        Self::new(coefficients)
    }
    pub fn subtract(&self, other: &Self) -> Self {
        let length = self.0.len().max(other.0.len());
        let coefficients = (0..length)
            .map(|i| self.coefficient(i) - other.coefficient(i))
            .collect();
        Self::new(coefficients)
    }
    pub fn scaled(&self, factor: &BigInt) -> Self {
        let coefficients = self.0.iter().map(|term| term * factor).collect();
        Self::new(coefficients)
    }
    pub fn shifted(&self) -> Self {
        let mut coefficients = vec![BigInt::from(0)];
        coefficients.extend(self.0.iter().cloned());
        Self::new(coefficients)
    }
    pub fn multiply(
        &self,
        other: &Self,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Self, RootIsolationError> {
        let coefficients = raw_coefficient_product(&self.0, &other.0, cancelled)?;
        Ok(Self::new(coefficients))
    }
    pub fn primitive(&self, limit: u16) -> Result<IntegerPolynomial, RootIsolationError> {
        // Applying content reduction earlier would corrupt sums in Fx/Fy substitution.
        IntegerPolynomial::from_integers(self.0.clone(), limit)
    }
    /// Exact primitive GCD in `Z[t]` when the two polynomials share a factor of degree >= 1 (#619).
    pub fn nonconstant_primitive_gcd(
        &self,
        other: &Self,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Option<Self>, RootIsolationError> {
        if self.degree().is_none_or(|d| d == 0) || other.degree().is_none_or(|d| d == 0) {
            return Ok(None);
        }
        let (mut left, mut right) = (self.primitive_part(), other.primitive_part());
        if left.degree() < right.degree() {
            std::mem::swap(&mut left, &mut right);
        }
        while right.degree().is_some() {
            poll(cancelled)?;
            let remainder = left.primitive_pseudo_remainder(&right, cancelled)?;
            left = right;
            right = remainder;
        }
        Ok(left.degree().is_some_and(|d| d > 0).then_some(left))
    }
    /// Exact quotient in `Z[t]` when `divisor` is a nonzero primitive factor of `self`.
    pub fn exact_divide(
        &self,
        divisor: &Self,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Option<Self>, RootIsolationError> {
        let Some(div_deg) = divisor.degree() else {
            return Ok(None);
        };
        let Some(self_deg) = self.degree() else {
            return Ok(Some(Self::new(Vec::new())));
        };
        if self_deg < div_deg {
            return Ok(None);
        }
        let mut remainder = self.0.clone();
        let mut quotient = vec![BigInt::from(0); self_deg - div_deg + 1];
        for power in (div_deg..=self_deg).rev() {
            poll(cancelled)?;
            if !exact_divide_step(&mut remainder, &mut quotient, &divisor.0, power) {
                return Ok(None);
            }
        }
        let clean = remainder[..div_deg]
            .iter()
            .all(|value| value.sign() == Sign::NoSign);
        Ok(clean.then(|| Self::new(quotient)))
    }
    fn primitive_part(&self) -> Self {
        let Some(last) = self.0.last() else {
            return Self(Vec::new());
        };
        let content = self.0.iter().fold(BigInt::from(0), |acc, value| {
            positive_gcd(acc, abs_bigint(value))
        });
        let negative_lead = last.sign() == Sign::Minus;
        let coefficients = self
            .0
            .iter()
            .map(|value| {
                let reduced = value / &content;
                if negative_lead { -reduced } else { reduced }
            })
            .collect();
        Self::new(coefficients)
    }
    fn primitive_pseudo_remainder(
        &self,
        divisor: &Self,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Self, RootIsolationError> {
        let Some(div_deg) = divisor.degree() else {
            return Ok(Self::new(Vec::new()));
        };
        let mut rem = self.clone();
        while rem.degree().is_some_and(|deg| deg >= div_deg) {
            poll(cancelled)?;
            rem = cancel_primitive_lead(&rem, divisor, div_deg).primitive_part();
        }
        Ok(rem)
    }
}
fn cancel_primitive_lead(
    rem: &ExactCoefficientPolynomial,
    divisor: &ExactCoefficientPolynomial,
    div_deg: usize,
) -> ExactCoefficientPolynomial {
    let rem_deg = rem.0.len() - 1;
    let offset = rem_deg - div_deg;
    let lead_r = &rem.0[rem_deg];
    let lead_d = &divisor.0[div_deg];
    let gcd = positive_gcd(abs_bigint(lead_r), abs_bigint(lead_d));
    let scale_r = lead_d / &gcd;
    let scale_d = lead_r / &gcd;
    let mut coefficients: Vec<BigInt> = rem.0.iter().map(|c| c * &scale_r).collect();
    for (index, coeff) in divisor.0.iter().enumerate() {
        coefficients[offset + index] -= coeff * &scale_d;
    }
    ExactCoefficientPolynomial::new(coefficients)
}
fn abs_bigint(value: &BigInt) -> BigInt {
    if value.sign() == Sign::Minus {
        -value
    } else {
        value.clone()
    }
}
fn positive_gcd(mut left: BigInt, mut right: BigInt) -> BigInt {
    while right.sign() != Sign::NoSign {
        let remainder = left % &right;
        left = right;
        right = remainder;
    }
    left
}
fn exact_divide_step(
    remainder: &mut [BigInt],
    quotient: &mut [BigInt],
    divisor: &[BigInt],
    power: usize,
) -> bool {
    if remainder[power].sign() == Sign::NoSign {
        return true;
    }
    let div_deg = divisor.len() - 1;
    let Some(q) = exact_int_div(&remainder[power], &divisor[div_deg]) else {
        return false;
    };
    let shift = power - div_deg;
    for (index, coeff) in divisor.iter().enumerate() {
        remainder[shift + index] -= &q * coeff;
    }
    quotient[shift] = q;
    true
}
fn exact_int_div(numerator: &BigInt, denominator: &BigInt) -> Option<BigInt> {
    if denominator.sign() == Sign::NoSign {
        return None;
    }
    if (numerator % denominator).sign() != Sign::NoSign {
        return None;
    }
    Some(numerator / denominator)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn coefficients(values: &[i32]) -> ExactCoefficientPolynomial {
        let values = values.iter().map(|value| BigInt::from(*value)).collect();
        ExactCoefficientPolynomial::new(values)
    }
    #[test]
    fn ring_preserves_content_before_addition_and_only_normalizes_solver_copy() {
        let a = coefficients(&[2, 2]);
        let b = coefficients(&[3, -3]);
        let product = a.multiply(&b, &mut || false).unwrap();
        assert_eq!(product, coefficients(&[6, 0, -6]));
        assert_eq!(product.add(&coefficients(&[3])), coefficients(&[9, 0, -6]));
        assert_eq!(product.primitive(2).unwrap().0, coefficients(&[1, 0, -1]).0);
        assert_eq!(product, coefficients(&[6, 0, -6]));
    }
    #[test]
    fn zero_degree_shift_scale_subtraction_and_cancel_are_exact() {
        let p = coefficients(&[1, 2, 0]);
        assert_eq!(p.degree(), Some(1));
        assert_eq!(p.subtract(&p).degree(), None);
        assert_eq!(p.scaled(&BigInt::from(-3)), coefficients(&[-3, -6]));
        assert_eq!(p.shifted(), coefficients(&[0, 1, 2]));
        assert_eq!(
            ExactCoefficientPolynomial::constant(0.into()).degree(),
            None
        );
        assert_eq!(
            p.multiply(&p, &mut || true),
            Err(RootIsolationError::Cancelled)
        );
    }
    #[test]
    fn primitive_gcd_and_exact_divide_cancel_shared_nonconstant_factors() {
        // a = 6(2t - 1)(t + 3), b = 4(2t - 1)(3t - 5); primitive GCD is (2t - 1).
        let factor = coefficients(&[-1, 2]);
        let a = factor
            .multiply(&coefficients(&[18, 6]), &mut || false)
            .unwrap();
        let b = factor
            .multiply(&coefficients(&[-20, 12]), &mut || false)
            .unwrap();
        let common = a.nonconstant_primitive_gcd(&b, &mut || false).unwrap();
        assert_eq!(common, Some(factor.clone()));
        assert_eq!(
            a.exact_divide(&factor, &mut || false).unwrap(),
            Some(coefficients(&[18, 6]))
        );
        assert_eq!(
            b.exact_divide(&factor, &mut || false).unwrap(),
            Some(coefficients(&[-20, 12]))
        );
        // Coprime nonconstant and constant cases return None without changing integer scale.
        assert_eq!(
            coefficients(&[0, 1])
                .nonconstant_primitive_gcd(&coefficients(&[1, 1]), &mut || false)
                .unwrap(),
            None
        );
        assert_eq!(
            coefficients(&[12])
                .nonconstant_primitive_gcd(&coefficients(&[96]), &mut || false)
                .unwrap(),
            None
        );
        assert_eq!(
            a.nonconstant_primitive_gcd(&b, &mut || true),
            Err(RootIsolationError::Cancelled)
        );
        assert_eq!(
            a.exact_divide(&factor, &mut || true),
            Err(RootIsolationError::Cancelled)
        );
    }
}
