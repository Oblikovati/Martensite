//! Reuse source capture and fitted cap evidence for a revision-bound normal tool.
use super::*;

/// Normal tool whose authored and fitted cap footprints fit a verified cylindrical band.
/// Example: `tool.extrusion().encoded_brep()`.
pub struct CylinderTrimmedTool {
    footprint: CylinderFootprintReport,
    extrusion: WrappedChartNormalResult,
    fitted_spans: usize,
    inner_trims: usize,
    shaped_outer_edges: usize,
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
    /// Number of bounded support holes excluded by both footprint checks.
    /// Example: `let excluded = tool.excluded_trim_wires();`.
    pub fn excluded_trim_wires(&self) -> usize {
        self.inner_trims
    }
    /// Checked shaped outer edge count. Rectangular bands return zero.
    /// Example: `let edges = tool.shaped_outer_edges();`.
    pub fn shaped_outer_edges(&self) -> usize {
        self.shaped_outer_edges
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
        let (_, mut band) = self.resolve_band(revision, request)?;
        self.build_trimmed_tool(revision, request, &mut band, &mut cancelled)
    }

    pub(super) fn build_trimmed_tool(
        &self,
        revision: SupportRevision,
        request: &WrappedNormalRequest,
        band: &mut band::Band,
        cancelled: &mut CancelPoll<'_>,
    ) -> FootprintChecked<CylinderTrimmedTool> {
        let mut exclusions = holes::Exclusions::new(&band.holes)?;
        let inputs = read_inside(request, band, &mut exclusions, cancelled)?;
        let extrusion = placement::from_canonical_inputs(request, inputs, cancelled)
            .map_err(extrusion_error)?;
        let fitted_spans = fitted::check(&extrusion, band, &mut exclusions, cancelled)?;
        poll(cancelled, WrappedStage::Publication)?;
        Ok(CylinderTrimmedTool {
            footprint: self.footprint_report(revision, request, band),
            extrusion,
            fitted_spans,
            inner_trims: band.holes.len(),
            shaped_outer_edges: band.outer.as_ref().map_or(0, outer::OuterTrim::edge_count),
        })
    }
}

fn read_inside(
    request: &WrappedNormalRequest,
    band: &mut band::Band,
    exclusions: &mut holes::Exclusions,
    cancelled: &mut CancelPoll<'_>,
) -> FootprintChecked<input::Inputs> {
    let intervals = bounds::authored_intervals(request, band)?;
    let inputs = input::read_canonical_chart(request, cancelled).map_err(extrusion_error)?;
    for region in &inputs.requests {
        poll(cancelled, WrappedStage::Input)?;
        bounds::check_region(region.region, &region.loops, intervals)?;
        bounds::check_authored_trim(
            region.region,
            &region.loops,
            request.support,
            exclusions,
            &mut band.outer,
        )?;
    }
    Ok(inputs)
}

pub(super) fn poll(cancelled: &mut CancelPoll<'_>, stage: WrappedStage) -> FootprintChecked<()> {
    input::poll(cancelled, stage).map_err(extrusion_error)
}

fn extrusion_error(cause: WrappedNormalFailure) -> CylinderFootprintFailure {
    CylinderFootprintFailure::Extrusion(Box::new(cause))
}
