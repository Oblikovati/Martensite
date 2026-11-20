//! Check the selected native band and each whole authoring and retained cap envelope.
use super::*;
use crate::surface_sketch::{
    ConeSupport, NormalSense,
    attachment::{ConeAttachmentFailure, ConeAuthoringChart},
};
use martensite_tkmath::predicates::roots::metric_bounds::{
    ConicalDevelopmentFailure, ConicalFootprint, ConicalSectorWedge, ConicalTrimBand,
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
}

/// Source identity and authored and fitted coverage in one selected native trim band.
#[derive(Clone, Debug, PartialEq)]
pub struct ConeFootprintReport {
    /// Logical selected-face identity.
    pub attachment: FaceAttachmentId,
    /// Frozen support revision.
    pub revision: SupportRevision,
    /// Native meridian interval from the rectangular trim.
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
}

impl ConeAuthoringChart {
    /// Extrude a solved profile and check its entire footprint against this chart's native
    /// rectangular trim band. The normal chart evidence and placed geometry bounds are separate.
    /// Example: `let tool = chart.extrude_within_trim_band(revision, &request, selection)?;`.
    pub fn extrude_within_trim_band(
        &self,
        revision: SupportRevision,
        request: &WrappedConeNormalRequest,
        selection: WrappedConeExecutionSelection,
    ) -> ConeChecked<ConeTrimmedTool> {
        self.extrude_within_trim_band_with_cancel(revision, request, selection, || false)
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
        let (extrusion, authored) =
            placement::cone_on_chart(request, selection, &mut cancelled, |sources, cancelled| {
                authored_envelopes(&plan.metric, &mut plan.exclusions, sources, cancelled)
            })?;
        let footprint = plan.finish(&extrusion, &authored, &mut cancelled)?;
        Ok(ConeTrimmedTool {
            extrusion,
            footprint,
            inner_trims,
        })
    }
}

struct TrimPlan {
    id: FaceAttachmentId,
    revision: SupportRevision,
    native: band::Band,
    metric: ConicalTrimBand,
    exclusions: holes::Exclusions,
}

impl TrimPlan {
    fn capture(
        chart: &ConeAuthoringChart,
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
        let native = band::capture(face)?;
        let mut metric = chart
            .development()
            .trim_band(native.axial, native.angular)
            .map_err(|cause| failure::metric(None, None, cause))?;
        let exclusions = capture_exclusions(chart, &native.holes)?;
        if exclusions.present() {
            metric = metric.with_angular_enclosures();
        }
        Ok(Self {
            id,
            revision,
            native,
            metric,
            exclusions,
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
            limits: self.metric.limits(),
            regions,
        })
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
            let authored =
                bounds.check(&self.metric, &mut self.exclusions, region, prepared.wedge)?;
            let caps = checked_caps(
                &self.metric,
                &mut self.exclusions,
                region,
                geometry,
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
    metric: &ConicalTrimBand,
    exclusions: &mut holes::Exclusions,
    region: SurfaceRegionId,
    geometry: &operation::cone::Geometry,
    normal: NormalSense,
    authored: ConicalFootprint,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<ConeRegionFootprint> {
    let mut caps = cap_envelopes(geometry, cancelled)?;
    if normal == NormalSense::Inward {
        caps.reverse();
    }
    let [first, last] = caps;
    Ok(ConeRegionFootprint {
        authored,
        caps: [
            first.check(metric, exclusions, region, 0)?,
            last.check(metric, exclusions, region, 1)?,
        ],
        fitted_spans: geometry.spans.len(),
    })
}

struct CapEnvelope {
    bounds: Envelope,
    carrier: martensite_tkmath::predicates::roots::metric_bounds::ConicalTrimCarrier,
}
impl CapEnvelope {
    fn check(
        self,
        metric: &ConicalTrimBand,
        exclusions: &mut holes::Exclusions,
        region: SurfaceRegionId,
        row: usize,
    ) -> ConeChecked<ConicalFootprint> {
        let footprint = metric
            .fitted(self.carrier, self.bounds.0)
            .map_err(|cause| failure::metric(Some(region), Some(row), cause))?;
        check_exclusions(exclusions, footprint, region, Some(row))?;
        Ok(footprint)
    }
}

fn cap_envelopes(
    geometry: &operation::cone::Geometry,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<[CapEnvelope; 2]> {
    let mut caps = geometry.material.carriers().map(|carrier| CapEnvelope {
        bounds: Envelope::empty(),
        carrier,
    });
    for span in &geometry.spans {
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
        metric: &ConicalTrimBand,
        exclusions: &mut holes::Exclusions,
        region: SurfaceRegionId,
        wedge: ConicalSectorWedge,
    ) -> ConeChecked<ConicalFootprint> {
        match self {
            Self::Certified(footprint) => Ok(footprint),
            Self::NeedsWedge(bounds) => {
                let footprint = metric
                    .authored(bounds.0, wedge)
                    .map_err(|cause| failure::metric(Some(region), None, cause))?;
                check_exclusions(exclusions, footprint, region, None)?;
                Ok(footprint)
            }
        }
    }
}

fn authored_envelopes(
    metric: &ConicalTrimBand,
    exclusions: &mut holes::Exclusions,
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
                metric,
                exclusions,
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

fn check_exclusions(
    exclusions: &mut holes::Exclusions,
    footprint: ConicalFootprint,
    region: SurfaceRegionId,
    depth_row: Option<usize>,
) -> ConeChecked<()> {
    if exclusions.present()
        && let Some(wire) = exclusions.overlap(normal_box(footprint)?)?
    {
        return Err(ConeFootprintFailure::InnerTrimOverlap {
            region,
            wire,
            depth_row,
        });
    }
    Ok(())
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
