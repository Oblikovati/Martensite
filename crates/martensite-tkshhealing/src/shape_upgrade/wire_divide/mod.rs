//! wire division.
use super::{
    EdgeDivider, ShapeUpgradeEdgeDivide, ShapeUpgradeFixSmallCurves, ShapeUpgradeSplitCurve2d,
    ShapeUpgradeSplitCurve3d, SplitCurve, SplitCurves2d, SplitCurves3d,
};
use crate::shape_upgrade::SplitCurve2d;
use crate::shape_upgrade::SplitCurve3d;
use crate::shape_upgrade::{UpgradeTool, UpgradeToolData};
use crate::{
    shape_analysis::{ShapeAnalysisEdge, ShapeAnalysisTransferParametersProj, TransferParameters},
    shape_build::ShapeBuildReShape,
    shape_extend::{ShapeExtend, ShapeExtendStatus},
};
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    brep_tools::ReShape,
    topods::{Edge, Face, Wire},
};
use martensite_tkernel::{
    impl_transient, precision,
    standard::{Failure, Handle, SharedState},
};
use martensite_tkg3d::geom::GeomSurface;
use martensite_tkmath::toploc::Location;
mod accessors;
mod perform;
mod pieces;
mod segments;
mod split_values;
mod state;
mod vertices;
pub use state::{WireDivideData, WireDivideFields};
/// Virtual splitting policy and parameter transfer; `tool.perform()?`.
pub trait WireDivider: UpgradeTool {
    /// Shared protected fields; `tool.wire_divide_data()`.
    fn wire_divide_data(&self) -> &WireDivideData;
    /// Inherited tolerance and context storage; `tool.wire_divide_base().precision()`.
    fn wire_divide_base(&self) -> &UpgradeToolData {
        self.upgrade_tool_data()
    }
    /// Split the loaded wire without mutating its original topology; `tool.perform()?`.
    fn perform(&self) -> Result<(), Failure> {
        perform::perform(self)
    }
    /// Virtual spatial splitter selection; `tool.split_curve_3d_tool()`.
    fn split_curve_3d_tool(&self) -> Handle<dyn SplitCurve3d> {
        self.wire_divide_data()
            .with(|fields| fields.spatial.clone())
    }
    /// Virtual pcurve splitter selection; `tool.split_curve_2d_tool()`.
    fn split_curve_2d_tool(&self) -> Handle<dyn SplitCurve2d> {
        self.wire_divide_data().with(|fields| fields.planar.clone())
    }
    /// Virtual edge criterion accessor; `tool.edge_divide_tool()`.
    fn edge_divide_tool(&self) -> Handle<dyn EdgeDivider> {
        self.wire_divide_data()
            .with(|fields| fields.edge_divide.clone())
    }
    /// Virtual transfer selection; `tool.transfer_param_tool()`.
    fn transfer_param_tool(&self) -> Handle<dyn TransferParameters> {
        self.wire_divide_data()
            .with(|fields| fields.transfer.clone())
    }
}
impl std::ops::Deref for dyn WireDivider {
    type Target = WireDivideData;
    fn deref(&self) -> &Self::Target {
        self.wire_divide_data()
    }
}

parent_interface!(dyn WireDivider, ShapeUpgradeWireDivide, [dyn UpgradeTool]);
/// Wire, free-edge and support-face division; `ShapeUpgradeWireDivide::new()`.
#[derive(Default)]
pub struct ShapeUpgradeWireDivide {
    state: WireDivideData,
}
impl_transient!(ShapeUpgradeWireDivide,"ShapeUpgrade_WireDivide",dyn UpgradeTool,interfaces:[dyn WireDivider, dyn UpgradeTool]);
impl WireDivider for ShapeUpgradeWireDivide {
    fn wire_divide_data(&self) -> &WireDivideData {
        &self.state
    }
}
impl std::ops::Deref for ShapeUpgradeWireDivide {
    type Target = WireDivideData;
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}
impl ShapeUpgradeWireDivide {
    /// Construct default criteria and projection transfer; `ShapeUpgradeWireDivide::new()`.
    pub fn new() -> Self {
        Self::default()
    }
}
impl WireDivideData {
    /// Initialize by existing face and reset status; `tool.init(&wire,&face)`.
    pub fn init(&self, wire: &Wire, face: &Face) {
        self.with_mut(|fields| {
            fields.wire = wire.clone();
            fields.face = face.clone();
            fields.status = 0;
        });
    }
    /// Build the natural surface face before resetting status; `tool.init_surface(&wire,&surface)?`.
    pub fn init_surface(
        &self,
        wire: &Wire,
        surface: &Handle<dyn GeomSurface>,
    ) -> Result<(), Failure> {
        self.load_wire(wire);
        self.set_surface(surface)?;
        self.with_mut(|fields| fields.status = 0);
        Ok(())
    }
    /// Load a wire while retaining status and support; `tool.load_wire(&wire)`.
    pub fn load_wire(&self, wire: &Wire) {
        self.with_mut(|fields| fields.wire = wire.clone());
    }
    /// Build a single-edge wire and load it when construction succeeds; `tool.load_edge(&edge)?`.
    pub fn load_edge(&self, edge: &Edge) -> Result<(), Failure> {
        let maker = martensite_tktopalgo::brep_lib::BRepLibMakeWire::from_edge(edge)?;
        if maker.is_done() {
            self.load_wire(&maker.wire()?);
        }
        Ok(())
    }
    /// Change the support face without resetting state; `tool.set_face(&face)`.
    pub fn set_face(&self, face: &Face) {
        self.with_mut(|fields| fields.face = face.clone());
    }
    /// Build the natural surface face; `tool.set_surface(&surface)?`.
    pub fn set_surface(&self, surface: &Handle<dyn GeomSurface>) -> Result<(), Failure> {
        let maker = martensite_tktopalgo::brep_lib::BRepLibMakeFace::from_surface(
            surface,
            precision::confusion(),
        )?;
        self.set_face(&maker.face()?);
        Ok(())
    }
    /// Create a face with explicit location, without natural-bound construction; `tool.set_surface_location(&surface,&location)?`.
    pub fn set_surface_location(
        &self,
        surface: &Handle<dyn GeomSurface>,
        location: &Location,
    ) -> Result<(), Failure> {
        let mut face = self.with(|fields| fields.face.clone());
        let result = BRepBuilder::default().make_face_on_located_surface(
            &mut face,
            surface,
            location,
            precision::confusion(),
        );
        self.set_face(&face);
        result
    }
}

impl UpgradeTool for ShapeUpgradeWireDivide {
    fn upgrade_tool_data(&self) -> &UpgradeToolData {
        &self.state.tool
    }
}

impl std::ops::Deref for WireDivideData {
    type Target = UpgradeToolData;
    fn deref(&self) -> &Self::Target {
        &self.tool
    }
}
