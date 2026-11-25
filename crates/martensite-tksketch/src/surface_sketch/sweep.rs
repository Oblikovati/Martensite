//! Private values shared by admitted analytic section sweeps.
use super::{failure::*, request::*};

#[derive(Clone, Copy)]
pub(super) enum SweepNames {
    Sector {
        corners: [SurfaceVertexId; 4],
        boundaries: [SurfaceCurveId; 4],
    },
    FullTurn {
        rims: [SurfaceCurveId; 2],
    },
}

pub(super) struct Sweep {
    pub region: SurfaceRegionId,
    pub names: SweepNames,
    pub options: NormalExtrusionOptions,
    pub origin: [f64; 3],
    pub axis: [f64; 3],
    pub angle: f64,
    pub points: [[f64; 3]; 4],
    // Requested depth index at wire vertices 0/3; winding may start at depth 1.
    // History uses this index to preserve requested First/Last roles.
    pub first_depth: usize,
    pub expected_volume: f64,
    pub surface_area: f64,
    pub coordinate_error: f64,
    pub reference_plane: Option<martensite_tkmath::gp::Plane3>,
}
impl Sweep {
    pub(super) fn failure(
        &self,
        stage: SurfaceStage,
        cause: SurfaceFailureCause,
    ) -> SurfaceSketchFailure {
        SurfaceSketchFailure {
            region: self.region,
            stage,
            cause,
        }
    }
}
