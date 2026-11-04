//! Expose the actual invocation-local controller timeline to custom presentation callbacks.
use super::ViewerSession;
use martensite_tkernel::standard::Handle;
use martensite_tkv3d::ais::AisAnimationSource;
impl ViewerSession {
    /// Retain this controller's actual object animation; `let timeline = session.objects_animation();`.
    pub fn objects_animation(&self) -> Handle<dyn AisAnimationSource> {
        self.controller.objects_animation().clone()
    }
}
#[test]
fn tutorial_animation_is_the_real_controller_timeline() {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    assert_eq!(
        &session.objects_animation(),
        session.controller.objects_animation()
    );
    assert!(!session.objects_animation().is_null());
}
