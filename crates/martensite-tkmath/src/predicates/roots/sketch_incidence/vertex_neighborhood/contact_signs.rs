// SPDX-License-Identifier: GPL-2.0-only
use super::super::rational_polynomial::RationalPolynomial as Polynomial;
use super::super::{SketchIncidenceOptions, root_sign};
use super::super::{
    algebraic::{IsolatedRoot, RootEquation},
    budget::Work as SignWork,
};
use super::contact_equations::LineFrame;
use super::contact_types::{GivenVertexContact, GivenVertexContactFailure as Failure};
use super::{capture::CurvePiece, isolation::Work};
use crate::predicates::roots::{RootBracket, polynomial::IntegerPolynomial};
use std::cmp::Ordering;

pub(super) fn classify(
    polynomial: &IntegerPolynomial,
    root: RootBracket,
    queries: &[Polynomial; 3],
    span: usize,
    line: &LineFrame,
    work: &mut Work<'_>,
) -> Result<Option<GivenVertexContact>, Failure> {
    work.charge().map_err(Failure::Exact)?;
    let signs = signs(polynomial, &root, queries, work)?;
    let line_bounds = parameter_signs([signs[0], signs[1]], line);
    if line_bounds[0] == Ordering::Less || line_bounds[1] == Ordering::Greater {
        return Ok(None);
    }
    accepted_contact(span, root, line_bounds, signs[2]).map(Some)
}
fn accepted_contact(
    span: usize,
    parameter: RootBracket,
    line_bounds: [Ordering; 2],
    neighborhood_sign: Ordering,
) -> Result<GivenVertexContact, Failure> {
    let contact = GivenVertexContact {
        span,
        parameter,
        line_bounds,
        neighborhood_sign,
    };
    if neighborhood_sign != Ordering::Less {
        return Err(Failure::NonIncident { contact });
    }
    Ok(contact)
}

fn signs(
    polynomial: &IntegerPolynomial,
    root: &RootBracket,
    queries: &[Polynomial; 3],
    work: &mut Work<'_>,
) -> Result<[Ordering; 3], Failure> {
    let remaining = work.policy.max_work.saturating_sub(work.used);
    let mut options = SketchIncidenceOptions::v1(1.);
    options.max_refinements = remaining;
    let mut account = SignWork::new(options, &mut *work.cancelled);
    let result = root_signs(polynomial, root, queries, &mut account);
    work.used += account.used();
    result.map_err(Failure::Classification)
}
fn root_signs(
    polynomial: &IntegerPolynomial,
    root: &RootBracket,
    queries: &[Polynomial; 3],
    work: &mut SignWork<'_>,
) -> Result<[Ordering; 3], super::super::SketchIncidenceCause> {
    let equation = RootEquation::new(polynomial.clone(), work)?;
    let mut isolated = IsolatedRoot::from_bracket(&equation, root);
    let mut result = [Ordering::Equal; 3];
    for (index, query) in queries.iter().enumerate() {
        work.charge()?;
        result[index] =
            super::isolation::order(root_sign::sign_at_root(&mut isolated, query, work)?);
    }
    Ok(result)
}
fn parameter_signs(mut signs: [Ordering; 2], line: &LineFrame) -> [Ordering; 2] {
    if line.direction[line.axis].sign() == Ordering::Less {
        signs = signs.map(Ordering::reverse);
    }
    signs
}
pub(super) fn queries(piece: &CurvePiece, line: &LineFrame, ball: Polynomial) -> [Polynomial; 3] {
    let [first, last] = line.parameter_bounds(piece);
    [first, last, ball]
}
