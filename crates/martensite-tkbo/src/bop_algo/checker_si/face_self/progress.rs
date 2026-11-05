//! Owned face inputs and ranges are joined before the interference table is updated.
use super::*;
use crate::bop_algo::{
    Options, ProgressCancellation, builder_area::progress::observe_user_break, run_context_tasks,
};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
struct SelfFaceWork<'p> {
    index: i32,
    face: Face,
    parallel: bool,
    progress: Option<ProgressRange<'p>>,
    termination: crate::bop_algo::checker_si::FaceTermination,
    failure: Option<Failure>,
}
impl BOPAlgoCheckerSI {
    /// Preserve face policy and actual worker ranges; `checker.check_faces_on_progress(range, observed)?`.
    pub(in crate::bop_algo::checker_si) fn check_faces_on_progress(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
        mut trace: Option<&mut SelfCheckCompletion>,
    ) -> Result<crate::bop_algo::ProgressCompletion, Failure> {
        if self.level_of_check < FACE_FACE_LEVEL {
            return Ok(crate::bop_algo::ProgressCompletion::Completed);
        }
        schedule_face_phase(trace.as_deref_mut());
        let parallel = self.filler.options().run_parallel();
        self.structure_mut()?.interference_table_mut().clear(false);
        let outer = ProgressScope::unnamed(range, 1., false);
        let candidates = self_intersection_candidates(self.structure_mut()?, trace.as_deref_mut())?;
        let scope = ProgressScope::with_literal(
            &outer.next(1.),
            "Checking surface on self-intersection",
            candidates.len() as f64,
            false,
        );
        enter_phase(trace.as_deref_mut(), SelfCheckPhase::FaceSelf);
        let unqualified = !candidates.is_empty();
        let outputs =
            run_self_face_work(candidates, parallel, &scope, observed, trace.as_deref_mut())?;
        if observe_user_break(self.filler.options(), &outer, observed) {
            return Ok(face_progress_completion(self, observed, unqualified));
        }
        let structure = self.structure_mut()?;
        for (index, termination) in outputs {
            record_joined_self_face(trace.as_deref_mut(), index, termination);
            if matches!(
                termination,
                super::super::FaceTermination::Terminated { has_contacts: true }
            ) {
                add_self_pair(structure, index);
            }
        }
        Ok(face_progress_completion(self, observed, unqualified))
    }
}
fn face_progress_completion(
    checker: &BOPAlgoCheckerSI,
    observed: &ProgressCancellation,
    unqualified: bool,
) -> crate::bop_algo::ProgressCompletion {
    use crate::bop_algo::{
        ProgressCompletion, ProgressFailureClass, ProgressOperation, ProgressStage,
        ProgressWorksite,
    };
    let actual = crate::bop_algo::builder_area::progress::classify_area_progress(
        ProgressOperation::CheckerSI,
        checker.options(),
        Ok(()),
        observed,
    );
    if !matches!(actual, ProgressCompletion::Completed) || !unqualified {
        return actual;
    }
    ProgressCompletion::Incomplete {
        operation: ProgressOperation::CheckerSI,
        stage: ProgressStage::Worker,
        class: ProgressFailureClass::UnqualifiedStage,
        cause: None,
        context: Some(ProgressWorksite {
            label: "CheckFaceSelfIntersection: inner search coverage unobserved",
            input_ordinal: None,
            entity_key: None,
        }),
    }
}
fn run_self_face_work(
    candidates: Vec<(i32, Face)>,
    parallel: bool,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
    trace: Option<&mut SelfCheckCompletion>,
) -> Result<Vec<(i32, crate::bop_algo::checker_si::FaceTermination)>, Failure> {
    let mut work = prepare_self_face_work(candidates, parallel, scope);
    let pooled = parallel && work.len() != 1;
    let joined = run_context_tasks(
        &mut work,
        &(),
        pooled,
        || (),
        |item, _| item.perform(observed),
    );
    if let Err(joined_failure) = joined {
        return Err(first_self_face_failure(&mut work, trace).unwrap_or(joined_failure));
    }
    Ok(work
        .into_iter()
        .map(|item| (item.index, item.termination))
        .collect())
}
/// The failure of the first failed face in source order, with that face recorded as active, so
/// the report does not depend on which worker failed first; `first_self_face_failure(&mut w, t)`.
fn first_self_face_failure(
    work: &mut [SelfFaceWork<'_>],
    trace: Option<&mut SelfCheckCompletion>,
) -> Option<Failure> {
    let failed = work.iter_mut().find(|item| item.failure.is_some())?;
    enter_face(trace, failed.index);
    failed.failure.take()
}
fn prepare_self_face_work<'p>(
    candidates: Vec<(i32, Face)>,
    parallel: bool,
    scope: &'p ProgressScope<'p>,
) -> Vec<SelfFaceWork<'p>> {
    candidates
        .into_iter()
        .map(|(index, face)| SelfFaceWork {
            index,
            face,
            parallel,
            progress: Some(scope.next(1.)),
            termination: crate::bop_algo::checker_si::FaceTermination::NotAttempted,
            failure: None,
        })
        .collect()
}
impl SelfFaceWork<'_> {
    fn perform(&mut self, observed: &ProgressCancellation) -> Result<(), Failure> {
        let range = self.progress.take().ok_or_else(|| {
            Failure::new(&format!(
                "self-face index={} progress=None; expected one unused owned range",
                self.index
            ))
        })?;
        let scope = ProgressScope::unnamed(&range, 1., false);
        if observe_user_break(&Options::default(), &scope, observed) {
            return Ok(());
        }
        match self_face_termination(&self.face, self.parallel) {
            Ok(termination) => self.termination = termination,
            Err(failure) => {
                self.failure = Some(failure.clone());
                return Err(failure);
            }
        }
        Ok(())
    }
}

fn record_joined_self_face(
    trace: Option<&mut SelfCheckCompletion>,
    index: i32,
    termination: crate::bop_algo::checker_si::FaceTermination,
) {
    use crate::bop_algo::checker_si::FaceTermination;
    match termination {
        FaceTermination::NotDone => record_termination(trace, index, false, false),
        FaceTermination::Terminated { has_contacts } => {
            record_termination(trace, index, true, has_contacts)
        }
        _ => (),
    }
}

#[cfg(test)]
mod failure_report_tests {
    use super::*;

    fn work(index: i32, message: Option<&str>) -> SelfFaceWork<'static> {
        SelfFaceWork {
            index,
            face: Face::default(),
            parallel: true,
            progress: None,
            termination: crate::bop_algo::checker_si::FaceTermination::NotAttempted,
            failure: message.map(Failure::new),
        }
    }

    #[test]
    fn first_failed_face_in_source_order_names_itself_and_its_own_failure() {
        let mut items = [
            work(1, None),
            work(2, Some("second")),
            work(5, Some("fifth")),
        ];
        let mut trace = SelfCheckCompletion::new(9);
        let failure = first_self_face_failure(&mut items, Some(&mut trace)).unwrap();
        assert_eq!(failure.message(), "second");
        assert_eq!(trace.active_source, Some(2));
    }

    #[test]
    fn no_recorded_failure_leaves_the_active_face_alone() {
        let mut items = [work(1, None)];
        let mut trace = SelfCheckCompletion::new(9);
        assert!(first_self_face_failure(&mut items, Some(&mut trace)).is_none());
        assert_eq!(trace.active_source, None);
    }
}
