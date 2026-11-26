// SPDX-License-Identifier: GPL-2.0-only
//! Sagitta chart trial rows on the authored start, end and witness coordinates.
use super::{DualNumber, ResidualError, Vector, checked_length, unit, vector};
use crate::constraints::ArcBranch;

pub(super) struct Sagitta {
    chord: Vector,
    offset: Vector,
    half_chord: DualNumber,
    height: DualNumber,
}

impl Sagitta {
    pub(super) fn new(
        values: &[DualNumber],
        start: usize,
        chord: Vector,
        branch: ArcBranch,
    ) -> Result<Self, ResidualError> {
        let witness = vector(values, start + 6)?;
        let from_start = witness.minus(&vector(values, start)?)?.scaled(0.5)?;
        let from_end = witness.minus(&vector(values, start + 3)?)?.scaled(0.5)?;
        let offset = from_start.plus(&from_end)?;
        let half_chord = checked_length(&chord)?.scaled(0.5)?;
        let height = checked_length(&offset)?;
        validate_branch(branch, height.value(), half_chord.value())?;
        Ok(Self {
            chord,
            offset,
            half_chord,
            height,
        })
    }

    pub(super) fn intrinsic(&self) -> Result<DualNumber, ResidualError> {
        Ok(unit(&self.chord)?.dot(&self.offset)?)
    }

    pub(super) fn radius(&self) -> Result<DualNumber, ResidualError> {
        // Multiplication after division avoids squaring a long but representable chord.
        let chord_term = self
            .half_chord
            .scaled(0.5)?
            .product(&self.half_chord.quotient(&self.height)?)?;
        Ok(chord_term.plus(&self.height.scaled(0.5)?)?)
    }
}

fn validate_branch(branch: ArcBranch, height: f64, half_chord: f64) -> Result<(), ResidualError> {
    if branch.matches(height, half_chord) {
        return Ok(());
    }
    Err(ResidualError::Domain(format!(
        "arc sagitta {height} and half chord {half_chord}: expected the {branch:?} branch"
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constraints::residual::displacement;

    #[test]
    fn boundary_trials_decline_and_sagitta_rows_have_independent_physical_partials() {
        for (height, branch) in [(2., ArcBranch::Minor), (8., ArcBranch::Major)] {
            let points = [-4., 0., 0., 4., 0., 0., 0., height, 0.];
            let trial = DualNumber::seed(&points).unwrap();
            let chart = Sagitta::new(&trial, 0, displacement(&trial, 0).unwrap(), branch).unwrap();
            let radius = chart.radius().unwrap();
            assert_eq!(radius.value(), 5.);
            let aspect = 4. / height;
            assert_eq!(radius.gradient()[7], 0.5 * (1. - aspect * aspect));
            let midpoint = chart.intrinsic().unwrap();
            assert_eq!(midpoint.value(), 0.);
            assert_eq!(midpoint.gradient()[6], 1.);
            assert_eq!(midpoint.gradient()[0], -0.5);
            assert_eq!(midpoint.gradient()[3], -0.5);
            assert!(validate_branch(branch, 4., 4.).is_err());
        }
    }

    #[test]
    fn half_turn_radius_keeps_a_chord_derivative_after_the_witness_derivative_vanishes() {
        let points = [-4., 0., 0., 4., 0., 0., 0., 4., 0.];
        let trial = DualNumber::seed(&points).unwrap();
        let chord = displacement(&trial, 0).unwrap();
        let chart = Sagitta::new(&trial, 0, chord, ArcBranch::CrossHalfTurn).unwrap();
        let radius = chart.radius().unwrap();
        assert_eq!(radius.value(), 4.);
        assert_eq!(radius.gradient()[0], -0.5);
        assert_eq!(radius.gradient()[3], 0.5);
        assert_eq!(radius.gradient()[7], 0.);
        assert_eq!(chart.intrinsic().unwrap().gradient()[6], 1.);
    }
}
