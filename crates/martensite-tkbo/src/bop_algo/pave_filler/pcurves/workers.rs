//! One owned progress range and one terminal packet per actual context-dependent pcurve worker.
use super::*;
use crate::bop_algo::{
    Options, ProgressFailureClass, ProgressOperation, ProgressStage, ProgressWorksite,
    run_launcher_context_tasks,
};
use crate::int_tools::Context;
struct PCurveInvocation<'a> {
    ordinal: usize,
    task: PCurveTask,
    range: Option<ProgressRange<'a>>,
    completion: Option<ProgressCompletion>,
}
/// Join context workers before any coordinator pcurve attachment; `execute_pcurve_tasks(tasks, scope, context, false)?`.
pub(super) fn execute_pcurve_tasks(
    tasks: Vec<PCurveTask>,
    scope: &ProgressScope<'_>,
    context: &Handle<Context>,
    parallel: bool,
) -> Result<Vec<(PCurveTask, ProgressCompletion)>, Failure> {
    let mut work = allocate_pcurve_invocations(tasks, scope);
    run_launcher_context_tasks(
        &mut work,
        context,
        parallel,
        || Handle::new(Context::default()),
        |item, context| item.perform(context),
    )?;
    work.into_iter()
        .map(PCurveInvocation::into_output)
        .collect()
}
fn allocate_pcurve_invocations<'a>(
    tasks: Vec<PCurveTask>,
    scope: &'a ProgressScope<'a>,
) -> Vec<PCurveInvocation<'a>> {
    tasks
        .into_iter()
        .enumerate()
        .map(|(ordinal, task)| PCurveInvocation {
            ordinal,
            task,
            range: Some(scope.next(1.)),
            completion: None,
        })
        .collect()
}
impl<'a> PCurveInvocation<'a> {
    /// Native `BOPAlgoMPC::Perform`; exceptions mark the task as failed.
    fn perform(&mut self, context: &Handle<Context>) -> Result<(), Failure> {
        let range = self.take_range()?;
        let scope = ProgressScope::unnamed(&range, 1., false);
        let observed = ProgressCancellation::default();
        let outcome = self.compute_pcurve(&scope, &observed, context);
        self.task.failed |= outcome.is_err();
        self.completion = Some(pcurve_worker_completion(
            &outcome,
            self.task.failed,
            observed.is_cancelled(),
            self.ordinal,
        ));
        Ok(())
    }
    fn take_range(&mut self) -> Result<ProgressRange<'a>, Failure> {
        self.range.take().ok_or_else(|| {
            Failure::new(&format!(
                "pcurve task {} has no range; expected one unused owned range",
                self.ordinal
            ))
        })
    }
    fn compute_pcurve(
        &mut self,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
        context: &Handle<Context>,
    ) -> Result<(), Failure> {
        if native_user_break(&Options::default(), scope, observed) {
            // Native worker UserBreak sets a local report, later converted to a projection warning.
            self.task.failed = true;
            return Ok(());
        }
        self.task.perform_on(context)
    }
    fn into_output(self) -> Result<(PCurveTask, ProgressCompletion), Failure> {
        let completion = self.completion.ok_or_else(|| {
            Failure::new(&format!(
                "pcurve task {} has no completion; expected one terminal worker result",
                self.ordinal
            ))
        })?;
        Ok((self.task, completion))
    }
}
fn pcurve_worker_completion(
    outcome: &Result<(), Failure>,
    reported: bool,
    cancelled: bool,
    ordinal: usize,
) -> ProgressCompletion {
    if let Err(cause) = outcome {
        return pcurve_failure(
            ordinal,
            ProgressFailureClass::KernelFailure,
            Some(cause.clone()),
        );
    }
    if cancelled {
        return ProgressCompletion::Cancelled {
            operation: ProgressOperation::PaveFiller,
            stage: ProgressStage::Worker,
            context: pcurve_worksite(ordinal),
        };
    }
    if reported {
        return pcurve_failure(ordinal, ProgressFailureClass::AlgorithmReport, None);
    }
    ProgressCompletion::Completed
}
fn pcurve_failure(
    ordinal: usize,
    class: ProgressFailureClass,
    cause: Option<Failure>,
) -> ProgressCompletion {
    ProgressCompletion::Failed {
        operation: ProgressOperation::PaveFiller,
        stage: ProgressStage::Worker,
        class,
        cause,
        context: pcurve_worksite(ordinal),
    }
}
fn pcurve_worksite(ordinal: usize) -> Option<ProgressWorksite> {
    Some(ProgressWorksite {
        label: "MakePCurves",
        input_ordinal: Some(ordinal),
        entity_key: None,
    })
}

#[cfg(test)]
#[path = "worker_contract.rs"]
mod tests;
