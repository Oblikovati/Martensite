//! Face subdivision orchestration.
use super::{ShapeUpgradeSplitSurface, ShapeUpgradeWireDivide};
use crate::shape_extend::{ShapeExtend, ShapeExtendStatus};
use crate::shape_upgrade::SplitSurface;
use crate::shape_upgrade::WireDivider;
use crate::shape_upgrade::{UpgradeTool, UpgradeToolData};
use martensite_tkbrep::topods::{Face, Shape};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, SharedState},
};
mod curves;
mod surface;

/// protected face subdivision state; `FaceDivideData::default()`.
pub struct FaceDivideData {
    pub(super) tool: UpgradeToolData,
    fields: SharedState<FaceDivideFields>,
}
/// Working face, result, modes, and configured criteria.
pub struct FaceDivideFields {
    /// Original support face.
    pub face: Face,
    /// Current intermediate/result shape.
    pub result: Shape,
    /// Whether split surfaces are segmented.
    pub segment: bool,
    /// ShapeExtend status bits.
    pub status: i32,
    surface_tool: Handle<dyn SplitSurface>,
    wire_tool: Handle<dyn WireDivider>,
}
impl Default for FaceDivideData {
    fn default() -> Self {
        Self {
            tool: UpgradeToolData::default(),
            fields: SharedState::new(FaceDivideFields {
                face: Face::default(),
                result: Shape::default(),
                segment: true,
                status: 0,
                surface_tool: Handle::new(ShapeUpgradeSplitSurface::new()).into_upcast(),
                wire_tool: Handle::new(ShapeUpgradeWireDivide::new()).into_upcast(),
            }),
        }
    }
}
impl FaceDivideData {
    /// Read protected state; `state.with(|fields| fields.segment)`.
    pub fn with<R>(&self, read: impl FnOnce(&FaceDivideFields) -> R) -> R {
        read(&self.fields.read())
    }
    /// Update protected state for a derived policy; `state.with_mut(|fields| fields.status = 0)`.
    pub fn with_mut<R>(&self, write: impl FnOnce(&mut FaceDivideFields) -> R) -> R {
        write(&mut self.fields.write())
    }
    fn add_status(&self, status: ShapeExtendStatus) {
        self.with_mut(|fields| fields.status |= ShapeExtend::encode_status(status));
    }
}

/// Virtual surface and boundary subdivision policy; `divider.perform(0.)?`.
pub trait FaceDivider: UpgradeTool {
    /// Shared protected state; `divider.face_divide_data()`.
    fn face_divide_data(&self) -> &FaceDivideData;
    /// Shared tolerance and reshape context; `divider.face_divide_base()`.
    fn face_divide_base(&self) -> &UpgradeToolData {
        self.upgrade_tool_data()
    }
    /// Split surfaces before curves and retain aggregate status; `divider.perform(0.)?`.
    fn perform(&self, area: f64) -> Result<bool, Failure> {
        perform_face_division(self, area)
    }
    /// Split the support surface and compose its patch faces; `divider.split_surface(0.)?`.
    fn split_surface(&self, area: f64) -> Result<bool, Failure> {
        surface::split_surface(self, area)
    }
    /// Split every boundary wire through its configured policy; `divider.split_curves()?`.
    fn split_curves(&self) -> Result<bool, Failure> {
        curves::split_curves(self)
    }
    /// Virtual surface criterion selection; `divider.split_surface_tool()`.
    fn split_surface_tool(&self) -> Handle<dyn SplitSurface> {
        self.face_divide_data()
            .with(|fields| fields.surface_tool.clone())
    }
    /// Virtual wire criterion selection; `divider.wire_divide_tool()`.
    fn wire_divide_tool(&self) -> Handle<dyn WireDivider> {
        self.face_divide_data().wire_divide_tool()
    }
}
impl std::ops::Deref for dyn FaceDivider {
    type Target = FaceDivideData;
    fn deref(&self) -> &Self::Target {
        self.face_divide_data()
    }
}

pub(super) fn perform_face_division(
    tool: &(impl FaceDivider + ?Sized),
    area: f64,
) -> Result<bool, Failure> {
    let state = tool.face_divide_data();
    state.with_mut(|fields| fields.status = 0);
    let face = state.with(|fields| fields.face.clone());
    if face.is_null() {
        return Ok(false);
    }
    let base = tool.face_divide_base();
    if base.context().is_null() {
        base.set_context(&Handle::new(crate::shape_build::ShapeBuildReShape::new()));
    }
    state.with_mut(|fields| fields.result = face.into_shape());
    tool.split_surface(area)?;
    tool.split_curves()?;
    Ok(state.with(|fields| ShapeExtend::decode_status(fields.status, ShapeExtendStatus::Done)))
}

parent_interface!(dyn FaceDivider, ShapeUpgradeFaceDivide, [dyn UpgradeTool]);

/// Divides a face using configurable surface and wire criteria.
///
/// ```
/// use martensite_tkbrep::topods::Face;
/// use martensite_tkshhealing::shape_upgrade::ShapeUpgradeFaceDivide;
/// let divider = ShapeUpgradeFaceDivide::new(Face::default());
/// assert!(!divider.perform(0.0)?);
/// # Ok::<(), martensite_tkernel::standard::Failure>(())
/// ```
#[derive(Default)]
pub struct ShapeUpgradeFaceDivide {
    state: FaceDivideData,
}
impl_transient!(ShapeUpgradeFaceDivide, "ShapeUpgrade_FaceDivide", dyn UpgradeTool, interfaces:[dyn FaceDivider, dyn UpgradeTool]);
impl FaceDivider for ShapeUpgradeFaceDivide {
    fn face_divide_data(&self) -> &FaceDivideData {
        &self.state
    }
}
impl std::ops::Deref for ShapeUpgradeFaceDivide {
    type Target = FaceDivideData;
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}
impl ShapeUpgradeFaceDivide {
    /// Create a divider without a face; `ShapeUpgradeFaceDivide::empty()`.
    pub fn empty() -> Self {
        Self::default()
    }
    /// Construct with the face and result initialized to `face`; `ShapeUpgradeFaceDivide::new(face)`.
    pub fn new(face: Face) -> Self {
        let tool = Self::default();
        tool.init(&face);
        tool
    }
    /// Dispatch the configured subdivision policies; `divider.perform(0.)?`.
    pub fn perform(&self, area: f64) -> Result<bool, Failure> {
        FaceDivider::perform(self, area)
    }
}
impl FaceDivideData {
    /// Replace working face and result without clearing statuses; `divider.init(&face)`.
    pub fn init(&self, face: &Face) {
        self.with_mut(|fields| {
            fields.face = face.clone();
            fields.result = face.clone().into_shape();
        });
    }
    /// Return the current intermediate/result shape; `divider.result()`.
    pub fn result(&self) -> Shape {
        self.with(|fields| fields.result.clone())
    }
    /// Query subdivision flags; `divider.status_flag(ShapeExtendStatus::Done)`.
    pub fn status_flag(&self, status: ShapeExtendStatus) -> bool {
        self.with(|fields| ShapeExtend::decode_status(fields.status, status))
    }
    /// Select geometric segmentation; `divider.set_surface_segment_mode(false)`.
    pub fn set_surface_segment_mode(&self, enabled: bool) {
        self.with_mut(|fields| fields.segment = enabled);
    }
    /// Replace the surface criterion, allowing a null tool; `divider.set_split_surface_tool(&tool)`.
    pub fn set_split_surface_tool(&self, tool: &Handle<dyn SplitSurface>) {
        self.with_mut(|fields| fields.surface_tool = tool.clone());
    }
    /// Return the configured wire criterion before any policy override; `state.wire_divide_tool()`.
    pub fn wire_divide_tool(&self) -> Handle<dyn WireDivider> {
        self.with(|fields| fields.wire_tool.clone())
    }
    /// Replace the wire criterion, allowing a null tool; `divider.set_wire_divide_tool(&tool)`.
    pub fn set_wire_divide_tool(&self, tool: &Handle<dyn WireDivider>) {
        self.with_mut(|fields| fields.wire_tool = tool.clone());
    }
}
#[cfg(test)]
#[path = "face_divide/legacy_tests.rs"]
mod legacy_tests;

impl UpgradeTool for ShapeUpgradeFaceDivide {
    fn upgrade_tool_data(&self) -> &UpgradeToolData {
        &self.state.tool
    }
}

impl std::ops::Deref for FaceDivideData {
    type Target = UpgradeToolData;
    fn deref(&self) -> &Self::Target {
        &self.tool
    }
}
