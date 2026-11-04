//! `dyn_highlight`: directional light presentations detect their sphere under the cursor,
//! draggable or not.
use super::scene::{ambient_light, brass_box_scene, direct, directional_light, shown};
use crate::session_run::{SessionRun, banner};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Failure;
use martensite_tkservice::graphic3d::Graphic3dTypeOfShadingModel;
use martensite_tkv3d::ais::{
    AisLightSource, AisLightSourceMembers as _, AisLightSourceSource as _,
};
use std::error::Error;

const RULE: &str = "=================================";
const RED_SPHERE: [(i32, i32); 4] = [(350, 202), (354, 101), (445, 169), (259, 239)];
const BLUE_SPHERE: [(i32, i32); 4] = [(122, 50), (126, 25), (149, 46), (101, 47)];

/// `dyn_highlight`.
pub(crate) fn sphere_detection() -> Result<(), Box<dyn Error>> {
    banner(&[
        RULE,
        "0032183: Visualization - implement AisLightSource::ProcessDragging() for rotating directional light",
        "Tests dynamic highlighting of directional light source",
        RULE,
    ]);
    let mut run = SessionRun::with_view_size(640, 480)?;
    let model = Graphic3dTypeOfShadingModel::Phong;
    brass_box_scene(&mut run, model, None, ([10.; 3], [10.; 3]))?;
    add_lights(&mut run)?;
    detect_each(
        &mut run,
        &RED_SPHERE,
        "dyn_highlight_dyn_highlighting_red_light_draggable.png",
    )?;
    detect_each(
        &mut run,
        &BLUE_SPHERE,
        "dyn_highlight_dyn_highlighting_blue_light_draggable.png",
    )?;
    let fixed = |prs: &AisLightSource| -> Result<(), Failure> {
        prs.set_draggable(false);
        Ok(())
    };
    let red = run.session().active_light(2)?;
    run.session().add_light(&red, Some(&fixed))?;
    run.session().move_cursor(350, 202)?;
    run.dump("dyn_highlight_dyn_highlighting_red_light_non_draggable.png")?;
    let blue = run.session().active_light(1)?;
    run.session().add_light(&blue, Some(&fixed))?;
    run.session().move_cursor(122, 50)?;
    run.dump("dyn_highlight_dyn_highlighting_blue_light_non_draggable.png")?;
    run.finish()
}

fn add_lights(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    let ambient = ambient_light("AMBIENT", NamedColor::WHITE)?;
    run.session().add_light(&ambient, Some(&shown))?;
    let blue = directional_light("DIR1")?;
    direct(&blue, [1., 0., 0.])?;
    blue.get()
        .ok_or("a live light")?
        .set_color(Color::named(NamedColor::BLUE));
    run.session().add_light(&blue, Some(&shown))?;
    let red = directional_light("DIR2")?;
    direct(&red, [0., 1., 0.])?;
    red.get()
        .ok_or("a live light")?
        .set_color(Color::named(NamedColor::RED));
    let sized = |prs: &AisLightSource| -> Result<(), Failure> {
        prs.set_size(200.);
        prs.set_arc_size(50);
        Ok(())
    };
    run.session().add_light(&red, Some(&sized))?;
    Ok(())
}

/// Move the cursor over each point, dumping `image` after the first, and print the original's
/// error unless a sensitive sphere is detected there.
fn detect_each(
    run: &mut SessionRun,
    points: &[(i32, i32)],
    image: &'static str,
) -> Result<(), Box<dyn Error>> {
    for (index, &(x, y)) in points.iter().enumerate() {
        run.session().move_cursor(x, y)?;
        if index == 0 {
            run.dump(image)?;
        }
        if !run
            .session()
            .picked_entity_types()?
            .iter()
            .any(|name| name == "Select3DSensitiveSphere")
        {
            println!("Error: sphere should be detected");
        }
    }
    Ok(())
}
