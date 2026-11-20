//! Compare selected authored material with an independently checked native trim band (#420).
use super::*;
use crate::surface_sketch::{
    CylinderSupport,
    attachment::{
        AttachmentFailure, CylinderFaceAttachment, DetachedCylinderSupport, FaceAttachmentId,
        SupportRevision,
    },
};
mod band;
mod bounds;
mod cone;
pub use cone::{ConeFootprintFailure, ConeFootprintReport, ConeRegionFootprint, ConeTrimmedTool};
mod extrusion;
pub use extrusion::CylinderTrimmedTool;
mod feature;
pub use feature::*;
mod fitted;
#[cfg(test)]
mod tests;

/// Evidence that all selected authored controls lie strictly inside a supported trim band.
/// Example: `assert_eq!(report.revision, current_revision)`.
#[derive(Clone, Debug, PartialEq)]
pub struct CylinderFootprintReport {
    /// Logical identity of the selected native face.
    pub attachment: FaceAttachmentId,
    /// Support revision used to check the footprint.
    pub revision: SupportRevision,
    /// Exact axial bounds of the selected trim band in native coordinates.
    pub axial: [f64; 2],
    /// Native angular bounds for a partial band. A paired seam reports an entire period.
    pub angular: Option<[f64; 2]>,
    /// Number of selected authored regions checked through the shared profile reader.
    pub regions: usize,
}

/// Typed refusal before any attached feature publishes geometry.
#[derive(Debug)]
#[non_exhaustive]
pub enum CylinderFootprintFailure {
    /// The attachment record, revision or chart could not be resolved.
    Attachment(Box<AttachmentFailure>),
    /// The authored request uses a different analytic support.
    SupportMismatch {
        /// Retained chart supplied by the caller.
        actual: Box<CylinderSupport>,
        /// The selected face's revision-bound chart.
        expected: Box<CylinderSupport>,
    },
    /// No complete rectangular trim band could be proved from the native wires.
    UnsupportedTrim {
        /// Failed structural requirement.
        expected: &'static str,
    },
    /// A checked native topology or geometry query failed.
    Native(Box<Failure>),
    /// The selected source cannot be captured by the wrapped profile reader.
    Source(Box<WrappedNormalFailure>),
    /// Construction or placement of the normal tool failed or was cancelled.
    Extrusion(Box<WrappedNormalFailure>),
    /// No authored material is selected.
    EmptySelection,
    /// The control hull could not be proved strictly inside a physical trim boundary.
    ControlOutside {
        /// Selected region that owns the control.
        region: SurfaceRegionId,
        /// Authored curve that owns the control.
        curve: SurfaceCurveId,
        /// Developed axis, 0 for arc length and 1 for axial height.
        axis: usize,
        /// Supplied control coordinate.
        coordinate: f64,
        /// Certified inward interval, excluding its endpoints.
        required_interval: [f64; 2],
    },
    /// A retained cap control could not be proved strictly inside the physical trim.
    FittedControlOutside {
        /// Selected region that owns the fitted span.
        region: SurfaceRegionId,
        /// Authored curve that produced the cap pcurve.
        curve: SurfaceCurveId,
        /// Denominator exponent of the fitted curve interval in loop traversal direction.
        fit_level: u8,
        /// Lower endpoint numerator in loop traversal direction.
        fit_index: u32,
        /// Chart axis, 0 for radians relative to the seam and 1 for axial height.
        axis: usize,
        /// Retained pcurve control coordinate.
        coordinate: f64,
        /// Inward bounds in cap pcurve coordinates, excluding the endpoints.
        required_interval: [f64; 2],
    },
}

impl std::fmt::Display for CylinderFootprintFailure {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(out, "selected cylinder footprint refused: {self:?}")
    }
}
impl std::error::Error for CylinderFootprintFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Attachment(cause) => Some(cause.as_ref()),
            Self::Native(cause) => Some(cause.as_ref()),
            Self::Source(cause) | Self::Extrusion(cause) => Some(cause.as_ref()),
            _ => None,
        }
    }
}
type FootprintChecked<T> = Result<T, CylinderFootprintFailure>;

impl CylinderFaceAttachment {
    /// Check the selected authored footprint against a proved rectangular cylindrical trim band.
    /// Supports partial bands and full bands with a paired native seam. Other trims refuse.
    /// The report describes the authored footprint in the requested chart.
    /// Example: `attachment.certify_footprint(revision, &request)?`.
    pub fn certify_footprint(
        &self,
        revision: SupportRevision,
        request: &WrappedNormalRequest,
    ) -> FootprintChecked<CylinderFootprintReport> {
        let (_, band) = self.resolve_band(revision, request)?;
        bounds::check(request, &band)?;
        Ok(self.footprint_report(revision, request, &band))
    }

    fn resolve_band(
        &self,
        revision: SupportRevision,
        request: &WrappedNormalRequest,
    ) -> FootprintChecked<(DetachedCylinderSupport, band::Band)> {
        let (detached, support) = self
            .resolve_chart(
                revision,
                request.support.chart_origin,
                request.support.normal,
            )
            .map_err(|cause| CylinderFootprintFailure::Attachment(Box::new(cause)))?;
        check_support(request.support, support)?;
        let band = band::capture(&detached.face)?;
        Ok((detached, band))
    }

    fn footprint_report(
        &self,
        revision: SupportRevision,
        request: &WrappedNormalRequest,
        band: &band::Band,
    ) -> CylinderFootprintReport {
        CylinderFootprintReport {
            attachment: self.record().id,
            revision,
            axial: band.axial,
            angular: band.angular,
            regions: request.selections.len(),
        }
    }
}

fn check_support(actual: CylinderSupport, expected: CylinderSupport) -> FootprintChecked<()> {
    if actual != expected {
        return Err(CylinderFootprintFailure::SupportMismatch {
            actual: Box::new(actual),
            expected: Box::new(expected),
        });
    }
    Ok(())
}

impl From<band::BandFailure> for CylinderFootprintFailure {
    fn from(cause: band::BandFailure) -> Self {
        match cause {
            band::BandFailure::Native(cause) => Self::Native(cause),
            band::BandFailure::Unsupported(expected) => Self::UnsupportedTrim { expected },
        }
    }
}
fn unsupported(expected: &'static str) -> CylinderFootprintFailure {
    CylinderFootprintFailure::UnsupportedTrim { expected }
}
