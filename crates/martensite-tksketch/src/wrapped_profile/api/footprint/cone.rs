//! Check the selected native band and each whole authoring and retained cap envelope.
use super::*;
use crate::surface_sketch::{
    ConeSupport, NormalSense,
    attachment::{ConeAttachmentFailure, ConeAuthoringChart},
};
use martensite_tkmath::predicates::roots::metric_bounds::{
    ConicalDevelopment, ConicalDevelopmentFailure, ConicalFootprint, ConicalSectorWedge,
    ConicalTrimBand,
};
use std::collections::BTreeMap;
mod failure;
pub use failure::ConeFootprintFailure;
type ConeChecked<T> = Result<T, ConeFootprintFailure>;

/// A placed normal extrusion with its selected cone's parametric trim evidence.
pub struct ConeTrimmedTool {
    extrusion: WrappedPlacedConeResult,
    footprint: ConeFootprintReport,
    inner_trims: usize,
    shaped_outer_edges: usize,
}

/// Source identity and authored and fitted coverage in one selected native trim band.
#[derive(Clone, Debug, PartialEq)]
pub struct ConeFootprintReport {
    /// Logical selected-face identity.
    pub attachment: FaceAttachmentId,
    /// Frozen support revision.
    pub revision: SupportRevision,
    /// Native meridian extent of the captured outer trim.
    pub native_meridian: [f64; 2],
    /// Native angular interval, or a full period with a paired seam.
    pub native_angular: Option<[f64; 2]>,
    /// Certified inward limits in the normal chart.
    pub limits: ConicalFootprint,
    /// Outward coverage by explicit authored region identity.
    pub regions: BTreeMap<SurfaceRegionId, ConeRegionFootprint>,
}

/// Whole-region authored and retained cap enclosures.
#[derive(Clone, Debug, PartialEq)]
pub struct ConeRegionFootprint {
    /// A rectangle that encloses the authored filled region, with slant and angle bounds.
    pub authored: ConicalFootprint,
    /// Retained pcurve envelopes in the caller's first and last depth order.
    pub caps: [ConicalFootprint; 2],
    /// Count of fitted spans. Both cap pcurves of each span contribute to the proof.
    pub fitted_spans: usize,
}

impl ConeTrimmedTool {
    /// Borrow the complete placed solid and its placement evidence.
    /// Example: `tool.extrusion().encoded_brep()`.
    pub fn extrusion(&self) -> &WrappedPlacedConeResult {
        &self.extrusion
    }

    /// Borrow the parametric trim evidence. Example: `tool.footprint().regions.len()`.
    pub fn footprint(&self) -> &ConeFootprintReport {
        &self.footprint
    }

    /// Number of bounded support holes excluded by the authored and fitted proofs.
    /// Example: `let excluded = tool.excluded_trim_wires();`.
    pub fn excluded_trim_wires(&self) -> usize {
        self.inner_trims
    }
    /// Checked shaped outer edge count. Rectangular bands return zero.
    /// Example: `let edges = tool.shaped_outer_edges();`.
    pub fn shaped_outer_edges(&self) -> usize {
        self.shaped_outer_edges
    }
}

impl ConeAuthoringChart {
    /// Extrude a solved profile and check its entire footprint against this chart's native
    /// supported trim. The normal chart evidence and placed geometry bounds are separate.
    /// Example: `let tool = chart.extrude_within_trim_band(revision, &request, selection)?;`.
    pub fn extrude_within_trim_band(
        &self,
        revision: SupportRevision,
        request: &WrappedConeNormalRequest,
        selection: WrappedConeExecutionSelection,
    ) -> ConeChecked<ConeTrimmedTool> {
        self.extrude_within_trim_band_with_cancel(revision, request, selection, || false)
    }

    pub(in crate::surface_sketch::wrapped_profile::api) fn materialize_attachment_tool(
        &self,
        revision: SupportRevision,
        request: &WrappedConeNormalRequest,
        selection: WrappedConeExecutionSelection,
        cancelled: &mut CancelPoll<'_>,
    ) -> ConeChecked<placement::attachment_tool::AttachmentTool> {
        input::poll(cancelled, WrappedStage::Admission)?;
        let mut plan = TrimPlan::capture(self, revision, request.support)?;
        let (tool, authored) = placement::attachment_tool::on_chart(
            request,
            selection,
            cancelled,
            |sources, poll| authored_envelopes(&mut plan.checks, sources, poll),
        )?;
        plan.check_attachment_regions(&tool, &authored, request.support.normal, cancelled)?;
        input::poll(cancelled, WrappedStage::Publication)?;
        Ok(tool)
    }

    /// Check cancellation throughout capture, extrusion and the final footprint walk.
    /// Example: `chart.extrude_within_trim_band_with_cancel(revision, &request, selection, || stop())?`.
    pub fn extrude_within_trim_band_with_cancel(
        &self,
        revision: SupportRevision,
        request: &WrappedConeNormalRequest,
        selection: WrappedConeExecutionSelection,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> ConeChecked<ConeTrimmedTool> {
        input::poll(&mut cancelled, WrappedStage::Admission)?;
        let mut plan = TrimPlan::capture(self, revision, request.support)?;
        let inner_trims = plan.native.holes.len();
        let shaped_outer_edges = plan
            .checks
            .outer
            .as_ref()
            .map_or(0, outer::OuterTrim::edge_count);
        let (extrusion, authored) =
            placement::cone_on_chart(request, selection, &mut cancelled, |sources, cancelled| {
                authored_envelopes(&mut plan.checks, sources, cancelled)
            })?;
        let footprint = plan.finish(&extrusion, &authored, &mut cancelled)?;
        Ok(ConeTrimmedTool {
            extrusion,
            footprint,
            inner_trims,
            shaped_outer_edges,
        })
    }
}

struct TrimPlan<'a> {
    id: FaceAttachmentId,
    revision: SupportRevision,
    native: band::Band,
    checks: TrimChecks<'a>,
}

impl<'a> TrimPlan<'a> {
    fn capture(
        chart: &'a ConeAuthoringChart,
        revision: SupportRevision,
        actual: ConeSupport,
    ) -> ConeChecked<Self> {
        let expected = chart.extrusion_support(revision, actual.normal)?;
        if expected != actual {
            return Err(ConeFootprintFailure::SupportMismatch {
                actual: Box::new(actual),
                expected: Box::new(expected),
            });
        }
        let (face, id, revision) = chart.trim_face();
        let mut native = band::capture(face)?;
        let development = chart.development();
        let mut metric = development
            .trim_band(native.axial, native.angular)
            .map_err(|cause| failure::metric(None, None, cause))?;
        let exclusions = capture_exclusions(chart, &native.holes)?;
        if exclusions.present() {
            metric = metric.with_angular_enclosures();
        }
        let checks = TrimChecks {
            development,
            metric,
            exclusions,
            outer: native.outer.take(),
        };
        Ok(Self {
            id,
            revision,
            native,
            checks,
        })
    }

    fn finish(
        mut self,
        extrusion: &WrappedPlacedConeResult,
        authored: &BTreeMap<SurfaceRegionId, AuthoredEnvelope>,
        cancelled: &mut CancelPoll<'_>,
    ) -> ConeChecked<ConeFootprintReport> {
        let regions = self.check_regions(extrusion, authored, cancelled)?;
        input::poll(cancelled, WrappedStage::Publication)?;
        Ok(ConeFootprintReport {
            attachment: self.id,
            revision: self.revision,
            native_meridian: self.native.axial,
            native_angular: self.native.angular,
            limits: self.checks.metric.limits(),
            regions,
        })
    }

    fn check_attachment_regions(
        &mut self,
        tool: &placement::attachment_tool::AttachmentTool,
        authored: &BTreeMap<SurfaceRegionId, AuthoredEnvelope>,
        normal: NormalSense,
        cancelled: &mut CancelPoll<'_>,
    ) -> ConeChecked<()> {
        for &region in tool.regions() {
            input::poll(cancelled, WrappedStage::Publication)?;
            let (wedge, carriers, spans) = tool
                .fitted(region)
                .ok_or(ConeFootprintFailure::MissingRegion(region))?;
            let bounds = authored
                .get(&region)
                .copied()
                .ok_or(ConeFootprintFailure::MissingRegion(region))?;
            let authored = bounds.check(&mut self.checks, region, wedge)?;
            checked_caps(
                &mut self.checks,
                region,
                carriers,
                spans,
                normal,
                authored,
                cancelled,
            )?;
        }
        Ok(())
    }

    fn check_regions(
        &mut self,
        extrusion: &WrappedPlacedConeResult,
        authored: &BTreeMap<SurfaceRegionId, AuthoredEnvelope>,
        cancelled: &mut CancelPoll<'_>,
    ) -> ConeChecked<BTreeMap<SurfaceRegionId, ConeRegionFootprint>> {
        let family = &extrusion.canonical().family.family;
        let mut regions = BTreeMap::new();
        for prepared in &family.selection.prepared {
            input::poll(cancelled, WrappedStage::Publication)?;
            let region = prepared.source.region;
            let bounds = *authored
                .get(&region)
                .ok_or(ConeFootprintFailure::MissingRegion(region))?;
            let geometry = &family
                .members
                .get(&region)
                .ok_or(ConeFootprintFailure::MissingRegion(region))?
                .geometry;
            let authored = bounds.check(&mut self.checks, region, prepared.wedge)?;
            let caps = checked_caps(
                &mut self.checks,
                region,
                geometry.material.carriers(),
                &geometry.spans,
                extrusion.support().normal,
                authored,
                cancelled,
            )?;
            regions.insert(region, caps);
        }
        Ok(regions)
    }
}

fn checked_caps(
    checks: &mut TrimChecks<'_>,
    region: SurfaceRegionId,
    carriers: [martensite_tkmath::predicates::roots::metric_bounds::ConicalTrimCarrier; 2],
    spans: &[operation::cone::SpanProof],
    normal: NormalSense,
    authored: ConicalFootprint,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<ConeRegionFootprint> {
    let mut caps = cap_envelopes(carriers, spans, cancelled)?;
    if normal == NormalSense::Inward {
        caps.reverse();
    }
    let [first, last] = caps;
    Ok(ConeRegionFootprint {
        authored,
        caps: [
            first.check(checks, region, 0)?,
            last.check(checks, region, 1)?,
        ],
        fitted_spans: spans.len(),
    })
}

struct CapEnvelope {
    bounds: Envelope,
    carrier: martensite_tkmath::predicates::roots::metric_bounds::ConicalTrimCarrier,
}
impl CapEnvelope {
    fn check(
        self,
        checks: &mut TrimChecks<'_>,
        region: SurfaceRegionId,
        row: usize,
    ) -> ConeChecked<ConicalFootprint> {
        let footprint = checks
            .metric
            .fitted(self.carrier, self.bounds.0)
            .map_err(|cause| failure::metric(Some(region), Some(row), cause))?;
        checks.check(footprint, region, Some(row))?;
        Ok(footprint)
    }
}

fn cap_envelopes(
    carriers: [martensite_tkmath::predicates::roots::metric_bounds::ConicalTrimCarrier; 2],
    spans: &[operation::cone::SpanProof],
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<[CapEnvelope; 2]> {
    let mut caps = carriers.map(|carrier| CapEnvelope {
        bounds: Envelope::empty(),
        carrier,
    });
    for span in spans {
        input::poll(cancelled, WrappedStage::Publication)?;
        for (cap, pcurve) in caps.iter_mut().zip(&span.pcurves) {
            cap.bounds.include(pcurve);
        }
    }
    Ok(caps)
}

#[derive(Clone, Copy)]
enum AuthoredEnvelope {
    Certified(ConicalFootprint),
    NeedsWedge(Envelope),
}
impl AuthoredEnvelope {
    fn check(
        self,
        checks: &mut TrimChecks<'_>,
        region: SurfaceRegionId,
        wedge: ConicalSectorWedge,
    ) -> ConeChecked<ConicalFootprint> {
        match self {
            Self::Certified(footprint) => Ok(footprint),
            Self::NeedsWedge(bounds) => {
                let footprint = checks
                    .metric
                    .authored(bounds.0, wedge)
                    .map_err(|cause| failure::metric(Some(region), None, cause))?;
                checks.check(footprint, region, None)?;
                Ok(footprint)
            }
        }
    }
}

fn authored_envelopes(
    checks: &mut TrimChecks<'_>,
    authored: &[DevelopedRegionRequest],
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<BTreeMap<SurfaceRegionId, AuthoredEnvelope>> {
    let mut bounds = BTreeMap::new();
    for original in authored {
        let mut region = Envelope::empty();
        for piece in original.loops.iter().flatten() {
            input::poll(cancelled, WrappedStage::Input)?;
            region.include(&piece.poles);
        }
        // The entire right half-plane uses the same lift under every compatible wedge.
        let mut proof = AuthoredEnvelope::NeedsWedge(region);
        if region.0[0][0] > 0. {
            proof = AuthoredEnvelope::Certified(proof.check(
                checks,
                original.region,
                ConicalSectorWedge::PositiveX,
            )?);
        }
        bounds.insert(original.region, proof);
    }
    Ok(bounds)
}

fn capture_exclusions(
    chart: &ConeAuthoringChart,
    inner: &[holes::TrimBox],
) -> ConeChecked<holes::Exclusions> {
    let mut envelopes = Vec::with_capacity(inner.len());
    for bounds in inner {
        let footprint = chart
            .development()
            .native_trim_envelope(bounds.0)
            .map_err(|cause| failure::metric(None, None, cause))?;
        envelopes.push(normal_box(footprint)?);
    }
    Ok(holes::Exclusions::new(&envelopes)?)
}

fn normal_box(footprint: ConicalFootprint) -> ConeChecked<holes::TrimBox> {
    let angular = footprint
        .angular
        .ok_or(ConeFootprintFailure::UnsupportedTrim {
            expected: "angular enclosure for a bounded inner trim check",
        })?;
    Ok(holes::TrimBox([angular, footprint.slant]))
}

struct TrimChecks<'a> {
    development: &'a ConicalDevelopment,
    metric: ConicalTrimBand,
    exclusions: holes::Exclusions,
    outer: Option<outer::OuterTrim>,
}
impl TrimChecks<'_> {
    fn check(
        &mut self,
        footprint: ConicalFootprint,
        region: SurfaceRegionId,
        depth_row: Option<usize>,
    ) -> ConeChecked<()> {
        if let Some(outer) = &mut self.outer {
            let uv = self
                .development
                .native_footprint_envelope(footprint)
                .map_err(|cause| failure::metric(Some(region), depth_row, cause))?;
            if let Some(cause) = outer.check(holes::TrimBox(uv))? {
                return Err(ConeFootprintFailure::OuterTrim {
                    region,
                    depth_row,
                    cause,
                });
            }
        }
        if self.exclusions.present()
            && let Some(wire) = self.exclusions.overlap(normal_box(footprint)?)?
        {
            return Err(ConeFootprintFailure::InnerTrimOverlap {
                region,
                wire,
                depth_row,
            });
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct Envelope([[f64; 2]; 2]);
impl Envelope {
    const fn empty() -> Self {
        Self([[f64::INFINITY, f64::NEG_INFINITY]; 2])
    }

    fn include(&mut self, poles: &[[f64; 2]]) {
        for &point in poles {
            for ([minimum, maximum], coordinate) in self.0.iter_mut().zip(point) {
                *minimum = minimum.min(coordinate);
                *maximum = maximum.max(coordinate);
            }
        }
    }
}

#[cfg(test)]
mod attachment_tests;
