//! Named headset I/O supplies a deterministic open session; camera synchronization remains real.
use martensite_tkernel::{
    ncollection::{Array1, Mat4, Vec2, Vec3},
    standard::{Failure, Handle, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tkmath::gp::Vector3;
use martensite_tkservice::{aspect::*, graphic3d::Graphic3dArrayOfTriangles, image::ImageTexture};
use std::ffi::c_void;
#[derive(Default)]
pub(super) struct NavigationOpenSession {
    transient: StandardTransient,
    backend: AspectOpenVRSession,
    /// Scripted two-hand input of the XR handler witness; false keeps the backend input.
    hands: bool,
}
martensite_tkernel::impl_transient!(NavigationOpenSession,"NavigationOpenSession",dyn AspectXRSession,transient,interfaces:[dyn AspectXRSession]);
impl AspectXRSession for NavigationOpenSession {
    fn xr_session_base(&self) -> &AspectXRSessionBase {
        // Delegate non-headset behavior to the existing Martensite backend.
        self.backend.xr_session_base()
    }
    fn is_open(&self) -> bool {
        // Simulated headset I/O exercises active XR without replacing cameras.
        true
    }
    fn open(&self) -> bool {
        // Delegate non-headset behavior to the existing Martensite backend.
        self.backend.open()
    }
    fn close(&self) {
        // Delegate non-headset behavior to the existing Martensite backend.
        self.backend.close()
    }
    fn wait_poses(&self) -> bool {
        // Delegate non-headset behavior to the existing Martensite backend.
        self.backend.wait_poses()
    }
    fn recommended_viewport(&self) -> Vec2<i32> {
        // Delegate non-headset behavior to the existing Martensite backend.
        self.backend.recommended_viewport()
    }
    fn eye_to_head_transform(&self, _eye: AspectEye) -> Mat4<f64> {
        // Delegate non-headset behavior to the existing Martensite backend.
        self.backend.eye_to_head_transform(_eye)
    }
    fn projection_matrix(&self, _eye: AspectEye, _z_near: f64, _z_far: f64) -> Mat4<f64> {
        // Delegate non-headset behavior to the existing Martensite backend.
        self.backend.projection_matrix(_eye, _z_near, _z_far)
    }
    fn has_projection_frustums(&self) -> bool {
        // Delegate non-headset behavior to the existing Martensite backend.
        self.backend.has_projection_frustums()
    }
    fn process_events(&self) {
        // Delegate non-headset behavior to the existing Martensite backend.
        self.backend.process_events()
    }
    fn submit_eye(
        &self,
        _texture: *mut c_void,
        _library: AspectGraphicsLibrary,
        _space: AspectColorSpace,
        _eye: AspectEye,
    ) -> bool {
        // Delegate non-headset behavior to the existing Martensite backend.
        self.backend.submit_eye(_texture, _library, _space, _eye)
    }
    fn named_tracked_device(&self, role: AspectXRTrackedDeviceRole) -> i32 {
        if self.hands {
            return xr_hand_device(role);
        }
        // Delegate non-headset behavior to the existing Martensite backend.
        self.backend.named_tracked_device(role)
    }
    fn get_digital_action_data(
        &self,
        action: &AspectXrActionRef,
    ) -> Result<AspectXRDigitalActionData, Failure> {
        if self.hands {
            return Ok(self.scripted_digital(action));
        }
        // Delegate non-headset behavior to the existing Martensite backend.
        self.backend.get_digital_action_data(action)
    }
    fn get_analog_action_data(
        &self,
        action: &AspectXrActionRef,
    ) -> Result<AspectXRAnalogActionData, Failure> {
        if self.hands {
            return Ok(self.scripted_analog(action));
        }
        // Delegate non-headset behavior to the existing Martensite backend.
        self.backend.get_analog_action_data(action)
    }
    fn get_pose_action_data_for_next_frame(
        &self,
        action: &AspectXrActionRef,
    ) -> Result<AspectXRPoseActionData, Failure> {
        // Delegate non-headset behavior to the existing Martensite backend.
        self.backend.get_pose_action_data_for_next_frame(action)
    }
    fn set_tracking_origin(&self, origin: AspectXRSessionTrackingUniverseOrigin) {
        // Delegate non-headset behavior to the existing Martensite backend.
        self.backend.set_tracking_origin(origin)
    }
    fn get_string(&self, _info: AspectXRSessionInfoString) -> AsciiString {
        // Delegate non-headset behavior to the existing Martensite backend.
        self.backend.get_string(_info)
    }
    fn load_render_model_impl(
        &self,
        _device: i32,
        _to_apply: bool,
        _texture: &mut Handle<ImageTexture>,
    ) -> Handle<Graphic3dArrayOfTriangles> {
        // Delegate non-headset behavior to the existing Martensite backend.
        self.backend
            .load_render_model_impl(_device, _to_apply, _texture)
    }
    fn trigger_haptic_vibration_action_impl(
        &self,
        action: &AspectXrActionRef,
        _params: &AspectXRHapticActionData,
    ) -> Result<(), Failure> {
        // Delegate non-headset behavior to the existing Martensite backend.
        self.backend
            .trigger_haptic_vibration_action_impl(action, _params)
    }
}
const XR_HANDS: [AspectXRTrackedDeviceRole; 2] = [
    AspectXRTrackedDeviceRole::RightHand,
    AspectXRTrackedDeviceRole::LeftHand,
];
const XR_HAND_ACTIONS: [(AspectXRGenericAction, AspectXRActionType); 4] = [
    (
        AspectXRGenericAction::InputTrackPadClick,
        AspectXRActionType::InputDigital,
    ),
    (
        AspectXRGenericAction::InputTrackPadPosition,
        AspectXRActionType::InputAnalog,
    ),
    (
        AspectXRGenericAction::InputTriggerClick,
        AspectXRActionType::InputDigital,
    ),
    (
        AspectXRGenericAction::InputTriggerPull,
        AspectXRActionType::InputAnalog,
    ),
];
/// Right hand is device 1, left hand device 2, as `WitnessXrSession` in
/// the recorded controller_xr_slots capture.
fn xr_hand_device(role: AspectXRTrackedDeviceRole) -> i32 {
    match role {
        AspectXRTrackedDeviceRole::RightHand => 1,
        AspectXRTrackedDeviceRole::LeftHand => 2,
        _ => -1,
    }
}
impl NavigationOpenSession {
    /// The XR handler witness session: head at y 1.5, valid hands at (+-0.25, 1.25, -0.5) and
    /// four actions per hand. The right hand holds its trackpad forward and half pulls and
    /// clicks its trigger; the left hand clicks its trackpad to the right.
    pub(super) fn with_hands() -> Self {
        let session = Self {
            hands: true,
            ..Self::default()
        };
        let mut fields = session.backend.xr_session_base().fields.write();
        fields
            .head_pose
            .set_translation(Vector3::new(0.0, 1.5, 0.0));
        fields.tracked_poses = Array1::with_size(3);
        for (device, x) in [(1, 0.25), (2, -0.25)] {
            let pose = &mut fields.tracked_poses[device];
            pose.is_valid_pose = true;
            pose.orientation
                .set_translation(Vector3::new(x, 1.25, -0.5));
        }
        for role in XR_HANDS {
            for (kind, action_type) in XR_HAND_ACTIONS {
                let id = AsciiString::from(format!("{}/{}", role as i32, kind as i32).as_str());
                fields.role_actions[role as usize][kind as i32] =
                    Handle::new(AspectXRAction::new(&id, action_type));
            }
        }
        drop(fields);
        session
    }
    fn is_hand_action(
        &self,
        action: &AspectXrActionRef,
        role: AspectXRTrackedDeviceRole,
        kind: AspectXRGenericAction,
    ) -> bool {
        let expected =
            self.xr_session_base().fields.read().role_actions[role as usize][kind as i32].clone();
        action.object_address() == expected.object_address()
    }
    fn scripted_digital(&self, action: &AspectXrActionRef) -> AspectXRDigitalActionData {
        use AspectXRGenericAction as G;
        use AspectXRTrackedDeviceRole as R;
        let changed = self.is_hand_action(action, R::LeftHand, G::InputTrackPadClick)
            || self.is_hand_action(action, R::RightHand, G::InputTriggerClick);
        AspectXRDigitalActionData {
            is_active: true,
            is_pressed: true,
            is_changed: changed,
            ..Default::default()
        }
    }
    fn scripted_analog(&self, action: &AspectXrActionRef) -> AspectXRAnalogActionData {
        use AspectXRGenericAction as G;
        use AspectXRTrackedDeviceRole as R;
        let vec_xyz = if self.is_hand_action(action, R::RightHand, G::InputTrackPadPosition) {
            Vec3::new(0.0, 0.8, 0.0)
        } else if self.is_hand_action(action, R::LeftHand, G::InputTrackPadPosition) {
            Vec3::new(0.9, 0.1, 0.0)
        } else if self.is_hand_action(action, R::RightHand, G::InputTriggerPull) {
            Vec3::new(0.5, 0.0, 0.0)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        };
        AspectXRAnalogActionData {
            is_active: true,
            vec_xyz,
            ..Default::default()
        }
    }
}
#[test]
fn deterministic_headset_preserves_backend_lifecycle_and_pose_queries() {
    let s = NavigationOpenSession::default();
    assert!(s.is_open());
    assert!(!s.open());
    s.close();
    assert!(!s.wait_poses());
    assert_eq!(s.recommended_viewport(), s.backend.recommended_viewport());
    assert_eq!(s.eye_to_head_transform(AspectEye::Left), Mat4::identity());
    assert_eq!(
        s.projection_matrix(AspectEye::Right, 0.1, 100.0),
        Mat4::identity()
    );
    assert!(s.has_projection_frustums());
    s.process_events();
}
#[test]
fn deterministic_headset_delegates_texture_and_action_validation() {
    let s = NavigationOpenSession::default();
    assert!(!s.submit_eye(
        std::ptr::null_mut(),
        AspectGraphicsLibrary::OpenGL,
        AspectColorSpace::Linear,
        AspectEye::Left
    ));
    assert!(
        s.load_render_model_impl(0, true, &mut Handle::default())
            .is_null()
    );
}

#[test]
fn deterministic_headset_delegates_metadata_and_tracking() {
    let s = NavigationOpenSession::default();
    assert_eq!(s.named_tracked_device(AspectXRTrackedDeviceRole::Head), -1);
    assert!(s.get_string(AspectXRSessionInfoString::Vendor).is_empty());
    s.set_tracking_origin(AspectXRSessionTrackingUniverseOrigin::Seated);
    assert_eq!(
        s.xr_session_base().fields.read().track_origin,
        AspectXRSessionTrackingUniverseOrigin::Seated
    );
}

#[test]
fn deterministic_headset_preserves_null_action_failures() {
    let s = NavigationOpenSession::default();
    assert!(
        s.get_digital_action_data(&AspectXrActionRef::borrowed(&Handle::default()))
            .is_err()
    );
    assert!(
        s.get_analog_action_data(&AspectXrActionRef::borrowed(&Handle::default()))
            .is_err()
    );
    assert!(
        s.get_pose_action_data_for_next_frame(&AspectXrActionRef::borrowed(&Handle::default()))
            .is_err()
    );
    assert!(
        s.trigger_haptic_vibration_action_impl(
            &AspectXrActionRef::borrowed(&Handle::default()),
            &AspectXRHapticActionData::default()
        )
        .is_err()
    );
}
