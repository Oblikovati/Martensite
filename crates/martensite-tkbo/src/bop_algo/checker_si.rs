//! `BOPAlgoCheckerSI`: checks one shape for self-interference by intersecting its
//! sub-shapes with each other and collecting the interfering pairs.
#[path = "checker_si/completion.rs"]
mod completion;
#[path = "checker_si/observation.rs"]
mod observation;
pub use observation::{
    SelfIntersectionFace, SelfIntersectionFaceStatus, SelfIntersectionObservation,
};
#[cfg(test)]
#[path = "checker_si/completion_tests.rs"]
mod completion_tests;
// The released SDK has no support-certificate join yet; this module cannot complete self-search.
#[cfg(test)]
#[allow(dead_code, unused_imports)]
#[path = "checker_si/face_binding/mod.rs"]
mod face_binding;
#[path = "checker_si/face_self.rs"]
mod face_self;
pub(crate) use completion::{
    FaceTermination, NativeAlertObservation, SelfCheckCompletion, SelfCheckPhase,
};
use completion::{capture_failure, enter_phase};
#[path = "checker_si/post_treat.rs"]
mod post_treat;
#[path = "checker_si/solids.rs"]
mod solids;
use super::alerts::alert;
use super::{AlertIntersectionFailed, AlertMultipleArguments, BOPAlgoPaveFiller};
use crate::bop_ds::{BOPDSIteratorSI, DataStructure};
use crate::int_tools::Context;
use martensite_tkernel::standard::{Failure, Handle};
use std::ops::{Deref, DerefMut};

/// The self-interference checker: a pave filler over one argument whose broad phase pairs
/// the argument's own sub-shapes; the interfering pairs end in `DS().Interferences()`.
///
/// ```
/// use martensite_tkbo::bop_algo::BOPAlgoCheckerSI;
/// let mut checker = BOPAlgoCheckerSI::new();
/// checker.perform();
/// // No argument: the checker needs exactly one.
/// assert!(checker.options().has_errors());
/// ```
pub struct BOPAlgoCheckerSI {
    filler: BOPAlgoPaveFiller,
    level_of_check: i32,
}

impl Default for BOPAlgoCheckerSI {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for BOPAlgoCheckerSI {
    type Target = BOPAlgoPaveFiller;
    fn deref(&self) -> &BOPAlgoPaveFiller {
        &self.filler
    }
}

impl DerefMut for BOPAlgoCheckerSI {
    fn deref_mut(&mut self) -> &mut BOPAlgoPaveFiller {
        &mut self.filler
    }
}

impl BOPAlgoCheckerSI {
    /// A checker of every interference kind, non-destructive and without p-curve
    /// construction (`BOPAlgoCheckerSI()`); `BOPAlgoCheckerSI::new()`.
    pub fn new() -> Self {
        let mut filler = BOPAlgoPaveFiller::new();
        filler.set_non_destructive(true);
        filler.set_avoid_build_pcurve(true);
        Self {
            filler,
            level_of_check: DataStructure::nb_interf_types() - 1,
        }
    }

    /// Limits the check to the interference kinds up to `level`: 0 V/V, 1 +V/E, 2 +E/E,
    /// 3 +V/F, 4 +E/F, 5 +F/F, 6 +V/S, 7 +E/S, 8 +F/S, 9 +S/S (the default); a level out of
    /// that range is ignored (`SetLevelOfCheck`); `checker.set_level_of_check(4)`.
    pub fn set_level_of_check(&mut self, level: i32) {
        if (0..DataStructure::nb_interf_types()).contains(&level) {
            self.level_of_check = level;
        }
    }

    /// The configured level of check; `checker.level_of_check() == 9`.
    pub fn level_of_check(&self) -> i32 {
        self.level_of_check
    }

    /// Intersects the sub-shapes of the single argument, checks the faces on
    /// self-intersection, the shapes against the solids, and records every interfering pair;
    /// any failure is reported as `AlertIntersectionFailed` (`Perform`); `checker.perform()`.
    pub fn perform(&mut self) {
        // Compatibility callers retain no new receipts or failure ownership.
        self.perform_on_progress(&martensite_tkernel::message::ProgressRange::default());
    }

    /// Observe exact execution privately; retain original diagnostics only on opt-in.
    /// Example: `let receipt = checker.perform_observed();`.
    pub(crate) fn perform_observed(&mut self) -> SelfCheckCompletion {
        let mut trace = SelfCheckCompletion::new(self.level_of_check);
        self.perform_recorded(Some(&mut trace));
        trace
    }

    fn perform_recorded(&mut self, mut trace: Option<&mut SelfCheckCompletion>) {
        let arguments = self.filler.arguments().extent();
        let completion = self.perform_on_progress_traced(
            &martensite_tkernel::message::ProgressRange::default(),
            trace.as_deref_mut(),
        );
        if arguments != 1 {
            finish_rejected(trace, arguments, self.filler.options());
            return;
        }
        if let super::ProgressCompletion::Failed {
            cause: Some(cause), ..
        } = completion
        {
            capture_failure(trace.as_deref_mut(), cause);
        }
        finish_returned(trace, self.filler.options());
    }

    /// The data structure, which `perform` must have created.
    fn structure_mut(&mut self) -> Result<&mut DataStructure, Failure> {
        self.filler
            .data_structure_mut()
            .ok_or_else(missing_structure)
    }
}

/// The failure of a stage that runs without the data structure `Init` creates.
fn missing_structure() -> Failure {
    Failure::with_type(
        "Standard_NullObject",
        "BOPAlgoCheckerSI: data structure is None; expected Init to have created it",
        "",
    )
}

/// `BOPAlgoCheckerSI::Init`: unlike the base `Init` it neither validates the arguments nor
/// derives the non-destructive mode, and its iterator pairs sub-shapes of the same argument
/// and keeps only the kinds up to the level of check.
fn initialize_checker(filler: &mut BOPAlgoPaveFiller, level: i32) -> Result<(), Failure> {
    filler.clear();
    filler.initialize_data_structure()?;
    let context = Handle::new(Context::default());
    let mut iterator = BOPDSIteratorSI::with_allocator(filler.options().allocator().cloned());
    let structure = filler.data_structure().ok_or_else(missing_structure)?;
    let options = filler.options();
    iterator.prepare_with_context(
        structure,
        context.get(),
        options.use_obb(),
        options.fuzzy_value(),
    )?;
    iterator.update_by_level_of_check(level);
    filler.install_context_and_iterator(context, iterator.into_iterator());
    Ok(())
}

/// Observe the actual specialized initializer only; `initialize_checker_for_progress_test(filler, 9)?`.
#[cfg(test)]
pub(super) fn initialize_checker_for_progress_test(
    filler: &mut BOPAlgoPaveFiller,
    level: i32,
) -> Result<(), Failure> {
    // The specialized initializer deliberately has no progress argument or base substitution.
    initialize_checker(filler, level)
}

fn finish_rejected(
    trace: Option<&mut SelfCheckCompletion>,
    arguments: i32,
    options: &super::Options,
) {
    if let Some(trace) = trace {
        trace.rejected_arguments = Some(arguments);
        trace.returned = true;
        trace.native_alerts = true;
        trace.alerts = observed_alerts(options);
    }
}

fn observed_alerts(options: &super::Options) -> Vec<NativeAlertObservation> {
    use martensite_tkernel::message::Gravity;
    options
        .report()
        .get_alerts(Gravity::Fail)
        .into_iter()
        .map(|original| NativeAlertObservation { original })
        .collect()
}

fn finish_returned(trace: Option<&mut SelfCheckCompletion>, options: &super::Options) {
    if let Some(trace) = trace {
        trace.returned = true;
        trace.native_alerts = options.has_errors();
        trace.alerts = observed_alerts(options);
    }
}

#[path = "checker_si/progress.rs"]
mod progress;
