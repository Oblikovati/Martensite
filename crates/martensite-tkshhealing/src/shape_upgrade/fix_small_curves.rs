//! base class is an extension point: Approx returns false and preserves outputs.
use crate::shape_extend::{ShapeExtend, ShapeExtendStatus};
use crate::shape_upgrade::SplitCurve2d;
use crate::shape_upgrade::SplitCurve3d;
use crate::shape_upgrade::{UpgradeTool, UpgradeToolData};
use martensite_tkbrep::topods::{Edge, Face};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, SharedState},
};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::GeomCurve;
/// Protected native fields for specialized approximation tools.
#[derive(Default)]
pub struct FixSmallCurvesFields {
    /// Status bitset, initially clear (success).
    pub status: i32,
    /// Spatial curve splitter, initially null.
    pub curve_3d_tool: Handle<dyn SplitCurve3d>,
    /// Pcurve splitter, initially null.
    pub curve_2d_tool: Handle<dyn SplitCurve2d>,
    /// Current edge, retained by Init without resetting status.
    pub edge: Edge,
    /// Current support face.
    pub face: Face,
}
/// Protected state shared by approximation subclasses; `FixSmallCurvesData::default()`.
#[derive(Default)]
pub struct FixSmallCurvesData {
    pub(super) tool: UpgradeToolData,
    fields: SharedState<FixSmallCurvesFields>,
}
impl FixSmallCurvesData {
    /// Inspect protected state; `state.with(|fields|fields.status)`.
    pub fn with<R>(&self, read: impl FnOnce(&FixSmallCurvesFields) -> R) -> R {
        read(&self.fields.read())
    }
    /// Update derived-tool state; `state.with_mut(|fields|fields.status=0)`.
    pub fn with_mut<R>(&self, write: impl FnOnce(&mut FixSmallCurvesFields) -> R) -> R {
        write(&mut self.fields.write())
    }
}
/// Virtual approximation extension point; `tool.approx(&mut c3,&mut c2,&mut reversed,&mut first,&mut last)?`.
pub trait FixSmallCurves: UpgradeTool {
    /// protected state; `tool.fix_small_curves_data()`.
    fn fix_small_curves_data(&self) -> &FixSmallCurvesData;
    /// Preserve all arguments and return false in the native base implementation.
    /// Example: `tool.approx(&mut c3,&mut c2,&mut reversed,&mut first,&mut last)?`.
    fn approx(
        &self,
        _curve_3d: &mut Handle<dyn GeomCurve>,
        _curve_2d: &mut Handle<dyn Geom2dCurve>,
        _reversed: &mut Handle<dyn Geom2dCurve>,
        _first: &mut f64,
        _last: &mut f64,
    ) -> Result<bool, Failure> {
        Ok(false)
    }
    /// Protected virtual spatial tool getter; `tool.split_curve_3d_tool()`.
    fn split_curve_3d_tool(&self) -> Handle<dyn SplitCurve3d> {
        self.fix_small_curves_data()
            .with(|f| f.curve_3d_tool.clone())
    }
    /// Protected virtual pcurve tool getter; `tool.split_curve_2d_tool()`.
    fn split_curve_2d_tool(&self) -> Handle<dyn SplitCurve2d> {
        self.fix_small_curves_data()
            .with(|f| f.curve_2d_tool.clone())
    }
}
impl std::ops::Deref for dyn FixSmallCurves {
    type Target = FixSmallCurvesData;
    fn deref(&self) -> &Self::Target {
        self.fix_small_curves_data()
    }
}

parent_interface!(
    dyn FixSmallCurves,
    ShapeUpgradeFixSmallCurves,
    [dyn UpgradeTool]
);
/// base small-curve criterion; `ShapeUpgradeFixSmallCurves::new()`.
#[derive(Default)]
pub struct ShapeUpgradeFixSmallCurves {
    state: FixSmallCurvesData,
}
impl_transient!(ShapeUpgradeFixSmallCurves,"ShapeUpgrade_FixSmallCurves",dyn UpgradeTool,interfaces:[dyn FixSmallCurves, dyn UpgradeTool]);
impl FixSmallCurves for ShapeUpgradeFixSmallCurves {
    fn fix_small_curves_data(&self) -> &FixSmallCurvesData {
        &self.state
    }
}
impl std::ops::Deref for ShapeUpgradeFixSmallCurves {
    type Target = FixSmallCurvesData;
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}
impl ShapeUpgradeFixSmallCurves {
    /// Create the base tool with null splitters and OK status; `ShapeUpgradeFixSmallCurves::new()`.
    pub fn new() -> Self {
        Self::default()
    }
}
impl FixSmallCurvesData {
    /// Retain edge and face without resetting status; `tool.init(&edge,&face)`.
    pub fn init(&self, edge: &Edge, face: &Face) {
        self.with_mut(|f| {
            f.edge = edge.clone();
            f.face = face.clone();
        });
    }
    /// Set the spatial splitting tool; `tool.set_split_curve_3d_tool(&splitter)`.
    pub fn set_split_curve_3d_tool(&self, tool: &Handle<dyn SplitCurve3d>) {
        self.with_mut(|f| f.curve_3d_tool = tool.clone());
    }
    /// Set the pcurve splitting tool; `tool.set_split_curve_2d_tool(&splitter)`.
    pub fn set_split_curve_2d_tool(&self, tool: &Handle<dyn SplitCurve2d>) {
        self.with_mut(|f| f.curve_2d_tool = tool.clone());
    }
    /// Decode the native status bitset; `tool.status_flag(ShapeExtendStatus::Ok)`.
    pub fn status_flag(&self, status: ShapeExtendStatus) -> bool {
        self.with(|f| ShapeExtend::decode_status(f.status, status))
    }
}

impl UpgradeTool for ShapeUpgradeFixSmallCurves {
    fn upgrade_tool_data(&self) -> &UpgradeToolData {
        &self.state.tool
    }
}

impl std::ops::Deref for FixSmallCurvesData {
    type Target = UpgradeToolData;
    fn deref(&self) -> &Self::Target {
        &self.tool
    }
}
