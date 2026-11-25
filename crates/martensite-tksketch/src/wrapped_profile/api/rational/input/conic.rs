//! Bind original analytic conic parameters before any stored reference fit.
use super::*;
use martensite_tkbo::profile_extrusion::{
    ExactConicQuarter, Resolution, admitted_conic_spans, exact_closed_conic,
    exact_conic_arc_source, exact_conic_quarter,
};
use martensite_tkmath::predicates::roots::metric_bounds::{
    AdmittedConicPhase, AdmittedConicSpan, CertifiedAdmittedConicSpan, certify_admitted_conic_span,
    lower_rational_conic_segment,
};

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
    let spans = if let Some(quarter) = quarter {
        vec![conic_span(quarter)]
    } else if let Some(span) =
        try_exact_arc(profile, used, resolution, analytic_options, cancelled)?
    {
        vec![span]
    } else {
        sewn_spans(profile, used.curve, resolution, analytic_options, cancelled)?
    };
    Ok(Curve {
        spans,
        occurrence: used,
        ends: Some(ends),
    })
}

fn try_exact_arc<'a>(
    profile: &SketchProfile,
    used: LoopUse,
    resolution: Resolution,
    options: &mut CylindricalProfileOptions,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Option<NativeSpan<'a>>> {
    let Some(source) =
        exact_conic_arc_source(profile, used.curve, resolution).map_err(analytic_failure)?
    else {
        return Ok(None);
    };
    let map = WrappedAnalyticArcMap {
        first: WrappedAnalyticArcEndpoint::Start,
        last: WrappedAnalyticArcEndpoint::End,
        increasing: source.increasing,
    };
    let lowered = match lower_rational_conic_segment(source, *options, cancelled) {
        Ok(lowered) => lowered,
        Err(
            RationalConicSegmentFailure::Endpoint { .. } | RationalConicSegmentFailure::Sweep(_),
        ) => {
            // The exact admission charged its initial policy poll before computing its fixed-size ratios.
            options.max_operations -= 1;
            return Ok(None);
        }
        Err(cause) => {
            return Err(RationalWrappedFailure::ConicTransport {
                curve: used.curve,
                cause: Box::new(cause),
            });
        }
    };
    options.max_operations -= lowered.transport.operations;
    Ok(Some(NativeSpan {
        controls: NativeControls::TransportedConic(Box::new(lowered)),
        domain: unit_interval(),
        map: Some(ParameterMap::Analytic(map)),
    }))
}

fn analytic_failure(cause: OperationDecline) -> RationalWrappedFailure {
    RationalWrappedFailure::Analytic(Box::new(cause))
}

pub(super) fn closed_conic<'a>(
    used: LoopUse,
    profile: &SketchProfile,
    options: WrappedNormalOptions,
    policy: &mut CylindricalProfileOptions,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Curve<'a>> {
    let resolution = pipeline::resolution(options.spatial_error);
    let exact = exact_closed_conic(profile, used.curve, resolution).map_err(analytic_failure)?;
    let spans = if let Some(quarters) = exact {
        quarters.into_iter().map(conic_span).collect()
    } else {
        sewn_spans(profile, used.curve, resolution, policy, cancelled)?
    };
    Ok(Curve {
        spans,
        occurrence: used,
        ends: None,
    })
}

pub(super) struct SewnConic {
    pub center: [f64; 2],
    pub poles: [[f64; 2]; 3],
    pub weights: [f64; 3],
    pub proof: CertifiedAdmittedConicSpan,
}

fn sewn_spans<'a>(
    profile: &SketchProfile,
    curve: ProfileCurveId,
    resolution: Resolution,
    policy: &mut CylindricalProfileOptions,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Vec<NativeSpan<'a>>> {
    let pieces = admitted_conic_spans(profile, curve, resolution)
        .map_err(analytic_failure)?
        .ok_or_else(|| {
            invalid(
                format!("conic={curve:?}"),
                "one bounded sewn analytic conic partition",
            )
        })?;
    pieces
        .into_iter()
        .map(|piece| sewn_span(piece, curve, resolution.sew, policy, cancelled))
        .collect()
}

fn sewn_span<'a>(
    piece: AdmittedConicSpan,
    curve: ProfileCurveId,
    sew: f64,
    policy: &mut CylindricalProfileOptions,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<NativeSpan<'a>> {
    let proof = certify_admitted_conic_span(piece, *policy, cancelled).map_err(|cause| {
        RationalWrappedFailure::AdmittedConicTransport {
            curve,
            cause: Box::new(cause),
        }
    })?;
    if proof.endpoint_error > sew {
        return Err(invalid(
            format!(
                "conic={curve:?}, endpoint error={}, sew={sew}",
                proof.endpoint_error
            ),
            "a certified analytic endpoint gap within the original sewing limit",
        ));
    }
    policy.max_operations -= proof.transport.operations;
    let controls = SewnConic {
        center: piece.frame.center,
        poles: piece.poles,
        weights: piece.weights,
        proof,
    };
    Ok(NativeSpan {
        controls: NativeControls::SewnConic(Box::new(controls)),
        domain: unit_interval(),
        map: Some(sewn_map(piece.phase)),
    })
}

fn sewn_map(phase: AdmittedConicPhase) -> ParameterMap {
    match phase {
        AdmittedConicPhase::Quarter(quarter) => ParameterMap::Quarter(WrappedConicParameterMap {
            first_quarter: quarter as i8,
            increasing: true,
        }),
        AdmittedConicPhase::Sweep {
            first,
            sweep,
            index,
            count,
            parameter,
        } => ParameterMap::Admitted(WrappedAdmittedArcMap {
            angles: [first.to_bits(), sweep.to_bits()],
            index,
            count,
            parameter,
        }),
    }
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
