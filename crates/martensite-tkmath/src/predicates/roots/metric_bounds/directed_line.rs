// SPDX-License-Identifier: GPL-2.0-only
//! Exact incidence and one-round placement on directed binary64 line supports.
use super::super::certificate_arithmetic::{
    ExactRatio,
    rounding::{nearest_binary64, outward_binary64, outward_sqrt},
};
use crate::predicates::{
    NonFiniteCoordinate, exact_vector3,
    segment_contact::{SupportParameters, difference, exact_support_parameters},
};
use std::{cmp::Ordering, error::Error, fmt};

/// Exact location relative to the authored normalized interval `[0, 1]`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineParameterLocation {
    /// Strictly before the start.
    BeforeStart,
    /// At the directed start.
    Start,
    /// Strictly between the ends.
    Interior,
    /// At the directed end.
    End,
    /// Strictly after the end.
    AfterEnd,
}

/// A rational normalized parameter ordered without rounding.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DirectedLineParameter(ExactRatio);

impl DirectedLineParameter {
    /// Classify against the authored interval. Example: `parameter.location()`.
    pub fn location(&self) -> LineParameterLocation {
        use LineParameterLocation::*;
        match self.0.sign() {
            Ordering::Less => BeforeStart,
            Ordering::Equal => Start,
            Ordering::Greater => match self.0.cmp(&ExactRatio::from_finite(1.)) {
                Ordering::Less => Interior,
                Ordering::Equal => End,
                Ordering::Greater => AfterEnd,
            },
        }
    }

    /// Compare with a finite binary64 pick, preserving exact ties.
    /// Example: `hit.parameters()[0].compare_authored(0.25)?`.
    pub fn compare_authored(&self, pick: f64) -> Result<Ordering, DirectedLineIncidenceFailure> {
        if !pick.is_finite() {
            return Err(DirectedLineIncidenceFailure::InvalidPick(pick));
        }
        Ok(self.0.cmp(&ExactRatio::from_finite(pick)))
    }
}

/// Exact intersection of the infinite supports behind two nondegenerate directed lines.
#[derive(Clone, Debug, PartialEq)]
pub enum DirectedLineContact {
    /// Distinct parallel supports.
    Parallel,
    /// Nonparallel supports with no common spatial point.
    Skew,
    /// One common support. The second authored segment projects onto the first
    /// authored support in normalized coordinates, in the second line's endpoint order.
    Collinear([DirectedLineParameter; 2]),
    /// One exact transverse contact, with a parameter on each support.
    Transverse(Box<DirectedLineHit>),
}

/// An exact transverse hit bound to the original authored endpoints of both supports.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectedLineHit {
    sources: [[[f64; 3]; 2]; 2],
    parameters: [DirectedLineParameter; 2],
}

/// Rounded contact and outward enclosures in both original source parameterizations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DirectedLinePlacement {
    /// Bounds for the first and second normalized parameters, in directed order.
    pub parameters: [[f64; 2]; 2],
    /// Outward coordinate enclosures in model units.
    pub coordinates: [[f64; 2]; 3],
    /// Contact rounded once to nearest, ties to even. An exact boundary endpoint retains its bits.
    pub position: [f64; 3],
    /// Outward Euclidean error upper bound relative to the exact contact.
    pub error_upper: f64,
}

/// Evidence that requires a finite binary64 enclosure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineEvidenceQuantity {
    /// Parameter of the first (0) or second (1) directed source.
    Parameter(usize),
    /// Coordinate of the exact contact, with zero-based world axis index.
    Coordinate(usize),
    /// Euclidean error of the rounded contact.
    PositionError,
}

/// Invalid input or exact contact evidence that cannot be represented.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum DirectedLineIncidenceFailure {
    /// Coordinate of the first start/end or second start/end, numbered 0 through 3.
    NonFinite(NonFiniteCoordinate),
    /// The given source, numbered 0 or 1, has coincident endpoints.
    DegenerateSource(usize),
    /// The supplied normalized pick is not finite.
    InvalidPick(f64),
    /// The exact quantity has no finite binary64 enclosure.
    Unrepresentable(LineEvidenceQuantity),
}

impl fmt::Display for DirectedLineIncidenceFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite(error) => error.fmt(output),
            Self::DegenerateSource(index) => write!(
                output,
                "directed line source {index} must have distinct endpoints"
            ),
            Self::InvalidPick(pick) => write!(output, "directed line pick {pick} must be finite"),
            Self::Unrepresentable(quantity) => write!(
                output,
                "directed line {quantity:?} must have a finite binary64 enclosure"
            ),
        }
    }
}

impl Error for DirectedLineIncidenceFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NonFinite(error) => Some(error),
            _ => None,
        }
    }
}

/// Find spatial support incidence exactly over the supplied finite binary64 coordinates.
/// Example: `directed_line_contact([[0.,0.,0.],[1.,0.,0.]], [[0.,-1.,0.],[0.,1.,0.]])?`.
pub fn directed_line_contact(
    first: [[f64; 3]; 2],
    second: [[f64; 3]; 2],
) -> Result<DirectedLineContact, DirectedLineIncidenceFailure> {
    let [a, b, c, d] = [first[0], first[1], second[0], second[1]];
    let read = |coordinates, index| {
        exact_vector3(coordinates, index).map_err(DirectedLineIncidenceFailure::NonFinite)
    };
    let (a, b, c, d) = (read(a, 0)?, read(b, 1)?, read(c, 2)?, read(d, 3)?);
    if a == b {
        return Err(DirectedLineIncidenceFailure::DegenerateSource(0));
    }
    if c == d {
        return Err(DirectedLineIncidenceFailure::DegenerateSource(1));
    }
    let directions = [difference(&b, &a), difference(&d, &c)];
    let delta = difference(&c, &a);
    let contact = exact_support_parameters(&directions[0], &directions[1], &delta);
    Ok(match contact {
        SupportParameters::Parallel => DirectedLineContact::Parallel,
        SupportParameters::Skew => DirectedLineContact::Skew,
        SupportParameters::Transverse {
            numerators,
            denominator,
        } => {
            let parameters =
                numerators.map(|n| DirectedLineParameter(ExactRatio::new(n, denominator.clone())));
            DirectedLineContact::Transverse(Box::new(DirectedLineHit {
                sources: [first, second],
                parameters,
            }))
        }
        SupportParameters::Collinear(Some(axis)) => {
            let numerator = [delta[axis].clone(), &d[axis] - &a[axis]];
            let range = numerator
                .map(|n| DirectedLineParameter(ExactRatio::new(n, directions[0][axis].clone())));
            DirectedLineContact::Collinear(range)
        }
        SupportParameters::Collinear(None) => {
            return Err(DirectedLineIncidenceFailure::DegenerateSource(0));
        }
    })
}

impl DirectedLineHit {
    /// Borrow the two exact parameter handles. Example: `hit.parameters()[0].location()`.
    pub fn parameters(&self) -> &[DirectedLineParameter; 2] {
        &self.parameters
    }

    /// Certify the rounded contact only when needed by the consuming operation.
    /// Example: `hit.placement()?.error_upper`.
    pub fn placement(&self) -> Result<DirectedLinePlacement, DirectedLineIncidenceFailure> {
        let parameters = [
            parameter_enclosure(&self.parameters[0], 0)?,
            parameter_enclosure(&self.parameters[1], 1)?,
        ];
        let [start, end] = self.sources[0];
        let t = &self.parameters[0].0;
        let contact = std::array::from_fn(|axis| {
            let start = ExactRatio::from_finite(start[axis]);
            let end = ExactRatio::from_finite(end[axis]);
            start.add(&end.subtract(&start).multiply(t))
        });
        let (position, coordinates, error_upper) =
            enclose_position(contact, self.boundary_endpoint())?;
        Ok(DirectedLinePlacement {
            parameters,
            coordinates,
            position,
            error_upper,
        })
    }
    fn boundary_endpoint(&self) -> Option<[f64; 3]> {
        match self.parameters[1].location() {
            LineParameterLocation::Start => Some(self.sources[1][0]),
            LineParameterLocation::End => Some(self.sources[1][1]),
            _ => None,
        }
    }
}

fn parameter_enclosure(
    parameter: &DirectedLineParameter,
    index: usize,
) -> Result<[f64; 2], DirectedLineIncidenceFailure> {
    outward_binary64(&parameter.0).ok_or(DirectedLineIncidenceFailure::Unrepresentable(
        LineEvidenceQuantity::Parameter(index),
    ))
}

type PositionEnclosure = ([f64; 3], [[f64; 2]; 3], f64);

fn enclose_position(
    exact: [ExactRatio; 3],
    literal: Option<[f64; 3]>,
) -> Result<PositionEnclosure, DirectedLineIncidenceFailure> {
    let mut position = [0.; 3];
    let mut coordinates = [[0.; 2]; 3];
    let mut squared = ExactRatio::from_finite(0.);
    for (axis, value) in exact.iter().enumerate() {
        let overflow =
            DirectedLineIncidenceFailure::Unrepresentable(LineEvidenceQuantity::Coordinate(axis));
        coordinates[axis] = outward_binary64(value).ok_or(overflow)?;
        position[axis] = match literal {
            Some(point) => point[axis],
            None => nearest_binary64(value).ok_or(overflow)?,
        };
        let residual = value.subtract(&ExactRatio::from_finite(position[axis]));
        squared = squared.add(&residual.multiply(&residual));
    }
    let error_upper = outward_sqrt(&squared).ok_or(
        DirectedLineIncidenceFailure::Unrepresentable(LineEvidenceQuantity::PositionError),
    )?;
    Ok((position, coordinates, error_upper))
}

#[cfg(test)]
mod tests;
