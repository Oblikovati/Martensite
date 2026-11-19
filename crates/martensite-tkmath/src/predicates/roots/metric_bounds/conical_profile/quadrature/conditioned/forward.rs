// SPDX-License-Identifier: GPL-2.0-only
//! Outward Newton differences preserve exact polynomial values at the fixed dyadic grid.
use super::*;
pub(super) struct ForwardDifference {
    differences: Vec<Interval>,
}
impl ForwardDifference {
    pub(super) fn new(
        poles: &[Interval],
        denominator: u64,
        arithmetic: &mut Arithmetic<'_>,
    ) -> Bounded<Self> {
        // With fewer grid nodes than degree+1, all requested nodes are seeded; no extrapolation is used.
        let count = poles.len().min(denominator as usize + 1);
        let values = (0..count)
            .map(|index| evaluate(poles, index as u64, denominator, arithmetic))
            .collect::<Bounded<Vec<_>>>()?;
        Ok(Self {
            differences: differences(values, arithmetic)?,
        })
    }
    pub(super) fn value(&self) -> &Interval {
        &self.differences[0]
    }
    pub(super) fn advance(&mut self, arithmetic: &mut Arithmetic<'_>) -> Bounded<()> {
        // Ascending order keeps each higher difference at its previous grid node.
        for index in 0..self.differences.len() - 1 {
            self.differences[index] =
                arithmetic.add(&self.differences[index], &self.differences[index + 1])?;
        }
        Ok(())
    }
}
fn differences(
    mut values: Vec<Interval>,
    arithmetic: &mut Arithmetic<'_>,
) -> Bounded<Vec<Interval>> {
    let mut result = Vec::with_capacity(values.len());
    while !values.is_empty() {
        result.push(values[0].clone());
        values = values
            .windows(2)
            .map(|pair| arithmetic.subtract(&pair[1], &pair[0]).map_err(Into::into))
            .collect::<Bounded<Vec<_>>>()?;
    }
    Ok(result)
}
fn evaluate(
    poles: &[Interval],
    numerator: u64,
    denominator: u64,
    arithmetic: &mut Arithmetic<'_>,
) -> Bounded<Interval> {
    let mut values = poles.to_vec();
    for count in (1..values.len()).rev() {
        for index in 0..count {
            let left = arithmetic.fraction(&values[index], denominator - numerator, denominator)?;
            let right = arithmetic.fraction(&values[index + 1], numerator, denominator)?;
            values[index] = arithmetic.add(&left, &right)?;
        }
    }
    Ok(values[0].clone())
}
