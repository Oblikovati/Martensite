//! Use full positive-basis hulls so a native edge subrange never relies on sampled extrema.
use super::*;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{
    Geom2dBSplineCurve, Geom2dBezierCurve, Geom2dCircle, Geom2dCurve, Geom2dEllipse, Geom2dLine,
    Geom2dTrimmedCurve,
};
use martensite_tkmath::{gp::Axis22d, predicates::roots::metric_bounds::exact_affine_coordinate};

pub(in crate::surface_sketch::wrapped_profile::api::footprint) struct CurveEnvelope {
    pub bounds: TrimBox,
    pub ends: Option<[[f64; 2]; 2]>,
    pub straight: bool,
}

pub(super) fn bounds(curve: &Handle<dyn Geom2dCurve>, range: [f64; 2]) -> Checked<TrimBox> {
    capture(curve, range, false).map(|capture| capture.bounds)
}

pub(in crate::surface_sketch::wrapped_profile::api::footprint) fn envelope(
    curve: &Handle<dyn Geom2dCurve>,
    range: [f64; 2],
) -> Checked<CurveEnvelope> {
    capture(curve, range, true)
}

fn capture(
    curve: &Handle<dyn Geom2dCurve>,
    range: [f64; 2],
    with_endpoints: bool,
) -> Checked<CurveEnvelope> {
    let trimmed = curve.cast::<Geom2dTrimmedCurve>();
    let source = trimmed
        .get()
        .map_or_else(|| curve.clone(), Geom2dTrimmedCurve::basis_curve);
    let spline = source.cast::<Geom2dBSplineCurve>();
    if let Some(spline) = spline.get() {
        return spline_bounds(spline, range, with_endpoints);
    }
    let bezier = source.cast::<Geom2dBezierCurve>();
    if let Some(bezier) = bezier.get() {
        return bezier_bounds(bezier, range, with_endpoints);
    }
    let line = source.cast::<Geom2dLine>();
    if let Some(line) = line.get() {
        return line_envelope(line, range, with_endpoints);
    }
    Ok(CurveEnvelope {
        bounds: conic_bounds(&source)?,
        ends: None,
        straight: false,
    })
}

fn conic_bounds(curve: &Handle<dyn Geom2dCurve>) -> Checked<TrimBox> {
    let circle = curve.cast::<Geom2dCircle>();
    if let Some(circle) = circle.get() {
        let native = native(circle.circ2d())?;
        return ellipse_box(native.axis(), [native.radius(); 2]);
    }
    let ellipse = curve.cast::<Geom2dEllipse>();
    if let Some(ellipse) = ellipse.get() {
        let native = native(ellipse.elips2d())?;
        return ellipse_box(
            native.axis(),
            [native.major_radius(), native.minor_radius()],
        );
    }
    Err(unsupported(
        "line, ellipse, circle, or stored positive-weight spline trim pcurve",
    ))
}

fn ellipse_box(frame: Axis22d, [major, minor]: [f64; 2]) -> Checked<TrimBox> {
    let origin = frame.location();
    let [x, y] = [frame.x_direction(), frame.y_direction()];
    let interval = |origin: f64, x: f64, y: f64| {
        let radius = ((major * x.abs()).next_up() + (minor * y.abs()).next_up()).next_up();
        [(origin - radius).next_down(), (origin + radius).next_up()]
    };
    let bounds = TrimBox([
        interval(origin.x(), x.x(), y.x()),
        interval(origin.y(), x.y(), y.y()),
    ]);
    if !bounds.finite() {
        return Err(unsupported("finite enclosed inner conic"));
    }
    Ok(bounds)
}

fn spline_bounds(
    curve: &Geom2dBSplineCurve,
    range: [f64; 2],
    with_endpoints: bool,
) -> Checked<CurveEnvelope> {
    let lease = curve
        .try_input_lease()
        .map_err(|_| unsupported("readable stored trim spline fields"))?;
    let source = lease.stored_input();
    let [first, last] = source.domain();
    if source.evaluation_override() || range[0] < first || range[1] > last {
        return Err(unsupported(
            "trim spline within its original evaluator domain without override",
        ));
    }
    if (source.rational() && source.weights().len() != source.poles().len())
        || !source.weights().iter().all(|w| w.is_finite() && *w > 0.)
    {
        return Err(unsupported("finite positive trim spline weights"));
    }
    let clamped = with_endpoints
        && !source.periodic()
        && range == [first, last]
        && source.multiplicities().first() == Some(&(source.degree() + 1))
        && source.multiplicities().last() == Some(&(source.degree() + 1));
    let bounds = hull(source.poles().iter().copied())?;
    let ends = if clamped {
        source
            .poles()
            .first()
            .copied()
            .zip(source.poles().last().copied())
            .map(|(a, b)| [a, b])
    } else {
        None
    };
    Ok(CurveEnvelope {
        bounds,
        ends,
        straight: source.poles().len() == 2,
    })
}

fn bezier_bounds(
    curve: &Geom2dBezierCurve,
    range: [f64; 2],
    with_endpoints: bool,
) -> Checked<CurveEnvelope> {
    if curve.has_eval_representation() || range[0] < 0. || range[1] > 1. {
        return Err(unsupported(
            "trim Bézier on its unit interval without evaluator override",
        ));
    }
    let (poles, weights) = curve.poles_weights_snapshot();
    if weights.as_ref().is_some_and(|weights| {
        weights.len() != poles.len() || weights.iter().any(|w| !w.is_finite() || *w <= 0.)
    }) {
        return Err(unsupported("finite positive trim Bézier weights"));
    }
    let uv = |point: &martensite_tkmath::gp::Point2| [point.x(), point.y()];
    let bounds = hull(poles.iter().map(uv))?;
    let ends = if with_endpoints && range == [0., 1.] {
        let controls = poles.as_slice();
        controls
            .first()
            .zip(controls.last())
            .map(|(a, b)| [uv(a), uv(b)])
    } else {
        None
    };
    Ok(CurveEnvelope {
        bounds,
        ends,
        straight: poles.len() == 2,
    })
}

fn line_envelope(
    line: &Geom2dLine,
    range: [f64; 2],
    with_endpoints: bool,
) -> Checked<CurveEnvelope> {
    let origin = line.location();
    let direction = line.direction();
    if with_endpoints {
        let origin = [origin.x(), origin.y()];
        let direction = [direction.x(), direction.y()];
        if let Some(ends) = exact_line_ends(origin, direction, range) {
            return Ok(CurveEnvelope {
                bounds: hull(ends.into_iter())?,
                ends: Some(ends),
                straight: true,
            });
        }
    }
    let mut bounds = TrimBox::empty();
    for t in range {
        let x = affine(origin.x(), direction.x(), t);
        let y = affine(origin.y(), direction.y(), t);
        let endpoint = TrimBox([x, y]);
        if !endpoint.finite() {
            return Err(unsupported("finite enclosed trim line"));
        }
        bounds.include_box(endpoint);
    }
    Ok(CurveEnvelope {
        bounds,
        ends: None,
        straight: true,
    })
}

fn exact_line_ends(
    origin: [f64; 2],
    direction: [f64; 2],
    range: [f64; 2],
) -> Option<[[f64; 2]; 2]> {
    if let Ok(ends) = band::endpoints(origin, direction, range, Orientation::Forward) {
        return Some(ends);
    }
    let point = |parameter| {
        Some([
            exact_affine_coordinate(origin[0], direction[0], parameter).ok()??,
            exact_affine_coordinate(origin[1], direction[1], parameter).ok()??,
        ])
    };
    Some([point(range[0])?, point(range[1])?])
}

fn affine(origin: f64, direction: f64, parameter: f64) -> [f64; 2] {
    let value = direction * parameter;
    [
        (origin + value.next_down()).next_down(),
        (origin + value.next_up()).next_up(),
    ]
}

fn hull(poles: impl Iterator<Item = [f64; 2]>) -> Checked<TrimBox> {
    let mut bounds = TrimBox::empty();
    for point in poles {
        if point.iter().any(|value| !value.is_finite()) {
            return Err(unsupported("finite stored trim pcurve controls"));
        }
        bounds.include(point);
    }
    if !bounds.finite() {
        return Err(unsupported("nonempty stored trim pcurve controls"));
    }
    Ok(bounds)
}

#[cfg(test)]
mod tests;
