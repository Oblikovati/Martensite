//! `headlight`: a spot and a positional headlight light the box from the eye.
use super::scene::{brass_box_scene, direct, positional_light, shown, spot_light};
use crate::session_run::{SessionRun, banner};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkservice::graphic3d::Graphic3dTypeOfShadingModel;
use std::error::Error;

const RULE: &str = "=================================";

/// Print the original's error unless pixel (200, 200) is named `expected`.
fn expect_centre(run: &mut SessionRun, expected: &str) -> Result<(), Box<dyn Error>> {
    if run.session().pixel_color_name(200, 200)? != expected {
        println!("Error: expected color near the light is {expected}");
    }
    Ok(())
}

/// `headlight`.
pub(crate) fn headlight_spot_and_positional() -> Result<(), Box<dyn Error>> {
    banner(&[
        RULE,
        "0031704: Visualization - add an interactive object AisLightSource representing a light source",
        RULE,
    ]);
    let mut run = SessionRun::new()?;
    let model = Graphic3dTypeOfShadingModel::Phong;
    brass_box_scene(&mut run, model, Some(0.1), ([10.; 3], [30.; 3]))?;
    println!("=== Check headlight option with spotlight ===");
    let spot = spot_light("aSpotlight")?;
    direct(&spot, [0., 0., -1.])?;
    let value = spot.get().ok_or("a live light")?;
    value.set_headlight(true)?;
    value.set_intensity(1_000_000_000.)?;
    value.set_color(Color::named(NamedColor::GREEN));
    run.session().add_light(&spot, Some(&shown))?;
    expect_centre(&mut run, "GREEN")?;
    run.dump("headlight_spot.png")?;
    println!("=== Check headlight option with positional light ===");
    run.session().remove_light(&spot)?;
    let positional = positional_light("aPosLight")?;
    let value = positional.get().ok_or("a live light")?;
    value.set_headlight(true)?;
    value.set_color(Color::named(NamedColor::RED));
    run.session().add_light(&positional, Some(&shown))?;
    expect_centre(&mut run, "RED")?;
    run.dump("headlight_pos.png")?;
    run.finish()
}
