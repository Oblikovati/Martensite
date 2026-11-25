// SPDX-License-Identifier: GPL-2.0-only
//! Exact Bernstein jets and bounded convex-hull regularity evidence.
use super::{
    super::certificate_arithmetic::rounding::outward_binary64, CartesianEquationError as Error,
    EquationDomainCertificate, R, poll, program::zero,
};

const PROOF_DEPTH: u8 = 8;

#[derive(Clone, Debug)]
pub(super) struct Net(pub [Vec<R>; 3]);

pub(super) fn jet(position: Net, span: &R) -> [Net; 4] {
    let first = position.derivative(span);
    let second = first.derivative(span);
    let third = second.derivative(span);
    [position, first, second, third]
}

pub(super) fn certify(
    rows: &[Net; 4],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<EquationDomainCertificate, Error> {
    let mut coordinate_bounds = [[[0.; 2]; 3]; 4];
    for (order, row) in rows.iter().enumerate() {
        poll(cancelled)?;
        for (axis, coefficients) in row.0.iter().enumerate() {
            let [lower, upper] = extrema(coefficients);
            let unrepresentable = || Error::Unrepresentable { order, axis };
            coordinate_bounds[order][axis] = [
                outward_binary64(lower).ok_or_else(unrepresentable)?[0],
                outward_binary64(upper).ok_or_else(unrepresentable)?[1],
            ];
        }
    }
    let speed = rows[1].regularity(0, cancelled)?;
    let [speed_lower, _] = outward_binary64(&speed).ok_or(Error::SpeedUnderflow)?;
    if speed_lower <= 0. {
        return Err(Error::SpeedUnderflow);
    }
    Ok(EquationDomainCertificate {
        coordinate_bounds,
        speed_lower,
    })
}

impl Net {
    pub(super) fn value(&self, u: &R) -> [R; 3] {
        let left = R::from_finite(1.).subtract(u);
        self.0.each_ref().map(|coordinate| {
            let mut work = coordinate.clone();
            for count in (1..work.len()).rev() {
                for i in 0..count {
                    work[i] = blend(&work[i], &work[i + 1], [&left, u]);
                }
            }
            work.remove(0)
        })
    }

    fn derivative(&self, span: &R) -> Self {
        Self(self.0.each_ref().map(|axis| {
            if axis.len() == 1 {
                return vec![zero()];
            }
            let degree = R::new((axis.len() - 1).into(), 1.into()).divide(span);
            axis.windows(2)
                .map(|pair| pair[1].subtract(&pair[0]).multiply(&degree))
                .collect()
        }))
    }

    fn regularity(&self, depth: u8, cancelled: &mut impl FnMut() -> bool) -> Result<R, Error> {
        poll(cancelled)?;
        if let Some(speed) = self.sign_separation() {
            return Ok(speed);
        }
        if self.stationary_endpoint() {
            return Err(Error::Singular);
        }
        if depth == PROOF_DEPTH {
            return Err(Error::RegularityUnproven { depth });
        }
        let [left, right] = self.bisect();
        let left = left.regularity(depth + 1, cancelled)?;
        let right = right.regularity(depth + 1, cancelled)?;
        Ok(left.min(right))
    }

    fn sign_separation(&self) -> Option<R> {
        self.0
            .iter()
            .filter_map(|axis| {
                let [lower, upper] = extrema(axis);
                if lower.sign().is_gt() {
                    Some(lower.clone())
                } else if upper.sign().is_lt() {
                    Some(upper.negated())
                } else {
                    None
                }
            })
            .max()
    }

    fn stationary_endpoint(&self) -> bool {
        self.0.iter().all(|axis| axis[0].sign().is_eq())
            || self
                .0
                .iter()
                .all(|axis| axis[axis.len() - 1].sign().is_eq())
    }

    fn bisect(&self) -> [Self; 2] {
        let [[lx, rx], [ly, ry], [lz, rz]] = self.0.each_ref().map(|axis| bisect(axis));
        [Self([lx, ly, lz]), Self([rx, ry, rz])]
    }
}

fn extrema(row: &[R]) -> [&R; 2] {
    // Each admitted Bernstein jet has at least one control per coordinate.
    row.iter().fold([&row[0]; 2], |[low, high], value| {
        [low.min(value), high.max(value)]
    })
}

fn bisect(row: &[R]) -> [Vec<R>; 2] {
    let half = R::from_finite(0.5);
    let mut work = row.to_vec();
    let mut left = vec![work[0].clone()];
    let mut right = vec![work[work.len() - 1].clone()];
    for count in (1..work.len()).rev() {
        for i in 0..count {
            work[i] = work[i].add(&work[i + 1]).multiply(&half);
        }
        left.push(work[0].clone());
        right.push(work[count - 1].clone());
    }
    right.reverse();
    [left, right]
}

fn blend(left: &R, right: &R, [one_minus_u, u]: [&R; 2]) -> R {
    left.multiply(one_minus_u).add(&right.multiply(u))
}
