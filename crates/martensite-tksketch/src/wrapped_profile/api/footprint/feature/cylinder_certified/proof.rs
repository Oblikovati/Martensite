//! Publish exact self-intersection coverage only after closing the actual decoded contact partition.
use super::{
    assembly::{self, Candidate},
    material::{self, Admitted, Tool},
    *,
};
use martensite_tkbo::body_check::{
    BodyGeometryValidation, CheckMode, StageReport, StageVerdict, ValidationStage,
    begin_body_validation,
};
use std::collections::BTreeSet;

mod contact;
mod sides;
mod stock;
mod trims;
pub(super) use trims::Trims;

const VALIDATION: CylinderFeatureStage = CylinderFeatureStage::Validation;

fn entity(
    attempt: Attempt,
    native: &DetachedCylinderFeature,
    id: CylinderBodyRef,
) -> Checked<&Shape> {
    native.entities.get(&id).ok_or_else(|| {
        missing(
            attempt,
            VALIDATION,
            "retained decoded native body reference",
        )
    })
}

pub(super) fn validate(
    attempt: Attempt,
    input: &Admitted,
    tool: &Tool,
    candidate: &Candidate,
    cancelled: &mut CancelPoll<'_>,
    complete: &mut impl FnMut(Phase),
) -> Checked<(ValidationReport, [f64; 2])> {
    attempt.poll(cancelled, VALIDATION)?;
    let ready = begin_body_validation(&candidate.native.body, &input.resolution)
        .map_err(|cause| attempt.fail(VALIDATION, CylinderFeatureCause::Resolution(cause)))?;
    let ordered = match ready {
        BodyGeometryValidation::Ready(ordered) => ordered,
        BodyGeometryValidation::Stopped(report) => return Err(rejected(attempt, report)),
    };
    complete(Phase::OrderedGeometry);
    let checked = Cover::verify(attempt, input, tool, candidate, cancelled)?;
    complete(Phase::ExactContact);
    attempt.poll(cancelled, VALIDATION)?;
    let mut report = ordered.report();
    report.stages[ValidationStage::SelfIntersection as usize] = checked.report();
    report.stages[ValidationStage::Tolerance as usize] = ordered.check_late_tolerance();
    complete(Phase::LateTolerance);
    if report.is_valid() {
        Ok((report, checked.residuals))
    } else {
        Err(rejected(attempt, report))
    }
}

fn rejected(attempt: Attempt, report: ValidationReport) -> CylinderFeatureFailure {
    attempt.fail(
        VALIDATION,
        CylinderFeatureCause::Validation(Box::new(report)),
    )
}

/// Complete pair closure on the single candidate. Native tolerances and occurrences have
/// already been admitted by the ordered report and remain attached to this candidate.
struct Cover {
    residuals: [f64; 2],
}

impl Cover {
    fn verify(
        attempt: Attempt,
        input: &Admitted,
        tool: &Tool,
        candidate: &Candidate,
        cancelled: &mut CancelPoll<'_>,
    ) -> Checked<Self> {
        verify_face_owners(attempt, tool, candidate)?;
        let index = body::Index::new(attempt, &candidate.native.body, VALIDATION)?;
        let traces = candidate
            .original
            .rebind(attempt, input, candidate, &index, cancelled)?;
        let contact = sides::validate(attempt, input, tool, candidate, &index, &traces, cancelled)?;
        stock::close(attempt, input, tool, candidate, &traces, contact)?;
        let residuals = std::array::from_fn(|depth| {
            traces[depth]
                .values()
                .map(|trace| trace.proof.spatial_residual_bound())
                .fold(0_f64, f64::max)
        });
        Ok(Self { residuals })
    }

    fn report(&self) -> StageReport {
        StageReport {
            stage: ValidationStage::SelfIntersection,
            verdict: StageVerdict::Passed,
            mode: CheckMode::Exact,
            findings: Vec::new(),
        }
    }
}

fn verify_face_owners(attempt: Attempt, tool: &Tool, candidate: &Candidate) -> Checked<()> {
    let mut walls = BTreeSet::new();
    let mut far = BTreeSet::new();
    let mut inherited_ends = BTreeSet::new();
    let mut stock_uses = 0;
    for (&id, parents) in &candidate.history.images {
        if id.kind != ShapeEnum::Face {
            continue;
        }
        let [parent] = parents.as_slice() else {
            return Err(missing(
                attempt,
                VALIDATION,
                "one exact owner of each final candidate face",
            ));
        };
        match parent.source {
            CylinderFeatureSource::SupportFace(key) if key == attempt.attachment => stock_uses += 1,
            CylinderFeatureSource::SupportMember(source) if source.kind == ShapeEnum::Face => {
                require(
                    attempt,
                    VALIDATION,
                    inherited_ends.insert(source),
                    "each inherited disk has one decoded image",
                )?;
            }
            CylinderFeatureSource::Tool(key) if key.role == HistoryRole::SideFace => {
                require(
                    attempt,
                    VALIDATION,
                    walls.insert(key),
                    "one actual face per authored side",
                )?;
            }
            CylinderFeatureSource::Tool(WrappedHistoryKey {
                source: WrappedEntitySource::Region(region),
                role: HistoryRole::LastDepthFace,
            }) => {
                require(
                    attempt,
                    VALIDATION,
                    far.insert(region),
                    "one actual cap per authored region",
                )?;
            }
            _ => {
                return Err(missing(
                    attempt,
                    VALIDATION,
                    "all final faces belong to the exact stock, side, or exposed-cap cover",
                ));
            }
        }
    }
    require(
        attempt,
        VALIDATION,
        stock_uses > 0
            && inherited_ends.len() == 2
            && walls.iter().eq(tool.spans.keys())
            && far.iter().eq(tool.regions.iter()),
        "complete stock, side, and exposed cap face census",
    )
}
