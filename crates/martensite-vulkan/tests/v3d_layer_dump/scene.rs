use super::Outcome;
use crate::case_view::viewer_session::{ViewParameters, ViewerSession};
#[cfg(test)]
use crate::hatch_surface::scene::SourceViewOutput;
use crate::primitive_shapes::box_at;
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::Handle,
    tcollection::AsciiString,
};
use martensite_tkmath::gp::Direction3;
use martensite_tkservice::graphic3d::{Graphic3dCLight, Graphic3dZLayerSettings};
use martensite_tkv3d::{
    ais::AisShape,
    prs_mgr::{PrsMgrPresentableObject, PrsMgrPresentableObjectMembers},
    v3d::{V3dDirectionalLight, V3dTypeOfOrientation},
};
const BOXES: [([f64; 3], [f64; 3]); 7] = [
    ([20., 0., 0.], [10.; 3]),
    ([-20., 0., 0.], [10.; 3]),
    ([0., -10., 0.], [5.; 3]),
    ([0., 5., 0.], [5.; 3]),
    ([0., 0., 5.], [3.; 3]),
    ([0., 0., -5.], [7.; 3]),
    ([0., -5., -10.], [3.; 3]),
];
const NAMES: [&str; 7] = ["b1", "b2", "b3", "b4", "b5", "b6", "b7"];
const COLORS: [NamedColor; 7] = [
    NamedColor::RED3,
    NamedColor::GREEN3,
    NamedColor::BLUE3,
    NamedColor::ORANGE3,
    NamedColor::YELLOW3,
    NamedColor::SALMON3,
    NamedColor::PURPLE3,
];

pub(super) fn prepare(session: &mut ViewerSession) -> Outcome<[i32; 7]> {
    let layers = [new_layer(session)?, new_layer(session)?, 0, -2, -3, -4, -5];
    display_boxes(session, layers)?;
    session.fit()?;
    authored_camera(session)?;
    for (name, color) in NAMES.into_iter().zip(COLORS) {
        session.set_color(name, Color::named(color))?;
    }
    Ok(layers)
}

fn display_boxes(session: &mut ViewerSession, layers: [i32; 7]) -> Outcome {
    for (index, (corner, size)) in BOXES.into_iter().enumerate() {
        let object = AisShape::new(&box_at(corner, size)?);
        object.set_z_layer(layers[index]);
        object.set_display_mode(1);
        session.display_object(NAMES[index], object.cast())?;
    }
    Ok(())
}

fn authored_camera(session: &ViewerSession) -> Outcome {
    session.set_view_parameters(&ViewParameters {
        scale: Some(15.),
        proj: Some([0.57735, -0.57735, 0.57735]),
        up: Some([-0.1, 0.1, 0.1]),
        at: Some([0.; 3]),
        ..Default::default()
    })?;
    Ok(())
}

fn new_layer(session: &ViewerSession) -> Outcome<i32> {
    let mut settings = Graphic3dZLayerSettings::default();
    settings.set_enable_depth_test(true);
    settings.set_enable_depth_write(true);
    settings.set_clear_depth(false);
    let id = session
        .context()
        .current_viewer()
        .add_z_layer(&settings)?
        .ok_or("no free layer; expected an allocated layer")?;
    session.view().redraw();
    Ok(id)
}

pub(super) fn lights(session: &mut ViewerSession) -> Outcome {
    session.clear_lights()?;
    for (name, direction) in [("v1", [-1., 0., -0.5]), ("v2", [-1., 0.1, 0.])] {
        let light = directional(name, direction)?;
        session.add_light(&light, None)?;
    }
    Ok(())
}

fn directional(name: &str, [x, y, z]: [f64; 3]) -> Outcome<Handle<Graphic3dCLight>> {
    let light: Handle<Graphic3dCLight> = Handle::new(V3dDirectionalLight::new(
        V3dTypeOfOrientation::XposYposZpos,
        Color::named(NamedColor::WHITE),
        false,
    )?)
    .cast();
    light.set_name(&AsciiString::from(name));
    light.set_intensity(1.)?;
    light.set_direction(&Direction3::new(x, y, z)?)?;
    light.set_cast_shadows(true)?;
    Ok(light)
}

#[test]
fn actual_boxes_keep_shaded_mode_and_distinct_requested_layers() {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    let layers = prepare(&mut session).unwrap();
    assert_ne!(layers[0], layers[1]);
    for (name, layer) in NAMES.into_iter().zip(layers) {
        let object = session.object(name).unwrap();
        assert_eq!(object.z_layer(), layer);
        assert_eq!(object.display_mode(), 1);
    }
    assert!((session.view().scale() - 15.).abs() <= 15. * f64::EPSILON);
    session.close();
}

#[test]
fn authored_layer_depth_contract_does_not_inherit_clear_depth_default() {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    let id = new_layer(&session).unwrap();
    let settings = session
        .context()
        .current_viewer()
        .z_layer_settings(id)
        .unwrap();
    assert!(settings.to_enable_depth_test());
    assert!(settings.to_enable_depth_write());
    assert!(!settings.to_clear_depth());
    session.close();
}

#[test]
fn named_directional_shadow_lights_replace_all_default_active_lights() {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    lights(&mut session).unwrap();
    let actual = session.context().current_viewer().active_lights();
    assert_eq!(actual.len(), 2);
    for (light, name) in actual.iter().zip(["v1", "v2"]) {
        assert_eq!(light.name(), AsciiString::from(name));
        assert_eq!(light.intensity(), 1.);
        assert!(light.to_cast_shadows());
    }
    session.close();
}
