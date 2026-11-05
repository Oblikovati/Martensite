//! Each wrapper mutates the completed scene at the authored capability boundary.
use super::{Outcome, ViewerSession, dimensions_sample};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::{
    Graphic3dCLight, Graphic3dTypeOfLightSource as LightKind, Graphic3dTypeOfShadingModel,
};

/// Run the sample then enable fixed lighting; `dimensions_fixed_function()?`.
pub(crate) fn dimensions_fixed_function() -> Outcome {
    dimensions_sample("dimensions.png", |run| {
        run.session().set_fixed_function_lighting(true)?;
        Ok(())
    })
}
/// Run the sample then select programmable lighting; `dimensions_programmable()?`.
pub(crate) fn dimensions_programmable() -> Outcome {
    dimensions_sample("dimensionsglsl.png", |run| {
        run.session().set_fixed_function_lighting(false)?;
        Ok(())
    })
}
/// Run the sample before the PBR model and light commands; `dimensions_physically_based()?`.
pub(crate) fn dimensions_physically_based() -> Outcome {
    dimensions_sample("dimensionspbr.png", |run| {
        configure_pbr_tail(run.session(), cfg!(target_os = "macos"))
    })
}
fn configure_pbr_tail(session: &mut ViewerSession, macos_core_request: bool) -> Outcome {
    session.set_fixed_function_lighting(false)?;
    if macos_core_request {
        // -core changes future GL-context creation and disables FFP. No later context is
        // created here; apply its actual live effect without inventing a Vulkan GL profile.
        session.set_fixed_function_lighting(false)?;
    }
    session.set_shading_model(Graphic3dTypeOfShadingModel::Pbr)?;
    session.clear_lights()?;
    session.add_light(&ambient_light()?, None)?;
    session.add_light(&directional_light()?, None)?;
    Ok(())
}
fn ambient_light() -> Result<Handle<Graphic3dCLight>, martensite_tkernel::standard::Failure> {
    let light = Handle::new(Graphic3dCLight::new(LightKind::Ambient));
    light.set_intensity(1.)?;
    Ok(light)
}
fn directional_light() -> Result<Handle<Graphic3dCLight>, martensite_tkernel::standard::Failure> {
    let light = Handle::new(Graphic3dCLight::new(LightKind::Directional));
    light.set_direction_coords(0., 0., -1.)?;
    light.set_headlight(true)?;
    light.set_intensity(1.)?;
    Ok(light)
}

#[cfg(test)]
#[path = "lighting_tail_controls.rs"]
mod controls;
