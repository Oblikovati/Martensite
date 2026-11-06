//! Raw execution completion is separate from geometry validity and publication policy.
//! These owned values contain no geometry, borrowed range, mutable builder, or validation proof.
//! A caller must still check diagnostics, required validation coverage and its publication contract.
use martensite_tkernel::standard::Failure;
use std::sync::atomic::{AtomicBool, Ordering};

/// Identifies the actual receiver whose execution is classified.
/// Example: `ProgressOperation::ShellSplitter` identifies that receiver, not its area base.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProgressOperation {
    /// High-level feature removal facade.
    Defeaturing,
    /// Feature-removal algorithm receiver.
    RemoveFeatures,
    /// General Boolean image-building receiver.
    Builder,
    /// Volume-building receiver.
    MakerVolume,
    /// Abstract area-stage protocol; this tag does not provide a concrete base operation.
    BuilderArea,
    /// Face-building receiver.
    BuilderFace,
    /// Solid-building receiver.
    BuilderSolid,
    /// Edge-to-wire splitting receiver.
    WireSplitter,
    /// Face-to-shell splitting receiver.
    ShellSplitter,
    /// Face classification operation.
    ClassifyFaces,
    /// Intersection preparation receiver.
    PaveFiller,
    /// Self-interference checking receiver.
    CheckerSI,
    /// Boolean argument analysis receiver.
    ArgumentAnalyzer,
    /// High-level Boolean argument check facade.
    Check,
    /// High-level General Fuse, Boolean, section and split facades.
    BooleanApi,
}

/// Identifies a known observation or classification boundary, not inferred worker timing.
/// Example: `ProgressStage::Completion` does not claim the original cancellation location.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProgressStage {
    /// Required input inspection.
    Input,
    /// Operation-local preparation.
    Preparation,
    /// Intersection computation.
    Intersections,
    /// Topological image construction.
    Images,
    /// Boundary-loop construction.
    Loops,
    /// Area construction.
    Areas,
    /// Internal-entity placement.
    InternalShapes,
    /// Entity classification.
    Classification,
    /// History assembly.
    History,
    /// Explicit simplification stage.
    Simplification,
    /// Final operation post-treatment.
    PostTreatment,
    /// A known worker-task boundary.
    Worker,
    /// Result classification; the original failure or cancellation location may be unknown.
    Completion,
}

/// Separates an algorithm report from an exception or an unqualified stage.
/// Example: `ProgressFailureClass::AlgorithmReport` refers to the receiver's retained report.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProgressFailureClass {
    /// The receiver recorded an algorithm error; inspect its diagnostics.
    AlgorithmReport,
    /// A typed kernel failure was returned by the actual execution path.
    KernelFailure,
    /// Required execution behavior has not been qualified for this path.
    UnqualifiedStage,
}

/// Records a known worksite without manufacturing geometry identity.
/// Example: `ProgressWorksite { label: "MakeShells", input_ordinal: None, entity_key: None }`.
/// Values are owned and structurally Send + Sync; unknown provenance remains `None`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgressWorksite {
    /// Known stage label supplied by the operation.
    pub label: &'static str,
    /// Original input position, never a rank inferred from returned output order.
    pub input_ordinal: Option<usize>,
    /// Caller-provided entity key; never generated from coordinates or proximity.
    pub entity_key: Option<String>,
}

/// Reports raw execution completion without certifying geometry or authorizing publication.
/// Example: `let status = ProgressCompletion::Completed;` still requires geometry validation.
/// This owned, Send + Sync value contains no topology, history, validation coverage or error bound.
/// Raw receiver state can remain stale, empty, partial or invalid on a non-completed outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProgressCompletion {
    /// Execution finished without an observed cancellation or reported execution failure.
    /// Geometry may be empty and has no validity or publication guarantee from this tag.
    Completed,
    /// Execution observed cancellation; retained raw state is diagnostic only.
    Cancelled {
        /// Actual receiver that classifies this outcome.
        operation: ProgressOperation,
        /// Known observation or classification boundary.
        stage: ProgressStage,
        /// Known caller provenance, or `None` when unavailable.
        context: Option<ProgressWorksite>,
    },
    /// Execution failed; the receiver's raw state is not a successful operation result.
    Failed {
        /// Actual receiver that classifies this outcome.
        operation: ProgressOperation,
        /// Known observation or classification boundary.
        stage: ProgressStage,
        /// Distinguishes a retained report from a returned kernel failure.
        class: ProgressFailureClass,
        /// Original typed failure when known; report-only failures can have no exception.
        cause: Option<Failure>,
        /// Known caller provenance, or `None` when unavailable.
        context: Option<ProgressWorksite>,
    },
    /// A required execution path remains incomplete; no successful output is implied.
    Incomplete {
        /// Actual receiver that classifies this outcome.
        operation: ProgressOperation,
        /// Known unqualified or classification boundary.
        stage: ProgressStage,
        /// Why execution qualification is incomplete.
        class: ProgressFailureClass,
        /// Original typed failure when available; no synthetic exception is fabricated.
        cause: Option<Failure>,
        /// Known caller provenance, or `None` when unavailable.
        context: Option<ProgressWorksite>,
    },
}

/// Remembers an actual cancellation observation when later hooks may be non-sticky.
/// Example: `let observed = ProgressCancellation::default(); observed.record_cancelled();`.
/// The owner attributes its known boundary; no thread race chooses provenance. This
/// stores no range and adds no alert. Normal scope exhaustion is not cancellation.
#[derive(Debug, Default)]
pub(crate) struct ProgressCancellation {
    observed: AtomicBool,
}

impl ProgressCancellation {
    /// Records a confirmed cancellation, without polling or changing native diagnostics.
    /// Example: `observed.record_cancelled();` after a confirmed worker cancellation.
    pub(crate) fn record_cancelled(&self) {
        let observed = true;
        self.observed.store(observed, Ordering::Release);
    }

    /// Preserves a previous observation even after a non-sticky hook returns false.
    /// Example: `assert!(observed.is_cancelled());` after recording cancellation.
    pub(crate) fn is_cancelled(&self) -> bool {
        // Pair with a worker's release store before classifying the operation.
        self.observed.load(Ordering::Acquire)
    }
}

#[cfg(test)]
#[path = "progress_result_contract.rs"]
mod contract;

/// Retain the earlier actual typed cause through later report checks; `retain_intersection_completion(intersection, built)`.
pub(crate) fn retain_intersection_completion(
    intersection: ProgressCompletion,
    built: ProgressCompletion,
) -> ProgressCompletion {
    if matches!(
        intersection,
        ProgressCompletion::Failed { cause: Some(_), .. }
    ) {
        return intersection;
    }
    if matches!(built, ProgressCompletion::Failed { cause: Some(_), .. }) {
        return built;
    }
    if matches!(intersection, ProgressCompletion::Completed) {
        return built;
    }
    intersection
}
