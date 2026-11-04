// SPDX-License-Identifier: GPL-2.0-only
use super::contact_equations::{self, LineFrame};
use super::contact_types::{GivenVertexContact as Contact, GivenVertexContactFailure as Failure};
use super::isolation::Work;
use super::*;

/// Classify every actual source-curve/target-line contact inside their complete trims.
/// Both neighborhoods must bind the same given vertex and radius. Complete empty
/// contacts are permitted; no proximity-based identity or exact embedding is inferred.
/// Example: `certify_given_vertex_contacts(&source, &line, policy, || false)`.
pub fn certify_given_vertex_contacts(
    source: &GivenVertexNeighborhoodReport,
    target: &GivenVertexNeighborhoodReport,
    policy: VertexNeighborhoodPolicy,
    mut cancelled: impl FnMut() -> bool,
) -> GivenVertexContactsReport {
    let mut work = Work {
        used: 0,
        policy,
        cancelled: &mut cancelled,
    };
    let outcome = execute(source, target, &mut work);
    GivenVertexContactsReport {
        source: source.clone(),
        target: target.clone(),
        policy,
        work: work.used,
        outcome,
    }
}
fn execute(
    source: &GivenVertexNeighborhoodReport,
    target: &GivenVertexNeighborhoodReport,
    work: &mut Work<'_>,
) -> Result<Vec<Contact>, Failure> {
    if work.policy.version != 1 {
        return Err(Failure::Exact(VertexNeighborhoodFailure::Version(
            work.policy.version,
        )));
    }
    binding(source, target)?;
    work.reserve(u64::from(source.work()) + u64::from(target.work()))
        .map_err(Failure::Exact)?;
    classify_pieces(source.input(), target.input(), work)
}
fn classify_pieces(
    source: &GivenVertexNeighborhoodInput,
    target: &GivenVertexNeighborhoodInput,
    work: &mut Work<'_>,
) -> Result<Vec<Contact>, Failure> {
    let pieces = capture::pieces(source, work).map_err(Failure::Exact)?;
    let target_pieces = capture::pieces(target, work).map_err(Failure::Exact)?;
    let line = LineFrame::new(&target_pieces[0])?;
    let center = capture::center(source);
    let mut contacts = Vec::new();
    for (index, piece) in pieces.iter().enumerate() {
        let ball = isolation::threshold(piece, &center, source.radius);
        append(
            &mut contacts,
            span_contacts(piece, &line, ball, index, work)?,
        );
    }
    Ok(contacts)
}
fn binding(
    source: &GivenVertexNeighborhoodReport,
    target: &GivenVertexNeighborhoodReport,
) -> Result<(), Failure> {
    for (operand, report) in [source, target].into_iter().enumerate() {
        if let Some(cause) = report.failure() {
            return Err(Failure::PriorIncomplete {
                operand,
                cause: cause.clone(),
            });
        }
    }
    same_vertex(source.input(), target.input())
}
fn same_vertex(
    a: &GivenVertexNeighborhoodInput,
    b: &GivenVertexNeighborhoodInput,
) -> Result<(), Failure> {
    let bindings = [
        (a.vertex == b.vertex, "given vertex identity"),
        (
            a.point == b.point && a.point_placements == b.point_placements,
            "given vertex representation",
        ),
        (a.radius == b.radius, "raw minimum radius"),
    ];
    for (agrees, field) in bindings {
        if !agrees {
            return Err(Failure::Binding { field });
        }
    }
    if !matches!(b.curve, VertexNeighborhoodCurve::Line { .. }) {
        return Err(Failure::TargetNotLine);
    }
    Ok(())
}

fn span_contacts(
    piece: &capture::CurvePiece,
    line: &LineFrame,
    ball: super::super::rational_polynomial::RationalPolynomial,
    index: usize,
    work: &mut Work<'_>,
) -> Result<Vec<Contact>, Failure> {
    let Some(equation) = contact_equations::equation(piece, line, index, work)? else {
        return Ok(Vec::new());
    };
    let bounds = exact_bounds(piece.range)?;
    let roots = isolation::solve(&equation, &bounds, work).map_err(Failure::Exact)?;
    let queries = contact_signs::queries(piece, line, ball);
    classify_roots(&equation, &roots, &queries, index, line, work)
}
fn classify_roots(
    equation: &crate::predicates::roots::polynomial::IntegerPolynomial,
    roots: &crate::predicates::roots::RootIsolation,
    queries: &[super::super::rational_polynomial::RationalPolynomial; 3],
    index: usize,
    line: &LineFrame,
    work: &mut Work<'_>,
) -> Result<Vec<Contact>, Failure> {
    let mut contacts = Vec::new();
    for root in roots.roots() {
        if let Some(contact) =
            contact_signs::classify(equation, root.clone(), queries, index, line, work)?
        {
            contacts.push(contact);
        }
    }
    Ok(contacts)
}

fn append(contacts: &mut Vec<Contact>, mut next: Vec<Contact>) {
    if let (Some(previous), Some(first)) = (contacts.last(), next.first()) {
        let (a, b) = previous.parameter.bounds();
        let (c, d) = first.parameter.bounds();
        if a == b && b == c && c == d && previous.span + 1 == first.span {
            next.remove(0);
        }
    }
    contacts.extend(next);
}

fn exact_bounds(range: [f64; 2]) -> Result<[crate::predicates::roots::DyadicBound; 2], Failure> {
    let [first, last] = range.map(crate::predicates::roots::DyadicBound::from_f64);
    Ok([
        first.map_err(|_| Failure::Binding {
            field: "finite first trim",
        })?,
        last.map_err(|_| Failure::Binding {
            field: "finite last trim",
        })?,
    ])
}
