//! Exercise the additive entry through the existing actual CPU callback backend.
use super::{
    V3dLightName, V3dStereoDumpOptions, V3dView, V3dViewer,
    light_name_callback_fixture::CallbackNameStorage, light_name_callback_tests::callback_view,
};
use martensite_tkernel::standard::Failure;
use martensite_tkservice::{
    graphic3d::{GRAPHIC3D_ZLAYER_ID_BOT_OSD, Graphic3dBufferType, Graphic3dCViewMembers as _},
    image::ImagePixMap,
    null_driver::NullGraphicDriver,
};

#[test]
fn full_overload_keeps_actual_redraw_and_shadow_consumption_order() {
    let control = CallbackNameStorage::new();
    let viewer = V3dViewer::new(&NullGraphicDriver::new_handle()).unwrap();
    let view = callback_view(&viewer, &control);
    let name = V3dLightName::from_c_str(c"sun");
    let result = explicit_shadow_dump(&view, name);
    assert!(!result.unwrap());
    assert_eq!(control.rows.read().as_slice(), ["consumed 0 3 115 117 110"]);
    assert_eq!(
        view.view_base().view().z_layer_target(),
        GRAPHIC3D_ZLAYER_ID_BOT_OSD
    );
    assert!(!view.view_base().view().z_layer_redraw_mode());
}

#[test]
fn full_overload_preserves_null_name_error_and_unfinished_cleanup_state() {
    let control = CallbackNameStorage::new();
    let viewer = V3dViewer::new(&NullGraphicDriver::new_handle()).unwrap();
    let view = callback_view(&viewer, &control);
    let result = explicit_shadow_dump(&view, V3dLightName::null());
    assert_eq!(result.unwrap_err().exception_type(), "Standard_NullObject");
    assert!(control.rows.read().is_empty());
    assert_eq!(view.view_base().view().z_layer_target(), 42);
    assert!(view.view_base().view().z_layer_redraw_mode());
}

fn explicit_shadow_dump(view: &V3dView, name: V3dLightName<'_>) -> Result<bool, Failure> {
    view.to_pix_map_full(
        &ImagePixMap::default(),
        8,
        8,
        Graphic3dBufferType::ShadowMap,
        false,
        42,
        -3,
        V3dStereoDumpOptions::Mono,
        name,
    )
}
