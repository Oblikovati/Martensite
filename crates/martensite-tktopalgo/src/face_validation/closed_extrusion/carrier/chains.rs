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
    first: MetricCurveChain<'_, A>,
    second: MetricCurveChain<'_, B>,
    budget: &mut Budget<'_, F>,
) -> CheckResult<CurveIdentityDecision> {
    budget.query()?;
    compare_curve_chains(first, second, &mut *budget.cancel)
        .map_err(|cause| ClosedFaceCompletion::Incomplete(ClosedFaceIncomplete::Metric(cause)))
}
pub(super) fn require<const A: usize, const B: usize, F: FnMut() -> bool>(
    owner: usize,
    relation: &'static str,
    first: MetricCurveChain<'_, A>,
    second: MetricCurveChain<'_, B>,
    budget: &mut Budget<'_, F>,
) -> CheckResult<()> {
    let actual = compare(first, second, budget)?;
    if actual != CurveIdentityDecision::Identical {
        return Err(ClosedFaceCompletion::Incomplete(
            ClosedFaceIncomplete::BoundaryRelation {
                owner,
                relation,
                actual,
            },
        ));
    }
    Ok(())
}
