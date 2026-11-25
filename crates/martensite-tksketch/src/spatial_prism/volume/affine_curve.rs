// SPDX-License-Identifier: GPL-2.0-only
//! Read only literal affine native carriers, keeping their actual trim parameterization.
use super::interval::{Span, mapped_vector, placed};
use martensite_tkbrep::brep::{EdgeCurve3D, EdgePCurve};
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dBezierCurve, Geom2dLine};
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomBezierCurve, GeomLine};
use martensite_tkmath::gp::Transform3;

enum Curve<const N: usize> {
    Line {
        origin: [f64; N],
        direction: [f64; N],
    },
    Segment {
        poles: [[f64; N]; 2],
        knots: [f64; 2],
    },
}

pub(super) fn planar(trim: &EdgePCurve, reverse: bool, degree_one: bool) -> Option<[[Span; 2]; 2]> {
    let curve = &trim.pcurve;
    let carrier = if let Some(line) = curve.downcast_ref::<Geom2dLine>() {
        Curve::Line {
            origin: [line.location().x(), line.location().y()],
            direction: [line.direction().x(), line.direction().y()],
        }
    } else if !degree_one {
        return None;
    } else if let Some(bezier) = curve.downcast_ref::<Geom2dBezierCurve>() {
        if bezier.degree() != 1 || bezier.weight(1).ok()? != 1. || bezier.weight(2).ok()? != 1. {
            return None;
        }
        let poles =
            [bezier.pole(1).ok()?, bezier.pole(2).ok()?].map(|point| [point.x(), point.y()]);
        Curve::Segment {
            poles,
            knots: [0., 1.],
        }
    } else {
        let lease = curve
            .downcast_ref::<Geom2dBSplineCurve>()?
            .try_input_lease()
            .ok()?;
        let stored = lease.stored_input();
        if stored.periodic()
            || stored.degree() != 1
            || stored.rational()
            || stored.evaluation_override()
            || !unit_weights(stored.weights())
            || !clamped(
                stored.knots(),
                stored.multiplicities(),
                stored.flat_knots(),
                stored.domain(),
            )
        {
            return None;
        }
        Curve::Segment {
            poles: stored.poles().try_into().ok()?,
            knots: stored.domain(),
        }
    };
    carrier.endpoints(trim.first, trim.last, reverse)
}

pub(super) fn spatial(
    trim: &EdgeCurve3D,
    reverse: bool,
    degree_one: bool,
    reference: [f64; 3],
) -> Option<[[Span; 3]; 2]> {
    let curve = &trim.curve;
    let carrier = if let Some(line) = curve.downcast_ref::<GeomLine>() {
        let line = line.lin();
        Curve::Line {
            origin: *line.location().coordinates(),
            direction: *line.direction().coordinates(),
        }
    } else if !degree_one {
        return None;
    } else if let Some(bezier) = curve.downcast_ref::<GeomBezierCurve>() {
        if bezier.degree() != 1 || bezier.weight(1).ok()? != 1. || bezier.weight(2).ok()? != 1. {
            return None;
        }
        let poles = [bezier.pole(1).ok()?, bezier.pole(2).ok()?].map(|point| *point.coordinates());
        Curve::Segment {
            poles,
            knots: [0., 1.],
        }
    } else {
        let lease = curve
            .downcast_ref::<GeomBSplineCurve>()?
            .try_input_lease()
            .ok()?;
        let stored = lease.stored_input();
        if stored.periodic()
            || stored.degree() != 1
            || stored.rational()
            || stored.evaluation_override()
            || !unit_weights(stored.weights())
            || !clamped(
                stored.knots(),
                stored.multiplicities(),
                stored.flat_knots(),
                stored.domain(),
            )
        {
            return None;
        }
        Curve::Segment {
            poles: stored.poles().try_into().ok()?,
            knots: stored.domain(),
        }
    };
    let transform = trim.location.transformation();
    let parameters = parameters(trim.first, trim.last, reverse)?;
    Some([
        spatial_point(&carrier, parameters[0], transform, reference)?,
        spatial_point(&carrier, parameters[1], transform, reference)?,
    ])
}

fn spatial_point(
    carrier: &Curve<3>,
    t: f64,
    transform: Transform3,
    reference: [f64; 3],
) -> Option<[Span; 3]> {
    let point = match *carrier {
        Curve::Line { origin, direction } => {
            let origin = placed(origin.map(Span::at), transform, reference);
            let direction = mapped_vector(direction.map(Span::at), transform);
            std::array::from_fn(|axis| origin[axis] + Span::at(t) * direction[axis])
        }
        Curve::Segment { poles, knots } if (knots[0]..=knots[1]).contains(&t) => {
            let poles = poles.map(|pole| placed(pole.map(Span::at), transform, reference));
            interpolate(poles, t, knots)
        }
        Curve::Segment { .. } => return None,
    };
    point.iter().all(|span| span.finite()).then_some(point)
}

fn parameters(first: f64, last: f64, reverse: bool) -> Option<[f64; 2]> {
    (first.is_finite() && last.is_finite() && first < last).then_some(if reverse {
        [last, first]
    } else {
        [first, last]
    })
}

fn interpolate<const N: usize>(poles: [[Span; N]; 2], t: f64, knots: [f64; 2]) -> [Span; N] {
    if t == knots[0] {
        return poles[0];
    }
    if t == knots[1] {
        return poles[1];
    }
    let parameter = Span::normalized(t, knots);
    std::array::from_fn(|axis| poles[0][axis] + parameter * (poles[1][axis] - poles[0][axis]))
}

fn unit_weights(weights: &[f64]) -> bool {
    weights.is_empty() || weights == [1., 1.]
}

pub(super) fn clamped(knots: &[f64], mults: &[i32], flat: &[f64], domain: [f64; 2]) -> bool {
    let [start, end] = domain;
    start.is_finite()
        && end.is_finite()
        && start < end
        && knots == domain
        && mults == [2, 2]
        && flat == [start, start, end, end]
}

impl<const N: usize> Curve<N> {
    fn endpoints(&self, first: f64, last: f64, reverse: bool) -> Option<[[Span; N]; 2]> {
        let parameters = parameters(first, last, reverse)?;
        Some([self.point(parameters[0])?, self.point(parameters[1])?])
    }

    fn point(&self, t: f64) -> Option<[Span; N]> {
        let result = match *self {
            Self::Line { origin, direction } => std::array::from_fn(|axis| {
                Span::at(origin[axis]) + Span::at(t) * Span::at(direction[axis])
            }),
            Self::Segment { poles, knots } if (knots[0]..=knots[1]).contains(&t) => {
                interpolate(poles.map(|poles| poles.map(Span::at)), t, knots)
            }
            Self::Segment { .. } => return None,
        };
        result.iter().all(|span| span.finite()).then_some(result)
    }
}
