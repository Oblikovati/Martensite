//! `BOPAlgoArgumentAnalyzer::Perform`: the native polls and weights around the enabled tests.
use super::{BOPAlgoArgumentAnalyzer, CheckResult};
use crate::bop_algo::builder_area::progress::{classify_area_progress, observe_user_break};
use crate::bop_algo::{ProgressCancellation, ProgressCompletion, ProgressOperation};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::Failure;
use std::cell::RefCell;

/// One test of `Perform` after self-interference, run when its mode is on and no fault stops it.
type AnalysisTest = fn(&mut BOPAlgoArgumentAnalyzer) -> Result<(), Failure>;

/// When the native code polls for a user break around one test.
#[derive(Clone, Copy, Eq, PartialEq)]
enum BreakPoll {
    /// After the test's block, whether the test ran or was skipped by a stop on first fault.
    AfterBlock,
    /// Only when the test ran (`TestTangent`).
    AfterRun,
    /// Never (`TestCurveOnSurface`, the last test).
    Never,
}

/// What the self-interference checkers report beyond the faults they add: a cancellation, or a
/// search they could not finish, which must not read as a completed analysis.
pub(super) struct SelfCheckOutcome<'a> {
    observed: &'a ProgressCancellation,
    incomplete: RefCell<Option<ProgressCompletion>>,
}

impl SelfCheckOutcome<'_> {
    /// Keep what one checker returned; `outcome.record(checker.perform_with_progress(range))`.
    pub(super) fn record(&self, completion: ProgressCompletion) {
        match completion {
            ProgressCompletion::Cancelled { .. } => self.observed.record_cancelled(),
            ProgressCompletion::Incomplete { .. } => {
                self.incomplete.borrow_mut().get_or_insert(completion);
            }
            // A failed checker is the OperationAborted fault the analysis adds, as natively.
            ProgressCompletion::Completed | ProgressCompletion::Failed { .. } => {}
        }
    }
}

impl BOPAlgoArgumentAnalyzer {
    /// Runs the enabled tests in the caller's range (`Perform`): eight of its ten units go to
    /// self-interference, and a failing test appends the unknown fault the native catch appends.
    /// The returned completion is about execution; the faults are in `check_result`.
    /// Example: `let status = analyzer.perform_with_progress(&indicator.start());`.
    pub fn perform_with_progress(&mut self, range: &ProgressRange<'_>) -> ProgressCompletion {
        let scope = ProgressScope::with_literal(range, "Analyze shapes", 10., false);
        let observed = ProgressCancellation::default();
        let outcome = SelfCheckOutcome {
            observed: &observed,
            incomplete: RefCell::new(None),
        };
        self.result.clear();
        let result = self.run_tests(&scope, &outcome);
        if result.is_err() {
            self.result.append(CheckResult::new());
        }
        let completion = classify_area_progress(
            ProgressOperation::ArgumentAnalyzer,
            &self.options,
            result,
            &observed,
        );
        match (&completion, outcome.incomplete.into_inner()) {
            (ProgressCompletion::Completed, Some(incomplete)) => incomplete,
            _ => completion,
        }
    }

    fn run_tests(
        &mut self,
        scope: &ProgressScope<'_>,
        outcome: &SelfCheckOutcome<'_>,
    ) -> Result<(), Failure> {
        if observe_user_break(&self.options, scope, outcome.observed) {
            return Ok(());
        }
        self.prepare()?;
        let modes = self.modes;
        if modes.argument_type {
            self.test_types()?;
            if observe_user_break(&self.options, scope, outcome.observed) {
                return Ok(());
            }
        }
        if modes.self_interference {
            self.test_self_interferences(&scope.next(8.), outcome)?;
            if observe_user_break(&self.options, scope, outcome.observed) {
                return Ok(());
            }
        }
        self.run_later_tests(scope, outcome.observed)
    }

    fn run_later_tests(
        &mut self,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let modes = self.modes;
        let tests: [(bool, AnalysisTest, BreakPoll); 7] = [
            (
                modes.small_edge,
                Self::test_small_edge,
                BreakPoll::AfterBlock,
            ),
            (
                modes.rebuild_face,
                Self::test_rebuild_face,
                BreakPoll::AfterBlock,
            ),
            (modes.tangent, Self::test_tangent, BreakPoll::AfterRun),
            (
                modes.merge_vertex,
                Self::test_merge_vertex,
                BreakPoll::AfterBlock,
            ),
            (
                modes.merge_edge,
                Self::test_merge_edge,
                BreakPoll::AfterBlock,
            ),
            (
                modes.continuity,
                Self::test_continuity,
                BreakPoll::AfterBlock,
            ),
            (
                modes.curve_on_surface,
                Self::test_curve_on_surface,
                BreakPoll::Never,
            ),
        ];
        for (enabled, test, poll) in tests {
            if !enabled {
                continue;
            }
            let runs = self.result.is_empty() || !self.stop_on_first;
            if runs {
                test(self)?;
            }
            let polls = poll == BreakPoll::AfterBlock || (poll == BreakPoll::AfterRun && runs);
            if polls && observe_user_break(&self.options, scope, observed) {
                return Ok(());
            }
        }
        Ok(())
    }
}
