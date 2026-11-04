// SPDX-License-Identifier: GPL-2.0-only
use super::super::rational_polynomial::{RationalPolynomial as Polynomial, ratio};
use super::capture::CurvePiece;
use super::contact_types::GivenVertexContactFailure as Failure;
use super::isolation::Work;
use crate::predicates::roots::certificate_arithmetic::ExactRatio;
use crate::predicates::roots::{chain::SturmChain, polynomial::IntegerPolynomial};
use std::cmp::Ordering;

pub(super) struct LineFrame {
    pub origin: [ExactRatio; 3],
    pub direction: [ExactRatio; 3],
    pub axis: usize,
    pub bounds: [f64; 2],
}
impl LineFrame {
    pub fn new(piece: &CurvePiece) -> Result<Self, Failure> {
        let origin = std::array::from_fn(|i| piece.coordinates[i].coefficient(0));
        let direction: [ExactRatio; 3] =
            std::array::from_fn(|i| piece.coordinates[i].coefficient(1));
        let axis = direction
            .iter()
            .position(|v| v.sign() != Ordering::Equal)
            .ok_or(Failure::ZeroDirection)?;
        Ok(Self {
            origin,
            direction,
            axis,
            bounds: piece.range,
        })
    }
    pub fn relative(&self, piece: &CurvePiece) -> [Polynomial; 3] {
        std::array::from_fn(|i| {
            piece.coordinates[i].add(&Polynomial::constant(self.origin[i].negated()))
        })
    }
    pub fn parameter_bounds(&self, piece: &CurvePiece) -> [Polynomial; 2] {
        let coordinate = self.relative(piece)[self.axis].clone();
        self.bounds.map(|bound| {
            coordinate.add(&Polynomial::constant(
                self.direction[self.axis].multiply(&ratio(bound)).negated(),
            ))
        })
    }
}
pub(super) fn equation(
    piece: &CurvePiece,
    line: &LineFrame,
    index: usize,
    work: &mut Work<'_>,
) -> Result<Option<IntegerPolynomial>, Failure> {
    work.charge().map_err(Failure::Exact)?;
    let relative = line.relative(piece);
    let axes: Vec<usize> = (0..3).filter(|i| *i != line.axis).collect();
    let projected = project(&axes, &relative, line);
    let first = projected[0].to_integer(2).map_err(root_failure)?;
    let second = projected[1].to_integer(2).map_err(root_failure)?;
    common(first, second, index, work)
}
fn project(axes: &[usize], relative: &[Polynomial; 3], line: &LineFrame) -> [Polynomial; 2] {
    std::array::from_fn(|i| {
        relative[axes[i]]
            .scale(&line.direction[line.axis])
            .add(&relative[line.axis].scale(&line.direction[axes[i]].negated()))
    })
}

fn common(
    first: Option<IntegerPolynomial>,
    second: Option<IntegerPolynomial>,
    span: usize,
    work: &mut Work<'_>,
) -> Result<Option<IntegerPolynomial>, Failure> {
    match (first, second) {
        (None, None) => Err(Failure::CollinearUnclassified { span }),
        (Some(p), None) | (None, Some(p)) => Ok((p.degree() > 0).then_some(p)),
        (Some(a), Some(b)) => SturmChain::from_pair(a, b, &mut work.cancelled)
            .map(|chain| chain.repeated_factor())
            .map_err(root_failure),
    }
}
pub(super) fn root_failure(cause: crate::predicates::roots::RootIsolationError) -> Failure {
    Failure::Exact(super::VertexNeighborhoodFailure::Roots(cause))
}
