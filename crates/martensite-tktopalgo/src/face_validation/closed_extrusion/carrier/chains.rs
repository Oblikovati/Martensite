//! Exact affine chains keep original coefficients and the reference parameter separate.
use super::*;
use crate::face_validation::closed_extrusion::placements::identity;
pub(super) fn point(
    point: [f64; 3],
    placement: MetricEmbedding<3>,
) -> MetricCurveChain<'static, 3> {
    MetricCurveChain {
        curve: MetricCurve::Line {
            origin: point,
            direction: [0.; 3],
        },
        embedding: placement,
        placements: &[],
    }
}
pub(super) fn line(
    axis: martensite_tkmath::gp::Axis1,
    placement: MetricEmbedding<3>,
) -> MetricCurveChain<'static, 3> {
    let p = axis.location();
    let d = axis.direction();
    MetricCurveChain {
        curve: MetricCurve::Line {
            origin: [p.x(), p.y(), p.z()],
            direction: [d.x(), d.y(), d.z()],
        },
        embedding: placement,
        placements: &[],
    }
}
pub(super) fn vector(
    direction: [f64; 3],
    mut placement: MetricEmbedding<3>,
) -> MetricCurveChain<'static, 3> {
    placement.origin = [0.; 3];
    MetricCurveChain {
        curve: MetricCurve::Line {
            origin: [0.; 3],
            direction,
        },
        embedding: placement,
        placements: &[],
    }
}
pub(super) fn sweep<'a>(
    pole: [f64; 3],
    direction: [f64; 3],
    map: [f64; 2],
    reference: f64,
    placements: &'a [MetricEmbedding<3>],
) -> MetricCurveChain<'a, 2> {
    MetricCurveChain {
        curve: MetricCurve::Line {
            origin: [map[0], reference],
            direction: [map[1], 0.],
        },
        embedding: MetricEmbedding {
            linear: direction.map(|v| [v, -v]),
            origin: pole,
            scale: 1.,
        },
        placements,
    }
}
pub(super) fn projection(axis: usize) -> MetricEmbedding<3> {
    let mut result = identity();
    result.linear = [[0.; 3]; 3];
    result.linear[0][axis] = 1.;
    result
}
pub(super) fn compare<const A: usize, const B: usize, F: FnMut() -> bool>(
    context: ClosedFaceQueryContext,
    first: MetricCurveChain<'_, A>,
    second: MetricCurveChain<'_, B>,
    budget: &mut Budget<'_, F>,
) -> CheckResult<CurveIdentityDecision> {
    budget.compare(context, first, second)
}
pub(super) fn require<const A: usize, const B: usize, F: FnMut() -> bool>(
    context: ClosedFaceQueryContext,
    first: MetricCurveChain<'_, A>,
    second: MetricCurveChain<'_, B>,
    budget: &mut Budget<'_, F>,
) -> CheckResult<()> {
    let actual = compare(context, first, second, budget)?;
    if actual != CurveIdentityDecision::Identical {
        return Err(ClosedFaceCompletion::Incomplete(
            ClosedFaceIncomplete::BoundaryRelation {
                owner: context.owner,
                relation: context.relation.label(),
                actual,
            },
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn context() -> ClosedFaceQueryContext {
        ClosedFaceQueryContext::new(97, ClosedFaceRelation::CarrierCap)
    }
    fn zero() -> MetricCurveChain<'static, 3> {
        point([0.; 3], identity())
    }
    fn query_budget<F: FnMut() -> bool>(cancel: &mut F) -> Budget<'_, F> {
        Budget {
            policy: ClosedFacePolicy::bounded(),
            work: ClosedFaceWork::default(),
            cancel,
        }
    }
    #[test]
    fn exact_relation_query_limit_keeps_its_source_context() {
        let mut cancelled = || false;
        let mut budget = query_budget(&mut cancelled);
        budget.policy.max_queries = 0;
        let Err(ClosedFaceCompletion::Incomplete(ClosedFaceIncomplete::Limit {
            query,
            actual,
            ..
        })) = compare(context(), zero(), zero(), &mut budget)
        else {
            panic!("expected query limit")
        };
        assert_eq!(query, Some(context()));
        assert_eq!(actual, 1);
    }
    #[test]
    fn numerical_cancellation_keeps_the_entered_exact_relation() {
        let mut polls = 0;
        let mut cancelled = || {
            polls += 1;
            polls > 1
        };
        let mut budget = query_budget(&mut cancelled);
        let Err(ClosedFaceCompletion::Incomplete(ClosedFaceIncomplete::Metric { query, cause })) =
            compare(context(), zero(), zero(), &mut budget)
        else {
            panic!("expected metric cancellation")
        };
        assert_eq!(query, context());
        assert!(matches!(
            cause,
            ResidualError::Roots(
                martensite_tkmath::predicates::roots::RootIsolationError::Cancelled
            )
        ));
        assert_eq!(budget.work.queries, 1);
    }
}
