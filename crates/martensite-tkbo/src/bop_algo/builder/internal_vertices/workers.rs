//! Lexical ranges move once to context-backed workers; insertion remains on the coordinator.
use super::*;
use crate::bop_algo::{Options, run_launcher_context_tasks};
use martensite_tkernel::standard::Handle;

struct InternalVertexWork<'a> {
    ordinal: usize,
    pair: (Shape, Shape),
    range: Option<ProgressRange<'a>>,
    options: Options,
    inside: bool,
    completed: bool,
}
impl InternalVertexWork<'_> {
    fn perform(
        &mut self,
        context: &Handle<Context>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let range = self.range.take().ok_or_else(|| {
            Failure::new(&format!(
                "internal vertex task {}; expected one unused range",
                self.ordinal
            ))
        })?;
        let scope = ProgressScope::unnamed(&range, 1.0, false);
        if !observe_user_break(&self.options, &scope, observed) {
            self.inside = internal_vertex_is_inside(
                &self.pair.0,
                &self.pair.1,
                required_context(context)?,
                self.options.fuzzy_value(),
            )?;
        }
        self.completed = true;
        Ok(())
    }
}

/// Reserve in input order and join before insertion; `classify_pairs(pairs, context, fuzzy, parallel, scope, observed)?`.
pub(super) fn classify_pairs(
    pairs: InternalVertexPairs,
    context: &Handle<Context>,
    fuzzy: f64,
    parallel: bool,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
) -> Result<InternalVertexPairs, Failure> {
    let mut work = reserve_pairs(pairs, scope, fuzzy);
    run_launcher_context_tasks(
        &mut work,
        context,
        parallel,
        || Handle::new(Context::default()),
        |item, actual| item.perform(actual, observed),
    )?;
    collect_inside(work)
}

fn reserve_pairs<'a>(
    pairs: InternalVertexPairs,
    scope: &'a ProgressScope<'a>,
    fuzzy: f64,
) -> Vec<InternalVertexWork<'a>> {
    pairs
        .into_iter()
        .enumerate()
        .map(|(ordinal, pair)| {
            let mut options = Options::new();
            options.set_fuzzy_value(fuzzy);
            InternalVertexWork {
                ordinal,
                pair,
                range: Some(scope.next(1.0)),
                options,
                inside: false,
                completed: false,
            }
        })
        .collect()
}

fn collect_inside(work: Vec<InternalVertexWork<'_>>) -> Result<InternalVertexPairs, Failure> {
    let mut inside = Vec::new();
    for item in work {
        if !item.completed {
            return Err(Failure::new(&format!(
                "internal vertex task {}; expected a terminal callback",
                item.ordinal
            )));
        }
        if item.inside {
            inside.push(item.pair);
        }
    }
    Ok(inside)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn range_consumption_and_terminal_callback_are_required() {
        static_assertions::assert_impl_all!(InternalVertexWork<'static>: Send);
        static_assertions::assert_not_impl_any!(InternalVertexWork<'static>: Sync);
        let range = ProgressRange::default();
        let scope = ProgressScope::unnamed(&range, 1., false);
        let mut work = reserve_pairs(vec![(Shape::default(), Shape::default())], &scope, 1e-7);
        assert!(
            collect_inside(reserve_pairs(
                vec![(Shape::default(), Shape::default())],
                &scope,
                1e-7
            ))
            .is_err()
        );
        drop(work[0].range.take());
        let error = work[0]
            .perform(&Handle::default(), &ProgressCancellation::default())
            .unwrap_err();
        assert!(error.message().contains("task 0"));
        assert!(!work[0].completed);
    }
}
