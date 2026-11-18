// SPDX-License-Identifier: GPL-2.0-only
use super::*;

/// One exact original interval and its conditioned local Hermite plan.
#[derive(Clone, Debug)]
pub struct CylindricalFitChart {
    pub(super) interval: CylindricalProfileInterval,
    pub(super) fit: CylindricalFitPlan,
}
impl CylindricalFitChart {
    /// Original source interval, independent of rounded split controls; `chart.interval()`.
    pub fn interval(&self) -> CylindricalProfileInterval {
        self.interval
    }
    /// Local fit subdivisions and center; `chart.fit_plan()`.
    pub fn fit_plan(&self) -> &CylindricalFitPlan {
        &self.fit
    }
}

/// A priori chart cover under one shared finite arithmetic allowance.
#[derive(Clone, Debug)]
pub struct CylindricalChartPlan {
    pub(super) charts: Vec<CylindricalFitChart>,
    pub(super) operations: u32,
}
impl CylindricalChartPlan {
    /// Canonical dyadic cover in original parameter order; `plan.charts()`.
    pub fn charts(&self) -> &[CylindricalFitChart] {
        &self.charts
    }
    /// Total chart-selection and local-fit arithmetic cost; `plan.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
}
