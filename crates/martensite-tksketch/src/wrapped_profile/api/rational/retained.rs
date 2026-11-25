//! Read the checked native result and rebind its occurrence history before publication.
use super::*;

pub(super) enum Retained {
    Cylinder(Box<WrappedNormalResult>),
    Cone(Box<WrappedConeNormalResult>),
}

impl From<WrappedNormalResult> for Retained {
    fn from(value: WrappedNormalResult) -> Self {
        Self::Cylinder(Box::new(value))
    }
}

impl From<WrappedConeNormalResult> for Retained {
    fn from(value: WrappedConeNormalResult) -> Self {
        Self::Cone(Box::new(value))
    }
}

impl Retained {
    pub(super) fn authored_volume(
        &self,
        area: WrappedCertifiedRange,
        region: SurfaceRegionId,
        inverse_error: f64,
    ) -> Checked<WrappedCertifiedRange> {
        match self {
            Self::Cylinder(result) => measure::cylinder(area, result.support(), result.depths()),
            Self::Cone(result) => measure::cone(area, result, region, inverse_error),
        }
    }

    pub(super) fn history(&self) -> &[WrappedHistoryRecord] {
        match self {
            Self::Cylinder(result) => result.history(),
            Self::Cone(result) => result.history(),
        }
    }

    pub(super) fn history_mut(&mut self) -> &mut history_projection::HistoryPublication {
        match self {
            Self::Cylinder(result) => &mut result.history,
            Self::Cone(result) => &mut result.history,
        }
    }

    pub(super) fn report(
        &self,
        region: SurfaceRegionId,
    ) -> Checked<(&ValidationReport, WrappedCertifiedRange)> {
        match self {
            Self::Cylinder(result) => result
                .reports()
                .get(&region)
                .map(|report| (&report.validation, report.measures.boundary_flux)),
            Self::Cone(result) => result
                .reports()
                .get(&region)
                .map(|report| (&report.validation, report.boundary_flux)),
        }
        .ok_or_else(|| invalid(format!("region={region:?}"), "one retained material report"))
    }

    pub(super) fn encoded_brep(&self) -> &[u8] {
        match self {
            Self::Cylinder(result) => result.encoded_brep(),
            Self::Cone(result) => result.encoded_brep(),
        }
    }

    pub(super) fn export_native(&self) -> Result<DetachedWrappedBodies, WrappedNormalFailure> {
        match self {
            Self::Cylinder(result) => result.export_native(),
            Self::Cone(result) => result.export_native(),
        }
    }
}
