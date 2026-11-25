// SPDX-License-Identifier: GPL-2.0-only
//! An extreme ray witnesses a sector narrower than pi; zero vectors are omitted by callers.
use super::*;
pub(super) fn certify(
    vectors: Vec<Vector>,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<Option<SectorWitness>> {
    for first in 0..vectors.len() {
        if let Some(signs) = candidate(&vectors, first, cancelled)? {
            return Ok(Some(SectorWitness {
                vectors,
                first,
                signs,
            }));
        }
    }
    Ok(None)
}
fn candidate(
    vectors: &[Vector],
    first: usize,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<Option<Vec<OrientationSign>>> {
    let mut signs = Vec::with_capacity(vectors.len());
    for vector in vectors {
        poll(cancelled)?;
        let sign = predicates::cross(&vectors[first], vector)?;
        if sign == OrientationSign::Negative
            || sign == OrientationSign::Zero && !predicates::same_direction(&vectors[first], vector)
        {
            return Ok(None);
        }
        signs.push(sign);
    }
    Ok(Some(signs))
}
pub(super) fn hodograph(controls: &[Point]) -> Vec<Vector> {
    controls
        .windows(2)
        .filter(|p| p[0] != p[1])
        .map(|p| Vector {
            from: p[0],
            to: p[1],
        })
        .collect()
}
pub(super) fn endpoint_vectors(a: &[Point], b: &[Point], point: Point) -> Vec<Vector> {
    let mut vectors: Vec<_> = a
        .iter()
        .filter(|p| **p != point)
        .map(|p| Vector {
            from: point,
            to: *p,
        })
        .collect();
    vectors.extend(b.iter().filter(|p| **p != point).map(|p| Vector {
        from: *p,
        to: point,
    }));
    vectors
}
