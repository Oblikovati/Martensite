// SPDX-License-Identifier: GPL-2.0-only
//! Wave 4 conical receipt enrichment, ordered contact index lookup, and
//! normalized polynomial caching (#497, #511, #512, #527, #564, #565).
use martensite_tkmath::gp::Point3;
use std::collections::BTreeMap;

/// Conical source receipt recording explicit request radius and chart center (#497).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConicalSourceReceipt {
    /// Requested reference radius on the cone.
    pub request_radius: f64,
    /// Center of the angular/radial chart on the cone axis.
    pub chart_center: Point3,
    /// Cone half-angle in radians.
    pub half_angle: f64,
    /// True when the conical source passed validation.
    pub verified: bool,
}

/// Ordered contact row indexed by `(source_id, target_id)` (#564).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OrderedContactRow {
    /// Source entity index.
    pub source_id: u32,
    /// Target entity index.
    pub target_id: u32,
    /// Contact parameter on the source entity.
    pub parameter: f64,
}

/// Sorted contact index supporting O(log N) range lookup and cardinality queries (#564).
#[derive(Clone, Debug, Default)]
pub struct OrderedContactIndex {
    rows: Vec<OrderedContactRow>,
}

impl OrderedContactIndex {
    /// Builds a deterministically sorted contact index from `rows` (#564).
    pub fn from_rows(mut rows: Vec<OrderedContactRow>) -> Self {
        rows.sort_by(|a, b| {
            a.source_id
                .cmp(&b.source_id)
                .then_with(|| a.target_id.cmp(&b.target_id))
                .then_with(|| a.parameter.total_cmp(&b.parameter))
        });
        Self { rows }
    }

    /// Returns the slice of contact rows matching `source_id` in O(log N) (#564).
    pub fn rows_for_source(&self, source_id: u32) -> &[OrderedContactRow] {
        let start = self.rows.partition_point(|row| row.source_id < source_id);
        let end = self.rows.partition_point(|row| row.source_id <= source_id);
        &self.rows[start..end]
    }

    /// Returns the number of contacts for `source_id` in O(log N) (#564).
    pub fn cardinality(&self, source_id: u32) -> usize {
        self.rows_for_source(source_id).len()
    }
}

/// Caches normalized polynomials by bit-exact coefficient key to avoid repeated
/// normalization across Bézier-pair membership checks (#565).
#[derive(Clone, Debug, Default)]
pub struct NormalizedPolynomialCache {
    entries: BTreeMap<Vec<u64>, Vec<f64>>,
    hits: usize,
}

impl NormalizedPolynomialCache {
    /// Creates an empty polynomial normalization cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns cached normalized coefficients (max-abs scaled to 1.0), computing once per key (#565).
    pub fn normalize_or_get(&mut self, coeffs: &[f64]) -> &[f64] {
        let key: Vec<u64> = coeffs.iter().map(|v| v.to_bits()).collect();
        if self.entries.contains_key(&key) {
            self.hits += 1;
            return &self.entries[&key];
        }
        let max_abs = coeffs.iter().fold(0.0_f64, |acc, v| acc.max(v.abs()));
        let normalized = if max_abs > 0.0 && max_abs.is_finite() {
            coeffs.iter().map(|v| v / max_abs).collect()
        } else {
            coeffs.to_vec()
        };
        self.entries.entry(key).or_insert(normalized)
    }

    /// Number of cache hits observed.
    pub fn hits(&self) -> usize {
        self.hits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordered_contact_index_answers_cardinality_via_binary_search() {
        let index = OrderedContactIndex::from_rows(vec![
            OrderedContactRow {
                source_id: 5,
                target_id: 2,
                parameter: 0.7,
            },
            OrderedContactRow {
                source_id: 2,
                target_id: 1,
                parameter: 0.1,
            },
            OrderedContactRow {
                source_id: 5,
                target_id: 1,
                parameter: 0.3,
            },
        ]);
        assert_eq!(index.cardinality(1), 0);
        assert_eq!(index.cardinality(2), 1);
        assert_eq!(index.cardinality(5), 2);
        assert_eq!(index.rows_for_source(5)[0].target_id, 1);
    }

    #[test]
    fn normalized_polynomial_cache_reuses_normalized_source_polynomial() {
        let mut cache = NormalizedPolynomialCache::new();
        let first = cache.normalize_or_get(&[2.0, -4.0, 1.0]).to_vec();
        assert_eq!(first, vec![0.5, -1.0, 0.25]);
        assert_eq!(cache.hits(), 0);
        let second = cache.normalize_or_get(&[2.0, -4.0, 1.0]).to_vec();
        assert_eq!(second, first);
        assert_eq!(cache.hits(), 1);
    }

    #[test]
    fn conical_source_receipt_preserves_request_radius_and_chart_center() {
        let receipt = ConicalSourceReceipt {
            request_radius: 12.5,
            chart_center: Point3::new(1.0, 2.0, 3.0),
            half_angle: 0.25,
            verified: true,
        };
        assert_eq!(receipt.request_radius, 12.5);
        assert_eq!(receipt.chart_center, Point3::new(1.0, 2.0, 3.0));
    }
}
