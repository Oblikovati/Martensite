//! `BRepAlgoAPIDefeaturing`: the API of the feature removal (`BOPAlgoRemoveFeatures`).
use super::algo::BRepAlgoAPIAlgo;
use crate::bop_algo::{BOPAlgoRemoveFeatures, ProgressCompletion};
use martensite_tkbrep::brep_tools::History;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::{Failure, Handle};
use std::ops::{Deref, DerefMut};

/// Removes faces (features) from a solid and fills their gaps with the extended adjacent
/// faces, keeping the history of the input.
///
/// ```
/// use martensite_tkbo::brep_algo_api::BRepAlgoAPIDefeaturing;
/// let defeaturing = BRepAlgoAPIDefeaturing::new();
/// assert!(defeaturing.has_history() && defeaturing.faces_to_remove().is_empty());
/// ```
pub struct BRepAlgoAPIDefeaturing {
    algo: BRepAlgoAPIAlgo,
    input_shape: Shape,
    faces_to_remove: List<Shape>,
    fill_history: bool,
    feature_removal_tool: BOPAlgoRemoveFeatures,
}

impl Default for BRepAlgoAPIDefeaturing {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for BRepAlgoAPIDefeaturing {
    type Target = BRepAlgoAPIAlgo;
    fn deref(&self) -> &BRepAlgoAPIAlgo {
        &self.algo
    }
}

impl DerefMut for BRepAlgoAPIDefeaturing {
    fn deref_mut(&mut self) -> &mut BRepAlgoAPIAlgo {
        &mut self.algo
    }
}

impl BRepAlgoAPIDefeaturing {
    /// An empty operation filling the history (`BRepAlgoAPIDefeaturing()`).
    /// `BRepAlgoAPIDefeaturing::new()`.
    pub fn new() -> Self {
        Self {
            algo: BRepAlgoAPIAlgo::new(),
            input_shape: Shape::default(),
            faces_to_remove: List::default(),
            fill_history: true,
            feature_removal_tool: BOPAlgoRemoveFeatures::new(),
        }
    }

    /// Sets the shape to remove the features from (`SetShape`); `api.set_shape(&solid)`.
    pub fn set_shape(&mut self, shape: &Shape) {
        self.input_shape = shape.clone();
    }

    /// The shape to remove the features from (`InputShape`); `api.input_shape()`.
    pub fn input_shape(&self) -> &Shape {
        &self.input_shape
    }

    /// Adds a face to remove (`AddFaceToRemove`); `api.add_face_to_remove(&face)`.
    pub fn add_face_to_remove(&mut self, face: &Shape) {
        self.faces_to_remove.append(face.clone());
    }

    /// Adds faces to remove (`AddFacesToRemove`); `api.add_faces_to_remove(&faces)`.
    pub fn add_faces_to_remove(&mut self, faces: &List<Shape>) {
        for face in faces.iter() {
            self.faces_to_remove.append(face.clone());
        }
    }

    /// The faces to remove (`FacesToRemove`); `api.faces_to_remove()`.
    pub fn faces_to_remove(&self) -> &List<Shape> {
        &self.faces_to_remove
    }

    /// Turns history filling on or off (`SetToFillHistory`); `api.set_to_fill_history(false)`.
    pub fn set_to_fill_history(&mut self, fill: bool) {
        self.fill_history = fill;
    }

    /// Whether the history is filled (`HasHistory`); `api.has_history()`.
    pub fn has_history(&self) -> bool {
        self.fill_history
    }

    /// Removes features with the same receiver program and an inactive caller range.
    /// `api.build()` retains the legacy raw Done/report behavior.
    pub fn build(&mut self) {
        let _ = self.build_with_progress(&ProgressRange::default());
    }

    /// Forwards the caller range and returns the actual receiver's owned completion.
    /// Raw Done/shape does not certify publication; `api.build_with_progress(&indicator.start())`.
    pub fn build_with_progress(&mut self, range: &ProgressRange<'_>) -> ProgressCompletion {
        self.prepare_removal_tool();
        let completion = self.feature_removal_tool.perform_with_progress(range);
        self.retain_removal_result();
        completion
    }

    fn prepare_removal_tool(&mut self) {
        self.algo.base.set_not_done();
        self.clear();
        let tool = &mut self.feature_removal_tool;
        tool.options_mut()
            .set_face_grouping_version(self.algo.face_grouping_version());
        tool.set_shape(&self.input_shape);
        tool.add_faces_to_remove(&self.faces_to_remove);
        tool.set_to_fill_history(self.fill_history);
        tool.options_mut()
            .set_run_parallel(self.algo.run_parallel());
    }

    fn retain_removal_result(&mut self) {
        self.algo
            .get_report()
            .merge(self.feature_removal_tool.options().report());
        if self.algo.has_errors() {
            return;
        }
        self.algo.base.set_done();
        self.algo.base.my_shape = self.feature_removal_tool.shape().clone();
    }

    /// Clears the report and the removal tool (`Clear`).
    fn clear(&mut self) {
        self.algo.clear();
        self.feature_removal_tool.clear();
    }

    /// The images of `shape` in the result (`Modified`); `api.modified(&face)?`.
    pub fn modified(&self, shape: &Shape) -> Result<Vec<Shape>, Failure> {
        self.feature_removal_tool.modified(shape)
    }

    /// The shapes generated from `shape` (`Generated`); `api.generated(&edge)?`.
    pub fn generated(&self, shape: &Shape) -> Result<Vec<Shape>, Failure> {
        self.feature_removal_tool.generated(shape)
    }

    /// Whether `shape` has no trace in the result (`IsDeleted`); `api.is_deleted(&face)?`.
    pub fn is_deleted(&self, shape: &Shape) -> Result<bool, Failure> {
        self.feature_removal_tool.is_deleted(shape)
    }

    /// Whether a shape was modified (`HasModified`); `api.has_modified()`.
    pub fn has_modified(&self) -> bool {
        self.feature_removal_tool.has_modified()
    }

    /// Whether a shape generated others (`HasGenerated`); `api.has_generated()`.
    pub fn has_generated(&self) -> bool {
        self.feature_removal_tool.has_generated()
    }

    /// Whether a shape was deleted (`HasDeleted`); `api.has_deleted()`.
    pub fn has_deleted(&self) -> bool {
        self.feature_removal_tool.has_deleted()
    }

    /// The history of the removal while it is filled (`History`); `api.history()`.
    pub fn history(&mut self) -> Option<Handle<History>> {
        self.feature_removal_tool.history()
    }
}

#[cfg(test)]
mod grouping_version_tests {
    use super::BRepAlgoAPIDefeaturing;
    use crate::bop_algo::FaceGroupingVersion;

    #[test]
    fn facade_forwards_explicit_replay_version_after_clear() {
        let mut api = BRepAlgoAPIDefeaturing::new();
        assert_eq!(
            api.face_grouping_version(),
            FaceGroupingVersion::ExactOccurrencesV2
        );
        api.set_face_grouping_version(FaceGroupingVersion::LegacyOccurrencesV1);
        api.prepare_removal_tool();
        assert_eq!(
            api.face_grouping_version(),
            FaceGroupingVersion::LegacyOccurrencesV1
        );
        assert_eq!(
            api.feature_removal_tool.options().face_grouping_version(),
            api.face_grouping_version()
        );
    }
}
