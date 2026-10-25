use super::{SurfaceErrorEvidence, SurfacePropertyOptions};
use martensite_tkgeombase::gprop::GProps;

/// Owned measurement values and their policy/evidence, with no input handles or caches.
///
/// Example: obtain a result with `BRepGProp::surface_properties_with_options`, then
/// read `result.properties().mass()`; moving it between threads shares no operation state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfacePropertyResult {
    pub(super) properties: GProps,
    pub(super) options: SurfacePropertyOptions,
    pub(super) evidence: SurfaceErrorEvidence,
    pub(super) effective_relative_target: Option<f64>,
    pub(super) geometric_faces: usize,
    pub(super) triangulated_faces: usize,
}

impl SurfacePropertyResult {
    /// Read the owned properties; e.g. `result.properties().mass()`.
    pub fn properties(&self) -> &GProps {
        &self.properties
    }

    /// Read the original requested controls; e.g. `result.options().skip_shared`.
    pub fn options(&self) -> SurfacePropertyOptions {
        self.options
    }

    /// Read estimate-or-unknown evidence; e.g. `result.error_evidence()`.
    pub fn error_evidence(&self) -> SurfaceErrorEvidence {
        self.evidence
    }

    /// Read the final effective global target, currently unknown because internal
    /// refinement budgets can relax without being published; e.g. `assert_eq!(result.effective_relative_target(), None)`.
    pub fn effective_relative_target(&self) -> Option<f64> {
        self.effective_relative_target
    }

    /// Whether the completed query contained no faces. Empty mass/inertia are zero;
    /// the centroid convention is the input's located origin, not a physical center.
    /// Example: `assert!(BRepGProp::surface_properties_with_options(&Shape::default(), SurfacePropertyOptions::default())?.is_empty());`.
    pub fn is_empty(&self) -> bool {
        self.geometric_faces == 0 && self.triangulated_faces == 0
    }

    /// Count geometric face contributions; e.g. `result.geometric_faces()`.
    pub fn geometric_faces(&self) -> usize {
        self.geometric_faces
    }

    /// Count triangle-based face contributions; e.g. `result.triangulated_faces()`.
    pub fn triangulated_faces(&self) -> usize {
        self.triangulated_faces
    }
}
