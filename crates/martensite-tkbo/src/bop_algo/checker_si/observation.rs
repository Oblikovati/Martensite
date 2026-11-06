//! Immutable scalar observations of one actual check; they authorize no publication.
use super::{BOPAlgoCheckerSI, FaceTermination, SelfCheckCompletion, SelfCheckPhase};
use crate::bop_algo::ProgressCompletion;
use martensite_tkernel::message::{Gravity, ProgressRange};
use martensite_tkmath::geom_abs::SurfaceType;

/// Actual face-worker termination, distinct from inner-search qualification.
/// Example: `SelfIntersectionFaceStatus::NotDone` must not be accepted as clear.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelfIntersectionFaceStatus {
    /// Excluded by the existing analytic-surface policy, rather than intersected.
    PolicyExcluded,
    /// Scheduled work did not reach a worker termination.
    NotAttempted,
    /// The actual intersector returned without completing.
    NotDone,
    /// The actual worker terminated; this does not certify exhaustive inner search.
    Terminated {
        /// Whether actual intersection curves or points were reported.
        has_contacts: bool,
    },
}

/// An invocation-local source face and its actual worker observation.
/// Example: `receipt.faces()[0].source_index()` addresses that invocation's DS.
#[derive(Clone, Copy, Debug)]
pub struct SelfIntersectionFace {
    source_index: i32,
    surface: SurfaceType,
    status: SelfIntersectionFaceStatus,
}

impl SelfIntersectionFace {
    /// Return the original DS index; e.g. `face.source_index()`.
    pub fn source_index(&self) -> i32 {
        // This key is meaningful only with this invocation's source bindings.
        self.source_index
    }
    /// Return the observed support type; e.g. `face.surface()`.
    pub fn surface(&self) -> SurfaceType {
        // Policy exclusion must remain distinguishable from actual intersection work.
        self.surface
    }
    /// Return the actual termination; e.g. `face.status()`.
    pub fn status(&self) -> SelfIntersectionFaceStatus {
        // Worker termination does not certify exhaustive inner coverage.
        self.status
    }
}

/// Owned, immutable diagnostics without geometry handles or a borrowed progress range.
/// Example: `checker.perform_with_progress_observed(&range).completion()` retains refusal.
pub struct SelfIntersectionObservation {
    completion: ProgressCompletion,
    level: i32,
    phase: &'static str,
    active_source: Option<i32>,
    scheduled_faces: bool,
    rejected_arguments: Option<i32>,
    faces: Vec<SelfIntersectionFace>,
    pairs: Vec<(i32, i32)>,
    diagnostics: Vec<(Gravity, String)>,
}

impl SelfIntersectionObservation {
    /// Read execution status, not validity; e.g. `receipt.completion()`.
    pub fn completion(&self) -> &ProgressCompletion {
        // Raw execution status alone cannot authorize geometry publication.
        &self.completion
    }
    /// Consume the receipt while retaining its original typed cause; e.g. `receipt.into_completion()`.
    pub fn into_completion(self) -> ProgressCompletion {
        // Moving retains the original cause without cloning or erasing it.
        self.completion
    }
    /// Read the requested native level; e.g. `receipt.level()`.
    pub fn level(&self) -> i32 {
        // A phase omitted by this level must not be reported as executed.
        self.level
    }
    /// Read the last entered stage; e.g. `receipt.phase()`.
    pub fn phase(&self) -> &'static str {
        // An early exit must not be reported as completed post-treatment.
        self.phase
    }
    /// Read a source index only if execution stopped there; e.g. `receipt.active_source()`.
    pub fn active_source(&self) -> Option<i32> {
        // Unknown worksite provenance remains None.
        self.active_source
    }
    /// Whether the face stage was scheduled; e.g. `receipt.scheduled_faces()`.
    pub fn scheduled_faces(&self) -> bool {
        // Empty work differs from a phase that never ran.
        self.scheduled_faces
    }
    /// Read the rejected count without treating it as success; e.g. `receipt.rejected_arguments()`.
    pub fn rejected_arguments(&self) -> Option<i32> {
        // Wrong arity never implies a successful empty check.
        self.rejected_arguments
    }
    /// Read source-indexed face observations; e.g. `receipt.faces()`.
    pub fn faces(&self) -> &[SelfIntersectionFace] {
        // Receiver reuse cannot change these invocation-owned scalar receipts.
        &self.faces
    }
    /// Read actual interference pairs; e.g. `receipt.pairs()`.
    pub fn pairs(&self) -> &[(i32, i32)] {
        // Native DS indices are diagnostics, not application naming identities.
        &self.pairs
    }
    /// Read alert gravity and dynamic type names; e.g. `receipt.diagnostics()`.
    pub fn diagnostics(&self) -> &[(Gravity, String)] {
        // Owned type names expose no mutable alert payloads.
        &self.diagnostics
    }
}

impl BOPAlgoCheckerSI {
    /// Run once with the caller's range, retaining raw status and scalar coverage.
    /// Example: `let receipt = checker.perform_with_progress_observed(&range);`.
    pub fn perform_with_progress_observed(
        &mut self,
        range: &ProgressRange<'_>,
    ) -> SelfIntersectionObservation {
        let arguments = self.arguments().extent();
        let mut trace = SelfCheckCompletion::new(self.level_of_check);
        let completion = self.perform_on_progress_traced(range, Some(&mut trace));
        if arguments != 1 {
            super::finish_rejected(Some(&mut trace), arguments, self.options());
        } else {
            super::finish_returned(Some(&mut trace), self.options());
        }
        snapshot_observation(self, trace, completion)
    }
}

fn snapshot_observation(
    checker: &BOPAlgoCheckerSI,
    trace: SelfCheckCompletion,
    completion: ProgressCompletion,
) -> SelfIntersectionObservation {
    SelfIntersectionObservation {
        completion,
        level: trace.level,
        phase: phase_name(trace.phase),
        active_source: trace.active_source,
        scheduled_faces: trace.face_phase_scheduled,
        rejected_arguments: trace.rejected_arguments,
        faces: trace.faces.into_iter().map(snapshot_face).collect(),
        pairs: if trace.rejected_arguments.is_some() {
            Vec::new()
        } else {
            snapshot_pairs(checker)
        },
        diagnostics: snapshot_diagnostics(checker),
    }
}

fn snapshot_pairs(checker: &BOPAlgoCheckerSI) -> Vec<(i32, i32)> {
    checker.data_structure().map_or_else(Vec::new, |structure| {
        structure
            .interferences()
            .iter()
            .map(|pair| pair.indices())
            .collect()
    })
}

fn snapshot_diagnostics(checker: &BOPAlgoCheckerSI) -> Vec<(Gravity, String)> {
    [Gravity::Warning, Gravity::Fail]
        .into_iter()
        .flat_map(|gravity| {
            checker
                .options()
                .report()
                .get_alerts(gravity)
                .into_iter()
                .map(move |alert| (gravity, alert.dynamic_type().name().to_owned()))
        })
        .collect()
}

fn snapshot_face(face: super::completion::FaceCompletion) -> SelfIntersectionFace {
    let status = match face.termination {
        FaceTermination::PolicyExcluded => SelfIntersectionFaceStatus::PolicyExcluded,
        FaceTermination::NotAttempted => SelfIntersectionFaceStatus::NotAttempted,
        FaceTermination::NotDone => SelfIntersectionFaceStatus::NotDone,
        FaceTermination::Terminated { has_contacts } => {
            SelfIntersectionFaceStatus::Terminated { has_contacts }
        }
    };
    SelfIntersectionFace {
        source_index: face.source_index,
        surface: face.kind,
        status,
    }
}

fn phase_name(phase: SelfCheckPhase) -> &'static str {
    match phase {
        SelfCheckPhase::Initialization => "Initialization",
        SelfCheckPhase::FacePolicy => "FacePolicy",
        SelfCheckPhase::FaceSelf => "FaceSelf",
        SelfCheckPhase::SolidPairs => "SolidPairs",
        SelfCheckPhase::PostTreat => "PostTreat",
    }
}

#[cfg(test)]
#[path = "observation_tests.rs"]
mod tests;
