// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) fn certify(
    curves: [&[Point]; 2],
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<Option<GeometricProof>> {
    for side in 0..2 {
        if let Some(proof) = side_edges(curves, side, cancelled)? {
            return Ok(Some(proof));
        }
    }
    Ok(None)
}
fn edges(
    curves: [&[Point]; 2],
    side: usize,
    i: usize,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<Option<GeometricProof>> {
    for j in i + 1..curves[side].len() {
        let edge = [curves[side][i], curves[side][j]];
        if edge[0] == edge[1] {
            continue;
        }
        let signs = [
            signs(curves[side], edge, cancelled)?,
            signs(curves[1 - side], edge, cancelled)?,
        ];
        if separated(&signs) {
            return Ok(Some(GeometricProof::Hull(HullWitness {
                edge,
                side,
                signs,
            })));
        }
    }
    Ok(None)
}
fn signs(
    points: &[Point],
    edge: [Point; 2],
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<Vec<OrientationSign>> {
    let vector = Vector {
        from: edge[0],
        to: edge[1],
    };
    points
        .iter()
        .map(|p| {
            poll(cancelled)?;
            predicates::cross(
                &vector,
                &Vector {
                    from: edge[0],
                    to: *p,
                },
            )
        })
        .collect()
}
fn separated(signs: &[Vec<OrientationSign>; 2]) -> bool {
    let forward = signs[0].iter().all(|s| *s != OrientationSign::Negative)
        && signs[1].iter().all(|s| *s == OrientationSign::Negative);
    let reverse = signs[0].iter().all(|s| *s != OrientationSign::Positive)
        && signs[1].iter().all(|s| *s == OrientationSign::Positive);
    forward || reverse
}

fn side_edges(
    curves: [&[Point]; 2],
    side: usize,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<Option<GeometricProof>> {
    for i in 0..curves[side].len() {
        if let Some(proof) = edges(curves, side, i, cancelled)? {
            return Ok(Some(proof));
        }
    }
    Ok(None)
}
