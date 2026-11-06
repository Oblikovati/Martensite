//! `display_all`: every light kind shown in a PBR view, toggled off and on by clicking its
//! presentation, and presented again by light index with ranges.
use super::scene::{
    ambient_light, brass_box_scene, direct, directional_light, place, positional_light, shown,
    spot_light,
};
use crate::session_run::{SessionRun, banner};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Failure;
use martensite_tkservice::graphic3d::Graphic3dTypeOfShadingModel;
use martensite_tkv3d::ais::{AisLightSource, AisLightSourceMembers as _};
use std::error::Error;

const RULE: &str = "=================================";
/// The presentation pick points of the ambient, spot, positional and directional lights.
const PICKS: [(i32, i32); 4] = [(105, 97), (897, 508), (640, 62), (330, 536)];

/// `display_all`.
pub(crate) fn every_light_kind() -> Result<(), Box<dyn Error>> {
    banner(&[
        RULE,
        "0031704: Visualization - add an interactive object AisLightSource representing a light source",
        RULE,
    ]);
    let mut run = SessionRun::with_view_size(1280, 720)?;
    let model = Graphic3dTypeOfShadingModel::Pbr;
    brass_box_scene(&mut run, model, None, ([10.; 3], [30.; 3]))?;
    add_four_lights(&mut run)?;
    run.dump("display_all_def.png")?;
    click_every_light(&mut run)?;
    run.dump("display_all_off.png")?;
    expect_pixel(&mut run, (796, 454), "LIGHTSALMON3", "the light spot")?;
    expect_pixel(&mut run, (630, 177), "THISTLE4", "the positional light")?;
    run.session()
        .remove(&["AMBIENT", "DIR", "SPOT", "POSITIONAL"])?;
    for index in 0..4 {
        let light = run.session().active_light(index)?;
        let setup: &dyn Fn(&AisLightSource) -> Result<(), Failure> =
            if index < 2 { &shown } else { &with_range };
        run.session().add_light(&light, Some(setup))?;
    }
    run.dump("display_all_on.png")?;
    click_every_light(&mut run)?;
    expect_pixel(&mut run, (796, 454), "CORAL", "the light spot")?;
    expect_pixel(
        &mut run,
        (630, 177),
        "LIGHTSLATEBLUE",
        "the positional light",
    )?;
    expect_pixel(&mut run, (453, 453), "BURLYWOOD3", "the directional light")?;
    run.finish()
}

/// Display with the range shown.
fn with_range(presentation: &AisLightSource) -> Result<(), Failure> {
    presentation.set_display_range(true);
    Ok(())
}

fn add_four_lights(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    let ambient = ambient_light("AMBIENT", NamedColor::WHITE)?;
    run.session().add_light(&ambient, Some(&shown))?;
    let directional = directional_light("DIR")?;
    direct(&directional, [0., 1., 0.])?;
    directional
        .get()
        .ok_or("a live light")?
        .set_color(Color::named(NamedColor::GREEN));
    run.session().add_light(&directional, Some(&shown))?;
    let spot = spot_light("SPOT")?;
    place(&spot, [50., 25., 25.])?;
    direct(&spot, [-1., 0., 0.])?;
    let value = spot.get().ok_or("a live light")?;
    value.set_intensity(1_000_000_000.)?;
    value.set_color(Color::named(NamedColor::RED));
    run.session().add_light(&spot, Some(&shown))?;
    let positional = positional_light("POSITIONAL")?;
    place(&positional, [25., 25., 50.])?;
    let value = positional.get().ok_or("a live light")?;
    value.set_intensity(10_000_000_000.)?;
    value.set_range(20.)?;
    value.set_color(Color::named(NamedColor::BLUE));
    run.session().add_light(&positional, Some(&shown))?;
    Ok(())
}

fn click_every_light(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    for (x, y) in PICKS {
        run.session().select_point(x, y)?;
    }
    Ok(())
}

/// Print the original's error unless pixel `at` is named `expected`.
fn expect_pixel(
    run: &mut SessionRun,
    (x, y): (i32, i32),
    expected: &str,
    near: &str,
) -> Result<(), Box<dyn Error>> {
    if run.session().pixel_color_name(x, y)? != expected {
        println!("Error: expected color near {near} is {expected}");
    }
    Ok(())
}
