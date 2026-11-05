//! Native VE work borrows each actual worker context and keeps its range on the worker stack.
use super::{VertexEdgeSolver, missing_structure};
use crate::bop_algo::pave_filler::progress_observation::native_user_break;
use crate::bop_algo::{
    AlertIntersectionFailed, Options, ProgressCancellation, ProgressCompletion,
    ProgressFailureClass, ProgressOperation, ProgressStage, ProgressWorksite,
    run_launcher_context_tasks,
};
use crate::int_tools::Context;
use martensite_tkbrep::topods::{Edge, Vertex};
use martensite_tkernel::{
    message::{AlertHandle, ProgressRange, ProgressScope},
    standard::{Failure, Handle},
};
use std::sync::Arc;

/// Owns the native VE solver values and terminal diagnostics, without a borrowed range.
/// Example: `VertexEdgeWork::new(vertex_index, edge_index, shapes, fuzzy)`.
pub(super) struct VertexEdgeWork {
    pub(super) solver: VertexEdgeSolver,
    vertex: Vertex,
    edge: Edge,
    options: Options,
    failure: Option<Failure>,
    completion: Option<ProgressCompletion>,
}

impl VertexEdgeWork {
    /// Prepares the same native scalar defaults and actual topology inputs.
    /// Example: `VertexEdgeWork::new(vertex_index, edge_index, shapes, fuzzy)`.
    pub(super) fn new(vertex: i32, edge: i32, shapes: (Vertex, Edge), fuzzy: f64) -> Self {
        let mut options = Options::new();
        options.set_fuzzy_value(fuzzy);
        Self {
            solver: VertexEdgeSolver::uncomputed(vertex, edge),
            vertex: shapes.0,
            edge: shapes.1,
            options,
            failure: None,
            completion: None,
        }
    }

    /// Requires the actual callback result before coordinator publication.
    /// Example: `let completion = work.take_completion()?;`.
    pub(super) fn take_completion(&mut self) -> Result<ProgressCompletion, Failure> {
        self.completion.take().ok_or_else(|| {
            Failure::new(&format!(
                "VE pair ({}, {}), expected one actual worker completion",
                self.solver.vertex, self.solver.edge
            ))
        })
    }

    fn compute(&mut self, context: &Handle<Context>) -> Result<(), Failure> {
        let context = context.get().ok_or_else(missing_structure)?;
        match context.compute_ve(
            &self.vertex,
            &self.edge,
            &mut self.solver.parameter,
            &mut self.solver.new_tolerance,
            self.options.fuzzy_value(),
        ) {
            Ok(flag) => self.solver.flag = flag,
            Err(cause) => self.record_failure(cause),
        }
        Ok(())
    }

    fn record_failure(&mut self, cause: Failure) {
        self.failure = Some(cause);
        let alert: AlertHandle = Arc::new(AlertIntersectionFailed);
        self.options.add_error(&alert);
    }

    fn classify(&self, observed: &ProgressCancellation, ordinal: usize) -> ProgressCompletion {
        if self.failure.is_some() {
            return self
                .failed_completion(ve_worksite(ordinal), ProgressFailureClass::KernelFailure);
        }
        if observed.is_cancelled() {
            return cancelled_ve_completion(ordinal);
        }
        if self.options.has_errors() {
            return self
                .failed_completion(ve_worksite(ordinal), ProgressFailureClass::AlgorithmReport);
        }
        ProgressCompletion::Completed
    }

    fn failed_completion(
        &self,
        context: Option<ProgressWorksite>,
        class: ProgressFailureClass,
    ) -> ProgressCompletion {
        ProgressCompletion::Failed {
            operation: ProgressOperation::PaveFiller,
            stage: ProgressStage::Worker,
            class,
            cause: self.failure.clone(),
            context,
        }
    }
}

/// Uses the native context overload, including its serial launcher catcher.
/// Example: `execute_vertex_edge_work(work, main, scope, parallel)?`.
pub(super) fn execute_vertex_edge_work(
    work: &mut [VertexEdgeWork],
    main: &Handle<Context>,
    scope: &ProgressScope<'_>,
    parallel: bool,
) -> Result<(), Failure> {
    main.get().ok_or_else(missing_structure)?;
    let mut invocations = vertex_edge_invocations(work, scope);
    run_launcher_context_tasks(
        &mut invocations,
        main,
        parallel,
        || Handle::new(Context::default()),
        |invocation, context| invocation.perform(context),
    )
}

// The lexical envelope is Send but not Sync: its owned range Cell is never shared.
struct VertexEdgeInvocation<'w, 'p> {
    ordinal: usize,
    work: &'w mut VertexEdgeWork,
    progress: Option<ProgressRange<'p>>,
}

impl VertexEdgeInvocation<'_, '_> {
    fn perform(&mut self, context: &Handle<Context>) -> Result<(), Failure> {
        let range = self.progress.take().ok_or_else(|| {
            Failure::new(&format!(
                "VE invocation {}, expected one unconsumed worker range",
                self.ordinal
            ))
        })?;
        perform_vertex_edge_work(self.work, context, &range, self.ordinal)
    }
}

fn vertex_edge_invocations<'w, 'p>(
    work: &'w mut [VertexEdgeWork],
    scope: &'p ProgressScope<'p>,
) -> Vec<VertexEdgeInvocation<'w, 'p>> {
    // Allocate native task ranges sequentially; no task scope exists before its worker runs.
    work.iter_mut()
        .enumerate()
        .map(|(ordinal, work)| VertexEdgeInvocation {
            ordinal,
            work,
            progress: Some(scope.next(1.0)),
        })
        .collect()
}

fn perform_vertex_edge_work(
    work: &mut VertexEdgeWork,
    context: &Handle<Context>,
    range: &ProgressRange<'_>,
    ordinal: usize,
) -> Result<(), Failure> {
    let scope = ProgressScope::unnamed(range, 1.0, false);
    let observed = ProgressCancellation::default();
    work.failure = None;
    work.completion = None;
    if !native_user_break(&work.options, &scope, &observed) {
        work.compute(context)?;
    }
    work.solver.failed = work.options.has_errors();
    work.completion = Some(work.classify(&observed, ordinal));
    Ok(())
}

fn ve_worksite(ordinal: usize) -> Option<ProgressWorksite> {
    Some(ProgressWorksite {
        label: "VertexEdge",
        input_ordinal: Some(ordinal),
        entity_key: None,
    })
}

fn cancelled_ve_completion(ordinal: usize) -> ProgressCompletion {
    ProgressCompletion::Cancelled {
        operation: ProgressOperation::PaveFiller,
        stage: ProgressStage::Worker,
        context: ve_worksite(ordinal),
    }
}

#[cfg(test)]
#[path = "contract.rs"]
mod contract;
