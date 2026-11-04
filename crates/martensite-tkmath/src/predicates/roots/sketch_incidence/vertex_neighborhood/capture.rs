// SPDX-License-Identifier: GPL-2.0-only
use super::super::rational_polynomial::{RationalPolynomial as Polynomial, ratio};
use super::super::{SketchIncidenceSpline, spans, spline_admission};
use super::isolation::Work;
use super::{GivenVertexNeighborhoodInput as Input, VertexNeighborhoodCurve as Curve};
use super::{VertexNeighborhoodFailure as Failure, VertexPlacement};
use crate::predicates::roots::certificate_arithmetic::ExactRatio;

pub(super) struct CurvePiece {
    pub source_span: usize,
    pub range: [f64; 2],
    pub coordinates: [Polynomial; 3],
}
pub(super) fn pieces(input: &Input, work: &mut Work<'_>) -> Result<Vec<CurvePiece>, Failure> {
    validate(input)?;
    reserve_input(input, work)?;
    let source = match &input.curve {
        Curve::Line { origin, direction } => vec![line(*origin, *direction, input.interval)],
        Curve::Quadratic { poles, flat_knots } => spline(poles, flat_knots, input.interval, work)?,
    };
    Ok(source
        .into_iter()
        .map(|mut piece| {
            piece.coordinates = placed(piece.coordinates, &input.placements);
            piece
        })
        .collect())
}
pub(super) fn center(input: &Input) -> [ExactRatio; 3] {
    placed(
        input.point.map(|v| Polynomial::constant(ratio(v))),
        &input.point_placements,
    )
    .map(|p| p.coefficient(0))
}
fn line(origin: [f64; 3], direction: [f64; 3], range: [f64; 2]) -> CurvePiece {
    CurvePiece {
        source_span: 0,
        range,
        coordinates: std::array::from_fn(|i| {
            Polynomial::linear(ratio(origin[i]), ratio(direction[i]))
        }),
    }
}
fn spline(
    poles: &[[f64; 3]],
    knots: &[f64],
    range: [f64; 2],
    work: &mut Work<'_>,
) -> Result<Vec<CurvePiece>, Failure> {
    let sources = [[0, 1], [2, 2]].map(|axes| coordinate_source(poles, knots, range, axes));
    for source in &sources {
        spline_admission::spline(source).map_err(Failure::Spline)?;
    }
    let exact_knots: Vec<ExactRatio> = knots.iter().copied().map(ratio).collect();
    let mut result = Vec::new();
    for index in 2..poles.len() {
        work.charge()?;
        if let Some(trim) = spans::clipped_range(&sources[0], index) {
            result.push(piece(&sources, &exact_knots, index, trim));
        }
    }
    Ok(result)
}
fn coordinate_source(
    poles: &[[f64; 3]],
    knots: &[f64],
    bounds: [f64; 2],
    axes: [usize; 2],
) -> SketchIncidenceSpline {
    SketchIncidenceSpline {
        degree: 2,
        poles: poles.iter().map(|p| [p[axes[0]], p[axes[1]]]).collect(),
        weights: None,
        flat_knots: knots.to_vec(),
        bounds,
    }
}
fn piece(
    sources: &[SketchIncidenceSpline; 2],
    knots: &[ExactRatio],
    index: usize,
    range: [f64; 2],
) -> CurvePiece {
    let xy = spans::span_piece(&sources[0], knots, index, range);
    let z = spans::span_piece(&sources[1], knots, index, range);
    CurvePiece {
        source_span: index,
        range,
        coordinates: [xy.point.x, xy.point.y, z.point.x],
    }
}
fn reserve_input(input: &Input, work: &mut Work<'_>) -> Result<(), Failure> {
    let coefficients = match &input.curve {
        Curve::Line { .. } => 6_u64,
        Curve::Quadratic { poles, flat_knots } => 3 * poles.len() as u64 + flat_knots.len() as u64,
    };
    let placements = input.placements.len() as u64 + input.point_placements.len() as u64;
    work.reserve(coefficients + 13 * placements + 4)
}

fn placed(mut coordinates: [Polynomial; 3], placements: &[VertexPlacement]) -> [Polynomial; 3] {
    for frame in placements {
        coordinates =
            std::array::from_fn(|axis| {
                (0..3).fold(
                    Polynomial::constant(ratio(frame.translation[axis])),
                    |sum, column| {
                        sum.add(&coordinates[column].scale(
                            &ratio(frame.matrix[axis][column]).multiply(&ratio(frame.scale)),
                        ))
                    },
                )
            });
    }
    coordinates
}
fn finite(field: &'static str, values: impl Iterator<Item = f64>) -> Result<(), Failure> {
    for value in values {
        if !value.is_finite() {
            return Err(Failure::Input { field, value });
        }
    }
    Ok(())
}
fn validate(input: &Input) -> Result<(), Failure> {
    validate_domain(input)?;
    finite("point", input.point.into_iter())?;
    validate_curve(&input.curve)?;
    for frame in input.placements.iter().chain(&input.point_placements) {
        validate_frame(frame)?;
    }
    Ok(())
}
fn validate_domain(input: &Input) -> Result<(), Failure> {
    finite("interval", input.interval.into_iter())?;
    if input.interval[0] >= input.interval[1] {
        return Err(Failure::Input {
            field: "increasing interval",
            value: input.interval[1],
        });
    }
    if !input.radius.is_finite() || input.radius <= 0. {
        return Err(Failure::Input {
            field: "positive radius",
            value: input.radius,
        });
    }
    Ok(())
}

fn validate_curve(curve: &Curve) -> Result<(), Failure> {
    match curve {
        Curve::Line { origin, direction } => {
            finite("line", origin.iter().chain(direction).copied())
        }
        Curve::Quadratic { poles, .. } => finite("poles", poles.iter().flatten().copied()),
    }
}
fn validate_frame(frame: &VertexPlacement) -> Result<(), Failure> {
    finite(
        "placement",
        frame
            .matrix
            .iter()
            .flatten()
            .chain(&frame.translation)
            .copied()
            .chain(std::iter::once(frame.scale)),
    )
}
