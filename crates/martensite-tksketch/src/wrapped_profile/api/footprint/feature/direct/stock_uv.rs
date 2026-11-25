//! Qualify the stock trim loops of one rooted, decoded direct attachment.
use super::*;
use crate::surface_sketch::WrappedHistoryRecord;
use martensite_tkbo::profile_extrusion::{ProfileCurveId, SketchProfile};
use std::collections::BTreeSet;

mod capture;
mod proof;
mod roots;

use roots::Roots;

const STAGE: CylinderFeatureStage = CylinderFeatureStage::Validation;

#[derive(Debug)]
enum Refusal {
    Feature(CylinderFeatureFailure),
    Cancelled,
    Missing(CylinderBodyRef),
    Role(CylinderBodyRef),
    Authored(WrappedHistoryKey),
    Incidence(CylinderBodyRef),
    Limit(CylinderBodyRef),
    Curve(CylinderBodyRef),
    Proof(proof::Failure),
}

type Checked<T> = Result<T, Refusal>;

impl From<CylinderFeatureFailure> for Refusal {
    fn from(cause: CylinderFeatureFailure) -> Self {
        Self::Feature(cause)
    }
}

impl std::fmt::Display for Refusal {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Feature(cause) => write!(out, "{cause:?}"),
            Self::Cancelled => write!(out, "cancelled stock trim"),
            Self::Missing(id) => write!(out, "missing rooted stock trim entity {id:?}"),
            Self::Role(id) => write!(out, "unbound stock trim face role {id:?}"),
            Self::Authored(key) => write!(out, "missing or repeated original cap edge {key:?}"),
            Self::Incidence(id) => write!(out, "incomplete stock trim incidence {id:?}"),
            Self::Limit(id) => write!(out, "stock trim exceeds source limits for {id:?}"),
            Self::Curve(id) => write!(out, "expected stored finite cubic spans on {id:?}"),
            Self::Proof(cause) => write!(out, "{cause}"),
        }
    }
}

impl Refusal {
    fn feature(self, attempt: Attempt) -> CylinderFeatureFailure {
        match self {
            Self::Feature(cause) => cause,
            Self::Cancelled | Self::Proof(proof::Failure::Cancelled) => {
                attempt.fail(STAGE, CylinderFeatureCause::Cancelled)
            }
            cause => attempt.fail(
                STAGE,
                CylinderFeatureCause::Native {
                    query: "stored stock UV proof",
                    cause: Box::new(Failure::new(&cause.to_string())),
                },
            ),
        }
    }
}

fn poll(cancelled: &mut CancelPoll<'_>) -> Checked<()> {
    if cancelled() {
        return Err(Refusal::Cancelled);
    }
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Image {
    SupportRemainder,
    SupportCounterIsland(SurfaceRegionId),
}

struct LoopSource {
    owner: Image,
    region: SurfaceRegionId,
    curves: Vec<ProfileCurveId>,
}

struct UvInput {
    profile: SketchProfile,
    loops: Vec<LoopSource>,
}

pub(super) struct Input<'a> {
    pub(super) history: &'a CylinderFeatureHistory,
    pub(super) body: &'a DetachedCylinderFeature,
    pub(super) topology: &'a topology::Topology,
    pub(super) band: CylinderBodyRef,
    pub(super) chart: stock::StockChart,
    pub(super) welded: &'a [seams::ReboundSeam],
    pub(super) records: &'a [WrappedHistoryRecord],
    pub(super) resolution: &'a Resolution,
}

pub(super) fn qualify(
    attempt: Attempt,
    input: Input<'_>,
    cancelled: &mut CancelPoll<'_>,
) -> FeatureChecked<usize> {
    issue(attempt, input, cancelled).map_err(|cause| cause.feature(attempt))
}

fn issue(attempt: Attempt, input: Input<'_>, cancelled: &mut CancelPoll<'_>) -> Checked<usize> {
    let roots = Roots::capture(attempt, &input, cancelled)?;
    let uv = capture::read(attempt, &roots, input.body, input.topology, cancelled)?;
    let proof =
        proof::Receipt::issue(&uv, *input.resolution, &mut *cancelled).map_err(Refusal::Proof)?;
    poll(cancelled)?;
    Ok(proof.pair_domains())
}
