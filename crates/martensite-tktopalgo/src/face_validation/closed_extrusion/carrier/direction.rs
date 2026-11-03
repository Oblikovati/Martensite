//! A floating inverse supplies only a candidate; exact world-vector queries remain mandatory.
use super::*;
use martensite_tkmath::gp::{Matrix3, Xyz};

pub(super) fn select(carrier: &Carrier<'_>, version: u32) -> CheckResult<[f64; 3]> {
    if version != 3 {
        return Ok(carrier.strip.direction);
    }
    candidate(
        carrier.strip.direction,
        carrier.strip.placement,
        carrier.edge.placement,
    )
    .map_err(|cause| refusal(carrier.edge.owner, cause))
}
fn candidate(
    direction: [f64; 3],
    support: MetricEmbedding<3>,
    cap: MetricEmbedding<3>,
) -> Result<[f64; 3], ClosedFaceDirectionCandidateFailure> {
    let inverse = Matrix3::new(cap.linear).inverted().map_err(|error| {
        ClosedFaceDirectionCandidateFailure::InverseUnavailable {
            cause: format!("{error:?}"),
        }
    })?;
    let world =
        Matrix3::new(support.linear).apply(Xyz::new(direction[0], direction[1], direction[2]));
    let world = world.coordinates().map(|value| value * support.scale);
    let local = inverse.apply(Xyz::new(world[0], world[1], world[2]));
    validate(local.coordinates().map(|value| value / cap.scale))
}
fn validate(direction: [f64; 3]) -> Result<[f64; 3], ClosedFaceDirectionCandidateFailure> {
    for (coordinate, value) in direction.iter().enumerate() {
        if !value.is_finite() {
            return Err(ClosedFaceDirectionCandidateFailure::NonFinite {
                coordinate,
                bits: value.to_bits(),
            });
        }
    }
    if direction == [0.; 3] {
        return Err(ClosedFaceDirectionCandidateFailure::ZeroVector);
    }
    Ok(direction)
}
fn refusal(owner: usize, cause: ClosedFaceDirectionCandidateFailure) -> ClosedFaceCompletion {
    ClosedFaceCompletion::Incomplete(ClosedFaceIncomplete::CarrierDirectionCandidate {
        query: ClosedFaceQueryContext::new(owner, ClosedFaceRelation::CarrierDirection),
        cause,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn embedding(linear: [[f64; 3]; 3], scale: f64) -> MetricEmbedding<3> {
        MetricEmbedding {
            linear,
            origin: [0.; 3],
            scale,
        }
    }
    #[test]
    fn reflected_frame_candidate_is_local_and_owned() {
        let identity = embedding([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], 1.);
        let reflected = embedding([[1., 0., 0.], [0., -1., 0.], [0., 0., -1.]], 1.);
        assert_eq!(
            candidate([0., 0., -1.], identity, reflected).unwrap(),
            [0., 0., 1.]
        );
    }
    #[test]
    fn singular_nonfinite_and_zero_candidates_refuse() {
        let identity = embedding([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], 1.);
        assert!(candidate([0., 0., 1.], identity, embedding([[0.; 3]; 3], 1.)).is_err());
        assert!(validate([f64::INFINITY, 0., 1.]).is_err());
        assert!(validate([0.; 3]).is_err());
    }
    #[test]
    fn rounded_inverse_candidate_cannot_replace_exact_world_proof() {
        let identity = embedding([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], 1.);
        let scaled = embedding(identity.linear, 3.);
        let local = candidate([0., 0., 1.], identity, scaled).unwrap();
        assert_eq!(local[2] * 3., 1.);
        let mut cancel = || false;
        let mut budget = Budget {
            policy: ClosedFacePolicy::placed_piecewise_boundary(),
            work: ClosedFaceWork::default(),
            cancel: &mut cancel,
        };
        let query = ClosedFaceQueryContext::new(17, ClosedFaceRelation::CarrierDirection);
        let result = chains::require(
            query,
            chains::vector([0., 0., 1.], identity),
            chains::vector(local, scaled),
            &mut budget,
        );
        assert_source_direction_refusal(result, budget.work.queries);
    }
    fn assert_source_direction_refusal(result: CheckResult<()>, queries: u64) {
        assert_eq!(queries, 1);
        assert!(matches!(
            result,
            Err(ClosedFaceCompletion::Incomplete(
                ClosedFaceIncomplete::BoundaryRelation { owner: 17, .. }
            ))
        ));
    }
}
