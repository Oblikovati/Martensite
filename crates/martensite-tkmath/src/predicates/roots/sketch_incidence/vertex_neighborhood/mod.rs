// SPDX-License-Identifier: GPL-2.0-only
//! Complete proper components of an analytic curve inside a given vertex ball.
//! This operation preserves declared identity and trims. It does not infer adjacency
//! from proximity, repair geometry, or certify a face or an exact embedded wire.
mod capture;
mod isolation;
#[cfg(test)]
mod tests;
mod types;
use isolation::Work;
use std::cmp::Ordering;
pub use types::*;

/// Prove precisely the endpoint components declared by the caller's incidence.
/// Example: `certify_given_vertex_neighborhood(input, VertexNeighborhoodPolicy::v1(10000), || false)`.
/// Complete algebraic sphere contacts are retained; exhausted requests return no certificate.
pub fn certify_given_vertex_neighborhood(
    input: GivenVertexNeighborhoodInput,
    policy: VertexNeighborhoodPolicy,
    mut cancelled: impl FnMut() -> bool,
) -> GivenVertexNeighborhoodReport {
    let mut work = Work {
        used: 0,
        policy,
        cancelled: &mut cancelled,
    };
    let outcome = execute(&input, &mut work);
    GivenVertexNeighborhoodReport {
        input,
        policy,
        work: work.used,
        outcome,
    }
}
fn execute(
    input: &GivenVertexNeighborhoodInput,
    work: &mut Work<'_>,
) -> Result<GivenVertexNeighborhoodCertificate, VertexNeighborhoodFailure> {
    if work.policy.version != 1 {
        return Err(VertexNeighborhoodFailure::Version(work.policy.version));
    }
    work.charge()?;
    let pieces = capture::pieces(input, work)?;
    let center = capture::center(input);
    let (crossings, endpoint_signs) = all_pieces(&pieces, &center, input.radius, work)?;
    proper(input.ends, &crossings, endpoint_signs)?;
    Ok(GivenVertexNeighborhoodCertificate {
        input: input.clone(),
        crossings,
        endpoint_signs,
    })
}

fn all_pieces(
    pieces: &[capture::CurvePiece],
    center: &[super::super::certificate_arithmetic::ExactRatio; 3],
    radius: f64,
    work: &mut Work<'_>,
) -> Result<(Vec<VertexNeighborhoodCrossing>, [Ordering; 2]), VertexNeighborhoodFailure> {
    let mut crossings = Vec::new();
    let mut endpoints = [Ordering::Equal; 2];
    for (index, piece) in pieces.iter().enumerate() {
        let equation = isolation::threshold(piece, center, radius);
        update_endpoints(&mut endpoints, &equation, piece.range, index);
        isolation::append(
            &mut crossings,
            isolation::isolate(&equation, piece.range, index, work)?,
        );
    }
    Ok((crossings, endpoints))
}
fn proper(
    ends: VertexNeighborhoodEnds,
    roots: &[VertexNeighborhoodCrossing],
    endpoints: [Ordering; 2],
) -> Result<(), VertexNeighborhoodFailure> {
    let expected = expected_components(ends);
    if roots.iter().any(|root| !root.simple)
        || endpoints != expected.1
        || roots.iter().map(|r| r.sides).collect::<Vec<_>>() != expected.0
    {
        return Err(VertexNeighborhoodFailure::Components {
            crossings: roots.to_vec(),
            endpoint_signs: endpoints,
        });
    }
    Ok(())
}

fn update_endpoints(
    endpoints: &mut [Ordering; 2],
    equation: &super::rational_polynomial::RationalPolynomial,
    range: [f64; 2],
    index: usize,
) {
    use super::rational_polynomial::ratio;
    if index == 0 {
        endpoints[0] = equation.evaluate(&ratio(range[0])).sign();
    }
    endpoints[1] = equation.evaluate(&ratio(range[1])).sign();
}
fn expected_components(
    ends: VertexNeighborhoodEnds,
) -> (Vec<[Option<Ordering>; 2]>, [Ordering; 2]) {
    use Ordering::{Greater, Less};
    match ends {
        VertexNeighborhoodEnds::First => (vec![[Some(Less), Some(Greater)]], [Less, Greater]),
        VertexNeighborhoodEnds::Last => (vec![[Some(Greater), Some(Less)]], [Greater, Less]),
        VertexNeighborhoodEnds::Both => (
            vec![[Some(Less), Some(Greater)], [Some(Greater), Some(Less)]],
            [Less, Less],
        ),
    }
}
