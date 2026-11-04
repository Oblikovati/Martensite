//! Set the image-owned shading aspect before the first display, as both QA commands do.
use super::*;
use crate::case_view::viewer_session::ViewerSession;
use crate::primitive_shapes::box_sized;
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkservice::graphic3d::{
    Graphic3dAspectFillArea3d, Graphic3dAspectsMembers as _, Graphic3dAspectsRenderMembers as _,
    Graphic3dTexture2D, Graphic3dTextureRootMembers as _, Graphic3dTypeOfShadingModel,
};
use martensite_tkservice::image::ImageAlienPixMap;
use martensite_tkv3d::ais::AisShape;
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;

pub(super) fn display_decoded(
    session: &mut ViewerSession,
    image: Option<Handle<ImageAlienPixMap>>,
) -> Outcome {
    if let Some(image) = image {
        textured_image(session, image)?;
    }
    Ok(())
}

pub(super) fn textured_image(
    session: &mut ViewerSession,
    image: Handle<ImageAlienPixMap>,
) -> Outcome {
    let shape = box_sized([100. * image.ratio(), 100., 1.])?;
    let object = AisShape::new(&shape);
    object.set_display_mode(1);
    let drawer = object.attributes();
    drawer.setup_own_shading_aspect(&Handle::default())?;
    let aspect = drawer.shading_aspect().aspect();
    set_image_aspect(&aspect, image)?;
    session.display_object("t", object.cast())?;
    Ok(())
}

fn set_image_aspect(
    aspect: &Graphic3dAspectFillArea3d,
    image: Handle<ImageAlienPixMap>,
) -> Result<(), Failure> {
    let top_down = image.is_top_down();
    let texture = Handle::new(Graphic3dTexture2D::from_pix_map(image.cast()));
    aspect.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    aspect.set_texture_map_on(true);
    aspect.set_texture_map(texture.clone().cast());
    if top_down {
        texture.get_params().set_translation(Vec2::new(0., -1.));
        texture.get_params().set_scale(Vec2::new(1., -1.));
    }
    Ok(())
}

#[cfg(test)]
mod controls {
    use super::*;
    use martensite_tkservice::image::ImageFormat;

    #[test]
    fn successful_command_without_decoded_image_keeps_the_previous_named_object() {
        use martensite_tkservice::null_driver::NullGraphicDriver;
        let driver = NullGraphicDriver::new_handle();
        let mut session = ViewerSession::with_graphic_driver(&driver, 37, 19).unwrap();
        let image = Handle::new(ImageAlienPixMap::default());
        assert!(image.init_zero(ImageFormat::Rgb, 2, 1, 0, 0));
        display_decoded(&mut session, Some(image)).unwrap();
        let owner = session.object("t").unwrap().object_address();
        display_decoded(&mut session, None).unwrap();
        assert_eq!(session.object("t").unwrap().object_address(), owner);
        session.fit().unwrap();
    }

    #[test]
    fn actual_image_orientation_controls_texture_translation_and_scale() {
        for top_down in [false, true] {
            let image = Handle::new(ImageAlienPixMap::default());
            assert!(image.init_zero(ImageFormat::Rgb, 2, 1, 0, 0));
            image.set_top_down(top_down);
            let aspect = Graphic3dAspectFillArea3d::default();
            set_image_aspect(&aspect, image).unwrap();
            let params = aspect.texture_map().get_params();
            assert_eq!(aspect.shading_model(), Graphic3dTypeOfShadingModel::Unlit);
            assert_eq!(
                params.translation(),
                Vec2::new(0., if top_down { -1. } else { 0. })
            );
            assert_eq!(
                params.scale(),
                Vec2::new(1., if top_down { -1. } else { 1. })
            );
        }
    }
}
