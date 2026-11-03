//! Exact projected quadratic spans retain original-coordinate Cox–de Boor coefficients.
use super::super::{input::SketchIncidenceSpline, spline_admission};
use super::{RationalPolynomial, bernstein, clipped_range, ratio, span_piece};
use crate::predicates::roots::certificate_arithmetic::ExactRatio;
use crate::predicates::roots::sketch_incidence::SketchIncidenceCause;

#[derive(Clone, Debug, PartialEq)]
pub(in crate::predicates::roots) struct ProjectedQuadraticSpan {
    pub range: [f64; 2],
    pub powers: [[ExactRatio; 3]; 2],
    pub controls: [[ExactRatio; 2]; 3],
}
#[derive(Clone, Debug, PartialEq)]
pub(in crate::predicates::roots) enum SpanBridgeFailure {
    Admission(SketchIncidenceCause),
    Direction([f64; 3]),
    Cancelled,
    NonUnitDenominator,
    CoefficientShape(usize),
}

pub(in crate::predicates::roots) fn projected_spans(
    poles: &[[f64; 3]],
    flat_knots: &[f64],
    direction: [f64; 3],
    bounds: [f64; 2],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<ProjectedQuadraticSpan>, SpanBridgeFailure> {
    ProjectionFrame::new(poles, flat_knots, direction, bounds)?.capture(cancelled)
}

struct ProjectionFrame {
    xy: SketchIncidenceSpline,
    z: SketchIncidenceSpline,
    knots: Vec<ExactRatio>,
    direction: [f64; 3],
    axis: usize,
    others: Vec<usize>,
}
impl ProjectionFrame {
    fn new(
        poles: &[[f64; 3]],
        flat_knots: &[f64],
        direction: [f64; 3],
        bounds: [f64; 2],
    ) -> Result<Self, SpanBridgeFailure> {
        let axis = direction_axis(direction)?;
        let others: Vec<usize> = (0..3).filter(|i| *i != axis).collect();
        let xy = coordinate_source(poles, flat_knots, [others[0], others[1]], bounds);
        let z = coordinate_source(poles, flat_knots, [axis, axis], bounds);
        admit_sources(&xy, &z)?;
        Ok(Self {
            xy,
            z,
            knots: flat_knots.iter().copied().map(ratio).collect(),
            direction,
            axis,
            others,
        })
    }
    fn capture(
        &self,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Vec<ProjectedQuadraticSpan>, SpanBridgeFailure> {
        let mut result = Vec::new();
        for index in 2..self.xy.poles.len() {
            if cancelled() {
                return Err(SpanBridgeFailure::Cancelled);
            }
            if let Some(range) = clipped_range(&self.xy, index) {
                let pair = [
                    span_piece(&self.xy, &self.knots, index, range),
                    span_piece(&self.z, &self.knots, index, range),
                ];
                result.push(project_span(pair, self.direction, self.axis, &self.others)?);
            }
        }
        Ok(result)
    }
}

fn admit_sources(
    xy: &SketchIncidenceSpline,
    z: &SketchIncidenceSpline,
) -> Result<(), SpanBridgeFailure> {
    spline_admission::spline(xy).map_err(SpanBridgeFailure::Admission)?;
    spline_admission::spline(z).map_err(SpanBridgeFailure::Admission)
}

fn coordinate_source(
    poles: &[[f64; 3]],
    knots: &[f64],
    axes: [usize; 2],
    bounds: [f64; 2],
) -> SketchIncidenceSpline {
    SketchIncidenceSpline {
        degree: 2,
        poles: poles.iter().map(|p| [p[axes[0]], p[axes[1]]]).collect(),
        weights: None,
        flat_knots: knots.to_vec(),
        bounds,
    }
}

fn direction_axis(direction: [f64; 3]) -> Result<usize, SpanBridgeFailure> {
    if direction.iter().any(|v| !v.is_finite()) {
        return Err(SpanBridgeFailure::Direction(direction));
    }
    direction
        .iter()
        .position(|v| *v != 0.)
        .ok_or(SpanBridgeFailure::Direction(direction))
}

fn project_span(
    pair: [super::ParametricSpan; 2],
    direction: [f64; 3],
    axis: usize,
    others: &[usize],
) -> Result<ProjectedQuadraticSpan, SpanBridgeFailure> {
    let [xy, z] = pair;
    if !unit_denominator(&xy.point.w) || !unit_denominator(&z.point.w) {
        return Err(SpanBridgeFailure::NonUnitDenominator);
    }
    let projected = [&xy.point.x, &xy.point.y].map(|p| p.scale(&ratio(direction[axis])));
    let x = projected[0].add(&z.point.x.scale(&ratio(-direction[others[0]])));
    let y = projected[1].add(&z.point.x.scale(&ratio(-direction[others[1]])));
    let range = xy.range.ok_or(SpanBridgeFailure::CoefficientShape(0))?;
    projected_span(x, y, range)
}

fn projected_span(
    x: RationalPolynomial,
    y: RationalPolynomial,
    range: [f64; 2],
) -> Result<ProjectedQuadraticSpan, SpanBridgeFailure> {
    let powers = [powers(&x)?, powers(&y)?];
    let controls = [bernstein(&x, range, 2), bernstein(&y, range, 2)];
    Ok(ProjectedQuadraticSpan {
        range,
        powers,
        controls: std::array::from_fn(|i| [controls[0][i].clone(), controls[1][i].clone()]),
    })
}

fn powers(polynomial: &RationalPolynomial) -> Result<[ExactRatio; 3], SpanBridgeFailure> {
    if polynomial.degree().is_some_and(|degree| degree > 2) {
        return Err(SpanBridgeFailure::CoefficientShape(polynomial.0.len()));
    }
    Ok(std::array::from_fn(|index| polynomial.coefficient(index)))
}

fn unit_denominator(polynomial: &RationalPolynomial) -> bool {
    polynomial.coefficient(0) == ratio(1.) && polynomial.degree() == Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nonuniform_spans_keep_exact_nonbinary_coefficients() {
        let poles = [[0., 0., 0.], [1., 2., 0.], [3., 1., 0.], [4., 0., 0.]];
        let knots = [0., 0., 0., 0.1, 1., 1., 1.];
        let spans = projected_spans(&poles, &knots, [0., 0., 1.], [0., 1.], &mut || false).unwrap();
        let expected = ratio(2.).divide(&ratio(0.1));
        assert_eq!(spans[0].powers[0][1], expected);
        assert_ne!(expected, ratio(2. / 0.1));
        assert_eq!(spans.len(), 2);
    }
    #[test]
    fn oblique_quotient_is_exact_and_cancelled_capture_is_not_partial_success() {
        let poles = [[0., 0., 0.], [1., 2., 3.], [4., 5., 6.]];
        let knots = [0., 0., 0., 1., 1., 1.];
        let spans = projected_spans(&poles, &knots, [1., 2., 3.], [0., 1.], &mut || false).unwrap();
        let expected = [[0., 0.], [0., 0.], [-3., -6.]].map(|p| p.map(ratio));
        assert_eq!(spans[0].controls, expected);
        assert_eq!(
            projected_spans(&poles, &knots, [0., 0., 1.], [0., 1.], &mut || true),
            Err(SpanBridgeFailure::Cancelled)
        );
    }
}
