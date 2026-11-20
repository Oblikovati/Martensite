//! Capture one input-sized aggregate admission before the shared proof ledger exists.
use super::*;
use crate::surface_sketch::wrapped_profile::planar::source::SourceSnapshot;
use WrappedConeArithmeticGrantPolicy::{InputBoundV1, OriginalV1, PhysicalAngleBoundV2};
mod angle;

const ORIGINAL_CEILING: u32 = 1_000_000;
const INPUT_HARD_CEILING: u32 = 16_000_000;
const PER_SPAN_LEVEL: u32 = 4096;

pub(in crate::surface_sketch::wrapped_profile) struct CapturedGrant {
    pub receipt: WrappedConeArithmeticGrantReceipt,
    regions: Vec<CapturedRegion>,
    proof: WrappedConeProofOptions,
    proof_metric_bits: [u64; 2],
    semi_angle_bits: Option<u64>,
}
struct CapturedRegion {
    region: SurfaceRegionId,
    source: SourceSnapshot,
    loops: Vec<Vec<CurveSpan>>,
    metric_bits: [u64; 3],
    fit_level: u8,
}

pub(super) fn admission_ceiling(execution: WrappedConeExecutionSelection) -> u32 {
    match grant_policy(execution) {
        OriginalV1 => ORIGINAL_CEILING,
        InputBoundV1 | PhysicalAngleBoundV2 => INPUT_HARD_CEILING,
    }
}

fn grant_policy(execution: WrappedConeExecutionSelection) -> WrappedConeArithmeticGrantPolicy {
    use WrappedConeExecutionSelection::*;
    match execution {
        DirectedFamilyChartRequireCertifiedV2 { arithmetic }
        | DirectedRecenteredChartRequireCertifiedV3 { arithmetic }
        | DirectedRegionAtlasRequireCertifiedV4 { arithmetic } => arithmetic,
        _ => OriginalV1,
    }
}

pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn capture(
    request: &ConeRun<'_>,
    execution: WrappedConeExecutionSelection,
    originals: &[DevelopedRegionRequest],
) -> ConeChecked<Option<CapturedGrant>> {
    let policy = grant_policy(execution);
    if policy == OriginalV1 {
        return Ok(None);
    }
    let receipt = admission(request, policy, originals)?;
    Ok(Some(CapturedGrant {
        receipt,
        regions: originals.iter().map(CapturedRegion::capture).collect(),
        proof: request.proof,
        proof_metric_bits: proof_metric_bits(request.proof),
        semi_angle_bits: semi_angle_bits(request, policy),
    }))
}

fn admission(
    request: &ConeRun<'_>,
    policy: WrappedConeArithmeticGrantPolicy,
    originals: &[DevelopedRegionRequest],
) -> ConeChecked<WrappedConeArithmeticGrantReceipt> {
    let spans = source_count(originals)?;
    let level = request.options.maximum_fit_level;
    let (ceiling, coefficient) = grant_limit(request, policy, originals, spans, level)?;
    let requested = request.proof.arithmetic.max_operations;
    if requested > ceiling {
        return Err(refuse(
            "input-bound aggregate operations",
            requested,
            ceiling,
        ));
    }
    Ok(WrappedConeArithmeticGrantReceipt {
        policy,
        source_spans: spans,
        maximum_fit_level: level,
        operations_per_span_level: coefficient,
        requested_operations: requested,
        input_ceiling: ceiling,
        hard_ceiling: INPUT_HARD_CEILING,
        primitive_ceiling: ORIGINAL_CEILING,
    })
}

fn grant_limit(
    request: &ConeRun<'_>,
    policy: WrappedConeArithmeticGrantPolicy,
    originals: &[DevelopedRegionRequest],
    spans: u32,
    level: u8,
) -> ConeChecked<(u32, u32)> {
    if policy == PhysicalAngleBoundV2 {
        fit_level(level)?;
        let ceiling = angle::ceiling(originals, spans, level, request.support.semi_angle);
        return Ok((ceiling, angle::BASE_PER_SPAN_LEVEL));
    }
    Ok((input_ceiling(spans, level)?, PER_SPAN_LEVEL))
}

fn semi_angle_bits(request: &ConeRun<'_>, policy: WrappedConeArithmeticGrantPolicy) -> Option<u64> {
    (policy == PhysicalAngleBoundV2).then(|| request.support.semi_angle.to_bits())
}

fn source_count(originals: &[DevelopedRegionRequest]) -> ConeChecked<u32> {
    let count = originals
        .iter()
        .flat_map(|r| &r.loops)
        .try_fold(0u32, |sum, pieces| {
            u32::try_from(pieces.len())
                .ok()
                .and_then(|n| sum.checked_add(n))
        });
    match count {
        Some(count @ 1..=4096) => Ok(count),
        count => Err(refuse(
            "input-bound original span occurrences",
            count.unwrap_or(u32::MAX),
            4096,
        )),
    }
}

fn fit_level(level: u8) -> ConeChecked<()> {
    if level > 8 {
        return Err(refuse("input-bound maximum fit level", u32::from(level), 8));
    }
    Ok(())
}
fn input_ceiling(spans: u32, level: u8) -> ConeChecked<u32> {
    fit_level(level)?;
    let ceiling = spans
        .checked_mul(u32::from(level) + 1)
        .and_then(|n| n.checked_mul(PER_SPAN_LEVEL))
        .ok_or_else(|| refuse("input-bound allowance multiplication", spans, 4096))?;
    Ok(ceiling.clamp(ORIGINAL_CEILING, INPUT_HARD_CEILING))
}

impl CapturedGrant {
    pub(in crate::surface_sketch::wrapped_profile) fn validate(
        &self,
        request: &ConeRun<'_>,
        execution: WrappedConeExecutionSelection,
        originals: &[DevelopedRegionRequest],
    ) -> ConeChecked<()> {
        let policy = grant_policy(execution);
        if policy == self.receipt.policy
            && self.proof == request.proof
            && self.proof_metric_bits == proof_metric_bits(request.proof)
            && self.semi_angle_bits == semi_angle_bits(request, policy)
            && self.receipt == admission(request, policy, originals)?
            && self.regions.len() == originals.len()
            && self
                .regions
                .iter()
                .zip(originals)
                .all(|(old, source)| old.matches(source))
        {
            return Ok(());
        }
        Err(binding_failure())
    }
}

impl CapturedRegion {
    fn capture(source: &DevelopedRegionRequest) -> Self {
        Self {
            region: source.region,
            source: SourceSnapshot::capture(source),
            loops: loop_spans(source),
            metric_bits: metric_bits(source),
            fit_level: source.max_fit_level,
        }
    }
    fn matches(&self, source: &DevelopedRegionRequest) -> bool {
        self.region == source.region
            && self.fit_level == source.max_fit_level
            && self.metric_bits == metric_bits(source)
            && self.loops == loop_spans(source)
            && self.source.matches(source)
    }
}

fn loop_spans(source: &DevelopedRegionRequest) -> Vec<Vec<CurveSpan>> {
    source
        .loops
        .iter()
        .map(|pieces| pieces.iter().map(|p| p.span).collect())
        .collect()
}
fn metric_bits(source: &DevelopedRegionRequest) -> [u64; 3] {
    [
        source.depths[0].to_bits(),
        source.depths[1].to_bits(),
        source.tolerance.to_bits(),
    ]
}
fn proof_metric_bits(proof: WrappedConeProofOptions) -> [u64; 2] {
    [
        proof.authored_volume.enclosure_width.to_bits(),
        proof.integration.absolute_error_limit.to_bits(),
    ]
}
fn refuse(field: &'static str, actual: u32, limit: u32) -> ConeFailure {
    ConeFailure::new(
        None,
        None,
        WrappedFailureClass::InvalidInput,
        ConeCause::Limit {
            field,
            actual,
            limit,
        },
    )
    .at(WrappedStage::Admission)
}
fn binding_failure() -> ConeFailure {
    ConeFailure::new(
        None,
        None,
        WrappedFailureClass::Incomplete,
        ConeCause::Binding {
            field: "captured input-bound arithmetic grant",
            actual: vec![0],
            expected: vec![1],
        },
    )
    .at(WrappedStage::Publication)
}

#[cfg(test)]
mod tests;
