// SPDX-License-Identifier: GPL-2.0-only

use super::{factor::Cholesky, policy::Meter, *};
use martensite_cmath::{hypot, powi};

mod preference;
pub(super) use preference::{run_preferred, verify_preferred};

pub(super) fn run(
    program: &Prepared,
    mut accepted: Vec<f64>,
    targets: &BTreeMap<usize, f64>,
    mut cancelled: impl FnMut() -> bool,
) -> Result<Update, PreviewError> {
    let mut run = Run {
        program,
        meter: Meter::new(program.limits, &mut cancelled),
        rejected: 0,
    };
    run.check_warm_state(&accepted)?;
    for component in program.components.iter() {
        run.meter.charge(component.free.len())?;
        let targets = local_targets(component, targets);
        if !targets.is_empty() {
            run.follow(component, &mut accepted, &targets)?;
        }
    }
    if let Some(incomplete) = fixed_target_refusal(program, targets) {
        return Err(PreviewError::Incomplete(incomplete));
    }
    run.report(accepted)
}

fn local_targets(component: &Component, targets: &BTreeMap<usize, f64>) -> Vec<(usize, f64)> {
    component
        .free
        .iter()
        .enumerate()
        .filter_map(|(i, global)| targets.get(global).map(|&v| (i, v)))
        .collect()
}

fn fixed_target_refusal(program: &Prepared, targets: &BTreeMap<usize, f64>) -> Option<Incomplete> {
    program
        .fixed
        .iter()
        .enumerate()
        .any(|(i, &fixed)| {
            fixed.is_some_and(|value| {
                targets.get(&i).is_some_and(|&target| {
                    (target - value).abs() / program.domains[i].scale()
                        > program.options.relative_tolerance
                })
            })
        })
        .then_some(Incomplete::BlockedTarget)
}

struct Run<'a> {
    program: &'a Prepared,
    meter: Meter<'a>,
    rejected: usize,
}

impl Run<'_> {
    fn check_warm_state(&mut self, values: &[f64]) -> Result<(), PreviewError> {
        self.meter.poll()?;
        for component in self.program.components.iter() {
            let rows = self.evaluate(component, values)?;
            if rows.maximum > self.program.options.relative_tolerance {
                return Err(PreviewError::Incomplete(Incomplete::InfeasibleWarmState));
            }
        }
        Ok(())
    }

    fn restore_rows(
        &mut self,
        component: &Component,
        mut values: Vec<f64>,
        mut rows: Rows,
    ) -> Result<Option<Vec<f64>>, PreviewError> {
        for _ in 0..self.program.options.max_iterations {
            if rows.maximum <= self.program.options.relative_tolerance {
                return Ok(Some(values));
            }
            self.meter.poll()?;
            let factor = self.factor(component, &rows)?;
            self.meter.charge(linear_work(component, &rows))?;
            let step = factor.correction(&rows.rows, &rows.rhs, component.free.len());
            let Some((next, evaluated)) =
                self.repair_step(component, &values, &step, rows.maximum)?
            else {
                return Ok(None);
            };
            values = next;
            rows = evaluated;
        }
        Ok((rows.maximum <= self.program.options.relative_tolerance).then_some(values))
    }

    fn factor(&mut self, component: &Component, rows: &Rows) -> Result<Cholesky, PreviewError> {
        Cholesky::new(
            &rows.rows,
            component.free.len(),
            self.program.options.rank_tolerance,
            &mut self.meter,
        )
    }

    fn evaluate(&mut self, component: &Component, values: &[f64]) -> Result<Rows, PreviewError> {
        self.program
            .evaluate(component, values, |work| self.meter.charge(work))
    }

    fn repair_step(
        &mut self,
        component: &Component,
        values: &[f64],
        step: &[f64],
        previous: f64,
    ) -> Result<Option<(Vec<f64>, Rows)>, PreviewError> {
        for backtrack in 0..self.program.options.max_trial_steps {
            self.meter.poll()?;
            self.meter.charge(values.len())?;
            if let Some(trial) = self.trial(component, values, step, backtrack)
                && let Some(rows) = self.trial_rows(component, &trial)?
                && rows.maximum < previous
            {
                return Ok(Some((trial, rows)));
            }
            self.rejected += 1;
        }
        Ok(None)
    }

    fn follow(
        &mut self,
        component: &Component,
        values: &mut Vec<f64>,
        targets: &[(usize, f64)],
    ) -> Result<(), PreviewError> {
        for _ in 0..self.program.options.max_iterations {
            self.meter.poll()?;
            let (error, pending) = self.preference(component, values, targets);
            if error <= self.program.options.relative_tolerance {
                return Ok(());
            }
            let rows = self.evaluate(component, values)?;
            let factor = self.factor(component, &rows)?;
            let width = targets.len();
            self.meter.charge(
                linear_work(component, &rows)
                    .saturating_mul(width)
                    .saturating_add(width.saturating_pow(3).saturating_mul(3)),
            )?;
            let motion = factor.preference(
                &rows.rows,
                &pending,
                component.free.len(),
                self.program.options.rank_tolerance,
            )?;
            if motion.iter().all(|&v| v == 0.) {
                return Err(PreviewError::Incomplete(Incomplete::BlockedTarget));
            }
            if let Some(next) = self.retract(component, values, targets, &motion, error)? {
                *values = next;
            } else {
                return Err(PreviewError::Incomplete(Incomplete::BlockedTarget));
            }
        }
        let error = self.preference(component, values, targets).0;
        if error > self.program.options.relative_tolerance {
            return Err(PreviewError::Incomplete(Incomplete::Iterations));
        }
        Ok(())
    }

    fn retract(
        &mut self,
        component: &Component,
        values: &[f64],
        targets: &[(usize, f64)],
        step: &[f64],
        previous: f64,
    ) -> Result<Option<Vec<f64>>, PreviewError> {
        for backtrack in 0..self.program.options.max_trial_steps {
            if let Some(next) = self.retracted_trial(component, values, step, backtrack)? {
                let error = self.preference(component, &next, targets).0;
                let threshold = self.program.options.relative_tolerance.min(previous * 1e-6);
                if previous - error > threshold {
                    return Ok(Some(next));
                }
            }
            self.rejected += 1;
        }
        Ok(None)
    }

    fn retracted_trial(
        &mut self,
        component: &Component,
        values: &[f64],
        step: &[f64],
        backtrack: usize,
    ) -> Result<Option<Vec<f64>>, PreviewError> {
        self.meter.poll()?;
        self.meter.charge(values.len())?;
        let Some(trial) = self.trial(component, values, step, backtrack) else {
            return Ok(None);
        };
        let Some(rows) = self.trial_rows(component, &trial)? else {
            return Ok(None);
        };
        self.restore_rows(component, trial, rows)
    }

    fn preference(
        &self,
        component: &Component,
        values: &[f64],
        targets: &[(usize, f64)],
    ) -> (f64, Vec<(usize, f64)>) {
        let pending = targets
            .iter()
            .map(|&(i, target)| {
                let global = component.free[i];
                (
                    i,
                    (target - values[global]) / self.program.domains[global].scale(),
                )
            })
            .collect::<Vec<_>>();
        let norm = pending
            .iter()
            .fold(0.0_f64, |sum, &(_, delta)| hypot(sum, delta));
        (norm, pending)
    }

    fn trial(
        &self,
        component: &Component,
        values: &[f64],
        step: &[f64],
        backtrack: usize,
    ) -> Option<Vec<f64>> {
        let damping = powi(0.5, i32::try_from(backtrack).ok()?);
        let mut result = values.to_vec();
        for (&index, &delta) in component.free.iter().zip(step) {
            if delta != 0. {
                result[index] += delta * damping * self.program.domains[index].scale();
            }
            if !result[index].is_finite() {
                return None;
            }
        }
        Some(result)
    }

    fn trial_rows(
        &mut self,
        component: &Component,
        values: &[f64],
    ) -> Result<Option<Rows>, PreviewError> {
        let evaluated = self.evaluate(component, values);
        match evaluated {
            Ok(rows) => Ok(Some(rows)),
            Err(PreviewError::Solve(
                SolveError::Evaluation(EvaluationError::Source { .. }) | SolveError::Arithmetic(_),
            )) => Ok(None),
            Err(error) => Err(error),
        }
    }

    fn report(&mut self, values: Vec<f64>) -> Result<Update, PreviewError> {
        self.report_with(values, |_, _, _, _, _| Ok(()))
    }

    fn report_with(
        &mut self,
        values: Vec<f64>,
        mut inspect: impl FnMut(
            &mut Self,
            &Component,
            &Rows,
            &Cholesky,
            &[f64],
        ) -> Result<(), PreviewError>,
    ) -> Result<Update, PreviewError> {
        let mut rank = self.program.fixed.iter().flatten().count();
        let mut hard_error = 0.0_f64;
        let tolerance = self.program.options.relative_tolerance;
        for component in self.program.components.iter() {
            let rows = self.evaluate(component, &values)?;
            hard_error = hard_error.max(rows.maximum);
            if hard_error > tolerance {
                return Err(PreviewError::Incomplete(Incomplete::FinalHardResidual {
                    observed: hard_error,
                    tolerance,
                }));
            }
            let factor = self.factor(component, &rows)?;
            rank += factor.rank();
            inspect(self, component, &rows, &factor, &values)?;
        }
        self.meter.poll()?;
        Ok(Update {
            dof: values.len() - rank,
            values,
            rank,
            hard_error,
            work: self.meter.work(),
            rejected_trials: self.rejected,
        })
    }
}

fn linear_work(component: &Component, rows: &Rows) -> usize {
    let row_entries = rows
        .rows
        .iter()
        .map(Vec::len)
        .fold(0_usize, usize::saturating_add);
    let row_square = rows.rows.len().saturating_pow(2);
    component
        .free
        .len()
        .saturating_add(row_entries)
        .saturating_add(row_square)
}
