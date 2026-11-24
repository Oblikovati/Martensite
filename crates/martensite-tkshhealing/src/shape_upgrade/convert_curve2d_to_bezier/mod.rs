//! Convert planar curves to Bezier pieces with shared split parameters.
use super::{CurveSplit2dData, SplitCurve, SplitCurve2d, SplitCurveData, SplitCurves2d};
use martensite_tkernel::{
    impl_transient,
    ncollection::HSequence,
    standard::{Failure, Handle, SharedState},
};
use martensite_tkg2d::geom2d::Geom2dCurve;

mod build;
mod compute;
mod spline;

struct ConversionState {
    segments: Handle<HSequence<Handle<dyn Geom2dCurve>>>,
    split_params: Handle<HSequence<f64>>,
}
impl Default for ConversionState {
    fn default() -> Self {
        Self {
            segments: Handle::new(HSequence::default()),
            split_params: Handle::new(HSequence::default()),
        }
    }
}

/// Convert an initialized p-curve into Bezier segments.
/// `let converter = ShapeUpgradeConvertCurve2dToBezier::new()`.
#[derive(Default)]
pub struct ShapeUpgradeConvertCurve2dToBezier {
    geometry: CurveSplit2dData,
    state: SharedState<ConversionState>,
}
impl_transient!(ShapeUpgradeConvertCurve2dToBezier, "ShapeUpgrade_ConvertCurve2dToBezier", dyn SplitCurve2d, interfaces: [dyn SplitCurve, dyn SplitCurve2d]);

impl ShapeUpgradeConvertCurve2dToBezier {
    /// Create an p-curve converter without an input curve; `ShapeUpgradeConvertCurve2dToBezier::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Initialize from a curve's natural bounds; `converter.init_curve(&curve)?`.
    pub fn init_curve(&self, curve: &Handle<dyn Geom2dCurve>) -> Result<(), Failure> {
        self.geometry.init_curve(curve)
    }
    /// Initialize over a caller-supplied range; `converter.init_curve_range(&curve, 0., 1.)?`.
    pub fn init_curve_range(
        &self,
        curve: &Handle<dyn Geom2dCurve>,
        first: f64,
        last: f64,
    ) -> Result<(), Failure> {
        self.geometry.init_curve_range(curve, first, last)
    }
    /// Parameters delimiting generated Bezier segments; `converter.split_params()`.
    pub fn split_params(&self) -> Handle<HSequence<f64>> {
        self.state.read().split_params.clone()
    }
    /// Shared Bezier result geometry inherited from `ShapeUpgradeSplitCurve2d`.
    /// `let curves: SplitCurves2d = converter.curves()`.
    pub fn curves(&self) -> SplitCurves2d {
        self.geometry.curves()
    }
}
impl SplitCurve for ShapeUpgradeConvertCurve2dToBezier {
    fn split_curve_data(&self) -> &SplitCurveData {
        self.geometry.split_curve_data()
    }
    fn build(&self, _segment: bool) -> Result<(), Failure> {
        build::build(self)
    }
    fn compute(&self) -> Result<(), Failure> {
        compute::compute(self)
    }
}

impl SplitCurve2d for ShapeUpgradeConvertCurve2dToBezier {
    fn curve_geometry(&self) -> &CurveSplit2dData {
        &self.geometry
    }
}
