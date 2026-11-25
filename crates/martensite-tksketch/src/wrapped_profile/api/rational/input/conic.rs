//! Bind original analytic conic parameters before any stored reference fit.
use super::*;
use martensite_tkbo::profile_extrusion::{
    ExactConicQuarter, exact_closed_conic, exact_conic_arc_source, exact_conic_quarter,
};
use martensite_tkmath::predicates::roots::metric_bounds::lower_rational_conic_segment;

pub(super) fn analytic_arc<'a>(
    curve: &ProfileCurve,
    used: LoopUse,
    profile: &SketchProfile,
    options: WrappedNormalOptions,
    analytic_options: &mut CylindricalProfileOptions,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Curve<'a>> {
    let Some(ends) = curve.ends() else {
        return Err(invalid(
            format!("arc={:?}", used.curve),
            "two original analytic endpoints",
        ));
    };
    let resolution = pipeline::resolution(options.spatial_error);
    let quarter = exact_conic_quarter(profile, used.curve, resolution).map_err(analytic_failure)?;
    let span = match quarter {
        Some(quarter) => conic_span(quarter),
        None => transported_arc(profile, used, resolution, analytic_options, cancelled)?,
    };
    Ok(Curve {
        spans: vec![span],
        occurrence: used,
        ends: Some(ends),
    })
}

fn transported_arc<'a>(
    profile: &SketchProfile,
    used: LoopUse,
    resolution: martensite_tkbo::profile_extrusion::Resolution,
    options: &mut CylindricalProfileOptions,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<NativeSpan<'a>> {
    let source = exact_conic_arc_source(profile, used.curve, resolution)
        .map_err(analytic_failure)?
        .ok_or_else(|| {
            invalid(
                format!("arc={:?}", used.curve),
                "an exact affine analytic carrier",
            )
        })?;
    let map = WrappedAnalyticArcMap {
        first: WrappedAnalyticArcEndpoint::Start,
        last: WrappedAnalyticArcEndpoint::End,
        increasing: source.increasing,
    };
    let lowered = lower_rational_conic_segment(source, *options, cancelled).map_err(|cause| {
        RationalWrappedFailure::ConicTransport {
            curve: used.curve,
            cause: Box::new(cause),
        }
    })?;
    options.max_operations -= lowered.transport.operations;
    Ok(NativeSpan {
        controls: NativeControls::TransportedConic(Box::new(lowered)),
        domain: unit_interval(),
        map: Some(ParameterMap::Analytic(map)),
    })
}

fn analytic_failure(cause: OperationDecline) -> RationalWrappedFailure {
    RationalWrappedFailure::Analytic(Box::new(cause))
}

pub(super) fn closed_conic<'a>(
    used: LoopUse,
    profile: &SketchProfile,
    options: WrappedNormalOptions,
) -> Checked<Curve<'a>> {
    let quarters = exact_closed_conic(
        profile,
        used.curve,
        pipeline::resolution(options.spatial_error),
    )
    .map_err(|cause| RationalWrappedFailure::Analytic(Box::new(cause)))?
    .ok_or_else(|| {
        invalid(
            format!("conic={:?}", used.curve),
            "an exact circle or axis-aligned ellipse",
        )
    })?;
    Ok(Curve {
        spans: quarters.into_iter().map(conic_span).collect(),
        occurrence: used,
        ends: None,
    })
}

fn conic_span<'a>(quarter: ExactConicQuarter) -> NativeSpan<'a> {
    let map = WrappedConicParameterMap {
        first_quarter: quarter.first_quarter,
        increasing: quarter.increasing,
    };
    NativeSpan {
        controls: NativeControls::Quarter(quarter.poles),
        domain: unit_interval(),
        map: Some(ParameterMap::Quarter(map)),
    }
}
