// SPDX-License-Identifier: GPL-2.0-only
//! Explicit nearest-cursor preference requests for a prepared hard constraint model.
use super::{Prepared, PreviewError, SolveError, Update, iterate};

/// Why a preferred cursor update settled at its feasible coordinate vector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreferenceSettlement {
    /// The scaled cursor miss meets the ordinary target convergence tolerance.
    TargetReached,
    /// Numerical tangential gradient met the requested first-order tolerance.
    /// This reports stationarity at the returned pose, with no minimum or uniqueness claim.
    FirstOrderStationary,
}

/// Remaining scaled cursor error and tangency evidence at a completed preferred update.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreferenceReport {
    /// Condition that stopped the update.
    pub settlement: PreferenceSettlement,
    /// Euclidean norm of the remaining scaled cursor error, including fixed coordinates.
    pub scaled_cursor_error: f64,
    /// Largest scaled projected gradient norm among components containing requested free coordinates.
    pub max_tangential_gradient: f64,
}

impl Prepared {
    /// Update an accepted state toward a preferred cursor using bounded hard retraction.
    ///
    /// The cursor may lie outside the feasible set. The result settles at the cursor or
    /// passes a first-order stationarity check. The caller selects a positive finite
    /// `tangent_tolerance` in scaled coordinate units and still owns hard geometry adoption.
    /// Example: `let (moved, preference) = prepared.update_preferred(&values, &targets, 1e-7, || false)?;`
    pub fn update_preferred(
        &self,
        values: &[f64],
        targets: &[(usize, f64)],
        tangent_tolerance: f64,
        cancelled: impl FnMut() -> bool,
    ) -> Result<(Update, PreferenceReport), PreviewError> {
        validate_tolerance(tangent_tolerance)?;
        self.limits.admit_targets(targets.len())?;
        let values = self.read_values(values)?;
        let targets = self.read_targets(targets)?;
        iterate::run_preferred(self, values, &targets, tangent_tolerance, cancelled)
    }

    /// Recheck an unchanged candidate against a rebound prepared program in the same tick.
    ///
    /// No coordinate step is attempted. Hard rows, sparse rank and the final projected
    /// gradient are verified within the supplied current tick's accumulated work.
    /// Example: `let (checked, preference) = rebound.verify_preferred(moved, &targets, 1e-7, || false)?;`
    pub fn verify_preferred(
        &self,
        moved: Update,
        targets: &[(usize, f64)],
        tangent_tolerance: f64,
        cancelled: impl FnMut() -> bool,
    ) -> Result<(Update, PreferenceReport), PreviewError> {
        validate_tolerance(tangent_tolerance)?;
        self.limits.admit_targets(targets.len())?;
        let targets = self.read_targets(targets)?;
        iterate::verify_preferred(self, moved, &targets, tangent_tolerance, cancelled)
    }
}

fn validate_tolerance(tolerance: f64) -> Result<(), PreviewError> {
    if !tolerance.is_finite() || tolerance <= 0. {
        return Err(SolveError::Options(format!(
            "projected cursor tolerance {tolerance}: expected finite and greater than zero"
        ))
        .into());
    }
    Ok(())
}
