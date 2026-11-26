// SPDX-License-Identifier: GPL-2.0-only
use super::{
    BTreeMap, Cholesky, Component, Incomplete, Meter, Prepared, PreviewError, Rows, Run,
    SolveError, Update, hypot, linear_work, local_targets, powi,
};
use crate::preview::{
    PreferenceReport, PreferenceSettlement,
    factor::{fit_targets, weighted_axes},
};

mod merit;

pub(in crate::preview) fn run_preferred(
    program: &Prepared,
    mut values: Vec<f64>,
    targets: &BTreeMap<usize, f64>,
    tolerance: f64,
    mut cancelled: impl FnMut() -> bool,
) -> Result<(Update, PreferenceReport), PreviewError> {
    let mut run = Run {
        program,
        meter: Meter::new(program.limits, &mut cancelled),
        rejected: 0,
    };
    run.check_warm_state(&values)?;
    for component in program.components.iter() {
        run.meter.charge(component.free.len())?;
        let targets = local_targets(component, targets);
        if !targets.is_empty() {
            run.follow_preferred(component, &mut values, &targets, tolerance)?;
        }
    }
    run.report_preferred(values, targets, tolerance)
}

pub(in crate::preview) fn verify_preferred(
    program: &Prepared,
    moved: Update,
    targets: &BTreeMap<usize, f64>,
    tolerance: f64,
    mut cancelled: impl FnMut() -> bool,
) -> Result<(Update, PreferenceReport), PreviewError> {
    let mut meter = Meter::resume(program.limits, moved.work, &mut cancelled)?;
    meter.charge(moved.values.len())?;
    let checked = program.read_values(&moved.values)?;
    if checked
        .iter()
        .zip(&moved.values)
        .any(|(&actual, &prior)| actual.to_bits() != prior.to_bits())
    {
        return Err(PreviewError::Incomplete(Incomplete::InfeasibleWarmState));
    }
    let mut run = Run {
        program,
        meter,
        rejected: moved.rejected_trials,
    };
    run.report_preferred(checked, targets, tolerance)
}

struct CursorTangent {
    axes: Vec<Vec<f64>>,
    gradient: Vec<f64>,
    norm: f64,
}

impl Run<'_> {
    fn follow_preferred(
        &mut self,
        component: &Component,
        values: &mut Vec<f64>,
        targets: &[(usize, f64)],
        tolerance: f64,
    ) -> Result<(), PreviewError> {
        for _ in 0..self.program.options.max_iterations {
            self.meter.poll()?;
            let (miss, pending) = self.preference(component, values, targets);
            check_finite(miss)?;
            if miss <= self.program.options.relative_tolerance.min(tolerance) {
                return Ok(());
            }
            let rows = self.evaluate(component, values)?;
            let factor = self.factor(component, &rows)?;
            let tangent = self.cursor_tangent(component, &rows, &factor, &pending)?;
            if tangent.norm <= tolerance {
                return Ok(());
            }
            *values = self.preferred_step(component, values, targets, &pending, &tangent)?;
        }
        Ok(())
    }

    fn cursor_tangent(
        &mut self,
        component: &Component,
        rows: &Rows,
        factor: &Cholesky,
        pending: &[(usize, f64)],
    ) -> Result<CursorTangent, PreviewError> {
        let columns = component.free.len();
        self.meter.charge(
            linear_work(component, rows)
                .saturating_add(columns)
                .saturating_mul(pending.len()),
        )?;
        if rows.rows.len() == columns {
            return Ok(CursorTangent {
                axes: Vec::new(),
                gradient: vec![0.; columns],
                norm: 0.,
            });
        }
        let axes = factor.project_axes(&rows.rows, pending, columns);
        let gradient = weighted_axes(&axes, pending.iter().map(|&(_, value)| value), columns);
        let norm = gradient.iter().fold(0., |acc, &value| hypot(acc, value));
        check_finite(norm)?;
        Ok(CursorTangent {
            axes,
            gradient,
            norm,
        })
    }

    fn preferred_step(
        &mut self,
        component: &Component,
        values: &[f64],
        targets: &[(usize, f64)],
        pending: &[(usize, f64)],
        tangent: &CursorTangent,
    ) -> Result<Vec<f64>, PreviewError> {
        let columns = component.free.len();
        self.meter.charge(
            pending
                .len()
                .saturating_pow(3)
                .saturating_mul(3)
                .saturating_add(columns.saturating_mul(pending.len().saturating_add(2))),
        )?;
        let motion = fit_targets(
            &tangent.axes,
            pending,
            columns,
            self.program.options.rank_tolerance,
        )?;
        let prediction = directional_gain(&tangent.gradient, &motion);
        if prediction.is_finite() && prediction > 0. {
            return self.improving_retraction(component, values, targets, &motion, prediction);
        }
        // A target-space rank cutoff can discard an otherwise useful tangent direction.
        self.improving_retraction(
            component,
            values,
            targets,
            &tangent.gradient,
            tangent.norm * tangent.norm,
        )
    }

    fn improving_retraction(
        &mut self,
        component: &Component,
        values: &[f64],
        targets: &[(usize, f64)],
        motion: &[f64],
        prediction: f64,
    ) -> Result<Vec<f64>, PreviewError> {
        check_finite(prediction)?;
        for backtrack in 0..self.program.options.max_trial_steps {
            if let Some(next) = self.retracted_trial(component, values, motion, backtrack)? {
                self.meter.charge(targets.len().saturating_mul(8))?;
                if merit::improves(
                    self.program,
                    component,
                    values,
                    &next,
                    targets,
                    prediction * damping(backtrack),
                ) {
                    return Ok(next);
                }
            }
            self.rejected += 1;
        }
        Err(PreviewError::Incomplete(Incomplete::BlockedTarget))
    }

    fn report_preferred(
        &mut self,
        values: Vec<f64>,
        targets: &BTreeMap<usize, f64>,
        tolerance: f64,
    ) -> Result<(Update, PreferenceReport), PreviewError> {
        self.meter.charge(targets.len())?;
        let cursor_error = cursor_miss(self.program, &values, targets)?;
        let target_reached = cursor_error <= self.program.options.relative_tolerance;
        let mut max_gradient: f64 = 0.;
        let verified = self.report_with(values, |run, component, rows, factor, values| {
            run.meter.charge(component.free.len())?;
            let targets = local_targets(component, targets);
            if !targets.is_empty() {
                let (_, pending) = run.preference(component, values, &targets);
                let current = run.cursor_tangent(component, rows, factor, &pending)?.norm;
                max_gradient = max_gradient.max(current);
            }
            Ok(())
        })?;
        if !target_reached && max_gradient > tolerance {
            return Err(PreviewError::Incomplete(Incomplete::Iterations));
        }
        let settlement = if target_reached {
            PreferenceSettlement::TargetReached
        } else {
            PreferenceSettlement::FirstOrderStationary
        };
        Ok((
            verified,
            PreferenceReport {
                settlement,
                scaled_cursor_error: cursor_error,
                max_tangential_gradient: max_gradient,
            },
        ))
    }
}

fn check_finite(value: f64) -> Result<(), PreviewError> {
    if !value.is_finite() {
        return Err(SolveError::Arithmetic(format!(
            "projected cursor measure {value}: expected finite scaled arithmetic"
        ))
        .into());
    }
    Ok(())
}

fn cursor_miss(
    program: &Prepared,
    values: &[f64],
    targets: &BTreeMap<usize, f64>,
) -> Result<f64, PreviewError> {
    let result = targets.iter().fold(0., |acc, (&i, &target)| {
        hypot(acc, (target - values[i]) / program.domains[i].scale())
    });
    check_finite(result)?;
    Ok(result)
}

fn directional_gain(gradient: &[f64], motion: &[f64]) -> f64 {
    gradient
        .iter()
        .zip(motion)
        .fold(0., |acc, (&a, &b)| a.mul_add(b, acc))
}

fn damping(backtrack: usize) -> f64 {
    i32::try_from(backtrack).map_or(0., |exponent| powi(0.5, exponent))
}
