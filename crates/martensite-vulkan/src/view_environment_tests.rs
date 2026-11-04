use super::*;
use crate::view_output_modes::OutputModeRetainedView;
use martensite_tkservice::image::{ImageFormat, ImagePixMap};
use martensite_tkservice::null_driver::NullGraphicDriver;

fn retained() -> RenderRetainedView {
    let driver = NullGraphicDriver::new_handle();
    let owner = Handle::new(Graphic3dStructureManager::new(&driver).unwrap());
    RenderRetainedView::new(OutputModeRetainedView::new(
        Graphic3dRetainedView::new(&owner, &[]).unwrap(),
    ))
}

pub(super) fn cube_map(shade: u8) -> Handle<dyn Graphic3dCubeMap> {
    let sides: Vec<_> = (0..6u8)
        .map(|side| {
            let image = Handle::new(ImagePixMap::default());
            let texels = vec![shade.wrapping_add(side); 2 * 2 * 3];
            assert!(image.init_wrapper(ImageFormat::Rgb, Some(texels), 2, 2, 6));
            image.set_top_down(true);
            image
        })
        .collect();
    Handle::new(Graphic3dCubeMapSeparate::from_images(&sides).unwrap()).cast()
}

fn request(view: &RenderRetainedView) -> bool {
    view.environment.read().request
}

#[test]
fn a_cube_map_background_with_pbr_update_becomes_the_ibl_source() {
    let view = retained();
    let cube = cube_map(10);
    view.set_background_image(&cube.clone().cast(), true);
    let fields = view.cview_base().fields.read();
    assert_eq!(fields.background_type, Graphic3dTypeOfBackground::Cubemap);
    assert_eq!(
        fields.cube_map_background.object_address(),
        cube.object_address()
    );
    assert_eq!(fields.cube_map_ibl.object_address(), cube.object_address());
    drop(fields);
    assert!(request(&view));
}

#[test]
fn disabling_ibl_requests_a_cleared_environment_only_once() {
    let view = retained();
    view.set_background_image(&cube_map(10).cast(), true);
    view.environment.write().request = false;
    view.set_image_based_lighting(false);
    assert!(view.cview_base().fields.read().cube_map_ibl.is_null());
    assert!(request(&view));
    view.environment.write().request = false;
    view.set_image_based_lighting(false);
    assert!(!request(&view));
    view.set_image_based_lighting(true);
    assert!(request(&view), "the background cube map is lit again");
}

#[test]
fn ibl_needs_a_cubemap_background() {
    let view = retained();
    view.set_image_based_lighting(true);
    assert!(view.cview_base().fields.read().cube_map_ibl.is_null());
    assert!(!request(&view));
}

#[test]
fn a_null_image_clears_a_cubemap_background() {
    let view = retained();
    view.set_background_image(&cube_map(10).cast(), false);
    view.set_background_image(&Handle::default(), false);
    let fields = view.cview_base().fields.read();
    assert_eq!(fields.background_type, Graphic3dTypeOfBackground::None);
    assert!(fields.cube_map_ibl.is_null());
}

#[test]
fn pbr_frames_consume_the_request_and_share_the_background_source() {
    let view = retained();
    view.set_background_image(&cube_map(10).cast(), true);
    view.cview_base().fields.write().render_params.shading_model = Graphic3dTypeOfShadingModel::Pbr;
    let first = view.frame_environment(&**view).unwrap().unwrap();
    let pbr = first.pbr.unwrap();
    assert!(pbr.request);
    assert_eq!((pbr.pow2_size, pbr.spec_levels), (9, 6));
    assert_eq!(pbr.source, first.background.map(|bg| bg.source));
    let second = view.frame_environment(&**view).unwrap().unwrap();
    assert!(!second.pbr.unwrap().request);
}

#[test]
fn non_pbr_frames_keep_the_request_pending() {
    let view = retained();
    view.set_background_image(&cube_map(10).cast(), true);
    let environment = view.frame_environment(&**view).unwrap().unwrap();
    assert!(environment.pbr.is_none());
    assert!(environment.background.is_some());
    assert!(request(&view));
}

#[test]
fn a_pending_skydome_becomes_the_background_texture() {
    let view = retained();
    view.set_background_image(&cube_map(10).cast(), false);
    view.cview_base().fields.write().to_update_skydome = true;
    let environment = view.frame_environment(&**view).unwrap().unwrap();
    let source = environment.background.unwrap().source;
    assert!(matches!(source, EnvironmentCubeSource::Skydome(params) if params.size == 512));
    assert!(!view.cview_base().fields.read().to_update_skydome);
}
