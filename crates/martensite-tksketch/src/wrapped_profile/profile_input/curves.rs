//! Preserve source controls, then orient traversal once for the wrapped operation.
use super::*;
use std::sync::Arc;
mod weighted;
pub(in crate::surface_sketch::wrapped_profile) use weighted::WeightedBezierSource;

pub(super) fn read(
    curve: &ProfileCurve,
    reversed: bool,
    index: &ProfileIndex<'_>,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<AuthoredPiece> {
    let [start, end] = curve
        .ends()
        .ok_or(ProfileInputFailure::Unsupported(curve.id()))?;
    let (mut poles, weighted) = controls(curve, index, cancelled)?;
    endpoints(curve.id(), [start, end], &poles, index)?;
    let [start, end] = if reversed {
        poles.reverse();
        [end, start]
    } else {
        [start, end]
    };
    let weighted = weighted.map(|source| Arc::new(source.orient(reversed)));
    Ok(authored_piece(curve.id(), [start, end], poles, weighted))
}

fn authored_piece(
    id: ProfileCurveId,
    [start, end]: [ProfileVertexId; 2],
    poles: Vec<[f64; 2]>,
    weighted_source: Option<Arc<WeightedBezierSource>>,
) -> AuthoredPiece {
    AuthoredPiece {
        span: CurveSpan {
            curve: SurfaceCurveId(id.0),
            level: 0,
            index: 0,
        },
        start: CornerSource::Authored(SurfaceVertexId(start.0)),
        end: CornerSource::Authored(SurfaceVertexId(end.0)),
        poles,
        weighted_source,
        chart_center: 0.,
    }
}

type Controls = (Vec<[f64; 2]>, Option<WeightedBezierSource>);

fn controls(
    curve: &ProfileCurve,
    index: &ProfileIndex<'_>,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Controls> {
    match curve {
        ProfileCurve::Line { id, start, end } => Ok((
            vec![index.vertex(*id, *start)?, index.vertex(*id, *end)?],
            None,
        )),
        ProfileCurve::Spline {
            id,
            degree,
            poles,
            weights,
            knots,
            ..
        } => {
            spline_shape(*id, *degree, poles.len(), weights.as_deref(), knots.len())?;
            unit_knots(*id, *degree, knots)?;
            weighted::resolve(*id, poles, weights.as_deref(), cancelled)
        }
        other => Err(ProfileInputFailure::Unsupported(other.id())),
    }
}

fn spline_shape(
    curve: ProfileCurveId,
    degree: u32,
    poles: usize,
    weights: Option<&[f64]>,
    knots: usize,
) -> Checked<()> {
    let count = degree.checked_add(1).and_then(|n| usize::try_from(n).ok());
    if !(1..=3).contains(&degree)
        || Some(poles) != count
        || Some(knots) != count.and_then(|n| n.checked_mul(2))
        || weights.is_some_and(|w| Some(w.len()) != count)
    {
        return Err(spline_failure(curve, degree, poles, weights, knots));
    }
    positive_weights(curve, weights.unwrap_or(&[]))
}

fn spline_failure(
    curve: ProfileCurveId,
    degree: u32,
    poles: usize,
    weights: Option<&[f64]>,
    knots: usize,
) -> ProfileInputFailure {
    ProfileInputFailure::Spline {
        curve,
        degree,
        poles,
        weights: weights.map(<[_]>::len),
        knots,
    }
}

fn positive_weights(curve: ProfileCurveId, weights: &[f64]) -> Checked<()> {
    for (index, value) in weights.iter().copied().enumerate() {
        if !value.is_finite() || value <= 0. {
            return Err(ProfileInputFailure::Weight {
                curve,
                index,
                value,
            });
        }
    }
    Ok(())
}

fn unit_knots(curve: ProfileCurveId, degree: u32, knots: &[f64]) -> Checked<()> {
    for (index, value) in knots.iter().copied().enumerate() {
        let expected = if index > degree as usize { 1. } else { 0. };
        if value != expected {
            return Err(ProfileInputFailure::Knot {
                curve,
                index,
                value,
            });
        }
    }
    Ok(())
}

fn endpoints(
    curve: ProfileCurveId,
    vertices: [ProfileVertexId; 2],
    poles: &[[f64; 2]],
    index: &ProfileIndex<'_>,
) -> Checked<()> {
    finite_controls(curve, poles)?;
    for (vertex, point) in vertices.into_iter().zip([poles[0], poles[poles.len() - 1]]) {
        let expected = index.vertex(curve, vertex)?;
        matching_endpoint(curve, vertex, point, expected)?;
    }
    Ok(())
}
fn finite_controls(curve: ProfileCurveId, poles: &[[f64; 2]]) -> Checked<()> {
    for (control, point) in poles.iter().copied().enumerate() {
        if !point.iter().all(|value| value.is_finite()) {
            return Err(ProfileInputFailure::NonFinite {
                curve,
                control,
                point,
            });
        }
    }
    Ok(())
}
fn matching_endpoint(
    curve: ProfileCurveId,
    vertex: ProfileVertexId,
    point: [f64; 2],
    expected: [f64; 2],
) -> Checked<()> {
    if point.map(f64::to_bits) != expected.map(f64::to_bits) {
        return Err(ProfileInputFailure::Endpoint {
            curve,
            vertex,
            point,
            expected,
        });
    }
    Ok(())
}
