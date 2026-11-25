//! Bind source-local reference spans to the authored normalized spline parameter.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CylindricalProfileInterval, CylindricalProfileOptions, RationalReferenceBounds,
    RationalReferenceProof, RationalReferenceRequest, fit_rational_bezier_reference_with_proof,
};

pub(super) struct Cover {
    pub curve: ProfileCurveId,
    pub spans: Vec<FittedSpan>,
}

pub(super) struct FittedSpan {
    pub span: RationalReferenceSpan,
    pub map: Option<input::ParameterMap>,
}

pub(super) fn all_spans(covers: &[Cover]) -> impl Iterator<Item = &RationalReferenceSpan> + Clone {
    covers
        .iter()
        .flat_map(|cover| cover.spans.iter().map(|fitted| &fitted.span))
}

pub(super) fn cover(
    level: u8,
    source: &input::Source<'_>,
    limit: f64,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Vec<Cover>> {
    let mut options = CylindricalProfileOptions::default();
    options.max_operations -= source.rounding_operations();
    source
        .curves
        .iter()
        .map(|curve| fit_curve(curve, level, limit, &mut options, cancelled))
        .collect()
}

fn fit_curve(
    curve: &input::Curve<'_>,
    level: u8,
    limit: f64,
    options: &mut CylindricalProfileOptions,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Cover> {
    let direction = curve.is_mapped_spline().then(|| {
        let [first, last] = curve.endpoints();
        std::array::from_fn(|axis| last[axis] - first[axis])
    });
    let mut spans = Vec::new();
    for native in &curve.spans {
        let local = fit_native(native, direction, level, limit, *options, cancelled)?;
        let work = local.last().map_or(0, |span| span.bounds.operations);
        options.max_operations -= work;
        spans.extend(local.into_iter().map(|span| FittedSpan {
            span,
            map: native.map,
        }));
    }
    Ok(Cover {
        curve: curve.occurrence.curve,
        spans,
    })
}

fn fit_native(
    native: &input::NativeSpan<'_>,
    direction: Option<[f64; 2]>,
    level: u8,
    limit: f64,
    options: CylindricalProfileOptions,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Vec<RationalReferenceSpan>> {
    let mut metric = native.metric();
    if metric.weights.is_none() {
        metric.weights = Some(&[1.; 17][..metric.poles.len()]);
    }
    let direction = direction.unwrap_or_else(|| native.direction());
    let direction_scale = direction[0].abs().max(direction[1].abs());
    let limits = transport_limits([limit, direction_scale], native.transport())?;
    let query = RationalReferenceRequest {
        source: metric,
        level,
        limits,
        direction,
        options,
    };
    let mut spans = fit_rational_bezier_reference_with_proof(
        query,
        RationalReferenceProof::BisectResidualV2,
        cancelled,
    )
    .map_err(fit_failure)?;
    for span in &mut spans {
        transport_bounds(span, native.transport(), direction_scale)?;
        span.interval = native_interval(native.domain, span.interval)?;
    }
    Ok(spans)
}

fn transport_limits(
    limits: [f64; 2],
    rounding: Option<RationalReferenceBounds>,
) -> Checked<[f64; 2]> {
    let Some(rounding) = rounding else {
        return Ok(limits);
    };
    let effective = [
        (limits[0] - rounding.position).next_down(),
        (limits[1] - rounding.derivative).next_down(),
    ];
    if effective.into_iter().all(|limit| limit > 0.) {
        return Ok(effective);
    }
    Err(invalid(
        format!("source transport={rounding:?}, limits={limits:?}"),
        "positive remaining reference-fit allowances",
    ))
}

fn transport_bounds(
    span: &mut RationalReferenceSpan,
    rounding: Option<RationalReferenceBounds>,
    direction: f64,
) -> Checked<()> {
    let Some(rounding) = rounding else {
        return Ok(());
    };
    let native_fraction = (1_u64 << span.interval.level()) as f64;
    let derivative = (rounding.derivative / native_fraction).next_up();
    span.bounds.position = (span.bounds.position + rounding.position).next_up();
    span.bounds.derivative = (span.bounds.derivative + derivative).next_up();
    let discrepancy = (derivative * direction).next_up();
    span.projection_rate_lower = (span.projection_rate_lower - discrepancy).next_down();
    if span.projection_rate_lower > 0. {
        return Ok(());
    }
    Err(invalid(
        format!(
            "transported span={:?}, projected rate={}",
            span.interval, span.projection_rate_lower
        ),
        "strict native source-reference projection throughout transport",
    ))
}

fn native_interval(
    base: WrappedSourceSpan,
    relative: CylindricalProfileInterval,
) -> Checked<CylindricalProfileInterval> {
    let level = base.level + relative.level();
    if level > 63 {
        return Err(invalid(
            format!("composed native interval level={level}"),
            "an exact native interval with dyadic level at most 63",
        ));
    }
    let index = (base.index << relative.level()) + relative.index();
    CylindricalProfileInterval::new(index, level).map_err(|cause| fit_failure(cause.into()))
}

fn fit_failure(cause: RationalReferenceFailure) -> RationalWrappedFailure {
    RationalWrappedFailure::Reference(Box::new(cause))
}
