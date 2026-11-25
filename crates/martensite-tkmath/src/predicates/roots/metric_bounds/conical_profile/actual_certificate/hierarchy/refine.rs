// SPDX-License-Identifier: GPL-2.0-only
//! Prove a complete leaf product with outward rational subpatch hulls.
use super::*;
use std::sync::Arc;
mod patch;
use patch::Patch;
#[cfg(test)]
mod tests;

// Count product-tree levels, each bisecting one of the four parameters.
const MAX_PRODUCT_DEPTH: u8 = 24;

pub(super) fn separated(
    sides: [&SideCapture; 2],
    visits: &mut u64,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<bool> {
    let first = Patch::capture(&sides[0].coordinates, &sides[0].source.weights, arithmetic)?;
    let last = Patch::capture(&sides[1].coordinates, &sides[1].source.weights, arithmetic)?;
    complete_product([first, last], visits, arithmetic)
}

fn complete_product(
    patches: [Patch; 2],
    visits: &mut u64,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<bool> {
    let mut pending = vec![(patches.map(Arc::new), 0)];
    while let Some((pair, level)) = pending.pop() {
        arithmetic.certificate_step()?;
        *visits += 1;
        if visit::separated(&pair[0].hull, &pair[1].hull) {
            continue;
        }
        if level == MAX_PRODUCT_DEPTH {
            return Ok(false);
        }
        push_children(&pair, level, &mut pending, arithmetic)?;
    }
    Ok(true)
}

fn push_children(
    pair: &[Arc<Patch>; 2],
    level: u8,
    pending: &mut Vec<([Arc<Patch>; 2], u8)>,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<()> {
    let side = usize::from(level % 2);
    let children = pair[side].bisect(level % 4 < 2, arithmetic)?;
    for child in children.into_iter().rev() {
        let mut next = pair.clone();
        next[side] = Arc::new(child);
        pending.push((next, level + 1));
    }
    Ok(())
}
