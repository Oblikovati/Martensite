//! Sparse exact tensor powers retain zero cancellation and common rational denominators.
use super::{Ratio, budget::Budget, types::SurfaceQueryCause as Cause};
use std::{cmp::Ordering, collections::BTreeMap};
#[derive(Clone, Debug, Default)]
pub(super) struct Polynomial(pub BTreeMap<[usize; 2], Ratio>);
pub(super) fn integer(value: i32) -> Ratio {
    Ratio::new(value.into(), 1.into())
}
impl Polynomial {
    pub fn insert(&mut self, power: [usize; 2], value: Ratio) {
        if value.sign() == Ordering::Equal {
            self.0.remove(&power);
        } else {
            self.0.insert(power, value);
        }
    }
    pub fn add(&self, other: &Self, budget: &mut Budget<'_>) -> Result<Self, Cause> {
        let mut result = self.clone();
        for (power, value) in &other.0 {
            budget.step()?;
            let sum = result
                .0
                .get(power)
                .map_or_else(|| value.clone(), |old| old.add(value));
            result.insert(*power, budget.retain(sum)?);
        }
        Ok(result)
    }
    pub fn scale(&self, scale: &Ratio, budget: &mut Budget<'_>) -> Result<Self, Cause> {
        let mut result = Self::default();
        for (power, value) in &self.0 {
            budget.step()?;
            result.insert(*power, budget.retain(value.multiply(scale))?);
        }
        Ok(result)
    }
    pub fn subtract(&self, other: &Self, budget: &mut Budget<'_>) -> Result<Self, Cause> {
        self.add(&other.scale(&integer(-1), budget)?, budget)
    }
    pub fn multiply(&self, other: &Self, budget: &mut Budget<'_>) -> Result<Self, Cause> {
        let mut result = Self::default();
        for (left, a) in &self.0 {
            for (right, b) in &other.0 {
                product_term(
                    &mut result,
                    [left[0] + right[0], left[1] + right[1]],
                    a,
                    b,
                    budget,
                )?;
            }
        }
        Ok(result)
    }
    fn accumulate(
        &mut self,
        power: [usize; 2],
        value: &Ratio,
        budget: &mut Budget<'_>,
    ) -> Result<(), Cause> {
        let sum = self
            .0
            .get(&power)
            .map_or_else(|| value.clone(), |old| old.add(value));
        self.insert(power, budget.retain(sum)?);
        Ok(())
    }
    pub fn derivative(&self, axis: usize, budget: &mut Budget<'_>) -> Result<Self, Cause> {
        let mut result = Self::default();
        for (power, value) in &self.0 {
            if power[axis] == 0 {
                continue;
            }
            budget.step()?;
            let mut next = *power;
            next[axis] -= 1;
            result.insert(
                next,
                budget.retain(value.multiply(&integer(power[axis] as i32)))?,
            );
        }
        Ok(result)
    }
    pub fn degree(&self) -> [usize; 2] {
        self.0.keys().fold([0, 0], |old, power| {
            [old[0].max(power[0]), old[1].max(power[1])]
        })
    }
    pub fn endpoint(
        &self,
        axis: usize,
        side: usize,
        budget: &mut Budget<'_>,
    ) -> Result<Self, Cause> {
        let mut result = Self::default();
        for (power, value) in &self.0 {
            if side == 0 && power[axis] != 0 {
                continue;
            }
            budget.step()?;
            let mut next = *power;
            next[axis] = 0;
            result.accumulate(next, value, budget)?;
        }
        Ok(result)
    }
}
fn product_term(
    result: &mut Polynomial,
    power: [usize; 2],
    a: &Ratio,
    b: &Ratio,
    budget: &mut Budget<'_>,
) -> Result<(), Cause> {
    budget.step()?;
    let product = budget.retain(a.multiply(b))?;
    let sum = result
        .0
        .get(&power)
        .map_or_else(|| product.clone(), |old| old.add(&product));
    result.insert(power, budget.retain(sum)?);
    Ok(())
}
