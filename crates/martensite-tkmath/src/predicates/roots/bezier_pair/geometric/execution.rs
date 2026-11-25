// SPDX-License-Identifier: GPL-2.0-only
use super::*;
mod hull;
pub(super) fn execute(
    query: &CapturedBezierPair,
    policy: BezierPairPolicy,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<Option<GeometricProof>> {
    let supported = admission::admit(query, policy)?;
    poll(cancelled)?;
    if !supported {
        return Ok(None);
    }
    let curves = query.captured_curves();
    let result = if query.is_self() {
        sector::certify(sector::hodograph(curves[0].controls), cancelled)?
            .map(GeometricProof::Monotone)
    } else {
        distinct(curves.map(|c| c.controls), cancelled)?
    };
    poll(cancelled)?;
    Ok(result)
}
fn distinct(
    curves: [&[Point]; 2],
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<Option<GeometricProof>> {
    if let Some(proof) = boxes(curves) {
        return Ok(Some(proof));
    }
    let endpoints = shared(curves);
    if endpoints.len() == 1 {
        return endpoint(curves, endpoints[0], cancelled);
    }
    if !endpoints.is_empty() {
        return Ok(None);
    }
    hull::certify(curves, cancelled)
}
fn boxes(curves: [&[Point]; 2]) -> Option<GeometricProof> {
    for axis in 0..2 {
        let bounds = curves.map(|c| bounds(c, axis));
        if bounds[0][1] < bounds[1][0] || bounds[1][1] < bounds[0][0] {
            return Some(GeometricProof::Box { axis, bounds });
        }
    }
    None
}
fn bounds(points: &[Point], axis: usize) -> [f64; 2] {
    points
        .iter()
        .fold([f64::INFINITY, f64::NEG_INFINITY], |h, p| {
            [h[0].min(p[axis]), h[1].max(p[axis])]
        })
}
fn shared(curves: [&[Point]; 2]) -> Vec<[u8; 2]> {
    let mut matches = Vec::new();
    for a in 0..2 {
        shared_with_first(curves, a, &mut matches);
    }
    matches
}
fn endpoint(
    curves: [&[Point]; 2],
    parameters: [u8; 2],
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<Option<GeometricProof>> {
    let point = curves[0][usize::from(parameters[0]) * (curves[0].len() - 1)];
    let opposite = std::array::from_fn::<_, 2, _>(|i| {
        curves[i][usize::from(1 - parameters[i]) * (curves[i].len() - 1)]
    });
    if opposite.contains(&point) {
        return Ok(None);
    }
    let vectors = sector::endpoint_vectors(curves[0], curves[1], point);
    Ok(
        sector::certify(vectors, cancelled)?.map(|sector| GeometricProof::Endpoint {
            sector,
            contact: Endpoint { parameters, point },
        }),
    )
}

fn shared_with_first(curves: [&[Point]; 2], a: usize, matches: &mut Vec<[u8; 2]>) {
    for b in 0..2 {
        if curves[0][a * (curves[0].len() - 1)] == curves[1][b * (curves[1].len() - 1)] {
            matches.push([a as u8, b as u8]);
        }
    }
}
