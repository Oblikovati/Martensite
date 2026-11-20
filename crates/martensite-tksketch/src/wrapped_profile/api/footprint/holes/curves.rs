//! Use full positive-basis hulls so a native edge subrange never relies on sampled extrema.
use super::*;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{
    Geom2dBSplineCurve, Geom2dBezierCurve, Geom2dCircle, Geom2dCurve, Geom2dEllipse, Geom2dLine,
    Geom2dTrimmedCurve,
};
use martensite_tkmath::gp::Axis22d;

pub(super) fn bounds(curve: &Handle<dyn Geom2dCurve>, range: [f64; 2]) -> Checked<TrimBox> {
    let trimmed = curve.cast::<Geom2dTrimmedCurve>();
    let source = trimmed
        .get()
        .map_or_else(|| curve.clone(), Geom2dTrimmedCurve::basis_curve);
    let spline = source.cast::<Geom2dBSplineCurve>();
    if let Some(spline) = spline.get() {
        return spline_bounds(spline, range);
    }
    let bezier = source.cast::<Geom2dBezierCurve>();
    if let Some(bezier) = bezier.get() {
        return bezier_bounds(bezier, range);
    }
    let line = source.cast::<Geom2dLine>();
    if let Some(line) = line.get() {
        return line_bounds(line, range);
    }
    conic_bounds(&source)
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
        "line, ellipse, circle, or stored positive-weight spline inner pcurve",
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

fn spline_bounds(curve: &Geom2dBSplineCurve, range: [f64; 2]) -> Checked<TrimBox> {
    let lease = curve
        .try_input_lease()
        .map_err(|_| unsupported("readable stored inner spline fields"))?;
    let source = lease.stored_input();
    let [first, last] = source.domain();
    if source.evaluation_override() || range[0] < first || range[1] > last {
        return Err(unsupported(
            "inner spline within its original evaluator domain without override",
        ));
    }
    if (source.rational() && source.weights().len() != source.poles().len())
        || !source.weights().iter().all(|w| w.is_finite() && *w > 0.)
    {
        return Err(unsupported("finite positive inner spline weights"));
    }
    hull(source.poles().iter().copied())
}

fn bezier_bounds(curve: &Geom2dBezierCurve, range: [f64; 2]) -> Checked<TrimBox> {
    if curve.has_eval_representation() || range[0] < 0. || range[1] > 1. {
        return Err(unsupported(
            "inner Bézier on its unit interval without evaluator override",
        ));
    }
    let (poles, weights) = curve.poles_weights_snapshot();
    if weights.as_ref().is_some_and(|weights| {
        weights.len() != poles.len() || weights.iter().any(|w| !w.is_finite() || *w <= 0.)
    }) {
        return Err(unsupported("finite positive inner Bézier weights"));
    }
    hull(poles.iter().map(|point| [point.x(), point.y()]))
}

fn line_bounds(line: &Geom2dLine, range: [f64; 2]) -> Checked<TrimBox> {
    let origin = line.location();
    let direction = line.direction();
    let mut bounds = TrimBox::empty();
    for t in range {
        let x = affine(origin.x(), direction.x(), t);
        let y = affine(origin.y(), direction.y(), t);
        let endpoint = TrimBox([x, y]);
        if !endpoint.finite() {
            return Err(unsupported("finite enclosed inner trim line"));
        }
        bounds.include_box(endpoint);
    }
    Ok(bounds)
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
            return Err(unsupported("finite stored inner pcurve controls"));
        }
        bounds.include(point);
    }
    if !bounds.finite() {
        return Err(unsupported("nonempty stored inner pcurve controls"));
    }
    Ok(bounds)
}

#[cfg(test)]
mod tests;
