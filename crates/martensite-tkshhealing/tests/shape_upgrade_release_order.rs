//! A default native destructor releases members in reverse declaration order:
//! the face tool before messages, with the reshape context still alive for messages.
use martensite_tkernel::{
    impl_transient,
    standard::{Handle, SharedState, WeakHandle},
};
use martensite_tkshhealing::{
    shape_build::ShapeBuildReShape,
    shape_extend::{BasicMsgRegistrator, ShapeExtendBasicMsgRegistrator},
    shape_upgrade::{ShapeDivider, ShapeUpgradeFaceDivide, ShapeUpgradeShapeDivide},
};
use std::sync::Arc;

struct ReleaseObserver {
    parent: ShapeExtendBasicMsgRegistrator,
    context: WeakHandle<ShapeBuildReShape>,
    face_tool: WeakHandle<ShapeUpgradeFaceDivide>,
    observations: Arc<SharedState<Vec<(bool, bool)>>>,
}
impl_transient!(ReleaseObserver, "ReleaseObserver", ShapeExtendBasicMsgRegistrator, parent,
    interfaces: [dyn BasicMsgRegistrator]);
impl BasicMsgRegistrator for ReleaseObserver {}
impl Drop for ReleaseObserver {
    fn drop(&mut self) {
        self.observations.write().push((
            !self.context.upgrade().is_null(),
            !self.face_tool.upgrade().is_null(),
        ));
    }
}
fn attach_observer(
    divider: &ShapeUpgradeShapeDivide,
    observations: Arc<SharedState<Vec<(bool, bool)>>>,
) -> Handle<ReleaseObserver> {
    let observer = Handle::new(ReleaseObserver {
        parent: ShapeExtendBasicMsgRegistrator::default(),
        context: divider.context().downgrade(),
        face_tool: divider.split_face_tool().downgrade(),
        observations,
    });
    divider.set_msg_registrator(&observer.cast());
    observer
}
#[test]
fn message_destruction_sees_live_context_after_face_tool_release() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let divider = ShapeUpgradeShapeDivide::empty();
    attach_observer(&divider, observations.clone());
    assert!(observations.read().is_empty());
    drop(divider);
    assert_eq!(*observations.read(), vec![(true, false)]);
}

#[test]
fn external_face_owner_survives_divider_release_without_retaining_context() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let divider = ShapeUpgradeShapeDivide::empty();
    let face_tool = divider.split_face_tool();
    let context = divider.context().downgrade();
    attach_observer(&divider, observations.clone());
    drop(divider);
    assert_eq!(*observations.read(), vec![(true, true)]);
    assert_eq!(face_tool.ref_count(), 1);
    assert!(context.upgrade().is_null());
}

#[test]
fn external_message_owner_defers_its_destructor_past_all_divider_members() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let divider = ShapeUpgradeShapeDivide::empty();
    let observer = attach_observer(&divider, observations.clone());
    drop(divider);
    assert!(observations.read().is_empty());
    drop(observer);
    assert_eq!(*observations.read(), vec![(false, false)]);
}
