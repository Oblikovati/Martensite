// SPDX-License-Identifier: GPL-2.0-only
use super::{super::MetricBezier, CylindricalProfileFailure};

/// Fixed algorithm revision, separate from geometry identity and work receipts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CylindricalProfileBehavior {
    /// Degree-64/65 scalar and degree-8 polynomial trigonometric Taylor enclosures.
    V1,
}

/// Unclamped arithmetic policy; `CylindricalProfileOptions::default()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CylindricalProfileOptions {
    /// Supported arithmetic revision.
    pub behavior: CylindricalProfileBehavior,
    /// Dyadic fractional bits in `8..=4096`.
    pub fractional_bits: u16,
    /// Interval-operation allowance, at most one million; zero refuses work.
    pub max_operations: u32,
}
impl Default for CylindricalProfileOptions {
    fn default() -> Self {
        Self {
            behavior: CylindricalProfileBehavior::V1,
            fractional_bits: 128,
            max_operations: 100_000,
        }
    }
}

/// Exact authored-curve interval `[index/2^level, (index+1)/2^level]`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CylindricalProfileInterval {
    index: u64,
    level: u8,
}
impl CylindricalProfileInterval {
    /// Admit an exact dyadic interval; `CylindricalProfileInterval::new(1, 2)?` is `[1/4,1/2]`.
    pub fn new(index: u64, level: u8) -> Result<Self, CylindricalProfileFailure> {
        if level > 63 || index >= (1u64 << level) {
            return Err(CylindricalProfileFailure::Interval { index, level });
        }
        Ok(Self { index, level })
    }
    /// Numerator of the left endpoint; `span.index()`.
    pub fn index(self) -> u64 {
        self.index
    }
    /// Power-of-two denominator exponent; `span.level()`.
    pub fn level(self) -> u8 {
        self.level
    }
}

/// Compare one decoded curve to the original authored polynomial, before rounded subdivision.
/// Example: `CylindricalProfileQuery { authored, stored, interval, support_radius, stored_radius, angular_origin: 0., spatial_limit, options }`.
#[derive(Clone, Copy)]
pub struct CylindricalProfileQuery<'a> {
    /// Original developed `(s,z)` control points, degree one through three.
    pub authored: &'a [[f64; 2]],
    /// Actual retained native curve, degree one through nine on `[0,1]`.
    pub stored: MetricBezier<'a, 3>,
    /// Authored parameter interval corresponding to the complete native curve.
    pub interval: CylindricalProfileInterval,
    /// Original support radius, used by `theta = angular_origin + s/support_radius`.
    pub support_radius: f64,
    /// Native curve's positive radius for the trigonometric residual comparison.
    pub stored_radius: f64,
    /// Declared angular chart origin in radians.
    pub angular_origin: f64,
    /// Maximum developed position discrepancy, combining angular displacement and height.
    pub spatial_limit: f64,
    /// Unchanged precision and work policy.
    pub options: CylindricalProfileOptions,
}

/// Certified discrepancy of the normalized stored profile from the authored interval.
/// Derivative bounds use the native piece's local `[0,1]` parameter, not the original full curve.
#[derive(Clone, Debug)]
pub struct CertifiedCylindricalProfile {
    bounds: [f64; 6],
    source_bits: Vec<u64>,
    limit: f64,
    options: CylindricalProfileOptions,
    operations: u32,
}
impl CertifiedCylindricalProfile {
    /// Radian error for the unique angular lift within a half-turn of the authored chart.
    /// Example: `proof.angular_position_bound()`.
    pub fn angular_position_bound(&self) -> f64 {
        self.bounds[0]
    }
    /// Absolute height discrepancy; `proof.height_position_bound()`.
    pub fn height_position_bound(&self) -> f64 {
        self.bounds[1]
    }
    /// Developed L1 position bound `R*angular+height`; `proof.developed_position_bound()`.
    pub fn developed_position_bound(&self) -> f64 {
        self.bounds[2]
    }
    /// Angular derivative discrepancy per local piece parameter; `proof.angular_derivative_bound()`.
    pub fn angular_derivative_bound(&self) -> f64 {
        self.bounds[3]
    }
    /// Height derivative discrepancy per local piece parameter; `proof.height_derivative_bound()`.
    pub fn height_derivative_bound(&self) -> f64 {
        self.bounds[4]
    }
    /// Developed L1 derivative bound; no regularity or join verdict is implied.
    /// Example: `proof.developed_derivative_bound()`.
    pub fn developed_derivative_bound(&self) -> f64 {
        self.bounds[5]
    }
    /// Original position cap; `proof.requested_spatial_limit()`.
    pub fn requested_spatial_limit(&self) -> f64 {
        self.limit
    }
    /// Original policy; `proof.requested_options()`.
    pub fn requested_options(&self) -> CylindricalProfileOptions {
        self.options
    }
    /// Interval operation count, independent of source identity; `proof.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Compare all source bits and the dyadic interval, independent of work policy.
    /// Example: `assert!(proof.matches_source(query));`.
    pub fn matches_source(&self, query: CylindricalProfileQuery<'_>) -> bool {
        super::admission::admit(&query).is_ok() && self.source_bits == source_bits(query)
    }
    pub(in crate::predicates::roots::metric_bounds) fn matches_authored(
        &self,
        authored: &[[f64; 2]],
        interval: CylindricalProfileInterval,
        radius: f64,
        origin: f64,
    ) -> bool {
        // Cone composition reuses O2 without recapturing mutable native objects.
        let bits = &self.source_bits;
        let actual = [bits[0], bits[1], bits[2], bits[4], bits[5]];
        let expected = [
            interval.index(),
            interval.level().into(),
            radius.to_bits(),
            origin.to_bits(),
            authored.len() as u64,
        ];
        actual == expected
            && bits[7..]
                .iter()
                .take(authored.len() * 2)
                .copied()
                .eq(authored.iter().flatten().map(|v| v.to_bits()))
    }
    pub(in crate::predicates::roots::metric_bounds) fn same_source(&self, other: &Self) -> bool {
        self.source_bits == other.source_bits
    }
    pub(super) fn assemble(
        query: CylindricalProfileQuery<'_>,
        bounds: [f64; 6],
        operations: u32,
    ) -> Self {
        Self {
            bounds,
            source_bits: source_bits(query),
            limit: query.spatial_limit,
            options: query.options,
            operations,
        }
    }
}

fn source_bits(query: CylindricalProfileQuery<'_>) -> Vec<u64> {
    let mut bits = vec![
        query.interval.index,
        query.interval.level.into(),
        query.support_radius.to_bits(),
        query.stored_radius.to_bits(),
        query.angular_origin.to_bits(),
        query.authored.len() as u64,
        query.stored.poles.len() as u64,
    ];
    bits.extend(query.authored.iter().flatten().map(|v| v.to_bits()));
    bits.extend(query.stored.poles.iter().flatten().map(|v| v.to_bits()));
    append_weights(&mut bits, query.stored.weights);
    bits
}

fn append_weights(bits: &mut Vec<u64>, weights: Option<&[f64]>) {
    bits.push(weights.map_or(u64::MAX, |w| w.len() as u64));
    bits.extend(weights.into_iter().flatten().map(|v| v.to_bits()));
}
