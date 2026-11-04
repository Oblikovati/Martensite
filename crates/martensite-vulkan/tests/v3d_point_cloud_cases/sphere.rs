//! Complete point-cloud modes of the actual triangulated radius10 sphere.
use super::{cloud::node_cloud, sensitive_box::meshed_sphere_face};
use crate::case_view::viewer_session::{TextureImage, ViewerSession};
use crate::session_run::{SessionRun, banner};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkservice::graphic3d::{
    GRAPHIC3D_ARRAY_FLAGS_NONE, GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR,
    GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL, GRAPHIC3D_ARRAY_FLAGS_VERTEX_TEXEL,
    Graphic3dNameOfMaterial, Graphic3dNameOfTexture2D, Graphic3dTypeOfShadingModel,
};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;
use std::error::Error;
type Outcome = Result<(), Box<dyn Error>>;

const SPHERE_BANNER: &[&str] = &[
    "========",
    "Point cloud object from triangulated sphere",
    "========",
];

/// Replay every replacement/mode transition and all seven captures; `colored_textured_sphere()?`.
pub(crate) fn colored_textured_sphere() -> Outcome {
    crate::dataset_shape::select_predefined_textures();
    banner(SPHERE_BANNER);
    let shape = meshed_sphere_face()?;
    let mut run = SessionRun::new()?;
    green_cloud(run.session(), &shape)?;
    run.dump("sphere_green.png")?;
    replace_cloud(run.session(), &shape, GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR)?;
    run.dump("sphere_rand.png")?;
    textured_modes(&mut run, &shape)?;
    replace_cloud(run.session(), &shape, GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL)?;
    run.session()
        .set_material("p", Graphic3dNameOfMaterial::Copper)?;
    run.dump("sphere_copper.png")?;
    run.session().move_cursor(200, 200)?;
    run.dump("sphere.png")?;
    run.finish()
}

/// Diagnostic prefix only, not completion evidence for the seven-image original.
/// `green_vertex_output_prefix()?` uses exactly the original geometry/view helpers.
pub(crate) fn green_vertex_output_prefix() -> Outcome {
    banner(SPHERE_BANNER);
    let shape = meshed_sphere_face()?;
    let mut run = SessionRun::new()?;
    green_cloud(run.session(), &shape)?;
    run.dump("sphere_green.png")?;
    run.finish()
}

fn replace_cloud(session: &mut ViewerSession, shape: &Shape, flags: i32) -> Outcome {
    let cloud = node_cloud(shape, flags)?;
    session.display_object("p", cloud.cast())?;
    Ok(())
}

fn green_cloud(session: &mut ViewerSession, shape: &Shape) -> Outcome {
    session.clear()?;
    session.set_display_mode(1)?;
    session.set_axonometric()?;
    replace_cloud(session, shape, GRAPHIC3D_ARRAY_FLAGS_NONE)?;
    session.set_color("p", Color::named(NamedColor::GREEN))?;
    session.fit()?;
    session.rotate(0.2, 0., 0.)?;
    Ok(())
}

fn textured_modes(run: &mut SessionRun, shape: &Shape) -> Outcome {
    let flags = GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL | GRAPHIC3D_ARRAY_FLAGS_VERTEX_TEXEL;
    replace_cloud(run.session(), shape, flags)?;
    run.session().set_texture(
        "p",
        TextureImage::Predefined(Graphic3dNameOfTexture2D::Matra),
    )?;
    for (model, image) in [
        (Graphic3dTypeOfShadingModel::Gouraud, "sphere_tex3.png"),
        (Graphic3dTypeOfShadingModel::Phong, "sphere_tex2.png"),
        (Graphic3dTypeOfShadingModel::Unlit, "sphere_tex1.png"),
    ] {
        cloud_shading(run.session(), model)?;
        run.dump(image)?;
    }
    Ok(())
}

fn cloud_shading(session: &ViewerSession, model: Graphic3dTypeOfShadingModel) -> Outcome {
    let object = session
        .object("p")
        .cloned()
        .ok_or("cloud p missing; expected current source point cloud")?;
    session.update_context(|context| {
        if object.attributes().set_shading_model(model, true)? {
            context.redisplay(&object, false, false)?;
        } else {
            object.synchronize_aspects();
        }
        Ok(())
    })?;
    Ok(())
}

#[cfg(test)]
mod controls {
    use super::*;
    use martensite_tkernel::standard::Handle;
    use martensite_tkservice::{
        graphic3d::{Graphic3dArrayOfPoints, Graphic3dAspectsMembers as _},
        null_driver::NullGraphicDriver,
    };
    use martensite_tkv3d::ais::{AisPointCloud, AisPointCloudSource as _};

    fn null_cloud_session() -> ViewerSession {
        let mut session =
            ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 409, 409).unwrap();
        let points =
            Handle::new(Graphic3dArrayOfPoints::new(1, GRAPHIC3D_ARRAY_FLAGS_NONE).unwrap());
        points.add_vertex(0., 0., 0.).unwrap();
        let cloud = AisPointCloud::new().unwrap();
        cloud.set_points(&points).unwrap();
        session.display_object("p", cloud.cast()).unwrap();
        session
    }

    #[test]
    fn shading_modes_create_then_reuse_the_actual_cloud_owned_fill() {
        let session = null_cloud_session();
        cloud_shading(&session, Graphic3dTypeOfShadingModel::Gouraud).unwrap();
        let object = session.object("p").unwrap();
        let fill = object.attributes().shading_aspect().aspect();
        for model in [
            Graphic3dTypeOfShadingModel::Phong,
            Graphic3dTypeOfShadingModel::Unlit,
        ] {
            cloud_shading(&session, model).unwrap();
            assert_eq!(object.attributes().shading_aspect().aspect(), fill);
            assert_eq!(fill.shading_model(), model);
        }
    }
}
