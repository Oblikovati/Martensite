//! Private capture policy and raw analytic values for selected cylindrical wall queries.
use super::{FaceCaptureBehavior, FaceInputSnapshot};
use martensite_tkmath::gp::{Axis2, Axis3};

#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum CaptureMode {
    Standard(FaceCaptureBehavior),
    CylinderWall,
    SphereBand,
}

impl CaptureMode {
    pub(super) fn extended(self) -> bool {
        self != Self::Standard(FaceCaptureBehavior::StoredRepresentationsV1)
    }

    pub(super) fn analytic(self) -> bool {
        matches!(self, Self::CylinderWall | Self::SphereBand)
    }
}

impl From<FaceCaptureBehavior> for CaptureMode {
    fn from(value: FaceCaptureBehavior) -> Self {
        Self::Standard(value)
    }
}

pub(crate) struct AnalyticFaceInput {
    pub(crate) common: FaceInputSnapshot,
    pub(crate) carriers: Vec<(usize, AnalyticCell)>,
}

pub(crate) enum AnalyticCell {
    Cylinder(Axis3, f64),
    Sphere(Axis3, f64),
    SpatialCircle(Axis2, f64),
    UnselectedPlanarCircle,
}
