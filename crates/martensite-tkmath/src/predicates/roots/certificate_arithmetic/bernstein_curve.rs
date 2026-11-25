// SPDX-License-Identifier: GPL-2.0-only
//! Admitted nonempty Bernstein curves: fixed-grid capture, restriction and derivatives.
use super::bounded_dyadic::{Arithmetic, Interval};
use super::bounded_failure::Checked;

pub(in crate::predicates::roots) fn capture_planar(
    authored: &[[f64; 2]],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Vec<Interval>; 2]> {
    let mut source: [Vec<Interval>; 2] = std::array::from_fn(|_| Vec::new());
    for pole in authored {
        source[0].push(arithmetic.scalar(pole[0])?);
        source[1].push(arithmetic.scalar(pole[1])?);
    }
    for axis in 0..2 {
        if authored.iter().all(|pole| pole[axis] == authored[0][axis]) {
            source[axis].truncate(1);
        }
    }
    Ok(source)
}

pub(in crate::predicates::roots) fn restrict(
    poles: &[Interval],
    index: u64,
    level: u8,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<Interval>> {
    let mut result = poles.to_vec();
    for bit in (0..level).rev() {
        let [left, right] = bisect(&result, arithmetic)?;
        result = if index >> bit & 1 == 0 { left } else { right };
    }
    Ok(result)
}

pub(in crate::predicates::roots) fn restrict_range(
    poles: &[Interval],
    [first, last]: [f64; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<Interval>> {
    if poles.len() == 1 || [first, last] == [0., 1.] {
        return Ok(poles.to_vec());
    }
    let first = arithmetic.scalar(first)?;
    let last = arithmetic.scalar(last)?;
    restrict_enclosed(poles, [&first, &last], arithmetic)
}

pub(in crate::predicates::roots) fn restrict_enclosed(
    poles: &[Interval],
    [first, last]: [&Interval; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<Interval>> {
    if poles.len() == 1 {
        return Ok(poles.to_vec());
    }
    let one = arithmetic.scalar(1.)?;
    let low = [arithmetic.subtract(&one, first)?, first.clone()];
    let high = [arithmetic.subtract(&one, last)?, last.clone()];
    let degree = poles.len() - 1;
    (0..=degree)
        .map(|i| blossom(poles, [&low, &high], i, arithmetic))
        .collect()
}

fn blossom(
    poles: &[Interval],
    [low, high]: [&[Interval; 2]; 2],
    high_args: usize,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let mut work = poles.to_vec();
    for count in (1..poles.len()).rev() {
        let [a, b] = if count <= high_args { high } else { low };
        for i in 0..count {
            let left = arithmetic.multiply(a, &work[i])?;
            let right = arithmetic.multiply(b, &work[i + 1])?;
            work[i] = arithmetic.add(&left, &right)?;
        }
    }
    Ok(work.remove(0))
}

pub(in crate::predicates::roots) fn bisect(
    poles: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Vec<Interval>; 2]> {
    let mut work = poles.to_vec();
    let mut left = vec![work[0].clone()];
    let mut right = vec![work[work.len() - 1].clone()];
    for count in (1..work.len()).rev() {
        for i in 0..count {
            let sum = arithmetic.add(&work[i], &work[i + 1])?;
            work[i] = arithmetic.fraction(&sum, 1, 2)?;
        }
        left.push(work[0].clone());
        right.push(work[count - 1].clone());
    }
    right.reverse();
    Ok([left, right])
}

pub(in crate::predicates::roots) fn derivative(
    poles: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<Interval>> {
    if poles.len() == 1 {
        return Ok(vec![arithmetic.zero()]);
    }
    poles
        .windows(2)
        .map(|pair| {
            let delta = arithmetic.subtract(&pair[1], &pair[0])?;
            arithmetic.fraction(&delta, (poles.len() - 1) as u64, 1)
        })
        .collect()
}

// An exactly constant coordinate may have one coefficient; the other retains the source degree.
pub(in crate::predicates::roots) fn align_planar(
    x: &[Interval],
    y: &[Interval],
) -> Vec<[Interval; 2]> {
    (0..x.len().max(y.len()))
        .map(|i| [coefficient(x, i), coefficient(y, i)])
        .collect()
}

fn coefficient(axis: &[Interval], index: usize) -> Interval {
    axis[if axis.len() == 1 { 0 } else { index }].clone()
}

#[cfg(test)]
mod tests;
