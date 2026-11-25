// SPDX-License-Identifier: GPL-2.0-only
//! Full-interval transport bounds from the exact roots of a placed cylindrical chart.
use super::super::certificate_arithmetic::{
    ExactRatio,
    rounding::{outward_sqrt, outward_sqrt_bounds},
};
use crate::{
    gp::{Cylinder3, Direction3, Line3, Point3},
    predicates::NonFiniteCoordinate,
};
use std::{cmp::Ordering, fmt};

/// Why an exact cylindrical contour position bound could not be established.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum CylindricalGeneratorError {
    /// A source or carrier component was nonfinite.
    Coordinate(NonFiniteCoordinate),
    /// The cylinder did not have a strictly positive finite radius.
    Radius(f64),
    /// A constructed direction contained only zero components.
    ZeroPull,
    /// The placed chart's three tangent directions are linearly dependent.
    Frame,
    /// The requested chart has no admitted pair of isolated roots.
    NonRadial,
    /// The requested axial interval is not finite and increasing.
    Range([f64; 2]),
    /// A required bound has no finite binary64 endpoint.
    Unrepresentable,
}

impl fmt::Display for CylindricalGeneratorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Coordinate(cause) => cause.fmt(formatter),
            Self::Radius(radius) => write!(
                formatter,
                "radius {radius} requires a positive finite value"
            ),
            Self::ZeroPull => write!(formatter, "zero pull requires a nonzero direction"),
            Self::Frame => write!(
                formatter,
                "singular chart requires three independent placed directions"
            ),
            Self::NonRadial => write!(
                formatter,
                "generator comparison requires an exactly radial pull"
            ),
            Self::Range(range) => write!(
                formatter,
                "range {range:?} requires finite increasing axial parameters"
            ),
            Self::Unrepresentable => write!(
                formatter,
                "contour transport requires finite binary64 bounds"
            ),
        }
    }
}

impl std::error::Error for CylindricalGeneratorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Coordinate(cause) => Some(cause),
            _ => None,
        }
    }
}

/// Exact pull classification on the native chart after its stored placement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CylindricalPullKind {
    /// The normal equation is zero throughout the wall.
    Axial,
    /// The pull lies in the placed section plane and has two isolated roots.
    Radial,
    /// The pull has both radial and axial components.
    Skew,
}

type Checked<T> = Result<T, CylindricalGeneratorError>;
type ExactVector = [ExactRatio; 3];
type ExactInterval = [ExactRatio; 2];

/// The exact placed source coefficients and angular root bounds for one cylinder and pull.
#[derive(Clone, Debug)]
pub struct CylindricalGeneratorChart {
    center: ExactVector,
    axis: ExactVector,
    radial: Option<[ExactInterval; 3]>,
    kind: CylindricalPullKind,
}

impl CylindricalGeneratorChart {
    /// Capture the stored local chart and exact affine placement coefficients for a pull.
    /// Each row is `[linear x, linear y, linear z, translation]`.
    /// Example: `CylindricalGeneratorChart::capture(cylinder, rows, pull)?`.
    pub fn capture(cylinder: Cylinder3, rows: [[f64; 4]; 3], pull: Direction3) -> Checked<Self> {
        let radius = cylinder.radius();
        if !radius.is_finite() || radius <= 0. {
            return Err(CylindricalGeneratorError::Radius(radius));
        }
        let placement = ExactPlacement::capture(rows)?;
        let frame = cylinder.position();
        let center = placement.point(exact_point(cylinder.location(), 0)?);
        let u = placement.vector(exact_vector(*frame.x_direction().coordinates(), 1)?);
        let v = placement.vector(exact_vector(*frame.y_direction().coordinates(), 2)?);
        let axis = placement.vector(exact_vector(*frame.direction().coordinates(), 3)?);
        let components = *pull.coordinates();
        let pull = exact_vector(components, 4)?;
        if components == [0.; 3] {
            return Err(CylindricalGeneratorError::ZeroPull);
        }
        let (kind, radial) = chart_roots(&u, &v, &axis, &pull, radius)?;
        Ok(Self {
            center,
            axis,
            radial,
            kind,
        })
    }

    /// Read the exact supplied-chart pull classification. Example: `chart.kind()`.
    pub fn kind(&self) -> CylindricalPullKind {
        self.kind
    }

    /// Bound both carrier distances against their source roots over the closed axial interval.
    ///
    /// For the placed chart `O + R(U cos u + V sin u) + A v`, the first carrier
    /// corresponds to `(cos u, sin u) = (a,b)/hypot(a,b)` where `a = det(d,U,A)`
    /// and `b = det(d,V,A)`. The second carrier corresponds to its negation.
    /// Example: `chart.enclose(carriers, [0., 10.])?`.
    pub fn enclose(&self, carriers: [Line3; 2], axial: [f64; 2]) -> Checked<[f64; 2]> {
        let radial = self
            .radial
            .as_ref()
            .ok_or(CylindricalGeneratorError::NonRadial)?;
        if !axial.into_iter().all(f64::is_finite) || axial[0] >= axial[1] {
            return Err(CylindricalGeneratorError::Range(axial));
        }
        let axial = axial.map(ExactRatio::from_finite);
        let opposite = radial
            .each_ref()
            .map(|[lo, hi]| [hi.negated(), lo.negated()]);
        let positive = self.bound_carrier(carriers[0], radial, &axial, 0)?;
        let negative = self.bound_carrier(carriers[1], &opposite, &axial, 1)?;
        Ok([positive, negative])
    }

    fn bound_carrier(
        &self,
        line: Line3,
        radial: &[ExactInterval; 3],
        axial: &ExactInterval,
        index: usize,
    ) -> Checked<f64> {
        let anchor = exact_point(line.location(), 8 + index * 2)?;
        let axis = exact_vector(*line.direction().coordinates(), 9 + index * 2)?;
        let offset = std::array::from_fn(|i| anchor[i].subtract(&self.center[i]));
        let slope = std::array::from_fn(|i| axis[i].subtract(&self.axis[i]));
        // At a fixed root the residual is affine in v. Its Euclidean norm is
        // convex, so the two certified endpoint upper bounds cover the interval.
        let a = squared_endpoint_bound(&offset, &slope, radial, &axial[0]);
        let b = squared_endpoint_bound(&offset, &slope, radial, &axial[1]);
        let squared = a.max(b);
        if squared.sign() == Ordering::Equal {
            return Ok(0.);
        }
        outward_sqrt(&squared).ok_or(CylindricalGeneratorError::Unrepresentable)
    }
}

struct ExactPlacement {
    linear: [ExactVector; 3],
    translation: ExactVector,
}

impl ExactPlacement {
    fn capture(rows: [[f64; 4]; 3]) -> Checked<Self> {
        let [a, b, c] = rows;
        let linear = [linear_row(a, 5)?, linear_row(b, 6)?, linear_row(c, 7)?];
        let translation = exact_vector([a[3], b[3], c[3]], 7)?;
        Ok(Self {
            linear,
            translation,
        })
    }

    fn vector(&self, source: ExactVector) -> ExactVector {
        self.linear.each_ref().map(|row| dot(row, &source))
    }

    fn point(&self, source: ExactVector) -> ExactVector {
        let placed = self.vector(source);
        std::array::from_fn(|i| placed[i].add(&self.translation[i]))
    }
}

fn linear_row([a, b, c, _]: [f64; 4], index: usize) -> Checked<ExactVector> {
    exact_vector([a, b, c], index)
}

fn exact_point(point: Point3, index: usize) -> Checked<ExactVector> {
    exact_vector([point.x(), point.y(), point.z()], index)
}

fn exact_vector(coordinates: [f64; 3], index: usize) -> Checked<ExactVector> {
    for (axis, value) in coordinates.into_iter().enumerate() {
        if !value.is_finite() {
            return Err(CylindricalGeneratorError::Coordinate(NonFiniteCoordinate {
                point: index,
                axis,
                value,
            }));
        }
    }
    Ok(coordinates.map(ExactRatio::from_finite))
}

fn dot(a: &ExactVector, b: &ExactVector) -> ExactRatio {
    a.iter()
        .zip(b)
        .fold(ExactRatio::from_finite(0.), |sum, (a, b)| {
            sum.add(&a.multiply(b))
        })
}

fn triple(a: &ExactVector, b: &ExactVector, c: &ExactVector) -> ExactRatio {
    let cross = std::array::from_fn(|i| {
        let [j, k] = [(i + 1) % 3, (i + 2) % 3];
        b[j].multiply(&c[k]).subtract(&b[k].multiply(&c[j]))
    });
    dot(a, &cross)
}

fn chart_roots(
    u: &ExactVector,
    v: &ExactVector,
    axis: &ExactVector,
    pull: &ExactVector,
    radius: f64,
) -> Checked<(CylindricalPullKind, Option<[ExactInterval; 3]>)> {
    if triple(axis, u, v).sign() == Ordering::Equal {
        return Err(CylindricalGeneratorError::Frame);
    }
    let [a, b] = [triple(pull, u, axis), triple(pull, v, axis)];
    if a.sign() == Ordering::Equal && b.sign() == Ordering::Equal {
        return Ok((CylindricalPullKind::Axial, None));
    }
    if triple(pull, u, v).sign() != Ordering::Equal {
        return Ok((CylindricalPullKind::Skew, None));
    }
    let radius = ExactRatio::from_finite(radius);
    let magnitude = positive_magnitude(&a, &b)?;
    let radial = std::array::from_fn(|axis| {
        let numerator = radius.multiply(&u[axis].multiply(&a).add(&v[axis].multiply(&b)));
        divide_by_magnitude(&numerator, &magnitude)
    });
    Ok((CylindricalPullKind::Radial, Some(radial)))
}

fn positive_magnitude(a: &ExactRatio, b: &ExactRatio) -> Checked<ExactInterval> {
    // An axis root gives an exact magnitude without evaluating a square root.
    if let Some(single) = single_nonzero(a, b) {
        let magnitude = if single.sign() == Ordering::Less {
            single.negated()
        } else {
            single.clone()
        };
        return Ok([magnitude.clone(), magnitude]);
    }
    let squared = a.multiply(a).add(&b.multiply(b));
    let bounds = outward_sqrt_bounds(&squared).ok_or(CylindricalGeneratorError::Unrepresentable)?;
    if bounds[0] == 0. {
        return Err(CylindricalGeneratorError::Unrepresentable);
    }
    Ok(bounds.map(ExactRatio::from_finite))
}

fn single_nonzero<'a>(a: &'a ExactRatio, b: &'a ExactRatio) -> Option<&'a ExactRatio> {
    match (a.sign(), b.sign()) {
        (Ordering::Equal, Ordering::Less | Ordering::Greater) => Some(b),
        (Ordering::Less | Ordering::Greater, Ordering::Equal) => Some(a),
        _ => None,
    }
}

fn divide_by_magnitude(value: &ExactRatio, [lo, hi]: &ExactInterval) -> ExactInterval {
    let mut bounds = [value.divide(hi), value.divide(lo)];
    if value.sign() == Ordering::Less {
        bounds.swap(0, 1);
    }
    bounds
}

fn squared_endpoint_bound(
    offset: &ExactVector,
    slope: &ExactVector,
    radial: &[ExactInterval; 3],
    axial: &ExactRatio,
) -> ExactRatio {
    (0..3).fold(ExactRatio::from_finite(0.), |sum, i| {
        let displacement = offset[i].add(&slope[i].multiply(axial));
        let [lo, hi] = &radial[i];
        let left = displacement.subtract(lo);
        let right = displacement.subtract(hi);
        sum.add(&left.multiply(&left).max(right.multiply(&right)))
    })
}

#[cfg(test)]
mod tests;
