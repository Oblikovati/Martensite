// SPDX-License-Identifier: GPL-2.0-only
use super::*;

/// Original geometry and explicit resource limits for a priori Hermite sizing.
#[derive(Clone, Copy)]
pub struct CylindricalFitQuery<'a> {
    /// Original developed polynomial, degree one through three.
    pub authored: &'a [[f64; 2]],
    /// Exact original-curve interval to fit.
    pub interval: CylindricalProfileInterval,
    /// Positive developed support radius.
    pub support_radius: f64,
    /// Declared angular chart origin.
    pub angular_origin: f64,
    /// Original developed-position allowance; V1 reserves half for ideal Hermite error.
    pub spatial_limit: f64,
    /// Maximum additional dyadic geometric subdivisions, with total level at most 63.
    pub max_level: u8,
    /// Operation allowance, at most one million; affects refusal, never selected geometry.
    pub max_operations: u32,
}

/// A priori geometry plan; actual stored coefficients still require profile certification.
#[derive(Clone, Debug)]
pub struct CylindricalFitPlan {
    level: u8,
    center: f64,
    ideal_error: f64,
    requested_limit: f64,
    operations: u32,
}
impl CylindricalFitPlan {
    /// Halvings relative to the requested interval; `plan.subdivision_level()`.
    pub fn subdivision_level(&self) -> u8 {
        self.level
    }
    /// Radian center for the whole requested chart; `plan.chart_center()`.
    pub fn chart_center(&self) -> f64 {
        self.center
    }
    /// Certified ideal Hermite angular displacement, measured in developed length.
    /// Example: `assert!(plan.ideal_developed_error_bound() <= plan.requested_spatial_limit()/2.);`.
    pub fn ideal_developed_error_bound(&self) -> f64 {
        self.ideal_error
    }
    /// Original requested position allowance; `plan.requested_spatial_limit()`.
    pub fn requested_spatial_limit(&self) -> f64 {
        self.requested_limit
    }
    /// Arithmetic operation count, excluded from geometry identity; `plan.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    pub(super) fn assemble(
        level: u8,
        center: f64,
        ideal_error: f64,
        query: CylindricalFitQuery<'_>,
        operations: u32,
    ) -> Self {
        Self {
            level,
            center,
            ideal_error,
            requested_limit: query.spatial_limit,
            operations,
        }
    }
}
