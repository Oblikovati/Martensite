//! Native shell stages borrow progress only for the duration of this operation.
use super::ShellSplitter;
use crate::bop_algo::{ProgressCompletion, ProgressOperation, ProgressStage, ProgressWorksite};
use crate::bop_tools::ConnexityBlock;
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::Failure,
};

/// Execute the actual shell receiver with an injected interval; `perform_shell_with_progress(&mut splitter, &range)?`.
pub(crate) fn perform_shell_with_progress(
    splitter: &mut ShellSplitter,
    range: &ProgressRange<'_>,
) -> ProgressCompletion {
    let result = run_shell_stages(splitter, range);
    match result {
        Ok(completion) => completion,
        Err(cause) => crate::bop_algo::builder_area::progress::classify_area_progress(
            ProgressOperation::ShellSplitter,
            splitter.options(),
            Err(cause),
            &crate::bop_algo::ProgressCancellation::default(),
        ),
    }
}

fn run_shell_stages(
    splitter: &mut ShellSplitter,
    range: &ProgressRange<'_>,
) -> Result<ProgressCompletion, Failure> {
    splitter.options.clear();
    let scope = ProgressScope::with_literal(range, "Building shells", 1., false);
    splitter.prepare_blocks()?;
    if splitter.options.user_break(&scope) {
        return Ok(cancelled(ProgressStage::Preparation));
    }
    splitter.make_shells_with_progress(&scope.next(1.))
}

impl ShellSplitter {
    fn make_shells_with_progress(
        &mut self,
        range: &ProgressRange<'_>,
    ) -> Result<ProgressCompletion, Failure> {
        let scope = ProgressScope::unnamed(range, 1., false);
        self.shells.clear();
        let irregular = self.regular_shells(&scope)?;
        if self.options.has_errors() {
            return Ok(cancelled(ProgressStage::Loops));
        }
        let parallel = ProgressScope::unnamed(&scope.next(1.), irregular.len() as f64, false);
        let completion = split_irregular_tasks(
            &mut self.blocks,
            &irregular,
            &parallel,
            self.options.run_parallel(),
        )?;
        self.append_irregular(&irregular)?;
        Ok(completion)
    }
}

struct ShellBlockTask<'task, 'progress> {
    block: &'task mut ConnexityBlock,
    scope: &'task ProgressScope<'progress>,
    outcome: Result<ProgressCompletion, Failure>,
}

fn run_block(
    block: &mut ConnexityBlock,
    scope: &ProgressScope<'_>,
) -> Result<ProgressCompletion, Failure> {
    let _range = scope.next(1.);
    if !scope.more() {
        // Native CBK::Perform skips the block without adding a local alert.
        return Ok(cancelled(ProgressStage::Worker));
    }
    ShellSplitter::split_block(block)?;
    Ok(ProgressCompletion::Completed)
}

fn execute_tasks(tasks: &mut [ShellBlockTask<'_, '_>], parallel: bool) -> Result<(), Failure> {
    crate::bop_algo::run_context_tasks(
        tasks,
        &(),
        parallel,
        || (),
        |task, _| {
            let completion = run_block(task.block, task.scope)?;
            task.outcome = Ok(completion);
            Ok(())
        },
    )
}

fn split_irregular_tasks(
    blocks: &mut [ConnexityBlock],
    irregular: &[usize],
    scope: &ProgressScope<'_>,
    parallel: bool,
) -> Result<ProgressCompletion, Failure> {
    let scopes = block_scopes(scope, irregular.len());
    let mut tasks = block_tasks(blocks, irregular, &scopes);
    execute_tasks(&mut tasks, parallel)?;
    for task in tasks {
        let completion = task.outcome?;
        if !matches!(completion, ProgressCompletion::Completed) {
            return Ok(completion);
        }
    }
    Ok(ProgressCompletion::Completed)
}

fn block_scopes<'a>(scope: &'a ProgressScope<'a>, count: usize) -> Vec<ProgressScope<'a>> {
    (0..count)
        .map(|_| ProgressScope::unnamed(&scope.next(1.), 1., false))
        .collect()
}

fn block_tasks<'task, 'progress>(
    blocks: &'task mut [ConnexityBlock],
    irregular: &[usize],
    scopes: &'task [ProgressScope<'progress>],
) -> Vec<ShellBlockTask<'task, 'progress>> {
    let selected = blocks
        .iter_mut()
        .enumerate()
        .filter(|(index, _)| irregular.contains(index));
    selected
        .zip(scopes)
        .map(|((_, block), scope)| ShellBlockTask {
            block,
            scope,
            outcome: Ok(pending_block()),
        })
        .collect()
}

fn cancelled(stage: ProgressStage) -> ProgressCompletion {
    ProgressCompletion::Cancelled {
        operation: ProgressOperation::ShellSplitter,
        stage,
        context: Some(ProgressWorksite {
            label: "BOPAlgoShellSplitter",
            input_ordinal: None,
            entity_key: None,
        }),
    }
}

fn pending_block() -> ProgressCompletion {
    ProgressCompletion::Incomplete {
        operation: ProgressOperation::ShellSplitter,
        stage: ProgressStage::Worker,
        class: crate::bop_algo::ProgressFailureClass::UnqualifiedStage,
        cause: None,
        context: None,
    }
}
