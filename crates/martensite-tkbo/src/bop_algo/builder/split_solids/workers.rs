//! One owned task range per solid builder, with deterministic ordered result collection.
use super::*;
use crate::bop_algo::{ProgressFailureClass, ProgressStage, run_context_tasks};
pub(super) type SplitSolidOutcome = (SolidBuilderTasks, ProgressCompletion);

struct SplitSolidWork<'a> {
    ordinal: usize,
    solid: Shape,
    builder: BuilderSolid,
    range: Option<ProgressRange<'a>>,
    completion: Option<ProgressCompletion>,
}
impl SplitSolidWork<'_> {
    fn perform(&mut self, observed: &ProgressCancellation) -> Result<(), Failure> {
        let range = self.range.take().ok_or_else(|| {
            Failure::new(&format!(
                "split-solid task {}; expected one unused range",
                self.ordinal
            ))
        })?;
        let scope = ProgressScope::unnamed(&range, 1., false);
        if !scope.more() {
            observed.record_cancelled();
            self.completion = Some(worker_cancelled());
            return Ok(());
        }
        let completion = self.builder.perform_with_progress(&scope.next(1.));
        if matches!(completion, ProgressCompletion::Cancelled { .. }) {
            observed.record_cancelled();
        }
        let thrown = match &completion {
            ProgressCompletion::Failed {
                class: ProgressFailureClass::KernelFailure,
                cause: Some(cause),
                ..
            } => Some(cause.clone()),
            _ => None,
        };
        self.completion = Some(completion);
        match thrown {
            Some(cause) => Err(cause),
            None => Ok(()),
        }
    }
}
/// Use actual solid receivers; `execute(builders, parallel, assigning, observed)?` joins before map updates.
pub(super) fn execute(
    builders: Vec<(Shape, BuilderSolid)>,
    parallel: bool,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
) -> Result<SplitSolidOutcome, Failure> {
    let mut work: Vec<_> = builders
        .into_iter()
        .enumerate()
        .map(|(ordinal, (solid, builder))| SplitSolidWork {
            ordinal,
            solid,
            builder,
            range: Some(scope.next(1.)),
            completion: None,
        })
        .collect();
    let pooled = parallel && work.len() != 1;
    run_context_tasks(
        &mut work,
        &(),
        pooled,
        || (),
        |item, _| item.perform(observed),
    )?;
    collect(work)
}
fn collect(work: Vec<SplitSolidWork<'_>>) -> Result<SplitSolidOutcome, Failure> {
    let mut first = ProgressCompletion::Completed;
    let mut builders = Vec::new();
    for task in work {
        let completion = task.completion.ok_or_else(|| {
            Failure::new(&format!(
                "split-solid task {}; expected one terminal callback",
                task.ordinal
            ))
        })?;
        let replace = matches!(first, ProgressCompletion::Completed)
            || (matches!(completion, ProgressCompletion::Failed { .. })
                && !matches!(first, ProgressCompletion::Failed { .. }))
            || (matches!(first, ProgressCompletion::Cancelled { .. })
                && matches!(completion, ProgressCompletion::Incomplete { .. }));
        if replace {
            first = completion;
        }
        builders.push((task.solid, task.builder));
    }
    Ok((builders, first))
}
fn worker_cancelled() -> ProgressCompletion {
    ProgressCompletion::Cancelled {
        operation: ProgressOperation::BuilderSolid,
        stage: ProgressStage::Worker,
        context: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
    fn closed_box_job(parallel: bool) -> (Shape, BuilderSolid) {
        let mut maker = BRepPrimAPIMakeBox::from_sizes(2., 3., 4.).unwrap();
        let solid = maker.shape().unwrap().clone();
        let faces: List<Shape> = Explorer::new(&solid, ShapeEnum::Face, ShapeEnum::Shape)
            .unwrap()
            .collect();
        let mut builder = BuilderSolid::new();
        builder.set_shapes(&faces);
        builder.options_mut().set_run_parallel(parallel);
        (solid, builder)
    }
    #[test]
    fn actual_closed_solid_workers_keep_order_and_input_face_identities() {
        for parallel in [false, true] {
            let jobs: Vec<_> = (0..2).map(|_| closed_box_job(parallel)).collect();
            let sources: Vec<_> = jobs.iter().map(|(source, _)| source.clone()).collect();
            let range = ProgressRange::default();
            let scope = ProgressScope::unnamed(&range, 2., false);
            let (outputs, completion) =
                execute(jobs, parallel, &scope, &ProgressCancellation::default()).unwrap();
            assert_eq!(completion, ProgressCompletion::Completed);
            assert_eq!(outputs.len(), 2);
            for ((source, builder), expected) in outputs.iter().zip(&sources) {
                assert!(source.is_same(expected));
                assert!(!builder.options().has_errors());
                assert_eq!(builder.areas().len(), 1);
                let inputs: Vec<_> = Explorer::new(source, ShapeEnum::Face, ShapeEnum::Shape)
                    .unwrap()
                    .collect();
                let faces: Vec<_> = Explorer::new(
                    builder.areas().iter().next().unwrap(),
                    ShapeEnum::Face,
                    ShapeEnum::Shape,
                )
                .unwrap()
                .collect();
                assert_eq!(faces.len(), 6);
                for face in faces {
                    assert!(inputs.iter().any(|input| input.is_same(&face)));
                }
            }
        }
    }
    #[test]
    fn wrapper_cancellation_is_silent_and_consumes_its_range_once() {
        static_assertions::assert_impl_all!(SplitSolidWork<'static>: Send);
        static_assertions::assert_not_impl_any!(SplitSolidWork<'static>: Sync);
        let (progress, trace) = crate::bop_algo::progress_test_support::pulse_indicator(1);
        let range = progress.start();
        let scope = ProgressScope::unnamed(&range, 1., false);
        let observed = ProgressCancellation::default();
        let (outputs, completion) =
            execute(vec![closed_box_job(false)], false, &scope, &observed).unwrap();
        assert!(matches!(completion, ProgressCompletion::Cancelled { .. }));
        assert!(observed.is_cancelled());
        assert!(!outputs[0].1.options().has_errors());
        assert!(outputs[0].1.areas().is_empty());
        assert_eq!(trace.queries.load(std::sync::atomic::Ordering::SeqCst), 1);
    }
}

#[cfg(test)]
mod refusal_tests {
    use super::*;
    #[test]
    fn consumed_range_and_missing_terminal_result_refuse_the_actual_ordinal() {
        let mut task = SplitSolidWork {
            ordinal: 7,
            solid: Shape::default(),
            builder: BuilderSolid::new(),
            range: None,
            completion: None,
        };
        let failure = task.perform(&ProgressCancellation::default()).unwrap_err();
        assert!(failure.message().contains("task 7"));
        let failure = collect(vec![task]).err().unwrap();
        assert!(failure.message().contains("task 7"));
    }
}

#[cfg(test)]
mod completion_tests {
    use super::*;
    #[test]
    fn ordered_collection_preserves_failure_over_later_incompleteness() {
        let failed = ProgressCompletion::Failed {
            operation: ProgressOperation::BuilderSolid,
            stage: ProgressStage::Worker,
            class: ProgressFailureClass::AlgorithmReport,
            cause: None,
            context: None,
        };
        let incomplete = ProgressCompletion::Incomplete {
            operation: ProgressOperation::BuilderSolid,
            stage: ProgressStage::Worker,
            class: ProgressFailureClass::UnqualifiedStage,
            cause: None,
            context: None,
        };
        let work = vec![failed.clone(), incomplete]
            .into_iter()
            .enumerate()
            .map(|(ordinal, completion)| SplitSolidWork {
                ordinal,
                solid: Shape::default(),
                builder: BuilderSolid::new(),
                range: None,
                completion: Some(completion),
            })
            .collect();
        assert_eq!(collect(work).unwrap().1, failed);
    }
}
