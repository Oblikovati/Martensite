// SPDX-License-Identifier: GPL-2.0-only
//! Prove a complete leaf product with outward rational subpatch hulls.
use super::*;
use std::sync::Arc;
mod memo;
mod patch;
pub(super) use memo::Memo;
use memo::PatchUse;
use patch::Patch;
#[cfg(test)]
mod tests;

// Count product-tree levels, each bisecting one of the four parameters.
const MAX_PRODUCT_DEPTH: u8 = 24;
const _: () = assert!(MAX_PRODUCT_DEPTH.div_ceil(2) as u32 * 2 < u32::BITS);

pub(super) fn separated(
    indices: [usize; 2],
    visits: &mut u64,
    arithmetic: &mut Arithmetic<'_>,
    memo: &mut Memo<'_>,
) -> Outcome<bool> {
    let first = memo.capture(indices[0], arithmetic)?;
    let last = memo.capture(indices[1], arithmetic)?;
    complete_product([first, last], visits, arithmetic, memo)
}

fn complete_product(
    patches: [PatchUse; 2],
    visits: &mut u64,
    arithmetic: &mut Arithmetic<'_>,
    memo: &mut Memo<'_>,
) -> Outcome<bool> {
    let mut pending = vec![(patches, 0)];
    while let Some((pair, level)) = pending.pop() {
        arithmetic.certificate_step()?;
        *visits += 1;
        if visit::separated(&pair[0].patch.hull, &pair[1].patch.hull) {
            continue;
        }
        if level == MAX_PRODUCT_DEPTH {
            return Ok(false);
        }
        push_children(&pair, level, &mut pending, arithmetic, memo)?;
    }
    Ok(true)
}

fn push_children(
    pair: &[PatchUse; 2],
    level: u8,
    pending: &mut Vec<([PatchUse; 2], u8)>,
    arithmetic: &mut Arithmetic<'_>,
    memo: &mut Memo<'_>,
) -> Outcome<()> {
    let side = usize::from(level % 2);
    let children = memo.split(&pair[side], level % 4 < 2, arithmetic)?;
    for child in children.into_iter().rev() {
        let mut next = pair.clone();
        next[side] = child;
        pending.push((next, level + 1));
    }
    Ok(())
}
