// SPDX-License-Identifier: GPL-2.0-only
//! Preserve native knots and clamped endpoint identities around spline lowering.
use super::*;
use martensite_tkbo::profile_extrusion::{NativeBernsteinSpan, exact_bernstein_partition};
use martensite_tkmath::predicates::roots::metric_bounds::{
    NativeSplineSource, lower_native_bspline,
};

pub(super) fn rational<'a>(
    curve: &'a ProfileCurve,
    used: LoopUse,
    vertices: &VertexMap,
    policy: &mut CylindricalProfileOptions,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Curve<'a>> {
    let ProfileCurve::Spline {
        id,
        start,
        end,
        degree,
        poles,
        weights,
        knots,
    } = curve
    else {
        return Err(invalid(
            format!("curve={:?}", curve.id()),
            "a clamped positive-rational Bézier",
        ));
    };
    let endpoints = [point(vertices, *start)?, point(vertices, *end)?];
    verify_endpoints(*id, poles, endpoints)?;
    let source = NativeSplineSource {
        degree: *degree as usize,
        poles,
        weights: weights.as_deref(),
        knots,
    };
    let spans = match exact_partition(source) {
        Some(spans) => spans,
        None => transported_partition(*id, source, endpoints, policy, cancelled)?,
    };
    Ok(Curve {
        spans,
        occurrence: used,
        ends: Some([*start, *end]),
    })
}

fn verify_endpoints(
    id: ProfileCurveId,
    poles: &[[f64; 2]],
    endpoints: [[f64; 2]; 2],
) -> Checked<()> {
    let authored = poles
        .first()
        .zip(poles.last())
        .map(|(a, b)| [a, b].map(|p| p.map(f64::to_bits)));
    if authored == Some(endpoints.map(|p| p.map(f64::to_bits))) {
        return Ok(());
    }
    Err(invalid(
        format!(
            "curve={id:?}, vertices={endpoints:?}, end poles={:?}",
            poles.first().zip(poles.last())
        ),
        "bitwise matching authored endpoints and first and last poles",
    ))
}

fn exact_partition(source: NativeSplineSource<'_>) -> Option<Vec<NativeSpan<'_>>> {
    let knots = source.knots;
    let spans = exact_bernstein_partition(source.degree, source.poles, source.weights, knots)
        .filter(|spans| spans.len() <= 64 && knots[0] == 0. && knots[knots.len() - 1] == 1.)?;
    spans.map(bind_span).collect()
}

fn bind_span(piece: NativeBernsteinSpan<'_>) -> Option<NativeSpan<'_>> {
    let domain = dyadic_cell(piece.interval).filter(|_| piece.matches_homogeneous_roundtrip())?;
    Some(NativeSpan {
        controls: NativeControls::Spline(MetricBezier {
            poles: piece.poles,
            weights: piece.weights,
        }),
        domain,
        map: None,
    })
}

fn transported_partition<'a>(
    id: ProfileCurveId,
    source: NativeSplineSource<'a>,
    endpoints: [[f64; 2]; 2],
    policy: &mut CylindricalProfileOptions,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Vec<NativeSpan<'a>>> {
    if endpoints[0] == endpoints[1] {
        return Err(invalid(
            format!("closed native spline={id:?}"),
            "an open spline with a strict full-curve projection, or an exact closed Bernstein partition",
        ));
    }
    let pieces = lower_native_bspline(source, *policy, cancelled).map_err(|cause| {
        RationalWrappedFailure::NativeSplineTransport {
            curve: id,
            cause: Box::new(cause),
        }
    })?;
    Ok(pieces
        .into_iter()
        .map(|piece| transported_span(piece, policy))
        .collect())
}

fn transported_span(
    piece: CertifiedNativeSplineSpan,
    policy: &mut CylindricalProfileOptions,
) -> NativeSpan<'static> {
    policy.max_operations -= piece.transport.operations;
    let knots = piece.interval.map(|knot| WrappedNativeKnot(knot.to_bits()));
    NativeSpan {
        controls: NativeControls::TransportedSpline(piece),
        domain: unit_interval(),
        map: Some(ParameterMap::Spline(WrappedSplineParameterMap { knots })),
    }
}

fn dyadic_cell([first, last]: [f64; 2]) -> Option<WrappedSourceSpan> {
    if !(0. <= first && first < last && last <= 1.) {
        return None;
    }
    for level in 0..=63 {
        let scale = (1_u64 << level) as f64;
        let [a, b] = [first * scale, last * scale];
        if a.fract() == 0. && b.fract() == 0. && a as u64 + 1 == b as u64 {
            return Some(WrappedSourceSpan {
                level,
                index: a as u64,
                reversed: false,
            });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_cells_require_one_exact_dyadic_interval() {
        let span = dyadic_cell([0.75, 1.]).unwrap();
        assert_eq!((span.level, span.index), (2, 3));
        assert!(dyadic_cell([0., 0.75]).is_none());
        assert!(dyadic_cell([0., f64::NAN]).is_none());
        assert!(dyadic_cell([1., 0.]).is_none());
    }
}
