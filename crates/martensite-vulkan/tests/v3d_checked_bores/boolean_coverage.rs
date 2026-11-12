//! Owned observations survive refusal; no entry grants geometry publication authority.
use super::boolean_outcome::RefusalClass;
use martensite_tkbo::bop_algo::{
    ProgressCompletion, SelfIntersectionFace, SelfIntersectionObservation,
};
use martensite_tkernel::message::Gravity;
use martensite_tktopalgo::brep_gprop::{SurfacePropertyOptions, SurfacePropertyResult};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum CoverageStage {
    Execution,
    Topology,
    Geometry,
    SelfIntersection,
    Tolerance,
    History,
    InputAuthority,
    SurfaceMeasurement,
    VolumeMeasurement,
    FiniteMeasurements,
    NurbsConversion,
    NurbsMap,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum StageState {
    Unattempted,
    Completed,
    Failed,
    Missing,
    Cancelled,
}

#[derive(Clone, Debug)]
pub(super) struct StageObservation {
    pub state: StageState,
    pub observed_items: usize,
    pub detail: Option<String>,
}

#[derive(Clone, Debug)]
pub(super) struct SelfCheckObservation {
    pub level: i32,
    pub phase: &'static str,
    pub active_source: Option<i32>,
    pub scheduled_faces: bool,
    pub rejected_arguments: Option<i32>,
    pub faces: Vec<SelfIntersectionFace>,
    pub pairs: Vec<(i32, i32)>,
    pub diagnostics: Vec<(Gravity, String)>,
    /// This owned typed clone preserves the actual context and failure without geometry.
    pub completion: ProgressCompletion,
    pub fuzzy: f64,
    pub parallel: bool,
}

#[derive(Clone, Debug)]
pub(super) struct OperationCoverage {
    pub mode: &'static str,
    pub analyzer_options: [(bool, bool, bool); 2],
    pub surface_options: SurfacePropertyOptions,
    pub volume_options: (bool, bool, bool),
    pub stages: BTreeMap<CoverageStage, StageObservation>,
    pub self_check: Option<SelfCheckObservation>,
    pub surface: Option<SurfacePropertyResult>,
    pub volume: Option<f64>,
    pub volume_error: &'static str,
}

impl Default for OperationCoverage {
    fn default() -> Self {
        Self {
            mode: "ExistingOrderedChecksV1",
            analyzer_options: [(false, false, false), (true, false, true)],
            surface_options: SurfacePropertyOptions::default(),
            volume_options: (false, false, false),
            stages: unattempted_stages(),
            self_check: None,
            surface: None,
            volume: None,
            volume_error: "Unknown",
        }
    }
}

impl OperationCoverage {
    pub(super) fn completed(&mut self, stage: CoverageStage, observed_items: usize) {
        self.stages.insert(
            stage,
            StageObservation {
                state: StageState::Completed,
                observed_items,
                detail: None,
            },
        );
    }
    pub(super) fn observed(&mut self, stage: CoverageStage) {
        self.stages
            .get_mut(&stage)
            .expect("declared coverage stage")
            .observed_items += 1;
    }
    pub(super) fn refused(&mut self, stage: CoverageStage, class: RefusalClass, detail: String) {
        let state = match class {
            RefusalClass::Incomplete => StageState::Missing,
            RefusalClass::Cancelled => StageState::Cancelled,
            _ => StageState::Failed,
        };
        let record = self
            .stages
            .get_mut(&stage)
            .expect("declared coverage stage");
        record.state = state;
        record.detail = Some(detail);
    }
    pub(super) fn self_receipt(
        &mut self,
        actual: &SelfIntersectionObservation,
        fuzzy: f64,
        parallel: bool,
    ) {
        self.self_check = Some(SelfCheckObservation {
            level: actual.level(),
            phase: actual.phase(),
            active_source: actual.active_source(),
            scheduled_faces: actual.scheduled_faces(),
            rejected_arguments: actual.rejected_arguments(),
            faces: actual.faces().to_vec(),
            pairs: actual.pairs().to_vec(),
            diagnostics: actual.diagnostics().to_vec(),
            completion: actual.completion().clone(),
            fuzzy,
            parallel,
        });
    }
}

fn unattempted_stages() -> BTreeMap<CoverageStage, StageObservation> {
    use CoverageStage::*;
    [
        Execution,
        Topology,
        Geometry,
        SelfIntersection,
        Tolerance,
        History,
        InputAuthority,
        SurfaceMeasurement,
        VolumeMeasurement,
        FiniteMeasurements,
        NurbsConversion,
        NurbsMap,
    ]
    .into_iter()
    .map(|stage| (stage, unattempted_stage()))
    .collect()
}

fn unattempted_stage() -> StageObservation {
    StageObservation {
        state: StageState::Unattempted,
        observed_items: 0,
        detail: None,
    }
}
