//! The 2d curve family and its shared splitter state.
use super::split_curve_geometry::{CurveFamily2d, CurveSplitKernel, define_curve_splitter};
use super::{SplitCurve, SplitCurveData};
use martensite_tkernel::{
    ncollection::HArray1,
    standard::{Failure, Handle},
};
use martensite_tkg2d::geom2d::Geom2dCurve;

/// Shared resulting curve array; multipart `build(false)` entries may be null.
pub type SplitCurves2d = Handle<HArray1<Handle<dyn Geom2dCurve>>>;

define_curve_splitter!(
    ShapeUpgradeSplitCurve2d,
    SplitCurve2d,
    CurveSplit2dData,
    SplitCurves2d,
    dyn Geom2dCurve,
    CurveFamily2d,
    "ShapeUpgrade_SplitCurve2d"
);
