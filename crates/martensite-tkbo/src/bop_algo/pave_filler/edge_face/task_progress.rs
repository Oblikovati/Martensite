//! Native task-entry cancellation precedes geometry and retains independent failure causes.
use super::task::EdgeFaceTask;
use crate::bop_algo::pave_filler::progress_observation::native_user_break;
use crate::bop_algo::{
    Options, ProgressCancellation, ProgressCompletion, ProgressFailureClass, ProgressOperation,
    ProgressStage, ProgressWorksite,
};
use crate::int_tools::Context;
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::Handle,
};

impl EdgeFaceTask {
    /// Polls the one native worker scope before accessing the edge or face.
    /// Example: `task.perform_with_range(range, observed, ordinal, context, "PerformEF")`.
    pub(super) fn perform_with_range(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
        ordinal: usize,
        context: &Handle<Context>,
        worksite: &'static str,
    ) -> ProgressCompletion {
        let scope = ProgressScope::unnamed(range, 1.0, false);
        let options = Options::default();
        if !native_user_break(&options, &scope, observed) {
            self.perform(context);
        }
        self.edge_face_task_completion(observed, ordinal, worksite)
    }

    fn edge_face_task_completion(
        &self,
        observed: &ProgressCancellation,
        ordinal: usize,
        worksite: &'static str,
    ) -> ProgressCompletion {
        let context = Some(edge_face_task_worksite(ordinal, worksite));
        if self.failure.is_some() {
            return self.edge_face_task_failure(context);
        }
        if observed.is_cancelled() {
            return cancelled_edge_face_task(context);
        }
        if self.failed() {
            return self.edge_face_task_failure(context);
        }
        ProgressCompletion::Completed
    }

    fn edge_face_task_failure(&self, context: Option<ProgressWorksite>) -> ProgressCompletion {
        let class = if self.failure.is_some() {
            ProgressFailureClass::KernelFailure
        } else {
            ProgressFailureClass::AlgorithmReport
        };
        ProgressCompletion::Failed {
            operation: ProgressOperation::PaveFiller,
            stage: ProgressStage::Worker,
            class,
            cause: self.failure.clone(),
            context,
        }
    }
}

fn edge_face_task_worksite(ordinal: usize, worksite: &'static str) -> ProgressWorksite {
    ProgressWorksite {
        label: worksite,
        input_ordinal: Some(ordinal),
        entity_key: None,
    }
}

fn cancelled_edge_face_task(context: Option<ProgressWorksite>) -> ProgressCompletion {
    ProgressCompletion::Cancelled {
        operation: ProgressOperation::PaveFiller,
        stage: ProgressStage::Worker,
        context,
    }
}

#[cfg(test)]
#[path = "task_progress_contract.rs"]
mod tests;
