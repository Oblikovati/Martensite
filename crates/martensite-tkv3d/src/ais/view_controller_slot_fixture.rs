//! Fixed CPU inputs retain the witnessed virtual-entrypoint contract.
use super::view_controller_action_scene_fixture::action_mapped_view;
use super::*;
use crate::v3d::{V3dViewSource, V3dViewer};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::{
    aspect::AspectScrollDelta,
    graphic3d::{Graphic3dCameraFocusType, Graphic3dCameraProjection},
    null_driver::NullGraphicDriver,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum ControllerSlot {
    Gravity,
    Navigation,
    Frame,
    Flush,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum ControllerSlotMode {
    Intercept,
    Delegate,
    Reenter,
    TypedFailure,
    UnrelatedFailure,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum ControllerSlotOperation {
    Direct,
    OrbitConsumer,
    FrameConsumer,
    FlushTrue,
    FlushFalse,
    DirectFlushFalse,
}
#[derive(Clone, Copy, Debug)]
pub(super) struct ControllerSlotCase {
    pub(super) name: &'static str,
    pub(super) slot: ControllerSlot,
    pub(super) mode: ControllerSlotMode,
    pub(super) operation: ControllerSlotOperation,
}
pub(super) struct ControllerSlotScene {
    _viewer: Handle<V3dViewer>,
    pub(super) view: Handle<dyn V3dViewSource>,
    pub(super) context: Handle<AisInteractiveContext>,
    pub(super) controller: AisViewController,
}
/// Construct actual CPU inputs once per case; `controller_slot_scene()`.
pub(super) fn controller_slot_scene() -> ControllerSlotScene {
    let viewer = V3dViewer::new(&NullGraphicDriver::new_handle()).unwrap();
    let view = action_mapped_view(&viewer);
    let context = AisInteractiveContext::new(&viewer).unwrap();
    configure_slot_camera(&view);
    let controller = seeded_slot_controller();
    ControllerSlotScene {
        _viewer: viewer,
        view,
        context,
        controller,
    }
}
fn seeded_slot_controller() -> AisViewController {
    let mut controller = AisViewController::new().unwrap();
    controller.set_show_rotate_center(false);
    controller
        .ui_input
        .zoom_actions
        .append(AspectScrollDelta::with_delta(11.0, 0));
    controller
        .render_input
        .zoom_actions
        .append(AspectScrollDelta::with_delta(22.0, 0));
    controller
}
fn configure_slot_camera(view: &Handle<dyn V3dViewSource>) {
    let camera = view.camera();
    camera
        .set_eye_and_center(&Point3::new(0.0, 0.0, 200.0), &Point3::default())
        .unwrap();
    camera.set_projection_type(Graphic3dCameraProjection::Orthographic);
    // Native Graphic3d camera defaults use relative focus1; mapped-view reuse sets.25.
    camera.set_z_focus(Graphic3dCameraFocusType::Relative, 1.0);
}
/// Observe a configured return rather than a camera implementation; `literal_slot_walk()`.
pub(super) fn literal_slot_walk() -> AisWalkDelta {
    let mut walk = AisWalkDelta::default();
    walk.set_defined(true);
    walk[AisWalkTranslation::Forward] = AisWalkPart {
        value: 0.25,
        pressure: 0.5,
        duration: 0.125,
    };
    walk
}
/// Keep the fixed catalog independent of returned order; `controller_slot_case("gravity-direct")`.
pub(super) fn controller_slot_case(name: &str) -> ControllerSlotCase {
    let row = SLOT_CASES
        .iter()
        .find(|row| row.0 == name)
        .unwrap_or_else(|| panic!("slot case={name:?}; expected fixed controller scenario"));
    ControllerSlotCase {
        name: row.0,
        slot: row.1,
        mode: row.2,
        operation: row.3,
    }
}
use ControllerSlot as S;
use ControllerSlotMode as M;
use ControllerSlotOperation as O;
const SLOT_CASES: [(&str, S, M, O); 25] = [
    ("gravity-direct", S::Gravity, M::Intercept, O::Direct),
    ("gravity-orbit", S::Gravity, M::Intercept, O::OrbitConsumer),
    ("navigation-direct", S::Navigation, M::Intercept, O::Direct),
    (
        "navigation-frame",
        S::Navigation,
        M::Intercept,
        O::FrameConsumer,
    ),
    ("frame-direct", S::Frame, M::Intercept, O::Direct),
    ("frame-from-flush", S::Frame, M::Intercept, O::FlushTrue),
    (
        "frame-not-from-flush-false",
        S::Frame,
        M::Intercept,
        O::FlushFalse,
    ),
    ("flush-direct", S::Flush, M::Intercept, O::Direct),
    ("gravity-delegate", S::Gravity, M::Delegate, O::Direct),
    ("navigation-delegate", S::Navigation, M::Delegate, O::Direct),
    ("frame-delegate", S::Frame, M::Delegate, O::Direct),
    ("flush-delegate", S::Flush, M::Delegate, O::Direct),
    ("gravity-reenter", S::Gravity, M::Reenter, O::Direct),
    ("navigation-reenter", S::Navigation, M::Reenter, O::Direct),
    ("frame-reenter", S::Frame, M::Reenter, O::Direct),
    ("flush-reenter", S::Flush, M::Reenter, O::Direct),
    ("gravity-typed", S::Gravity, M::TypedFailure, O::Direct),
    (
        "navigation-typed",
        S::Navigation,
        M::TypedFailure,
        O::Direct,
    ),
    ("frame-typed", S::Frame, M::TypedFailure, O::Direct),
    ("flush-typed", S::Flush, M::TypedFailure, O::Direct),
    (
        "gravity-unrelated",
        S::Gravity,
        M::UnrelatedFailure,
        O::Direct,
    ),
    (
        "navigation-unrelated",
        S::Navigation,
        M::UnrelatedFailure,
        O::Direct,
    ),
    ("frame-unrelated", S::Frame, M::UnrelatedFailure, O::Direct),
    ("flush-unrelated", S::Flush, M::UnrelatedFailure, O::Direct),
    (
        "flush-direct-false",
        S::Flush,
        M::Intercept,
        O::DirectFlushFalse,
    ),
];
