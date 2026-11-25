// SPDX-License-Identifier: GPL-2.0-only
//! Reuse the authored coefficient index for membership and complete reference coverage.
use super::sector::SourceBits;
use std::cmp::Ordering;

#[derive(Clone)]
pub(super) struct OriginalSourceIndex {
    ordered: Vec<usize>,
}

impl OriginalSourceIndex {
    pub(super) fn from_sources(source: &SourceBits) -> Self {
        // Only sort the index. The receipt and its diagnostics retain authored order.
        let mut ordered: Vec<_> = (0..source.len()).collect();
        ordered.sort_unstable_by(|&a, &b| exact_order(stored(source, a), stored(source, b)));
        Self { ordered }
    }

    pub(super) fn contains_exact(&self, source: &SourceBits, original: &[[f64; 2]]) -> bool {
        self.ordered
            .binary_search_by(|&i| exact_order(stored(source, i), incoming(original)))
            .is_ok()
    }

    pub(super) fn covers<'a>(
        &self,
        source: &SourceBits,
        fitted: impl Iterator<Item = &'a [[f64; 2]]>,
    ) -> bool {
        let mut seen = vec![false; self.ordered.len()];
        for poles in fitted {
            let Some(slot) = self.numeric_slot(source, poles) else {
                return false;
            };
            seen[slot] = true;
        }
        self.group_starts(source).all(|slot| seen[slot])
    }

    fn numeric_slot(&self, source: &SourceBits, original: &[[f64; 2]]) -> Option<usize> {
        let compare = |i| numeric_order(stored(source, i), incoming(original));
        let slot = self.ordered.partition_point(|&i| compare(i).is_lt());
        self.ordered.get(slot).filter(|&&i| compare(i).is_eq())?;
        Some(slot)
    }

    fn group_starts<'a>(&'a self, source: &'a SourceBits) -> impl Iterator<Item = usize> + 'a {
        let following = self
            .ordered
            .windows(2)
            .enumerate()
            .filter_map(move |(i, pair)| {
                numeric_order(stored(source, pair[0]), stored(source, pair[1]))
                    .is_ne()
                    .then_some(i + 1)
            });
        (0..self.ordered.len().min(1)).chain(following)
    }
}

fn stored(source: &SourceBits, index: usize) -> impl Iterator<Item = [u64; 2]> + Clone + '_ {
    source[index].iter().copied()
}

fn incoming(original: &[[f64; 2]]) -> impl Iterator<Item = [u64; 2]> + Clone + '_ {
    original.iter().map(|point| point.map(f64::to_bits))
}

fn exact_order(
    left: impl Iterator<Item = [u64; 2]> + Clone,
    right: impl Iterator<Item = [u64; 2]> + Clone,
) -> Ordering {
    numeric_order(left.clone(), right.clone()).then_with(|| left.cmp(right))
}

fn numeric_order(
    left: impl Iterator<Item = [u64; 2]>,
    right: impl Iterator<Item = [u64; 2]>,
) -> Ordering {
    left.map(numeric_bits).cmp(right.map(numeric_bits))
}

fn numeric_bits(point: [u64; 2]) -> [u64; 2] {
    // Coverage compares finite coordinate values. Proof membership compares the raw bits.
    point.map(|value| {
        if value == (-0.0_f64).to_bits() {
            0
        } else {
            value
        }
    })
}

#[cfg(test)]
mod tests;
