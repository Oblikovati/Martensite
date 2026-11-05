//! Pair tasks own native scalar defaults and lexical ranges; coordinator maps remain untouched.
use super::{Context, Shape, query, required_context};
use crate::bop_algo::{
    Options, ProgressCancellation, builder_area::progress::observe_user_break,
    run_launcher_context_tasks,
};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::{Failure, Handle},
};

/// Joined pair evidence has no progress borrow or worker report; `result.same_domain` is the actual flag.
pub(super) struct CheckedSameDomainPair {
    pub(super) first: Shape,
    pub(super) second: Shape,
    pub(super) same_domain: bool,
}

struct SameDomainPairWork<'p> {
    first: Shape,
    second: Shape,
    same_domain: bool,
    options: Options,
    progress: Option<ProgressRange<'p>>,
}

impl<'p> SameDomainPairWork<'p> {
    fn new((first, second): (Shape, Shape), fuzzy: f64, range: ProgressRange<'p>) -> Self {
        let mut options = Options::new();
        options.set_fuzzy_value(fuzzy);
        Self {
            first,
            second,
            same_domain: false,
            options,
            progress: Some(range),
        }
    }

    fn perform(
        &mut self,
        context: &Handle<Context>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let range = self.take_range()?;
        let scope = ProgressScope::unnamed(&range, 1.0, false);
        if observe_user_break(&self.options, &scope, observed) {
            return Ok(());
        }
        self.same_domain = query::same_domain_pair(
            &self.first,
            &self.second,
            required_context(context)?,
            self.options.fuzzy_value(),
        )?;
        Ok(())
    }

    fn take_range(&mut self) -> Result<ProgressRange<'p>, Failure> {
        self.progress.take().ok_or_else(|| {
            Failure::with_message(
                "Standard_ProgramError",
                "SD pair progress=None; expected one fresh task range",
            )
        })
    }

    fn into_result(self) -> CheckedSameDomainPair {
        CheckedSameDomainPair {
            first: self.first,
            second: self.second,
            same_domain: self.same_domain,
        }
    }
}

/// Use the actual context overload even serially; `check_pairs(pairs, context, false, fuzzy, range, observed)?`.
/// Empty work never dereferences a possibly-null caller context, matching native filtered FF input.
pub(super) fn check_pairs(
    pairs: Vec<(Shape, Shape)>,
    context: &Handle<Context>,
    parallel: bool,
    fuzzy: f64,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
) -> Result<Vec<CheckedSameDomainPair>, Failure> {
    let mut work: Vec<_> = pairs
        .into_iter()
        .map(|pair| SameDomainPairWork::new(pair, fuzzy, scope.next(1.0)))
        .collect();
    run_pair_callbacks(&mut work, context, parallel, observed)?;
    Ok(work
        .into_iter()
        .map(SameDomainPairWork::into_result)
        .collect())
}

fn new_worker_context() -> Handle<Context> {
    // The existing launcher owns lazy worker slots; the caller's context is never replaced.
    Handle::new(Context::default())
}

fn run_pair_callbacks(
    work: &mut [SameDomainPairWork<'_>],
    context: &Handle<Context>,
    parallel: bool,
    observed: &ProgressCancellation,
) -> Result<(), Failure> {
    run_launcher_context_tasks(
        work,
        context,
        parallel,
        new_worker_context,
        |task, actual| task.perform(actual, observed),
    )
}

/// Keep the actual six-unit scope with its coordinator; `let checking = checking_scope(outer, count)`.
/// Its native lifetime extends through post-join polls, map binding and origin updates, including zero tasks.
pub(super) fn checking_scope<'a>(outer: &'a ProgressScope<'a>, count: usize) -> ProgressScope<'a> {
    let range = outer.next(6.0);
    ProgressScope::with_literal(&range, "Checking SD faces", count as f64, false)
}

#[cfg(test)]
#[path = "tasks_contract.rs"]
mod contract;
