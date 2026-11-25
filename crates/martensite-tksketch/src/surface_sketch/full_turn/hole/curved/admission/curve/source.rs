// SPDX-License-Identifier: GPL-2.0-only
//! Capture original Bernstein controls and analytic parameter maps before fitting.
use super::*;
use martensite_tkbo::profile_extrusion::{Resolution, exact_conic_arc_source, exact_conic_quarter};
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedRationalConicSegment, lower_rational_conic_segment,
};

pub(super) struct SourceNet {
    pub points: Vec<[f64; 2]>,
    pub weights: Option<[f64; 3]>,
    pub conic_map: Option<PeriodicConicMap>,
}

pub(super) fn capture(
    profile: &SketchProfile,
    curve: &ProfileCurve,
    [first, last]: [[f64; 2]; 2],
    model: SourceModel,
) -> Checked<SourceNet> {
    match (curve, model) {
        (ProfileCurve::Line { .. }, _) => Ok(SourceNet {
            points: vec![first, last],
            weights: None,
            conic_map: None,
        }),
        (
            ProfileCurve::Arc { .. } | ProfileCurve::EllipticArc { .. },
            SourceModel::Rational(resolution),
        ) => conic_net(profile, curve, resolution),
        _ => spline_net(curve, [first, last], model),
    }
}

fn spline_net(curve: &ProfileCurve, ends: [[f64; 2]; 2], model: SourceModel) -> Checked<SourceNet> {
    let ProfileCurve::Spline {
        degree: 2,
        poles,
        weights,
        knots,
        ..
    } = curve
    else {
        return Err(incomplete(
            "line, exact conic segment or clamped quadratic span",
            format!("{curve:?}"),
        ));
    };
    if poles.len() != 3 || [poles[0], poles[2]] != ends || knots != &[0., 0., 0., 1., 1., 1.] {
        return Err(incomplete(
            "literal clamped quadratic span and its authored endpoints",
            format!("{curve:?}"),
        ));
    }
    Ok(SourceNet {
        points: poles.clone(),
        weights: source_weights(curve.id(), weights.as_deref(), model)?,
        conic_map: None,
    })
}

fn conic_net(
    profile: &SketchProfile,
    curve: &ProfileCurve,
    resolution: Resolution,
) -> Checked<SourceNet> {
    let id = curve.id();
    let quarter = exact_conic_quarter(profile, id, resolution).map_err(|cause| {
        incomplete(
            "admitted exact analytic conic",
            format!("{id:?}: {cause:?}"),
        )
    })?;
    let Some(quarter) = quarter else {
        return exact_short_arc(profile, curve, resolution);
    };
    Ok(SourceNet {
        points: quarter.poles.into(),
        weights: Some([1., 1., 2.]),
        conic_map: Some(PeriodicConicMap::Quarter(WrappedConicParameterMap {
            first_quarter: quarter.first_quarter,
            increasing: quarter.increasing,
        })),
    })
}

fn exact_short_arc(
    profile: &SketchProfile,
    curve: &ProfileCurve,
    resolution: Resolution,
) -> Checked<SourceNet> {
    let id = curve.id();
    let source = exact_conic_arc_source(profile, id, resolution)
        .map_err(|cause| {
            incomplete(
                "admitted exact analytic conic",
                format!("{id:?}: {cause:?}"),
            )
        })?
        .ok_or_else(|| incomplete("exact short analytic conic source", format!("{id:?}")))?;
    let lowered =
        lower_rational_conic_segment(source, Default::default(), || false).map_err(|cause| {
            incomplete(
                "exact short analytic conic source",
                format!("{id:?}: {cause:?}"),
            )
        })?;
    if !exact_binary64_coefficients(&lowered) {
        return Err(incomplete(
            "exactly representable analytic conic cut controls",
            format!("{id:?}"),
        ));
    }
    let map = WrappedAnalyticArcMap {
        first: WrappedAnalyticArcEndpoint::Start,
        last: WrappedAnalyticArcEndpoint::End,
        increasing: source.increasing,
    };
    Ok(SourceNet {
        points: lowered.poles.into(),
        weights: Some(lowered.weights),
        conic_map: Some(PeriodicConicMap::Arc(map)),
    })
}

fn exact_binary64_coefficients(lowered: &CertifiedRationalConicSegment) -> bool {
    lowered
        .control_enclosures
        .iter()
        .flatten()
        .chain(&lowered.weight_enclosures)
        .all(|range| range[0] == range[1])
}

fn source_weights(
    id: martensite_tkbo::profile_extrusion::ProfileCurveId,
    weights: Option<&[f64]>,
    model: SourceModel,
) -> Checked<Option<[f64; 3]>> {
    match (weights, model) {
        (None, _) => Ok(None),
        (Some([a, b, c]), SourceModel::Rational(_))
            if [*a, *b, *c].into_iter().all(|w| w.is_finite() && w > 0.) =>
        {
            Ok(Some([*a, *b, *c]))
        }
        _ => Err(incomplete(
            "three positive rational quadratic weights",
            format!("{id:?}: {weights:?}"),
        )),
    }
}
