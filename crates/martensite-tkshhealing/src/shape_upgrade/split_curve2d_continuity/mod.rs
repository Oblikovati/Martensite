//! ShapeUpgradeSplitCurve2dContinuity.
use super::{
    CurveSplit2dData, SplitCurve, SplitCurve2d, SplitCurveData, SplitCurves2d,
    curve_continuity::ContinuitySettings,
};
use martensite_tkernel::{
    impl_transient, precision,
    standard::{Failure, Handle, SharedState},
};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dOffsetCurve, Geom2dTrimmedCurve};
use martensite_tkmath::geom_abs::Shape;

mod compute;

/// Correct or split a private 2D curve copy at insufficiently continuous knots.
///
/// ```
/// use martensite_tkshhealing::shape_upgrade::ShapeUpgradeSplitCurve2dContinuity;
/// let splitter = ShapeUpgradeSplitCurve2dContinuity::new();
/// splitter.set_criterion(martensite_tkmath::geom_abs::Shape::C2);
/// splitter.set_tolerance(1e-7);
/// ```
pub struct ShapeUpgradeSplitCurve2dContinuity {
    geometry: CurveSplit2dData,
    settings: SharedState<ContinuitySettings>,
}
impl Default for ShapeUpgradeSplitCurve2dContinuity {
    fn default() -> Self {
        Self {
            geometry: CurveSplit2dData::default(),
            settings: SharedState::new(
                ContinuitySettings::with_tolerance(precision::p_confusion()),
            ),
        }
    }
}
impl_transient!(ShapeUpgradeSplitCurve2dContinuity, "ShapeUpgrade_SplitCurve2dContinuity",
    dyn SplitCurve2d, interfaces: [dyn SplitCurve, dyn SplitCurve2d]);

impl ShapeUpgradeSplitCurve2dContinuity {
    /// defaults are C1 and `precision::p_confusion()`;
    /// `ShapeUpgradeSplitCurve2dContinuity::new()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the requested continuity; unsupported geometric criteria use order one;
    /// `splitter.set_criterion(Shape::C2)`.
    pub fn set_criterion(&self, criterion: Shape) {
        self.settings.write().set_criterion(criterion);
    }

    /// Set the tolerance passed to `Geom2d_BSplineCurve::RemoveKnot` without clamping;
    /// `splitter.set_tolerance(1e-7)`.
    pub fn set_tolerance(&self, tolerance: f64) {
        self.settings.write().tolerance = tolerance;
    }

    /// Copy a 2D curve over its natural bounds; `splitter.init_curve(&curve)?`.
    pub fn init_curve(&self, curve: &Handle<dyn Geom2dCurve>) -> Result<(), Failure> {
        self.geometry.init_curve(curve)
    }

    /// Copy a 2D curve over an explicit range; `splitter.init_curve_range(&curve, 0., 1.)?`.
    pub fn init_curve_range(
        &self,
        curve: &Handle<dyn Geom2dCurve>,
        first: f64,
        last: f64,
    ) -> Result<(), Failure> {
        self.geometry.init_curve_range(curve, first, last)
    }

    /// The corrected private curve copy; `let curve = splitter.curve()`.
    pub fn curve(&self) -> Handle<dyn Geom2dCurve> {
        self.geometry.source_curve()
    }

    /// The result array shared with the base splitter; `let curves = splitter.curves()`.
    pub fn curves(&self) -> SplitCurves2d {
        self.geometry.curves()
    }
}

impl SplitCurve for ShapeUpgradeSplitCurve2dContinuity {
    fn split_curve_data(&self) -> &SplitCurveData {
        self.geometry.split_curve_data()
    }

    fn build(&self, segment: bool) -> Result<(), Failure> {
        self.geometry.build_geometry(segment)
    }

    fn compute(&self) -> Result<(), Failure> {
        self.compute_continuity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_uses_the_native_c1_and_pconfusion_defaults() {
        let settings = ShapeUpgradeSplitCurve2dContinuity::new();
        let settings = *settings.settings.read();
        assert_eq!(settings.criterion, Shape::C1);
        assert_eq!(settings.order, 1);
        assert_eq!(settings.tolerance, precision::p_confusion());
    }
}

impl SplitCurve2d for ShapeUpgradeSplitCurve2dContinuity {
    fn curve_geometry(&self) -> &CurveSplit2dData {
        &self.geometry
    }
}
