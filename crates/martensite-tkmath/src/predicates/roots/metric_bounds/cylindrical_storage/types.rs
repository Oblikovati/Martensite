// SPDX-License-Identifier: GPL-2.0-only
use super::super::MetricBezier;

/// Work and storage limits, refused rather than clamped; `CylindricalStorageOptions::default()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CylindricalStorageOptions {
    /// Fixed dyadic fractional bits, in `8..=4096`.
    pub fractional_bits: u16,
    /// Largest admitted stored degree, in `1..=25`.
    pub max_degree: u8,
    /// Maximum interval arithmetic operations, at most one million; zero refuses work.
    pub max_operations: u32,
}
impl Default for CylindricalStorageOptions {
    fn default() -> Self {
        Self {
            fractional_bits: 128,
            max_degree: 25,
            max_operations: 100_000,
        }
    }
}

/// Spatial limits in the local support chart; `CylindricalStorageLimits::uniform(1e-8)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CylindricalStorageLimits {
    /// Maximum radial distance from either analytic cylinder.
    pub radial_distance: f64,
    /// Maximum XY distance of C1 from `(r1/r0)*C0` at the same parameter.
    pub scaled_ruling: f64,
    /// Maximum absolute Z difference of the two curves at the same parameter.
    pub height_drift: f64,
    /// Maximum combined discrepancy from the normalized first-curve radial sweep.
    pub normal_sweep: f64,
}
impl CylindricalStorageLimits {
    /// Allocate one stated bound to each residual; `CylindricalStorageLimits::uniform(1e-8)`.
    pub fn uniform(limit: f64) -> Self {
        Self {
            radial_distance: limit,
            scaled_ruling: limit,
            height_drift: limit,
            normal_sweep: limit,
        }
    }
}

/// Decoded control points and actual retained weights; omitted weights mean exactly one.
/// Example: `CylindricalStorageQuery { curves, radii: [9., 12.], limits, options }`.
#[derive(Clone, Copy)]
pub struct CylindricalStorageQuery<'a> {
    /// Ordered depth curves with their natural common `[0,1]` parameter domain.
    pub curves: [MetricBezier<'a, 3>; 2],
    /// Strictly positive support radii; the first radius scales the ruling residual.
    pub radii: [f64; 2],
    /// Original requested bounds, with no implicit no-limit mode.
    pub limits: CylindricalStorageLimits,
    /// Original precision and work policy.
    pub options: CylindricalStorageOptions,
}

/// Residual whose conservative bound did not meet the request; no violation is inferred.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CylindricalStorageResidual {
    /// First curve's distance from the cylinder at its requested radius.
    FirstRadius,
    /// Last curve's distance from the cylinder at its requested radius.
    LastRadius,
    /// XY scaled-ruling discrepancy.
    ScaledRuling,
    /// Same-parameter Z difference.
    HeightDrift,
    /// Combined bound for the complete depth interpolation, using the first curve as reference.
    NormalSweep,
}

/// Complete conservative bounds over `[0,1]`, tied to exact supplied binary64 bits.
/// Example: `assert!(proof.radial_distance_bounds()[0] <= proof.requested_limits().radial_distance);`.
#[derive(Clone, Debug)]
pub struct CertifiedCylindricalStorage {
    bounds: [f64; 5],
    weight_lower: [f64; 2],
    radii: [f64; 2],
    source_bits: Vec<u64>,
    limits: CylindricalStorageLimits,
    options: CylindricalStorageOptions,
    operations: u32,
}
impl CertifiedCylindricalStorage {
    /// Exact retained radius inputs; `proof.stored_radii()`.
    pub fn stored_radii(&self) -> [f64; 2] {
        self.radii
    }

    /// Upward-rounded radial bounds for the ordered depths; `proof.radial_distance_bounds()`.
    pub fn radial_distance_bounds(&self) -> [f64; 2] {
        [self.bounds[0], self.bounds[1]]
    }
    /// Upward-rounded XY ruling bound, using the L1 norm as an upper bound on L2.
    /// Example: `assert!(proof.scaled_ruling_bound() <= proof.requested_limits().scaled_ruling);`.
    pub fn scaled_ruling_bound(&self) -> f64 {
        self.bounds[2]
    }
    /// Upward-rounded same-parameter height bound; `proof.height_difference_bound()`.
    pub fn height_difference_bound(&self) -> f64 {
        self.bounds[3]
    }
    /// Bound the ruled interpolation's distance from its normalized first-curve radial sweep.
    /// Example: `assert!(proof.normal_sweep_error_bound() <= proof.requested_limits().normal_sweep);`.
    pub fn normal_sweep_error_bound(&self) -> f64 {
        self.bounds[4]
    }
    /// Exact positive minima of the supplied weights; `proof.weight_lower_bounds()`.
    pub fn weight_lower_bounds(&self) -> [f64; 2] {
        self.weight_lower
    }
    /// Read the unchanged requested caps; `proof.requested_limits()`.
    pub fn requested_limits(&self) -> CylindricalStorageLimits {
        self.limits
    }
    /// Read the unchanged work policy; `proof.requested_options()`.
    pub fn requested_options(&self) -> CylindricalStorageOptions {
        self.options
    }
    /// Count of interval arithmetic operations, independent of source identity; `proof.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Check coefficient and radius identity, including signed zero; `proof.matches_source(curves, radii)`.
    pub fn matches_source(&self, curves: [MetricBezier<'_, 3>; 2], radii: [f64; 2]) -> bool {
        let input = CylindricalStorageQuery {
            curves,
            radii,
            limits: self.limits,
            options: self.options,
        };
        super::admission::admit(&input).is_ok() && self.source_bits == source_bits(curves, radii)
    }
    pub(super) fn assemble(
        query: CylindricalStorageQuery<'_>,
        bounds: [f64; 5],
        operations: u32,
    ) -> Self {
        Self {
            bounds,
            radii: query.radii,
            weight_lower: query.curves.map(weight_minimum),
            source_bits: source_bits(query.curves, query.radii),
            limits: query.limits,
            options: query.options,
            operations,
        }
    }
}

pub(super) fn weight_minimum(curve: MetricBezier<'_, 3>) -> f64 {
    curve
        .weights
        .map_or(1., |w| w.iter().copied().fold(f64::INFINITY, f64::min))
}

pub(super) fn source_bits(curves: [MetricBezier<'_, 3>; 2], reference: [f64; 2]) -> Vec<u64> {
    let mut result: Vec<_> = reference.into_iter().map(f64::to_bits).collect();
    for curve in curves {
        result.push(curve.poles.len() as u64);
        result.push(curve.weights.map_or(u64::MAX, |w| w.len() as u64));
        result.extend(curve.poles.iter().flatten().map(|v| v.to_bits()));
        result.extend(curve.weights.into_iter().flatten().map(|v| v.to_bits()));
    }
    result
}
