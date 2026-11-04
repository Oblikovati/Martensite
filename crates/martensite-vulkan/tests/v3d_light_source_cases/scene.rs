//! The lit brass box the light source regressions share and their light constructors.
use crate::case_view::viewer_session::DisplayRequest;
use crate::primitive_shapes::box_at;
use crate::session_run::SessionRun;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkservice::graphic3d::{
    Graphic3dCLight, Graphic3dNameOfMaterial, Graphic3dTypeOfShadingModel,
};
use martensite_tkv3d::ais::AisLightSource;
use martensite_tkv3d::v3d::{
    V3dAmbientLight, V3dDirectionalLight, V3dPositionalLight, V3dSpotLight, V3dTypeOfOrientation,
};
use std::error::Error;

/// Clear the lights, set a gray background and shading `model`, an optional
/// white ambient light of `ambient` intensity, then the box `b` (corner, size) shaded in brass
/// and fitted.
pub(super) fn brass_box_scene(
    run: &mut SessionRun,
    model: Graphic3dTypeOfShadingModel,
    ambient: Option<f32>,
    (corner, size): ([f64; 3], [f64; 3]),
) -> Result<(), Box<dyn Error>> {
    let session = run.session();
    session.clear_lights()?;
    session.set_background_color(Color::named(NamedColor::GRAY))?;
    session.set_shading_model(model)?;
    if let Some(intensity) = ambient {
        let light = ambient_light("amblight", NamedColor::WHITE)?;
        light
            .get()
            .ok_or("a live light")?
            .set_intensity(intensity)?;
        session.add_light(&light, None)?;
    }
    let shape = box_at(corner, size)?;
    session.display("b", &shape, &DisplayRequest::default().with_display_mode(1))?;
    session.set_material("b", Graphic3dNameOfMaterial::Brass)?;
    session.fit()?;
    Ok(())
}

fn named(light: Handle<Graphic3dCLight>, name: &str) -> Handle<Graphic3dCLight> {
    light
        .get()
        .expect("a new light")
        .set_name(&AsciiString::from(name));
    light
}

/// An ambient light named `name` of `color`.
pub(super) fn ambient_light(
    name: &str,
    color: NamedColor,
) -> Result<Handle<Graphic3dCLight>, Failure> {
    Ok(named(
        Handle::new(V3dAmbientLight::new(Color::named(color))).cast(),
        name,
    ))
}

/// A spot light with the default parameters.
pub(super) fn spot_light(name: &str) -> Result<Handle<Graphic3dCLight>, Failure> {
    let white = Color::named(NamedColor::WHITE);
    let light = V3dSpotLight::new(
        &Point3::new(0., 0., 0.),
        V3dTypeOfOrientation::XnegYnegZpos,
        white,
    )?;
    Ok(named(Handle::new(light).cast(), name))
}

/// A positional light with the default parameters.
pub(super) fn positional_light(name: &str) -> Result<Handle<Graphic3dCLight>, Failure> {
    let light = V3dPositionalLight::new(&Point3::new(0., 0., 0.), Color::named(NamedColor::WHITE))?;
    Ok(named(Handle::new(light).cast(), name))
}

/// A directional light with the default parameters.
pub(super) fn directional_light(name: &str) -> Result<Handle<Graphic3dCLight>, Failure> {
    let white = Color::named(NamedColor::WHITE);
    let light = V3dDirectionalLight::new(V3dTypeOfOrientation::XposYposZpos, white, false)?;
    Ok(named(Handle::new(light).cast(), name))
}

/// Set the position; `place(&light, [50., 25., 25.])?`.
pub(super) fn place(light: &Handle<Graphic3dCLight>, [x, y, z]: [f64; 3]) -> Result<(), Failure> {
    light
        .get()
        .ok_or_else(null_light)?
        .set_position(&Point3::new(x, y, z))
}

/// Set the direction; `direct(&light, [0., 0., -1.])?`.
pub(super) fn direct(light: &Handle<Graphic3dCLight>, [x, y, z]: [f64; 3]) -> Result<(), Failure> {
    light
        .get()
        .ok_or_else(null_light)?
        .set_direction(&Direction3::new(x, y, z)?)
}

fn null_light() -> Failure {
    martensite_tkernel::standard::exceptions::program_error("a null light; expected a live one")
}

/// Display with no presentation options.
pub(super) fn shown(_: &AisLightSource) -> Result<(), Failure> {
    Ok(())
}
