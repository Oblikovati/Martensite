//! `manipulator`: a manipulator attached to a spot light rotates and moves it.
use super::scene::{brass_box_scene, direct, place, shown, spot_light};
use crate::case_view::viewer_session::{ManipulatorCommand, ManipulatorDrag, ManipulatorPlacement};
use crate::session_run::{SessionRun, banner};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkservice::graphic3d::Graphic3dTypeOfShadingModel;
use std::error::Error;

/// `manipulator`.
pub(crate) fn light_manipulator() -> Result<(), Box<dyn Error>> {
    banner(&[
        "===================================================",
        "0031704: Visualization - add an interactive object AisLightSource representing a light source",
        "===================================================",
    ]);
    let mut run = SessionRun::new()?;
    let model = Graphic3dTypeOfShadingModel::Phong;
    brass_box_scene(&mut run, model, Some(0.1), ([0.; 3], [30.; 3]))?;
    let spot = spot_light("aSpotLight")?;
    place(&spot, [15., -10., 15.])?;
    direct(&spot, [0., 1., 0.])?;
    spot.get()
        .ok_or("a live light")?
        .set_color(Color::named(NamedColor::RED));
    run.session().add_light(&spot, Some(&shown))?;
    let attach = ManipulatorCommand {
        attach: Some("aSpotLight"),
        placement: Some(ManipulatorPlacement::Location),
        ..Default::default()
    };
    run.session().manipulate("m", &attach)?;
    run.dump("manipulator_manip.png")?;
    let follow = ManipulatorCommand {
        follow_rotation: Some(true),
        ..Default::default()
    };
    run.session().manipulate("m", &follow)?;
    drag_from(&mut run, (128, 231), (98, 200))?;
    run.dump("manipulator_rot.png")?;
    drag_from(&mut run, (87, 315), (167, 358))?;
    run.session().manipulate(
        "m",
        &ManipulatorCommand {
            detach: true,
            ..Default::default()
        },
    )?;
    run.session().remove(&["aSpotLight"])?;
    run.dump("manipulator_trans.png")?;
    run.finish()
}

/// Hover and select at `from`, drag start/transform/stop to `to`, then deselect and
/// move the cursor to `to`; the translation drag stops before the detach.
fn drag_from(run: &mut SessionRun, from: (i32, i32), to: (i32, i32)) -> Result<(), Box<dyn Error>> {
    run.session().move_cursor(from.0, from.1)?;
    run.session().select_point(from.0, from.1)?;
    let steps = [
        ManipulatorDrag {
            start: Some(from),
            ..Default::default()
        },
        ManipulatorDrag {
            to: Some(to),
            ..Default::default()
        },
        ManipulatorDrag {
            stop: Some(true),
            ..Default::default()
        },
    ];
    for step in steps {
        run.session().manipulate(
            "m",
            &ManipulatorCommand {
                drag: step,
                ..Default::default()
            },
        )?;
    }
    run.session().select_point(0, 0)?;
    run.session().move_cursor(to.0, to.1)?;
    Ok(())
}
