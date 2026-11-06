// SPDX-License-Identifier: GPL-2.0-only
use super::super::equation::side_signs;
use super::super::rational_polynomial::{RationalPolynomial as Polynomial, ratio};
use super::{VertexNeighborhoodCrossing as Crossing, VertexNeighborhoodFailure as Failure};
use super::{VertexNeighborhoodPolicy as Policy, capture::CurvePiece};
use crate::predicates::roots::certificate_arithmetic::ExactRatio;
use crate::predicates::roots::{DyadicBound, RootIsolationOptions, polynomial_solve};
use num_bigint::Sign;
use std::cmp::Ordering;

pub(super) struct Work<'a> {
    pub used: u32,
    pub policy: Policy,
    pub cancelled: &'a mut dyn FnMut() -> bool,
}
impl Work<'_> {
    pub fn charge(&mut self) -> Result<(), Failure> {
        if (self.cancelled)() {
            return Err(Failure::Cancelled);
        }
        if self.used >= self.policy.max_work {
            return Err(Failure::WorkLimit {
                limit: self.policy.max_work,
            });
        }
        self.used += 1;
        Ok(())
    }
    pub fn reserve(&mut self, count: u64) -> Result<(), Failure> {
        if (self.cancelled)() {
            return Err(Failure::Cancelled);
        }
        if count > u64::from(self.policy.max_work.saturating_sub(self.used)) {
            return Err(Failure::WorkLimit {
                limit: self.policy.max_work,
            });
        }
        self.used += count as u32;
        Ok(())
    }
}
pub(super) fn threshold(piece: &CurvePiece, center: &[ExactRatio; 3], radius: f64) -> Polynomial {
    let square = ratio(radius).multiply(&ratio(radius));
    piece.coordinates.iter().zip(center).fold(
        Polynomial::constant(square.negated()),
        |p, (c, v)| {
            let relative = c.add(&Polynomial::constant(v.negated()));
            p.add(&relative.multiply(&relative))
        },
    )
}
pub(super) fn isolate(
    equation: &Polynomial,
    range: [f64; 2],
    span: usize,
    work: &mut Work<'_>,
) -> Result<Vec<Crossing>, Failure> {
    work.charge()?;
    let polynomial = equation
        .to_integer(4)
        .map_err(Failure::Roots)?
        .ok_or(Failure::BoundaryOverlap { span })?;
    let bounds = [bound(range[0])?, bound(range[1])?];
    let roots = solve(&polynomial, &bounds, work)?;
    Ok(roots
        .roots()
        .iter()
        .map(|root| crossing(span, root.clone(), &polynomial, &bounds))
        .collect())
}
fn crossing(
    span: usize,
    parameter: crate::predicates::roots::RootBracket,
    polynomial: &crate::predicates::roots::polynomial::IntegerPolynomial,
    bounds: &[DyadicBound; 2],
) -> Crossing {
    let sides = side_signs(polynomial, &parameter).map(|s| Some(order(s)));
    let simple = parameter.multiplicity() == 1;
    let mut result = Crossing {
        span,
        parameter,
        sides,
        simple,
    };
    clip_endpoint_sides(&mut result, bounds);
    result
}
pub(super) fn order(sign: Sign) -> Ordering {
    match sign {
        Sign::Minus => Ordering::Less,
        Sign::NoSign => Ordering::Equal,
        Sign::Plus => Ordering::Greater,
    }
}
pub(super) fn append(target: &mut Vec<Crossing>, mut roots: Vec<Crossing>) {
    if let (Some(previous), Some(next)) = (target.last_mut(), roots.first())
        && same_knot(previous, next)
    {
        previous.sides[1] = next.sides[1];
        previous.simple &= next.simple;
        roots.remove(0);
    }
    target.extend(roots);
}
fn same_knot(previous: &Crossing, next: &Crossing) -> bool {
    let (a, b) = previous.parameter.bounds();
    let (c, d) = next.parameter.bounds();
    a == b && b == c && c == d
}

fn bound(value: f64) -> Result<DyadicBound, Failure> {
    DyadicBound::from_f64(value).map_err(|_| Failure::Input {
        field: "finite trim",
        value,
    })
}

pub(super) fn solve(
    polynomial: &crate::predicates::roots::polynomial::IntegerPolynomial,
    bounds: &[DyadicBound; 2],
    work: &mut Work<'_>,
) -> Result<crate::predicates::roots::RootIsolation, Failure> {
    let options = isolation_options(work);
    let mut splits = 0_u32;
    let result = polynomial_solve::solve_polynomial_accounted(
        polynomial.clone(),
        bounds.clone(),
        bounds[1].difference(&bounds[0]),
        options,
        &mut work.cancelled,
        &mut || splits += 1,
    );
    work.used += splits;
    result.map_err(Failure::Roots)
}
fn clip_endpoint_sides(result: &mut Crossing, bounds: &[DyadicBound; 2]) {
    let (lower, upper) = result.parameter.bounds();
    if lower != upper {
        return;
    }
    if lower == &bounds[0] {
        result.sides[0] = None;
    }
    if upper == &bounds[1] {
        result.sides[1] = None;
    }
}

fn isolation_options(work: &Work<'_>) -> RootIsolationOptions {
    let remaining = work
        .policy
        .max_work
        .saturating_sub(work.used)
        .min(u16::MAX.into()) as u16;
    RootIsolationOptions {
        max_bisections: remaining,
        max_degree: 4,
        ..RootIsolationOptions::new(1.)
    }
}
