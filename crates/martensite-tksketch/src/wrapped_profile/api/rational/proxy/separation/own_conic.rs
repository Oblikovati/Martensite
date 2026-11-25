// SPDX-License-Identifier: GPL-2.0-only
//! Certify the simultaneous homotopy across a sewn conic's native span joins.
use super::*;

pub(super) fn prove((curve, cover): Paired<'_, '_>) -> Checked<()> {
    if !curve.has_sewn_conic_partition() {
        return Ok(());
    }
    let center = curve.spans[0]
        .sewn_center()
        .ok_or_else(|| failure(cover.curve))?;
    let pieces = groups(curve, cover)?;
    let closed = curve.ends.is_none();
    for (index, left) in pieces.iter().enumerate() {
        for (later, right) in pieces.iter().enumerate().skip(index + 1) {
            if closed && index == 0 && later + 1 == pieces.len() {
                adjacent(right, left, center, cover.curve)?;
            } else if later == index + 1 {
                adjacent(left, right, center, cover.curve)?;
            } else {
                separate(left, right, center, cover.curve)?;
            }
        }
    }
    Ok(())
}

struct Piece<'a, 'source> {
    original: &'a input::NativeSpan<'source>,
    reference: &'a [fit::FittedSpan],
}

fn groups<'a, 'source>(
    curve: &'a input::Curve<'source>,
    cover: &'a fit::Cover,
) -> Checked<Vec<Piece<'a, 'source>>> {
    let count = curve.spans.len();
    let length = cover.spans.len() / count;
    if length == 0 || length * count != cover.spans.len() {
        return Err(failure(cover.curve));
    }
    curve
        .spans
        .iter()
        .zip(cover.spans.chunks_exact(length))
        .map(|(original, reference)| {
            if reference.iter().all(|span| span.map == original.map) {
                Ok(Piece {
                    original,
                    reference,
                })
            } else {
                Err(failure(cover.curve))
            }
        })
        .collect()
}

impl Piece<'_, '_> {
    fn ends(&self) -> [[f64; 2]; 2] {
        let poles = self.original.metric().poles;
        [poles[0], poles[poles.len() - 1]]
    }

    fn halfplane(&self, plane: &Halfplane, shared: [bool; 2]) -> Checked<()> {
        plane.controls(self.original.metric().poles, None, shared)?;
        for (index, part) in self.reference.iter().enumerate() {
            let shared = [
                shared[0] && index == 0,
                shared[1] && index + 1 == self.reference.len(),
            ];
            plane.controls(&part.span.poles, None, shared)?;
        }
        Ok(())
    }
}

fn adjacent(
    left: &Piece<'_, '_>,
    right: &Piece<'_, '_>,
    center: [f64; 2],
    id: ProfileCurveId,
) -> Checked<()> {
    let [outer, joint] = left.ends();
    if right.ends()[0] != joint {
        return Err(failure(id));
    }
    let cut = [center, joint];
    let sign = orientation(cut, outer, id)?;
    left.halfplane(
        &Halfplane {
            cut,
            sign,
            curve: id,
        },
        [false, true],
    )?;
    right.halfplane(
        &Halfplane {
            cut,
            sign: opposite(sign),
            curve: id,
        },
        [true, false],
    )
}

fn separate(
    left: &Piece<'_, '_>,
    right: &Piece<'_, '_>,
    center: [f64; 2],
    id: ProfileCurveId,
) -> Checked<()> {
    let [start, end] = left.ends();
    let [x, y] =
        std::array::from_fn(|axis| (start[axis] - center[axis]) + (end[axis] - center[axis]));
    let cut = [center, [center[0] - y, center[1] + x]];
    let sign = orientation(cut, start, id)?;
    left.halfplane(
        &Halfplane {
            cut,
            sign,
            curve: id,
        },
        [false, false],
    )?;
    right.halfplane(
        &Halfplane {
            cut,
            sign: opposite(sign),
            curve: id,
        },
        [false, false],
    )
}

fn orientation(
    cut: [[f64; 2]; 2],
    point: [f64; 2],
    id: ProfileCurveId,
) -> Checked<OrientationSign> {
    orient2d(cut[0], cut[1], point)
        .ok()
        .filter(|sign| *sign != OrientationSign::Zero)
        .ok_or_else(|| failure(id))
}

fn failure(curve: ProfileCurveId) -> RationalWrappedFailure {
    invalid(
        format!("authored conic={curve:?}"),
        "an exact separated conic homotopy across native span joins",
    )
}
