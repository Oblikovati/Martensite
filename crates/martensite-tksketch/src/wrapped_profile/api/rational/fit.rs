//! Bind source-local reference spans to the authored normalized spline parameter.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CylindricalProfileInterval, CylindricalProfileOptions, RationalReferenceRequest,
    fit_rational_bezier_reference,
};

pub(super) struct Cover {
    pub curve: ProfileCurveId,
    pub spans: Vec<RationalReferenceSpan>,
}

pub(super) fn all_spans(covers: &[Cover]) -> impl Iterator<Item = &RationalReferenceSpan> + Clone {
    covers.iter().flat_map(|cover| cover.spans.iter())
}

pub(super) fn cover(
    level: u8,
    source: &input::Source<'_>,
    limit: f64,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Vec<Cover>> {
    let mut options = CylindricalProfileOptions::default();
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
    let mut spans = Vec::new();
    for native in &curve.spans {
        let mut local = fit_native(native, level, limit, *options, cancelled)?;
        let work = local.last().map_or(0, |span| span.bounds.operations);
        options.max_operations -= work;
        spans.append(&mut local);
    }
    Ok(Cover {
        curve: curve.occurrence.curve,
        spans,
    })
}

fn fit_native(
    native: &input::NativeSpan<'_>,
    level: u8,
    limit: f64,
    options: CylindricalProfileOptions,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Vec<RationalReferenceSpan>> {
    let mut metric = native.metric();
    if metric.weights.is_none() {
        metric.weights = Some(&[1.; 17][..metric.poles.len()]);
    }
    let query = RationalReferenceRequest {
        source: metric,
        level,
        limits: [limit, limit],
        direction: native.direction(),
        options,
    };
    let mut spans = fit_rational_bezier_reference(query, cancelled).map_err(fit_failure)?;
    for span in &mut spans {
        span.interval = native_interval(native.domain, span.interval)?;
    }
    Ok(spans)
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
