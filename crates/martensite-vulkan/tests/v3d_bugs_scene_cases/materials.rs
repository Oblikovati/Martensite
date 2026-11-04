//! The standard materials on a shaded shape over a gradient: Gouraud and Phong rasterization,
//! then ray tracing with reflections, anti-aliasing and an environment texture.
use super::prelude::*;
use crate::dataset_shape::read_dataset_shape;
use martensite_tkernel::quantity::Color;
use martensite_tkservice::aspect::AspectGradientFillMethod;
use martensite_tkservice::graphic3d::{
    Graphic3dMaterialAspect, Graphic3dNameOfTextureEnv, Graphic3dTypeOfShadingModel,
};

/// The materials of `testmat` in order, with the dump names of each pass.
macro_rules! material_dumps {
    ($($material:literal),* $(,)?) => {
        const MATERIALS: [&str; 17] = [$($material),*];
        const GOURAUD_DUMPS: [&str; 17] = [$(concat!("bug24855_", $material, ".png")),*];
        const PHONG_DUMPS: [&str; 17] = [$(concat!("bug24855_phong_", $material, ".png")),*];
        const RAY_TRACED_DUMPS: [&str; 17] = [$(concat!("bug24855_rt_", $material, ".png")),*];
    };
}
material_dumps!(
    "brass",
    "bronze",
    "copper",
    "gold",
    "jade",
    "neon_phc",
    "pewter",
    "obsidian",
    "plaster",
    "plastic",
    "satin",
    "silver",
    "steel",
    "stone",
    "chrome",
    "aluminium",
    "charcoal",
);

/// `testmat`: set each material without a redraw, then dump it.
fn dump_materials(run: &mut SessionRun, dumps: &[&'static str; 17]) -> Result<(), Box<dyn Error>> {
    for (material, dump) in MATERIALS.iter().zip(dumps) {
        let name = Graphic3dMaterialAspect::material_from_name(material)
            .ok_or_else(|| format!("material {material:?}; expected a standard material name"))?;
        set_dump_material(run.session(), name)?;
        run.dump(dump)?;
    }
    Ok(())
}

/// The shaded `occ/Top.brep` in a 768x768 view over the B4C8FF-B4B4B4 vertical gradient, lit
/// along (0.577, -0.577, -0.577).
fn material_view() -> Result<SessionRun, Box<dyn Error>> {
    let mut run = SessionRun::with_view_size(768, 768)?;
    println!("{}", run.session().graphics_information());
    let hex = |text: &str| Color::from_hex(text).ok_or_else(|| format!("colour {text:?}"));
    let vertical = AspectGradientFillMethod::Vertical;
    let session = run.session();
    session.set_gradient_background(hex("B4C8FF")?, hex("B4B4B4")?, vertical)?;
    session.set_light_direction(0, [0.577, -0.577, -0.577])?;
    let request = DisplayRequest {
        display_mode: Some(1),
        ..DisplayRequest::default()
    };
    session.display("s", &read_dataset_shape("occ/Top.brep")?, &request)?;
    session.fit()?;
    Ok(run)
}

/// `bug24855`: every standard material rasterized with Gouraud and Phong shading, then ray
/// traced with reflections, FSAA and the `Sky2` environment texture.
pub(crate) fn standard_materials() -> Result<(), Box<dyn Error>> {
    crate::dataset_shape::select_predefined_textures();
    banner(&[
        "========",
        "0024855: Revision of parameters of standard materials",
        "========",
    ]);
    let mut run = material_view()?;
    run.session()
        .set_shading_model(Graphic3dTypeOfShadingModel::Gouraud)?;
    dump_materials(&mut run, &GOURAUD_DUMPS)?;
    run.session()
        .set_shading_model(Graphic3dTypeOfShadingModel::Phong)?;
    dump_materials(&mut run, &PHONG_DUMPS)?;
    run.session().set_ray_tracing(true, true)?;
    run.session()
        .set_texture_env(Graphic3dNameOfTextureEnv::Sky2)?;
    dump_materials(&mut run, &RAY_TRACED_DUMPS)?;
    run.finish()
}

// The source changes each material with -noredraw; only the following dump draws the frame.
fn set_dump_material(
    session: &mut crate::case_view::viewer_session::ViewerSession,
    material: martensite_tkservice::graphic3d::Graphic3dNameOfMaterial,
) -> Result<(), martensite_tkernel::standard::Failure> {
    session.without_updates(|session| session.set_material("s", material))
}

#[cfg(test)]
#[path = "materials_command_controls.rs"]
mod command_controls;
