//! Each task owns its native range without sharing its Cell or inserting an extra ancestor.
use super::WireSplitter;
use crate::{
    bop_algo::{ProgressCancellation, run_launcher_context_tasks},
    bop_tools::ConnexityBlock,
    int_tools::Context,
};
use martensite_tkbrep::topods::Face;
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::{Failure, Handle},
};

struct WireBlockTask<'b, 'p> {
    ordinal: usize,
    block: &'b mut ConnexityBlock,
    range: Option<ProgressRange<'p>>,
}

impl WireBlockTask<'_, '_> {
    fn perform(
        &mut self,
        face: &Face,
        context: &Handle<Context>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let range = self.range.take().ok_or_else(|| {
            Failure::new(&format!(
                "wire block {} has no range; expected one unused owned range",
                self.ordinal
            ))
        })?;
        let scope = ProgressScope::unnamed(&range, 1., false);
        if !scope.more() {
            // Fresh unit scope: More=false is cancellation, not ordinary exhaustion.
            // Native WS task returns without a local alert; typed completion remembers it.
            observed.record_cancelled();
            return Ok(());
        }
        WireSplitter::split_block(face, self.block, context)
    }
}

/// Run real blocks with worker contexts; `split_irregular(face, blocks, context, parallel, range, observed)`.
/// Ordered block storage is retained after the executor joins; the caller publishes loops in input order.
pub(super) fn split_irregular(
    face: &Face,
    blocks: &mut [ConnexityBlock],
    context: &Handle<Context>,
    parallel: bool,
    range: &ProgressRange<'_>,
    observed: &ProgressCancellation,
) -> Result<(), Failure> {
    let scope = ProgressScope::unnamed(range, blocks.len() as f64, false);
    let mut tasks = wire_tasks(blocks, &scope);
    run_launcher_context_tasks(
        &mut tasks,
        context,
        parallel,
        new_worker_context,
        |task, worker_context| task.perform(face, worker_context, observed),
    )
}

fn wire_tasks<'b, 'p>(
    blocks: &'b mut [ConnexityBlock],
    scope: &'p ProgressScope<'p>,
) -> Vec<WireBlockTask<'b, 'p>> {
    blocks
        .iter_mut()
        .enumerate()
        .map(|(ordinal, block)| WireBlockTask {
            ordinal,
            block,
            range: Some(scope.next(1.)),
        })
        .collect()
}

fn new_worker_context() -> Handle<Context> {
    let context = Context::default();
    Handle::new(context)
}

#[cfg(test)]
#[path = "tasks_contract.rs"]
mod tests;
