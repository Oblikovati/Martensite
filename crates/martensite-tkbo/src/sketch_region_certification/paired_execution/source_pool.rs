// SPDX-License-Identifier: GPL-2.0-only
//! Share immutable coefficient storage across pair reports.
use super::*;

#[derive(Default)]
pub(in crate::sketch_region_certification) struct PairSourcePool {
    curves: Vec<Option<CapturedBezierCurve>>,
}

impl PairSourcePool {
    pub(super) fn capture(census: &BoundaryCensus) -> Self {
        let Some(paired) = census.paired.as_ref().filter(|paired| {
            matches!(
                (paired.exclusion_version, paired.requested.version),
                (Some(2), _) | (None, 2)
            )
        }) else {
            return Self::default();
        };
        let curves = census
            .inputs
            .segments
            .iter()
            .zip(&paired.occurrences)
            .enumerate()
            .map(|(slot, (source, row))| {
                if row.physical_source == slot {
                    return CapturedBezierCurve::capture(source_view(source)).ok();
                }
                None
            })
            .collect();
        Self { curves }
    }

    pub(in crate::sketch_region_certification) fn query(
        &self,
        census: &BoundaryCensus,
        sources: [usize; 2],
        mode: PairedQueryMode,
    ) -> Result<CapturedBezierPair, Box<BezierPairCaptureFailure>> {
        let [left, right] = sources.map(|slot| self.curves.get(slot).and_then(Option::as_ref));
        match (mode, left, right) {
            (PairedQueryMode::Distinct, Some(left), Some(right)) => {
                Ok(CapturedBezierPair::from_captured_pair(left, right))
            }
            (PairedQueryMode::SelfOffDiagonal, Some(source), _) => {
                Ok(CapturedBezierPair::from_captured_self(source))
            }
            // Materialize a failed query in pair-slot order so the original typed error is retained.
            _ => captured_query(census, sources, mode),
        }
    }
}
