//! Null-driver controls check real erased registration without a window or GPU.
use super::*;
use martensite_tkservice::graphic3d::Graphic3dCViewMembers as _;
use martensite_tkservice::null_driver::NullGraphicDriver;
use martensite_tkv3d::v3d::V3dViewer;

#[test]
fn erased_registration_preserves_shaded_mode_and_second_view_only_affinity() {
    let viewer = V3dViewer::new(&NullGraphicDriver::new_handle()).unwrap();
    let context = AisInteractiveContext::new(&viewer).unwrap();
    let first = viewer.create_view().unwrap();
    let second = viewer.create_view().unwrap();
    let object: Handle<dyn AisInteractiveObject> =
        AisShape::new(&box_sized([1., 2., 3.]).unwrap()).cast();
    object.set_display_mode(1);
    erased_load_affinity(&context, &object, &first, &second).unwrap();
    assert_eq!(object.display_status(), PrsMgrDisplayStatus::Erased);
    assert_eq!(object.display_mode(), 1);
    assert!(!context.is_displayed(&object));
    let affinity = object.view_affinity();
    assert!(!affinity.is_visible(first.get().unwrap().view().identification()));
    assert!(affinity.is_visible(second.get().unwrap().view().identification()));
}
