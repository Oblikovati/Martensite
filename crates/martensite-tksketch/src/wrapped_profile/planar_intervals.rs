//! Every authored curve must have an exact, nonoverlapping cover of its whole parameter range.
use super::*;
use std::collections::BTreeMap;
pub(super) fn complete(
    spans: &[CurveSpan],
    authored: &BTreeMap<SurfaceCurveId, &[[f64; 2]]>,
) -> bool {
    let mut groups: BTreeMap<SurfaceCurveId, Vec<CurveSpan>> = BTreeMap::new();
    for span in spans {
        groups.entry(span.curve).or_default().push(*span);
    }
    groups.keys().eq(authored.keys()) && groups.values().all(|spans| partition(spans))
}
fn partition(spans: &[CurveSpan]) -> bool {
    let level = spans.iter().map(|s| s.level).max().unwrap_or(0);
    if !valid_level(spans, level) {
        return false;
    }
    let mut ranges = normalized_ranges(spans, level);
    ranges.sort_unstable();
    let mut cursor = 0;
    for (start, width) in ranges {
        if start != cursor {
            return false;
        }
        cursor += width;
    }
    cursor == (1u128 << level)
}

fn valid_level(spans: &[CurveSpan], level: u8) -> bool {
    level <= 63 && spans.iter().all(|s| u64::from(s.index) < (1u64 << s.level))
}
fn normalized_ranges(spans: &[CurveSpan], level: u8) -> Vec<(u128, u128)> {
    spans
        .iter()
        .map(|s| {
            let width = 1u128 << (level - s.level);
            (u128::from(s.index) * width, width)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn span(level: u8, index: u32) -> CurveSpan {
        CurveSpan {
            curve: SurfaceCurveId(9),
            level,
            index,
        }
    }
    #[test]
    fn nonuniform_dyadic_partition_is_complete_without_order_identity() {
        assert!(partition(&[span(2, 3), span(1, 0), span(2, 2)]));
    }
    #[test]
    fn gaps_overlaps_empty_and_invalid_levels_are_refused() {
        for input in [
            vec![],
            vec![span(1, 0)],
            vec![span(0, 0), span(0, 0)],
            vec![span(64, 0)],
            vec![span(1, 2)],
        ] {
            assert!(!partition(&input));
        }
    }
}
