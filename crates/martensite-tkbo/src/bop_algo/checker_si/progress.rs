//! The actual self-check receiver keeps its 8/1/1 program and specialized initialization.
use super::*;
use crate::bop_algo::{
    ProgressCancellation, ProgressCompletion, ProgressOperation,
    builder_area::progress::{classify_area_progress, observe_user_break},
    progress_result::retain_intersection_completion,
};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
type RangedSolidCheck =
    fn(&mut BOPAlgoCheckerSI, &ProgressRange<'_>, &ProgressCancellation) -> Result<(), Failure>;
const SOLID_CHECKS: [RangedSolidCheck; 4] = [
    BOPAlgoCheckerSI::perform_vz_on_progress,
    BOPAlgoCheckerSI::perform_ez_on_progress,
    BOPAlgoCheckerSI::perform_fz_on_progress,
    BOPAlgoCheckerSI::perform_zz_on_progress,
];
impl BOPAlgoCheckerSI {
    /// Check this receiver's sub-shapes with its specialized initialization and caller range.
    /// Example: `let status = checker.perform_with_progress(&range);` retains incomplete search coverage.
    pub fn perform_with_progress(&mut self, range: &ProgressRange<'_>) -> ProgressCompletion {
        self.perform_on_progress(range)
    }

    /// Run this receiver and retain actual execution evidence; `checker.perform_on_progress(range)` still requires publication validation.
    pub(crate) fn perform_on_progress(&mut self, range: &ProgressRange<'_>) -> ProgressCompletion {
        self.perform_on_progress_traced(range, None)
    }
    /// Keep optional scalar receipts on the same execution path; `checker.perform_on_progress_traced(range, trace)`.
    pub(in crate::bop_algo::checker_si) fn perform_on_progress_traced(
        &mut self,
        range: &ProgressRange<'_>,
        trace: Option<&mut SelfCheckCompletion>,
    ) -> ProgressCompletion {
        let observed = ProgressCancellation::default();
        let mut intersection = ProgressCompletion::Completed;
        let computed = self.run_checker_program(range, &observed, &mut intersection, trace);
        let (face_completion, result) = match computed {
            Ok(completion) => (completion, Ok(())),
            Err(cause) => (ProgressCompletion::Completed, Err(cause)),
        };
        if result.is_err() {
            self.filler
                .options()
                .add_error(&alert(AlertIntersectionFailed));
        }
        let completion = classify_area_progress(
            ProgressOperation::CheckerSI,
            self.filler.options(),
            result,
            &observed,
        );
        let completion = if matches!(completion, ProgressCompletion::Completed) {
            face_completion
        } else {
            completion
        };
        retain_intersection_completion(intersection, completion)
    }
    fn run_checker_program(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
        intersection: &mut ProgressCompletion,
        mut trace: Option<&mut SelfCheckCompletion>,
    ) -> Result<ProgressCompletion, Failure> {
        if self.filler.arguments().extent() != 1 {
            self.filler
                .options()
                .add_error(&alert(AlertMultipleArguments));
            return Ok(ProgressCompletion::Completed);
        }
        let scope =
            ProgressScope::with_literal(range, "Checking shape on self-intersection", 10., false);
        let level = self.level_of_check;
        *intersection = self.filler.perform_on_progress_with_initializer(
            &scope.next(8.),
            observed,
            |actual, _| initialize_checker(actual, level),
        );
        if observe_user_break(self.filler.options(), &scope, observed) {
            return Ok(ProgressCompletion::Completed);
        }
        enter_phase(trace.as_deref_mut(), SelfCheckPhase::FacePolicy);
        let face_completion =
            self.check_faces_on_progress(&scope.next(1.), observed, trace.as_deref_mut())?;
        enter_phase(trace.as_deref_mut(), SelfCheckPhase::SolidPairs);
        self.run_ranged_solid_checks(&scope.next(1.), observed, trace)?;
        Ok(face_completion)
    }
    fn run_ranged_solid_checks(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
        trace: Option<&mut SelfCheckCompletion>,
    ) -> Result<(), Failure> {
        let scope = ProgressScope::unnamed(range, 4., false);
        for check in SOLID_CHECKS {
            if !self.filler.options().has_errors() {
                check(self, &scope.next(1.), observed)?;
            }
        }
        if self.filler.options().has_errors() {
            return Ok(());
        }
        enter_phase(trace, SelfCheckPhase::PostTreat);
        self.post_treat()
    }
}

#[cfg(test)]
#[path = "progress_census.rs"]
mod census;
#[cfg(test)]
#[path = "progress_fixture.rs"]
mod fixture;
#[cfg(test)]
#[path = "progress_snapshot.rs"]
mod snapshot;

#[cfg(test)]
#[path = "progress_contract.rs"]
mod tests;

#[cfg(test)]
#[path = "progress_owned_reference.rs"]
mod owned_reference;
