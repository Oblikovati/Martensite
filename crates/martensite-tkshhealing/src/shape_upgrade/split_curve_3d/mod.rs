//! The 3d curve family and its shared splitter state.
use super::split_curve_geometry::{CurveFamily3d, CurveSplitKernel, define_curve_splitter};
use super::{SplitCurve, SplitCurveData};
use martensite_tkernel::{
    ncollection::HArray1,
    standard::{Failure, Handle},
};
use martensite_tkg3d::geom::GeomCurve;

/// Shared resulting curve array; multipart `build(false)` entries may be null.
pub type SplitCurves3d = Handle<HArray1<Handle<dyn GeomCurve>>>;

define_curve_splitter!(
    ShapeUpgradeSplitCurve3d,
    SplitCurve3d,
    CurveSplit3dData,
    SplitCurves3d,
    dyn GeomCurve,
    CurveFamily3d,
    "ShapeUpgrade_SplitCurve3d"
);
