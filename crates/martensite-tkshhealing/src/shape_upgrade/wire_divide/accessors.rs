//! Public result and tool configuration preserve native no-reset setters.
use super::*;
use crate::shape_analysis::TransferParameters;
use crate::shape_upgrade::EdgeDivider;
use crate::shape_upgrade::FixSmallCurves;
impl WireDivideData {
    /// Return the loaded/resulting wire; `tool.wire()`.
    pub fn wire(&self) -> Wire {
        self.with(|fields| fields.wire.clone())
    }
    /// Decode the last Perform status; `tool.status_flag(ShapeExtendStatus::Done1)`.
    pub fn status_flag(&self, status: ShapeExtendStatus) -> bool {
        self.with(|fields| ShapeExtend::decode_status(fields.status, status))
    }
    /// Configure spatial curve splitting; `tool.set_split_curve_3d_tool(&splitter)`.
    pub fn set_split_curve_3d_tool(&self, tool: &Handle<dyn SplitCurve3d>) {
        self.with_mut(|fields| fields.spatial = tool.clone());
    }
    /// Configure pcurve splitting; `tool.set_split_curve_2d_tool(&splitter)`.
    pub fn set_split_curve_2d_tool(&self, tool: &Handle<dyn SplitCurve2d>) {
        self.with_mut(|fields| fields.planar = tool.clone());
    }
    /// Configure parameter transfer; `tool.set_transfer_param_tool(&transfer)`.
    pub fn set_transfer_param_tool(&self, tool: &Handle<dyn TransferParameters>) {
        self.with_mut(|fields| fields.transfer = tool.clone());
    }
    /// Replace the edge criterion; `tool.set_edge_divide_tool(&criterion)`.
    pub fn set_edge_divide_tool(&self, tool: &Handle<dyn EdgeDivider>) {
        self.with_mut(|fields| fields.edge_divide = tool.clone());
    }
    /// Select free/shared/all spatial curves, preserving other integer modes; `tool.set_edge_mode(2)`.
    pub fn set_edge_mode(&self, mode: i32) {
        self.with_mut(|fields| fields.edge_mode = mode);
    }
    /// Replace small-curve approximation; `tool.set_fix_small_curve_tool(&approximation)`.
    pub fn set_fix_small_curve_tool(&self, tool: &Handle<dyn FixSmallCurves>) {
        self.with_mut(|fields| fields.small = tool.clone());
    }
    /// Access the small-curve tool; `tool.fix_small_curve_tool()`.
    pub fn fix_small_curve_tool(&self) -> Handle<dyn FixSmallCurves> {
        self.with(|fields| fields.small.clone())
    }
}
