//! Native preparation/storage polls surround joined context workers without inventing parent checks.
use super::super::progress_observation::native_user_break;
use super::*;
use crate::bop_algo::{ProgressCancellation, ProgressCompletion};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
#[path = "workers.rs"]
mod workers;
impl BOPAlgoPaveFiller {
    /// Retain actual worker outcomes alongside native warning-only and cancelled diagnostic state.
    /// Example: `filler.make_pcurves_with_progress(range, observed)?` before degenerated edges.
    pub(in crate::bop_algo::pave_filler) fn make_pcurves_with_progress(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        let outer = ProgressScope::unnamed(range, 1., false);
        let attribute = self.section_attribute;
        if self.avoid_build_pcurve || (!attribute.pcurve_on_s1() && !attribute.pcurve_on_s2()) {
            return Ok(Vec::new());
        }
        let Some(mut tasks) = self.face_block_pcurve_tasks(&outer, observed)? else {
            return Ok(Vec::new());
        };
        tasks.extend(
            self.section_pcurve_tasks([attribute.pcurve_on_s1(), attribute.pcurve_on_s2()])?,
        );
        self.run_pcurve_tasks(tasks, &outer, observed)
    }
    fn run_pcurve_tasks(
        &mut self,
        tasks: Vec<PCurveTask>,
        outer: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        let scope = ProgressScope::with_literal(
            &outer.next(1.),
            "Projecting edges on faces",
            tasks.len() as f64,
            false,
        );
        let outputs = workers::execute_pcurve_tasks(
            tasks,
            &scope,
            &self.context,
            self.options.run_parallel(),
        )?;
        self.commit_pcurve_outputs(outputs, outer, observed)
    }
    fn commit_pcurve_outputs(
        &mut self,
        outputs: Vec<(PCurveTask, ProgressCompletion)>,
        outer: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        let (tasks, outcomes): (Vec<PCurveTask>, Vec<ProgressCompletion>) =
            outputs.into_iter().unzip();
        if outcomes
            .iter()
            .any(|outcome| matches!(outcome, ProgressCompletion::Cancelled { .. }))
        {
            observed.record_cancelled();
        }
        if !self.options.has_errors() {
            self.store_pcurve_tasks(tasks, outer, observed)?;
        }
        Ok(outcomes)
    }
    fn store_pcurve_tasks(
        &mut self,
        tasks: Vec<PCurveTask>,
        outer: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        for task in tasks {
            if native_user_break(&self.options, outer, observed) {
                return Ok(());
            }
            self.store_pcurve(task)?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "census.rs"]
mod census;

#[cfg(test)]
#[path = "contract.rs"]
mod tests;
