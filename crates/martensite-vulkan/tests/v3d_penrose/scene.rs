//! Literal perspective camera, rendering flags, lights, color sequence and text placement.
use crate::case_view::viewer_session::{
    DisplayRequest, TextLabelRequest, ViewParameters, ViewerSession,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::{Failure, Handle},
    tcollection::AsciiString,
};
use martensite_tkmath::gp::Direction3;
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::v3d::{V3dDirectionalLight, V3dTypeOfOrientation};

pub(super) fn camera(session: &ViewerSession) -> Result<(), Failure> {
    session.set_background_color(Color::named(NamedColor::WHITE))?;
    session.set_ray_tracing(false, true)?;
    let view = session.view().view_base();
    view.change_rendering_params(&mut |p| p.ray_tracing.is_shadow_enabled = false);
    session.set_projection(Graphic3dCameraProjection::Perspective)?;
    view.camera().set_fov_y(25.);
    session.set_view_parameters(&ViewParameters {
        eye: Some([14., -14., 6.8]),
        up: Some([0., 0., 1.]),
        at: Some([4., -4., 0.]),
        scale: Some(70.),
        ..Default::default()
    })?;
    session.set_display_mode(1)?;
    Ok(())
}

pub(super) fn lights(session: &mut ViewerSession) -> Result<(), Failure> {
    let viewer = session.view().view_base().viewer();
    viewer.set_default_lights()?;
    for (name, direction) in [
        ("dirlight1", [1., -2., -10.]),
        ("dirlight2", [0., -10., 0.]),
    ] {
        let light = directional(name, direction)?;
        session.add_light(&light, None)?;
    }
    Ok(())
}

fn directional(name: &str, [x, y, z]: [f64; 3]) -> Result<Handle<Graphic3dCLight>, Failure> {
    let light: Handle<Graphic3dCLight> = Handle::new(V3dDirectionalLight::new(
        V3dTypeOfOrientation::XposYposZpos,
        Color::named(NamedColor::WHITE),
        true,
    )?)
    .cast();
    light.set_name(&AsciiString::from(name));
    light.set_direction(&Direction3::new(x, y, z)?)?;
    Ok(light)
}

pub(super) fn display(session: &mut ViewerSession, shapes: &[Shape]) -> Result<(), Failure> {
    let names: Vec<_> = (1..=10).map(|i| format!("b{i}")).collect();
    let named: Vec<_> = names
        .iter()
        .zip(shapes)
        .map(|(name, shape)| (name.as_str(), shape))
        .collect();
    session.display_many(&named, &DisplayRequest::default())?;
    for (name, hex) in names.iter().zip(COLORS) {
        session.set_color(
            name,
            Color::from_hex(hex).expect("literal six-digit source color"),
        )?;
    }
    for name in &names {
        session.set_material(name, Graphic3dNameOfMaterial::Plastified)?;
    }
    label(session)
}

pub(super) const COLORS: [&str; 10] = [
    "DD0029", "F6DD00", "98F918", "E9007A", "007ABC", "93007A", "EE9800", "00B489", "00A47A",
    "007ABC",
];
fn label(session: &mut ViewerSession) -> Result<(), Failure> {
    let request = TextLabelRequest {
        position: [0., -6., -2.],
        color: Some(Color::named(NamedColor::BLACK)),
        horizontal: Some(Graphic3dHorizontalTextAlignment::Left),
        vertical: Some(Graphic3dVerticalTextAlignment::Bottom),
        angle_degrees: Some(0.),
        zoomable: Some(false),
        height: Some(40.),
        ..TextLabelRequest::new("Which\nbox\nis\ncloser\nto\nyou?")
    };
    session.draw_text("label", &request)
}
