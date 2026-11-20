//! Reuse source capture and fitted cap evidence for a revision-bound normal tool.
use super::*;

/// Normal tool whose authored and fitted cap footprints fit a verified cylindrical band.
/// Example: `tool.extrusion().encoded_brep()`.
pub struct CylinderTrimmedTool {
    footprint: CylinderFootprintReport,
    extrusion: WrappedChartNormalResult,
    fitted_spans: usize,
}

impl CylinderTrimmedTool {
    /// Authored footprint coverage. Example: `tool.footprint().revision`.
    pub fn footprint(&self) -> &CylinderFootprintReport {
        &self.footprint
    }

    /// Each counted span has two checked cap pcurves. Example: `tool.fitted_spans()`.
    pub fn fitted_spans(&self) -> usize {
        self.fitted_spans
    }

    /// The qualified normal tool. Example: `tool.extrusion().export_native()?`.
    pub fn extrusion(&self) -> &WrappedChartNormalResult {
        &self.extrusion
    }
}

impl CylinderFaceAttachment {
    /// Extrude a normal tool whose authored footprint and stored cap pcurves fit the selected band.
    /// Containment uses the requested support chart. The extrusion retains separate placement bounds.
    /// Example: `let tool = attachment.extrude_within_trim_band(revision, &request)?;`.
    pub fn extrude_within_trim_band(
        &self,
        revision: SupportRevision,
        request: &WrappedNormalRequest,
    ) -> FootprintChecked<CylinderTrimmedTool> {
        self.extrude_within_trim_band_with_cancel(revision, request, || false)
    }

    /// Cancel before returning a trim-checked tool. Example:
    /// `attachment.extrude_within_trim_band_with_cancel(revision, &request, || stop())?`.
    pub fn extrude_within_trim_band_with_cancel(
        &self,
        revision: SupportRevision,
        request: &WrappedNormalRequest,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> FootprintChecked<CylinderTrimmedTool> {
        poll(&mut cancelled, WrappedStage::Admission)?;
        let band = self.resolve_band(revision, request)?;
        let inputs = read_inside(request, &band, &mut cancelled)?;
        let extrusion = placement::from_canonical_inputs(request, inputs, &mut cancelled)
            .map_err(extrusion_error)?;
        let fitted_spans = fitted::check(&extrusion, &band, &mut cancelled)?;
        poll(&mut cancelled, WrappedStage::Publication)?;
        Ok(CylinderTrimmedTool {
            footprint: self.footprint_report(revision, request, &band),
            extrusion,
            fitted_spans,
        })
    }
}

fn read_inside(
    request: &WrappedNormalRequest,
    band: &band::Band,
    cancelled: &mut CancelPoll<'_>,
) -> FootprintChecked<input::Inputs> {
    let intervals = bounds::authored_intervals(request, band)?;
    let inputs = input::read_canonical_chart(request, cancelled).map_err(extrusion_error)?;
    for region in &inputs.requests {
        poll(cancelled, WrappedStage::Input)?;
        bounds::check_region(region.region, &region.loops, intervals)?;
    }
    Ok(inputs)
}

pub(super) fn poll(cancelled: &mut CancelPoll<'_>, stage: WrappedStage) -> FootprintChecked<()> {
    input::poll(cancelled, stage).map_err(extrusion_error)
}

fn extrusion_error(cause: WrappedNormalFailure) -> CylinderFootprintFailure {
    CylinderFootprintFailure::Extrusion(Box::new(cause))
}
