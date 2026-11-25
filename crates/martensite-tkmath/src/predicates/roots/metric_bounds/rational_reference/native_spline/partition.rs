// SPDX-License-Identifier: GPL-2.0-only
//! Shared knot insertion schedule for literal replay and outward fixed-grid proofs.
use super::NativeSplineFailure;

pub(super) struct Span<P> {
    pub interval: [f64; 2],
    pub poles: Vec<P>,
}

pub(super) fn check_knots(
    degree: usize,
    count: usize,
    knots: &[f64],
) -> Result<(), NativeSplineFailure> {
    let valid_count = (1..=16).contains(&degree)
        && count > degree
        && count.checked_add(degree + 1) == Some(knots.len());
    if valid_count && valid_clamping(degree, knots) {
        return Ok(());
    }
    Err(NativeSplineFailure::Knots {
        degree,
        poles: count,
        knots: knots.len(),
    })
}

fn valid_clamping(p: usize, knots: &[f64]) -> bool {
    let n = knots.len();
    let first = &knots[..=p];
    let last = &knots[n - p - 1..];
    let repeats = |group: &[f64]| group.iter().all(|&knot| knot == group[0]);
    knots.iter().all(|knot| knot.is_finite())
        && knots.windows(2).all(|pair| pair[0] <= pair[1])
        && repeats(first)
        && repeats(last)
        && knots[p] < knots[p + 1]
        && knots[n - p - 2] < knots[n - p - 1]
        && !knots[p + 1..n - p - 1].windows(p + 1).any(repeats)
}

/// NURBS Book A5.6. The two arithmetic paths share the knots, insertion order and
/// single computed junction. Preserve the literal multiplication order in replay.
pub(super) fn decompose<P: Clone, A, E>(
    p: usize,
    knots: &[f64],
    poles: &[P],
    mut arithmetic: impl Insertion<P, A, E>,
) -> Result<Vec<Span<P>>, E> {
    let m = knots.len() - 1;
    let (mut a, mut b) = (p, p + 1);
    let mut piece = poles[..=p].to_vec();
    let mut spans = Vec::new();
    while b < m {
        let first = b;
        while b < m && knots[b + 1] == knots[b] {
            b += 1;
        }
        let multiplicity = b - first + 1;
        let mut next = vec![piece[p].clone(); p + 1];
        if b < m {
            let first = p.saturating_sub(multiplicity);
            next[first..].clone_from_slice(&poles[b - p + first..=b]);
        }
        refine(
            [a, b],
            multiplicity,
            knots,
            &mut piece,
            &mut next,
            &mut arithmetic,
        )?;
        next[0] = piece[p].clone();
        spans.push(Span {
            interval: [knots[a], knots[b]],
            poles: piece,
        });
        piece = next;
        (a, b) = (b, b + 1);
    }
    Ok(spans)
}

pub(super) trait Insertion<P, A, E> {
    fn ratio(&mut self, first: f64, upper: f64, knot: f64) -> Result<A, E>;
    fn blend(&mut self, alpha: &A, a: &P, b: &P) -> Result<P, E>;
}

fn refine<P: Clone, A, E>(
    [a, b]: [usize; 2],
    multiplicity: usize,
    knots: &[f64],
    piece: &mut [P],
    next: &mut [P],
    arithmetic: &mut impl Insertion<P, A, E>,
) -> Result<(), E> {
    let p = piece.len() - 1;
    if multiplicity >= p {
        return Ok(());
    }
    let alphas = (multiplicity + 1..=p)
        .map(|j| arithmetic.ratio(knots[a], knots[a + j], knots[b]))
        .collect::<Result<Vec<_>, _>>()?;
    let r = p - multiplicity;
    for j in 1..=r {
        let s = multiplicity + j;
        for k in (s..=p).rev() {
            piece[k] = arithmetic.blend(&alphas[k - s], &piece[k], &piece[k - 1])?;
        }
        next[r - j] = piece[p].clone();
    }
    Ok(())
}

pub(super) struct Literal;

impl Insertion<[f64; 3], f64, std::convert::Infallible> for Literal {
    fn ratio(
        &mut self,
        first: f64,
        upper: f64,
        knot: f64,
    ) -> Result<f64, std::convert::Infallible> {
        Ok((knot - first) / (upper - first))
    }

    fn blend(
        &mut self,
        alpha: &f64,
        a: &[f64; 3],
        b: &[f64; 3],
    ) -> Result<[f64; 3], std::convert::Infallible> {
        Ok(std::array::from_fn(|axis| {
            alpha * a[axis] + (1. - alpha) * b[axis]
        }))
    }
}
